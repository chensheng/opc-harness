use crate::commands::quality::parser::parse_markdown_to_consistency_prd;
use crate::commands::quality::types::{CheckPRDConsistencyRequest, CheckPRDConsistencyResponse};

/// 检查 PRD 一致性
#[tauri::command]
pub async fn check_prd_consistency(
    request: CheckPRDConsistencyRequest,
) -> Result<CheckPRDConsistencyResponse, String> {
    // 解析 Markdown 内容为 PRDDocument
    let prd = parse_markdown_to_consistency_prd(&request.prd_content);

    // 创建检查器并执行检查
    let checker = crate::quality::prd_consistency_checker::PRDConsistencyChecker::new();
    let report = checker.check_consistency(&prd);

    Ok(report)
}
