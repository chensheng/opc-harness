use tauri::State;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};

use crate::agent::agent_worker::{AgentWorker, AgentWorkerConfig};
use crate::agent::agent_manager::AgentManager;

/// 启动完全去中心化的 Agent Worker
#[tauri::command]
pub async fn start_agent_worker(
    state: State<'_, Arc<RwLock<AgentManager>>>,
    #[serde(alias = "workerId")] worker_id: Option<String>,
    #[serde(alias = "projectId")] project_id: String,
    #[serde(alias = "checkInterval")] check_interval: Option<u64>,
) -> Result<String, String> {
    let mut manager = state.write().await;
    
    // 生成或使用提供的 worker_id
    let worker_id = worker_id.unwrap_or_else(|| format!("worker-{}", uuid::Uuid::new_v4()));
    let check_interval = check_interval.unwrap_or(30); // 默认 30 秒
    
    log::info!(
        "[AgentWorkerCommand] Starting fully decentralized agent worker: {} for project {}",
        worker_id,
        project_id
    );
    
    // 创建配置
    let config = AgentWorkerConfig {
        worker_id: worker_id.clone(),
        project_id: project_id.clone(),
        check_interval_secs: check_interval,
        max_concurrent: 1,
    };
    
    // 获取 Daemon Manager 和 WebSocket Manager
    let daemon_manager = manager.daemon.clone();
    let websocket_manager = manager.websocket.clone();
    
    // 创建 Agent Worker
    let mut worker = AgentWorker::new(config, daemon_manager);
    
    // 设置 WebSocket Manager（用于实时日志推送）
    worker.set_websocket_manager(websocket_manager);
    
    // 设置 Worktree Manager (从项目路径获取)
    if let Some(ref project_path) = manager.project_path {
        worker.set_worktree_manager(project_path);
    } else {
        // 使用默认工作空间目录
        let workspaces_root = crate::utils::paths::get_workspaces_dir();
        worker.set_worktree_manager(&workspaces_root.to_string_lossy());
    }
    
    // 启动 Worker
    worker.start().await?;
    
    // 保存 Worker 引用到 Manager
    manager.agent_workers.insert(worker_id.clone(), Arc::new(RwLock::new(worker)));
    
    log::info!("[AgentWorkerCommand] Started fully decentralized agent worker: {}", worker_id);
    
    Ok(worker_id)
}

/// 停止完全去中心化的 Agent Worker
#[tauri::command]
pub async fn stop_agent_worker(
    state: State<'_, Arc<RwLock<AgentManager>>>,
    #[serde(alias = "workerId")] worker_id: String,
) -> Result<(), String> {
    let mut manager = state.write().await;
    
    log::info!("[AgentWorkerCommand] Stopping agent worker: {}", worker_id);
    
    if let Some(worker_arc) = manager.agent_workers.remove(&worker_id) {
        let mut worker = worker_arc.write().await;
        worker.stop().await?;
        
        log::info!("[AgentWorkerCommand] Stopped fully decentralized agent worker: {}", worker_id);
        Ok(())
    } else {
        Err(format!("Worker {} not found", worker_id))
    }
}

/// 列出所有完全去中心化的 Agent Workers
#[tauri::command]
pub async fn list_agent_workers(
    state: State<'_, Arc<RwLock<AgentManager>>>,
) -> Result<Vec<WorkerInfo>, String> {
    let manager = state.read().await;
    
    let mut workers_info = vec![];
    
    for (worker_id, worker_arc) in &manager.agent_workers {
        let worker = worker_arc.read().await;
        workers_info.push(WorkerInfo {
            worker_id: worker_id.clone(),
            is_running: worker.is_running(),
            current_story_id: worker.current_story_id().map(|s| s.to_string()),
        });
    }
    
    Ok(workers_info)
}

/// Worker 信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerInfo {
    pub worker_id: String,
    pub is_running: bool,
    pub current_story_id: Option<String>,
}
