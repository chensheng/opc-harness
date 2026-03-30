/// PRD 质量检查相关的 Tauri Commands

use crate::quality::prd_checker::{PRDDocument, PRDQualityChecker, PRDQualityReport};
use tauri::command;

/// 检查 PRD 质量
/// 
/// # Arguments
/// * `prd_content` - PRD 的 Markdown 内容
/// 
/// # Returns
/// * `PRDQualityReport` - 质量检查报告
#[command]
pub async fn check_prd_quality(prd_content: String) -> Result<PRDQualityReport, String> {
    // 1. 解析 Markdown 内容为 PRDDocument
    let prd = parse_markdown_to_prd(&prd_content);

    // 2. 创建质量检查器
    let checker = PRDQualityChecker::with_defaults();

    // 3. 执行质量检查
    let report = checker.check_quality(&prd);

    Ok(report)
}

/// 简单的 Markdown 解析器（基础实现）
/// 
/// 实际项目中应该使用 pulldown-cmark 等库进行更完善的解析
fn parse_markdown_to_prd(content: &str) -> PRDDocument {
    let mut title = None;
    let mut overview = None;
    let mut target_users = None;
    let mut core_features = None;
    let mut tech_stack = None;
    let mut estimated_effort = None;

    let lines: Vec<&str> = content.lines().collect();
    let mut current_section = String::new();
    let mut current_content = Vec::new();

    for line in lines {
        let trimmed = line.trim();

        // 检测章节标题 (# 开头)
        if trimmed.starts_with('#') {
            // 保存之前的章节内容
            if !current_section.is_empty() {
                save_section(&current_section, &current_content, &mut title, &mut overview, &mut target_users, &mut core_features, &mut tech_stack, &mut estimated_effort);
            }

            // 开始新章节
            current_section = trimmed.trim_start_matches('#').trim().to_lowercase();
            current_content.clear();
        } else if !trimmed.is_empty() {
            // 收集章节内容
            current_content.push(trimmed);
        }
    }

    // 保存最后一个章节
    if !current_section.is_empty() {
        save_section(&current_section, &current_content, &mut title, &mut overview, &mut target_users, &mut core_features, &mut tech_stack, &mut estimated_effort);
    }

    PRDDocument {
        title,
        overview,
        target_users,
        core_features,
        tech_stack,
        estimated_effort,
    }
}

/// 保存章节内容到对应的字段
fn save_section(
    section: &str,
    content: &[&str],
    title: &mut Option<String>,
    overview: &mut Option<String>,
    target_users: &mut Option<Vec<String>>,
    core_features: &mut Option<Vec<String>>,
    tech_stack: &mut Option<Vec<String>>,
    estimated_effort: &mut Option<String>,
) {
    let text = content.join("\n");

    match section {
        s if s.contains("产品标题") || s.contains("title") => {
            *title = Some(text);
        }
        s if s.contains("概述") || s.contains("overview") || s.contains("介绍") => {
            *overview = Some(text);
        }
        s if s.contains("用户") || s.contains("target") || s.contains("persona") => {
            *target_users = Some(parse_list_items(&text));
        }
        s if s.contains("功能") || s.contains("feature") || s.contains("需求") => {
            *core_features = Some(parse_list_items(&text));
        }
        s if s.contains("技术") || s.contains("tech") || s.contains("stack") => {
            *tech_stack = Some(parse_list_items(&text));
        }
        s if s.contains("工作量") || s.contains("effort") || s.contains("时间") => {
            *estimated_effort = Some(text);
        }
        _ => {}
    }
}

/// 解析列表项（支持 -, *, • 等标记）
fn parse_list_items(text: &str) -> Vec<String> {
    let mut items = Vec::new();

    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('-') || trimmed.starts_with('*') || trimmed.starts_with('•') {
            let item = trimmed[1..].trim().to_string();
            if !item.is_empty() {
                items.push(item);
            }
        } else if !trimmed.is_empty() && !trimmed.contains('#') {
            // 也收集没有列表标记的行
            items.push(trimmed.to_string());
        }
    }

    items
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_prd() {
        let markdown = r#"
# 产品标题
测试产品

# 产品概述
这是一个测试产品

# 目标用户
- 用户 A
- 用户 B

# 核心功能
- 功能 1
- 功能 2
- 功能 3

# 技术栈
- React
- Rust

# 预估工作量
2 周
"#;

        let prd = parse_markdown_to_prd(markdown);
        assert_eq!(prd.title, Some("测试产品".to_string()));
        assert_eq!(prd.overview, Some("这是一个测试产品".to_string()));
        assert!(prd.target_users.is_some());
        assert_eq!(prd.target_users.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_parse_list_items_basic() {
        let text = r#"
- 项目 1
- 项目 2
- 项目 3
"#;

        let items = parse_list_items(text);
        assert_eq!(items.len(), 3);
        assert_eq!(items[0], "项目 1");
        assert_eq!(items[1], "项目 2");
    }

    #[test]
    fn test_parse_list_items_mixed() {
        let text = r#"
- 项目 1
普通文本
* 项目 2
• 项目 3
"#;

        let items = parse_list_items(text);
        assert_eq!(items.len(), 4);
    }
}
