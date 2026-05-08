use crate::commands::quality::parser::parse_markdown_to_quality_prd;

/// 检查 PRD 质量
#[tauri::command]
pub async fn check_prd_quality(
    prd_content: String,
) -> Result<crate::quality::prd_checker::PRDQualityReport, String> {
    // 1. 解析 Markdown 内容为 PRDDocument
    let prd = parse_markdown_to_quality_prd(&prd_content);

    // 2. 创建质量检查器
    let checker = crate::quality::prd_checker::PRDQualityChecker::with_defaults();

    // 3. 执行质量检查
    let report = checker.check_quality(&prd);

    Ok(report)
}
