//! Agent Tools Module
//!
//! 提供 Native Coding Agent 使用的各种工具集：
//! - 文件系统操作工具
//! - Git 版本控制工具
//! - 代码质量检查工具
//! - 代码搜索工具
//! - 依赖管理工具

pub mod code_search;
pub mod dependency_manager;
pub mod filesystem;
pub mod git;
pub mod quality;

// 重新导出主要结构体
pub use code_search::CodeSearchTools;
pub use dependency_manager::{DependencyInstallResult, DependencyManager, PackageManager};
pub use filesystem::FileSystemTools;
pub use git::GitTools;
pub use quality::{QualityCheckResult, QualityTools};
