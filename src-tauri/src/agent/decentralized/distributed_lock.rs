/// 去中心化智能体系统 - 分布式锁管理器 (纯内存实现)
/// 
/// 基于 DashMap + tokio::sync::Mutex 实现进程内分布式锁,无需 Redis

use dashmap::DashMap;
use std::sync::Arc;
use tokio::time::{Duration, Instant};
use log;

/// 锁信息
struct LockInfo {
    node_id: String,
    acquired_at: Instant,
    ttl_secs: u64,
}

impl LockInfo {
    fn is_expired(&self) -> bool {
        self.acquired_at.elapsed() > Duration::from_secs(self.ttl_secs)
    }
}

/// 分布式锁管理器 (所有 Node 实例共享)
pub struct SharedLockManager {
    locks: DashMap<String, Arc<tokio::sync::Mutex<LockInfo>>>,
    default_ttl_secs: u64,
}

impl SharedLockManager {
    /// 创建新的锁管理器
    pub fn new(default_ttl_secs: u64) -> Self {
        Self {
            locks: DashMap::new(),
            default_ttl_secs,
        }
    }
    
    /// 尝试获取 Story 的分布式锁
    pub async fn try_lock_story(&self, story_id: &str, node_id: &str) -> Result<bool, String> {
        let lock_key = format!("story_lock:{}", story_id);
        
        // 检查是否已有锁
        if let Some(existing_lock) = self.locks.get(&lock_key) {
            let lock_guard = existing_lock.lock().await;
            
            // 检查锁是否过期
            if lock_guard.is_expired() {
                log::debug!("[Lock] Lock for story {} expired, releasing", story_id);
                drop(lock_guard);
                self.locks.remove(&lock_key);
            } else {
                // 锁仍有效
                return Ok(false);
            }
        }
        
        // 创建新锁
        let lock_info = LockInfo {
            node_id: node_id.to_string(),
            acquired_at: Instant::now(),
            ttl_secs: self.default_ttl_secs,
        };
        
        let mutex = Arc::new(tokio::sync::Mutex::new(lock_info));
        
        // 尝试插入 (如果其他线程已插入,则失败)
        if self.locks.insert(lock_key.clone(), mutex).is_none() {
            log::info!("[Lock] Node {} acquired lock for story {}", node_id, story_id);
            Ok(true)
        } else {
            log::debug!("[Lock] Story {} already locked by another node", story_id);
            Ok(false)
        }
    }
    
    /// 释放锁
    pub async fn release_lock(&self, story_id: &str, node_id: &str) -> Result<(), String> {
        let lock_key = format!("story_lock:{}", story_id);
        
        if let Some((_, existing_lock)) = self.locks.remove(&lock_key) {
            let lock_guard = existing_lock.lock().await;
            
            if lock_guard.node_id == node_id {
                log::info!("[Lock] Node {} released lock for story {}", node_id, story_id);
                Ok(())
            } else {
                log::warn!("[Lock] Node {} tried to release lock for story {} but doesn't own it (owned by {})", 
                    node_id, story_id, lock_guard.node_id);
                
                // 重新插入锁
                self.locks.insert(lock_key, existing_lock);
                Err("Lock not owned by this node".to_string())
            }
        } else {
            log::warn!("[Lock] No lock found for story {}", story_id);
            Ok(()) // 锁不存在视为成功
        }
    }
    
    /// 续期锁
    pub async fn renew_lock(&self, story_id: &str, node_id: &str) -> Result<bool, String> {
        let lock_key = format!("story_lock:{}", story_id);
        
        if let Some(existing_lock) = self.locks.get(&lock_key) {
            let mut lock_guard = existing_lock.lock().await;
            
            if lock_guard.node_id == node_id {
                lock_guard.acquired_at = Instant::now();
                log::debug!("[Lock] Node {} renewed lock for story {}", node_id, story_id);
                Ok(true)
            } else {
                log::warn!("[Lock] Node {} failed to renew lock for story {} (owned by {})", 
                    node_id, story_id, lock_guard.node_id);
                Ok(false)
            }
        } else {
            log::warn!("[Lock] No lock found for story {}", story_id);
            Ok(false)
        }
    }
    
    /// 检查锁是否被当前节点持有
    pub async fn is_lock_held_by_node(&self, story_id: &str, node_id: &str) -> Result<bool, String> {
        let lock_key = format!("story_lock:{}", story_id);
        
        if let Some(existing_lock) = self.locks.get(&lock_key) {
            let lock_guard = existing_lock.lock().await;
            Ok(lock_guard.node_id == node_id && !lock_guard.is_expired())
        } else {
            Ok(false)
        }
    }
}
