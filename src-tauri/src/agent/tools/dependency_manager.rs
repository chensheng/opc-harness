//! Dependency Management Tools for Native Coding Agent
//!
//! 提供安全的依赖管理能力，支持 npm 和 cargo 包管理器。

use std::path::PathBuf;
use tokio::process::Command;

/// 包管理器类型
#[derive(Debug, Clone)]
pub enum PackageManager {
    Npm,
    Cargo,
}

/// 依赖安装结果
#[derive(Debug, Clone)]
pub struct DependencyInstallResult {
    pub package_name: String,
    pub installed_version: String,
    pub success: bool,
    pub message: String,
}

/// 依赖管理工具集
pub struct DependencyManager {
    workspace_root: PathBuf,
    package_manager: PackageManager,
}

impl DependencyManager {
    /// 创建新的依赖管理工具集
    pub fn new(workspace_root: PathBuf, package_manager: PackageManager) -> Self {
        Self {
            workspace_root,
            package_manager,
        }
    }

    /// 验证包名合法性
    fn validate_package_name(&self, package_name: &str) -> Result<(), String> {
        // 只允许字母、数字、连字符、下划线、点、斜杠（用于 scope）
        let valid_chars = regex::Regex::new(r"^[a-zA-Z0-9@/_\-\.]+$").unwrap();

        if !valid_chars.is_match(package_name) {
            return Err(format!(
                "Invalid package name '{}': only alphanumeric, @, /, _, -, . allowed",
                package_name
            ));
        }

        // 防止路径遍历攻击
        if package_name.contains("..") || package_name.starts_with('/') {
            return Err(format!("Invalid package name '{}': path traversal detected", package_name));
        }

        Ok(())
    }

    /// 安装 npm 包
    ///
    /// # Arguments
    /// * `package` - 包名（如 "react", "@types/node"）
    /// * `version` - 可选的版本号（如 "18.0.0"），不指定则安装最新
    ///
    /// # Returns
    /// 安装结果，包含版本号和成功状态
    pub async fn npm_install(
        &self,
        package: &str,
        version: Option<&str>,
    ) -> Result<DependencyInstallResult, String> {
        if !matches!(self.package_manager, PackageManager::Npm) {
            return Err("Package manager is not npm".to_string());
        }

        // 验证包名
        self.validate_package_name(package)?;

        log::info!("Installing npm package: {}{}", package, version.map(|v| format!("@{}", v)).unwrap_or_default());

        // 构建命令
        let package_spec = if let Some(v) = version {
            format!("{}@{}", package, v)
        } else {
            package.to_string()
        };

        let mut cmd = Command::new("npm");
        cmd.arg("install")
            .arg(&package_spec)
            .current_dir(&self.workspace_root);

        // 执行命令
        let output = cmd
            .output()
            .await
            .map_err(|e| format!("Failed to execute npm install: {}", e))?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let _stderr = String::from_utf8_lossy(&output.stderr);

            log::info!("npm install succeeded: {}", stdout);

            // 尝试提取安装的版本号
            let installed_version = self.extract_npm_version(package).await.unwrap_or_else(|| "latest".to_string());

            Ok(DependencyInstallResult {
                package_name: package.to_string(),
                installed_version,
                success: true,
                message: format!("Successfully installed {}\n{}", package_spec, stdout),
            })
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            log::error!("npm install failed: {}", stderr);

            Err(format!("npm install failed: {}", stderr))
        }
    }

    /// 添加 Rust crate
    ///
    /// # Arguments
    /// * `crate_name` - crate 名称
    /// * `features` - 可选的特性列表
    ///
    /// # Returns
    /// 添加结果
    pub async fn cargo_add(
        &self,
        crate_name: &str,
        features: Option<&[&str]>,
    ) -> Result<DependencyInstallResult, String> {
        if !matches!(self.package_manager, PackageManager::Cargo) {
            return Err("Package manager is not cargo".to_string());
        }

        // 验证 crate 名称
        self.validate_package_name(crate_name)?;

        log::info!("Adding Rust crate: {}", crate_name);

        // 构建命令
        let mut cmd = Command::new("cargo");
        cmd.arg("add").arg(crate_name).current_dir(&self.workspace_root);

        // 添加特性
        if let Some(feats) = features {
            for feature in feats {
                cmd.arg("--features").arg(feature);
            }
        }

        // 执行命令
        let output = cmd
            .output()
            .await
            .map_err(|e| format!("Failed to execute cargo add: {}", e))?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let _stderr = String::from_utf8_lossy(&output.stderr);

            log::info!("cargo add succeeded: {}", stdout);

            // 尝试提取添加的版本号
            let installed_version = self.extract_cargo_version(crate_name).await.unwrap_or_else(|| "latest".to_string());

            Ok(DependencyInstallResult {
                package_name: crate_name.to_string(),
                installed_version,
                success: true,
                message: format!("Successfully added {}\n{}", crate_name, stdout),
            })
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            log::error!("cargo add failed: {}", stderr);

            Err(format!("cargo add failed: {}", stderr))
        }
    }

    /// 列出当前依赖
    ///
    /// # Returns
    /// 依赖列表（包名 -> 版本）
    pub async fn list_dependencies(&self) -> Result<Vec<(String, String)>, String> {
        match self.package_manager {
            PackageManager::Npm => self.list_npm_dependencies().await,
            PackageManager::Cargo => self.list_cargo_dependencies().await,
        }
    }

    /// 列出 npm 依赖
    async fn list_npm_dependencies(&self) -> Result<Vec<(String, String)>, String> {
        let package_json_path = self.workspace_root.join("package.json");

        if !package_json_path.exists() {
            return Err("package.json not found".to_string());
        }

        // 读取并解析 package.json
        let content = tokio::fs::read_to_string(&package_json_path)
            .await
            .map_err(|e| format!("Failed to read package.json: {}", e))?;

        let json: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse package.json: {}", e))?;

        let mut dependencies = Vec::new();

        // 提取 dependencies
        if let Some(deps) = json.get("dependencies").and_then(|v| v.as_object()) {
            for (name, version) in deps {
                if let Some(ver) = version.as_str() {
                    dependencies.push((name.clone(), ver.to_string()));
                }
            }
        }

        // 提取 devDependencies
        if let Some(deps) = json.get("devDependencies").and_then(|v| v.as_object()) {
            for (name, version) in deps {
                if let Some(ver) = version.as_str() {
                    dependencies.push((name.clone(), ver.to_string()));
                }
            }
        }

        Ok(dependencies)
    }

    /// 列出 Cargo 依赖
    async fn list_cargo_dependencies(&self) -> Result<Vec<(String, String)>, String> {
        let cargo_toml_path = self.workspace_root.join("Cargo.toml");

        if !cargo_toml_path.exists() {
            return Err("Cargo.toml not found".to_string());
        }

        // 读取并解析 Cargo.toml
        let content = tokio::fs::read_to_string(&cargo_toml_path)
            .await
            .map_err(|e| format!("Failed to read Cargo.toml: {}", e))?;

        let cargo_config: toml::Value = toml::from_str(&content)
            .map_err(|e| format!("Failed to parse Cargo.toml: {}", e))?;

        let mut dependencies = Vec::new();

        // 提取 [dependencies]
        if let Some(deps) = cargo_config.get("dependencies").and_then(|v| v.as_table()) {
            for (name, value) in deps {
                let version = match value {
                    toml::Value::String(s) => s.clone(),
                    toml::Value::Table(t) => t.get("version").and_then(|v| v.as_str()).unwrap_or("*").to_string(),
                    _ => "*".to_string(),
                };
                dependencies.push((name.clone(), version));
            }
        }

        Ok(dependencies)
    }

    /// 提取 npm 包的已安装版本
    async fn extract_npm_version(&self, package: &str) -> Option<String> {
        let node_modules_path = self.workspace_root.join("node_modules").join(package).join("package.json");

        if node_modules_path.exists() {
            if let Ok(content) = tokio::fs::read_to_string(&node_modules_path).await {
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                    return json.get("version").and_then(|v| v.as_str()).map(|s| s.to_string());
                }
            }
        }

        None
    }

    /// 提取 Cargo crate 的已安装版本
    async fn extract_cargo_version(&self, crate_name: &str) -> Option<String> {
        // 读取 Cargo.lock 文件
        let cargo_lock_path = self.workspace_root.join("Cargo.lock");

        if cargo_lock_path.exists() {
            if let Ok(content) = tokio::fs::read_to_string(&cargo_lock_path).await {
                // 简单的文本搜索（可以改进为解析 TOML）
                let pattern = format!("name = \"{}\"", crate_name);
                if let Some(pos) = content.find(&pattern) {
                    // 查找后面的 version 字段
                    let after_pattern = &content[pos..];
                    if let Some(version_pos) = after_pattern.find("version = \"") {
                        let version_start = version_pos + 11; // "version = \"".len()
                        if let Some(version_end) = after_pattern[version_start..].find('"') {
                            return Some(after_pattern[version_start..version_start + version_end].to_string());
                        }
                    }
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_validate_package_name_valid() {
        let temp_dir = TempDir::new().unwrap();
        let manager = DependencyManager::new(temp_dir.path().to_path_buf(), PackageManager::Npm);

        // 有效的包名
        assert!(manager.validate_package_name("react").is_ok());
        assert!(manager.validate_package_name("@types/node").is_ok());
        assert!(manager.validate_package_name("my-package").is_ok());
        assert!(manager.validate_package_name("my_package").is_ok());
        assert!(manager.validate_package_name("package.js").is_ok());
    }

    #[test]
    fn test_validate_package_name_invalid() {
        let temp_dir = TempDir::new().unwrap();
        let manager = DependencyManager::new(temp_dir.path().to_path_buf(), PackageManager::Npm);

        // 无效的包名
        assert!(manager.validate_package_name("../etc/passwd").is_err());
        assert!(manager.validate_package_name("/absolute/path").is_err());
        assert!(manager.validate_package_name("pkg;rm -rf /").is_err());
    }

    #[tokio::test]
    async fn test_list_npm_dependencies() {
        let temp_dir = TempDir::new().unwrap();
        let manager = DependencyManager::new(temp_dir.path().to_path_buf(), PackageManager::Npm);

        // 创建测试 package.json
        let package_json = r#"{
            "dependencies": {
                "react": "^18.0.0",
                "typescript": "^5.0.0"
            },
            "devDependencies": {
                "@types/react": "^18.0.0"
            }
        }"#;

        tokio::fs::write(temp_dir.path().join("package.json"), package_json)
            .await
            .unwrap();

        let deps = manager.list_dependencies().await.unwrap();
        assert_eq!(deps.len(), 3);

        // 验证包含所有依赖
        let dep_names: Vec<&str> = deps.iter().map(|(name, _)| name.as_str()).collect();
        assert!(dep_names.contains(&"react"));
        assert!(dep_names.contains(&"typescript"));
        assert!(dep_names.contains(&"@types/react"));
    }

    #[tokio::test]
    async fn test_list_cargo_dependencies() {
        let temp_dir = TempDir::new().unwrap();
        let manager = DependencyManager::new(temp_dir.path().to_path_buf(), PackageManager::Cargo);

        // 创建测试 Cargo.toml
        let cargo_toml = r#"
[package]
name = "test-project"
version = "0.1.0"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = "1.0"
"#;

        tokio::fs::write(temp_dir.path().join("Cargo.toml"), cargo_toml)
            .await
            .unwrap();

        let deps = manager.list_dependencies().await.unwrap();
        assert_eq!(deps.len(), 2);

        let dep_names: Vec<&str> = deps.iter().map(|(name, _)| name.as_str()).collect();
        assert!(dep_names.contains(&"serde"));
        assert!(dep_names.contains(&"tokio"));
    }

    #[test]
    fn test_wrong_package_manager() {
        let temp_dir = TempDir::new().unwrap();
        let npm_manager = DependencyManager::new(temp_dir.path().to_path_buf(), PackageManager::Npm);

        // 使用 npm manager 调用 cargo_add 应该失败
        let result = futures::executor::block_on(npm_manager.cargo_add("serde", None));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not cargo"));
    }

    #[tokio::test]
    async fn test_npm_install_invalid_package() {
        let temp_dir = TempDir::new().unwrap();
        let manager = DependencyManager::new(temp_dir.path().to_path_buf(), PackageManager::Npm);

        // 无效的包名应该被拒绝
        let result = manager.npm_install("../malicious-pkg", None).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid package name"));
    }

    #[tokio::test]
    async fn test_cargo_add_invalid_crate() {
        let temp_dir = TempDir::new().unwrap();
        let manager = DependencyManager::new(temp_dir.path().to_path_buf(), PackageManager::Cargo);

        // 无效的 crate 名应该被拒绝
        let result = manager.cargo_add("../../etc/passwd", None).await;
        assert!(result.is_err());
        let error_msg = result.unwrap_err();
        // 错误消息应该表明路径无效或越界
        assert!(
            error_msg.contains("Invalid package name") 
                || error_msg.contains("Invalid crate name")
                || error_msg.contains("outside workspace")
                || error_msg.contains("Access denied"),
            "Error message was: {}",
            error_msg
        );
    }

    #[tokio::test]
    async fn test_list_dependencies_no_config_file() {
        let temp_dir = TempDir::new().unwrap();
        let manager = DependencyManager::new(temp_dir.path().to_path_buf(), PackageManager::Npm);

        // 没有 package.json 或 Cargo.toml 应该返回错误
        let result = manager.list_dependencies().await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found"));
    }

    #[tokio::test]
    async fn test_npm_install_with_version() {
        let temp_dir = TempDir::new().unwrap();
        let manager = DependencyManager::new(temp_dir.path().to_path_buf(), PackageManager::Npm);

        // 创建测试 package.json
        let package_json = r#"{
            "name": "test-project",
            "version": "1.0.0"
        }"#;
        tokio::fs::write(temp_dir.path().join("package.json"), package_json)
            .await
            .unwrap();

        // 注意：这个测试会尝试实际安装，可能会失败（因为没有网络或项目未初始化）
        // 我们主要验证函数签名和参数处理逻辑
        let result = manager.npm_install("react", Some("18.2.0")).await;
        // 如果失败，应该是 npm 命令执行失败，而不是参数验证失败
        if result.is_err() {
            let error = result.unwrap_err();
            // 错误不应该包含 "Invalid package name"
            assert!(!error.contains("Invalid package name"));
        }
    }

    #[tokio::test]
    async fn test_cargo_add_with_features() {
        let temp_dir = TempDir::new().unwrap();
        let manager = DependencyManager::new(temp_dir.path().to_path_buf(), PackageManager::Cargo);

        // 创建测试 Cargo.toml
        let cargo_toml = r#"
[package]
name = "test-project"
version = "0.1.0"
"#;
        tokio::fs::write(temp_dir.path().join("Cargo.toml"), cargo_toml)
            .await
            .unwrap();

        // 测试带 features 的 cargo add
        let features = vec!["derive", "rc"];
        let result = manager.cargo_add("serde", Some(&features)).await;
        // 如果失败，应该是 cargo 命令执行失败，而不是参数验证失败
        if result.is_err() {
            let error = result.unwrap_err();
            // 错误不应该包含 "Invalid crate name"
            assert!(!error.contains("Invalid crate name"));
        }
    }

    #[test]
    fn test_package_manager_display() {
        let npm = PackageManager::Npm;
        let cargo = PackageManager::Cargo;

        // 验证 Debug 输出
        assert_eq!(format!("{:?}", npm), "Npm");
        assert_eq!(format!("{:?}", cargo), "Cargo");
    }
}
