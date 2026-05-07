/// 去中心化智能体系统 - Agent Node (纯内存实现)
/// 
/// 每个 Node 是一个独立的 Agent Loop 实例,共享 EventBus 和 LockManager

use std::sync::Arc;
use tokio::sync::RwLock;
use log;

use super::event_bus::{EventBusClient, EventType, EventMessage, SharedEventBus};
use super::distributed_lock::SharedLockManager;
use crate::agent::daemon::DaemonManager;
use crate::agent::worktree_manager::WorktreeManager;

/// Agent Node 配置
#[derive(Debug, Clone)]
pub struct NodeConfig {
    /// 节点唯一标识
    pub node_id: String,
    /// 最大并发 Agent 数
    pub max_concurrent_agents: usize,
    /// 锁过期时间(秒)
    pub lock_ttl_secs: u64,
}

impl Default for NodeConfig {
    fn default() -> Self {
        Self {
            node_id: format!("node-{}", uuid::Uuid::new_v4()),
            max_concurrent_agents: 3,
            lock_ttl_secs: 600, // 10 分钟
        }
    }
}

/// 去中心化 Agent Node
pub struct DecentralizedAgentNode {
    config: NodeConfig,
    event_bus: EventBusClient,
    lock_manager: Arc<SharedLockManager>,
    daemon_manager: Arc<RwLock<DaemonManager>>,
    worktree_manager: Option<Arc<WorktreeManager>>,
    is_running: bool,
}

impl DecentralizedAgentNode {
    /// 创建新的 Agent Node (需要传入共享的 EventBus 和 LockManager)
    pub async fn new(
        config: NodeConfig,
        shared_event_bus: Arc<SharedEventBus>,
        shared_lock_manager: Arc<SharedLockManager>,
        daemon_manager: Arc<RwLock<DaemonManager>>,
    ) -> Result<Self, String> {
        let event_bus = EventBusClient::new(shared_event_bus, &config.node_id);
        
        log::info!("[DecentralizedNode] Created node {} with config: {:?}", 
            config.node_id, config);
        
        Ok(Self {
            config,
            event_bus,
            lock_manager: shared_lock_manager,
            daemon_manager,
            worktree_manager: None,
            is_running: false,
        })
    }
    
    /// 设置 Worktree Manager
    pub fn set_worktree_manager(&mut self, project_path: &str) {
        self.worktree_manager = Some(Arc::new(WorktreeManager::new(project_path)));
        log::info!("[DecentralizedNode:{}] Worktree manager initialized", self.config.node_id);
    }
    
    /// 启动 Node (开始监听事件并处理任务)
    pub async fn start(&mut self) -> Result<(), String> {
        if self.is_running {
            return Err("Node is already running".to_string());
        }
        
        self.is_running = true;
        log::info!("[DecentralizedNode:{}] Starting decentralized agent loop", self.config.node_id);
        
        // 订阅事件
        let mut event_rx = self.event_bus.subscribe();
        
        let node_id = self.config.node_id.clone();
        let event_bus = self.event_bus.clone();
        let lock_manager = self.lock_manager.clone();
        let daemon_manager = self.daemon_manager.clone();
        let worktree_manager = self.worktree_manager.clone();
        let max_concurrent = self.config.max_concurrent_agents;
        
        // 启动事件监听循环
        tokio::spawn(async move {
            log::info!("[DecentralizedNode:{}] Event listener started", node_id);
            
            loop {
                match event_rx.recv().await {
                    Ok(message) => {
                        log::info!("[DecentralizedNode:{}] Received event: {}", node_id, message.event_type);
                        
                        // 处理新 Story 事件
                        if message.event_type == "story.created" {
                            if let Err(e) = Self::handle_story_created(
                                &message,
                                &node_id,
                                &event_bus,
                                &lock_manager,
                                &daemon_manager,
                                &worktree_manager,
                                max_concurrent,
                            ).await {
                                log::error!("[DecentralizedNode:{}] Failed to handle story.created: {}", node_id, e);
                            }
                        }
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Lagged(n)) => {
                        log::warn!("[DecentralizedNode:{}] Lagged behind by {} messages", node_id, n);
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Closed) => {
                        log::warn!("[DecentralizedNode:{}] Event stream closed", node_id);
                        break;
                    }
                }
            }
        });
        
        Ok(())
    }
    
    /// 处理 Story 创建事件
    async fn handle_story_created(
        message: &EventMessage,
        node_id: &str,
        event_bus: &EventBusClient,
        lock_manager: &Arc<SharedLockManager>,
        _daemon_manager: &Arc<RwLock<DaemonManager>>,
        worktree_manager: &Option<Arc<WorktreeManager>>,
        max_concurrent: usize,
    ) -> Result<(), String> {
        let story_id = message.payload.get("story_id")
            .and_then(|v| v.as_str())
            .ok_or("Missing story_id in payload")?;
        
        let project_id = message.payload.get("project_id")
            .and_then(|v| v.as_str())
            .ok_or("Missing project_id in payload")?;
        
        log::info!("[DecentralizedNode:{}] Evaluating story {} from project {}", 
            node_id, story_id, project_id);
        
        // ✅ 自主决策: 是否处理这个 Story?
        if !Self::should_accept_story(node_id, max_concurrent).await {
            log::info!("[DecentralizedNode:{}] Declining story {} (load too high or not capable)", 
                node_id, story_id);
            return Ok(());
        }
        
        // ✅ 尝试锁定 Story (分布式锁)
        let lock_acquired = lock_manager.try_lock_story(story_id, node_id).await?;
        if !lock_acquired {
            log::info!("[DecentralizedNode:{}] Story {} already locked by another node", 
                node_id, story_id);
            return Ok(());
        }
        
        // ✅ 发布 story.locked 事件
        event_bus.publish(EventType::StoryLocked, serde_json::json!({
            "story_id": story_id,
            "locked_by": node_id,
            "timestamp": chrono::Utc::now().timestamp(),
        }))?;
        
        // ✅ 创建 Worktree
        let worktree_path = if let Some(ref wt_manager) = worktree_manager {
            let branch_name = format!("feature/{}", story_id);
            wt_manager.create_worktree(node_id, story_id, &branch_name).await?
        } else {
            return Err("Worktree manager not initialized".to_string());
        };
        
        // ✅ 发布 worktree.ready 事件
        event_bus.publish(EventType::WorktreeReady, serde_json::json!({
            "story_id": story_id,
            "node_id": node_id,
            "worktree_path": worktree_path,
        }))?;
        
        // ✅ 启动 Agent 执行任务
        let agent_id = format!("agent-{}-{}", node_id, story_id);
        // let daemon = daemon_manager.read().await; // 未使用 - TODO: 调用 daemon.start_agent()
        
        // TODO: 调用 daemon.start_agent() 启动实际的 AI CLI 进程
        log::info!("[DecentralizedNode:{}] Started agent {} for story {} at {}", 
            node_id, agent_id, story_id, worktree_path);
        
        // ✅ 发布 agent.started 事件
        event_bus.publish(EventType::AgentStarted, serde_json::json!({
            "agent_id": agent_id,
            "story_id": story_id,
            "node_id": node_id,
            "worktree_path": worktree_path,
        }))?;
        
        Ok(())
    }
    
    /// 自主决策逻辑: 根据负载决定是否接受任务
    async fn should_accept_story(node_id: &str, _max_concurrent: usize) -> bool {
        // TODO: 从 daemon_manager 获取当前运行的 Agent 数量
        // 这里简化为随机决策用于演示
        
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        // 模拟: 70% 概率接受任务
        let accept = rng.gen_bool(0.7);
        
        log::debug!("[DecentralizedNode:{}] Decision: accept={}", node_id, accept);
        
        accept
    }
    
    /// 停止 Node
    pub async fn stop(&mut self) -> Result<(), String> {
        if !self.is_running {
            return Err("Node is not running".to_string());
        }
        
        self.is_running = false;
        log::info!("[DecentralizedNode:{}] Stopping decentralized agent loop", self.config.node_id);
        
        // TODO: 优雅停止所有正在运行的 Agent
        
        Ok(())
    }
    
    /// 检查 Node 是否正在运行
    pub fn is_running(&self) -> bool {
        self.is_running
    }
}
