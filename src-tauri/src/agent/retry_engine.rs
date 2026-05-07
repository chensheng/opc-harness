use chrono::{Duration, Utc};
use rand::Rng;
use regex::Regex;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration as StdDuration;
use tokio::sync::RwLock;

/// 错误类型分类
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorType {
    /// 临时错误（可重试）
    Temporary,
    /// 永久错误（不可重试）
    Permanent,
}

/// 重试决策结果
#[derive(Debug, Clone, PartialEq)]
pub enum RetryDecision {
    /// 允许重试，附带下次重试时间
    Retry { next_retry_at: String },
    /// 终止重试
    Abort { reason: String },
}

/// 指数退避策略配置
#[derive(Debug, Clone)]
pub struct BackoffConfig {
    /// 基础延迟时间（秒）
    pub base_delay_seconds: u64,
    /// 最大延迟时间（秒）
    pub max_delay_seconds: u64,
    /// 随机抖动范围（百分比，0.1 = 10%）
    pub jitter_ratio: f64,
}

impl Default for BackoffConfig {
    fn default() -> Self {
        Self {
            base_delay_seconds: 60,   // 1 分钟
            max_delay_seconds: 3600,  // 1 小时
            jitter_ratio: 0.1,        // ±10%
        }
    }
}

/// 错误分类器
pub struct ErrorClassifier {
    /// 临时错误模式（正则表达式）
    temporary_patterns: Vec<Regex>,
    /// 永久错误模式（正则表达式）
    permanent_patterns: Vec<Regex>,
}

impl ErrorClassifier {
    pub fn new() -> Self {
        // 临时错误模式
        let temporary_patterns = vec![
            Regex::new(r"(?i)timeout").unwrap(),
            Regex::new(r"(?i)connection refused").unwrap(),
            Regex::new(r"(?i)network error").unwrap(),
            Regex::new(r"(?i)rate limit").unwrap(),
            Regex::new(r"(?i)429 Too Many Requests").unwrap(),
            Regex::new(r"(?i)503 Service Unavailable").unwrap(),
            Regex::new(r"(?i)temporary failure").unwrap(),
        ];

        // 永久错误模式
        let permanent_patterns = vec![
            Regex::new(r"(?i)syntax error").unwrap(),
            Regex::new(r"(?i)compilation failed").unwrap(),
            Regex::new(r"(?i)type error").unwrap(),
            Regex::new(r"(?i)module not found").unwrap(),
            Regex::new(r"(?i)dependency resolution failed").unwrap(),
            Regex::new(r"(?i)permission denied").unwrap(),
            Regex::new(r"(?i)unauthorized").unwrap(),
            Regex::new(r"(?i)forbidden").unwrap(),
            Regex::new(r"(?i)invalid input").unwrap(),
            Regex::new(r"(?i)data validation failed").unwrap(),
        ];

        Self {
            temporary_patterns,
            permanent_patterns,
        }
    }

    /// 分类错误类型
    pub fn classify_error(&self, error_message: &str) -> ErrorType {
        // 先检查是否为临时错误
        for pattern in &self.temporary_patterns {
            if pattern.is_match(error_message) {
                log::info!(
                    "[ErrorClassifier] Classified as TEMPORARY: {}",
                    error_message
                );
                return ErrorType::Temporary;
            }
        }

        // 再检查是否为永久错误
        for pattern in &self.permanent_patterns {
            if pattern.is_match(error_message) {
                log::info!(
                    "[ErrorClassifier] Classified as PERMANENT: {}",
                    error_message
                );
                return ErrorType::Permanent;
            }
        }

        // 未知错误默认为永久错误（安全策略）
        log::warn!(
            "[ErrorClassifier] Unknown error type, defaulting to PERMANENT: {}",
            error_message
        );
        ErrorType::Permanent
    }
}

/// 指数退避计算器
pub struct BackoffCalculator {
    config: BackoffConfig,
}

impl BackoffCalculator {
    pub fn new(config: BackoffConfig) -> Self {
        Self { config }
    }

    /// 计算下次重试的延迟时间（秒）
    pub fn calculate_delay(&self, retry_count: u32) -> u64 {
        // 指数退避公式：base_delay * 2^retry_count
        let exponential_delay = self.config.base_delay_seconds * (2_u64.pow(retry_count));

        // 限制在最大值以内
        let capped_delay = exponential_delay.min(self.config.max_delay_seconds);

        // 添加随机抖动
        let jitter = self.calculate_jitter(capped_delay);

        capped_delay + jitter
    }

    /// 计算随机抖动
    fn calculate_jitter(&self, delay: u64) -> u64 {
        let mut rng = rand::thread_rng();
        let jitter_range = (delay as f64 * self.config.jitter_ratio) as u64;
        
        if jitter_range == 0 {
            return 0;
        }

        // 生成 -jitter_range 到 +jitter_range 之间的随机值
        let jitter = rng.gen_range(0..jitter_range * 2) as i64 - jitter_range as i64;
        
        // 确保不为负数
        jitter.max(0) as u64
    }

    /// 计算下次重试的时间戳（RFC3339 格式）
    pub fn calculate_next_retry_at(&self, retry_count: u32) -> String {
        let delay_seconds = self.calculate_delay(retry_count);
        let now = Utc::now();
        let next_retry = now + Duration::seconds(delay_seconds as i64);
        next_retry.to_rfc3339()
    }
}

/// 重试引擎
pub struct RetryEngine {
    classifier: ErrorClassifier,
    backoff_calculator: BackoffCalculator,
    max_retries: u32,
}

impl RetryEngine {
    pub fn new(max_retries: u32, backoff_config: BackoffConfig) -> Self {
        Self {
            classifier: ErrorClassifier::new(),
            backoff_calculator: BackoffCalculator::new(backoff_config),
            max_retries,
        }
    }

    /// 决定是否应该重试
    pub fn should_retry(
        &self,
        current_retry_count: u32,
        error_message: &str,
    ) -> RetryDecision {
        // 1. 检查是否超过最大重试次数
        if current_retry_count >= self.max_retries {
            log::warn!(
                "[RetryEngine] Max retries ({}) exceeded for error: {}",
                self.max_retries,
                error_message
            );
            return RetryDecision::Abort {
                reason: format!("Exceeded maximum retry count ({})", self.max_retries),
            };
        }

        // 2. 分类错误类型
        let error_type = self.classifier.classify_error(error_message);

        // 3. 根据错误类型决定
        match error_type {
            ErrorType::Temporary => {
                // 临时错误：计算下次重试时间
                let next_retry_at = self.backoff_calculator.calculate_next_retry_at(current_retry_count);
                log::info!(
                    "[RetryEngine] Decided to RETRY (temporary error). Next retry at: {}",
                    next_retry_at
                );
                RetryDecision::Retry { next_retry_at }
            }
            ErrorType::Permanent => {
                // 永久错误：直接终止
                log::info!(
                    "[RetryEngine] Decided to ABORT (permanent error): {}",
                    error_message
                );
                RetryDecision::Abort {
                    reason: format!("Permanent error: {}", error_message),
                }
            }
        }
    }

    /// 获取最大重试次数
    pub fn get_max_retries(&self) -> u32 {
        self.max_retries
    }
}

/// 重试调度器配置
#[derive(Debug, Clone)]
pub struct SchedulerConfig {
    /// 检查间隔（秒）
    pub check_interval_seconds: u64,
    /// 最大并发重试数量
    pub max_concurrent_retries: usize,
}

impl Default for SchedulerConfig {
    fn default() -> Self {
        Self {
            check_interval_seconds: 30,  // 每 30 秒检查一次
            max_concurrent_retries: 3,   // 最多同时重试 3 个
        }
    }
}

/// 调度器状态
#[derive(Debug, Clone)]
pub struct SchedulerStatus {
    /// 是否正在运行
    pub is_running: bool,
    /// 活跃重试数量
    pub active_retry_count: usize,
    /// 活跃重试列表 (story_id -> agent_id)
    pub active_retries: HashMap<String, String>,
    /// 最后扫描时间
    pub last_scan_at: Option<String>,
}

/// 重试调度器
pub struct RetryScheduler {
    config: SchedulerConfig,
    active_retries: HashMap<String, String>, // story_id -> agent_id
    last_scan_at: Option<String>,
    is_running: bool,  // 运行状态标志
}

impl RetryScheduler {
    pub fn new(config: SchedulerConfig) -> Self {
        Self {
            config,
            active_retries: HashMap::new(),
            last_scan_at: None,
            is_running: false,
        }
    }

    /// 检查是否有待重试的故事
    pub fn has_pending_retries(&self) -> bool {
        !self.active_retries.is_empty()
    }

    /// 获取当前活跃的重试数量
    pub fn get_active_retry_count(&self) -> usize {
        self.active_retries.len()
    }

    /// 检查是否可以启动新的重试
    pub fn can_start_new_retry(&self) -> bool {
        self.active_retries.len() < self.config.max_concurrent_retries
    }

    /// 注册一个重试任务
    pub fn register_retry(&mut self, story_id: String, agent_id: String) -> bool {
        if !self.can_start_new_retry() {
            log::warn!(
                "[RetryScheduler] Cannot register retry for {}: max concurrent retries reached",
                story_id
            );
            return false;
        }

        let story_id_clone = story_id.clone();
        let agent_id_clone = agent_id.clone();
        self.active_retries.insert(story_id, agent_id);
        log::info!(
            "[RetryScheduler] Registered retry for story {} with agent {}",
            story_id_clone,
            agent_id_clone
        );
        true
    }

    /// 完成一个重试任务
    pub fn complete_retry(&mut self, story_id: &str) {
        if let Some(agent_id) = self.active_retries.remove(story_id) {
            log::info!(
                "[RetryScheduler] Completed retry for story {} (agent {})",
                story_id,
                agent_id
            );
        }
    }

    /// 获取检查间隔（秒）
    pub fn get_check_interval(&self) -> u64 {
        self.config.check_interval_seconds
    }

    /// 获取调度器状态
    pub fn get_status(&self) -> SchedulerStatus {
        SchedulerStatus {
            is_running: self.is_running,
            active_retry_count: self.active_retries.len(),
            active_retries: self.active_retries.clone(),
            last_scan_at: self.last_scan_at.clone(),
        }
    }

    /// 检查调度器是否正在运行
    pub fn is_running(&self) -> bool {
        self.is_running
    }

    /// 更新重试结果并发送 WebSocket 通知
    pub async fn update_retry_result(
        &mut self,
        story_id: &str,
        story_number: &str,
        success: bool,
        error_message: Option<&str>,
        websocket_manager: &Arc<RwLock<crate::agent::websocket_manager::WebSocketManager>>,
    ) -> Result<(), String> {
        let conn = crate::db::get_connection()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;

        let now = chrono::Utc::now().to_rfc3339();
        let result = if success { "success" } else { "failed" };

        // 1. 查找最新的 pending 重试记录
        let histories = crate::db::get_user_story_retry_history(&conn, story_id)
            .map_err(|e| format!("Failed to query retry history: {}", e))?;

        if let Some(latest_history) = histories.iter().find(|h| h.result.as_deref() == Some("pending")) {
            // 2. 更新重试历史记录
            crate::db::update_retry_history_result(&conn, &latest_history.id, result, &now)
                .map_err(|e| format!("Failed to update retry history: {}", e))?;

            // 3. 如果失败，更新 Story 的 error_message
            if !success {
                if let Some(err_msg) = error_message {
                    let _ = crate::db::fail_user_story(&conn, story_id, err_msg);
                }
            } else {
                // 如果成功，标记 Story 为 completed
                let _ = crate::db::complete_user_story(&conn, story_id);
            }

            // 4. 发送 WebSocket 通知
            {
                let ws_manager = websocket_manager.read().await;
                let message = if success {
                    format!("✅ Story {} 重试成功", story_number)
                } else {
                    format!("❌ Story {} 重试失败: {}", story_number, error_message.unwrap_or("未知错误"))
                };
                
                if let Err(e) = ws_manager.send_log(
                    &story_id.to_string(),
                    if success { "success" } else { "error" },
                    &message,
                    Some("RetryScheduler"),
                ).await {
                    log::warn!("[RetryScheduler] Failed to send WebSocket notification: {}", e);
                }
            }

            // 5. 从 active_retries 中移除
            self.complete_retry(story_id);

            log::info!(
                "[RetryScheduler] Updated retry result for story {}: {}",
                story_number,
                result
            );
        } else {
            log::warn!(
                "[RetryScheduler] No pending retry history found for story {}",
                story_id
            );
        }

        Ok(())
    }

    /// 运行调度器主循环
    pub async fn run(
        &mut self,
        project_id: String,
        websocket_manager: Arc<RwLock<crate::agent::websocket_manager::WebSocketManager>>,
    ) {
        self.is_running = true;
        log::info!("[RetryScheduler] Started for project: {}", project_id);

        let mut interval = tokio::time::interval(StdDuration::from_secs(self.config.check_interval_seconds));
        
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    // 执行单次扫描
                    if let Err(e) = self.scan_and_trigger(&project_id, &websocket_manager).await {
                        log::error!("[RetryScheduler] Scan failed: {}", e);
                    }
                }
                _ = tokio::signal::ctrl_c() => {
                    log::info!("[RetryScheduler] Received shutdown signal, waiting for active retries...");
                    
                    // 等待所有活跃的重试任务完成
                    while !self.active_retries.is_empty() {
                        log::info!(
                            "[RetryScheduler] Waiting for {} active retries to complete...",
                            self.active_retries.len()
                        );
                        tokio::time::sleep(StdDuration::from_secs(5)).await;
                    }
                    
                    log::info!("[RetryScheduler] All active retries completed, shutting down");
                    self.is_running = false;
                    break;
                }
            }
        }
    }

    /// 执行单次扫描并触发待重试任务
    async fn scan_and_trigger(
        &mut self,
        project_id: &str,
        websocket_manager: &Arc<RwLock<crate::agent::websocket_manager::WebSocketManager>>,
    ) -> Result<(), String> {
        let scan_start = std::time::Instant::now();
        log::debug!("[RetryScheduler] Scanning for pending retries...");

        // 更新最后扫描时间
        self.last_scan_at = Some(chrono::Utc::now().to_rfc3339());

        // 1. 查询待重试队列
        let conn = crate::db::get_connection()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;

        let pending_stories = crate::db::get_pending_retries(
            &conn,
            self.config.max_concurrent_retries,
        ).map_err(|e| format!("Failed to query pending retries: {}", e))?;

        if pending_stories.is_empty() {
            log::debug!("[RetryScheduler] No pending retries found");
            return Ok(());
        }

        log::info!(
            "[RetryScheduler] Found {} pending retries",
            pending_stories.len()
        );

        let total_found = pending_stories.len();
        let mut triggered_count = 0;

        // 2. 遍历待重试故事，触发重试
        for story in pending_stories {
            // 检查并发限制
            if !self.can_start_new_retry() {
                log::warn!(
                    "[RetryScheduler] Max concurrent retries reached ({}), skipping remaining stories",
                    self.config.max_concurrent_retries
                );
                break;
            }

            // 触发重试
            if let Err(e) = self.trigger_retry(&story, websocket_manager).await {
                log::error!(
                    "[RetryScheduler] Failed to trigger retry for story {}: {}",
                    story.id,
                    e
                );
                // 继续处理下一个故事
                continue;
            }
            triggered_count += 1;
        }

        let scan_duration = scan_start.elapsed();
        log::info!(
            "[RetryScheduler] Scan completed in {:?}: found {} stories, triggered {} retries",
            scan_duration,
            total_found,
            triggered_count
        );

        Ok(())
    }

    /// 触发单个故事的重试
    async fn trigger_retry(
        &mut self,
        story: &crate::models::UserStory,
        websocket_manager: &Arc<RwLock<crate::agent::websocket_manager::WebSocketManager>>,
    ) -> Result<(), String> {
        log::info!(
            "[RetryScheduler] Triggering retry for story {} (attempt {})",
            story.story_number,
            story.retry_count + 1
        );

        let conn = crate::db::get_connection()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;

        // 1. 更新 Story 状态为 in_progress
        let now = chrono::Utc::now().to_rfc3339();
        crate::db::update_user_story_status(&conn, &story.id, "in_progress")
            .map_err(|e| format!("Failed to update story status: {}", e))?;

        // 2. 创建重试历史记录（result='pending'）
        let retry_history = crate::models::UserStoryRetryHistory {
            id: format!("retry_{}_{}", story.id, now.replace(':', "-")),
            user_story_id: story.id.clone(),
            retry_number: (story.retry_count + 1) as i32,
            triggered_at: now.clone(),
            error_message: None,
            error_type: None,
            decision: "retry".to_string(),
            next_retry_at: None,
            completed_at: None,
            result: Some("pending".to_string()),
            created_at: now.clone(),
        };

        crate::db::create_retry_history_record(&conn, &retry_history)
            .map_err(|e| format!("Failed to create retry history: {}", e))?;

        // 3. 通过 WebSocket 发送通知（使用 story.id 作为 session_id）
        {
            let ws_manager = websocket_manager.read().await;
            let session_id = story.id.clone();
            if let Err(e) = ws_manager.send_log(
                &session_id,
                "info",
                &format!("🔄 开始重试 Story {}（第 {} 次）", story.story_number, story.retry_count + 1),
                Some("RetryScheduler"),
            ).await {
                log::warn!("[RetryScheduler] Failed to send WebSocket notification: {}", e);
            }
        }
        
        // 4. 注册到 active_retries
        let agent_id = format!("retry-agent-{}", story.id);
        if !self.register_retry(story.id.clone(), agent_id.clone()) {
            return Err(format!(
                "Failed to register retry for story {}: max concurrent retries reached",
                story.story_number
            ));
        }

        // 5. TODO: 调用 execute_user_story 启动 Agent
        // 
        // 完整的实现需要：
        // 1. 获取 daemon_manager 和 worktree_manager 引用
        // 2. 调用 start_coding_agent() 方法
        // 3. 传递必要的参数（project_id, session_id, story 等）
        // 4. 处理 Agent 执行结果并更新重试历史
        //
        // 当前简化方案：
        // - 将 Story 状态设置为 in_progress
        // - 下一个 Agent Loop 周期会自动拾取并执行
        // - 或者通过 WebSocket 通知前端手动触发
        
        log::info!(
            "[RetryScheduler] Registered story {} for retry execution (agent_id: {})",
            story.story_number,
            agent_id
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_classifier_temporary() {
        let classifier = ErrorClassifier::new();
        
        assert_eq!(
            classifier.classify_error("Connection timeout occurred"),
            ErrorType::Temporary
        );
        assert_eq!(
            classifier.classify_error("Rate limit exceeded: 429 Too Many Requests"),
            ErrorType::Temporary
        );
        assert_eq!(
            classifier.classify_error("503 Service Unavailable"),
            ErrorType::Temporary
        );
    }

    #[test]
    fn test_error_classifier_permanent() {
        let classifier = ErrorClassifier::new();
        
        assert_eq!(
            classifier.classify_error("Syntax error in line 42"),
            ErrorType::Permanent
        );
        assert_eq!(
            classifier.classify_error("Module not found: ./utils"),
            ErrorType::Permanent
        );
        assert_eq!(
            classifier.classify_error("Permission denied: /etc/config"),
            ErrorType::Permanent
        );
    }

    #[test]
    fn test_backoff_calculator() {
        let config = BackoffConfig {
            base_delay_seconds: 60,
            max_delay_seconds: 3600,
            jitter_ratio: 0.0, // 禁用抖动以便测试
        };
        let calculator = BackoffCalculator::new(config);

        // 第 0 次重试（首次）：60 秒
        assert_eq!(calculator.calculate_delay(0), 60);

        // 第 1 次重试：120 秒
        assert_eq!(calculator.calculate_delay(1), 120);

        // 第 2 次重试：240 秒
        assert_eq!(calculator.calculate_delay(2), 240);

        // 第 10 次重试：应该被限制在 3600 秒
        assert_eq!(calculator.calculate_delay(10), 3600);
    }

    #[test]
    fn test_retry_engine_max_retries_exceeded() {
        let engine = RetryEngine::new(3, BackoffConfig::default());

        // 超过最大重试次数
        let decision = engine.should_retry(3, "Some error");
        
        match decision {
            RetryDecision::Abort { reason } => {
                assert!(reason.contains("Exceeded maximum retry count"));
            }
            _ => panic!("Expected Abort decision"),
        }
    }

    #[test]
    fn test_retry_engine_temporary_error() {
        let engine = RetryEngine::new(3, BackoffConfig::default());

        // 临时错误应该重试
        let decision = engine.should_retry(0, "Connection timeout");
        
        match decision {
            RetryDecision::Retry { next_retry_at } => {
                assert!(!next_retry_at.is_empty());
            }
            _ => panic!("Expected Retry decision"),
        }
    }

    #[test]
    fn test_retry_engine_permanent_error() {
        let engine = RetryEngine::new(3, BackoffConfig::default());

        // 永久错误应该终止
        let decision = engine.should_retry(0, "Syntax error in code");
        
        match decision {
            RetryDecision::Abort { reason } => {
                assert!(reason.contains("Permanent error"));
            }
            _ => panic!("Expected Abort decision"),
        }
    }

    #[test]
    fn test_scheduler_concurrency_limit() {
        let config = SchedulerConfig {
            check_interval_seconds: 30,
            max_concurrent_retries: 2,
        };
        let mut scheduler = RetryScheduler::new(config);

        // 可以注册前两个
        assert!(scheduler.register_retry("story1".to_string(), "agent1".to_string()));
        assert!(scheduler.register_retry("story2".to_string(), "agent2".to_string()));

        // 第三个应该失败
        assert!(!scheduler.register_retry("story3".to_string(), "agent3".to_string()));

        // 完成一个后可以注册第三个
        scheduler.complete_retry("story1");
        assert!(scheduler.register_retry("story3".to_string(), "agent3".to_string()));
    }

    #[test]
    fn test_error_classifier_network_timeout() {
        let classifier = ErrorClassifier::new();

        // 测试网络超时错误
        let error_msg = "Network timeout after 30000ms";
        let error_type = classifier.classify_error(error_msg);
        assert_eq!(error_type, ErrorType::Temporary);

        // 测试连接超时(使用正确的关键词)
        let error_msg2 = "Connection timeout";
        let error_type2 = classifier.classify_error(error_msg2);
        assert_eq!(error_type2, ErrorType::Temporary);
    }

    #[test]
    fn test_error_classifier_compilation_error() {
        let classifier = ErrorClassifier::new();

        // 测试编译错误(应该是永久错误)
        let error_msg = "error[E0308]: mismatched types";
        let error_type = classifier.classify_error(error_msg);
        assert_eq!(error_type, ErrorType::Permanent);

        // 测试语法错误
        let error_msg2 = "syntax error: unexpected token";
        let error_type2 = classifier.classify_error(error_msg2);
        assert_eq!(error_type2, ErrorType::Permanent);
    }

    #[test]
    fn test_error_classifier_rate_limit() {
        let classifier = ErrorClassifier::new();

        // 测试速率限制(应该是临时错误)
        let error_msg = "Rate limit exceeded: 429 Too Many Requests";
        let error_type = classifier.classify_error(error_msg);
        assert_eq!(error_type, ErrorType::Temporary);

        // 测试 API 限流
        let error_msg2 = "API rate limit reached, please retry after 60 seconds";
        let error_type2 = classifier.classify_error(error_msg2);
        assert_eq!(error_type2, ErrorType::Temporary);
    }

    #[test]
    fn test_retry_scheduler_can_start_new_retry() {
        let config = SchedulerConfig {
            check_interval_seconds: 30,
            max_concurrent_retries: 1,
        };
        let mut scheduler = RetryScheduler::new(config);

        // 初始状态可以开始
        assert!(scheduler.can_start_new_retry());

        // 注册一个后不能再开始
        scheduler.register_retry("story1".to_string(), "agent1".to_string());
        assert!(!scheduler.can_start_new_retry());

        // 完成后又可以开始
        scheduler.complete_retry("story1");
        assert!(scheduler.can_start_new_retry());
    }

    #[test]
    fn test_retry_scheduler_concurrent_limit() {
        let config = SchedulerConfig {
            check_interval_seconds: 30,
            max_concurrent_retries: 3,
        };
        let mut scheduler = RetryScheduler::new(config);

        // 可以注册前三个
        assert!(scheduler.register_retry("story1".to_string(), "agent1".to_string()));
        assert!(scheduler.register_retry("story2".to_string(), "agent2".to_string()));
        assert!(scheduler.register_retry("story3".to_string(), "agent3".to_string()));

        // 第四个应该失败
        assert!(!scheduler.register_retry("story4".to_string(), "agent4".to_string()));

        // 检查活跃数量
        assert_eq!(scheduler.get_active_retry_count(), 3);

        // 完成一个后可以注册第四个
        scheduler.complete_retry("story2");
        assert!(scheduler.register_retry("story4".to_string(), "agent4".to_string()));
        assert_eq!(scheduler.get_active_retry_count(), 3);
    }

    #[test]
    fn test_scheduler_status() {
        let config = SchedulerConfig::default();
        let mut scheduler = RetryScheduler::new(config);

        // 初始状态（未启动）
        let status = scheduler.get_status();
        assert!(!status.is_running);  // 初始为 false
        assert_eq!(status.active_retry_count, 0);
        assert!(status.last_scan_at.is_none());

        // 模拟启动
        scheduler.is_running = true;
        let status = scheduler.get_status();
        assert!(status.is_running);

        // 注册一些重试任务
        scheduler.register_retry("story1".to_string(), "agent1".to_string());
        scheduler.register_retry("story2".to_string(), "agent2".to_string());

        let status = scheduler.get_status();
        assert_eq!(status.active_retry_count, 2);
        assert_eq!(status.active_retries.len(), 2);
    }

    #[test]
    fn test_scheduler_has_pending_retries() {
        let config = SchedulerConfig::default();
        let mut scheduler = RetryScheduler::new(config);

        // 初始没有待重试
        assert!(!scheduler.has_pending_retries());

        // 注册后有
        scheduler.register_retry("story1".to_string(), "agent1".to_string());
        assert!(scheduler.has_pending_retries());

        // 完成后又没有
        scheduler.complete_retry("story1");
        assert!(!scheduler.has_pending_retries());
    }

    #[test]
    fn test_error_classifier_temporary_errors() {
        let classifier = ErrorClassifier::new();

        // 测试临时错误
        assert_eq!(classifier.classify_error("Connection timeout"), ErrorType::Temporary);
        assert_eq!(classifier.classify_error("Network error occurred"), ErrorType::Temporary);
        assert_eq!(classifier.classify_error("Rate limit exceeded"), ErrorType::Temporary);
        assert_eq!(classifier.classify_error("429 Too Many Requests"), ErrorType::Temporary);
        assert_eq!(classifier.classify_error("503 Service Unavailable"), ErrorType::Temporary);
    }

    #[test]
    fn test_error_classifier_permanent_errors() {
        let classifier = ErrorClassifier::new();

        // 测试永久错误
        assert_eq!(classifier.classify_error("Syntax error in line 10"), ErrorType::Permanent);
        assert_eq!(classifier.classify_error("Compilation failed"), ErrorType::Permanent);
        assert_eq!(classifier.classify_error("Type error: expected String"), ErrorType::Permanent);
        assert_eq!(classifier.classify_error("Module not found: foo"), ErrorType::Permanent);
        assert_eq!(classifier.classify_error("Permission denied"), ErrorType::Permanent);
    }

    #[test]
    fn test_backoff_calculator_exponential_growth() {
        let config = BackoffConfig {
            base_delay_seconds: 60,
            max_delay_seconds: 3600,
            jitter_ratio: 0.0, // 禁用随机性以便测试
        };
        let calculator = BackoffCalculator::new(config);

        // 验证指数增长（返回的是秒数 u64）
        let delay_0 = calculator.calculate_delay(0);
        let delay_1 = calculator.calculate_delay(1);
        let delay_2 = calculator.calculate_delay(2);
        let delay_3 = calculator.calculate_delay(3);

        assert_eq!(delay_0, 60);   // 60s
        assert_eq!(delay_1, 120);  // 120s
        assert_eq!(delay_2, 240);  // 240s
        assert_eq!(delay_3, 480);  // 480s
    }

    #[test]
    fn test_backoff_calculator_max_delay_cap() {
        let config = BackoffConfig {
            base_delay_seconds: 60,
            max_delay_seconds: 3600,
            jitter_ratio: 0.0,
        };
        let calculator = BackoffCalculator::new(config);

        // 第 10 次重试应该被限制在最大值
        let delay_10 = calculator.calculate_delay(10);
        assert_eq!(delay_10, 3600); //  capped at max

        // 第 20 次重试也应该被限制
        let delay_20 = calculator.calculate_delay(20);
        assert_eq!(delay_20, 3600);
    }

    #[test]
    fn test_retry_engine_decision_logic() {
        let backoff_config = BackoffConfig::default();
        let engine = RetryEngine::new(3, backoff_config);

        // 第 1 次重试，临时错误 -> 应该重试
        let decision = engine.should_retry(0, "Network timeout");
        assert!(matches!(decision, RetryDecision::Retry { .. }));

        // 第 2 次重试，临时错误 -> 应该重试
        let decision = engine.should_retry(1, "Connection refused");
        assert!(matches!(decision, RetryDecision::Retry { .. }));

        // 第 3 次重试，临时错误 -> 应该重试
        let decision = engine.should_retry(2, "Rate limit");
        assert!(matches!(decision, RetryDecision::Retry { .. }));

        // 第 4 次重试（超过最大次数）-> 应该终止
        let decision = engine.should_retry(3, "Network timeout");
        assert!(matches!(decision, RetryDecision::Abort { .. }));

        // 永久错误 -> 立即终止
        let decision = engine.should_retry(0, "Syntax error");
        assert!(matches!(decision, RetryDecision::Abort { .. }));
    }

    #[test]
    fn test_scheduler_config_defaults() {
        let config = SchedulerConfig::default();
        assert_eq!(config.check_interval_seconds, 30);
        assert_eq!(config.max_concurrent_retries, 3);
    }

    #[test]
    fn test_backoff_config_defaults() {
        let config = BackoffConfig::default();
        assert_eq!(config.base_delay_seconds, 60);
        assert_eq!(config.max_delay_seconds, 3600);
        assert!((config.jitter_ratio - 0.1).abs() < f64::EPSILON);
    }
}
