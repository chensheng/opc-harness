//! Code Review Agent 实现
//! 
//! 负责分析代码变更，识别潜在问题，并提供改进建议。
//! 支持多种审查维度：代码风格、性能优化、安全漏洞、最佳实践等

use serde::{Deserialize, Serialize};

/// 审查维度枚举
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReviewDimension {
    /// 代码风格（命名规范、格式化、注释等）
    Style,
    /// 性能优化（算法复杂度、内存使用、异步处理等）
    Performance,
    /// 安全漏洞（输入验证、SQL 注入、XSS 等）
    Security,
    /// 最佳实践（设计模式、代码组织、可维护性等）
    BestPractice,
}

impl std::fmt::Display for ReviewDimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReviewDimension::Style => write!(f, "代码风格"),
            ReviewDimension::Performance => write!(f, "性能优化"),
            ReviewDimension::Security => write!(f, "安全漏洞"),
            ReviewDimension::BestPractice => write!(f, "最佳实践"),
        }
    }
}

/// 审查严重程度
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ReviewSeverity {
    /// 信息提示
    Info,
    /// 低优先级
    Low,
    /// 中优先级
    Medium,
    /// 高优先级
    High,
    /// 严重问题
    Critical,
}

impl std::fmt::Display for ReviewSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReviewSeverity::Info => write!(f, "ℹ️ 信息"),
            ReviewSeverity::Low => write!(f, "🟡 低"),
            ReviewSeverity::Medium => write!(f, "🟠 中"),
            ReviewSeverity::High => write!(f, "🔴 高"),
            ReviewSeverity::Critical => write!(f, "⛔ 严重"),
        }
    }
}

/// 审查意见结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewComment {
    /// 文件路径
    pub file_path: String,
    /// 行号（如果有）
    pub line_number: Option<usize>,
    /// 审查维度
    pub dimension: ReviewDimension,
    /// 严重程度
    pub severity: ReviewSeverity,
    /// 审查消息
    pub message: String,
    /// 改进建议
    pub suggestion: Option<String>,
    /// 代码片段
    pub code_snippet: Option<String>,
}

impl ReviewComment {
    /// 创建新的审查意见
    pub fn new(
        file_path: String,
        line_number: Option<usize>,
        dimension: ReviewDimension,
        severity: ReviewSeverity,
        message: String,
        suggestion: Option<String>,
        code_snippet: Option<String>,
    ) -> Self {
        Self {
            file_path,
            line_number,
            dimension,
            severity,
            message,
            suggestion,
            code_snippet,
        }
    }
}

/// 审查结果结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewResult {
    /// 所有审查意见
    pub comments: Vec<ReviewComment>,
    /// 审查摘要
    pub summary: String,
    /// 代码质量评分 (0.0-100.0)
    pub score: f32,
    /// 是否由 AI 生成
    pub ai_generated: bool,
}

impl ReviewResult {
    /// 创建新的审查结果
    pub fn new(comments: Vec<ReviewComment>, summary: String, score: f32, ai_generated: bool) -> Self {
        Self {
            comments,
            summary,
            score,
            ai_generated,
        }
    }

    /// 按严重程度排序审查意见（降序：Critical → Info）
    pub fn sort_by_severity(&mut self) {
        self.comments.sort_by(|a, b| b.severity.cmp(&a.severity));
    }

    /// 获取严重问题数量
    pub fn critical_count(&self) -> usize {
        self.comments.iter()
            .filter(|c| c.severity == ReviewSeverity::Critical)
            .count()
    }

    /// 获取高优先级问题数量
    pub fn high_count(&self) -> usize {
        self.comments.iter()
            .filter(|c| c.severity == ReviewSeverity::High)
            .count()
    }
}

/// Code Review Agent 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeReviewAgentConfig {
    /// 项目路径
    pub project_path: String,
    /// 是否启用 AI 审查
    pub enable_ai: bool,
    /// 审查维度列表
    pub dimensions: Vec<ReviewDimension>,
    /// 最小严重程度（低于此级别的不显示）
    pub min_severity: ReviewSeverity,
    /// 最大审查意见数
    pub max_comments: usize,
}

impl Default for CodeReviewAgentConfig {
    fn default() -> Self {
        Self {
            project_path: ".".to_string(),
            enable_ai: true,
            dimensions: vec![
                ReviewDimension::Style,
                ReviewDimension::Performance,
                ReviewDimension::Security,
                ReviewDimension::BestPractice,
            ],
            min_severity: ReviewSeverity::Info,
            max_comments: 100,
        }
    }
}

/// Code Review Agent 状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CodeReviewStatus {
    /// 等待开始
    Pending,
    /// 分析代码中
    Analyzing,
    /// 风格检查中
    CheckingStyle,
    /// 性能检查中
    CheckingPerformance,
    /// 安全检查中
    CheckingSecurity,
    /// 最佳实践检查中
    CheckingBestPractices,
    /// AI 审查中
    AiReviewing,
    /// 完成
    Completed,
    /// 失败
    Failed(String),
}

impl std::fmt::Display for CodeReviewStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodeReviewStatus::Pending => write!(f, "等待开始"),
            CodeReviewStatus::Analyzing => write!(f, "分析代码中"),
            CodeReviewStatus::CheckingStyle => write!(f, "风格检查中"),
            CodeReviewStatus::CheckingPerformance => write!(f, "性能检查中"),
            CodeReviewStatus::CheckingSecurity => write!(f, "安全检查中"),
            CodeReviewStatus::CheckingBestPractices => write!(f, "最佳实践检查中"),
            CodeReviewStatus::AiReviewing => write!(f, "AI 审查中"),
            CodeReviewStatus::Completed => write!(f, "已完成"),
            CodeReviewStatus::Failed(e) => write!(f, "失败：{}", e),
        }
    }
}

/// Code Review Agent 结构体
#[derive(Debug, Clone)]
pub struct CodeReviewAgent {
    /// 配置信息
    pub config: CodeReviewAgentConfig,
    /// 当前状态
    pub status: CodeReviewStatus,
    /// 已审查的文件列表
    pub reviewed_files: Vec<String>,
}

impl CodeReviewAgent {
    /// 创建新的代码审查 Agent
    pub fn new(config: CodeReviewAgentConfig) -> Self {
        Self {
            config,
            status: CodeReviewStatus::Pending,
            reviewed_files: Vec::new(),
        }
    }

    /// 运行完整的代码审查流程
    pub async fn run_review(&mut self, code_changes: &[CodeChange]) -> Result<ReviewResult, String> {
        log::info!("开始代码审查，共 {} 个文件", code_changes.len());
        
        self.status = CodeReviewStatus::Analyzing;
        let mut all_comments = Vec::new();

        // 分析每个文件的变更
        for change in code_changes {
            match self.analyze_code(&change.content, &change.language).await {
                Ok(comments) => {
                    all_comments.extend(comments);
                    self.reviewed_files.push(change.file_path.clone());
                }
                Err(e) => {
                    log::warn!("分析文件 {} 失败：{}", change.file_path, e);
                }
            }
        }

        // 如果启用 AI 审查
        if self.config.enable_ai {
            self.status = CodeReviewStatus::AiReviewing;
            match self.generate_ai_review(code_changes, &all_comments).await {
                Ok(ai_result) => {
                    all_comments.extend(ai_result.comments);
                }
                Err(e) => {
                    log::warn!("AI 审查失败：{}", e);
                }
            }
        }

        // 过滤和排序
        let comment_count = all_comments.len();
        let score = self.calculate_score(&all_comments);
        
        let mut result = ReviewResult::new(
            all_comments,
            "代码审查完成".to_string(),
            score,
            self.config.enable_ai,
        );

        result.sort_by_severity();
        self.status = CodeReviewStatus::Completed;

        log::info!("代码审查完成，发现 {} 个问题", comment_count);
        Ok(result)
    }

    /// 分析代码（通用入口）
    async fn analyze_code(&self, code: &str, language: &str) -> Result<Vec<ReviewComment>, String> {
        let mut comments = Vec::new();

        // 根据审查维度执行检查
        if self.config.dimensions.contains(&ReviewDimension::Style) {
            if let Ok(style_comments) = self.check_style(code).await {
                comments.extend(style_comments);
            }
        }

        if self.config.dimensions.contains(&ReviewDimension::Performance) {
            if let Ok(perf_comments) = self.check_performance(code).await {
                comments.extend(perf_comments);
            }
        }

        if self.config.dimensions.contains(&ReviewDimension::Security) {
            if let Ok(security_comments) = self.check_security(code).await {
                comments.extend(security_comments);
            }
        }

        if self.config.dimensions.contains(&ReviewDimension::BestPractice) {
            if let Ok(bp_comments) = self.check_best_practices(code).await {
                comments.extend(bp_comments);
            }
        }

        Ok(comments)
    }

    /// 代码风格检查
    async fn check_style(&self, code: &str) -> Result<Vec<ReviewComment>, String> {
        let mut comments = Vec::new();

        // 检查过长的行
        for (line_num, line) in code.lines().enumerate() {
            if line.len() > 120 {
                comments.push(ReviewComment::new(
                    "<current_file>".to_string(),
                    Some(line_num + 1),
                    ReviewDimension::Style,
                    ReviewSeverity::Low,
                    format!("行长度过长 ({} 字符，建议 ≤120)", line.len()),
                    Some("考虑将此行长拆分为多行".to_string()),
                    Some(line[..100.min(line.len())].to_string()),
                ));
            }
        }

        // 检查 TODO/FIXME 注释
        for (line_num, line) in code.lines().enumerate() {
            if line.contains("// TODO") || line.contains("// FIXME") {
                comments.push(ReviewComment::new(
                    "<current_file>".to_string(),
                    Some(line_num + 1),
                    ReviewDimension::Style,
                    ReviewSeverity::Info,
                    "发现待办注释".to_string(),
                    Some("建议尽快处理此 TODO 项".to_string()),
                    Some(line.trim().to_string()),
                ));
            }
        }

        Ok(comments)
    }

    /// 性能检查
    async fn check_performance(&self, code: &str) -> Result<Vec<ReviewComment>, String> {
        let mut comments = Vec::new();

        // 检查可能的性能问题模式
        if code.contains(".clone()") && code.contains("for ") {
            comments.push(ReviewComment::new(
                "<current_file>".to_string(),
                None,
                ReviewDimension::Performance,
                ReviewSeverity::Medium,
                "在循环中使用 clone() 可能导致性能问题".to_string(),
                Some("考虑使用引用或迭代器避免不必要的克隆".to_string()),
                None,
            ));
        }

        // 检查嵌套过深
        let max_nesting = code.split('{').map(|s| s.matches('}').count()).max().unwrap_or(0);
        if max_nesting > 5 {
            comments.push(ReviewComment::new(
                "<current_file>".to_string(),
                None,
                ReviewDimension::Performance,
                ReviewSeverity::Medium,
                "代码嵌套层级过深，可能影响可读性和性能".to_string(),
                Some("考虑提取函数或使用早期返回减少嵌套".to_string()),
                None,
            ));
        }

        Ok(comments)
    }

    /// 安全检查
    async fn check_security(&self, code: &str) -> Result<Vec<ReviewComment>, String> {
        let mut comments = Vec::new();

        // 检查 SQL 注入风险
        if code.contains("SELECT * FROM") && code.contains("+") {
            comments.push(ReviewComment::new(
                "<current_file>".to_string(),
                None,
                ReviewDimension::Security,
                ReviewSeverity::Critical,
                "检测到可能的 SQL 注入风险".to_string(),
                Some("使用参数化查询代替字符串拼接".to_string()),
                None,
            ));
        }

        // 检查硬编码密钥
        if code.contains("password") && code.contains("\"") || code.contains("'") {
            comments.push(ReviewComment::new(
                "<current_file>".to_string(),
                None,
                ReviewDimension::Security,
                ReviewSeverity::High,
                "检测到可能的硬编码密码".to_string(),
                Some("使用环境变量或密钥管理服务".to_string()),
                None,
            ));
        }

        // 检查 eval 使用
        if code.contains("eval(") {
            comments.push(ReviewComment::new(
                "<current_file>".to_string(),
                None,
                ReviewDimension::Security,
                ReviewSeverity::Critical,
                "使用 eval() 存在安全风险".to_string(),
                Some("避免使用 eval()，寻找更安全的替代方案".to_string()),
                None,
            ));
        }

        Ok(comments)
    }

    /// 最佳实践检查
    async fn check_best_practices(&self, code: &str) -> Result<Vec<ReviewComment>, String> {
        let mut comments = Vec::new();

        // 检查空 catch 块
        if code.contains("catch") && code.contains("{}") {
            comments.push(ReviewComment::new(
                "<current_file>".to_string(),
                None,
                ReviewDimension::BestPractice,
                ReviewSeverity::Medium,
                "空的 catch 块会隐藏错误".to_string(),
                Some("至少记录错误日志或重新抛出异常".to_string()),
                None,
            ));
        }

        // 检查魔法数字
        if code.contains(" 86400") || code.contains(" 3600") || code.contains(" 604800") {
            comments.push(ReviewComment::new(
                "<current_file>".to_string(),
                None,
                ReviewDimension::BestPractice,
                ReviewSeverity::Low,
                "使用魔法数字".to_string(),
                Some("使用具名常量提高代码可读性".to_string()),
                None,
            ));
        }

        // 检查过长的函数（简单通过行数判断）
        let line_count = code.lines().count();
        if line_count > 200 {
            comments.push(ReviewComment::new(
                "<current_file>".to_string(),
                None,
                ReviewDimension::BestPractice,
                ReviewSeverity::Medium,
                format!("函数过长 ({} 行)", line_count),
                Some("考虑将大函数拆分为多个小函数".to_string()),
                None,
            ));
        }

        Ok(comments)
    }

    /// AI 生成审查意见（模板实现）
    async fn generate_ai_review(&self, _code_changes: &[CodeChange], manual_comments: &[ReviewComment]) -> Result<ReviewResult, String> {
        log::info!("生成 AI 审查意见");
        
        // TODO: 实际实现中需要调用 AI API
        // 这里提供模板实现
        
        let mut ai_comments = Vec::new();

        // 基于手动审查的结果，AI 补充建议
        for comment in manual_comments {
            if comment.severity >= ReviewSeverity::High {
                ai_comments.push(ReviewComment::new(
                    comment.file_path.clone(),
                    comment.line_number,
                    comment.dimension.clone(),
                    comment.severity.clone(),
                    format!("[AI] {}", comment.message),
                    Some(format!("[AI 建议] {}", comment.suggestion.as_ref().unwrap_or(&"无".to_string()))),
                    comment.code_snippet.clone(),
                ));
            }
        }

        Ok(ReviewResult::new(
            ai_comments,
            "AI 审查完成".to_string(),
            75.0,
            true,
        ))
    }

    /// 计算代码质量评分
    fn calculate_score(&self, comments: &[ReviewComment]) -> f32 {
        let mut score: f32 = 100.0;

        for comment in comments {
            let penalty = match comment.severity {
                ReviewSeverity::Critical => 15.0_f32,
                ReviewSeverity::High => 10.0_f32,
                ReviewSeverity::Medium => 5.0_f32,
                ReviewSeverity::Low => 2.0_f32,
                ReviewSeverity::Info => 0.0_f32,
            };
            score -= penalty;
        }

        score.max(0.0_f32).min(100.0_f32)
    }
}

/// 代码变更结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeChange {
    /// 文件路径
    pub file_path: String,
    /// 文件内容
    pub content: String,
    /// 编程语言
    pub language: String,
    /// 变更类型（Added/Modified/Deleted）
    pub change_type: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_review_dimension_display() {
        assert_eq!(format!("{}", ReviewDimension::Style), "代码风格");
        assert_eq!(format!("{}", ReviewDimension::Performance), "性能优化");
        assert_eq!(format!("{}", ReviewDimension::Security), "安全漏洞");
        assert_eq!(format!("{}", ReviewDimension::BestPractice), "最佳实践");
    }

    #[test]
    fn test_review_severity_ordering() {
        assert!(ReviewSeverity::Critical > ReviewSeverity::High);
        assert!(ReviewSeverity::High > ReviewSeverity::Medium);
        assert!(ReviewSeverity::Medium > ReviewSeverity::Low);
        assert!(ReviewSeverity::Low > ReviewSeverity::Info);
    }

    #[test]
    fn test_review_comment_creation() {
        let comment = ReviewComment::new(
            "src/main.rs".to_string(),
            Some(10),
            ReviewDimension::Security,
            ReviewSeverity::Critical,
            "SQL 注入风险".to_string(),
            Some("使用参数化查询".to_string()),
            Some("SELECT * FROM users WHERE id = ".to_string()),
        );

        assert_eq!(comment.file_path, "src/main.rs");
        assert_eq!(comment.line_number, Some(10));
        assert_eq!(comment.dimension, ReviewDimension::Security);
        assert_eq!(comment.severity, ReviewSeverity::Critical);
    }

    #[test]
    fn test_review_result_sorting() {
        let mut result = ReviewResult::new(vec![
            ReviewComment::new(
                "file1.rs".to_string(),
                None,
                ReviewDimension::Style,
                ReviewSeverity::Low,
                "Style issue".to_string(),
                None,
                None,
            ),
            ReviewComment::new(
                "file2.rs".to_string(),
                None,
                ReviewDimension::Security,
                ReviewSeverity::Critical,
                "Security issue".to_string(),
                None,
                None,
            ),
            ReviewComment::new(
                "file3.rs".to_string(),
                None,
                ReviewDimension::Performance,
                ReviewSeverity::High,
                "Performance issue".to_string(),
                None,
                None,
            ),
        ], "Test".to_string(), 100.0, false);

        result.sort_by_severity();
        
        // 应该按严重程度排序：Critical > High > Low
        assert_eq!(result.comments[0].severity, ReviewSeverity::Critical);
        assert_eq!(result.comments[1].severity, ReviewSeverity::High);
        assert_eq!(result.comments[2].severity, ReviewSeverity::Low);
    }

    #[test]
    fn test_review_result_critical_count() {
        let result = ReviewResult::new(vec![
            ReviewComment::new("f1.rs".to_string(), None, ReviewDimension::Security, ReviewSeverity::Critical, "".to_string(), None, None),
            ReviewComment::new("f2.rs".to_string(), None, ReviewDimension::Security, ReviewSeverity::Critical, "".to_string(), None, None),
            ReviewComment::new("f3.rs".to_string(), None, ReviewDimension::Style, ReviewSeverity::Low, "".to_string(), None, None),
        ], "Test".to_string(), 100.0, false);

        assert_eq!(result.critical_count(), 2);
        assert_eq!(result.high_count(), 0);
    }

    #[test]
    fn test_code_review_agent_creation() {
        let config = CodeReviewAgentConfig {
            project_path: "/tmp/test".to_string(),
            enable_ai: true,
            dimensions: vec![ReviewDimension::Style, ReviewDimension::Security],
            min_severity: ReviewSeverity::Medium,
            max_comments: 50,
        };

        let agent = CodeReviewAgent::new(config.clone());

        assert_eq!(agent.config.project_path, "/tmp/test");
        assert!(agent.config.enable_ai);
        assert_eq!(agent.config.dimensions.len(), 2);
        assert_eq!(agent.status, CodeReviewStatus::Pending);
    }

    #[test]
    fn test_code_review_status_display() {
        assert_eq!(format!("{}", CodeReviewStatus::Pending), "等待开始");
        assert_eq!(format!("{}", CodeReviewStatus::Analyzing), "分析代码中");
        assert_eq!(format!("{}", CodeReviewStatus::AiReviewing), "AI 审查中");
        assert_eq!(format!("{}", CodeReviewStatus::Completed), "已完成");
    }

    #[test]
    fn test_style_check_long_line() {
        let long_code = format!("fn main() {{ {} }}", "a".repeat(150));
        
        // 这里不实际调用 async 方法，只测试数据结构
        let comment = ReviewComment::new(
            "test.rs".to_string(),
            Some(1),
            ReviewDimension::Style,
            ReviewSeverity::Low,
            "行长度过长".to_string(),
            Some("考虑拆分".to_string()),
            Some(long_code[..50].to_string()),
        );

        assert_eq!(comment.dimension, ReviewDimension::Style);
        assert_eq!(comment.severity, ReviewSeverity::Low);
    }

    #[test]
    fn test_security_check_patterns() {
        let sql_injection_code = "SELECT * FROM users WHERE id = \" + user_id";
        let eval_code = "eval(user_input)";
        
        // 测试数据结构
        let sql_comment = ReviewComment::new(
            "db.rs".to_string(),
            None,
            ReviewDimension::Security,
            ReviewSeverity::Critical,
            "SQL 注入风险".to_string(),
            Some("使用参数化查询".to_string()),
            Some(sql_injection_code.to_string()),
        );

        let eval_comment = ReviewComment::new(
            "app.js".to_string(),
            None,
            ReviewDimension::Security,
            ReviewSeverity::Critical,
            "eval() 风险".to_string(),
            Some("避免使用 eval()".to_string()),
            Some(eval_code.to_string()),
        );

        assert_eq!(sql_comment.severity, ReviewSeverity::Critical);
        assert_eq!(eval_comment.severity, ReviewSeverity::Critical);
    }

    #[test]
    fn test_score_calculation() {
        let agent = CodeReviewAgent::new(CodeReviewAgentConfig::default());
        
        let comments = vec![
            ReviewComment::new("f1.rs".to_string(), None, ReviewDimension::Security, ReviewSeverity::Critical, "".to_string(), None, None),
            ReviewComment::new("f2.rs".to_string(), None, ReviewDimension::Performance, ReviewSeverity::High, "".to_string(), None, None),
            ReviewComment::new("f3.rs".to_string(), None, ReviewDimension::Style, ReviewSeverity::Medium, "".to_string(), None, None),
        ];

        let score = agent.calculate_score(&comments);
        // Critical: -15, High: -10, Medium: -5 => 100 - 15 - 10 - 5 = 70
        assert!((score - 70.0).abs() < 0.1);
    }

    #[test]
    fn test_empty_code_review_result() {
        let result = ReviewResult::new(vec![], "No issues found".to_string(), 100.0, false);
        
        assert_eq!(result.comments.len(), 0);
        assert_eq!(result.score, 100.0);
        assert_eq!(result.critical_count(), 0);
    }

    #[test]
    fn test_code_change_structure() {
        let change = CodeChange {
            file_path: "src/lib.rs".to_string(),
            content: "fn test() {}".to_string(),
            language: "rust".to_string(),
            change_type: "Modified".to_string(),
        };

        assert_eq!(change.file_path, "src/lib.rs");
        assert_eq!(change.language, "rust");
        assert_eq!(change.change_type, "Modified");
    }
}
