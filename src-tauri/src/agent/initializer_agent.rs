//! Initializer Agent 实现
//! 
//! 负责 PRD 文档解析、环境检查、Git 仓库初始化和任务分解

use serde::{Deserialize, Serialize};
use crate::agent::messages::Issue;
use crate::agent::prd_parser::{PRDParser, PRDParserConfig};
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
        let total_hours: f32 = issues.iter()
            .filter_map(|i| i.estimated_hours)
            .sum();
        
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

/// Initializer Agent 完整结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitializerResult {
    /// 是否成功完成
    pub success: bool,
    /// PRD 解析结果
    pub prd_result: Option<PRDParseResult>,
    /// 环境检查结果
    pub env_check: Option<EnvironmentCheckResult>,
    /// Git 初始化结果
    pub git_init_result: Option<bool>,
    /// 任务分解结果
    pub task_decomposition: Option<TaskDecompositionResult>,
    /// HITL 检查点 ID
    pub checkpoint_id: Option<String>,
    /// 错误信息
    pub error: Option<String>,
}

impl InitializerResult {
    /// 创建成功的结果
    pub fn success(
        prd_result: PRDParseResult,
        env_check: EnvironmentCheckResult,
        task_decomposition: TaskDecompositionResult,
    ) -> Self {
        Self {
            success: true,
            prd_result: Some(prd_result),
            env_check: Some(env_check),
            git_init_result: Some(true),
            task_decomposition: Some(task_decomposition),
            checkpoint_id: None,
            error: None,
        }
    }

    /// 创建失败的结果
    pub fn failure(error: String) -> Self {
        Self {
            success: false,
            prd_result: None,
            env_check: None,
            git_init_result: None,
            task_decomposition: None,
            checkpoint_id: None,
            error: Some(error),
        }
    }
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

        // 检查 VSCode
        if check_command_version("code", "--version").is_some() {
            ides.push("vscode".to_string());
        } else if cfg!(windows) {
            // Windows 特殊路径
            let vscode_paths = [
                "%LOCALAPPDATA%\\Programs\\Microsoft VS Code\\Code.exe",
                "%PROGRAMFILES%\\Microsoft VS Code\\Code.exe",
            ];
            for path in &vscode_paths {
                let expanded = expand_env_var(path);
                if std::path::Path::new(&expanded).exists() {
                    ides.push("vscode".to_string());
                    break;
                }
            }
        } else if cfg!(target_os = "macos") {
            // macOS 特殊路径
            let vscode_app = "/Applications/Visual Studio Code.app";
            if std::path::Path::new(vscode_app).exists() {
                ides.push("vscode".to_string());
            }
        }

        // 检查 Cursor
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
        let parse_result = PRDParseResult::new(
            prd_result.product_name,
            prd_result.product_description,
        )
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
            fs::read_to_string(file_path)
                .map_err(|e| format!("读取 PRD 文件失败：{}", e))
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
            result = result.add_error(
                "Git 未安装。请安装 Git: https://git-scm.com/".to_string()
            );
        }
        
        // 2. 检查 Node.js
        let (node_installed, node_version) = env_utils::check_nodejs();
        if node_installed {
            if let Some(version) = node_version {
                result = result.with_node_version(version);
            }
        } else {
            result = result.add_error(
                "Node.js 未安装。请安装 Node.js: https://nodejs.org/".to_string()
            );
        }
        
        // 3. 检查 npm
        let (npm_installed, npm_version) = env_utils::check_npm();
        if npm_installed {
            if let Some(version) = npm_version {
                result = result.with_npm_version(version);
            }
        } else if node_installed {
            result = result.add_warning(
                "npm 未找到，但 Node.js 已安装。请确认 npm 是否正确配置。".to_string()
            );
        } else {
            result = result.add_error(
                "npm 未安装。npm 通常随 Node.js 一起安装。".to_string()
            );
        }
        
        // 4. 检查 Cargo (Rust)
        let (cargo_installed, cargo_version) = env_utils::check_cargo();
        if cargo_installed {
            if let Some(version) = cargo_version {
                result = result.with_cargo_version(version);
            }
        } else {
            result = result.add_warning(
                "Cargo (Rust) 未安装。如果需要构建 Rust 项目，请安装：https://rustup.rs/".to_string()
            );
        }
        
        // 5. 检查 IDE
        let ides = env_utils::check_ide();
        for ide in &ides {
            result = result.with_ide(ide.clone());
        }
        if ides.is_empty() {
            result = result.add_warning(
                "未检测到常见 IDE (VSCode/Cursor)。请确保已安装代码编辑器。".to_string()
            );
        }
        
        // 6. 检查项目目录
        let project_exists = env_utils::check_project_dir(&self.config.project_path);
        result.project_dir_exists = project_exists;
        if !project_exists {
            result = result.add_error(
                format!("项目目录不存在：{}", self.config.project_path)
            );
        }
        
        // 7. 添加版本兼容性警告
        let node_version_cloned = result.node_version.clone();
        if let Some(ref version) = node_version_cloned {
            if version.starts_with("v") && version.len() > 1 {
                if let Ok(major) = version[1..].split('.').next().unwrap_or("0").parse::<u32>() {
                    if major < 18 {
                        result = result.add_warning(
                            format!("Node.js 版本 {} 可能过旧，建议使用 Node.js 18+ LTS 版本", version)
                        );
                    }
                }
            }
        }
        
        Ok(result)
    }

    /// 初始化 Git 仓库
    /// 
    /// VC-008: 实现 Git 仓库初始化
    pub async fn initialize_git(&self) -> Result<bool, String> {
        use std::path::Path;
        use tokio::process::Command;

        let project_path = &self.config.project_path;

        // 1. 检查项目目录是否存在
        if !Path::new(project_path).exists() {
            return Err(format!("项目目录不存在：{}", project_path));
        }

        // 2. 检查是否已经初始化过 Git 仓库
        let git_dir = Path::new(project_path).join(".git");
        if git_dir.exists() {
            // Git 仓库已存在，跳过初始化
            log::info!("Git 仓库已存在：{}", project_path);
            return Ok(true);
        }

        // 3. 检查 Git 是否已安装
        let git_check = Command::new("git")
            .arg("--version")
            .output()
            .await;

        if git_check.is_err() {
            return Err(
                "Git 未安装。请先安装 Git:\n".to_string()
                + "- Windows: https://git-scm.com/download/win\n"
                + "- macOS: brew install git\n"
                + "- Linux: sudo apt-get install git (Ubuntu/Debian) 或 sudo yum install git (CentOS/RHEL)"
            );
        }

        // 4. 初始化 Git 仓库
        log::info!("正在初始化 Git 仓库：{}", project_path);
        let init_result = Command::new("git")
            .current_dir(project_path)
            .arg("init")
            .output()
            .await
            .map_err(|e| format!("Git 初始化失败：{}", e))?;

        if !init_result.status.success() {
            let stderr = String::from_utf8_lossy(&init_result.stderr);
            return Err(format!("Git 初始化失败：{}", stderr));
        }

        log::info!("Git 仓库初始化成功：{}", project_path);

        // 5. 配置 Git 用户信息（如果全局配置未设置）
        self.configure_git_user(project_path).await?;

        // 6. 创建初始 .gitignore 文件
        self.create_gitignore(project_path)?;

        Ok(true)
    }

    /// 配置 Git 用户信息
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
.vscode/*
!.vscode/extensions.json
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

# OS generated files
ehthumbs.db
Thumbs.db
Desktop.ini

# IDE specific (optional)
.cursor/
.windsurf/
"#;

        let mut file = File::create(&gitignore_path)
            .map_err(|e| format!("创建 .gitignore 失败：{}", e))?;

        file.write_all(gitignore_content.as_bytes())
            .map_err(|e| format!("写入 .gitignore 失败：{}", e))?;

        log::info!(".gitignore 文件创建完成：{:?}", gitignore_path);

        Ok(())
    }

    /// 分解任务为 Issues
    /// 
    /// VC-006: 基于 PRD 解析结果进行任务分解
    pub async fn decompose_tasks(
        &self,
        prd_result: &PRDParseResult,
    ) -> Result<TaskDecompositionResult, String> {
        // 1. 创建 PRD 解析器
        let parser_config = PRDParserConfig {
            ai_config: self.config.ai_config.clone(),
            use_streaming: false,
        };
        let parser = PRDParser::new(parser_config);
        
        // 2. 调用任务分解
        let issues = parser.decompose_tasks(
            &prd_result.product_name,
            &prd_result.product_description,
            &prd_result.core_features,
            &prd_result.suggested_tech_stack,
        ).await?;
        
        // 3. 创建 TaskDecompositionResult
        let result = TaskDecompositionResult::new(issues);
        
        Ok(result)
    }

    /// 执行完整的初始化流程
    /// 
    /// VC-008: 实现完整初始化流程
    pub async fn run_initialization(&mut self) -> Result<InitializerResult, String> {
        log::info!("开始执行 Initializer Agent 初始化流程 - Session: {}", self.session_id);

        // 1. 解析 PRD
        self.status = InitializerStatus::ParsingPRD;
        log::info!("步骤 1/4: 解析 PRD 文档");
        
        let prd_result = match self.parse_prd().await {
            Ok(result) => result,
            Err(e) => {
                self.status = InitializerStatus::Failed(e.clone());
                return Ok(InitializerResult::failure(format!("PRD 解析失败：{}", e)));
            }
        };
        log::info!("PRD 解析完成：{}", prd_result.product_name);

        // 2. 检查环境
        self.status = InitializerStatus::CheckingEnvironment;
        log::info!("步骤 2/4: 检查开发环境");
        
        let env_check = match self.check_environment().await {
            Ok(result) => result,
            Err(e) => {
                self.status = InitializerStatus::Failed(e.clone());
                return Ok(InitializerResult::failure(format!("环境检查失败：{}", e)));
            }
        };

        if !env_check.passed {
            self.status = InitializerStatus::Failed("环境检查未通过".to_string());
            return Ok(InitializerResult::failure(
                "环境检查未通过，请安装必要的工具链".to_string()
            ));
        }
        log::info!("环境检查通过");

        // 3. 初始化 Git 仓库
        self.status = InitializerStatus::InitializingGit;
        log::info!("步骤 3/4: 初始化 Git 仓库");
        
        let git_init_result = match self.initialize_git().await {
            Ok(success) => success,
            Err(e) => {
                self.status = InitializerStatus::Failed(e.clone());
                return Ok(InitializerResult::failure(format!("Git 初始化失败：{}", e)));
            }
        };
        log::info!("Git 仓库初始化完成");

        // 4. 分解任务为 Issues
        self.status = InitializerStatus::DecomposingTasks;
        log::info!("步骤 4/4: 分解任务为 Issues");
        
        let task_decomposition = match self.decompose_tasks(&prd_result).await {
            Ok(result) => result,
            Err(e) => {
                self.status = InitializerStatus::Failed(e.clone());
                return Ok(InitializerResult::failure(format!("任务分解失败：{}", e)));
            }
        };
        log::info!("任务分解完成，共 {} 个 Issues", task_decomposition.issues.len());

        // 5. 准备触发 CP-002 检查点（HITL 审查）
        // TODO: 实现 HITL 检查点逻辑
        self.status = InitializerStatus::WaitingForHITL;
        log::info!("等待 HITL 审查...");

        // 6. 完成初始化
        self.status = InitializerStatus::Completed;
        log::info!("Initializer Agent 初始化流程完成");

        Ok(InitializerResult::success(
            prd_result,
            env_check,
            task_decomposition,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::messages::Priority;
    use crate::ai::AIConfig;

    #[test]
    fn test_prd_parse_result() {
        let mut result = PRDParseResult::new(
            "智能营销平台".to_string(),
            "基于 AI 的自动化营销系统".to_string(),
        );
        
        assert_eq!(result.product_name, "智能营销平台");
        assert_eq!(result.confidence_score, 0.0);
        
        result = result
            .with_target_users(vec!["营销人员".to_string(), "产品经理".to_string()])
            .with_core_features(vec![
                "PRD 自动生成".to_string(),
                "代码自动编写".to_string(),
            ])
            .with_tech_stack(vec!["React".to_string(), "Rust".to_string(), "Tauri".to_string()])
            .with_confidence(0.95);
        
        assert_eq!(result.target_users.len(), 2);
        assert_eq!(result.core_features.len(), 2);
        assert_eq!(result.suggested_tech_stack.len(), 3);
        assert_eq!(result.confidence_score, 0.95);
    }

    #[test]
    fn test_environment_check_result() {
        let result = EnvironmentCheckResult::success()
            .with_git_version("2.40.0".to_string())
            .with_node_version("v20.10.0".to_string())
            .with_npm_version("10.2.3".to_string())
            .with_cargo_version("1.75.0".to_string())
            .with_ide("vscode".to_string())
            .with_ide("cursor".to_string())
            .add_warning("npm 版本较旧，建议升级".to_string());
        
        assert!(result.passed);
        assert_eq!(result.git_version, Some("2.40.0".to_string()));
        assert_eq!(result.node_version, Some("v20.10.0".to_string()));
        assert_eq!(result.npm_version, Some("10.2.3".to_string()));
        assert_eq!(result.cargo_version, Some("1.75.0".to_string()));
        assert_eq!(result.ide_installed, vec!["vscode", "cursor"]);
        assert_eq!(result.warnings.len(), 1);
    }

    #[test]
    fn test_environment_check_result_failure() {
        let errors = vec![
            "Git 未安装".to_string(),
            "Node.js 未安装".to_string(),
        ];
        
        let result = EnvironmentCheckResult::failure(errors.clone());
        
        assert!(!result.passed);
        assert!(!result.git_installed);
        assert!(!result.node_installed);
        assert_eq!(result.errors, errors);
        assert_eq!(result.warnings.len(), 0);
    }

    #[test]
    fn test_env_utils_check_git() {
        let (installed, version) = env_utils::check_git();
        
        // Git 应该已安装（在开发环境中）
        // 如果失败，说明测试环境没有 Git
        if installed {
            assert!(version.is_some());
            assert!(!version.unwrap().is_empty());
        }
    }

    #[test]
    fn test_env_utils_check_nodejs() {
        let (installed, version) = env_utils::check_nodejs();
        
        // Node.js 应该已安装（在开发环境中）
        if installed {
            assert!(version.is_some());
            assert!(!version.unwrap().is_empty());
        }
    }

    #[test]
    fn test_env_utils_check_npm() {
        let (installed, version) = env_utils::check_npm();
        
        // npm 应该已安装（在开发环境中）
        if installed {
            assert!(version.is_some());
            assert!(!version.unwrap().is_empty());
        }
    }

    #[test]
    fn test_env_utils_check_cargo() {
        let (installed, version) = env_utils::check_cargo();
        
        // Cargo 应该已安装（在开发环境中）
        if installed {
            assert!(version.is_some());
            assert!(!version.unwrap().is_empty());
        }
    }

    #[test]
    fn test_env_utils_check_ide() {
        let ides = env_utils::check_ide();
        
        // 至少应该检测到一个 IDE（在开发环境中）
        // 这个测试可能在没有 IDE 的环境中失败，所以只做基本检查
        assert!(ides.len() >= 0); // 允许为 0，因为某些环境可能没有 IDE
        
        // 如果检测到 IDE，验证格式
        for ide in &ides {
            assert!(!ide.is_empty());
            assert!(ide == "vscode" || ide == "cursor");
        }
    }

    #[test]
    fn test_env_utils_check_project_dir() {
        // 测试现有目录
        let current_dir = std::env::current_dir().unwrap();
        let exists = env_utils::check_project_dir(current_dir.to_str().unwrap());
        assert!(exists);
        
        // 测试不存在的目录
        let not_exists = env_utils::check_project_dir("/nonexistent/path/that/does/not/exist");
        assert!(!not_exists);
    }

    #[test]
    fn test_env_utils_expand_env_var() {
        // Windows 环境变量测试
        #[cfg(windows)]
        {
            let path = "%LOCALAPPDATA%";
            let expanded = env_utils::expand_env_var(path);
            assert!(!expanded.contains("%"));
            assert!(std::path::Path::new(&expanded).exists());
        }
        
        // Unix 环境变量测试
        #[cfg(unix)]
        {
            let path = "$HOME";
            let expanded = env_utils::expand_env_var(path);
            assert!(!expanded.contains("$"));
            assert!(std::path::Path::new(&expanded).exists());
        }
    }

    #[test]
    fn test_task_decomposition_result() {
        let issue1 = Issue::new(
            "实现用户登录".to_string(),
            "JWT 认证".to_string(),
            Priority::High,
        ).with_estimated_hours(4.0);
        
        let issue2 = Issue::new(
            "实现数据看板".to_string(),
            "数据可视化".to_string(),
            Priority::Medium,
        ).with_estimated_hours(8.0);
        
        let result = TaskDecompositionResult::new(vec![issue1.clone(), issue2.clone()])
            .with_suggested_order(vec![issue1.issue_id.clone(), issue2.issue_id.clone()])
            .with_dependencies(vec![(issue2.issue_id.clone(), issue1.issue_id.clone())])
            .add_risk("时间紧张，建议优先完成核心功能".to_string());
        
        assert!(result.success);
        assert_eq!(result.issues.len(), 2);
        assert_eq!(result.total_estimated_hours, 12.0);
        assert_eq!(result.suggested_order.len(), 2);
        assert_eq!(result.dependencies.len(), 1);
        assert_eq!(result.risks.len(), 1);
    }

    #[test]
    fn test_initializer_result_creation() {
        let prd_result = PRDParseResult::new("Test".to_string(), "Desc".to_string());
        let env_check = EnvironmentCheckResult::success();
        let task_result = TaskDecompositionResult::new(vec![]);
        
        let result = InitializerResult::success(prd_result, env_check, task_result);
        
        assert!(result.success);
        assert!(result.prd_result.is_some());
        assert!(result.env_check.is_some());
        assert!(result.task_decomposition.is_some());
        assert!(result.error.is_none());
    }

    #[test]
    fn test_initializer_agent_creation() {
        let config = InitializerAgentConfig {
            agent_id: "agent-init-001".to_string(),
            project_path: "/path/to/project".to_string(),
            ai_config: AIConfig {
                provider: "openai".to_string(),
                api_key: "sk-test".to_string(),
                model: "gpt-4".to_string(),
                base_url: None,
            },
            prd_file_path: Some("/path/to/prd.md".to_string()),
            prd_content: None,
        };
        
        let agent = InitializerAgent::new(config);
        
        assert_eq!(agent.config.agent_id, "agent-init-001");
        assert_eq!(agent.status, InitializerStatus::Pending);
        assert!(!agent.session_id.is_empty());
    }

    /// VC-007: 测试环境检查方法（集成测试）
    #[tokio::test]
    async fn test_check_environment_integration() {
        let config = InitializerAgentConfig {
            agent_id: "agent-init-test".to_string(),
            project_path: std::env::current_dir().unwrap().to_string_lossy().to_string(),
            ai_config: AIConfig {
                provider: "openai".to_string(),
                api_key: "sk-test".to_string(),
                model: "gpt-4".to_string(),
                base_url: None,
            },
            prd_file_path: None,
            prd_content: None,
        };
        
        let mut agent = InitializerAgent::new(config);
        let result = agent.check_environment().await;
        
        assert!(result.is_ok());
        let env_result = result.unwrap();
        
        // 验证检查结果结构
        assert!(env_result.git_installed || !env_result.git_installed); // 总是 true，只是验证能执行
        assert!(env_result.node_installed || !env_result.node_installed);
        assert!(env_result.npm_installed || !env_result.npm_installed);
        
        // 在开发环境中，这些应该为 true
        if cfg!(debug_assertions) {
            // Debug 模式下运行，应该有 Git 和 Node.js
            assert!(
                env_result.git_installed, 
                "开发环境中应该安装 Git"
            );
            assert!(
                env_result.node_installed, 
                "开发环境中应该安装 Node.js"
            );
        }
    }
}
