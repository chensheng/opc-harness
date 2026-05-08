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

// =====================================================================
// MiniMax 优化版本
// =====================================================================

/// MiniMax 优化的用户画像生成模板
///
/// MiniMax (名之梦) 在中文理解和创意写作方面表现优异，
/// 此模板针对 MiniMax 的特性进行了优化：
/// - 强调情感化和故事性描述
/// - 生动具体的细节描写
/// - 适合中国用户语境
const MINIMAX_USER_PERSONA_TEMPLATE: &str = r#"你是一位富有创意的用户体验设计师，擅长用生动的笔触描绘真实鲜活的用户形象。

## 任务
请为以下产品创建 3-5 个有血有肉、令人印象深刻的用户画像。

## 产品信息
{product_info}

## 创作要求

### 🎨 画像结构
为每个用户创建一个完整的故事板：

#### 1. 人物卡片
- **姓名**：（富有时代特色的中文名）
- **年龄**：
- **职业标签**：（用 2-3 个词概括，如"996 程序员/技术控/效率达人"）
- **坐标**：（城市 + 区域特色，如"北京中关村"、"深圳南山"）

#### 2. 人物小传 📖
用讲故事的口吻描述这个人的背景：
- 成长经历和教育背景
- 职业发展轨迹
- 当前的工作状态和生活状态
- 性格特点（用具体事例说明）

#### 3. 与产品的相遇 💡
描述这个人在什么场景下遇到了什么问题，从而需要这个产品：
- 遇到的具体困难或痛点
- 之前的解决方案及其不足
- 第一次接触产品的场景
- 使用产品后的改变

#### 4. 日常使用场景 🌟
生动描绘一个典型的使用场景：
- 时间、地点、情境
- 如何使用产品
- 解决了什么问题
- 带来的情绪价值

#### 5. 用户心声 💬
用这个人自己的话说出一句代表性的话：
- 对产品的真实感受
- 核心诉求或期望
- 推荐给朋友的理由

### ✨ 创作要点

1. **情感共鸣**: 让读者能够感同身受
2. **细节丰富**: 用具体的细节让人物鲜活起来
3. **故事性强**: 像写小说一样塑造人物
4. **真实可信**: 避免过度夸张或刻板印象
5. **差异化**: 确保每个人物都有独特的个性和背景

### 📝 格式要求
- 使用 Markdown 格式
- 适当使用 emoji 增强可读性
- 每个人物 400-600 字
- 语言生动有趣，避免枯燥的列表式描述

现在，请用你的创造力，让这些用户形象跃然纸上！"#;

/// 生成 MiniMax 优化的用户画像提示词
///
/// # Arguments
/// * `product_info` - 产品信息
///
/// # Returns
/// 返回 MiniMax 优化的完整提示词字符串
pub fn generate_user_persona_prompt_minimax(product_info: &str) -> String {
    MINIMAX_USER_PERSONA_TEMPLATE.replace("{product_info}", product_info)
}

// =====================================================================
// GLM 优化版本
// =====================================================================

/// GLM 优化的用户画像生成模板
///
/// GLM (智谱 AI) 在技术理解和逻辑分析方面表现优异，
/// 此模板针对 GLM 的特性进行了优化：
/// - 强调数据驱动和用户研究
/// - 注重逻辑性和系统性
/// - 适合技术导向型产品
const GLM_USER_PERSONA_TEMPLATE: &str = r#"你是一位资深用户研究员，擅长基于数据和逻辑分析创建科学严谨的用户画像。

## 任务
请基于以下产品信息，运用专业的用户研究方法，创建 3-5 个结构化的用户画像。

## 产品信息
{product_info}

## 研究方法

### 数据分析维度
1. **人口统计学特征**
   - 年龄段、性别比例
   - 教育程度、收入水平
   - 地域分布、城市等级

2. **心理学特征**
   - 价值观和态度
   - 动机和需求层次
   - 风险偏好和决策风格

3. **行为学特征**
   - 使用习惯和频率
   - 付费意愿和能力
   - 信息获取渠道
   - 社交影响力

## 画像结构

### 用户档案 #N

#### A. 基础画像 (Profile)
| 维度 | 描述 |
|------|------|
| 姓名 | （代表性中文名） |
| 年龄段 | X-X 岁 |
| 职业 | 具体职位 |
| 城市 | 一线/新一线/二线等 |
| 收入 | 月收入范围 |
| 教育 | 学历背景 |

#### B. 需求分析 (Needs Analysis)
1. **核心需求** (Must-have)
   - 需求描述
   - 重要程度 (1-5 分)
   - 当前满足度

2. **期望需求** (Nice-to-have)
   - 需求描述
   - 期待程度
   - 优先级

3. **潜在需求** (Latent)
   - 可能存在的隐性需求
   - 挖掘依据

#### C. 使用场景 (Usage Scenarios)
- **主要场景**: 高频使用的核心场景
- **次要场景**: 辅助性功能场景
- **边缘场景**: 特殊情况下的使用

#### D. 决策路径 (Decision Journey)
```
认知 → 了解 → 评估 → 试用 → 购买 → 推荐
```
描述用户在每个阶段的行为和考虑因素。

#### E. 量化指标 (Metrics)
- 预期使用频率：X 次/周
- 付费意愿：高/中/低
- 推荐可能性：NPS 评分预估
- 生命周期价值：LTV 预估

#### F. 用户原话 (Voice of Customer)
"..." - 用一句话精准表达核心诉求

## 输出标准
1. 数据支撑：每个判断都应有合理依据
2. 逻辑清晰：结构化呈现，便于分析
3. 可验证性：画像特征可被用户调研验证
4. 可操作性：能直接指导产品设计和营销

请开始你的专业分析。"#;

/// 生成 GLM 优化的用户画像提示词
///
/// # Arguments
/// * `product_info` - 产品信息
///
/// # Returns
/// 返回 GLM 优化的完整提示词字符串
pub fn generate_user_persona_prompt_glm(product_info: &str) -> String {
    GLM_USER_PERSONA_TEMPLATE.replace("{product_info}", product_info)
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

#[cfg(test)]
mod minimax_tests {
    use super::*;

    #[test]
    fn test_minimax_persona_prompt_generation() {
        let product_info = "一个在线学习平台";
        let prompt = generate_user_persona_prompt_minimax(product_info);

        assert!(prompt.contains("创作要求"));
        assert!(prompt.contains("人物小传"));
        assert!(prompt.contains("用户心声"));
        assert!(prompt.contains("📖"));
    }

    #[test]
    fn test_minimax_persona_style() {
        let product_info = "一个效率工具";
        let prompt = generate_user_persona_prompt_minimax(product_info);

        assert!(prompt.contains("生动"));
        assert!(prompt.contains("故事"));
        assert!(prompt.contains("情感"));
    }
}

#[cfg(test)]
mod glm_tests {
    use super::*;

    #[test]
    fn test_glm_persona_prompt_generation() {
        let product_info = "一个开发者工具";
        let prompt = generate_user_persona_prompt_glm(product_info);

        assert!(prompt.contains("数据分析"));
        assert!(prompt.contains("需求分析"));
        assert!(prompt.contains("量化指标"));
        assert!(prompt.contains("Profile"));
    }

    #[test]
    fn test_glm_persona_structure() {
        let product_info = "一个 SaaS 产品";
        let prompt = generate_user_persona_prompt_glm(product_info);

        assert!(prompt.contains("基础画像"));
        assert!(prompt.contains("决策路径"));
        assert!(prompt.contains("可验证性"));
    }
}
