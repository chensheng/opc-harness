/// 测试工具模块
/// 
/// 提供跨文件共享的测试互斥锁和清理工具

use std::sync::Mutex;
use std::path::PathBuf;

/// 全局测试互斥锁，用于保护对环境变量的并发访问
pub static TEST_MUTEX: Mutex<()> = Mutex::new(());

/// RAII 守卫，确保测试结束后清理临时目录和环境变量
pub struct TestCleanup {
    pub temp_dir: PathBuf,
}

impl TestCleanup {
    /// 创建新的测试清理守卫
    pub fn new(temp_dir: PathBuf) -> Self {
        Self { temp_dir }
    }
}

impl Drop for TestCleanup {
    fn drop(&mut self) {
        // 即使测试失败或 panic，也会执行清理
        std::env::remove_var("OPC_HARNESS_HOME");
        std::fs::remove_dir_all(&self.temp_dir).ok();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cleanup_guard_removes_directory() {
        let temp_dir = std::env::temp_dir().join(format!("test-cleanup-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&temp_dir).unwrap();
        
        {
            let _cleanup = TestCleanup::new(temp_dir.clone());
            assert!(temp_dir.exists());
        }
        
        // _cleanup 已 drop，目录应该被删除
        assert!(!temp_dir.exists());
    }
    
    #[test]
    fn test_cleanup_guard_removes_env_var() {
        std::env::set_var("OPC_HARNESS_HOME", "/tmp/test");
        
        {
            let temp_dir = std::env::temp_dir().join(format!("test-env-{}", uuid::Uuid::new_v4()));
            let _cleanup = TestCleanup::new(temp_dir);
        }
        
        // 环境变量应该被移除
        assert!(std::env::var("OPC_HARNESS_HOME").is_err());
    }
}