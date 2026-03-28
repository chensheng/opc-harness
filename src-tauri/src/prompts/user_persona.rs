//! 用户画像生成提示词模板
//! 
//! VD-018: 创建用户画像生成提示词模板

/// 用户画像生成提示词模板
const USER_PERSONA_TEMPLATE: &str = r#"你是一位用户体验专家，擅长创建详细、真实的用户画像。

## 任务
请根据以下产品信息，生成 3-5 个目标用户画像。

## 产品信息
{product_info}

## 用户画像结构
为每个用户创建以下结构的画像：

### 基本信息
- **姓名**：（中文名）
- **年龄**：
- **职业**：
- **所在城市**：

### 个人背景
描述用户的教育背景、工作经历、技能特长等。

### 与产品相关的特征
- 使用场景
- 核心需求
- 痛点问题

### 行为特征
- 使用频率预期
- 付费意愿
- 技术熟练度
- 信息获取渠道

### 用户引言
用一句用户自己的话总结核心需求。（例如："我希望能有一个工具帮我自动..."）

## 输出要求
1. 每个画像控制在 200-300 字
2. 使用具体、生动的描述
3. 避免刻板印象
4. 确保画像之间的差异性
5. 使用 Markdown 格式输出

## 注意事项
- 画像应该代表真实的目标用户群体
- 关注与产品使用相关的特征
- 避免过度细节或无关信息"#;

/// 生成用户画像提示词
/// 
/// # Arguments
/// * `product_info` - 产品信息
/// 
/// # Returns
/// 返回完整的提示词字符串
pub fn generate_user_persona_prompt(product_info: &str) -> String {
    USER_PERSONA_TEMPLATE.replace("{product_info}", product_info)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_user_persona_prompt() {
        let product_info = "一个帮助独立开发者管理项目进度的 SaaS 工具";
        let prompt = generate_user_persona_prompt(product_info);
        
        assert!(prompt.contains("一个帮助独立开发者管理项目进度的 SaaS 工具"));
        assert!(prompt.contains("用户画像"));
        assert!(prompt.contains("基本信息"));
        assert!(prompt.contains("核心需求"));
    }

    #[test]
    fn test_template_structure() {
        // 验证模板包含所有必要的章节
        assert!(USER_PERSONA_TEMPLATE.contains("基本信息"));
        assert!(USER_PERSONA_TEMPLATE.contains("个人背景"));
        assert!(USER_PERSONA_TEMPLATE.contains("特征"));
        assert!(USER_PERSONA_TEMPLATE.contains("行为"));
        assert!(USER_PERSONA_TEMPLATE.contains("用户"));
        assert!(USER_PERSONA_TEMPLATE.contains("输出要求"));
    }
}
