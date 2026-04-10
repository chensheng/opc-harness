// AI Commands 模块
// 将大型 ai.rs 拆分为多个子模块以提高可维护性

pub mod types;
pub mod parser;
pub mod api_key;
pub mod chat;
pub mod prd;
pub mod persona;
pub mod competitor;
pub mod marketing;
pub mod claude;
pub mod kimi;
pub mod glm;
pub mod provider_info;

// 重新导出所有公共 API，保持向后兼容
pub use api_key::*;
pub use chat::*;
pub use prd::*;
pub use persona::*;
pub use competitor::*;
pub use marketing::*;
pub use claude::*;
pub use kimi::*;
pub use glm::*;
pub use provider_info::*;
