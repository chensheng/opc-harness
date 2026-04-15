//! Agent Manager 实现 (VC-004)
//! 
//! 统一的 Agent 管理器，整合 WebSocketManager、StdioChannelManager 和 DaemonManager
//! 提供高级 API 供前端调用
//! 
//! 核心功能:
//! - Agent 生命周期管理（创建、启动、暂停、恢复、终止）
//! - Agent 状态追踪和持久化
//! - 并发控制和资源调度
//! - 消息路由（Stdio + WebSocket）
//! - Tauri Commands 暴露

// 重新导出主要类型
pub use crate::agent::agent_manager_types::{AgentHandle, AgentManagerStats};
pub use crate::agent::agent_manager_core::AgentManager;

// 重新导出所有 Tauri Commands
pub use crate::agent::agent_manager_commands::*;

// ============================================================================
// Tests (VC-004)
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::types::{AgentType, AgentStatus, AgentPhase};

    #[test]
    fn test_agent_handle_creation() {
        let handle = AgentHandle::new(
            AgentType::Initializer,
            "test-session".to_string(),
            "/tmp/test".to_string(),
            None,
        );

        assert!(!handle.agent_id.is_empty());
        assert_eq!(handle.agent_type, AgentType::Initializer);
        assert_eq!(handle.session_id, "test-session");
        assert_eq!(handle.status, AgentStatus::Idle);
        assert_eq!(handle.phase, AgentPhase::Initializer);
        assert!(!handle.registered_to_daemon);
        assert!(handle.stdio_channel_id.is_none());
    }

    #[test]
    fn test_agent_handle_status_update() {
        let mut handle = AgentHandle::new(
            AgentType::Coding,
            "session-123".to_string(),
            "/tmp/project".to_string(),
            None,
        );

        assert_eq!(handle.status, AgentStatus::Idle);
        
        handle.update_status(AgentStatus::Running);
        assert_eq!(handle.status, AgentStatus::Running);
        
        handle.update_status(AgentStatus::Completed);
        assert_eq!(handle.status, AgentStatus::Completed);
    }

    #[test]
    fn test_agent_handle_stdio_channel() {
        let mut handle = AgentHandle::new(
            AgentType::MRCreation,
            "session-456".to_string(),
            "/tmp/mr".to_string(),
            None,
        );

        assert!(handle.stdio_channel_id.is_none());
        
        handle.set_stdio_channel("channel-789".to_string());
        assert_eq!(handle.stdio_channel_id, Some("channel-789".to_string()));
    }

    #[test]
    fn test_agent_handle_mark_registered() {
        let mut handle = AgentHandle::new(
            AgentType::Initializer,
            "test".to_string(),
            "/tmp".to_string(),
            None,
        );

        assert!(!handle.registered_to_daemon);
        
        handle.mark_registered();
        assert!(handle.registered_to_daemon);
    }

    #[test]
    fn test_agent_manager_stats_default() {
        let stats = AgentManagerStats::default();
        
        assert_eq!(stats.total_agents, 0);
        assert_eq!(stats.running_agents, 0);
        assert_eq!(stats.idle_agents, 0);
        assert_eq!(stats.completed_agents, 0);
        assert_eq!(stats.failed_agents, 0);
    }

    #[tokio::test]
    async fn test_agent_manager_creation() {
        // 注意：这个测试需要一个 mock 的 AppHandle
        // 在实际测试中需要使用 tauri::test::mock_context 等工具
        // 这里只是一个示例
        println!("AgentManager creation test - requires mock AppHandle");
    }

    #[test]
    fn test_agent_type_display() {
        assert_eq!(format!("{}", AgentType::Initializer), "initializer");
        assert_eq!(format!("{}", AgentType::Coding), "coding");
        assert_eq!(format!("{}", AgentType::MRCreation), "mr_creation");
    }

    // ========================================================================
    // VC-005: Session Persistence Tests
    // ========================================================================

    #[test]
    fn test_agent_status_display() {
        assert_eq!(format!("{}", AgentStatus::Idle), "idle");
        assert_eq!(format!("{}", AgentStatus::Running), "running");
        assert_eq!(format!("{}", AgentStatus::Paused), "paused");
        assert_eq!(format!("{}", AgentStatus::Completed), "completed");
        assert_eq!(format!("{}", AgentStatus::Failed("error".to_string())), "failed:error");
    }

    #[test]
    fn test_agent_phase_display() {
        assert_eq!(format!("{}", AgentPhase::Initializer), "initializer");
        assert_eq!(format!("{}", AgentPhase::Coding), "coding");
        assert_eq!(format!("{}", AgentPhase::MRCreation), "mr_creation");
    }

    #[test]
    fn test_agent_session_serialization() {
        let session = crate::models::AgentSession {
            session_id: "test-session-123".to_string(),
            agent_id: "agent-456".to_string(),
            agent_type: "initializer".to_string(),
            project_id: "project-uuid-789".to_string(),
            name: None,
            status: "running".to_string(),
            phase: "initializer".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
            stdio_channel_id: Some("channel-789".to_string()),
            registered_to_daemon: true,
            metadata: None,
            agents_md_content: None,
        };

        // Test serialization
        let json = serde_json::to_string(&session).unwrap();
        assert!(json.contains("test-session-123"));
        assert!(json.contains("agent-456"));
        assert!(json.contains("initializer"));

        // Test deserialization
        let deserialized: crate::models::AgentSession = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.session_id, session.session_id);
        assert_eq!(deserialized.agent_id, session.agent_id);
        assert_eq!(deserialized.registered_to_daemon, session.registered_to_daemon);
    }
}
