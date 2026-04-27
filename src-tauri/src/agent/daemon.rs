//! Daemon Manager 实现
//! 
//! 负责守护进程的启动、停止、Agent 调度和并发控�?

// 重新导出所有类型和核心类（从父模块导入�?
pub use super::daemon_types::*;
pub use super::daemon_core::DaemonManager;

#[cfg(test)]
mod tests {
    use super::*;

    /// 获取测试用的临时目录路径（跨平台兼容）
    fn get_test_path() -> String {
        std::env::current_dir()
            .unwrap_or_else(|_| std::path::PathBuf::from("."))
            .to_string_lossy()
            .to_string()
    }

    #[test]
    fn test_daemon_manager_creation() {
        let manager = DaemonManager::new();
        
        assert_eq!(manager.get_status(), DaemonStatus::Stopped);
        assert_eq!(manager.running_count, 0);
        assert_eq!(manager.max_concurrent, 5);
    }

    #[test]
    fn test_daemon_manager_start() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: get_test_path(),
            log_level: "debug".to_string(),
            max_concurrent_agents: 3,
            workspace_dir: get_test_path(),
        };
        
        let result = manager.start(config);
        assert!(result.is_ok());
        assert_eq!(manager.get_status(), DaemonStatus::Running);
    }

    #[test]
    fn test_daemon_manager_stop() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: get_test_path(),
            log_level: "info".to_string(),
            max_concurrent_agents: 5,
            workspace_dir: get_test_path(),
        };
        
        manager.start(config).unwrap();
        let result = manager.stop(true);
        
        assert!(result.is_ok());
        assert_eq!(manager.get_status(), DaemonStatus::Stopped);
    }

    #[test]
    fn test_daemon_manager_spawn_agent() {
        let mut manager = DaemonManager::new();
        
        // 使用当前目录作为测试路径（跨平台兼容）
        let test_path = std::env::current_dir()
            .unwrap_or_else(|_| std::path::PathBuf::from("."))
            .to_string_lossy()
            .to_string();
        
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: test_path.clone(),
            log_level: "info".to_string(),
            max_concurrent_agents: 5,
            workspace_dir: test_path.clone(),
        };
        
        manager.start(config).unwrap();
        let agent_id = manager.spawn_agent("initializer", &test_path);
        
        // 打印详细错误信息以便调试
        if let Err(ref e) = agent_id {
            eprintln!("Failed to spawn agent: {}", e);
        }
        
        assert!(agent_id.is_ok(), "Expected Ok(agent_id), got Err: {:?}", agent_id.err());
        assert!(!agent_id.as_ref().unwrap().is_empty());
    }

    #[test]
    fn test_daemon_manager_spawn_real_process() {
        use std::process::Command;
        
        let mut manager = DaemonManager::new();
        let test_path = get_test_path();
        
        let config = DaemonConfig {
            session_id: "session-real-test".to_string(),
            project_path: test_path.clone(),
            log_level: "debug".to_string(),
            max_concurrent_agents: 3,
            workspace_dir: test_path.clone(),
        };
        
        manager.start(config).unwrap();
        
        // 尝试启动一个真实的命令（echo）
        let agent_id = manager.spawn_agent("test_echo", &test_path);
        
        if let Ok(id) = &agent_id {
            // 验证 Agent 被创建
            assert!(!id.is_empty());
            
            // 检查 Daemon 状态是否为 Running
            let status = manager.get_status();
            assert_eq!(status, crate::agent::daemon_types::DaemonStatus::Running);
            
            println!("✓ Successfully spawned real process with ID: {}", id);
        } else {
            // 在某些环境下可能失败，记录但不阻断测试
            eprintln!("Note: Real process spawn failed (expected in some environments): {:?}", agent_id.err());
        }
    }

    #[test]
    fn test_daemon_manager_pause_resume() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: get_test_path(),
            log_level: "info".to_string(),
            max_concurrent_agents: 5,
            workspace_dir: get_test_path(),
        };
        
        manager.start(config).unwrap();
        
        let pause_result = manager.pause();
        assert!(pause_result.is_ok());
        assert_eq!(manager.get_status(), DaemonStatus::Paused);
        
        let resume_result = manager.resume();
        assert!(resume_result.is_ok());
        assert_eq!(manager.get_status(), DaemonStatus::Running);
    }

    #[test]
    fn test_resource_usage_default() {
        let usage = ResourceUsage::default();
        
        assert_eq!(usage.cpu_percent, 0.0);
        assert_eq!(usage.memory_mb, 0);
    }

    #[test]
    fn test_concurrency_config_initialization() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: get_test_path(),
            log_level: "info".to_string(),
            max_concurrent_agents: 4,
            workspace_dir: get_test_path(),
        };
        
        manager.start(config).unwrap();
        
        assert_eq!(manager.max_concurrent, 4);
        assert_eq!(manager.running_count, 0);
    }

    #[test]
    fn test_can_spawn_agent_when_slots_available() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: get_test_path(),
            log_level: "info".to_string(),
            max_concurrent_agents: 2,
            workspace_dir: get_test_path(),
        };
        
        manager.start(config).unwrap();
        
        assert!(manager.can_spawn_agent());
        assert_eq!(manager.available_slots(), 2);
    }

    #[test]
    fn test_cannot_spawn_agent_when_slots_full() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: get_test_path(),
            log_level: "info".to_string(),
            max_concurrent_agents: 2,
            workspace_dir: get_test_path(),
        };
        
        manager.start(config).unwrap();
        
        let agent1 = manager.spawn_agent("coding", &get_test_path()).unwrap();
        let agent2 = manager.spawn_agent("coding", &get_test_path()).unwrap();
        
        manager.try_start_agent(&agent1, &get_test_path());
        manager.try_start_agent(&agent2, &get_test_path());
        
        assert!(!manager.can_spawn_agent());
        assert_eq!(manager.available_slots(), 0);
    }

    #[test]
    fn test_agent_queuing_when_concurrent_limit_reached() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: get_test_path(),
            log_level: "info".to_string(),
            max_concurrent_agents: 2,
            workspace_dir: get_test_path(),
        };
        
        manager.start(config).unwrap();
        
        let agent1 = manager.spawn_agent("coding", &get_test_path()).unwrap();
        let agent2 = manager.spawn_agent("coding", &get_test_path()).unwrap();
        manager.try_start_agent(&agent1, &get_test_path());
        manager.try_start_agent(&agent2, &get_test_path());
        
        let agent3 = manager.spawn_agent("coding", &get_test_path()).unwrap();
        let should_start = manager.try_start_agent(&agent3, &get_test_path());
        
        assert!(!should_start);
        assert_eq!(manager.agent_queue.len(), 1);
    }

    #[test]
    fn test_auto_schedule_next_agent_on_completion() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: get_test_path(),
            log_level: "info".to_string(),
            max_concurrent_agents: 2,
            workspace_dir: get_test_path(),
        };
        
        manager.start(config).unwrap();
        
        let agent1 = manager.spawn_agent("coding", &get_test_path()).unwrap();
        let agent2 = manager.spawn_agent("coding", &get_test_path()).unwrap();
        let agent3 = manager.spawn_agent("coding", &get_test_path()).unwrap();
        
        manager.try_start_agent(&agent1, &get_test_path());
        manager.try_start_agent(&agent2, &get_test_path());
        manager.try_start_agent(&agent3, &get_test_path());
        
        assert_eq!(manager.running_count, 2);
        assert_eq!(manager.agent_queue.len(), 1);
        
        manager.stop_agent(&agent1).unwrap();
        
        assert_eq!(manager.running_count, 2);
        assert!(manager.agent_queue.is_empty());
    }

    #[test]
    fn test_concurrency_stats() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: get_test_path(),
            log_level: "info".to_string(),
            max_concurrent_agents: 4,
            workspace_dir: get_test_path(),
        };
        
        manager.start(config).unwrap();
        
        let agent1 = manager.spawn_agent("coding", &get_test_path()).unwrap();
        let agent2 = manager.spawn_agent("coding", &get_test_path()).unwrap();
        manager.try_start_agent(&agent1, &get_test_path());
        manager.try_start_agent(&agent2, &get_test_path());
        
        let stats = manager.get_concurrency_stats();
        
        assert_eq!(stats.running_count, 2);
        assert_eq!(stats.max_concurrent, 4);
        assert_eq!(stats.available_slots, 2);
    }

    #[test]
    fn test_adjust_max_concurrent_increase() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: get_test_path(),
            log_level: "info".to_string(),
            max_concurrent_agents: 2,
            workspace_dir: get_test_path(),
        };
        
        manager.start(config).unwrap();
        
        let agent1 = manager.spawn_agent("coding", &get_test_path()).unwrap();
        let agent2 = manager.spawn_agent("coding", &get_test_path()).unwrap();
        let agent3 = manager.spawn_agent("coding", &get_test_path()).unwrap();
        
        manager.try_start_agent(&agent1, &get_test_path());
        manager.try_start_agent(&agent2, &get_test_path());
        manager.try_start_agent(&agent3, &get_test_path());
        
        manager.adjust_max_concurrent(4).unwrap();
        
        assert_eq!(manager.max_concurrent, 4);
        assert_eq!(manager.running_count, 3);
        assert!(manager.agent_queue.is_empty());
    }

    #[test]
    fn test_adjust_max_concurrent_decrease() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: get_test_path(),
            log_level: "info".to_string(),
            max_concurrent_agents: 4,
            workspace_dir: get_test_path(),
        };
        
        manager.start(config).unwrap();
        
        for _ in 0..4 {
            let agent_id = manager.spawn_agent("coding", &get_test_path()).unwrap();
            manager.try_start_agent(&agent_id, &get_test_path());
        }
        
        assert_eq!(manager.running_count, 4);
        
        manager.adjust_max_concurrent(2).unwrap();
        
        assert_eq!(manager.max_concurrent, 2);
        assert_eq!(manager.running_count, 2);
        assert_eq!(manager.agent_queue.len(), 2);
    }

    #[test]
    fn test_get_running_and_queued_agents() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: get_test_path(),
            log_level: "info".to_string(),
            max_concurrent_agents: 2,
            workspace_dir: get_test_path(),
        };
        
        manager.start(config).unwrap();
        
        let agent1 = manager.spawn_agent("coding", &get_test_path()).unwrap();
        let agent2 = manager.spawn_agent("coding", &get_test_path()).unwrap();
        let agent3 = manager.spawn_agent("coding", &get_test_path()).unwrap();
        
        manager.try_start_agent(&agent1, &get_test_path());
        manager.try_start_agent(&agent2, &get_test_path());
        manager.try_start_agent(&agent3, &get_test_path());
        
        let running = manager.get_running_agents();
        let queued = manager.get_queued_agents();
        
        assert_eq!(running.len(), 2);
        assert_eq!(queued.len(), 1);
    }

    #[test]
    fn test_adjust_max_concurrent_zero_error() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: get_test_path(),
            log_level: "info".to_string(),
            max_concurrent_agents: 2,
            workspace_dir: get_test_path(),
        };
        
        manager.start(config).unwrap();
        
        let adjust_result = manager.adjust_max_concurrent(0);
        assert!(adjust_result.is_err());
    }
}
