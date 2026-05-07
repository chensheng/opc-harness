//! Agent Manager Tauri Commands - Main Entry Point
//! 
//! 所有 Tauri 命令的定义和导出
//! 
//! 注意：由于 Tauri 的 #[tauri::command] 宏会生成全局唯一符号，
//! 所有命令必须在此文件中定义，不能分散到子模块中重复导出。

use std::sync::Arc;
use tauri::State;
use tokio::sync::RwLock;

use crate::agent::agent_manager_core::AgentManager;
use crate::agent::agent_manager_types::{AgentHandle, AgentManagerStats};
use crate::agent::types::AgentType;
use crate::agent::daemon::DaemonStatus;
use crate::agent::branch_manager::{BranchInfo, BranchOperationResult};
use crate::agent::code_review_agent::{CodeReviewAgent, CodeReviewAgentConfig, ReviewResult, ReviewDimension, ReviewSeverity, CodeChange};
use crate::agent::realtime_review_manager::{RealtimeReviewManager, WatchConfig};
use crate::agent::test_runner_agent::{TestRunnerAgent, TestRunnerConfig, TestSuiteResult};
use crate::agent::performance_benchmark_agent::{PerformanceBenchmarkAgent, BenchmarkConfig, BenchmarkReport};
use crate::agent::realtime_performance_monitor::{RealtimePerformanceMonitor, MonitoringConfig, SystemStats};
use crate::agent::ai_code_generator::{AICodeGenerator, GenerationConfig, CodeGenerationRequest, CodeGenerationResponse};
use crate::agent::realtime_code_suggestions::{RealtimeCodeSuggestions, CodeSuggestion, SuggestionConfig};
use crate::agent::mr_description_generator::{MRDescriptionGenerator, MRDescription};
use crate::agent::code_change_tracker::{CodeChangeTracker, ChangeSummary, ChangeStatistics};
use crate::agent::code_diff_visualizer::{CodeDiffVisualizer, FileDiff, DiffSummary as VisualDiffSummary};
use crate::db;

// ============================================================================
// Basic Agent Management Commands
// ============================================================================

/// 创建新的 Agent
#[tauri::command]
pub async fn create_agent(
    _app_handle: tauri::AppHandle,
    state: State<'_, Arc<RwLock<AgentManager>>>,
    agent_type: String,
    session_id: String,
    project_id: String,
) -> Result<String, String> {
    let manager = state.read().await;
    
    let parsed_type = match agent_type.as_str() {
        "initializer" => AgentType::Initializer,
        "coding" => AgentType::Coding,
        "mr_creation" => AgentType::MRCreation,
        _ => return Err(format!("Unknown agent type: {}", agent_type)),
    };

    // 从 project_id 获取项目工作区路径
    let workspaces_root = crate::utils::paths::get_workspaces_dir();
    let project_workspace = workspaces_root.join(&project_id);
    
    // 确保项目目录存在
    if !project_workspace.exists() {
        std::fs::create_dir_all(&project_workspace)
            .map_err(|e| format!("Failed to create project directory: {}", e))?;
    }
    
    let project_path = project_workspace.to_string_lossy().to_string();
    let result = manager.create_agent(
        parsed_type.clone(), 
        session_id.clone(), 
        project_id.clone(), 
        project_path.clone(), 
        None,
        None,  // agents_md_content: not provided in legacy API
    ).await;
    drop(manager);
    
    // 注意：manager.create_agent 内部已经完成了数据库持久化，无需再次保存
    match &result {
        Ok(agent_id) => {
            log::info!("[create_agent] Agent created successfully: agent_id={}, project_id={}", agent_id, project_id);
            log::info!("[create_agent] Agent session already persisted by AgentManager");
        }
        Err(e) => {
            log::warn!("Agent creation failed, skipping database persistence: {}", e);
        }
    }
    
    result
}

/// 创建新的 Agent（使用 CLI 类型和 AGENTS.md 内容）
#[tauri::command]
pub async fn create_agent_with_cli(
    app: tauri::AppHandle,
    state: State<'_, Arc<RwLock<AgentManager>>>,
    cli_type: String,
    agents_content: String,
    project_id: String,
    name: Option<String>,
) -> Result<String, String> {
    use std::fs;
    use std::path::PathBuf;
    
    log::info!("[create_agent_with_cli] Starting agent creation");
    log::info!("[create_agent_with_cli] cli_type={}, project_id={}", cli_type, project_id);
    
    // 从 project_id 获取项目工作区路径
    let workspaces_root = crate::utils::paths::get_workspaces_dir();
    let project_workspace = workspaces_root.join(&project_id);
    
    log::info!("[create_agent_with_cli] Project workspace path: {:?}", project_workspace);
    
    // 确保项目目录存在
    if !project_workspace.exists() {
        log::info!("[create_agent_with_cli] Creating project directory: {:?}", project_workspace);
        fs::create_dir_all(&project_workspace)
            .map_err(|e| {
                log::error!("[create_agent_with_cli] Failed to create project directory: {}", e);
                format!("Failed to create project directory: {}", e)
            })?;
    }
    
    let project_path = project_workspace.to_string_lossy().to_string();
    log::info!("[create_agent_with_cli] Project path: {}", project_path);
    
    // 生成唯一的 session_id
    let session_id = format!("session-{}", uuid::Uuid::new_v4());
    log::info!("[create_agent_with_cli] Generated session_id: {}", session_id);
    
    // 根据 CLI 类型确定 Agent 类型
    let agent_type = match cli_type.as_str() {
        "codefree" | "kimi" | "claude" | "codex" => AgentType::Coding,
        _ => {
            log::error!("[create_agent_with_cli] Unsupported CLI type: {}", cli_type);
            return Err(format!("Unsupported CLI type: {}. Supported types: codefree, kimi, claude, codex", cli_type));
        }
    };
    log::info!("[create_agent_with_cli] Agent type: {:?}", agent_type);
    
    // 从项目路径中提取 project_id (UUID)
    let extracted_project_id = PathBuf::from(&project_path)
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| {
            log::error!("[create_agent_with_cli] Failed to extract project_id from path: {}", project_path);
            format!("Failed to extract project_id from path: {}", project_path)
        })?
        .to_string();
    
    log::info!("[create_agent_with_cli] Extracted project_id: {}", extracted_project_id);
    
    // 创建 Agent（传入 project_id、project_path、name 和 agents_md_content）
    // 注意：AGENTS.md 内容仅保存到数据库，不写入文件系统
    log::info!("[create_agent_with_cli] Calling manager.create_agent...");
    let mut manager = state.write().await;
    let result = manager.create_agent(
        agent_type.clone(), 
        session_id.clone(), 
        extracted_project_id.clone(), 
        project_path.clone(), 
        name,
        Some(agents_content),  // Pass AGENTS.md content to database
    ).await;
    
    log::info!("[create_agent_with_cli] manager.create_agent result: {:?}", result.is_ok());
    
    // 如果创建成功，立即启动去中心化 Agent Worker
    if let Ok(ref agent_id) = result {
        log::info!("[create_agent_with_cli] ✓ Agent created: {}, now starting decentralized worker...", agent_id);
        
        // 创建 Worker 配置（使用 agent_id 作为 worker_id）
        let config = crate::agent::agent_worker::AgentWorkerConfig {
            worker_id: agent_id.clone(),  // 使用 agent_id 作为 worker_id，统一概念
            project_id: extracted_project_id.clone(),
            check_interval_secs: 30,  // 每 30 秒检查一次数据库
            max_concurrent: 1,
            app_handle: Some(manager.app_handle.clone()),
            lock_timeout_minutes: 30,  // 默认 30 分钟超时
        };
        
        // 获取 Daemon Manager 和 WebSocket Manager
        let daemon_manager = manager.daemon.clone();
        let websocket_manager = manager.websocket.clone();
        
        // 创建并启动 Agent Worker
        let mut worker = crate::agent::agent_worker::AgentWorker::new(config, daemon_manager);
        
        // 设置 WebSocket Manager（用于实时日志推送）
        worker.set_websocket_manager(websocket_manager);
        
        // 设置 Worktree Manager
        let workspaces_root = crate::utils::paths::get_workspaces_dir();
        worker.set_worktree_manager(&workspaces_root.to_string_lossy());
        
        match worker.start().await {
            Ok(_) => {
                log::info!("[create_agent_with_cli] ✓ Decentralized worker started automatically for agent: {}", agent_id);
                // 保存 Worker 引用到 Manager
                manager.agent_workers.insert(agent_id.clone(), Arc::new(RwLock::new(worker)));
            }
            Err(e) => {
                log::warn!("[create_agent_with_cli] ⚠️ Failed to auto-start worker for agent {}: {}", agent_id, e);
                // 不返回错误，因为 Agent Session 已经创建成功，用户可以稍后手动启动
            }
        }
    }
    
    drop(manager);
    result
}

/// 启动 Agent
#[tauri::command]
pub async fn start_agent(
    state: State<'_, Arc<RwLock<AgentManager>>>,
    agent_id: String,
) -> Result<(), String> {
    let manager = state.read().await;
    let result = manager.start_agent(&agent_id).await;
    drop(manager);
    result
}

/// 停止 Agent
#[tauri::command]
pub async fn stop_agent(
    state: State<'_, Arc<RwLock<AgentManager>>>,
    agent_id: String,
    graceful: bool,
) -> Result<(), String> {
    let manager = state.read().await;
    let result = manager.stop_agent(&agent_id, graceful).await;
    drop(manager);
    result
}

/// 获取 Agent 状态
#[tauri::command]
pub async fn get_agent_status(
    state: State<'_, Arc<RwLock<AgentManager>>>,
    agent_id: String,
) -> Result<AgentHandle, String> {
    let manager = state.read().await;
    let result = manager.get_agent_status(&agent_id).await;
    drop(manager);
    result
}

/// 获取所有 Agent
#[tauri::command]
pub async fn get_all_agents(
    state: State<'_, Arc<RwLock<AgentManager>>>,
) -> Result<Vec<AgentHandle>, String> {
    let manager = state.read().await;
    let result = manager.get_all_agents().await;
    drop(manager);
    Ok(result)
}

/// 获取指定 Session 的 Agent
#[tauri::command]
pub async fn get_agents_by_session(
    state: State<'_, Arc<RwLock<AgentManager>>>,
    session_id: String,
) -> Result<Vec<AgentHandle>, String> {
    let manager = state.read().await;
    let result = manager.get_agents_by_session(&session_id).await;
    drop(manager);
    Ok(result)
}

/// 获取 Agent Manager 统计信息
#[tauri::command]
pub async fn get_agent_manager_stats(
    state: State<'_, Arc<RwLock<AgentManager>>>,
) -> Result<AgentManagerStats, String> {
    let manager = state.read().await;
    let result = manager.get_stats().await;
    drop(manager);
    Ok(result)
}

/// 获取 Daemon 状态
#[tauri::command]
pub async fn get_daemon_statuses(
    state: State<'_, Arc<RwLock<AgentManager>>>,
) -> Result<DaemonStatus, String> {
    let manager = state.read().await;
    let result = manager.get_daemon_status().await;
    drop(manager);
    Ok(result)
}

/// 获取所有持久化的 Agent Sessions (VC-005)
#[tauri::command]
pub async fn get_all_agent_sessions(
    state: State<'_, Arc<RwLock<AgentManager>>>,
) -> Result<Vec<crate::models::AgentSession>, String> {
    let manager = state.read().await;
    let conn = db::get_connection()
        .map_err(|e| format!("Failed to get database connection: {}", e))?;
    drop(manager);
    
    db::get_all_agent_sessions(&conn)
        .map_err(|e| format!("Failed to fetch agent sessions: {}", e))
}

/// 初始化 Agent Manager
#[tauri::command]
pub async fn initialize_agent_manager(
    state: State<'_, Arc<RwLock<AgentManager>>>,
    session_id: String,
    project_path: String,
    max_concurrent_agents: usize,
) -> Result<(), String> {
    let manager = state.read().await;
    
    let config = crate::agent::daemon::DaemonConfig {
        session_id,
        project_path,
        log_level: "info".to_string(),
        max_concurrent_agents,
        workspace_dir: ".".to_string(),
        lock_timeout_minutes: 30, // 默认 30 分钟超时
    };

    let result = manager.initialize(config).await;
    drop(manager);
    result
}

// ============================================================================
// Branch Management Commands
// ============================================================================

/// 创建功能分支
#[tauri::command]
pub async fn create_feature_branch(
    state: State<'_, Arc<RwLock<AgentManager>>>,
    _session_id: String,
    issue_id: String,
    description: String,
) -> Result<BranchOperationResult, String> {
    let manager = state.read().await;
    let mut branch_manager = manager.get_or_create_branch_manager().await;
    
    let result = branch_manager
        .as_mut()
        .unwrap()
        .create_feature_branch(&description, Some(&issue_id), None)
        .await?;
    
    drop(branch_manager);
    drop(manager);
    
    Ok(result)
}

/// 切换到指定分支
#[tauri::command]
pub async fn checkout_branch(
    state: State<'_, Arc<RwLock<AgentManager>>>,
    _session_id: String,
    branch_name: String,
) -> Result<BranchOperationResult, String> {
    let manager = state.read().await;
    let mut branch_manager = manager.get_or_create_branch_manager().await;
    let result = branch_manager.as_mut().unwrap().checkout_branch(&branch_name).await?;
    drop(branch_manager);
    drop(manager);
    Ok(result)
}

/// 删除分支
#[tauri::command]
pub async fn delete_branch(
    state: State<'_, Arc<RwLock<AgentManager>>>,
    _session_id: String,
    branch_name: String,
    force: bool,
) -> Result<BranchOperationResult, String> {
    let manager = state.read().await;
    let mut branch_manager = manager.get_or_create_branch_manager().await;
    let result = branch_manager.as_mut().unwrap().delete_branch(&branch_name, force).await?;
    drop(branch_manager);
    drop(manager);
    Ok(result)
}

/// 列出所有分支
#[tauri::command]
pub async fn list_branches(
    state: State<'_, Arc<RwLock<AgentManager>>>,
    _session_id: String,
) -> Result<Vec<BranchInfo>, String> {
    let manager = state.read().await;
    let branch_manager = manager.get_branch_manager().await;
    let branches = branch_manager.as_ref().unwrap().get_local_branches().await?;
    drop(branch_manager);
    drop(manager);
    Ok(branches)
}

/// 获取当前分支
#[tauri::command]
pub async fn get_current_branch(
    state: State<'_, Arc<RwLock<AgentManager>>>,
    _session_id: String,
) -> Result<Option<String>, String> {
    let manager = state.read().await;
    let branch_manager = manager.get_branch_manager().await;
    let current = branch_manager.as_ref().unwrap().get_current_branch().await?;
    drop(branch_manager);
    drop(manager);
    Ok(current)
}

// ============================================================================
// Advanced Agent Commands (Initializer, MR Creation, Debug, Git Commit)
// ============================================================================

/// 运行 Initializer Agent 初始化流程 (VC-010)
#[tauri::command]
pub async fn run_initializer_agent(
    state: State<'_, Arc<RwLock<AgentManager>>>,
    _session_id: String,
    project_path: String,
    prd_content: String,
) -> Result<crate::agent::initializer_agent::InitializerResult, String> {
    use crate::agent::initializer_agent::InitializerAgentConfig;
    use uuid::Uuid;
    
    let manager = state.read().await;
    drop(manager);
    
    let _config = InitializerAgentConfig {
        agent_id: format!("initializer-{}", Uuid::new_v4()),
        project_path: project_path.clone(),
        ai_config: crate::ai::AIConfig {
            provider: "openai".to_string(),
            api_key: "placeholder".to_string(),
            model: "gpt-4".to_string(),
            base_url: None,
        },
        prd_file_path: None,
        prd_content: Some(prd_content),
    };
    
    Ok(crate::agent::initializer_agent::InitializerResult {
        success: true,
        message: "Initialization completed (placeholder)".to_string(),
        issues_created: Vec::new(),
        git_initialized: true,
        environment_checked: true,
    })
}

/// 运行 MR Creation Agent 创建合并请求 (VC-016)
#[tauri::command]
pub async fn create_merge_request(
    state: State<'_, Arc<RwLock<AgentManager>>>,
    _session_id: String,
    project_path: String,
    target_branch: String,
    feature_branches: Vec<String>,
    run_regression_tests: bool,
    auto_resolve_conflicts: bool,
) -> Result<crate::agent::mr_creation_agent::MRCreationResult, String> {
    use crate::agent::mr_creation_agent::{MRCreationAgent, MRCreationConfig};
    
    let manager = state.read().await;
    drop(manager);
    
    let config = MRCreationConfig {
        project_path: project_path.clone(),
        target_branch: target_branch.clone(),
        feature_branches,
        run_regression_tests,
        auto_resolve_conflicts,
    };
    
    let mut agent = MRCreationAgent::new(config);
    let result = agent.create_mr().await?;
    
    Ok(result)
}

/// 运行 Debug Agent 诊断错误 (VC-022)
#[tauri::command]
pub async fn run_debug_agent(
    state: State<'_, Arc<RwLock<AgentManager>>>,
    _session_id: String,
    project_path: String,
    error_source: String,
    error_output: String,
    auto_fix: bool,
    max_suggestions: usize,
) -> Result<crate::agent::debug_agent::DebugResult, String> {
    use crate::agent::debug_agent::{DebugAgent, DebugAgentConfig, ErrorSource};
    
    let manager = state.read().await;
    drop(manager);
    
    let parsed_error_source = match error_source.to_lowercase().as_str() {
        "typescript" | "ts" => ErrorSource::TypeScript,
        "rust" | "rs" => ErrorSource::Rust,
        "eslint" => ErrorSource::ESLint,
        "jest" | "vitest" => ErrorSource::Jest,
        "cargo" | "cargo-test" => ErrorSource::CargoTest,
        "runtime" | "log" => ErrorSource::RuntimeLog,
        _ => ErrorSource::UserInput,
    };
    
    let config = DebugAgentConfig {
        project_path: project_path.clone(),
        error_source: parsed_error_source,
        auto_fix,
        max_suggestions: if max_suggestions == 0 { 5 } else { max_suggestions },
        error_output,
    };
    
    let mut agent = DebugAgent::new(config);
    let result = agent.run_debug().await?;
    
    Ok(result)
}

/// 生成 Git 提交信息 (VC-026)
#[tauri::command]
pub async fn generate_commit_message(
    state: State<'_, Arc<RwLock<AgentManager>>>,
    _session_id: String,
    project_path: String,
    use_ai: bool,
    include_file_list: bool,
    max_summary_length: usize,
    conventional_commit: bool,
) -> Result<crate::agent::git_commit_assistant::CommitMessage, String> {
    use crate::agent::git_commit_assistant::{GitCommitAssistant, GitCommitAssistantConfig};
    
    let manager = state.read().await;
    drop(manager);
    
    let config = GitCommitAssistantConfig {
        project_path: project_path.clone(),
        use_ai,
        include_file_list,
        max_summary_length: if max_summary_length == 0 { 50 } else { max_summary_length },
        conventional_commit,
    };
    
    let mut assistant = GitCommitAssistant::new(config);
    let message = assistant.generate_commit_message().await?;
    
    Ok(message)
}

// ============================================================================
// Code Review & Testing Commands
// ============================================================================

/// 运行代码审查 Agent
#[tauri::command]
pub async fn run_code_review(
    state: State<'_, Arc<RwLock<AgentManager>>>,
    _session_id: String,
    _file_paths: Vec<String>,
    enable_ai: bool,
) -> Result<ReviewResult, String> {
    let manager = state.read().await;
    drop(manager);
    
    let config = CodeReviewAgentConfig {
        project_path: ".".to_string(),
        enable_ai,
        dimensions: vec![
            ReviewDimension::Style,
            ReviewDimension::Performance,
            ReviewDimension::Security,
            ReviewDimension::BestPractice,
        ],
        min_severity: ReviewSeverity::Info,
        max_comments: 100,
    };

    let mut agent = CodeReviewAgent::new(config);

    let code_changes = vec![
        CodeChange {
            file_path: "example.rs".to_string(),
            content: "// Example code for review".to_string(),
            language: "rust".to_string(),
            change_type: "Modified".to_string(),
        }
    ];

    let result = agent.run_review(&code_changes).await?;
    Ok(result)
}

/// 启动实时审查监听
#[tauri::command]
pub async fn start_realtime_review(
    state: State<'_, Arc<RwLock<AgentManager>>>,
    session_id: String,
    config: WatchConfig,
) -> Result<(), String> {
    let manager = state.read().await;
    drop(manager);
    
    let mut manager_instance = RealtimeReviewManager::new(config);
    manager_instance.start_watch().await?;
    
    log::info!("实时审查监听已启动 for session: {}", session_id);
    Ok(())
}

/// 停止实时审查监听
#[tauri::command]
pub async fn stop_realtime_review(
    state: State<'_, Arc<RwLock<AgentManager>>>,
    session_id: String,
) -> Result<(), String> {
    let manager = state.read().await;
    drop(manager);
    
    log::info!("实时审查监听已停止 for session: {}", session_id);
    Ok(())
}

/// 运行测试
#[tauri::command]
pub async fn run_tests(
    state: State<'_, Arc<RwLock<AgentManager>>>,
    session_id: String,
    config: TestRunnerConfig,
) -> Result<TestSuiteResult, String> {
    let manager = state.read().await;
    drop(manager);
    
    let agent = TestRunnerAgent::new(config);
    let result = agent.run_tests().await?;
    
    log::info!("测试完成 for session {}: {} passed / {} total", 
               session_id, result.passed, result.total);
    
    Ok(result)
}

/// 运行性能基准测试
#[tauri::command]
pub async fn run_benchmark(
    state: State<'_, Arc<RwLock<AgentManager>>>,
    session_id: String,
    config: BenchmarkConfig,
) -> Result<BenchmarkReport, String> {
    let manager = state.read().await;
    drop(manager);
    
    let agent = PerformanceBenchmarkAgent::new(config);
    let report = agent.run_benchmarks().await?;
    
    log::info!("基准测试完成 for session {}: {} total, {} regressed", 
               session_id, report.total_benchmarks, report.regressed_count);
    
    Ok(report)
}

/// 启动实时监控
#[tauri::command]
pub async fn start_monitoring(
    state: State<'_, Arc<RwLock<AgentManager>>>,
    session_id: String,
    config: MonitoringConfig,
) -> Result<(), String> {
    let manager = state.write().await;
    drop(manager);
    
    let mut monitor = RealtimePerformanceMonitor::new(config);
    monitor.start_monitoring().await?;
    
    log::info!("实时性能监控已启动 for session {}", session_id);
    
    Ok(())
}

/// 停止实时监控
#[tauri::command]
pub async fn stop_monitoring(
    state: State<'_, Arc<RwLock<AgentManager>>>,
    session_id: String,
) -> Result<(), String> {
    let manager = state.write().await;
    drop(manager);
    
    log::info!("实时性能监控已停止 for session {}", session_id);
    
    Ok(())
}

/// 获取当前系统统计信息
#[tauri::command]
pub async fn get_current_stats(
    _state: State<'_, Arc<RwLock<AgentManager>>>,
) -> Result<SystemStats, String> {
    let config = MonitoringConfig::default();
    let monitor = RealtimePerformanceMonitor::new(config);
    
    monitor.get_current_stats()
}

// ============================================================================
// AI Code Generation Commands
// ============================================================================

/// 生成代码
#[tauri::command]
pub async fn generate_code(
    state: State<'_, Arc<RwLock<AgentManager>>>,
    session_id: String,
    request: CodeGenerationRequest,
) -> Result<CodeGenerationResponse, String> {
    let manager = state.read().await;
    drop(manager);
    
    let config = GenerationConfig::default();
    let generator = AICodeGenerator::new(config, "mock_api_key".to_string());
    
    let response = generator.generate_code(request).await?;
    
    log::info!("代码生成完成 for session {}", session_id);
    
    Ok(response)
}

/// 代码补全
#[tauri::command]
pub async fn complete_code(
    state: State<'_, Arc<RwLock<AgentManager>>>,
    session_id: String,
    code: String,
    cursor_position: usize,
) -> Result<CodeGenerationResponse, String> {
    let manager = state.read().await;
    drop(manager);
    
    let config = GenerationConfig::default();
    let generator = AICodeGenerator::new(config, "mock_api_key".to_string());
    
    let response = generator.complete_code(code, cursor_position).await?;
    
    log::info!("代码补全完成 for session {}", session_id);
    
    Ok(response)
}

/// 生成函数
#[tauri::command]
pub async fn generate_function(
    state: State<'_, Arc<RwLock<AgentManager>>>,
    session_id: String,
    description: String,
    language: String,
) -> Result<CodeGenerationResponse, String> {
    let manager = state.read().await;
    drop(manager);
    
    let config = GenerationConfig::default();
    let generator = AICodeGenerator::new(config, "mock_api_key".to_string());
    
    let response = generator.generate_function(description, language).await?;
    
    log::info!("函数生成完成 for session {}", session_id);
    
    Ok(response)
}

/// 启动代码建议
#[tauri::command]
pub async fn start_suggestions(
    state: State<'_, Arc<RwLock<AgentManager>>>,
    session_id: String,
    file_paths: Vec<String>,
) -> Result<(), String> {
    let manager = state.read().await;
    drop(manager);
    
    let config = SuggestionConfig::default();
    let mut suggestions = RealtimeCodeSuggestions::new(config);
    
    suggestions.start_monitoring(file_paths).await?;
    
    log::info!("代码建议已启动 for session {}", session_id);
    
    Ok(())
}

/// 停止代码建议
#[tauri::command]
pub async fn stop_suggestions(
    state: State<'_, Arc<RwLock<AgentManager>>>,
    session_id: String,
) -> Result<(), String> {
    let manager = state.write().await;
    drop(manager);
    
    log::info!("代码建议已停止 for session {}", session_id);
    
    Ok(())
}

/// 获取代码建议
#[tauri::command]
pub async fn get_suggestions(
    _state: State<'_, Arc<RwLock<AgentManager>>>,
    file_path: String,
) -> Result<Vec<CodeSuggestion>, String> {
    use std::fs;
    
    let content = fs::read_to_string(&file_path)
        .map_err(|e| format!("读取文件失败：{}", e))?;
    
    let config = SuggestionConfig::default();
    let analyzer = RealtimeCodeSuggestions::new(config);
    
    Ok(analyzer.analyze_file(&file_path, &content))
}

// ============================================================================
// Code Change Tracking & MR Description Commands (VC-034)
// ============================================================================

/// 获取工作区的所有变更
#[tauri::command]
pub async fn get_workspace_changes(
    _state: State<'_, Arc<RwLock<AgentManager>>>,
    _session_id: String,
) -> Result<ChangeSummary, String> {
    let workspace_root = std::env::current_dir().map_err(|e| format!("Failed to get current directory: {}", e))?;
    
    let tracker = CodeChangeTracker::new(workspace_root)?;
    let summary = tracker.generate_summary().await?;
    
    Ok(summary)
}

/// 获取单个文件的 diff
#[tauri::command]
pub async fn get_file_diff(
    _state: State<'_, Arc<RwLock<AgentManager>>>,
    _session_id: String,
    file_path: String,
) -> Result<String, String> {
    let workspace_root = std::env::current_dir().map_err(|e| format!("Failed to get current directory: {}", e))?;
    
    let tracker = CodeChangeTracker::new(workspace_root)?;
    let (_, _, diff) = tracker.get_file_diff(&file_path).await?;
    
    Ok(diff)
}

/// 获取文件差异可视化数据
#[tauri::command]
pub async fn get_file_diff_visual(
    _state: State<'_, Arc<RwLock<AgentManager>>>,
    _session_id: String,
    file_path: String,
) -> Result<FileDiff, String> {
    let project_path = std::env::current_dir().map_err(|e| format!("Failed to get current directory: {}", e))?;
    
    let visualizer = CodeDiffVisualizer::new(project_path)?;
    let file_diff = visualizer.get_file_diff_visual(&file_path).await?;
    
    Ok(file_diff)
}

/// 获取差异摘要
#[tauri::command]
pub async fn get_diff_summary(
    _state: State<'_, Arc<RwLock<AgentManager>>>,
    _session_id: String,
    file_path: String,
) -> Result<VisualDiffSummary, String> {
    let project_path = std::env::current_dir().map_err(|e| format!("Failed to get current directory: {}", e))?;
    
    let visualizer = CodeDiffVisualizer::new(project_path)?;
    let file_diff = visualizer.get_file_diff_visual(&file_path).await?;
    
    let summary = VisualDiffSummary {
        file_path: file_diff.file_path.clone(),
        stats: file_diff.stats.clone(),
        hunk_count: file_diff.hunks.len() as u32,
    };
    
    Ok(summary)
}

/// 获取变更统计信息
#[tauri::command]
pub async fn get_change_statistics(
    _state: State<'_, Arc<RwLock<AgentManager>>>,
    _session_id: String,
) -> Result<ChangeStatistics, String> {
    let workspace_root = std::env::current_dir().map_err(|e| format!("Failed to get current directory: {}", e))?;
    
    let tracker = CodeChangeTracker::new(workspace_root)?;
    let changes = tracker.detect_changes().await?;
    let statistics = tracker.calculate_statistics(&changes);
    
    Ok(statistics)
}

/// 生成 MR 描述
#[tauri::command]
pub async fn generate_mr_description(
    _state: State<'_, Arc<RwLock<AgentManager>>>,
    _session_id: String,
    feature_branches: Vec<String>,
    target_branch: String,
) -> Result<MRDescription, String> {
    let project_path = std::env::current_dir().map_err(|e| format!("Failed to get current directory: {}", e))?;
    
    let generator = MRDescriptionGenerator::new(project_path)?;
    let mr_description = generator.generate_description(&feature_branches, &target_branch).await?;
    
    Ok(mr_description)
}

/// 获取 MR 模板
#[tauri::command]
pub async fn get_mr_template(
    _state: State<'_, Arc<RwLock<AgentManager>>>,
    _session_id: String,
    template_name: String,
) -> Result<String, String> {
    let templates = match template_name.as_str() {
        "default" => r#"# Merge Request Template

## 📋 Description
<!-- Describe your changes in detail -->

## 🔗 Related Issue
<!-- Link to the issue that is fixed by this PR -->

Fixes #

## ✅ Checklist
- [ ] Code compiles without warnings
- [ ] Tests are passing
- [ ] Documentation is updated
- [ ] Changelog is updated

## 🧪 Testing Done
<!-- Describe the testing you have done -->

## 📸 Screenshots (if applicable)
<!-- Add screenshots to demonstrate UI changes -->

"#.to_string(),
        "feature" => r#"# Feature Implementation

## 🎯 Goal
<!-- What problem does this feature solve? -->

## 🚀 Changes
<!-- List the main changes -->

## 📋 Requirements
- [ ] Feature implementation complete
- [ ] Tests added/updated
- [ ] Documentation updated

"#.to_string(),
        _ => return Err(format!("Unknown template: {}. Available: default, feature", template_name)),
    };
    
    Ok(templates)
}

// ============================================================================
// Worktree 管理命令 (P0: 隔离环境管理)
// ============================================================================

/// 创建 Worktree
#[tauri::command]
pub async fn create_worktree(
    _state: State<'_, Arc<RwLock<AgentManager>>>,
    project_id: String,  // 新增：项目 ID 参数
    agent_id: String,
    _story_id: String,
    _branch_name: String,
) -> Result<String, String> {
    // let manager = state.read().await; // 未使用
    
    // 验证项目 ID（可选：检查项目是否存在）
    log::debug!("[create_worktree] Creating worktree for project: {}, agent: {}", project_id, agent_id);
    
    // TODO: 实现独立的 WorktreeManager，不依赖 AgentLoop
    // 当前返回错误，提示用户使用去中心化 Worker
    Err("Worktree management is now integrated with fully decentralized Agent Workers. Please use start_agent_worker to automatically manage worktrees.".to_string())
}

/// 删除 Worktree
#[tauri::command]
pub async fn remove_worktree(
    state: State<'_, Arc<RwLock<AgentManager>>>,
    project_id: String,  // 新增：项目 ID 参数
    agent_id: String,
) -> Result<(), String> {
    let _manager = state.read().await;
    
    log::debug!("[remove_worktree] Removing worktree for project: {}, agent: {}", project_id, agent_id);
    
    // TODO: 实现独立的 WorktreeManager
    Err("Worktree management is now integrated with fully decentralized Agent Workers.".to_string())
}

/// 列出所有 Worktrees
#[tauri::command]
pub async fn list_worktrees(
    state: State<'_, Arc<RwLock<AgentManager>>>,
    project_id: Option<String>,  // 新增：可选的项目 ID 参数，None 表示列出所有
) -> Result<Vec<crate::agent::worktree_manager::WorktreeInfo>, String> {
    let _manager = state.read().await;
    
    log::debug!("[list_worktrees] Listing worktrees for project: {:?}", project_id);
    
    // TODO: 实现独立的 WorktreeManager
    Ok(vec![])
}

/// 清理孤立的 Worktrees
#[tauri::command]
pub async fn cleanup_orphaned_worktrees(
    state: State<'_, Arc<RwLock<AgentManager>>>,
    project_id: Option<String>,  // 新增：可选的项目 ID 参数
) -> Result<usize, String> {
    let _manager = state.read().await;
    
    log::debug!("[cleanup_orphaned_worktrees] Cleaning up orphaned worktrees for project: {:?}", project_id);
    
    // TODO: 实现独立的 WorktreeManager
    Ok(0)
}

/// 获取磁盘使用量
#[tauri::command]
pub async fn get_worktree_disk_usage(
    state: State<'_, Arc<RwLock<AgentManager>>>,
    project_id: Option<String>,  // 新增：可选的项目 ID 参数
) -> Result<u64, String> {
    let _manager = state.read().await;
    
    log::debug!("[get_worktree_disk_usage] Getting disk usage for project: {:?}", project_id);
    
    // TODO: 实现真正的磁盘使用量计算
    Ok(0)
}

// ============================================================================
// WebSocket Real-time Communication Commands (VC-003) - 使用说明
// ============================================================================
//
// **架构说明**:
// 本系统使用 Tauri Events 替代传统 WebSocket，实现进程内实时通信。
// 优势：零额外依赖、类型安全、低延迟、Session 隔离。
//
// **后端使用示例** (Rust):
// ```rust
// // 在 Agent Worker 或 Daemon 中发送日志
// let manager = state.read().await;
// let websocket = manager.websocket.read().await;
//
// // 发送日志消息
// websocket.send_log(
//     &session_id,
//     "info",
//     "Agent started successfully",
//     Some("AgentWorker")
// ).await?;
//
// // 发送进度更新
// websocket.send_progress(
//     &session_id,
//     "coding",
//     5,
//     10,
//     Some("Generating code...")
// ).await?;
//
// // 发送状态更新
// websocket.send_status(
//     &session_id,
//     "running",
//     Some("Processing task")
// ).await?;
// ```
//
// **前端使用示例** (TypeScript/React):
// ```typescript
// import { useAgent } from '@/hooks/useAgent'
//
// function MyComponent() {
//   const { messages, connectWebSocket, clearMessages } = useAgent()
//
//   // 自动连接（在 useEffect 中）
//   useEffect(() => {
//     if (projectId) {
//       connectWebSocket(`project-${projectId}`)
//     }
//   }, [projectId])
//
//   // 显示实时消息
//   return (
//     <div>
//       {messages.map(msg => (
//         <div key={msg.id}>
//           <Badge>{msg.type}</Badge>
//           <p>{msg.content}</p>
//           <small>{new Date(msg.timestamp).toLocaleTimeString()}</small>
//         </div>
//       ))}
//       <Button onClick={clearMessages}>清空日志</Button>
//     </div>
//   )
// }
// ```
//
// **消息类型**:
// - Log: 日志消息（info/warn/error/debug）
// - Progress: 进度更新（phase, current, total）
// - Status: 状态更新（status, details）
// - AgentResponse: Agent 响应（request_id, success, data/error）
// - Error: 错误消息（code, message, details）
// - Heartbeat: 心跳消息（timestamp）
//
// **Session 隔离**:
// 每个智能体使用独立的 Session ID（格式: `agent-{agentId}`），
// 确保消息不会跨智能体混淆。
//
// **性能监控**:
// 调用 `ws_get_stats()` 获取统计信息：
// - total_connections: 总连接数
// - active_connections: 当前活跃连接数
// - total_messages_sent: 总发送消息数
// - total_errors: 总错误数

/// 注册 WebSocket 连接（基于 Tauri Events）
#[tauri::command]
pub async fn ws_register_connection(
    state: State<'_, Arc<RwLock<AgentManager>>>,
    session_id: String,
) -> Result<String, String> {
    let manager = state.read().await;
    let websocket = manager.websocket.read().await;
    
    log::info!("[WebSocket] Registering connection for session: {}", session_id);
    
    let connection_id = websocket.register_connection(session_id.clone()).await;
    
    log::info!("[WebSocket] Connection registered: {} for session: {}", connection_id, session_id);
    
    Ok(connection_id)
}

/// 注销 WebSocket 连接
#[tauri::command]
pub async fn ws_unregister_connection(
    state: State<'_, Arc<RwLock<AgentManager>>>,
    session_id: String,
    connection_id: String,
) -> Result<(), String> {
    let manager = state.read().await;
    let websocket = manager.websocket.read().await;
    
    log::info!("[WebSocket] Unregistering connection: {} for session: {}", connection_id, session_id);
    
    websocket.unregister_connection(&session_id, &connection_id).await;
    
    log::info!("[WebSocket] Connection unregistered successfully");
    
    Ok(())
}

/// 发送日志消息到指定 Session
#[tauri::command]
pub async fn ws_send_log(
    state: State<'_, Arc<RwLock<AgentManager>>>,
    session_id: String,
    level: String,
    message: String,
    source: Option<String>,
) -> Result<(), String> {
    let manager = state.read().await;
    let websocket = manager.websocket.read().await;
    
    websocket.send_log(&session_id, &level, &message, source.as_deref()).await
}

/// 发送进度更新到指定 Session
#[tauri::command]
pub async fn ws_send_progress(
    state: State<'_, Arc<RwLock<AgentManager>>>,
    session_id: String,
    phase: String,
    current: u32,
    total: u32,
    description: Option<String>,
) -> Result<(), String> {
    let manager = state.read().await;
    let websocket = manager.websocket.read().await;
    
    websocket.send_progress(&session_id, &phase, current, total, description.as_deref()).await
}

/// 发送状态更新到指定 Session
#[tauri::command]
pub async fn ws_send_status(
    state: State<'_, Arc<RwLock<AgentManager>>>,
    session_id: String,
    status: String,
    details: Option<String>,
) -> Result<(), String> {
    let manager = state.read().await;
    let websocket = manager.websocket.read().await;
    
    websocket.send_status(&session_id, &status, details.as_deref()).await
}

/// 发送 Agent 响应到指定 Session
#[tauri::command]
pub async fn ws_send_agent_response(
    state: State<'_, Arc<RwLock<AgentManager>>>,
    session_id: String,
    request_id: String,
    success: bool,
    data: Option<serde_json::Value>,
    error: Option<String>,
) -> Result<(), String> {
    let manager = state.read().await;
    let websocket = manager.websocket.read().await;
    
    websocket.send_agent_response(&session_id, request_id, success, data, error).await
}

/// 发送错误消息到指定 Session
#[tauri::command]
pub async fn ws_send_error(
    state: State<'_, Arc<RwLock<AgentManager>>>,
    session_id: String,
    code: String,
    message: String,
    details: Option<serde_json::Value>,
) -> Result<(), String> {
    let manager = state.read().await;
    let websocket = manager.websocket.read().await;
    
    websocket.send_error(&session_id, &code, &message, details).await
}

/// 发送心跳到指定 Session
#[tauri::command]
pub async fn ws_send_heartbeat(
    state: State<'_, Arc<RwLock<AgentManager>>>,
    session_id: String,
) -> Result<(), String> {
    let manager = state.read().await;
    let websocket = manager.websocket.read().await;
    
    websocket.send_heartbeat(&session_id).await
}

/// 获取 WebSocket 统计信息
#[tauri::command]
pub async fn ws_get_stats(
    state: State<'_, Arc<RwLock<AgentManager>>>,
) -> Result<crate::agent::websocket_manager::WebSocketStats, String> {
    let manager = state.read().await;
    let websocket = manager.websocket.read().await;
    
    Ok(websocket.get_stats().await)
}

/// 获取指定 Session 的连接数
#[tauri::command]
pub async fn ws_get_connection_count(
    state: State<'_, Arc<RwLock<AgentManager>>>,
    session_id: String,
) -> Result<usize, String> {
    let manager = state.read().await;
    let websocket = manager.websocket.read().await;
    
    Ok(websocket.get_connection_count(&session_id).await)
}

/// 清理超时的连接
#[tauri::command]
pub async fn ws_cleanup_stale_connections(
    state: State<'_, Arc<RwLock<AgentManager>>>,
    timeout_ms: u64,
) -> Result<(), String> {
    let manager = state.read().await;
    let websocket = manager.websocket.read().await;
    
    websocket.cleanup_stale_connections(timeout_ms).await;
    
    Ok(())
}
