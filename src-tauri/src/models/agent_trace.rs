use serde::{Deserialize, Serialize};
use crate::db::Entity;
use rusqlite::Row;

/// 智能体追踪模型（思考链、工具调用、决策）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentTrace {
    pub id: String,
    pub agent_id: String,
    pub session_id: String,
    pub event_type: String,
    pub timestamp: String,
    pub data: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    pub created_at: String,
}

impl Entity for AgentTrace {
    fn table_name() -> &'static str { "agent_traces" }

    fn primary_key() -> &'static str { "id" }

    fn get_primary_key(&self) -> &str { &self.id }

    fn from_row(row: &Row) -> rusqlite::Result<Self> {
        Ok(AgentTrace {
            id: row.get("id")?,
            agent_id: row.get("agent_id")?,
            session_id: row.get("session_id")?,
            event_type: row.get("event_type")?,
            timestamp: row.get("timestamp")?,
            data: row.get("data")?,
            parent_id: row.get("parent_id").ok(),
            created_at: row.get("created_at")?,
        })
    }
}

impl AgentTrace {
    /// 创建新的追踪记录
    pub fn new(
        id: String,
        agent_id: String,
        session_id: String,
        event_type: String,
        timestamp: String,
        data: String,
        parent_id: Option<String>,
    ) -> Self {
        Self {
            id,
            agent_id,
            session_id,
            event_type,
            timestamp,
            data,
            parent_id,
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// 创建思考链记录
    pub fn new_thought(
        id: String,
        agent_id: String,
        session_id: String,
        content: String,
        parent_id: Option<String>,
    ) -> Self {
        let data = serde_json::json!({
            "content": content
        })
        .to_string();
        Self::new(
            id,
            agent_id,
            session_id,
            "thought".to_string(),
            chrono::Utc::now().to_rfc3339(),
            data,
            parent_id,
        )
    }

    /// 创建工具调用记录
    pub fn new_tool_call(
        id: String,
        agent_id: String,
        session_id: String,
        tool_name: String,
        parameters: serde_json::Value,
        parent_id: Option<String>,
    ) -> Self {
        let data = serde_json::json!({
            "tool_name": tool_name,
            "parameters": parameters
        })
        .to_string();
        Self::new(
            id,
            agent_id,
            session_id,
            "tool_call".to_string(),
            chrono::Utc::now().to_rfc3339(),
            data,
            parent_id,
        )
    }

    /// 创建工具执行结果记录
    pub fn new_tool_result(
        id: String,
        agent_id: String,
        session_id: String,
        success: bool,
        result: serde_json::Value,
        duration_ms: u64,
        parent_id: String,
    ) -> Self {
        let data = serde_json::json!({
            "success": success,
            "result": result,
            "duration_ms": duration_ms
        })
        .to_string();
        Self::new(
            id,
            agent_id,
            session_id,
            "tool_result".to_string(),
            chrono::Utc::now().to_rfc3339(),
            data,
            Some(parent_id),
        )
    }

    /// 创建决策记录
    pub fn new_decision(
        id: String,
        agent_id: String,
        session_id: String,
        context: String,
        decision: String,
        reason: String,
        parent_id: Option<String>,
    ) -> Self {
        let data = serde_json::json!({
            "context": context,
            "decision": decision,
            "reason": reason
        })
        .to_string();
        Self::new(
            id,
            agent_id,
            session_id,
            "decision".to_string(),
            chrono::Utc::now().to_rfc3339(),
            data,
            parent_id,
        )
    }
}
