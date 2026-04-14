//! 用户故事拆分提示词模板
//! 
//! 用于将 PRD 或产品需求文档拆分为符合 INVEST 原则的用户故事

use crate::commands::quality::types::ExistingStoryInfo;

/// 用户故事拆分提示词模板（基础版，不包含已有故事信息）
const USER_STORY_DECOMPOSITION_TEMPLATE: &str = r#"# 系统指令

你是一位经验丰富的敏捷开发专家和产品经理，擅长将产品需求拆分为符合 INVEST 原则的用户故事。

## 任务
请读取 @.opc-harness/PRD.md 获取 PRD 内容，将其拆分为符合 INVEST 原则的用户故事列表。

## ⚠️ 输出格式要求（非常重要）
**你必须以 Markdown 表格格式输出用户故事，不要输出 JSON！**

表格必须包含以下列：
- **序号**: 故事编号（US-001, US-002...）
- **标题**: 简洁的故事标题
- **角色**: 用户角色（As a...）
- **功能**: 想要的功能（I want...）
- **价值**: 业务价值（So that...）
- **优先级**: P0/P1/P2/P3
- **故事点**: 1/2/3/5/8/13
- **验收标准**: 3-5条验收标准，用分号分隔
- **模块**: 功能模块名称
- **标签**: 标签列表，用逗号分隔
- **依赖**: 依赖的故事编号，无则填"无"

### 表格示例

| 序号 | 标题 | 角色 | 功能 | 价值 | 优先级 | 故事点 | 验收标准 | 模块 | 标签 | 依赖 |
|------|------|------|------|------|--------|--------|----------|------|------|------|
| US-001 | 用户注册与登录 | 新用户 | 能够通过邮箱或手机号注册账号并登录系统 | 我可以访问系统的核心功能并保存我的个人数据 | P0 | 5 | 用户可以通过邮箱注册，收到验证邮件；用户可以通过手机号注册，收到短信验证码；登录成功后跳转到首页；密码强度验证（至少8位，包含字母和数字） | 用户认证 | 认证,核心功能 | 无 |
| US-002 | 创建任务 | 注册用户 | 能够创建新的任务并设置基本信息 | 我可以记录和管理我需要完成的工作 | P0 | 3 | 用户可以输入任务标题；用户可以设置任务描述；用户可以设置截止日期；创建后任务显示在任务列表中 | 任务管理 | 任务,核心功能 | US-001 |

---

## PRD 内容
请读取 @.opc-harness/PRD.md 文件获取完整的 PRD 内容。

**重要提示**: 
- 如果 PRD 内容非常长,请重点关注其中的**核心功能需求**和**用户场景**
- 忽略技术实现细节、UI设计描述等非功能性内容
- 只提取需要拆分为用户故事的业务需求部分

---

## INVEST 原则说明
- **I**ndependent（独立的）：每个故事应尽可能独立
- **N**egotiable（可协商的）：故事细节可以讨论和调整
- **V**aluable（有价值的）：对用户或客户有价值
- **E**stimable（可估算的）：工作量可以估算
- **S**mall（小的）：故事应该足够小，能在一个迭代内完成
- **T**estable（可测试的）：有明确的验收标准

## 用户故事标准格式
- **角色（Role）**：As a [具体用户角色]
- **功能（Feature）**：I want [具体功能或行为]
- **价值（Benefit）**：So that [业务价值或用户收益]

## 优先级定义
- **P0**：核心功能，必须实现，影响产品基本可用性
- **P1**：重要功能，应该在第一个版本中实现
- **P2**：增强功能，可以在后续版本中实现
- **P3**：锦上添花，低优先级功能

## 故事点估算参考
- **1-3 点**：简单任务，1-2 天完成
- **5 点**：中等复杂度，3-5 天完成
- **8 点**：较复杂，1-2 周完成
- **13 点**：非常复杂，需要分解为更小的故事

## 验收标准要求
每个故事必须包含 3-5 条具体的、可测试的验收标准。

## 注意事项
1. **角色具体化**：避免使用泛泛的"用户"，而是使用具体的角色如"新用户"、"管理员"、"付费用户"等
2. **功能明确**：功能描述应该清晰、具体，避免模糊表述
3. **价值导向**：强调业务价值，而不仅仅是技术实现
4. **合理拆分**：确保故事大小适中，既不过大也不过小
5. **依赖关系**：准确识别故事间的依赖关系
6. **模块划分**：合理划分功能模块，便于团队分工
7. **标签系统**：使用有意义的标签进行分类

---

## 最终要求

**请严格按照上述表格格式输出，确保：**
1. 表格包含所有必需的列
2. 每行代表一个完整的用户故事
3. 验收标准用分号（；）分隔多条
4. 标签用逗号（，）分隔多个
5. 没有依赖时填写"无"
6. 故事编号从 US-001 开始连续编号

**现在，请读取 @.opc-harness/PRD.md，以 Markdown 表格格式输出用户故事列表：**"#;

/// 生成用户故事拆分提示词（基础版）
/// 
/// # Returns
/// 返回完整的提示词字符串
pub fn generate_user_story_decomposition_prompt() -> String {
    USER_STORY_DECOMPOSITION_TEMPLATE.to_string()
}

/// 生成用户故事拆分提示词（包含已有故事信息，避免重复）
/// 
/// # Arguments
/// * `existing_stories` - 已有的用户故事列表（用于避免重复生成）
/// 
/// # Returns
/// 返回完整的提示词字符串
pub fn generate_user_story_decomposition_prompt_with_existing(
    existing_stories: &[ExistingStoryInfo],
) -> String {
    // 构建已有故事的文本描述
    let existing_stories_text = if existing_stories.is_empty() {
        String::from("当前没有已有的用户故事。")
    } else {
        let mut text = String::from("⚠️ **重要：以下用户故事已经存在，请不要重复生成类似的故事！**\n\n");
        text.push_str("| 序号 | 标题 | 角色 | 功能 |\n");
        text.push_str("|------|------|------|------|\n");
        
        for (idx, story) in existing_stories.iter().enumerate() {
            text.push_str(&format!(
                "| {} | {} | {} | {} |\n",
                idx + 1,
                story.title,
                story.role,
                story.feature
            ));
        }
        
        text.push_str("\n**请在拆分新故事时：**\n");
        text.push_str("1. 避免生成与上述故事重复或高度相似的内容\n");
        text.push_str("2. 如果需要补充或细化已有故事，请明确说明是改进而非新增\n");
        text.push_str("3. 重点关注 PRD 中尚未被覆盖的需求\n");
        
        text
    };
    
    // 在基础模板的基础上，添加已有故事的信息
    let base_prompt = USER_STORY_DECOMPOSITION_TEMPLATE.to_string();
    
    // 在"PRD 内容"之后插入已有故事信息
    let insert_marker = "---\n\n## INVEST 原则说明";
    let enhanced_section = format!(
        "---\n\n## 已有用户故事（避免重复）\n\n{}\n\n---\n\n## INVEST 原则说明",
        existing_stories_text
    );
    
    base_prompt.replace(insert_marker, &enhanced_section)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_contains_required_sections() {
        let prompt = generate_user_story_decomposition_prompt();
        
        assert!(prompt.contains("INVEST"));
        assert!(prompt.contains("As a"));
        assert!(prompt.contains("I want"));
        assert!(prompt.contains("So that"));
        assert!(prompt.contains("P0"));
        assert!(prompt.contains("验收标准"));
        assert!(prompt.contains("故事点"));
        assert!(prompt.contains("| 序号 |"));
        assert!(prompt.contains("@.opc-harness/PRD.md"));
    }

    #[test]
    fn test_template_not_empty() {
        assert!(!USER_STORY_DECOMPOSITION_TEMPLATE.is_empty());
        assert!(USER_STORY_DECOMPOSITION_TEMPLATE.len() > 1000);
    }

    #[test]
    fn test_prompt_generation() {
        let prompt = generate_user_story_decomposition_prompt();
        
        assert!(prompt.contains("@.opc-harness/PRD.md"));
        assert!(prompt.contains("任务")); // From template context or general expectation if specific PRD content was removed
    }
    
    #[test]
    fn test_prompt_with_existing_stories() {
        let existing_stories = vec![
            ExistingStoryInfo {
                title: "用户注册".to_string(),
                role: "新用户".to_string(),
                feature: "能够通过邮箱注册账号".to_string(),
            },
            ExistingStoryInfo {
                title: "创建任务".to_string(),
                role: "注册用户".to_string(),
                feature: "能够创建新的任务".to_string(),
            },
        ];
        
        let prompt = generate_user_story_decomposition_prompt_with_existing(
            &existing_stories
        );
        
        // 验证提示词包含已有故事信息
        assert!(prompt.contains("已有用户故事（避免重复）"));
        assert!(prompt.contains("用户注册"));
        assert!(prompt.contains("创建任务"));
        assert!(prompt.contains("不要重复生成类似的故事"));
        assert!(prompt.contains("PRD 内容"));
    }
    
    #[test]
    fn test_prompt_with_empty_existing_stories() {
        let existing_stories: Vec<ExistingStoryInfo> = vec![];
        
        let prompt = generate_user_story_decomposition_prompt_with_existing(
            &existing_stories
        );
        
        // 验证提示词包含空故事的提示
        assert!(prompt.contains("当前没有已有的用户故事"));
    }
}