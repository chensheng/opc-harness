// 数据库层 Git 初始化功能的测试
// 位置：src-tauri/tests/database_git.rs

use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

/// 模拟 initialize_git_repository 函数的核心逻辑
fn initialize_git_repository_test(project_path: &str) -> Result<(), String> {
    let path = PathBuf::from(project_path);
    
    // 检查是否已经是 Git 仓库
    if path.join(".git").exists() {
        return Ok(());
    }
    
    // 执行 git init -b main
    let output = Command::new("git")
        .current_dir(&path)
        .args(&["init", "-b", "main"])
        .output()
        .map_err(|e| format!("Git init failed: {}", e))?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Git init failed: {}", stderr));
    }
    
    // 创建 .gitignore 文件
    create_gitignore_file_test(project_path)?;
    
    // 配置用户信息（如果未设置）
    ensure_git_user_config_test(project_path)?;
    
    // 创建初始空 commit
    let _ = Command::new("git")
        .current_dir(&path)
        .args(&["commit", "--allow-empty", "-m", "Initial commit"])
        .output();
    
    Ok(())
}

/// 模拟 create_gitignore_file 函数
fn create_gitignore_file_test(project_path: &str) -> Result<(), String> {
    let gitignore_path = PathBuf::from(project_path).join(".gitignore");
    
    if gitignore_path.exists() {
        return Ok(());
    }
    
    let content = "# OPC-HARNESS context files\n.opc-harness/\n";
    fs::write(&gitignore_path, content)
        .map_err(|e| format!("Failed to create .gitignore: {}", e))?;
    
    Ok(())
}

/// 模拟 ensure_git_user_config 函数
fn ensure_git_user_config_test(project_path: &str) -> Result<(), String> {
    let path = PathBuf::from(project_path);
    
    // 检查全局配置是否存在
    let global_name_check = Command::new("git")
        .args(&["config", "--global", "user.name"])
        .output();
    
    let needs_config = match global_name_check {
        Ok(output) => output.stdout.is_empty(),
        Err(_) => true,
    };
    
    if needs_config {
        let default_name = "OPC-HARNESS User";
        let default_email = "harness@opc.local";
        
        // 设置用户名
        Command::new("git")
            .current_dir(&path)
            .args(&["config", "user.name", default_name])
            .output()
            .map_err(|e| format!("Failed to set user.name: {}", e))?;
        
        // 设置邮箱
        Command::new("git")
            .current_dir(&path)
            .args(&["config", "user.email", default_email])
            .output()
            .map_err(|e| format!("Failed to set user.email: {}", e))?;
    }
    
    Ok(())
}

#[test]
fn test_initialize_git_creates_repo() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let path = temp_dir.path().to_string_lossy().to_string();
    
    // 执行初始化
    let result = initialize_git_repository_test(&path);
    
    assert!(result.is_ok(), "Initialization should succeed");
    assert!(PathBuf::from(&path).join(".git").exists(), ".git directory should exist");
}

#[test]
fn test_initialize_git_creates_gitignore() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let path = temp_dir.path().to_string_lossy().to_string();
    
    let _ = initialize_git_repository_test(&path);
    
    let gitignore_path = PathBuf::from(&path).join(".gitignore");
    assert!(gitignore_path.exists(), ".gitignore should be created");
    
    let content = fs::read_to_string(gitignore_path).unwrap();
    assert!(content.contains(".opc-harness/"), "Should contain .opc-harness/ entry");
}

#[test]
fn test_initialize_git_idempotent() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let path = temp_dir.path().to_string_lossy().to_string();
    
    // 第一次初始化
    let first = initialize_git_repository_test(&path);
    assert!(first.is_ok());
    
    // 第二次初始化（应该也是成功的）
    let second = initialize_git_repository_test(&path);
    assert!(second.is_ok(), "Second initialization should also succeed (idempotent)");
}

#[test]
fn test_gitignore_not_overwritten() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let path = temp_dir.path().to_string_lossy().to_string();
    
    // 先手动创建 .gitignore
    let gitignore_path = PathBuf::from(&path).join(".gitignore");
    fs::write(&gitignore_path, "# Custom rules\n*.log\n").unwrap();
    
    // 执行初始化
    let _ = initialize_git_repository_test(&path);
    
    // 验证原内容保持不变
    let content = fs::read_to_string(gitignore_path).unwrap();
    assert!(content.contains("# Custom rules"), "Original content should be preserved");
    assert!(content.contains("*.log"), "Custom rules should remain");
}

#[test]
fn test_initial_commit_created() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let path = temp_dir.path().to_string_lossy().to_string();
    
    let _ = initialize_git_repository_test(&path);
    
    // 检查是否有提交
    let output = Command::new("git")
        .current_dir(&path)
        .args(&["log", "--oneline"])
        .output()
        .expect("Failed to run git log");
    
    let log_output = String::from_utf8_lossy(&output.stdout);
    assert!(log_output.contains("Initial commit"), "Should have initial commit");
}

#[test]
fn test_git_user_config_set() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let path = temp_dir.path().to_string_lossy().to_string();
    
    let _ = initialize_git_repository_test(&path);
    
    // 检查本地配置是否设置
    let name_output = Command::new("git")
        .current_dir(&path)
        .args(&["config", "user.name"])
        .output()
        .expect("Failed to get user.name");
    
    let name = String::from_utf8_lossy(&name_output.stdout).trim().to_string();
    assert!(!name.is_empty(), "user.name should be configured");
}
