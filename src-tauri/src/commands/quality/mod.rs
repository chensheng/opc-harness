// Quality Commands 模块
// 将大型 quality.rs 拆分为多个子模块以提高可维护性

pub mod consistency;
#[allow(dead_code)]
pub mod deep_analysis;
pub mod feasibility;
#[allow(dead_code)]
pub mod feedback;
pub mod iteration;
pub mod parser;
pub mod persistence;
pub mod quality_check;
pub mod sprint_assignment;
pub mod sprint_assignment_streaming;
#[allow(dead_code)]
pub mod task_decomposition;
pub mod types;
pub mod user_story;
pub mod user_story_ai_service;
pub mod user_story_parser;
pub mod user_story_streaming;

// 重新导出所有公共 API，保持向后兼容
pub use consistency::*;
#[allow(unused_imports)]
pub use deep_analysis::*;
pub use feasibility::*;
#[allow(unused_imports)]
pub use feedback::*;
pub use iteration::*;
pub use persistence::*;
pub use sprint_assignment::*;
#[allow(unused_imports)]
pub use task_decomposition::*;
pub use user_story::*;
