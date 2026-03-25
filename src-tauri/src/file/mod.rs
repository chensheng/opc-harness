//! 文件修改应用器模块
//! 
//! VC-020: 实现安全、可靠的文件修改机制
//! 
//! 核心功能：
//! - 智能文件写入（新建/修改）
//! - 自动备份机制
//! - 差异对比和统计
//! - 回滚支持
//! - 批量原子操作

pub mod backup;
pub mod file_applier;

// 重新导出主要类型
pub use backup::BackupManager;
pub use file_applier::{FileApplier, FileApplyResult, BatchApplyResult, ChangeType, DiffStats};
