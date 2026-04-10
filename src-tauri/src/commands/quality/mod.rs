// Quality Commands 模块
// 将大型 quality.rs 拆分为多个子模块以提高可维护性

pub mod types;
pub mod parser;
pub mod consistency;
pub mod feasibility;
pub mod quality_check;
pub mod iteration;
#[allow(dead_code)]
pub mod feedback;
#[allow(dead_code)]
pub mod deep_analysis;
#[allow(dead_code)]
pub mod task_decomposition;
pub mod user_story;
pub mod user_story_parser;
pub mod user_story_ai_service;
pub mod user_story_streaming;
pub mod persistence;

// 重新导出所有公共 API，保持向后兼容
pub use consistency::*;
pub use feasibility::*;
pub use quality_check::*;
pub use iteration::*;
#[allow(unused_imports)]
pub use feedback::*;
#[allow(unused_imports)]
pub use deep_analysis::*;
#[allow(unused_imports)]
pub use task_decomposition::*;
pub use user_story::*;
pub use persistence::*;
