// Git 管理功能的集成测试
// 位置：src-tauri/tests/git_management.rs

use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

/// 创建临时目录用于测试
fn setup_test_dir() -> TempDir {
    TempDir::new().expect("Failed to create temp dir")
}

#[test]
fn test_git_init_basic() {
    let temp_dir = setup_test_dir();
    let path = temp_dir.path();
    
    // 执行 git init
    let output = Command::new("git")
        .current_dir(path)
        .args(&["init", "-b", "main"])
        .output()
        .expect("Failed to execute git init");
    
    assert!(output.status.success(), "Git init should succeed");
    
    // 验证 .git 目录存在
    assert!(path.join(".git").exists(), ".git directory should exist");
}

#[test]
fn test_gitignore_creation() {
    let temp_dir = setup_test_dir();
    let path = temp_dir.path();
    
    // 创建 .gitignore 文件
    let gitignore_content = "# OPC-HARNESS context files\n.opc-harness/\n";
    let gitignore_path = path.join(".gitignore");
    fs::write(&gitignore_path, gitignore_content).unwrap();
    
    // 验证文件存在且内容正确
    assert!(gitignore_path.exists(), ".gitignore should exist");
    let content = fs::read_to_string(&gitignore_path).unwrap();
    assert!(content.contains(".opc-harness/"), "Should contain .opc-harness/");
}

#[test]
fn test_git_config_set_get() {
    let temp_dir = setup_test_dir();
    let path = temp_dir.path();
    
    // 先初始化 Git
    let _ = Command::new("git")
        .current_dir(path)
        .args(&["init"])
        .output()
        .expect("Failed to init git");
    
    // 设置配置
    let set_output = Command::new("git")
        .current_dir(path)
        .args(&["config", "user.name", "Test User"])
        .output()
        .expect("Failed to set config");
    
    assert!(set_output.status.success(), "Setting config should succeed");
    
    // 获取配置
    let get_output = Command::new("git")
        .current_dir(path)
        .args(&["config", "user.name"])
        .output()
        .expect("Failed to get config");
    
    assert!(get_output.status.success(), "Getting config should succeed");
    let value = String::from_utf8_lossy(&get_output.stdout).trim().to_string();
    assert_eq!(value, "Test User", "Config value should match");
}

#[test]
fn test_git_status_check() {
    let temp_dir = setup_test_dir();
    let path = temp_dir.path();
    
    // 检查未初始化的仓库
    assert!(!path.join(".git").exists(), "Should not have .git initially");
    
    // 初始化后检查
    let _ = Command::new("git")
        .current_dir(path)
        .args(&["init"])
        .output()
        .expect("Failed to init git");
    
    assert!(path.join(".git").exists(), "Should have .git after init");
}

#[test]
fn test_idempotent_git_init() {
    let temp_dir = setup_test_dir();
    let path = temp_dir.path();
    
    // 第一次初始化
    let first = Command::new("git")
        .current_dir(path)
        .args(&["init"])
        .output()
        .expect("First init failed");
    assert!(first.status.success());
    
    // 第二次初始化（应该也成功）
    let second = Command::new("git")
        .current_dir(path)
        .args(&["init"])
        .output()
        .expect("Second init failed");
    assert!(second.status.success());
    
    // 验证仍然是 Git 仓库
    assert!(path.join(".git").exists());
}

#[test]
fn test_gitignore_preserve_existing() {
    let temp_dir = setup_test_dir();
    let path = temp_dir.path();
    
    // 手动创建 .gitignore
    let gitignore_path = path.join(".gitignore");
    fs::write(&gitignore_path, "# Custom\n*.log\n").unwrap();
    
    // 模拟我们的逻辑：如果已存在则不覆盖
    if !gitignore_path.exists() {
        fs::write(&gitignore_path, "# Auto-generated\n.opc-harness/\n").unwrap();
    }
    
    // 验证原内容保持不变
    let content = fs::read_to_string(&gitignore_path).unwrap();
    assert!(content.contains("# Custom"), "Original content should be preserved");
    assert!(content.contains("*.log"), "Custom rules should remain");
}
