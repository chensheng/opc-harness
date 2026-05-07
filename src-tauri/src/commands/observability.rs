//! 可观测性相关命令
//!
//! 提供前端与后端可观测性功能的交互接口

use crate::db::{
    get_connection,
};
use crate::models::{AgentAlert, AgentLog, AgentTrace};
use crate::services::observability_service::{ApiCallStats, ObservabilityService};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::State;
use uuid::Uuid;

/// 可观测性服务状态
pub type ObservabilityState = Arc<ObservabilityService>;

/// 日志查询参数
#[derive(Debug, Deserialize)]
pub struct GetLogsParams {
    pub agent_id: String,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub level: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub keyword: Option<String>,
}

/// 日志统计响应
#[derive(Debug, Serialize)]
pub struct LogStatsResponse {
    pub total: i64,
    pub info: i64,
    pub warn: i64,
    pub error: i64,
    pub debug: i64,
    pub success: i64,
}

impl From<LogStats> for LogStatsResponse {
    fn from(stats: LogStats) -> Self {
        Self {
            total: stats.total,
            info: stats.info,
            warn: stats.warn,
            error: stats.error,
            debug: stats.debug,
            success: stats.success,
        }
    }
}

/// 追踪记录响应
#[derive(Debug, Serialize)]
pub struct TraceResponse {
    pub id: String,
    pub agent_id: String,
    pub session_id: String,
    pub event_type: String,
    pub timestamp: String,
    pub data: serde_json::Value,
    pub parent_id: Option<String>,
    pub created_at: String,
}

impl From<AgentTrace> for TraceResponse {
    fn from(trace: AgentTrace) -> Self {
        Self {
            id: trace.id,
            agent_id: trace.agent_id,
            session_id: trace.session_id,
            event_type: trace.event_type,
            timestamp: trace.timestamp,
            data: serde_json::from_str(&trace.data).unwrap_or(serde_json::Value::Null),
            parent_id: trace.parent_id,
            created_at: trace.created_at,
        }
    }
}

/// 告警响应
#[derive(Debug, Serialize)]
pub struct AlertResponse {
    pub id: String,
    pub agent_id: String,
    pub level: String,
    pub alert_type: String,
    pub message: String,
    pub status: String,
    pub created_at: String,
    pub resolved_at: Option<String>,
}

impl From<AgentAlert> for AlertResponse {
    fn from(alert: AgentAlert) -> Self {
        Self {
            id: alert.id,
            agent_id: alert.agent_id,
            level: alert.level,
            alert_type: alert.alert_type,
            message: alert.message,
            status: alert.status,
            created_at: alert.created_at,
            resolved_at: alert.resolved_at,
        }
    }
}

/// 性能指标响应
#[derive(Debug, Serialize)]
pub struct PerformanceMetricsResponse {
    pub agent_id: String,
    pub cpu_usage: f64,
    pub memory_usage_mb: f64,
    pub api_calls: ApiCallStatsResponse,
    pub last_updated: String,
}

#[derive(Debug, Serialize)]
pub struct ApiCallStatsResponse {
    pub total_calls: u64,
    pub successful_calls: u64,
    pub failed_calls: u64,
    pub response_times: ResponseTimeStats,
}

#[derive(Debug, Serialize)]
pub struct ResponseTimeStats {
    pub p50: u64,
    pub p90: u64,
    pub p99: u64,
}

impl From<ApiCallStats> for ApiCallStatsResponse {
    fn from(stats: ApiCallStats) -> Self {
        Self {
            total_calls: stats.total_calls,
            successful_calls: stats.successful_calls,
            failed_calls: stats.failed_calls,
            response_times: ResponseTimeStats {
                p50: stats.get_percentile(50.0),
                p90: stats.get_percentile(90.0),
                p99: stats.get_percentile(99.0),
            },
        }
    }
}

/// 获取智能体日志
#[tauri::command]
pub async fn get_agent_logs(
    obs_state: State<'_, ObservabilityState>,
    params: GetLogsParams,
) -> Result<Vec<AgentLog>, String> {
    let conn = get_connection().map_err(|e| e.to_string())?;
    let repo = crate::db::agent_logs::get_logs_repository(&conn);

    let limit = params.limit.unwrap_or(50);
    let offset = params.offset.unwrap_or(0);

    let logs = if let Some(level) = params.level {
        repo.get_by_level(&params.agent_id, &level, limit).map_err(|e| e.to_string())?
    } else if let (Some(start), Some(end)) = (params.start_time, params.end_time) {
        repo.get_by_agent_id_and_time_range(&params.agent_id, &start, &end, limit).map_err(|e| e.to_string())?
    } else if let Some(keyword) = params.keyword {
        repo.search(&params.agent_id, &keyword, limit).map_err(|e| e.to_string())?
    } else {
        repo.get_by_agent_id(&params.agent_id, limit, offset).map_err(|e| e.to_string())?
    };

    Ok(logs)
}

/// 获取日志统计
#[tauri::command]
pub async fn get_agent_log_stats(
    obs_state: State<'_, ObservabilityState>,
    agent_id: String,
) -> Result<LogStatsResponse, String> {
    let conn = get_connection().map_err(|e| e.to_string())?;
    let repo = crate::db::agent_logs::get_logs_repository(&conn);

    let stats = repo.get_stats(&agent_id).map_err(|e| e.to_string())?;
    Ok(stats.into())
}

/// 获取智能体追踪记录
#[tauri::command]
pub async fn get_agent_traces(
    obs_state: State<'_, ObservabilityState>,
    agent_id: String,
    event_type: Option<String>,
    limit: Option<i64>,
) -> Result<Vec<TraceResponse>, String> {
    let conn = get_connection().map_err(|e| e.to_string())?;
    let repo = crate::db::agent_traces::get_traces_repository(&conn);

    let limit = limit.unwrap_or(100);

    let traces = if let Some(et) = event_type {
        repo.get_by_event_type(&agent_id, &et, limit).map_err(|e| e.to_string())?
    } else {
        repo.get_by_agent_id(&agent_id, limit).map_err(|e| e.to_string())?
    };

    Ok(traces.into_iter().map(Into::into).collect())
}

/// 获取告警列表
#[tauri::command]
pub async fn get_agent_alerts(
    obs_state: State<'_, ObservabilityState>,
    agent_id: String,
    status: Option<String>,
    limit: Option<i64>,
) -> Result<Vec<AlertResponse>, String> {
    let conn = get_connection().map_err(|e| e.to_string())?;
    let repo = crate::db::agent_alerts::get_alerts_repository(&conn);

    let limit = limit.unwrap_or(50);

    let alerts = if let Some(s) = status {
        repo.get_by_agent_id(&agent_id, Some(&s), limit).map_err(|e| e.to_string())?
    } else {
        repo.get_by_agent_id(&agent_id, None, limit).map_err(|e| e.to_string())?
    };

    Ok(alerts.into_iter().map(Into::into).collect())
}

/// 清空智能体日志
#[tauri::command]
pub async fn clear_agent_logs(
    obs_state: State<'_, ObservabilityState>,
    agent_id: String,
) -> Result<usize, String> {
    let conn = get_connection().map_err(|e| e.to_string())?;
    let repo = crate::db::agent_logs::get_logs_repository(&conn);

    let count = repo.clear_by_agent_id(&agent_id).map_err(|e| e.to_string())?;
    obs_state.clear_logs(&agent_id);

    Ok(count)
}

/// 导出智能体日志（CSV 格式）
#[tauri::command]
pub async fn export_agent_logs(
    obs_state: State<'_, ObservabilityState>,
    agent_id: String,
) -> Result<String, String> {
    let conn = get_connection().map_err(|e| e.to_string())?;
    let repo = crate::db::agent_logs::get_logs_repository(&conn);

    let logs = repo.get_by_agent_id(&agent_id, 1000, 0).map_err(|e| e.to_string())?;

    // CSV 格式
    let mut csv = String::from("id,timestamp,level,source,message\n");
    for log in logs {
        let message = log.message.replace('"', "\"\"");
        csv.push_str(&format!(
            "{},{},{},\"{}\",\"{}\"\n",
            log.id, log.timestamp, log.level, log.source, message
        ));
    }

    Ok(csv)
}

/// 记录日志（从 WebSocket 消息接收）
#[tauri::command]
pub async fn log_agent_message(
    obs_state: State<'_, ObservabilityService>,
    agent_id: String,
    session_id: String,
    level: String,
    source: String,
    message: String,
) -> Result<String, String> {
    let id = format!("log-{}", Uuid::new_v4());
    let agent_id_clone = agent_id.clone();
    let log = AgentLog::new(
        id.clone(),
        agent_id,
        session_id,
        chrono::Utc::now().to_rfc3339(),
        level,
        source,
        message,
    );

    obs_state.add_log(&agent_id_clone, log);
    Ok(id)
}

/// 记录思考链
#[tauri::command]
pub async fn record_agent_thought(
    obs_state: State<'_, ObservabilityService>,
    agent_id: String,
    session_id: String,
    content: String,
    parent_id: Option<String>,
) -> Result<String, String> {
    Ok(obs_state.record_thought(&agent_id, &session_id, &content, parent_id.as_deref()))
}

/// 记录工具调用
#[tauri::command]
pub async fn record_agent_tool_call(
    obs_state: State<'_, ObservabilityService>,
    agent_id: String,
    session_id: String,
    tool_name: String,
    parameters: String,
    parent_id: Option<String>,
) -> Result<String, String> {
    let params: serde_json::Value = serde_json::from_str(&parameters)
        .map_err(|e| format!("Invalid JSON parameters: {}", e))?;

    Ok(obs_state.record_tool_call(&agent_id, &session_id, &tool_name, params, parent_id.as_deref()))
}

/// 记录工具执行结果
#[tauri::command]
pub async fn record_agent_tool_result(
    obs_state: State<'_, ObservabilityService>,
    agent_id: String,
    session_id: String,
    success: bool,
    result: String,
    duration_ms: u64,
    parent_id: String,
) -> Result<(), String> {
    let result_value: serde_json::Value = serde_json::from_str(&result)
        .map_err(|e| format!("Invalid JSON result: {}", e))?;

    obs_state.record_tool_result(&agent_id, &session_id, success, result_value, duration_ms, &parent_id);
    Ok(())
}

/// 记录决策
#[tauri::command]
pub async fn record_agent_decision(
    obs_state: State<'_, ObservabilityService>,
    agent_id: String,
    session_id: String,
    context: String,
    decision: String,
    reason: String,
    parent_id: Option<String>,
) -> Result<(), String> {
    obs_state.record_decision(&agent_id, &session_id, &context, &decision, &reason, parent_id.as_deref());
    Ok(())
}

/// 记录 API 调用
#[tauri::command]
pub async fn record_api_call(
    obs_state: State<'_, ObservabilityService>,
    agent_id: String,
    success: bool,
    duration_ms: u64,
) -> Result<(), String> {
    obs_state.record_api_call(&agent_id, success, duration_ms);
    Ok(())
}

/// 更新性能指标
#[tauri::command]
pub async fn update_performance_metrics(
    obs_state: State<'_, ObservabilityService>,
    agent_id: String,
    cpu_usage: f64,
    memory_usage_mb: f64,
) -> Result<(), String> {
    obs_state.update_metrics(&agent_id, cpu_usage, memory_usage_mb, None);
    Ok(())
}

/// 获取性能指标
#[tauri::command]
pub async fn get_performance_metrics(
    obs_state: State<'_, ObservabilityService>,
    agent_id: String,
) -> Result<Option<PerformanceMetricsResponse>, String> {
    Ok(obs_state.get_metrics(&agent_id).map(|m| {
        PerformanceMetricsResponse {
            agent_id: m.agent_id,
            cpu_usage: m.cpu_usage,
            memory_usage_mb: m.memory_usage_mb,
            api_calls: m.api_calls.into(),
            last_updated: m.timestamp,
        }
    }))
}

/// 解决告警
#[tauri::command]
pub async fn resolve_agent_alert(
    obs_state: State<'_, ObservabilityService>,
    alert_id: String,
) -> Result<(), String> {
    let conn = get_connection().map_err(|e| e.to_string())?;
    let repo = crate::db::agent_alerts::get_alerts_repository(&conn);

    repo.resolve(&alert_id).map_err(|e| e.to_string())?;
    Ok(())
}

/// 获取告警配置
#[tauri::command]
pub async fn get_alert_config(
    obs_state: State<'_, ObservabilityService>,
) -> Result<crate::services::observability_service::AlertConfig, String> {
    Ok(obs_state.get_alert_config())
}

/// 更新告警配置
#[tauri::command]
pub async fn update_alert_config(
    obs_state: State<'_, ObservabilityService>,
    config: crate::services::observability_service::AlertConfig,
) -> Result<(), String> {
    obs_state.update_alert_config(config);
    Ok(())
}
