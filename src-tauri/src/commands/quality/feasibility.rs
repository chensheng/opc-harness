use crate::commands::quality::types::AssessPRDFeasibilityRequest;
use crate::commands::quality::parser::parse_markdown_to_feasibility_prd;

/// 评估 PRD 可行性
#[tauri::command]
pub async fn assess_prd_feasibility(
    request: AssessPRDFeasibilityRequest,
) -> Result<crate::quality::prd_feasibility_assessor::PRDFeasibilityReport, String> {
    // 解析 Markdown 内容为 PRDDocument
    let prd = parse_markdown_to_feasibility_prd(&request.prd_content);
    
    // 创建评估器并执行评估
    let assessor = crate::quality::prd_feasibility_assessor::PRDFeasibilityAssessor::new();
    let report = assessor.assess_feasibility(&prd);
    
    Ok(report)
}
