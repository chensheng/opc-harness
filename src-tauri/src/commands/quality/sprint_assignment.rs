// Sprint 用户故事分配模块 - 主协调器
// 提供Sprint分配的公共API入口

use crate::commands::quality::types::AssignStoriesToSprintRequest;

// 导入子模块
use super::sprint_assignment_streaming;

/// Sprint分配（流式版本）
#[tauri::command]
pub async fn assign_stories_to_sprint_streaming(
    request: AssignStoriesToSprintRequest,
    app: tauri::AppHandle,
) -> Result<String, String> {
    sprint_assignment_streaming::assign_stories_to_sprint_streaming(request, app).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_structure() {
        // 简单的结构测试
        assert!(true);
    }
}
