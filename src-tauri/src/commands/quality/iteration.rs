use crate::commands::quality::types::{
    CreateInitialVersionRequest, CreateInitialVersionResponse, IterationRequest, IterationResponse,
    RollbackRequest, RollbackResponse,
};

/// 创建初始版本
#[tauri::command]
pub async fn create_initial_version(
    request: CreateInitialVersionRequest,
) -> Result<CreateInitialVersionResponse, String> {
    // 创建迭代管理器（简化实现，不使用全局状态）
    let mut manager = crate::quality::prd_iteration_manager::PRDIterationManager::new();
    let version_id = manager.create_initial_version(&request.prd_json);

    Ok(CreateInitialVersionResponse { version_id })
}

/// 执行 PRD 迭代
#[tauri::command]
pub async fn iterate_prd(request: IterationRequest) -> Result<IterationResponse, String> {
    let mut manager = crate::quality::prd_iteration_manager::PRDIterationManager::new();
    manager.iterate_with_feedback(&request)
}

/// 回滚到指定版本
#[tauri::command]
pub async fn rollback_to_version(
    _request: RollbackRequest, // 添加下划线前缀
) -> Result<RollbackResponse, String> {
    // let mut manager = PRDIterationManager::new();  // 已注释未使用的变量
    // 简化版本不支持回滚
    Err("简化版本不支持回滚".to_string())
}
