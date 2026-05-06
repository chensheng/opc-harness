//! 智能体可观测性服务
//!
//! 提供日志收集、持久化、追踪记录、告警检测和性能监控服务

use crate::db::{
    agent_alerts::{self, AgentAlertsRepository},
    agent_logs::{self, AgentLogsRepository},
    agent_traces::{self, AgentTracesRepository},
    get_connection,
};
use crate::models::{AgentAlert, AgentLog, AgentTrace, alert_level, alert_type};
use chrono::Utc;
use rusqlite::Connection;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

/// 内存日志缓存
#[derive(Debug, Clone)]
pub struct LogCache {
    pub logs: Vec<AgentLog>,
    pub max_size: usize,
}

impl LogCache {
    pub fn new(max_size: usize) -> Self {
        Self {
            logs: Vec::with_capacity(max_size),
            max_size,
        }
    }

    pub fn add(&mut self, log: AgentLog) {
        self.logs.push(log);
        if self.logs.len() > self.max_size {
            self.logs.remove(0);
        }
    }

    pub fn get_all(&self) -> Vec<AgentLog> {
        self.logs.clone()
    }

    pub fn clear(&mut self) {
        self.logs.clear();
    }
}

/// 内存追踪缓存
#[derive(Debug, Clone)]
pub struct TraceCache {
    pub traces: Vec<AgentTrace>,
}

impl TraceCache {
    pub fn new() -> Self {
        Self {
            traces: Vec::new(),
        }
    }

    pub fn add(&mut self, trace: AgentTrace) {
        self.traces.push(trace);
    }

    pub fn get_all(&self) -> Vec<AgentTrace> {
        self.traces.clone()
    }

    pub fn clear(&mut self) {
        self.traces.clear();
    }
}

/// 性能指标数据
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub agent_id: String,
    pub cpu_usage: f64,
    pub memory_usage_mb: f64,
    pub api_calls: ApiCallStats,
    pub timestamp: String,
}

#[derive(Debug, Clone)]
pub struct ApiCallStats {
    pub total_calls: u64,
    pub successful_calls: u64,
    pub failed_calls: u64,
    pub response_times_ms: Vec<u64>,
}

impl ApiCallStats {
    pub fn new() -> Self {
        Self {
            total_calls: 0,
            successful_calls: 0,
            failed_calls: 0,
            response_times_ms: Vec::new(),
        }
    }

    pub fn record_call(&mut self, success: bool, duration_ms: u64) {
        self.total_calls += 1;
        if success {
            self.successful_calls += 1;
        } else {
            self.failed_calls += 1;
        }
        self.response_times_ms.push(duration_ms);
        // 只保留最近 100 次调用
        if self.response_times_ms.len() > 100 {
            self.response_times_ms.remove(0);
        }
    }

    pub fn get_percentile(&self, percentile: f64) -> u64 {
        if self.response_times_ms.is_empty() {
            return 0;
        }

        let mut sorted = self.response_times_ms.clone();
        sorted.sort();

        let index = ((percentile / 100.0) * (sorted.len() - 1) as f64) as usize;
        sorted[index.min(sorted.len() - 1)]
    }
}

impl Default for ApiCallStats {
    fn default() -> Self {
        Self::new()
    }
}

/// 告警配置
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AlertConfig {
    pub enabled: bool,
    pub no_response_warning_minutes: i64,
    pub no_response_critical_minutes: i64,
    pub error_rate_warning_per_minute: i64,
    pub error_rate_critical_per_minute: i64,
    pub cpu_high_percent: f64,
    pub memory_high_percent: f64,
}

impl Default for AlertConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            no_response_warning_minutes: 5,
            no_response_critical_minutes: 10,
            error_rate_warning_per_minute: 10,
            error_rate_critical_per_minute: 30,
            cpu_high_percent: 90.0,
            memory_high_percent: 95.0,
        }
    }
}

/// 可观测性服务
pub struct ObservabilityService {
    log_caches: Arc<RwLock<HashMap<String, LogCache>>>,
    trace_caches: Arc<RwLock<HashMap<String, TraceCache>>>,
    metrics: Arc<RwLock<HashMap<String, PerformanceMetrics>>>,
    alert_config: Arc<RwLock<AlertConfig>>,
}

impl ObservabilityService {
    pub fn new() -> Self {
        Self {
            log_caches: Arc::new(RwLock::new(HashMap::new())),
            trace_caches: Arc::new(RwLock::new(HashMap::new())),
            metrics: Arc::new(RwLock::new(HashMap::new())),
            alert_config: Arc::new(RwLock::new(AlertConfig::default())),
        }
    }

    // ==================== 日志服务 ====================

    /// 添加日志到内存缓存
    pub fn add_log(&self, agent_id: &str, log: AgentLog) {
        let mut caches = self.log_caches.write().unwrap();
        let cache = caches
            .entry(agent_id.to_string())
            .or_insert_with(|| LogCache::new(100));
        cache.add(log);
    }

    /// 批量添加日志
    pub fn add_logs_batch(&self, agent_id: &str, logs: Vec<AgentLog>) {
        let mut caches = self.log_caches.write().unwrap();
        let cache = caches
            .entry(agent_id.to_string())
            .or_insert_with(|| LogCache::new(100));
        for log in logs {
            cache.add(log);
        }
    }

    /// 获取内存中的日志
    pub fn get_logs(&self, agent_id: &str) -> Vec<AgentLog> {
        let caches = self.log_caches.read().unwrap();
        caches
            .get(agent_id)
            .map(|cache| cache.get_all())
            .unwrap_or_default()
    }

    /// 清空日志缓存
    pub fn clear_logs(&self, agent_id: &str) {
        let mut caches = self.log_caches.write().unwrap();
        if let Some(cache) = caches.get_mut(agent_id) {
            cache.clear();
        }
    }

    /// 持久化日志到数据库
    pub fn persist_logs(&self, agent_id: &str) -> Result<usize, String> {
        let logs = self.get_logs(agent_id);
        if logs.is_empty() {
            return Ok(0);
        }

        let conn = get_connection().map_err(|e| e.to_string())?;
        let repo = crate::db::agent_logs::get_logs_repository(&conn);

        let count = repo.insert_batch(&logs).map_err(|e| e.to_string())?;
        self.clear_logs(agent_id);

        Ok(count)
    }

    // ==================== 追踪服务 ====================

    /// 添加追踪记录
    pub fn add_trace(&self, agent_id: &str, trace: AgentTrace) {
        let mut caches = self.trace_caches.write().unwrap();
        let cache = caches
            .entry(agent_id.to_string())
            .or_insert_with(TraceCache::new);
        cache.add(trace);
    }

    /// 获取追踪记录
    pub fn get_traces(&self, agent_id: &str) -> Vec<AgentTrace> {
        let caches = self.trace_caches.read().unwrap();
        caches
            .get(agent_id)
            .map(|cache| cache.get_all())
            .unwrap_or_default()
    }

    /// 创建思考链记录
    pub fn record_thought(
        &self,
        agent_id: &str,
        session_id: &str,
        content: &str,
        parent_id: Option<&str>,
    ) -> String {
        let id = format!("trace-{}", Uuid::new_v4());
        let trace = AgentTrace::new_thought(
            id.clone(),
            agent_id.to_string(),
            session_id.to_string(),
            content.to_string(),
            parent_id.map(|s| s.to_string()),
        );
        self.add_trace(agent_id, trace);
        id
    }

    /// 创建工具调用记录
    pub fn record_tool_call(
        &self,
        agent_id: &str,
        session_id: &str,
        tool_name: &str,
        parameters: serde_json::Value,
        parent_id: Option<&str>,
    ) -> String {
        let id = format!("trace-{}", Uuid::new_v4());
        let trace = AgentTrace::new_tool_call(
            id.clone(),
            agent_id.to_string(),
            session_id.to_string(),
            tool_name.to_string(),
            parameters,
            parent_id.map(|s| s.to_string()),
        );
        self.add_trace(agent_id, trace);
        id
    }

    /// 创建工具执行结果记录
    pub fn record_tool_result(
        &self,
        agent_id: &str,
        session_id: &str,
        success: bool,
        result: serde_json::Value,
        duration_ms: u64,
        parent_id: &str,
    ) {
        let trace = AgentTrace::new_tool_result(
            format!("trace-{}", Uuid::new_v4()),
            agent_id.to_string(),
            session_id.to_string(),
            success,
            result,
            duration_ms,
            parent_id.to_string(),
        );
        self.add_trace(agent_id, trace);
    }

    /// 创建决策记录
    pub fn record_decision(
        &self,
        agent_id: &str,
        session_id: &str,
        context: &str,
        decision: &str,
        reason: &str,
        parent_id: Option<&str>,
    ) {
        let trace = AgentTrace::new_decision(
            format!("trace-{}", Uuid::new_v4()),
            agent_id.to_string(),
            session_id.to_string(),
            context.to_string(),
            decision.to_string(),
            reason.to_string(),
            parent_id.map(|s| s.to_string()),
        );
        self.add_trace(agent_id, trace);
    }

    // ==================== 告警服务 ====================

    /// 获取告警配置
    pub fn get_alert_config(&self) -> AlertConfig {
        self.alert_config.read().unwrap().clone()
    }

    /// 更新告警配置
    pub fn update_alert_config(&self, config: AlertConfig) {
        *self.alert_config.write().unwrap() = config;
    }

    /// 创建告警
    pub fn create_alert(
        &self,
        agent_id: &str,
        level: &str,
        alert_type: &str,
        message: &str,
    ) -> String {
        let id = format!("alert-{}", Uuid::new_v4());
        let alert = AgentAlert::new(
            id.clone(),
            agent_id.to_string(),
            level.to_string(),
            alert_type.to_string(),
            message.to_string(),
        );

        let conn = get_connection().map_err(|e| {
            log::error!("Failed to create alert: {}", e);
            e.to_string()
        });

        if let Ok(conn) = conn {
            let repo = crate::db::agent_alerts::get_alerts_repository(&conn);
            if let Err(e) = repo.insert(&alert) {
                log::error!("Failed to insert alert: {}", e);
            }
        }

        id
    }

    /// 检测无响应告警
    pub fn check_no_response_alert(&self, agent_id: &str, last_update: &str) -> Option<AgentAlert> {
        let config = self.get_alert_config();
        if !config.enabled {
            return None;
        }

        let last_update_time = chrono::DateTime::parse_from_rfc3339(last_update).ok()?;
        let now = Utc::now();
        let duration = now.signed_duration_since(last_update_time);

        let (level, alert_type, message) = if duration.num_minutes() >= config.no_response_critical_minutes
        {
            (
                alert_level::CRITICAL,
                alert_type::NO_RESPONSE,
                format!("Agent has not responded for {} minutes", duration.num_minutes()),
            )
        } else if duration.num_minutes() >= config.no_response_warning_minutes {
            (
                alert_level::WARNING,
                alert_type::NO_RESPONSE,
                format!("Agent has not responded for {} minutes", duration.num_minutes()),
            )
        } else {
            return None;
        };

        Some(AgentAlert::new(
            format!("alert-{}", Uuid::new_v4()),
            agent_id.to_string(),
            level.to_string(),
            alert_type.to_string(),
            message,
        ))
    }

    /// 检测错误率告警
    pub fn check_error_rate_alert(&self, agent_id: &str, error_count: i64, duration_minutes: i64) -> Option<AgentAlert> {
        let config = self.get_alert_config();
        if !config.enabled {
            return None;
        }

        let error_rate = error_count / duration_minutes.max(1);

        let (level, message) = if error_rate >= config.error_rate_critical_per_minute {
            (
                alert_level::CRITICAL,
                format!("Error rate is {} errors/minute (critical)", error_rate),
            )
        } else if error_rate >= config.error_rate_warning_per_minute {
            (
                alert_level::WARNING,
                format!("Error rate is {} errors/minute (warning)", error_rate),
            )
        } else {
            return None;
        };

        Some(AgentAlert::new(
            format!("alert-{}", Uuid::new_v4()),
            agent_id.to_string(),
            level.to_string(),
            alert_type::ERROR_RATE.to_string(),
            message,
        ))
    }

    // ==================== 性能指标服务 ====================

    /// 更新性能指标
    pub fn update_metrics(
        &self,
        agent_id: &str,
        cpu_usage: f64,
        memory_usage_mb: f64,
        api_call_stats: Option<ApiCallStats>,
    ) {
        let metrics = PerformanceMetrics {
            agent_id: agent_id.to_string(),
            cpu_usage,
            memory_usage_mb,
            api_calls: api_call_stats.unwrap_or_default(),
            timestamp: Utc::now().to_rfc3339(),
        };

        let mut metrics_map = self.metrics.write().unwrap();
        metrics_map.insert(agent_id.to_string(), metrics);
    }

    /// 记录 API 调用
    pub fn record_api_call(&self, agent_id: &str, success: bool, duration_ms: u64) {
        let mut metrics_map = self.metrics.write().unwrap();
        if let Some(metrics) = metrics_map.get_mut(agent_id) {
            metrics.api_calls.record_call(success, duration_ms);
        } else {
            let mut stats = ApiCallStats::new();
            stats.record_call(success, duration_ms);
            let metrics = PerformanceMetrics {
                agent_id: agent_id.to_string(),
                cpu_usage: 0.0,
                memory_usage_mb: 0.0,
                api_calls: stats,
                timestamp: Utc::now().to_rfc3339(),
            };
            metrics_map.insert(agent_id.to_string(), metrics);
        }
    }

    /// 获取性能指标
    pub fn get_metrics(&self, agent_id: &str) -> Option<PerformanceMetrics> {
        let metrics = self.metrics.read().unwrap();
        metrics.get(agent_id).cloned()
    }
}

impl Default for ObservabilityService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_cache() {
        let mut cache = LogCache::new(3);
        cache.add(AgentLog::new(
            "log-1".to_string(),
            "agent-1".to_string(),
            "session-1".to_string(),
            "2024-01-01T00:00:00Z".to_string(),
            "info".to_string(),
            "test".to_string(),
            "Message 1".to_string(),
        ));
        cache.add(AgentLog::new(
            "log-2".to_string(),
            "agent-1".to_string(),
            "session-1".to_string(),
            "2024-01-01T00:00:01Z".to_string(),
            "info".to_string(),
            "test".to_string(),
            "Message 2".to_string(),
        ));

        assert_eq!(cache.logs.len(), 2);

        cache.add(AgentLog::new(
            "log-3".to_string(),
            "agent-1".to_string(),
            "session-1".to_string(),
            "2024-01-01T00:00:02Z".to_string(),
            "info".to_string(),
            "test".to_string(),
            "Message 3".to_string(),
        ));

        assert_eq!(cache.logs.len(), 3);

        cache.add(AgentLog::new(
            "log-4".to_string(),
            "agent-1".to_string(),
            "session-1".to_string(),
            "2024-01-01T00:00:03Z".to_string(),
            "info".to_string(),
            "test".to_string(),
            "Message 4".to_string(),
        ));

        assert_eq!(cache.logs.len(), 3);
        assert_eq!(cache.logs[0].message, "Message 2");
    }

    #[test]
    fn test_api_call_stats() {
        let mut stats = ApiCallStats::new();
        stats.record_call(true, 100);
        stats.record_call(true, 200);
        stats.record_call(false, 300);

        assert_eq!(stats.total_calls, 3);
        assert_eq!(stats.successful_calls, 2);
        assert_eq!(stats.failed_calls, 1);
        // For [100, 200, 300] with linear interpolation:
        // p50: index = 0.5 * 2 = 1 -> 200
        // p90: index = 0.9 * 2 = 1.8 -> truncated to 1 -> 200
        // p99: index = 0.99 * 2 = 1.98 -> truncated to 1 -> 200
        assert_eq!(stats.get_percentile(50.0), 200u64);
        assert_eq!(stats.get_percentile(90.0), 200u64);
        assert_eq!(stats.get_percentile(99.0), 200u64);
    }
}
