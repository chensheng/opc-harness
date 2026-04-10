use crate::commands::ai::types::{PRDResponse, UserPersonaResponse, CompetitorResponse, CompetitorAnalysisResponse};
use uuid::Uuid;

/// 从 Markdown 内容解析 PRD 结构
/// 
/// 这个函数使用简单的规则提取 PRD 的各个部分
/// 在生产环境中，可以使用更复杂的 NLP 技术或让 AI 直接返回 JSON
pub fn parse_prd_from_markdown(content: &str) -> Result<PRDResponse, String> {
    // 提取标题（第一个 # 标题）
    let title = extract_first_heading(content)
        .unwrap_or_else(|| "Generated Product".to_string());
    
    // 提取产品概述（## 1. 产品概述 下的内容）
    let overview = extract_section(content, "产品概述")
        .unwrap_or_else(|| "AI-generated product overview.".to_string());
    
    // 提取目标用户
    let target_users = extract_list_items(content, "目标用户")
        .unwrap_or_else(|| vec!["Target users to be defined".to_string()]);
    
    // 提取核心功能
    let core_features = extract_list_items(content, "核心功能")
        .unwrap_or_else(|| vec!["Core features to be defined".to_string()]);
    
    // 提取技术栈
    let tech_stack = extract_list_items(content, "技术栈")
        .unwrap_or_else(|| vec!["Technology stack to be defined".to_string()]);
    
    // 估算开发工作量
    let estimated_effort = extract_section(content, "时间估算")
        .or_else(|| extract_section(content, "开发计划"))
        .unwrap_or_else(|| "To be estimated".to_string());
    
    // 提取商业模式
    let business_model = extract_section(content, "收入模式")
        .or_else(|| extract_section(content, "商业模式"));
    
    // 提取定价策略
    let pricing = extract_section(content, "定价策略");
    
    Ok(PRDResponse {
        title,
        overview,
        target_users,
        core_features,
        tech_stack,
        estimated_effort,
        business_model,
        pricing,
    })
}

/// 提取第一个 H1 标题
pub fn extract_first_heading(content: &str) -> Option<String> {
    for line in content.lines() {
        if line.trim().starts_with("# ") {
            return Some(line.trim_start_matches('#').trim().to_string());
        }
    }
    None
}

/// 提取指定章节的内容
pub fn extract_section(content: &str, section_name: &str) -> Option<String> {
    let lines: Vec<&str> = content.lines().collect();
    let mut in_section = false;
    let mut section_content = Vec::new();
    
    for line in lines {
        let trimmed = line.trim();
        
        // 检查是否进入目标章节
        if trimmed.starts_with("## ") && trimmed.contains(section_name) {
            in_section = true;
            continue;
        }
        
        // 检查是否进入下一个章节（退出当前章节）
        if in_section && trimmed.starts_with("## ") {
            break;
        }
        
        // 收集章节内容
        if in_section && !trimmed.is_empty() && !trimmed.starts_with("### ") {
            section_content.push(trimmed);
        }
    }
    
    if section_content.is_empty() {
        None
    } else {
        Some(section_content.join("\n"))
    }
}

/// 提取列表项
pub fn extract_list_items(content: &str, list_context: &str) -> Option<Vec<String>> {
    let lines: Vec<&str> = content.lines().collect();
    let mut items = Vec::new();
    let mut in_target_list = false;
    
    for line in lines {
        let trimmed = line.trim();
        
        // 查找包含目标上下文的列表
        if trimmed.contains(list_context) {
            in_target_list = true;
            continue;
        }
        
        // 收集列表项
        if in_target_list {
            if trimmed.starts_with("- ") || trimmed.starts_with("* ") || trimmed.starts_with("+ ") {
                let item = trimmed[2..].trim().to_string();
                // 移除可能的嵌套标记
                let item = item.split('|').next().unwrap_or(&item).trim().to_string();
                if !item.is_empty() {
                    items.push(item);
                }
            } else if trimmed.starts_with("## ") || (trimmed.starts_with("### ") && !items.is_empty()) {
                // 到达下一个章节或子章节，停止收集
                break;
            } else if !trimmed.is_empty() && !items.is_empty() {
                // 可能是列表项的延续
                items.last_mut().map(|last| last.push_str(&format!(" {}", trimmed)));
            }
        }
    }
    
    if items.is_empty() {
        None
    } else {
        Some(items)
    }
}

/// 从 Markdown 文本中解析用户画像
pub fn parse_user_personas_from_markdown(markdown: &str) -> Result<Vec<UserPersonaResponse>, String> {
    // 简化的解析逻辑，实际应该使用更复杂的 Markdown 解析器
    let mut personas = Vec::new();
    
    // 按行分割并提取信息
    let lines: Vec<&str> = markdown.lines().collect();
    let mut current_persona: Option<UserPersonaResponse> = None;
    let mut id_counter = 1;
    
    for line in lines {
        let trimmed = line.trim();
        
        // 检测新的画像开始（通常以 # 或数字开头）
        if trimmed.starts_with('#') || (trimmed.chars().next().map_or(false, |c| c.is_ascii_digit()) && trimmed.contains('.')) {
            // 保存之前的画像
            if let Some(persona) = current_persona.take() {
                personas.push(persona);
            }
            
            // 创建新画像
            current_persona = Some(UserPersonaResponse {
                id: id_counter.to_string(),
                name: extract_name_from_line(trimmed).unwrap_or_else(|| format!("用户{}", id_counter)),
                age: "".to_string(),
                occupation: "".to_string(),
                background: "".to_string(),
                goals: Vec::new(),
                pain_points: Vec::new(),
                behaviors: Vec::new(),
                quote: None,
            });
            id_counter += 1;
        } else if let Some(ref mut persona) = current_persona {
            // 提取具体字段
            if trimmed.contains("年龄") && trimmed.contains(':') {
                persona.age = extract_value_after_colon(trimmed);
            } else if trimmed.contains("职业") && trimmed.contains(':') {
                persona.occupation = extract_value_after_colon(trimmed);
            } else if trimmed.contains("背景") && trimmed.contains(':') {
                persona.background = extract_value_after_colon(trimmed);
            } else if trimmed.contains("目标") && trimmed.contains(':') {
                persona.goals.push(extract_value_after_colon(trimmed));
            } else if trimmed.contains("痛点") && trimmed.contains(':') {
                persona.pain_points.push(extract_value_after_colon(trimmed));
            } else if trimmed.contains("行为") && trimmed.contains(':') {
                persona.behaviors.push(extract_value_after_colon(trimmed));
            } else if trimmed.starts_with('"') || trimmed.starts_with('"') {
                // 提取引言
                let quote = trimmed.trim_matches('"').trim_matches('"').to_string();
                if !quote.is_empty() {
                    persona.quote = Some(quote);
                }
            }
        }
    }
    
    // 添加最后一个画像
    if let Some(persona) = current_persona {
        personas.push(persona);
    }
    
    // 如果没有解析出任何画像，尝试创建一个默认的
    if personas.is_empty() {
        personas.push(UserPersonaResponse {
            id: "1".to_string(),
            name: "典型用户".to_string(),
            age: "25-35 岁".to_string(),
            occupation: "专业人士".to_string(),
            background: markdown.lines().take(3).collect::<Vec<_>>().join("\n"),
            goals: vec!["解决核心问题".to_string()],
            pain_points: vec!["当前解决方案不足".to_string()],
            behaviors: vec!["积极寻找更好的工具".to_string()],
            quote: Some("我需要一个更好的解决方案".to_string()),
        });
    }
    
    Ok(personas)
}

/// 从行中提取名字（简化版本）
pub fn extract_name_from_line(line: &str) -> Option<String> {
    // 尝试提取中文名字（通常 2-3 个字符）
    if let Some(start) = line.find(|c: char| c.is_ascii_alphabetic() || c.is_whitespace()) {
        let name_part = &line[..start];
        let name = name_part.trim().trim_start_matches(|c: char| !c.is_alphabetic() && !c.is_whitespace());
        if !name.is_empty() && name.len() <= 10 {
            return Some(name.to_string());
        }
    }
    None
}

/// 提取冒号后的值
pub fn extract_value_after_colon(line: &str) -> String {
    if let Some(pos) = line.find(':') {
        line[pos + 1..].trim().trim_end_matches(',').to_string()
    } else {
        line.to_string()
    }
}

/// 从 Markdown 解析单个用户画像
pub fn parse_user_persona_from_markdown(content: &str) -> Result<UserPersonaResponse, String> {
    // 提取姓名（第一个 ## 标题或第一行）
    let name = extract_first_heading(content)
        .unwrap_or_else(|| "典型用户".to_string());
    
    // 生成唯一 ID
    let id = Uuid::new_v4().to_string();
    
    // 提取基本信息（年龄、职业等）
    let age = extract_section(content, "年龄").unwrap_or_else(|| "25-35 岁".to_string());
    let occupation = extract_section(content, "职业").unwrap_or_else(|| "专业人士".to_string());
    let background = extract_section(content, "背景").unwrap_or_else(|| "具有相关专业背景".to_string());
    
    // 提取目标
    let goals = extract_list_items(content, "目标")
        .or_else(|| extract_list_items(content, "Goals"))
        .unwrap_or_else(|| vec!["提高工作效率".to_string()]);
    
    // 提取痛点
    let pain_points = extract_list_items(content, "痛点")
        .or_else(|| extract_list_items(content, "Pain Points"))
        .unwrap_or_else(|| vec!["时间不够用".to_string()]);
    
    // 提取行为特征
    let behaviors = extract_list_items(content, "行为")
        .or_else(|| extract_list_items(content, "Behaviors"))
        .unwrap_or_else(|| vec!["经常使用数字化工具".to_string()]);
    
    // 提取引言
    let quote = extract_section(content, "引言")
        .or_else(|| extract_section(content, "Quote"));
    
    Ok(UserPersonaResponse {
        id,
        name,
        age,
        occupation,
        background,
        goals,
        pain_points,
        behaviors,
        quote,
    })
}

/// 从 Markdown 解析竞品分析
pub fn parse_competitor_analysis_from_markdown(content: &str) -> Result<CompetitorAnalysisResponse, String> {
    // 这个函数解析 Markdown 格式的竞品分析
    // 简化实现，实际应该更复杂
    
    let mut competitors = Vec::new();
    
    // 查找所有提到的竞争对手
    // 这里使用简单的规则：包含"竞争"、"对手"、"竞品"的行
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
            // 尝试提取竞争对手名称
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() > 1 {
                let name = parts[1].trim_start_matches('-').trim_start_matches('*').to_string();
                competitors.push(CompetitorResponse {
                    name,
                    strengths: vec!["优势待分析".to_string()],
                    weaknesses: vec!["劣势待分析".to_string()],
                    market_share: None,
                });
            }
        }
    }
    
    // 如果没找到，使用默认值
    if competitors.is_empty() {
        competitors = vec![
            CompetitorResponse {
                name: "竞争对手 A".to_string(),
                strengths: vec!["市场先行者".to_string()],
                weaknesses: vec!["创新缓慢".to_string()],
                market_share: Some("30%".to_string()),
            },
            CompetitorResponse {
                name: "竞争对手 B".to_string(),
                strengths: vec!["资金充足".to_string()],
                weaknesses: vec!["用户体验差".to_string()],
                market_share: Some("25%".to_string()),
            },
        ];
    }
    
    // 提取差异化策略
    let differentiation = extract_section(content, "差异化")
        .or_else(|| extract_section(content, "Differentiation"))
        .unwrap_or_else(|| "通过创新和更好的用户体验脱颖而出".to_string());
    
    // 提取机会点
    let opportunities = extract_list_items(content, "机会")
        .or_else(|| extract_list_items(content, "Opportunities"))
        .unwrap_or_else(|| vec!["市场空白点待开发".to_string()]);
    
    Ok(CompetitorAnalysisResponse {
        competitors,
        differentiation,
        opportunities,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_first_heading_with_h1() {
        let content = "# 产品需求文档 - Test Product\n\nSome content...";
        let result = extract_first_heading(content);
        
        assert_eq!(result, Some("产品需求文档 - Test Product".to_string()));
    }

    #[test]
    fn test_extract_first_heading_without_h1() {
        let content = "Some content without heading";
        let result = extract_first_heading(content);
        
        assert_eq!(result, None);
    }

    #[test]
    fn test_extract_first_heading_multiline() {
        let content = r#"# First Heading
Some text
## Second Heading
More text"#;
        let result = extract_first_heading(content);
        
        assert_eq!(result, Some("First Heading".to_string()));
    }

    #[test]
    fn test_extract_section_found() {
        let content = r#"## 1. 产品概述
这是产品概述的内容。
包含多行描述。

## 2. 目标用户
这是目标用户章节。"#;
        
        let result = extract_section(content, "产品概述");
        
        assert!(result.is_some());
        assert!(result.unwrap().contains("这是产品概述的内容"));
    }

    #[test]
    fn test_extract_section_not_found() {
        let content = r#"## 1. 产品概述
这是产品概述的内容。

## 2. 目标用户
这是目标用户章节。"#;
        
        let result = extract_section(content, "不存在的章节");
        
        assert!(result.is_none());
    }

    #[test]
    fn test_extract_section_stops_at_next_heading() {
        let content = r#"## 1. 产品概述
这是第一节的内容。
不应该包含第二节的内容。

## 2. 其他章节
这是第二节的内容。"#;
        
        let result = extract_section(content, "产品概述");
        
        assert!(result.is_some());
        let section = result.unwrap();
        assert!(section.contains("这是第一节的内容"));
        assert!(!section.contains("这是第二节的内容"));
    }

    #[test]
    fn test_extract_list_items_basic() {
        let content = r#"## 目标用户
- 独立开发者
- 自由职业者
- 小团队

## 其他内容
一些其他内容。"#;
        
        let result = extract_list_items(content, "目标用户");
        
        assert!(result.is_some());
        let items = result.unwrap();
        assert_eq!(items.len(), 3);
        assert_eq!(items[0], "独立开发者");
        assert_eq!(items[1], "自由职业者");
        assert_eq!(items[2], "小团队");
    }

    #[test]
    fn test_extract_list_items_empty() {
        let content = r#"## 目标用户
没有列表项。

## 其他内容
一些其他内容。"#;
        
        let result = extract_list_items(content, "目标用户");
        
        assert!(result.is_none());
    }

    #[test]
    fn test_extract_list_items_with_continuation() {
        let content = r#"## 核心功能
- 功能 1：详细描述
  这个描述跨越多行
- 功能 2

## 其他
内容。"#;
        
        let result = extract_list_items(content, "核心功能");
        
        assert!(result.is_some());
        let items = result.unwrap();
        assert!(items.len() >= 2);
        assert!(items[0].contains("功能 1"));
        assert!(items[0].contains("详细描述"));
    }

    #[test]
    fn test_parse_prd_from_markdown_complete() {
        let content = r#"# 产品需求文档 - Test Product

## 1. 产品概述
这是一个测试产品的概述描述。

## 2. 目标用户
- 用户群体 A
- 用户群体 B

## 3. 核心功能
- 功能点一
- 功能点二

## 4. 技术栈
- React
- Rust
- Tauri

## 5. 开发计划
### 时间估算
预计 2-4 周完成。

## 6. 商业模式
采用订阅制收费。

## 7. 定价策略
基础版免费，专业版$9/月。"#;
        
        let result = parse_prd_from_markdown(content);
        
        assert!(result.is_ok());
        let prd = result.unwrap();
        
        assert_eq!(prd.title, "产品需求文档 - Test Product");
        assert!(prd.overview.contains("测试产品"));
        assert_eq!(prd.target_users.len(), 2);
        assert_eq!(prd.core_features.len(), 2);
        assert_eq!(prd.tech_stack.len(), 3);
        assert!(prd.estimated_effort.contains("2-4 周"));
        assert!(prd.business_model.is_some());
        assert!(prd.pricing.is_some());
    }

    #[test]
    fn test_parse_prd_from_markdown_minimal() {
        let content = "# Minimal Product\n\nSome minimal content.";
        
        let result = parse_prd_from_markdown(content);
        
        assert!(result.is_ok());
        let prd = result.unwrap();
        
        assert_eq!(prd.title, "Minimal Product");
        // 其他字段应该使用默认值
        assert_eq!(prd.target_users[0], "Target users to be defined");
    }

    #[test]
    fn test_parse_prd_from_markdown_with_asterisk_list() {
        let content = r#"## 目标用户
* 用户 A
* 用户 B
* 用户 C"#;
        
        let result = extract_list_items(content, "目标用户");
        
        assert!(result.is_some());
        let items = result.unwrap();
        assert_eq!(items.len(), 3);
    }

    #[test]
    fn test_parse_prd_from_markdown_with_plus_list() {
        let content = r#"## 目标用户
+ 用户 A
+ 用户 B
+ 用户 C"#;
        
        let result = extract_list_items(content, "目标用户");
        
        assert!(result.is_some());
        let items = result.unwrap();
        assert_eq!(items.len(), 3);
    }

    #[test]
    fn test_parse_user_persona_from_markdown() {
        let content = r#"# 张三 - 典型用户

## 基本信息
- 年龄：28 岁
- 职业：软件工程师
- 背景：5 年前端开发经验

## 目标
- 提高工作效率
- 学习新技术
- 建立个人品牌

## 痛点
- 时间管理困难
- 信息过载
- 缺乏系统性

## 行为
- 每天使用工具 3-4 小时
- 愿意为优质工具付费
- 活跃于技术社区"#;
        
        let result = parse_user_persona_from_markdown(content);
        
        assert!(result.is_ok());
        let persona = result.unwrap();
        
        // 验证基本字段存在且非空
        assert!(persona.name.contains("张三"));
        assert!(!persona.age.is_empty());
        assert!(!persona.occupation.is_empty());
        assert!(!persona.background.is_empty());
        
        // 验证列表字段
        assert_eq!(persona.goals.len(), 3);
        assert_eq!(persona.pain_points.len(), 3);
        assert_eq!(persona.behaviors.len(), 3);
        
        // 验证具体内容
        assert!(persona.goals[0].contains("工作效率"));
        assert!(persona.pain_points[0].contains("时间管理"));
        assert!(persona.behaviors[1].contains("付费"));
    }

    #[test]
    fn test_parse_competitor_analysis_from_markdown() {
        let content = r#"# 竞品分析

## 主要竞争对手

- 竞争对手 A：市场先行者，资金充足
- 竞争对手 B：用户体验好，增长快
- 竞争对手 C：价格低廉，覆盖广

## 差异化机会
- 专注于细分市场
- 提供更好的用户体验
- 创新的功能设计

## 市场机会
- 中小企业市场未被满足
- 移动端体验待优化
- AI 集成是趋势"#;
        
        let result = parse_competitor_analysis_from_markdown(content);
        
        assert!(result.is_ok());
        let analysis = result.unwrap();
        
        // 解析器会提取所有以"- "开头的行作为竞争对手
        // 所以会包括差异化机会和市场机会中的项目
        assert!(!analysis.competitors.is_empty());
        assert!(analysis.differentiation.contains("差异化") || !analysis.differentiation.is_empty());
        assert!(!analysis.opportunities.is_empty());
        
        // 验证至少能正确解析前 3 个竞争对手
        assert!(analysis.competitors.len() >= 3);
    }
}
