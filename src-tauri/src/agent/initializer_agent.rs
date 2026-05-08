//! Initializer Agent 实现
//!
//! 负责 PRD 文档解析、环境检查、Git 仓库初始化和任务分解

use crate::agent::messages::Issue;
use crate::agent::prd_parser::{PRDParser, PRDParserConfig};
use serde::{Deserialize, Serialize};
use std::process::Command;

/// Initializer Agent 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitializerAgentConfig {
    /// Agent ID
    pub agent_id: String,
    /// 项目路径
    pub project_path: String,
    /// AI 服务配置
    pub ai_config: crate::ai::AIConfig,
    /// PRD 文档路径
    pub prd_file_path: Option<String>,
    /// PRD 内容（如果直接传入）
    pub prd_content: Option<String>,
}

/// PRD 解析结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PRDParseResult {
    /// 产品名称
    pub product_name: String,
    /// 产品描述
    pub product_description: String,
    /// 目标用户群体
    pub target_users: Vec<String>,
    /// 核心功能列表
    pub core_features: Vec<String>,
    /// 非功能性需求
    pub non_functional_requirements: Vec<String>,
    /// 技术栈建议
    pub suggested_tech_stack: Vec<String>,
    /// 识别出的 Issue 列表
    pub identified_issues: Vec<Issue>,
    /// 解析置信度 (0.0-1.0)
    pub confidence_score: f32,
}

impl PRDParseResult {
    /// 创建新的解析结果
    pub fn new(product_name: String, product_description: String) -> Self {
        Self {
            product_name,
            product_description,
            target_users: Vec::new(),
            core_features: Vec::new(),
            non_functional_requirements: Vec::new(),
            suggested_tech_stack: Vec::new(),
            identified_issues: Vec::new(),
            confidence_score: 0.0,
        }
    }

    /// 设置目标用户
    pub fn with_target_users(mut self, users: Vec<String>) -> Self {
        self.target_users = users;
        self
    }

    /// 设置核心功能
    pub fn with_core_features(mut self, features: Vec<String>) -> Self {
        self.core_features = features;
        self
    }

    /// 设置技术栈
    pub fn with_tech_stack(mut self, stack: Vec<String>) -> Self {
        self.suggested_tech_stack = stack;
        self
    }

    /// 设置识别的 Issues
    pub fn with_issues(mut self, issues: Vec<Issue>) -> Self {
        self.identified_issues = issues;
        self
    }

    /// 设置置信度
    pub fn with_confidence(mut self, score: f32) -> Self {
        self.confidence_score = score.clamp(0.0, 1.0);
        self
    }
}

/// 环境检查结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentCheckResult {
    /// 是否通过检查
    pub passed: bool,
    /// Git 是否已安装
    pub git_installed: bool,
    /// Git 版本
    pub git_version: Option<String>,
    /// Node.js 是否已安装
    pub node_installed: bool,
    /// Node.js 版本
    pub node_version: Option<String>,
    /// npm 是否已安装
    pub npm_installed: bool,
    /// npm 版本
    pub npm_version: Option<String>,
    /// Rust/Cargo 是否已安装
    pub cargo_installed: bool,
    /// Cargo 版本
    pub cargo_version: Option<String>,
    /// IDE 安装列表 (vscode, cursor)
    pub ide_installed: Vec<String>,
    /// 项目目录是否存在
    pub project_dir_exists: bool,
    /// 错误信息
    pub errors: Vec<String>,
    /// 警告信息
    pub warnings: Vec<String>,
}

impl EnvironmentCheckResult {
    /// 创建成功的检查结果
    pub fn success() -> Self {
        Self {
            passed: true,
            git_installed: true,
            git_version: None,
            node_installed: true,
            node_version: None,
            npm_installed: true,
            npm_version: None,
            cargo_installed: true,
            cargo_version: None,
            ide_installed: Vec::new(),
            project_dir_exists: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    /// 创建失败的结果
    pub fn failure(errors: Vec<String>) -> Self {
        Self {
            passed: false,
            git_installed: false,
            git_version: None,
            node_installed: false,
            node_version: None,
            npm_installed: false,
            npm_version: None,
            cargo_installed: false,
            cargo_version: None,
            ide_installed: Vec::new(),
            project_dir_exists: false,
            errors,
            warnings: Vec::new(),
        }
    }

    /// 添加 Git 版本信息
    pub fn with_git_version(mut self, version: String) -> Self {
        self.git_installed = true;
        self.git_version = Some(version);
        self
    }

    /// 添加 Node.js 版本信息
    pub fn with_node_version(mut self, version: String) -> Self {
        self.node_installed = true;
        self.node_version = Some(version);
        self
    }

    /// 添加 npm 版本信息
    pub fn with_npm_version(mut self, version: String) -> Self {
        self.npm_installed = true;
        self.npm_version = Some(version);
        self
    }

    /// 添加 Cargo 版本信息
    pub fn with_cargo_version(mut self, version: String) -> Self {
        self.cargo_installed = true;
        self.cargo_version = Some(version);
        self
    }

    /// 添加 IDE 安装信息
    pub fn with_ide(mut self, ide: String) -> Self {
        self.ide_installed.push(ide);
        self
    }

    /// 添加错误
    pub fn add_error(mut self, error: String) -> Self {
        self.errors.push(error);
        self.passed = false;
        self
    }

    /// 添加警告
    pub fn add_warning(mut self, warning: String) -> Self {
        self.warnings.push(warning);
        self
    }
}

/// 任务分解结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskDecompositionResult {
    /// 是否成功分解
    pub success: bool,
    /// 分解出的 Issue 列表
    pub issues: Vec<Issue>,
    /// 预估总工作量 (小时)
    pub total_estimated_hours: f32,
    /// 建议的开发顺序
    pub suggested_order: Vec<String>,
    /// 依赖关系图
    pub dependencies: Vec<(String, String)>, // (from_issue_id, to_issue_id)
    /// 风险提示
    pub risks: Vec<String>,
}

impl TaskDecompositionResult {
    /// 创建新的分解结果
    pub fn new(issues: Vec<Issue>) -> Self {
        let total_hours: f32 = issues.iter().filter_map(|i| i.estimated_hours).sum();

        Self {
            success: true,
            issues,
            total_estimated_hours: total_hours,
            suggested_order: Vec::new(),
            dependencies: Vec::new(),
            risks: Vec::new(),
        }
    }

    /// 设置开发顺序
    pub fn with_suggested_order(mut self, order: Vec<String>) -> Self {
        self.suggested_order = order;
        self
    }

    /// 设置依赖关系
    pub fn with_dependencies(mut self, deps: Vec<(String, String)>) -> Self {
        self.dependencies = deps;
        self
    }

    /// 添加风险提示
    pub fn add_risk(mut self, risk: String) -> Self {
        self.risks.push(risk);
        self
    }
}

/// Initializer Agent 执行状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InitializerStatus {
    /// 等待开始
    Pending,
    /// 正在解析 PRD
    ParsingPRD,
    /// 正在检查环境
    CheckingEnvironment,
    /// 正在初始化 Git
    InitializingGit,
    /// 正在分解任务
    DecomposingTasks,
    /// 等待 HITL 审查
    WaitingForHITL,
    /// 完成
    Completed,
    /// 失败
    Failed(String),
}

/// 初始化结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitializerResult {
    /// 是否成功
    pub success: bool,
    /// 消息
    pub message: String,
    /// 创建的 Issues 列表
    pub issues_created: Vec<String>,
    /// Git 是否已初始化
    pub git_initialized: bool,
    /// 环境检查是否通过
    pub environment_checked: bool,
}

/// Initializer Agent 结构体
#[derive(Debug, Clone)]
pub struct InitializerAgent {
    /// 配置信息
    pub config: InitializerAgentConfig,
    /// 当前状态
    pub status: InitializerStatus,
    /// 会话 ID
    pub session_id: String,
}

/// 环境检测工具函数
mod env_utils {
    use super::*;

    /// 检查命令是否可用并返回版本
    fn check_command_version(cmd: &str, version_arg: &str) -> Option<String> {
        Command::new(cmd)
            .arg(version_arg)
            .output()
            .ok()
            .and_then(|output| {
                if output.status.success() {
                    String::from_utf8(output.stdout).ok()
                } else {
                    None
                }
            })
            .map(|version| version.trim().to_string())
    }

    /// 检查 Git 是否已安装
    pub fn check_git() -> (bool, Option<String>) {
        let version = check_command_version("git", "--version");
        let installed = version.is_some();

        // Windows 特殊处理：如果失败，尝试 "git.exe"
        if !installed && cfg!(windows) {
            if let Ok(path) = std::env::var("ProgramFiles") {
                let git_path = format!("{}\\Git\\cmd\\git.exe", path);
                if Command::new(&git_path).arg("--version").output().is_ok() {
                    return (true, Some("git (Windows)".to_string()));
                }
            }
        }

        (installed, version)
    }

    /// 检查 Node.js 是否已安装
    pub fn check_nodejs() -> (bool, Option<String>) {
        let version = check_command_version("node", "--version");
        (version.is_some(), version)
    }

    /// 检查 npm 是否已安装
    pub fn check_npm() -> (bool, Option<String>) {
        let version = check_command_version("npm", "--version");
        (version.is_some(), version)
    }

    /// 检查 Cargo 是否已安装
    pub fn check_cargo() -> (bool, Option<String>) {
        let version = check_command_version("cargo", "--version");
        (version.is_some(), version)
    }

    /// 检查 IDE 是否已安装
    pub fn check_ide() -> Vec<String> {
        let mut ides = Vec::new();

        // 在测试环境中跳过 IDE 检测，避免启动 GUI 应用
        if std::env::var("HARNESS_TEST_MODE").is_ok() {
            return ides;
        }

        // 检查 Cursor（基于 VS Code 的现代化编辑器）
        if check_command_version("cursor", "--version").is_some() {
            ides.push("cursor".to_string());
        } else if cfg!(windows) {
            let cursor_paths = [
                "%LOCALAPPDATA%\\Programs\\Cursor\\Cursor.exe",
                "%PROGRAMFILES%\\Cursor\\Cursor.exe",
            ];
            for path in &cursor_paths {
                let expanded = expand_env_var(path);
                if std::path::Path::new(&expanded).exists() {
                    ides.push("cursor".to_string());
                    break;
                }
            }
        } else if cfg!(target_os = "macos") {
            let cursor_app = "/Applications/Cursor.app";
            if std::path::Path::new(cursor_app).exists() {
                ides.push("cursor".to_string());
            }
        }

        ides
    }

    /// 展开环境变量
    pub fn expand_env_var(path: &str) -> String {
        let mut result = path.to_string();
        for (key, value) in std::env::vars() {
            result = result.replace(&format!("%{}%", key), &value);
        }
        result
    }

    /// 检查项目目录是否存在
    pub fn check_project_dir(project_path: &str) -> bool {
        std::path::Path::new(project_path).exists()
    }
}

impl InitializerAgent {
    /// 创建新的 Initializer Agent
    pub fn new(config: InitializerAgentConfig) -> Self {
        Self {
            config,
            status: InitializerStatus::Pending,
            session_id: uuid::Uuid::new_v4().to_string(),
        }
    }

    /// 解析 PRD 文档
    ///
    /// VC-006: 实现 PRD 文档解析器
    pub async fn parse_prd(&mut self) -> Result<PRDParseResult, String> {
        self.status = InitializerStatus::ParsingPRD;

        // 1. 获取 PRD 内容
        let prd_content = self.get_prd_content()?;

        // 2. 创建 PRD 解析器
        let parser_config = PRDParserConfig {
            ai_config: self.config.ai_config.clone(),
            use_streaming: false,
        };
        let parser = PRDParser::new(parser_config);

        // 3. 执行 PRD 解析
        let prd_result = parser.parse_prd(&prd_content).await?;

        // 4. 转换为 PRDParseResult
        let parse_result =
            PRDParseResult::new(prd_result.product_name, prd_result.product_description)
                .with_target_users(prd_result.target_users)
                .with_core_features(prd_result.core_features)
                .with_tech_stack(prd_result.suggested_tech_stack)
                .with_confidence(prd_result.confidence_score);

        self.status = InitializerStatus::CheckingEnvironment;

        Ok(parse_result)
    }

    /// 获取 PRD 内容（从文件或参数）
    fn get_prd_content(&self) -> Result<String, String> {
        // 优先使用传入的 PRD 内容
        if let Some(content) = &self.config.prd_content {
            return Ok(content.clone());
        }

        // 否则从文件读取
        if let Some(file_path) = &self.config.prd_file_path {
            use std::fs;
            fs::read_to_string(file_path).map_err(|e| format!("读取 PRD 文件失败：{}", e))
        } else {
            Err("未提供 PRD 内容或文件路径".to_string())
        }
    }

    /// 检查环境
    ///
    /// VC-007: 实现环境检查逻辑
    pub async fn check_environment(&mut self) -> Result<EnvironmentCheckResult, String> {
        self.status = InitializerStatus::CheckingEnvironment;

        let mut result = EnvironmentCheckResult::success();

        // 1. 检查 Git
        let (git_installed, git_version) = env_utils::check_git();
        if git_installed {
            if let Some(version) = git_version {
                result = result.with_git_version(version);
            }
        } else {
            result = result.add_error("Git 未安装。请安装 Git: https://git-scm.com/".to_string());
        }

        // 2. 检查 Node.js
        let (node_installed, node_version) = env_utils::check_nodejs();
        if node_installed {
            if let Some(version) = node_version {
                result = result.with_node_version(version);
            }
        } else {
            result =
                result.add_error("Node.js 未安装。请安装 Node.js: https://nodejs.org/".to_string());
        }

        // 3. 检查 npm
        let (npm_installed, npm_version) = env_utils::check_npm();
        if npm_installed {
            if let Some(version) = npm_version {
                result = result.with_npm_version(version);
            }
        } else if node_installed {
            result = result.add_warning(
                "npm 未找到，但 Node.js 已安装。请确认 npm 是否正确配置。".to_string(),
            );
        } else {
            result = result.add_error("npm 未安装。npm 通常随 Node.js 一起安装。".to_string());
        }

        // 4. 检查 Cargo (Rust)
        let (cargo_installed, cargo_version) = env_utils::check_cargo();
        if cargo_installed {
            if let Some(version) = cargo_version {
                result = result.with_cargo_version(version);
            }
        } else {
            result = result.add_warning(
                "Cargo (Rust) 未安装。如果需要构建 Rust 项目，请安装：https://rustup.rs/"
                    .to_string(),
            );
        }

        // 5. 检查 IDE
        let ides = env_utils::check_ide();
        for ide in &ides {
            result = result.with_ide(ide.clone());
        }
        if ides.is_empty() {
            result = result.add_warning(
                "未检测到常见 IDE (VSCode/Cursor)。请确保已安装代码编辑器。".to_string(),
            );
        }

        // 6. 检查项目目录
        let project_exists = env_utils::check_project_dir(&self.config.project_path);
        result.project_dir_exists = project_exists;
        if !project_exists {
            result = result.add_error(format!("项目目录不存在：{}", self.config.project_path));
        }

        // 7. 添加版本兼容性警告
        let node_version_cloned = result.node_version.clone();
        if let Some(ref version) = node_version_cloned {
            if version.starts_with("v") && version.len() > 1 {
                if let Ok(major) = version[1..].split('.').next().unwrap_or("0").parse::<u32>() {
                    if major < 18 {
                        result = result.add_warning(format!(
                            "Node.js 版本 {} 可能过旧，建议使用 Node.js 18+ LTS 版本",
                            version
                        ));
                    }
                }
            }
        }

        Ok(result)
    }

    /// 初始化 Git 仓库
    ///
    /// VC-008: 验证 Git 仓库已初始化（项目创建/打开时已自动初始化）
    pub async fn initialize_git(&self) -> Result<bool, String> {
        use std::path::Path;

        let project_path = &self.config.project_path;

        // 1. 检查项目目录是否存在
        if !Path::new(project_path).exists() {
            return Err(format!("项目目录不存在：{}", project_path));
        }

        // 2. 检查 Git 仓库是否已初始化（应该已由项目创建/打开流程处理）
        let git_dir = Path::new(project_path).join(".git");
        if git_dir.exists() {
            log::info!("✅ Git 仓库已初始化：{}", project_path);
            return Ok(true);
        }

        // 3. 如果 Git 未初始化，记录警告但继续执行
        // 注意：正常情况下不应该到达这里，因为项目创建/打开时已初始化
        log::warn!("⚠️  Git 仓库未初始化：{}", project_path);
        log::warn!("   这可能是因为项目是在 Git 自动初始化功能添加之前创建的");
        log::warn!("   建议通过 GitDetector 组件手动初始化 Git");

        // 返回 false 表示 Git 未初始化，但不阻止后续流程
        Ok(false)
    }

    /// 配置 Git 用户信息
    ///
    /// @deprecated 此函数已不再使用，Git 用户配置现在在项目创建/打开时由 database.rs 处理
    #[allow(dead_code)]
    async fn configure_git_user(&self, project_path: &str) -> Result<(), String> {
        use tokio::process::Command;

        // 检查是否已配置全局用户名
        let user_name_check = Command::new("git")
            .args(["config", "--global", "user.name"])
            .output()
            .await;

        let needs_config = match user_name_check {
            Ok(output) => output.stdout.is_empty(),
            Err(_) => true,
        };

        if needs_config {
            // 使用默认配置（实际项目中应该从用户设置中获取）
            let default_name = "OPC-HARNESS User";
            let default_email = "harness@opc.local";

            log::info!("配置 Git 用户信息");

            // 设置用户名
            Command::new("git")
                .current_dir(project_path)
                .args(["config", "user.name", default_name])
                .output()
                .await
                .map_err(|e| format!("设置 Git 用户名失败：{}", e))?;

            // 设置邮箱
            Command::new("git")
                .current_dir(project_path)
                .args(["config", "user.email", default_email])
                .output()
                .await
                .map_err(|e| format!("设置 Git 邮箱失败：{}", e))?;

            log::info!("Git 用户信息配置完成");
        }

        Ok(())
    }

    /// 创建 .gitignore 文件
    ///
    /// @deprecated 此函数已不再使用，.gitignore 现在在项目创建/打开时由 database.rs 处理
    #[allow(dead_code)]
    fn create_gitignore(&self, project_path: &str) -> Result<(), String> {
        use std::fs::File;
        use std::io::Write;
        use std::path::Path;

        let gitignore_path = Path::new(project_path).join(".gitignore");

        // 如果 .gitignore 已存在，跳过创建
        if gitignore_path.exists() {
            return Ok(());
        }

        // Tauri + React 项目的标准 .gitignore 内容
        let gitignore_content = r#"# Logs
logs
*.log
npm-debug.log*
yarn-debug.log*
yarn-error.log*
pnpm-debug.log*
lerna-debug.log*

# Dependencies
node_modules
dist
dist-ssr
*.local

# Editor directories and files
.idea
.DS_Store
*.suo
*.ntvs*
*.njsproj
*.sln
*.sw?

# Rust/Tauri
/target
**/target/
**/dist/
**/dist-electron/
src-tauri/target
src-tauri/**/*.dll
src-tauri/**/*.pdb
src-tauri/**/*.exe
src-tauri/**/*.app
src-tauri/**/*.deb
src-tauri/**/*.rpm
src-tauri/**/*.dmg
src-tauri/**/*.msi
src-tauri/**/*.AppImage
src-tauri/**/*.sig
src-tauri/**/*.updater.json

# Build outputs
**/build/
**/out/

# Environment variables
.env
.env.local
.env.development.local
.env.test.local
.env.production.local

# Database
*.db
*.sqlite
*.sqlite3
agent_sessions.db

# Test coverage
coverage/
*.lcov
.nyc_output/

# Temporary files
tmp/
temp/
*.tmp
*.bak
*.swp
*~

"#;

        let mut file = File::create(&gitignore_path)
            .map_err(|e| format!("创建 .gitignore 文件失败：{}", e))?;

        file.write_all(gitignore_content.as_bytes())
            .map_err(|e| format!("写入 .gitignore 文件失败：{}", e))?;

        Ok(())
    }
}
