// AI Commands 模块
// 将大型 ai.rs 拆分为多个子模块以提高可维护性

pub mod api_key;
pub mod chat;
pub mod claude;
pub mod competitor;
pub mod error_handler;
pub mod glm;
pub mod kimi;
pub mod marketing;
pub mod parser;
pub mod persona;
pub mod prd;
pub mod provider_info;
pub mod types;

// 重新导出所有公共 API，保持向后兼容
pub use api_key::*;
pub use chat::*;
pub use claude::*;
pub use competitor::*;
pub use glm::*;
pub use kimi::*;
pub use marketing::*;
pub use persona::*;
pub use prd::*;
pub use provider_info::*;
// error_handler 中的函数可以直接通过路径访问，无需重新导出
