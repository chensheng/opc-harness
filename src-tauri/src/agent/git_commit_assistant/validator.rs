//! Git Commit Assistant 验证器
//! 
//! 负责验证提交信息是否符合 Conventional Commits 规范

/// 提交信息验证器
#[derive(Clone, Debug)]
pub struct CommitValidator;

impl CommitValidator {
    /// 创建新的验证器
    pub fn new() -> Self {
        Self
    }

    /// 验证是否符合 Conventional Commits 规范
    pub fn validate_conventional_commit(&self, message: &str) -> Result<bool, String> {
        // Conventional Commits 格式：type(scope): description
        // type 必须是 feat, fix, docs, style, refactor, perf, test, chore 之一
        
        let pattern = regex::Regex::new(r"^(feat|fix|docs|style|refactor|perf|test|chore)(\([a-z0-9-]+\))?: .{1,72}$")
            .map_err(|e| format!("Invalid regex: {}", e))?;

        if pattern.is_match(message) {
            Ok(true)
        } else {
            Err("Message does not follow Conventional Commits format".to_string())
        }
    }
}
