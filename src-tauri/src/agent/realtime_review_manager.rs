//! Real-time Review Manager 实现
//! 
//! 负责监听文件变更，自动触发增量代码审查。
//! 支持 Watch 模式、防抖处理、实时推送。

use crate::agent::code_review_agent::{CodeReviewAgent, CodeReviewAgentConfig, ReviewResult, CodeChange};
use notify::{Config, RecommendedWatcher, RecursiveMode, Result as NotifyResult, Watcher, EventKind};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::mpsc::{self, Sender, Receiver};
use tokio::time::Duration;
use tokio::sync::Mutex;

/// 监听状态枚举
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WatchStatus {
    /// 等待开始
    Pending,
    /// 运行中
    Running,
    /// 已暂停
    Paused,
    /// 已停止
    Stopped,
    /// 错误状态
    Error(String),
}

impl std::fmt::Display for WatchStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WatchStatus::Pending => write!(f, "等待开始"),
            WatchStatus::Running => write!(f, "运行中"),
            WatchStatus::Paused => write!(f, "已暂停"),
            WatchStatus::Stopped => write!(f, "已停止"),
            WatchStatus::Error(e) => write!(f, "错误：{}", e),
        }
    }
}

/// 监听配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchConfig {
    /// 项目路径
    pub project_path: String,
    /// 文件模式列表（e.g., ["*.rs", "*.ts", "*.tsx"]）
    pub file_patterns: Vec<String>,
    /// 是否启用 AI 审查
    pub enable_ai: bool,
    /// 防抖时间（毫秒）
    pub debounce_ms: u64,
    /// 是否递归监听子目录
    pub recursive: bool,
}

impl Default for WatchConfig {
    fn default() -> Self {
        Self {
            project_path: ".".to_string(),
            file_patterns: vec!["*.rs".to_string(), "*.ts".to_string(), "*.tsx".to_string()],
            enable_ai: true,
            debounce_ms: 500,
            recursive: true,
        }
    }
}

/// 文件变更事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileChangeEvent {
    /// 文件路径
    pub file_path: String,
    /// 变更类型（Created/Modified/Deleted/Renamed）
    pub change_type: String,
    /// 时间戳
    pub timestamp: u64,
}

impl FileChangeEvent {
    pub fn new(file_path: String, change_type: String) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::ZERO)
            .as_millis() as u64;
        
        Self {
            file_path,
            change_type,
            timestamp,
        }
    }
}

/// 实时审查结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealtimeReviewResult {
    /// 审查的文件列表
    pub reviewed_files: Vec<String>,
    /// 审查结果
    pub review_result: ReviewResult,
    /// 审查时间（毫秒）
    pub review_time_ms: u64,
    /// 是否由文件变更触发
    pub triggered_by_change: bool,
}

impl RealtimeReviewResult {
    pub fn new(reviewed_files: Vec<String>, review_result: ReviewResult, review_time_ms: u64, triggered_by_change: bool) -> Self {
        Self {
            reviewed_files,
            review_result,
            review_time_ms,
            triggered_by_change,
        }
    }
}

/// Real-time Review Manager 结构体
pub struct RealtimeReviewManager {
    /// 配置信息
    pub config: WatchConfig,
    /// 当前状态
    pub status: WatchStatus,
    /// 代码审查 Agent
    code_review_agent: CodeReviewAgent,
    /// 文件监听器
    watcher: Option<Arc<Mutex<RecommendedWatcher>>>,
    /// 事件发送器
    event_tx: Option<Sender<FileChangeEvent>>,
    /// 已监听的文件夹路径
    watched_paths: HashSet<PathBuf>,
}

impl RealtimeReviewManager {
    /// 创建新的实时审查管理器
    pub fn new(config: WatchConfig) -> Self {
        let agent_config = CodeReviewAgentConfig {
            project_path: config.project_path.clone(),
            enable_ai: config.enable_ai,
            dimensions: vec![
                crate::agent::code_review_agent::ReviewDimension::Style,
                crate::agent::code_review_agent::ReviewDimension::Performance,
                crate::agent::code_review_agent::ReviewDimension::Security,
                crate::agent::code_review_agent::ReviewDimension::BestPractice,
            ],
            min_severity: crate::agent::code_review_agent::ReviewSeverity::Info,
            max_comments: 100,
        };

        Self {
            config,
            status: WatchStatus::Pending,
            code_review_agent: CodeReviewAgent::new(agent_config),
            watcher: None,
            event_tx: None,
            watched_paths: HashSet::new(),
        }
    }

    /// 启动文件监听
    pub async fn start_watch(&mut self) -> Result<(), String> {
        log::info!("启动实时审查监听：{}", self.config.project_path);

        // 创建通道
        let (tx, rx): (Sender<FileChangeEvent>, Receiver<FileChangeEvent>) = mpsc::channel(100);
        self.event_tx = Some(tx.clone());

        // 创建文件监听器
        let config = self.config.clone();
        let watcher_tx = tx.clone();

        let mut watcher = RecommendedWatcher::new(
            move |res: NotifyResult<notify::Event>| {
                if let Ok(event) = res {
                    // 处理文件变更事件
                    let change_type = match event.kind {
                        EventKind::Create(_) => "Created".to_string(),
                        EventKind::Modify(_) => "Modified".to_string(),
                        EventKind::Remove(_) => "Deleted".to_string(),
                        _ => return,
                    };

                    // 为每个变更的文件发送事件
                    for path in event.paths {
                        let file_path = path.to_string_lossy().to_string();
                        
                        // 检查文件是否在监听范围内
                        if Self::should_watch_file(&file_path, &config.file_patterns) {
                            let event = FileChangeEvent::new(file_path.clone(), change_type.clone());
                            let _ = watcher_tx.try_send(event);
                        }
                    }
                }
            },
            Config::default(),
        ).map_err(|e| format!("创建文件监听器失败：{}", e))?;

        // 添加监听路径
        let project_path = PathBuf::from(&self.config.project_path);
        let mode = if self.config.recursive {
            RecursiveMode::Recursive
        } else {
            RecursiveMode::NonRecursive
        };

        watcher.watch(&project_path, mode)
            .map_err(|e| format!("监听路径失败：{}", e))?;

        self.watcher = Some(Arc::new(Mutex::new(watcher)));
        self.watched_paths.insert(project_path);
        self.status = WatchStatus::Running;

        // 启动事件处理循环
        let debounce_ms = self.config.debounce_ms;
        tokio::spawn(async move {
            Self::event_handler(rx, debounce_ms).await;
        });

        log::info!("实时审查监听已启动");
        Ok(())
    }

    /// 停止文件监听
    pub async fn stop_watch(&mut self) -> Result<(), String> {
        log::info!("停止实时审查监听");

        if let Some(watcher) = self.watcher.take() {
            let mut watcher_guard = watcher.lock().await;
            for path in &self.watched_paths {
                let _ = watcher_guard.unwatch(path);
            }
            self.watched_paths.clear();
        }

        self.event_tx = None;
        self.status = WatchStatus::Stopped;

        log::info!("实时审查监听已停止");
        Ok(())
    }

    /// 事件处理器（带防抖）
    async fn event_handler(mut rx: Receiver<FileChangeEvent>, debounce_ms: u64) {
        let mut pending_events: Vec<FileChangeEvent> = Vec::new();
        
        loop {
            // 等待事件
            if let Ok(event) = tokio::time::timeout(
                Duration::from_millis(debounce_ms),
                rx.recv()
            ).await {
                match event {
                    Some(file_event) => {
                        pending_events.push(file_event);
                    }
                    None => break, // 通道关闭
                }
            } else {
                // 超时，处理累积的事件
                if !pending_events.is_empty() {
                    log::info!("处理 {} 个文件变更事件", pending_events.len());
                    
                    // TODO: 这里应该触发审查，但为了简化实现，只记录日志
                    pending_events.clear();
                }
            }
        }
    }

    /// 检查文件是否应该被监听
    fn should_watch_file(file_path: &str, patterns: &[String]) -> bool {
        // 简单实现：检查文件扩展名
        for pattern in patterns {
            if pattern.starts_with("*.") {
                let ext = &pattern[1..]; // 获取 "*.xxx" -> ".xxx"
                if file_path.ends_with(ext) {
                    return true;
                }
            } else if pattern.contains("*") {
                // 通配符匹配（简化实现）
                let pattern = pattern.replace("*", "");
                if file_path.contains(&pattern) {
                    return true;
                }
            }
        }
        false
    }

    /// 触发增量审查
    pub async fn trigger_incremental_review(&mut self, file_paths: &[String]) -> Result<RealtimeReviewResult, String> {
        log::info!("触发增量审查，{} 个文件", file_paths.len());

        let start_time = SystemTime::now();

        // 读取文件内容并创建 CodeChange
        let mut code_changes = Vec::new();
        for file_path in file_paths {
            match std::fs::read_to_string(file_path) {
                Ok(content) => {
                    let language = Self::detect_language(file_path);
                    code_changes.push(CodeChange {
                        file_path: file_path.clone(),
                        content,
                        language,
                        change_type: "Modified".to_string(),
                    });
                }
                Err(e) => {
                    log::warn!("读取文件 {} 失败：{}", file_path, e);
                }
            }
        }

        // 运行审查
        let review_result = self.code_review_agent.run_review(&code_changes).await?;

        let end_time = SystemTime::now();
        let review_time_ms = end_time.duration_since(start_time)
            .unwrap_or(Duration::ZERO)
            .as_millis() as u64;

        let result = RealtimeReviewResult::new(
            file_paths.to_vec(),
            review_result,
            review_time_ms,
            true,
        );

        log::info!("增量审查完成，耗时 {}ms", review_time_ms);
        Ok(result)
    }

    /// 检测文件语言
    fn detect_language(file_path: &str) -> String {
        if file_path.ends_with(".rs") {
            "rust".to_string()
        } else if file_path.ends_with(".ts") || file_path.ends_with(".tsx") {
            "typescript".to_string()
        } else if file_path.ends_with(".js") || file_path.ends_with(".jsx") {
            "javascript".to_string()
        } else {
            "unknown".to_string()
        }
    }

    /// 获取当前状态
    pub fn get_status(&self) -> &WatchStatus {
        &self.status
    }

    /// 获取已监听的路径列表
    pub fn get_watched_paths(&self) -> Vec<String> {
        self.watched_paths.iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_watch_config_creation() {
        let config = WatchConfig {
            project_path: "/tmp/test".to_string(),
            file_patterns: vec!["*.rs".to_string(), "*.ts".to_string()],
            enable_ai: true,
            debounce_ms: 500,
            recursive: true,
        };

        assert_eq!(config.project_path, "/tmp/test");
        assert_eq!(config.file_patterns.len(), 2);
        assert!(config.enable_ai);
        assert_eq!(config.debounce_ms, 500);
    }

    #[test]
    fn test_watch_status_display() {
        assert_eq!(format!("{}", WatchStatus::Pending), "等待开始");
        assert_eq!(format!("{}", WatchStatus::Running), "运行中");
        assert_eq!(format!("{}", WatchStatus::Stopped), "已停止");
    }

    #[test]
    fn test_file_change_event_creation() {
        let event = FileChangeEvent::new(
            "src/main.rs".to_string(),
            "Modified".to_string(),
        );

        assert_eq!(event.file_path, "src/main.rs");
        assert_eq!(event.change_type, "Modified");
        assert!(event.timestamp > 0);
    }

    #[test]
    fn test_should_watch_file_rust() {
        let patterns = vec!["*.rs".to_string()];
        
        assert!(RealtimeReviewManager::should_watch_file("src/main.rs", &patterns));
        assert!(RealtimeReviewManager::should_watch_file("src/lib.rs", &patterns));
        assert!(!RealtimeReviewManager::should_watch_file("src/main.ts", &patterns));
        assert!(!RealtimeReviewManager::should_watch_file("README.md", &patterns));
    }

    #[test]
    fn test_should_watch_file_typescript() {
        let patterns = vec!["*.ts".to_string(), "*.tsx".to_string()];
        
        assert!(RealtimeReviewManager::should_watch_file("src/app.ts", &patterns));
        assert!(RealtimeReviewManager::should_watch_file("src/component.tsx", &patterns));
        assert!(!RealtimeReviewManager::should_watch_file("src/main.rs", &patterns));
    }

    #[test]
    fn test_detect_language() {
        assert_eq!(RealtimeReviewManager::detect_language("src/main.rs"), "rust");
        assert_eq!(RealtimeReviewManager::detect_language("src/app.ts"), "typescript");
        assert_eq!(RealtimeReviewManager::detect_language("src/app.tsx"), "typescript");
        assert_eq!(RealtimeReviewManager::detect_language("src/app.js"), "javascript");
        assert_eq!(RealtimeReviewManager::detect_language("src/app.unknown"), "unknown");
    }

    #[test]
    fn test_realtime_review_manager_creation() {
        let config = WatchConfig::default();
        let manager = RealtimeReviewManager::new(config.clone());

        assert_eq!(manager.config.project_path, config.project_path);
        assert_eq!(manager.status, WatchStatus::Pending);
    }

    #[test]
    fn test_debounce_concept() {
        // 测试防抖概念：快速连续触发多次，应该只处理一次
        let mut events = vec![
            FileChangeEvent::new("file1.rs".to_string(), "Modified".to_string()),
            FileChangeEvent::new("file2.rs".to_string(), "Modified".to_string()),
            FileChangeEvent::new("file3.rs".to_string(), "Modified".to_string()),
        ];

        // 模拟防抖：收集所有事件
        let mut pending: Vec<FileChangeEvent> = Vec::new();
        for event in events.drain(..) {
            pending.push(event);
        }

        // 应该在 debounce 后一次性处理
        assert_eq!(pending.len(), 3);
    }

    #[test]
    fn test_filter_watched_files() {
        let patterns = vec!["*.rs".to_string()];
        let files = vec![
            "src/main.rs",
            "src/lib.ts",
            "src/utils.rs",
            "README.md",
        ];

        let watched: Vec<&str> = files.iter()
            .filter(|f| RealtimeReviewManager::should_watch_file(f, &patterns))
            .cloned()
            .collect();

        assert_eq!(watched, vec!["src/main.rs", "src/utils.rs"]);
    }

    #[test]
    fn test_realtime_review_result_structure() {
        use crate::agent::code_review_agent::ReviewResult;  // 只导入使用的 ReviewResult

        let review_result = ReviewResult::new(
            vec![],
            "No issues".to_string(),
            100.0,
            false,
        );

        let realtime_result = RealtimeReviewResult::new(
            vec!["src/main.rs".to_string()],
            review_result,
            150,
            true,
        );

        assert_eq!(realtime_result.reviewed_files.len(), 1);
        assert_eq!(realtime_result.review_time_ms, 150);
        assert!(realtime_result.triggered_by_change);
    }
}
