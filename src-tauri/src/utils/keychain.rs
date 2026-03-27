use keyring::Entry;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KeychainError {
    #[error("Failed to access keychain: {0}")]
    AccessError(String),
    #[error("API key not found for provider: {0}")]
    NotFound(String),
    #[error("Invalid API key format")]
    InvalidFormat,
}

/// Service name for all OPC-HARNESS keychain entries
const SERVICE_NAME: &str = "opc-harness";

/// Save API key securely to OS keychain
///
/// # Arguments
/// * `provider` - AI provider name (e.g., "openai", "anthropic", "kimi", "glm")
/// * `api_key` - The API key to store
///
/// # Returns
/// * `Ok(())` if successful
/// * `Err(KeychainError)` if failed
pub fn save_api_key(provider: &str, api_key: &str) -> Result<(), KeychainError> {
    // Validate provider name
    if provider.is_empty() {
        return Err(KeychainError::InvalidFormat);
    }

    // Validate API key is not empty
    if api_key.is_empty() {
        return Err(KeychainError::InvalidFormat);
    }

    // Create keychain entry using provider as user identifier
    let entry = Entry::new(SERVICE_NAME, provider)
        .map_err(|e| KeychainError::AccessError(e.to_string()))?;

    // Store the API key
    entry
        .set_password(api_key)
        .map_err(|e| KeychainError::AccessError(e.to_string()))?;

    Ok(())
}

/// Retrieve API key from OS keychain
///
/// # Arguments
/// * `provider` - AI provider name
///
/// # Returns
/// * `Ok(String)` with the API key if found
/// * `Err(KeychainError::NotFound)` if key doesn't exist
/// * `Err(KeychainError)` if other error occurs
pub fn get_api_key(provider: &str) -> Result<String, KeychainError> {
    // Create keychain entry
    let entry = Entry::new(SERVICE_NAME, provider)
        .map_err(|e| KeychainError::AccessError(e.to_string()))?;

    // Retrieve the password
    let password = entry.get_password().map_err(|e| match e {
        keyring::Error::NoEntry => KeychainError::NotFound(provider.to_string()),
        _ => KeychainError::AccessError(e.to_string()),
    })?;

    Ok(password)
}

/// Delete API key from OS keychain
///
/// # Arguments
/// * `provider` - AI provider name
///
/// # Returns
/// * `Ok(())` if successful or key didn't exist
/// * `Err(KeychainError)` if failed to delete
pub fn delete_api_key(provider: &str) -> Result<(), KeychainError> {
    // Create keychain entry
    let entry = Entry::new(SERVICE_NAME, provider)
        .map_err(|e| KeychainError::AccessError(e.to_string()))?;

    // Delete the entry (ignore if it doesn't exist)
    match entry.delete_credential() {
        Ok(_) => Ok(()),
        Err(keyring::Error::NoEntry) => Ok(()), // Not an error if it doesn't exist
        Err(e) => Err(KeychainError::AccessError(e.to_string())),
    }
}

/// Check if API key exists in keychain
///
/// # Arguments
/// * `provider` - AI provider name
///
/// # Returns
/// * `true` if key exists
/// * `false` otherwise
pub fn has_api_key(provider: &str) -> bool {
    get_api_key(provider).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_and_retrieve_api_key() {
        let provider = "test-provider";
        let api_key = "test-key-12345";

        // Save the key
        save_api_key(provider, api_key).expect("Failed to save API key");

        // Retrieve the key
        let retrieved = get_api_key(provider).expect("Failed to retrieve API key");
        assert_eq!(retrieved, api_key);

        // Clean up
        delete_api_key(provider).expect("Failed to delete API key");
    }

    #[test]
    fn test_delete_nonexistent_key() {
        let provider = "nonexistent-provider";
        // Should not error even if key doesn't exist
        delete_api_key(provider).expect("Delete should succeed even for nonexistent keys");
    }

    #[test]
    fn test_has_api_key() {
        let provider = format!("test-has-key-{}", std::process::id());
        let api_key = "test-key";

        // Initially should not have key
        assert!(!has_api_key(&provider));

        // Save and check again
        save_api_key(&provider, api_key).expect("Failed to save");
        assert!(has_api_key(&provider));

        // Delete and verify
        delete_api_key(&provider).expect("Failed to delete");
        assert!(!has_api_key(&provider));
    }

    #[test]
    fn test_empty_provider_fails() {
        let result = save_api_key("", "some-key");
        assert!(result.is_err());
        assert!(matches!(result, Err(KeychainError::InvalidFormat)));
    }

    #[test]
    fn test_empty_api_key_fails() {
        let result = save_api_key("openai", "");
        assert!(result.is_err());
        assert!(matches!(result, Err(KeychainError::InvalidFormat)));
    }
}
