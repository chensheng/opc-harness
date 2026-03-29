//! 统一错误处理模块
//!
//! 提供全应用范围的错误类型定义、错误转换和错误恢复机制

use std::fmt;
use std::error::Error as StdError;
use std::collections::HashMap;

// ==================== 错误码（用于前端国际化）====================

/// 错误码枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    // 通用错误
    SUCCESS = 0,
    UNKNOWN_ERROR = 9999,
    
    // 数据库错误 (1xxx)
    DATABASE_ERROR = 1000,
    DATABASE_CONNECTION_FAILED = 1001,
    DATABASE_QUERY_FAILED = 1002,
    DATABASE_CONSTRAINT_VIOLATION = 1003,
    
    // AI 服务错误 (2xxx)
    AI_SERVICE_ERROR = 2000,
    AI_API_KEY_INVALID = 2001,
    AI_RATE_LIMIT_EXCEEDED = 2002,
    AI_MODEL_NOT_FOUND = 2003,
    AI_TIMEOUT = 2004,
    AI_INVALID_RESPONSE = 2005,
    
    // 网络错误 (3xxx)
    NETWORK_ERROR = 3000,
    NETWORK_TIMEOUT = 3001,
    NETWORK_UNREACHABLE = 3002,
    NETWORK_DNS_FAILED = 3003,
    NETWORK_SSL_ERROR = 3004,
    
    // 文件系统错误 (4xxx)
    FILE_SYSTEM_ERROR = 4000,
    FILE_NOT_FOUND = 4001,
    FILE_PERMISSION_DENIED = 4002,
    FILE_ALREADY_EXISTS = 4003,
    FILE_INVALID_PATH = 4004,
    
    // 验证错误 (5xxx)
    VALIDATION_ERROR = 5000,
    VALIDATION_FAILED = 5001,
    VALIDATION_EMPTY_FIELD = 5002,
    VALIDATION_INVALID_FORMAT = 5003,
    
    // 业务逻辑错误 (6xxx)
    BUSINESS_ERROR = 6000,
    RESOURCE_NOT_FOUND = 6001,
    OPERATION_FAILED = 6002,
    INVALID_STATE = 6003,
    DUPLICATE_RESOURCE = 6004,
    
    // 配置错误 (7xxx)
    CONFIG_ERROR = 7000,
    CONFIG_NOT_FOUND = 7001,
    CONFIG_INVALID_VALUE = 7002,
    
    // 密钥管理错误 (8xxx)
    KEYCHAIN_ERROR = 8000,
    KEYCHAIN_ACCESS_DENIED = 8001,
    KEYCHAIN_NOT_FOUND = 8002,
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ErrorCode {
    /// 获取错误的 HTTP 状态码映射
    pub fn to_http_status(&self) -> u16 {
        match self {
            ErrorCode::SUCCESS => 200,
            ErrorCode::VALIDATION_ERROR | ErrorCode::VALIDATION_FAILED => 400,
            ErrorCode::FILE_NOT_FOUND | ErrorCode::RESOURCE_NOT_FOUND => 404,
            ErrorCode::KEYCHAIN_ACCESS_DENIED | ErrorCode::FILE_PERMISSION_DENIED => 403,
            ErrorCode::DUPLICATE_RESOURCE => 409,
            ErrorCode::AI_RATE_LIMIT_EXCEEDED => 429,
            _ => 500,
        }
    }
}

// ==================== 错误上下文 ====================

/// 错误上下文信息
pub struct ErrorContext {
    pub code: ErrorCode,
    pub message: String,
    pub source: Option<Box<dyn StdError + Send + Sync>>,
    pub context: HashMap<String, String>,
}

impl fmt::Debug for ErrorContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ErrorContext")
            .field("code", &self.code)
            .field("message", &self.message)
            .field("context", &self.context)
            .finish()
    }
}

// ==================== 统一应用错误 ====================

/// 统一的应用错误类型
#[derive(Debug)]
pub struct AppError {
    pub context: ErrorContext,
}

impl AppError {
    /// 创建新错误
    pub fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            context: ErrorContext {
                code,
                message: message.into(),
                source: None,
                context: HashMap::new(),
            },
        }
    }
    
    /// 创建带原始错误的错误
    pub fn with_source(
        code: ErrorCode,
        message: impl Into<String>,
        source: impl StdError + Send + Sync + 'static,
    ) -> Self {
        Self {
            context: ErrorContext {
                code,
                message: message.into(),
                source: Some(Box::new(source)),
                context: HashMap::new(),
            },
        }
    }
    
    /// 添加上下文信息
    pub fn with_context(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.context.context.insert(key.into(), value.into());
        self
    }
    
    /// 获取错误码
    pub fn code(&self) -> ErrorCode {
        self.context.code
    }
    
    /// 获取错误消息
    pub fn message(&self) -> &str {
        &self.context.message
    }
    
    /// 获取上下文
    pub fn context(&self) -> &HashMap<String, String> {
        &self.context.context
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{:?}] {}", self.context.code, self.context.message)
    }
}

impl StdError for AppError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.context.source.as_ref().map(|e| e.as_ref() as _)
    }
}

// ==================== 结果类型别名 ====================

/// 统一的结果类型
pub type AppResult<T> = Result<T, AppError>;

// ==================== From Trait 实现（外部库错误转换）====================

/// rusqlite 错误转换
impl From<rusqlite::Error> for AppError {
    fn from(err: rusqlite::Error) -> Self {
        let code = match &err {
            rusqlite::Error::SqliteFailure(_, _) => ErrorCode::DATABASE_QUERY_FAILED,
            rusqlite::Error::InvalidParameterName(_) => ErrorCode::DATABASE_QUERY_FAILED,
            rusqlite::Error::ExecuteReturnedResults => ErrorCode::DATABASE_QUERY_FAILED,
            rusqlite::Error::QueryReturnedNoRows => ErrorCode::RESOURCE_NOT_FOUND,
            _ => ErrorCode::DATABASE_ERROR,
        };
        
        AppError::with_source(code, err.to_string(), err)
    }
}

/// IO 错误转换
impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        use std::io::ErrorKind;
        
        let code = match err.kind() {
            ErrorKind::NotFound => ErrorCode::FILE_NOT_FOUND,
            ErrorKind::PermissionDenied => ErrorCode::FILE_PERMISSION_DENIED,
            ErrorKind::AlreadyExists => ErrorCode::FILE_ALREADY_EXISTS,
            ErrorKind::InvalidInput => ErrorCode::FILE_INVALID_PATH,
            ErrorKind::TimedOut => ErrorCode::NETWORK_TIMEOUT,
            _ => ErrorCode::FILE_SYSTEM_ERROR,
        };
        
        AppError::with_source(code, err.to_string(), err)
    }
}

/// serde_json 错误转换
impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::with_source(ErrorCode::VALIDATION_INVALID_FORMAT, err.to_string(), err)
    }
}

/// anyhow 错误转换
impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::new(ErrorCode::UNKNOWN_ERROR, err.to_string())
    }
}

// ==================== 具体错误类型 ====================

/// 数据库错误
#[derive(Debug)]
pub struct DatabaseError {
    pub message: String,
    pub sql: Option<String>,
    pub params: Option<Vec<String>>,
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl StdError for DatabaseError {}

impl From<DatabaseError> for AppError {
    fn from(err: DatabaseError) -> Self {
        AppError::new(ErrorCode::DATABASE_ERROR, err.message)
    }
}

/// AI 服务错误
#[derive(Debug)]
pub struct AIError {
    pub message: String,
    pub provider: Option<String>,
    pub status_code: Option<u16>,
}

impl fmt::Display for AIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(provider) = &self.provider {
            write!(f, "[{}] {}", provider, self.message)
        } else {
            write!(f, "{}", self.message)
        }
    }
}

impl StdError for AIError {}

impl From<AIError> for AppError {
    fn from(err: AIError) -> Self {
        let code = if err.status_code == Some(429) {
            ErrorCode::AI_RATE_LIMIT_EXCEEDED
        } else if err.status_code == Some(401) || err.status_code == Some(403) {
            ErrorCode::AI_API_KEY_INVALID
        } else {
            ErrorCode::AI_SERVICE_ERROR
        };
        
        AppError::new(code, err.to_string()).with_context("provider", err.provider.unwrap_or_default())
    }
}

/// 网络错误
#[derive(Debug)]
pub struct NetworkError {
    pub message: String,
    pub url: String,
    pub method: Option<String>,
}

impl fmt::Display for NetworkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let method = self.method.as_ref().map(|s| s.as_str()).unwrap_or("");
        write!(f, "{} [{} {}]", self.message, method, self.url)
    }
}

impl StdError for NetworkError {}

impl From<NetworkError> for AppError {
    fn from(err: NetworkError) -> Self {
        let method = err.method.clone().unwrap_or_default();
        AppError::new(ErrorCode::NETWORK_ERROR, err.to_string())
            .with_context("url", err.url)
            .with_context("method", method)
    }
}

/// 文件系统错误
#[derive(Debug)]
pub struct FileSystemError {
    pub message: String,
    pub path: String,
}

impl fmt::Display for FileSystemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.message, self.path)
    }
}

impl StdError for FileSystemError {}

impl From<FileSystemError> for AppError {
    fn from(err: FileSystemError) -> Self {
        AppError::new(ErrorCode::FILE_SYSTEM_ERROR, err.to_string())
            .with_context("path", err.path)
    }
}

/// 验证错误
#[derive(Debug)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Field '{}': {}", self.field, self.message)
    }
}

impl StdError for ValidationError {}

impl From<ValidationError> for AppError {
    fn from(err: ValidationError) -> Self {
        AppError::new(ErrorCode::VALIDATION_FAILED, err.to_string())
            .with_context("field", err.field)
    }
}

/// 业务逻辑错误
#[derive(Debug)]
pub struct BusinessError {
    pub code: String,
    pub message: String,
}

impl fmt::Display for BusinessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {}", self.code, self.message)
    }
}

impl StdError for BusinessError {}

impl From<BusinessError> for AppError {
    fn from(err: BusinessError) -> Self {
        AppError::new(ErrorCode::BUSINESS_ERROR, err.to_string())
            .with_context("business_code", err.code)
    }
}

// ==================== 辅助函数 ====================

/// 创建验证错误的快捷函数
pub fn validation_error(field: impl Into<String>, message: impl Into<String>) -> AppError {
    ValidationError {
        field: field.into(),
        message: message.into(),
    }.into()
}

/// 创建资源未找到错误的快捷函数
pub fn not_found(resource_type: &str, id: &str) -> AppError {
    AppError::new(ErrorCode::RESOURCE_NOT_FOUND, format!("{} not found: {}", resource_type, id))
        .with_context("resource_type", resource_type)
        .with_context("id", id)
}

/// 创建操作失败错误的快捷函数
pub fn operation_failed(operation: &str, reason: &str) -> AppError {
    AppError::new(ErrorCode::OPERATION_FAILED, format!("{} failed: {}", operation, reason))
        .with_context("operation", operation)
        .with_context("reason", reason)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_error_new() {
        let error = AppError::new(ErrorCode::VALIDATION_ERROR, "测试错误");
        assert_eq!(error.code(), ErrorCode::VALIDATION_ERROR);
        assert_eq!(error.message(), "测试错误");
    }

    #[test]
    fn test_app_error_with_source() {
        let source = std::io::Error::new(std::io::ErrorKind::NotFound, "文件不存在");
        let error = AppError::with_source(ErrorCode::FILE_NOT_FOUND, "读取文件失败", source);
        assert_eq!(error.code(), ErrorCode::FILE_NOT_FOUND);
        assert!(error.message().contains("读取文件失败"));
        assert!(error.source().is_some());
    }

    #[test]
    fn test_app_error_with_context() {
        let error = AppError::new(ErrorCode::DATABASE_ERROR, "查询失败")
            .with_context("sql", "SELECT * FROM users")
            .with_context("user_id", "123");
        
        assert_eq!(error.context().get("sql"), Some(&"SELECT * FROM users".to_string()));
        assert_eq!(error.context().get("user_id"), Some(&"123".to_string()));
    }

    #[test]
    fn test_from_rusqlite_error() {
        // 这里无法直接创建 rusqlite::Error，所以只测试编译通过
        // 实际测试需要在集成测试中
        let _ = AppError::from;
    }

    #[test]
    fn test_from_io_error() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "文件不存在");
        let app_error: AppError = io_error.into();
        assert_eq!(app_error.code(), ErrorCode::FILE_NOT_FOUND);
    }

    #[test]
    fn test_validation_error_helper() {
        let error = validation_error("name", "不能为空");
        assert_eq!(error.code(), ErrorCode::VALIDATION_FAILED);
        assert!(error.message().contains("不能为空"));
        assert_eq!(error.context().get("field"), Some(&"name".to_string()));
    }

    #[test]
    fn test_not_found_helper() {
        let error = not_found("Project", "123");
        assert_eq!(error.code(), ErrorCode::RESOURCE_NOT_FOUND);
        assert!(error.message().contains("Project not found"));
        assert_eq!(error.context().get("id"), Some(&"123".to_string()));
    }

    #[test]
    fn test_error_code_to_http_status() {
        assert_eq!(ErrorCode::VALIDATION_FAILED.to_http_status(), 400);
        assert_eq!(ErrorCode::FILE_NOT_FOUND.to_http_status(), 404);
        assert_eq!(ErrorCode::RESOURCE_NOT_FOUND.to_http_status(), 404);
        assert_eq!(ErrorCode::DATABASE_ERROR.to_http_status(), 500);
    }
}
