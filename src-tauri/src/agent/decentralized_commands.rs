use tauri::State;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};

use crate::agent::decentralized::node::{DecentralizedAgentNode, NodeConfig};
use crate::agent::decentralized::event_bus::SharedEventBus;
use crate::agent::decentralized::distributed_lock::SharedLockManager;
use crate::agent::agent_manager::AgentManager;

/// 启动去中心化 Agent Node
#[tauri::command]
pub async fn start_decentralized_node(
    state: State<'_, Arc<RwLock<AgentManager>>>,
    node_id: Option<String>,
    max_concurrent: Option<usize>,
) -> Result<String, String> {
    let mut manager = state.write().await;
    
    // 生成或使用提供的 node_id
    let node_id = node_id.unwrap_or_else(|| format!("node-{}", uuid::Uuid::new_v4()));
    let max_concurrent = max_concurrent.unwrap_or(3);
    
    // 创建配置
    let config = NodeConfig {
        node_id: node_id.clone(),
        max_concurrent_agents: max_concurrent,
        lock_ttl_secs: 600,
    };
    
    // 获取或创建共享的 EventBus
    let shared_event_bus = if let Some(ref bus) = manager.shared_event_bus {
        bus.clone()
    } else {
        let bus = Arc::new(SharedEventBus::new());
        manager.shared_event_bus = Some(bus.clone());
        bus
    };
    
    // 获取或创建共享的 LockManager
    let shared_lock_manager = if let Some(ref lock_mgr) = manager.shared_lock_manager {
        lock_mgr.clone()
    } else {
        let lock_mgr = Arc::new(SharedLockManager::new(600));
        manager.shared_lock_manager = Some(lock_mgr.clone());
        lock_mgr
    };
    
    // 获取 Daemon Manager
    let daemon_manager = manager.daemon.clone();
    
    // 创建并启动 Node
    let mut node = DecentralizedAgentNode::new(
        config,
        shared_event_bus,
        shared_lock_manager,
        daemon_manager,
    ).await?;
    
    // 设置 Worktree Manager (从项目路径获取)
    if let Some(ref project_path) = manager.project_path {
        node.set_worktree_manager(project_path);
    }
    
    // 启动 Node
    node.start().await?;
    
    // 保存 Node 引用到 Manager
    manager.decentralized_nodes.insert(node_id.clone(), Arc::new(RwLock::new(node)));
    
    log::info!("[DecentralizedCommand] Started decentralized node: {}", node_id);
    
    Ok(node_id)
}

/// 停止去中心化 Agent Node
#[tauri::command]
pub async fn stop_decentralized_node(
    state: State<'_, Arc<RwLock<AgentManager>>>,
    node_id: String,
) -> Result<(), String> {
    let mut manager = state.write().await;
    
    if let Some(node_arc) = manager.decentralized_nodes.remove(&node_id) {
        let mut node = node_arc.write().await;
        node.stop().await?;
        
        log::info!("[DecentralizedCommand] Stopped decentralized node: {}", node_id);
        Ok(())
    } else {
        Err(format!("Node {} not found", node_id))
    }
}

/// 列出所有去中心化 Agent Nodes
#[tauri::command]
pub async fn list_decentralized_nodes(
    state: State<'_, Arc<RwLock<AgentManager>>>,
) -> Result<Vec<NodeInfo>, String> {
    let manager = state.read().await;
    
    let mut nodes_info = vec![];
    
    for (node_id, node_arc) in &manager.decentralized_nodes {
        let node = node_arc.read().await;
        nodes_info.push(NodeInfo {
            node_id: node_id.clone(),
            is_running: node.is_running(),
        });
    }
    
    Ok(nodes_info)
}

/// Node 信息结构
#[derive(Serialize, Deserialize)]
pub struct NodeInfo {
    pub node_id: String,
    pub is_running: bool,
}
