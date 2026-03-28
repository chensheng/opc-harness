//! 竞品分析提示词模板
//! 
//! VD-003: 创建竞品分析生成提示词模板

/// 竞品分析基础模板
const COMPETITOR_ANALYSIS_TEMPLATE: &str = r#"你是一位资深市场分析师，擅长进行系统性的竞品分析。

## 任务
请根据以下产品信息，识别并分析主要竞争对手。

## 产品信息
{product_info}

## 分析要求

### 1. 竞品识别
找出 3-5 个主要竞争对手，包括：
- 直接竞争对手（功能相似）
- 间接竞争对手（解决同类问题但方式不同）
- 潜在竞争对手（可能进入该领域的公司）

### 2. 竞品分析结构
为每个竞品提供以下信息：

#### 基本信息
- **公司名称**：
- **产品名称**：
- **官网链接**：
- **成立时间**：
- **融资阶段**：

#### 产品优势 (Strengths)
列出 3-5 个核心优势：
- 功能特点
- 技术壁垒
- 品牌影响力
- 用户基础
- 资金实力

#### 产品劣势 (Weaknesses)
列出 3-5 个主要劣势：
- 功能缺陷
- 用户体验问题
- 定价策略问题
- 服务不足
- 技术局限

#### 市场份额
- 预估市场占有率（百分比或范围）
- 用户规模估算
- 增长趋势（上升/稳定/下降）

#### 差异化定位
- 目标用户群体
- 核心价值主张
- 主要营销策略

### 3. 竞争格局分析
- 市场集中度（寡头垄断/充分竞争/分散市场）
- 进入壁垒（高/中/低）
- 替代品威胁程度

### 4. 我们的机会
基于以上分析，提出 3-5 个差异化机会：
- 未被满足的用户需求
- 竞品的薄弱环节
- 技术创新空间
- 市场空白点

## 输出要求
1. 使用 Markdown 格式
2. 结构化呈现，便于阅读
3. 数据驱动，避免主观臆断
4. 每个竞品分析控制在 300-500 字
5. 提供具体的行动建议

## 注意事项
- 保持客观公正的态度
- 区分事实和推测
- 关注可验证的数据
- 避免过度贬低竞品"#;

/// 生成竞品分析提示词
pub fn generate_competitor_analysis_prompt(product_info: &str) -> String {
    COMPETITOR_ANALYSIS_TEMPLATE.replace("{product_info}", product_info)
}

// =====================================================================
// MiniMax 优化版本
// =====================================================================

/// MiniMax 优化的竞品分析模板
/// 
/// MiniMax 在中文理解和创意写作方面表现优异，
/// 此模板针对 MiniMax 的特性进行了优化：
/// - 强调故事性和场景化描述
/// - 注重情感化和用户体验视角
/// - 适合中国市场竞争环境
const MINIMAX_COMPETITOR_ANALYSIS_TEMPLATE: &str = r#"你是一位富有洞察力的产品经理，擅长用生动的笔触描绘市场竞争格局。

## 任务
请为以下产品绘制一幅鲜活的"竞争地图"，帮助我们理解市场格局和找到突破口。

## 产品信息
{product_info}

## 分析框架

### 🗺️ 竞争地图总览
先用一个比喻或故事来描述整个市场的竞争态势：
- 这个市场像什么？（如"群雄逐鹿的三国时代"、"一超多强的格局"等）
- 主要的"玩家"都有谁？
- 竞争的焦点在哪里？

### 👥 竞品画像（3-5 个）
为每个竞品画一幅"人物肖像"：

#### 【竞品名称】- 用一句话概括其特点
**人设标签**：用 2-3 个词概括（如"行业老大哥"、"创新挑战者"、"性价比之王"）

**发家史**：简述其发展历程和关键转折点

**独门绝技**（优势）：
- 最核心的竞争力是什么？
- 用户为什么选择它？
- 有什么难以复制的优势？

**阿喀琉斯之踵**（劣势）：
- 最大的短板是什么？
- 用户抱怨最多的是什么？
- 有哪些明显的改进空间？

**江湖地位**：
- 市场份额估算（用形象的说法，如"占据半壁江山"、"稳坐第二把交椅"等）
- 用户口碑如何？
- 行业影响力怎样？

### ⚔️ 竞争策略分析
分析各家的"打法"：
- 产品策略：功能导向还是体验导向？
- 定价策略：高端路线还是性价比？
- 营销策略：高举高打还是润物无声？
- 用户策略：服务大众还是聚焦细分？

### 💡 破局机会
基于以上分析，找到我们的"弯道超车"机会：

1. **人无我有**：竞品都没做但用户很需要的功能/服务
2. **人有我优**：竞品做得一般但我们可以做到极致的点
3. **差异化定位**：避开正面竞争，开辟新战场
4. **模式创新**：用全新的方式满足用户需求

### 🎯 行动建议
给出 3-5 条具体可行的建议：
- 短期（1-3 个月）可以做什么？
- 中期（3-6 个月）应该布局什么？
- 长期（6-12 个月）需要投入什么？

## 输出风格
- 语言生动有趣，避免枯燥的列表
- 多用比喻和类比，让抽象的概念具象化
- 适当使用 emoji 增强可读性
- 像讲故事一样呈现分析结果
- 每个人物画像要鲜活有个性

## 篇幅要求
整体控制在 2000-3000 字

现在，请用你的洞察力，为我们绘制这幅竞争地图！"#;

/// 生成 MiniMax 优化的竞品分析提示词
pub fn generate_competitor_analysis_prompt_minimax(product_info: &str) -> String {
    MINIMAX_COMPETITOR_ANALYSIS_TEMPLATE.replace("{product_info}", product_info)
}

// =====================================================================
// GLM 优化版本
// =====================================================================

/// GLM 优化的竞品分析模板
/// 
/// GLM 在逻辑分析和数据驱动方面表现优异，
/// 此模板针对 GLM 的特性进行了优化：
/// - 强调数据支撑和量化分析
/// - 注重逻辑框架和系统性
/// - 适合技术驱动型产品分析
const GLM_COMPETITOR_ANALYSIS_TEMPLATE: &str = r#"你是一位专业的市场研究顾问，擅长基于数据和框架进行严谨的竞争分析。

## 任务
请运用专业的分析框架，对以下产品所在市场进行系统性竞争分析。

## 产品信息
{product_info}

## 分析框架

### 一、市场概况 (Market Overview)

#### 1.1 市场规模
- TAM (Total Addressable Market): 总体可服务市场规模
- SAM (Serviceable Available Market): 可获取的市场规模
- SOM (Serviceable Obtainable Market): 实际可获得市场份额

#### 1.2 市场生命周期
- 导入期 / 成长期 / 成熟期 / 衰退期
- 判断依据和数据支撑

#### 1.3 市场集中度
- CR4/CR8 指标（前 4/8 名市场份额总和）
- 赫芬达尔指数 (HHI) 评估

### 二、竞争格局分析 (Competitive Landscape)

#### 2.1 竞争者识别与分类
使用波特五力模型分析：
- 现有竞争者的竞争强度
- 潜在进入者的威胁
- 替代品的威胁
- 供应商的议价能力
- 购买者的议价能力

#### 2.2 主要竞争者画像 (3-5 家)

| 维度 | 竞争者 A | 竞争者 B | 竞争者 C |
|------|----------|----------|----------|
| 公司名称 | | | |
| 成立时间 | | | |
| 融资阶段 | | | |
| 员工规模 | | | |
| 估计年收入 | | | |
| 市场份额 | | | |
| 用户规模 | | | |
| 增长率 | | | |

#### 2.3 竞争优势对比 (SWOT 分析)

**竞争者 A**:
- Strengths (优势): [列举 3-5 项，按重要性排序]
- Weaknesses (劣势): [列举 3-5 项，按重要性排序]
- Opportunities (机会): [市场机会点]
- Threats (威胁): [面临的外部威胁]

*(对 B、C 同样分析)*

### 三、核心竞争力评估

#### 3.1 成功关键因素 (KSF - Key Success Factors)
识别本行业的 3-5 个 KSF，并评估各竞争者的表现：

| KSF | 权重 | 竞争者 A | 竞争者 B | 竞争者 C | 我们 |
|-----|------|----------|----------|----------|------|
| 技术创新能力 | 30% | 评分 1-5 | 评分 1-5 | 评分 1-5 | 预期评分 |
| 品牌影响力 | 25% | ... | ... | ... | ... |
| 渠道覆盖 | 20% | ... | ... | ... | ... |
| 成本控制 | 15% | ... | ... | ... | ... |
| 用户体验 | 10% | ... | ... | ... | ... |

#### 3.2 战略群组分析
- 按价格/质量定位分组
- 按产品线宽度分组
- 按地理覆盖分组

### 四、市场机会识别

#### 4.1 未满足需求分析
- 用户痛点调研数据引用
- 竞品满意度最低的环节
- 新兴需求趋势

#### 4.2 市场空白点
- 地域空白
- 用户群体空白
- 功能/服务空白
- 价格带空白

#### 4.3 差异化机会矩阵

| 机会类型 | 市场吸引力 | 实施可行性 | 优先级 |
|----------|------------|------------|--------|
| 机会 1 描述 | 高/中/低 | 高/中/低 | P0/P1/P2 |
| 机会 2 描述 | ... | ... | ... |

### 五、战略建议

#### 5.1 定位策略
- 目标细分市场选择
- 价值主张设计
- 差异化定位陈述

#### 5.2 竞争策略
- 成本领先 / 差异化 / 集中化
- 市场领导者/挑战者/跟随者/补缺者策略选择

#### 5.3 行动路线图
| 阶段 | 时间 | 关键行动 | 预期成果 | 资源投入 |
|------|------|----------|----------|----------|
| 第一阶段 | Q1 | ... | ... | ... |
| 第二阶段 | Q2 | ... | ... | ... |
| 第三阶段 | Q3 | ... | ... | ... |

## 输出标准
1. **数据驱动**: 每个判断都应有数据或事实支撑
2. **框架清晰**: 运用成熟的商业分析框架
3. **逻辑严谨**: 论证过程严密，避免跳跃
4. **可操作性**: 建议具体可行，有明确的时间表和责任人
5. **量化指标**: 尽可能给出数值化的目标和评估

请开始你的专业分析。"#;

/// 生成 GLM 优化的竞品分析提示词
pub fn generate_competitor_analysis_prompt_glm(product_info: &str) -> String {
    GLM_COMPETITOR_ANALYSIS_TEMPLATE.replace("{product_info}", product_info)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_competitor_analysis_prompt() {
        let product_info = "一个帮助独立开发者管理项目进度的 AI 工具";
        let prompt = generate_competitor_analysis_prompt(product_info);
        
        assert!(prompt.contains("竞品识别"));
        assert!(prompt.contains("Strengths"));
        assert!(prompt.contains("市场份额"));
        assert!(prompt.contains("差异化"));
    }

    #[test]
    fn test_template_structure() {
        assert!(COMPETITOR_ANALYSIS_TEMPLATE.contains("竞品识别"));
        assert!(COMPETITOR_ANALYSIS_TEMPLATE.contains("优势"));
        assert!(COMPETITOR_ANALYSIS_TEMPLATE.contains("劣势"));
        assert!(COMPETITOR_ANALYSIS_TEMPLATE.contains("市场份额"));
    }
}

#[cfg(test)]
mod minimax_tests {
    use super::*;

    #[test]
    fn test_minimax_competitor_analysis_prompt() {
        let product_info = "一个在线学习平台";
        let prompt = generate_competitor_analysis_prompt_minimax(product_info);
        
        assert!(prompt.contains("竞争地图"));
        assert!(prompt.contains("竞品画像"));
        assert!(prompt.contains("破局机会"));
        assert!(prompt.contains("emoji"));
    }

    #[test]
    fn test_minimax_style_features() {
        let product_info = "一个效率工具";
        let prompt = generate_competitor_analysis_prompt_minimax(product_info);
        
        assert!(prompt.contains("比喻"));
        assert!(prompt.contains("故事"));
        assert!(prompt.contains("人设"));
        assert!(prompt.contains("独门绝技"));
    }
}

#[cfg(test)]
mod glm_tests {
    use super::*;

    #[test]
    fn test_glm_competitor_analysis_prompt() {
        let product_info = "一个 SaaS 产品";
        let prompt = generate_competitor_analysis_prompt_glm(product_info);
        
        assert!(prompt.contains("TAM"));
        assert!(prompt.contains("SWOT"));
        assert!(prompt.contains("| 维度 |"));
        assert!(prompt.contains("数据驱动"));
    }

    #[test]
    fn test_glm_framework_completeness() {
        let product_info = "一个电商平台";
        let prompt = generate_competitor_analysis_prompt_glm(product_info);
        
        assert!(prompt.contains("市场概况"));
        assert!(prompt.contains("竞争格局"));
        assert!(prompt.contains("核心竞争力"));
        assert!(prompt.contains("战略建议"));
    }
}
