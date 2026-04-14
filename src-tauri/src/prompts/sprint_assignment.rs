//! Sprint用户故事分配提示词模板
//! 
//! 用于AI分析并推荐最适合分配到指定Sprint的用户故事

/// Sprint分配系统提示词模板（用于AGENTS.md）
pub fn generate_sprint_assignment_system_prompt() -> String {
    r#"# 系统指令

你是一位专业的敏捷开发教练，擅长Sprint规划和用户故事优先级排序。

## 任务
请分析给定的Sprint信息和未分配的用户故事，推荐最适合分配到该Sprint的故事。

## ⚠️ 输出格式要求（非常重要）
**你必须以 Markdown 表格格式输出推荐结果，不要输出 JSON！**

表格必须包含以下列：
- **故事ID**: 用户故事的ID（如 US-001）
- **推荐理由**: 详细说明为什么这个故事适合这个Sprint，如果用户提供了建议，需要说明如何遵循了这些建议
- **置信度**: 0-100的整数，反映你对推荐的确定程度

### 表格示例

| 故事ID | 推荐理由 | 置信度 |
|--------|----------|--------|
| US-001 | 这是P0优先级故事，与Sprint目标"提升用户体验"高度匹配，且无外部依赖，可以快速完成 | 95 |
| US-003 | 虽然优先级为P1，但故事点较小(3点)，可以快速完成，符合用户建议的"优先处理小功能" | 85 |
| US-005 | 该故事与US-001有技术协同效应，可以复用部分代码，提高开发效率 | 80 |

---

## 考虑因素
1. **优先级**：P0 > P1 > P2 > P3
2. **故事之间的依赖关系**：优先推荐低依赖或依赖已满足的故事
3. **业务价值和Sprint目标的匹配度**：优先推荐与Sprint目标高度相关的故事
4. **技术实现的可行性**：考虑技术复杂度和风险
5. **用户的特殊建议和约束**：严格遵循用户提出的任何特殊要求

## 重要提示
- 只返回Markdown表格，不要有其他多余内容
- 推荐理由要具体、有说服力
- 置信度反映你对推荐的确定程度
- 优先推荐高优先级、高价值、低依赖的故事
- 严格遵循用户提出的任何特殊要求或约束
"#.to_string()
}

/// Sprint分配用户提示词模板（包含Sprint信息、用户故事和用户建议）
#[cfg(test)]
pub fn generate_sprint_assignment_user_prompt(
    sprint_name: &str,
    sprint_goal: Option<&str>,
    start_date: &str,
    end_date: &str,
    total_points: u32,
    completed_points: u32,
    _remaining_points: u32,
    stories_info: &str,
    user_suggestions: Option<&str>,
) -> String {
    let goal_text = sprint_goal.unwrap_or("未设置");
    
    let mut prompt = format!(
r#"# Sprint信息

- **名称**：{sprint_name}
- **目标**：{goal_text}
- **时间范围**：{start_date} 至 {end_date}
- **当前容量**：{total_points} 故事点
- **已完成**：{completed_points} 故事点

# 未分配的用户故事列表

{stories_info}
"#
    );

    // 添加用户建议部分（如果有）
    if let Some(suggestions) = user_suggestions {
        if !suggestions.trim().is_empty() {
            prompt.push_str(&format!(
r#"
# 用户的分配建议和特殊要求

{suggestions}

**请特别注意并优先考虑上述用户建议，在推荐时充分考虑这些约束条件和要求。**
"#
            ));
        }
    }

    prompt.push_str(
r#"
# 任务要求

请分析以上Sprint信息和未分配的用户故事，推荐最适合分配到该Sprint的故事。

请将推荐结果以Markdown表格格式保存到 SPRINT.md 文件中。
"#
    );

    prompt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_prompt_not_empty() {
        let prompt = generate_sprint_assignment_system_prompt();
        assert!(!prompt.is_empty());
        assert!(prompt.contains("Markdown 表格"));
        assert!(prompt.contains("故事ID"));
        assert!(prompt.contains("推荐理由"));
        assert!(prompt.contains("置信度"));
    }

    #[test]
    fn test_user_prompt_generation() {
        let prompt = generate_sprint_assignment_user_prompt(
            "Sprint 1",
            Some("实现核心功能"),
            "2024-01-01",
            "2024-01-14",
            20,
            5,
            15,
            "US-001: 用户登录",
            Some("优先处理认证相关功能"),
        );
        
        assert!(prompt.contains("Sprint 1"));
        assert!(prompt.contains("实现核心功能"));
        assert!(prompt.contains("US-001"));
        assert!(prompt.contains("优先处理认证相关功能"));
        // Ensure remaining capacity info is removed
        assert!(!prompt.contains("剩余容量"));
    }
}
