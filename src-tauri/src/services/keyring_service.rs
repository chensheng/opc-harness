//! OS 密钥存储服务
//!
//! 使用 keyring-rs 库在各平台安全地存储敏感信息（如 API 密钥）

use keyring::Entry;
use thiserror::Error;

/// 密钥服务错误类型
#[derive(Debug, Error)]
pub enum KeyringError {
    #[error("Keyring error: {0}")]
    Keyring(#[from] keyring::Error),
    
    #[error("Invalid service name: {0}")]
    InvalidService(String),
    
    #[error("Invalid account name: {0}")]
    InvalidAccount(String),
}

/// 密钥服务
pub struct KeyringService {
    service_name: String,
}

impl KeyringService {
    /// 创建新的密钥服务实例
    pub fn new(app_name: &str) -> Self {
        Self {
            service_name: app_name.to_string(),
        }
    }

    /// 存储密码/密钥
    ///
    /// # 参数
    /// * `account` - 账户名（如 provider 名称）
    /// * `password` - 要存储的密码/密钥
    ///
    /// # 返回
    /// * `Result<(), KeyringError>` - 成功或错误
    pub fn set_password(&self, account: &str, password: &str) -> Result<(), KeyringError> {
        let entry = Entry::new(&self.service_name, account)?;
        entry.set_password(password)?;
        log::info!("Password stored for account: {}", account);
        Ok(())
    }

    /// 获取密码/密钥
    ///
    /// # 参数
    /// * `account` - 账户名
    ///
    /// # 返回
    /// * `Result<Option<String>, KeyringError>` - 密码（如果不存在则返回 None）或错误
    pub fn get_password(&self, account: &str) -> Result<Option<String>, KeyringError> {
        let entry = Entry::new(&self.service_name, account)?;
        match entry.get_password() {
            Ok(password) => {
                log::debug!("Password retrieved for account: {}", account);
                Ok(Some(password))
            }
            Err(keyring::Error::NoEntry) => {
                log::debug!("No password found for account: {}", account);
                Ok(None)
            }
            Err(e) => Err(KeyringError::Keyring(e)),
        }
    }

    /// 删除密码/密钥
    ///
    /// # 参数
    /// * `account` - 账户名
    ///
    /// # 返回
    /// * `Result<(), KeyringError>` - 成功或错误
    pub fn delete_password(&self, account: &str) -> Result<(), KeyringError> {
        let entry = Entry::new(&self.service_name, account)?;
        entry.delete_password()?;
        log::info!("Password deleted for account: {}", account);
        Ok(())
    }

    /// 检查密码是否存在
    ///
    /// # 参数
    /// * `account` - 账户名
    ///
    /// # 返回
    /// * `Result<bool, KeyringError>` - 是否存在或错误
    pub fn has_password(&self, account: &str) -> Result<bool, KeyringError> {
        match self.get_password(account) {
            Ok(Some(_)) => Ok(true),
            Ok(None) => Ok(false),
            Err(e) => Err(e),
        }
    }
}

/// AI Provider 密钥管理
impl KeyringService {
    /// 存储 AI Provider 的 API 密钥
    pub fn set_ai_api_key(&self, provider: &str, api_key: &str) -> Result<(), KeyringError> {
        let account = format!("ai:{}", provider);
        self.set_password(&account, api_key)
    }

    /// 获取 AI Provider 的 API 密钥
    pub fn get_ai_api_key(&self, provider: &str) -> Result<Option<String>, KeyringError> {
        let account = format!("ai:{}", provider);
        self.get_password(&account)
    }

    /// 删除 AI Provider 的 API 密钥
    pub fn delete_ai_api_key(&self, provider: &str) -> Result<(), KeyringError> {
        let account = format!("ai:{}", provider);
        self.delete_password(&account)
    }

    /// 获取所有存储的 AI Provider 密钥
    ///
    /// 注意：keyring-rs 不提供列出所有条目的功能，
    /// 这里返回的是常用 provider 的密钥状态
    pub fn get_all_ai_api_keys(&self) -> Result<Vec<(String, bool)>, KeyringError> {
        let providers = vec!["openai", "anthropic", "kimi", "glm"];
        let mut result = Vec::new();
        
        for provider in providers {
            let has_key = self.has_ai_api_key(provider)?;
            result.push((provider.to_string(), has_key));
        }
        
        Ok(result)
    }

    /// 检查 AI Provider 是否有存储的 API 密钥
    pub fn has_ai_api_key(&self, provider: &str) -> Result<bool, KeyringError> {
        let account = format!("ai:{}", provider);
        self.has_password(&account)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyring_basic() {
        let service = KeyringService::new("opc-harness-test");
        let test_account = "test_account";
        let test_password = "test_password_123";

        // 存储密码
        service.set_password(test_account, test_password).unwrap();

        // 读取密码
        let retrieved = service.get_password(test_account).unwrap();
        assert_eq!(retrieved, Some(test_password.to_string()));

        // 检查存在性
        assert!(service.has_password(test_account).unwrap());

        // 删除密码
        service.delete_password(test_account).unwrap();

        // 确认已删除
        assert!(!service.has_password(test_account).unwrap());
    }

    #[test]
    fn test_ai_api_key_management() {
        let service = KeyringService::new("opc-harness-test-ai");
        let provider = "test-provider";
        let api_key = "sk-test123456789";

        // 存储 API 密钥
        service.set_ai_api_key(provider, api_key).unwrap();

        // 读取 API 密钥
        let retrieved = service.get_ai_api_key(provider).unwrap();
        assert_eq!(retrieved, Some(api_key.to_string()));

        // 检查存在性
        assert!(service.has_ai_api_key(provider).unwrap());

        // 删除 API 密钥
        service.delete_ai_api_key(provider).unwrap();

        // 确认已删除
        assert!(!service.has_ai_api_key(provider).unwrap());
    }
}
