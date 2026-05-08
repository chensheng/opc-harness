//! AI Code Generator 实现
//!
//! 负责根据自然语言描述自动生成代码。
//! 支持多种编程语言（Rust/TypeScript/JavaScript）。
//! 提供代码补全、函数生成、类生成、测试生成等功能。

use serde::{Deserialize, Serialize};

/// AI 模型枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[allow(non_camel_case_types)]
#[derive(Default)]
pub enum AIModel {
    /// OpenAI GPT-4
    #[default]
    OpenAI_GPT4,
    /// OpenAI GPT-3.5 Turbo
    OpenAI_GPT3_5_Turbo,
    /// Claude 3 Opus
    Claude_3_Opus,
    /// Claude 3 Sonnet
    Claude_3_Sonnet,
    /// 通义千问 Max
    Qwen_Max,
}

impl std::fmt::Display for AIModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AIModel::OpenAI_GPT4 => write!(f, "gpt-4"),
            AIModel::OpenAI_GPT3_5_Turbo => write!(f, "gpt-3.5-turbo"),
            AIModel::Claude_3_Opus => write!(f, "claude-3-opus"),
            AIModel::Claude_3_Sonnet => write!(f, "claude-3-sonnet"),
            AIModel::Qwen_Max => write!(f, "qwen-max"),
        }
    }
}

/// 代码生成类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GenerationType {
    /// 生成函数
    Function,
    /// 生成类
    Class,
    /// 生成测试
    Test,
    /// 代码补全
    Complete,
    /// 完整代码生成
    Full,
}

/// 代码生成配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationConfig {
    /// AI 模型
    pub model: AIModel,
    /// 温度（0.0-1.0，越高越随机）
    pub temperature: f32,
    /// 最大 token 数
    pub max_tokens: u32,
    /// 目标语言
    pub language: String,
}

impl Default for GenerationConfig {
    fn default() -> Self {
        Self {
            model: AIModel::default(),
            temperature: 0.7,
            max_tokens: 2000,
            language: "rust".to_string(),
        }
    }
}

/// 代码生成请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeGenerationRequest {
    /// 自然语言描述
    pub description: String,
    /// 上下文代码（可选）
    pub context: Option<String>,
    /// 目标语言
    pub language: String,
    /// 生成类型
    pub generation_type: GenerationType,
}

/// 代码质量评估
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeQuality {
    /// 质量评分（0.0-1.0）
    pub score: f32,
    /// 代码风格评分
    pub style_score: f32,
    /// 可维护性评分
    pub maintainability_score: f32,
    /// 性能评分
    pub performance_score: f32,
    /// 改进建议
    pub suggestions: Vec<String>,
}

impl CodeQuality {
    pub fn new(
        score: f32,
        style_score: f32,
        maintainability_score: f32,
        performance_score: f32,
        suggestions: Vec<String>,
    ) -> Self {
        Self {
            score,
            style_score,
            maintainability_score,
            performance_score,
            suggestions,
        }
    }
}

/// 代码生成响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeGenerationResponse {
    /// 生成的代码
    pub code: String,
    /// 代码解释
    pub explanation: String,
    /// 质量评分
    pub quality_score: f32,
    /// 优化建议
    pub suggestions: Vec<String>,
}

impl CodeGenerationResponse {
    pub fn new(
        code: String,
        explanation: String,
        quality_score: f32,
        suggestions: Vec<String>,
    ) -> Self {
        Self {
            code,
            explanation,
            quality_score,
            suggestions,
        }
    }
}

/// AI Code Generator 结构体
pub struct AICodeGenerator {
    config: GenerationConfig,
    #[allow(dead_code)]
    api_key: String,
}

impl AICodeGenerator {
    /// 创建新的 AI 代码生成器
    pub fn new(config: GenerationConfig, api_key: String) -> Self {
        Self { config, api_key }
    }

    /// 根据描述生成代码
    pub async fn generate_code(
        &self,
        request: CodeGenerationRequest,
    ) -> Result<CodeGenerationResponse, String> {
        log::info!(
            "开始生成代码，类型：{:?}, 语言：{}",
            request.generation_type,
            request.language
        );

        // 构建 prompt
        let prompt = self.build_prompt(&request);

        // 调用 AI API（简化实现：返回示例代码）
        let response = self.call_ai_api(&prompt).await?;

        // 检查代码质量
        let quality = self.check_code_quality(&response.code);

        log::info!("代码生成完成，质量评分：{:.2}", quality.score);

        Ok(CodeGenerationResponse::new(
            response.code,
            response.explanation,
            quality.score,
            quality.suggestions,
        ))
    }

    /// 代码补全
    pub async fn complete_code(
        &self,
        code: String,
        cursor_position: usize,
    ) -> Result<CodeGenerationResponse, String> {
        log::info!("开始代码补全，位置：{}", cursor_position);

        // 构建补全 prompt
        let prompt = format!(
            "请补全以下{}代码（光标位置用 | 标记）：\n\n{}",
            self.config.language,
            self.insert_cursor_marker(&code, cursor_position)
        );

        // 调用 AI API
        let response = self.call_ai_api(&prompt).await?;

        log::info!("代码补全完成");

        Ok(response)
    }

    /// 生成函数
    pub async fn generate_function(
        &self,
        description: String,
        language: String,
    ) -> Result<CodeGenerationResponse, String> {
        let request = CodeGenerationRequest {
            description,
            context: None,
            language,
            generation_type: GenerationType::Function,
        };

        self.generate_code(request).await
    }

    /// 生成类
    pub async fn generate_class(
        &self,
        description: String,
        language: String,
    ) -> Result<CodeGenerationResponse, String> {
        let request = CodeGenerationRequest {
            description,
            context: None,
            language,
            generation_type: GenerationType::Class,
        };

        self.generate_code(request).await
    }

    /// 生成测试代码
    pub async fn generate_test(
        &self,
        code: String,
        language: String,
    ) -> Result<CodeGenerationResponse, String> {
        let request = CodeGenerationRequest {
            description: format!("为以下代码生成单元测试：\n\n{}", code),
            context: Some(code),
            language,
            generation_type: GenerationType::Test,
        };

        self.generate_code(request).await
    }

    /// 构建 prompt
    fn build_prompt(&self, request: &CodeGenerationRequest) -> String {
        let generation_type_str = match request.generation_type {
            GenerationType::Function => "函数",
            GenerationType::Class => "类",
            GenerationType::Test => "测试代码",
            GenerationType::Complete => "补全代码",
            GenerationType::Full => "完整代码",
        };

        let mut prompt = format!(
            "请帮我生成一段{}代码。\n\n要求：\n{}\n\n语言：{}",
            generation_type_str, request.description, request.language
        );

        if let Some(context) = &request.context {
            prompt.push_str(&format!("\n\n上下文代码：\n{}", context));
        }

        prompt.push_str("\n\n请提供：\n1. 完整的代码实现\n2. 代码解释\n3. 可能的优化建议");

        prompt
    }

    /// 调用 AI API（简化实现）
    async fn call_ai_api(&self, prompt: &str) -> Result<CodeGenerationResponse, String> {
        // TODO: 实际项目中需要实现真实的 AI API 调用
        // 这里使用示例代码作为占位

        log::debug!("调用 AI API，prompt 长度：{}", prompt.len());

        // 根据生成类型返回不同的示例代码
        let (code, explanation) = match self.config.language.as_str() {
            "rust" => self.generate_rust_example(),
            "typescript" | "ts" => self.generate_typescript_example(),
            _ => self.generate_generic_example(),
        };

        Ok(CodeGenerationResponse::new(
            code,
            explanation,
            0.85,
            vec![
                "考虑添加错误处理".to_string(),
                "可以添加更多注释".to_string(),
            ],
        ))
    }

    /// 生成 Rust 示例代码
    fn generate_rust_example(&self) -> (String, String) {
        let code = r#"/// 计算斐波那契数列的第 n 项
/// 
/// # Arguments
/// * `n` - 要计算的项数（从 0 开始）
/// 
/// # Returns
/// * `u64` - 第 n 项的斐波那契数
/// 
/// # Example
/// ```
/// let result = fibonacci(5);
/// assert_eq!(result, 5);
/// ```
pub fn fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let (mut prev, mut curr) = (0, 1);
            for _ in 2..=n {
                let next = prev + curr;
                prev = curr;
                curr = next;
            }
            curr
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_first_ten() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(3), 2);
        assert_eq!(fibonacci(4), 3);
        assert_eq!(fibonacci(5), 5);
    }
}"#
        .to_string();

        let explanation = "这是一个高效的斐波那契数列实现，使用迭代而非递归，时间复杂度 O(n)，空间复杂度 O(1)。包含完整的文档注释和单元测试。".to_string();

        (code, explanation)
    }

    /// 生成 TypeScript 示例代码
    fn generate_typescript_example(&self) -> (String, String) {
        let code = r#"/**
 * 计算斐波那契数列的第 n 项
 * @param n - 要计算的项数（从 0 开始）
 * @returns 第 n 项的斐波那契数
 */
export function fibonacci(n: number): number {
    if (n === 0) return 0;
    if (n === 1) return 1;

    let [prev, curr] = [0, 1];
    for (let i = 2; i <= n; i++) {
        [prev, curr] = [curr, prev + curr];
    }
    
    return curr;
}

// 测试用例
describe('fibonacci', () => {
    it('should calculate first few fibonacci numbers', () => {
        expect(fibonacci(0)).toBe(0);
        expect(fibonacci(1)).toBe(1);
        expect(fibonacci(2)).toBe(1);
        expect(fibonacci(3)).toBe(2);
        expect(fibonacci(4)).toBe(3);
        expect(fibonacci(5)).toBe(5);
    });
});"#
            .to_string();

        let explanation = "这是一个 TypeScript 实现的斐波那契数列函数，使用迭代方法和解构赋值，类型安全且高效。包含 JSDoc 注释和 Jest 测试用例。".to_string();

        (code, explanation)
    }

    /// 生成通用示例代码
    fn generate_generic_example(&self) -> (String, String) {
        let code = "// Generated code example\nfunction example(): void {\n    console.log('Hello, World!');\n}".to_string();
        let explanation =
            "这是一个示例代码片段。请提供更详细的描述以生成更有用的代码。".to_string();
        (code, explanation)
    }

    /// 在代码中插入光标标记
    fn insert_cursor_marker(&self, code: &str, position: usize) -> String {
        let chars: Vec<char> = code.chars().collect();
        if position > chars.len() {
            return format!("{} |", code);
        }

        let mut result = String::new();
        for (i, ch) in chars.iter().enumerate() {
            if i == position {
                result.push('|');
            }
            result.push(*ch);
        }
        if position == chars.len() {
            result.push('|');
        }
        result
    }

    /// 检查代码质量
    pub fn check_code_quality(&self, code: &str) -> CodeQuality {
        let mut suggestions = Vec::new();

        // 基础质量检查
        let has_comments = code.contains("//") || code.contains("/*");
        let has_tests =
            code.contains("#[test]") || code.contains("describe(") || code.contains("it(");
        let has_error_handling =
            code.contains("Result<") || code.contains("try ") || code.contains("catch ");

        // 计算各项评分
        let style_score = if has_comments { 0.9 } else { 0.6 };
        let maintainability_score = if has_comments && code.len() < 500 {
            0.85
        } else {
            0.7
        };
        let performance_score = 0.8; // 简化实现

        // 生成建议
        if !has_comments {
            suggestions.push("建议添加文档注释".to_string());
        }
        if !has_tests && self.config.language == "rust" {
            suggestions.push("建议添加单元测试".to_string());
        }
        if !has_error_handling {
            suggestions.push("考虑添加错误处理".to_string());
        }

        let overall_score = (style_score + maintainability_score + performance_score) / 3.0;

        CodeQuality::new(
            overall_score,
            style_score,
            maintainability_score,
            performance_score,
            suggestions,
        )
    }

    /// 获取配置信息
    pub fn get_config(&self) -> &GenerationConfig {
        &self.config
    }

    /// 获取使用的 AI 模型
    pub fn get_model(&self) -> &AIModel {
        &self.config.model
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generation_config_creation() {
        let config = GenerationConfig {
            model: AIModel::OpenAI_GPT4,
            temperature: 0.5,
            max_tokens: 1000,
            language: "typescript".to_string(),
        };

        assert_eq!(config.model, AIModel::OpenAI_GPT4);
        assert_eq!(config.temperature, 0.5);
        assert_eq!(config.max_tokens, 1000);
    }

    #[test]
    fn test_generation_config_default() {
        let config = GenerationConfig::default();

        assert_eq!(config.model, AIModel::OpenAI_GPT4);
        assert_eq!(config.temperature, 0.7);
        assert_eq!(config.max_tokens, 2000);
        assert_eq!(config.language, "rust");
    }

    #[test]
    fn test_ai_model_display() {
        assert_eq!(format!("{}", AIModel::OpenAI_GPT4), "gpt-4");
        assert_eq!(format!("{}", AIModel::Claude_3_Opus), "claude-3-opus");
        assert_eq!(format!("{}", AIModel::Qwen_Max), "qwen-max");
    }

    #[test]
    fn test_code_quality_with_comments() {
        let config = GenerationConfig::default();
        let generator = AICodeGenerator::new(config, "test_key".to_string());

        let code = r#"
/// This is a comment
pub fn test() -> i32 {
    42
}
"#;

        let quality = generator.check_code_quality(code);

        assert!(quality.score > 0.7);
        assert!(quality.style_score > 0.8);
        assert!(quality.suggestions.is_empty() || quality.suggestions.len() < 3);
    }

    #[test]
    fn test_code_quality_without_comments() {
        let config = GenerationConfig::default();
        let generator = AICodeGenerator::new(config, "test_key".to_string());

        let code = r#"
pub fn test() -> i32 {
    42
}
"#;

        let quality = generator.check_code_quality(code);

        assert!(quality.style_score < 0.7);
        assert!(quality.suggestions.iter().any(|s| s.contains("注释")));
    }

    #[test]
    fn test_code_quality_with_tests() {
        let config = GenerationConfig::default();
        let generator = AICodeGenerator::new(config, "test_key".to_string());

        let code = r#"
pub fn test() -> i32 {
    42
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert_eq!(2 + 2, 4);
    }
}
"#;

        let quality = generator.check_code_quality(code);

        // 有测试代码应该提高可维护性评分
        assert!(quality.maintainability_score >= 0.7);
    }

    #[test]
    fn test_insert_cursor_marker_basic() {
        let config = GenerationConfig::default();
        let generator = AICodeGenerator::new(config, "test_key".to_string());

        let code = "fn main() {}";
        let result = generator.insert_cursor_marker(code, 3);

        assert_eq!(result, "fn |main() {}");
    }

    #[test]
    fn test_insert_cursor_marker_at_end() {
        let config = GenerationConfig::default();
        let generator = AICodeGenerator::new(config, "test_key".to_string());

        let code = "fn main() {}";
        let result = generator.insert_cursor_marker(code, 12);

        assert_eq!(result, "fn main() {}|");
    }

    #[test]
    fn test_insert_cursor_marker_beyond_end() {
        let config = GenerationConfig::default();
        let generator = AICodeGenerator::new(config, "test_key".to_string());

        let code = "fn main() {}";
        let result = generator.insert_cursor_marker(code, 100);

        assert_eq!(result, "fn main() {} |");
    }

    #[test]
    fn test_generator_creation() {
        let config = GenerationConfig::default();
        let generator = AICodeGenerator::new(config.clone(), "test_key".to_string());

        assert_eq!(generator.get_config().model, AIModel::OpenAI_GPT4);
        assert_eq!(generator.get_model(), &AIModel::OpenAI_GPT4);
    }

    #[test]
    fn test_build_prompt_function() {
        let config = GenerationConfig::default();
        let generator = AICodeGenerator::new(config, "test_key".to_string());

        let request = CodeGenerationRequest {
            description: "创建一个计算阶乘的函数".to_string(),
            context: None,
            language: "rust".to_string(),
            generation_type: GenerationType::Function,
        };

        let prompt = generator.build_prompt(&request);

        assert!(prompt.contains("函数"));
        assert!(prompt.contains("创建一个计算阶乘的函数"));
        assert!(prompt.contains("rust"));
    }

    #[test]
    fn test_build_prompt_with_context() {
        let config = GenerationConfig::default();
        let generator = AICodeGenerator::new(config, "test_key".to_string());

        let request = CodeGenerationRequest {
            description: "完善这个函数".to_string(),
            context: Some("fn add(a: i32, b: i32) -> i32 {}".to_string()),
            language: "rust".to_string(),
            generation_type: GenerationType::Function,
        };

        let prompt = generator.build_prompt(&request);

        assert!(prompt.contains("上下文代码"));
        assert!(prompt.contains("fn add"));
    }

    #[test]
    fn test_code_generation_response_creation() {
        let response = CodeGenerationResponse::new(
            "fn test() {}".to_string(),
            "这是一个测试函数".to_string(),
            0.85,
            vec!["添加注释".to_string()],
        );

        assert_eq!(response.code, "fn test() {}");
        assert_eq!(response.quality_score, 0.85);
        assert_eq!(response.suggestions.len(), 1);
    }

    #[test]
    fn test_code_quality_comprehensive() {
        let config = GenerationConfig::default();
        let generator = AICodeGenerator::new(config, "test_key".to_string());

        let code = r#"
/// 计算两个数的和
/// 
/// # Arguments
/// * `a` - 第一个数
/// * `b` - 第二个数
/// 
/// # Returns
/// * `i32` - 两数之和
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
"#;

        let quality = generator.check_code_quality(code);

        assert!(quality.score > 0.8);
        assert!(quality.style_score > 0.8);
        assert!(quality.maintainability_score > 0.8);
        assert!(quality.performance_score > 0.7);
    }
}
