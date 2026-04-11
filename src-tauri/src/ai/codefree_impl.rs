//! CodeFree CLI Provider Implementation
//! 
//! 通过调用系统 codefree CLI 工具实现 AI 对话功能
//! 
//! CodeFree CLI 使用方法（基于 codefree --help）：
//! - 非交互式: codefree [query...] -o json
//! - 流式输出: codefree [query...] -o stream-json
//! - 指定模型: codefree -m <model> [query...]
//! - 交互模式: codefree -i "prompt"

use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

use super::ai_types::*;
use super::provider_core::AIProvider;

/// CodeFree CLI 命令配置
struct CodeFreeCommand {
    /// 可执行文件路径或命令名
    executable: String,
    /// 是否通过 cmd.exe 执行（Windows .cmd/.bat 文件需要）
    use_shell: bool,
}

/// 智能查找并准备 CodeFree CLI 执行环境
/// 
/// 返回可执行路径和执行方式配置
async fn find_codefree_executable() -> Result<CodeFreeCommand, AIError> {
    log::info!("🔍 Searching for CodeFree CLI...");
    
    // 策略1: 直接尝试 "codefree" 命令
    if test_codefree_command("codefree", false).await.is_ok() {
        log::info!("✅ Found CodeFree CLI via direct execution");
        return Ok(CodeFreeCommand {
            executable: "codefree".to_string(),
            use_shell: false,
        });
    }
    
    // 策略2: Windows 上使用 where 命令查找完整路径
    #[cfg(windows)]
    {
        if let Some(path) = find_via_where().await? {
            let is_batch = path.to_lowercase().ends_with(".cmd") 
                || path.to_lowercase().ends_with(".bat");
            
            log::info!("✅ Found CodeFree CLI at: {} (batch: {})", path, is_batch);
            return Ok(CodeFreeCommand {
                executable: path,
                use_shell: is_batch,
            });
        }
    }
    
    // 策略3: Unix 系统使用 which 命令
    #[cfg(not(windows))]
    {
        if let Some(path) = find_via_which().await? {
            log::info!("✅ Found CodeFree CLI at: {}", path);
            return Ok(CodeFreeCommand {
                executable: path,
                use_shell: false,
            });
        }
    }
    
    Err(AIError {
        message: format!(
            "❌ CodeFree CLI 未找到\n\n\
            可能原因：\n\
            1. 未全局安装 CodeFree CLI\n\
            2. Node.js/npm 不在系统 PATH 中\n\
            3. Tauri 应用未继承完整的系统环境变量\n\n\
            解决方案：\n\
            1. 确认已安装：在终端运行 'codefree --version'\n\
            2. 重新安装：npm install -g @codefree/cli\n\
            3. 重启应用以确保环境变量生效\n\
            4. 检查系统 PATH 是否包含 Node.js 安装目录"
        ),
    })
}

/// 测试 CodeFree 命令是否可用
async fn test_codefree_command(cmd: &str, use_shell: bool) -> Result<(), AIError> {
    let output = if use_shell {
        Command::new("cmd")
            .args(&["/c", cmd, "--version"])
            .output()
            .await
    } else {
        Command::new(cmd)
            .arg("--version")
            .output()
            .await
    };
    
    match output {
        Ok(out) if out.status.success() => Ok(()),
        Ok(out) => {
            let stderr = String::from_utf8_lossy(&out.stderr);
            log::debug!("Test command failed: {}", stderr.chars().take(200).collect::<String>());
            Err(AIError {
                message: format!("Command test failed: {}", stderr),
            })
        }
        Err(e) => {
            log::debug!("Test command error: {}", e);
            Err(AIError {
                message: format!("Command test error: {}", e),
            })
        }
    }
}

/// Windows: 通过 where 命令查找 codefree 路径
#[cfg(windows)]
async fn find_via_where() -> Result<Option<String>, AIError> {
    log::info!("Strategy 2: Using 'where' command to find full path");
    let where_output = Command::new("where")
        .arg("codefree")
        .output()
        .await
        .map_err(|e| AIError {
            message: format!("Failed to execute 'where' command: {}", e),
        })?;
    
    if where_output.status.success() {
        let paths = String::from_utf8_lossy(&where_output.stdout);
        log::info!("Found potential paths:\n{}", paths);
        
        // 收集所有候选路径
        let mut batch_files = Vec::new();
        let mut other_files = Vec::new();
        
        for path in paths.lines() {
            let path = path.trim();
            if !path.is_empty() {
                log::info!("Testing path: {}", path);
                
                // 检查是否是批处理文件
                let is_batch = path.to_lowercase().ends_with(".cmd") 
                    || path.to_lowercase().ends_with(".bat");
                
                if is_batch {
                    batch_files.push(path.to_string());
                } else {
                    other_files.push(path.to_string());
                }
            }
        }
        
        // 优先返回批处理文件（Windows 下 npm 全局脚本的正确形式）
        if let Some(batch_path) = batch_files.first() {
            return Ok(Some(batch_path.clone()));
        }
        
        // 如果没有批处理文件，尝试其他文件
        if let Some(other_path) = other_files.first() {
            return Ok(Some(other_path.clone()));
        }
    }
    
    Ok(None)
}

/// Unix: 通过 which 命令查找 codefree 路径
#[cfg(not(windows))]
async fn find_via_which() -> Result<Option<String>, AIError> {
    log::info!("Strategy 3: Using 'which' command");
    let which_output = Command::new("which")
        .arg("codefree")
        .output()
        .await
        .map_err(|e| AIError {
            message: format!("Failed to execute 'which' command: {}", e),
        })?;
    
    if which_output.status.success() {
        let path = String::from_utf8_lossy(&which_output.stdout).trim().to_string();
        log::info!("Found codefree at: {}", path);
        return Ok(Some(path));
    }
    
    Ok(None)
}

impl AIProvider {
    /// CodeFree CLI 非流式聊天
    pub(super) async fn chat_codefree(&self, request: ChatRequest) -> Result<ChatResponse, AIError> {
        // 智能查找 codefree 命令
        let cmd_config = find_codefree_executable().await?;
        
        log::info!("Using CodeFree CLI: {}, use_shell: {}", cmd_config.executable, cmd_config.use_shell);
        
        // 构建查询内容（将消息合并为单个查询字符串）
        let query = request.messages.iter()
            .map(|m| format!("{}: {}", m.role, m.content))
            .collect::<Vec<_>>()
            .join("\n\n");
        
        log::info!("Query length: {} chars", query.len());
        
        // 构建命令参数
        // 格式: codefree [query...] -o json [-m model]
        let mut args = vec![query.clone()];
        args.push("-o".to_string());
        args.push("json".to_string());
        
        // 如果指定了模型，添加 -m 参数
        if !request.model.is_empty() && request.model != "codefree-default" {
            args.push("-m".to_string());
            args.push(request.model.clone());
            log::info!("Using model: {}", request.model);
        }
        
        // 执行命令
        let output = if cmd_config.use_shell {
            log::info!("Executing via cmd.exe /c");
            // Windows 批处理文件需要通过 cmd.exe 执行
            let mut full_cmd = vec![cmd_config.executable.clone()];
            full_cmd.extend(args.clone());
            
            Command::new("cmd")
                .args(&["/c"])
                .args(&full_cmd)
                .output()
                .await
                .map_err(|e| {
                    log::error!("Failed to execute cmd.exe with codefree: {}", e);
                    AIError {
                        message: format!("CodeFree CLI 执行失败（cmd）：{}", e),
                    }
                })?
        } else {
            log::info!("Executing directly");
            Command::new(&cmd_config.executable)
                .args(&args)
                .output()
                .await
                .map_err(|e| {
                    log::error!("Failed to execute codefree command: {}", e);
                    AIError {
                        message: format!("CodeFree CLI 执行失败：{}", e),
                    }
                })?
        };
        
        // 检查输出
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        
        // 记录详细输出信息（用于调试）
        log::info!("CodeFree CLI exit status: {}", output.status);
        log::info!("CodeFree CLI stdout length: {}", stdout.len());
        log::info!("CodeFree CLI stderr length: {}", stderr.len());
        
        // 记录 stderr 内容（可能是警告或 JSON）
        if !stderr.is_empty() {
            log::warn!("CodeFree stderr (first 500 chars): {}", stderr.chars().take(500).collect::<String>());
        }
        
        // 策略1: 尝试从 stdout 解析 JSON
        if !stdout.is_empty() {
            log::info!("Attempting to parse JSON from stdout...");
            
            match self.parse_codefree_json_response(&stdout) {
                Ok(text) => {
                    log::info!("✓ Successfully parsed JSON from stdout, content length: {}", text.len());
                    return Ok(ChatResponse {
                        content: text,
                        model: request.model.clone(),
                        usage: None,
                    });
                }
                Err(e) => {
                    // 如果解析过程中提取到了具体的错误信息（如认证错误），直接返回
                    if e.message.starts_with("CodeFree CLI 错误：") {
                        log::info!("✓ Extracted specific error from stdout: {}", e.message.chars().take(100).collect::<String>());
                        return Err(e);
                    }
                    log::warn!("✗ Failed to parse JSON from stdout: {}", e);
                }
            }
        }
        
        // 策略2: 尝试从 stderr 解析 JSON
        // CodeFree CLI 经常将 JSON 输出到 stderr（尤其是错误情况）
        if !stderr.is_empty() {
            // 查找最后一个 '[' 开始的内容（JSON 数组起始）
            if let Some(json_start) = stderr.rfind('[') {
                let json_content = &stderr[json_start..];
                log::info!("Found potential JSON in stderr, length: {}", json_content.len());
                log::info!("JSON preview: {}", json_content.chars().take(200).collect::<String>());
                
                match self.parse_codefree_json_response(json_content) {
                    Ok(text) => {
                        log::info!("✓ Successfully parsed JSON from stderr, content length: {}", text.len());
                        return Ok(ChatResponse {
                            content: text,
                            model: request.model.clone(),
                            usage: None,
                        });
                    }
                    Err(e) => {
                        // 同样检查是否提取到了具体错误
                        if e.message.starts_with("CodeFree CLI 错误：") {
                            log::info!("✓ Extracted specific error from stderr: {}", e.message.chars().take(100).collect::<String>());
                            return Err(e);
                        }
                        log::warn!("✗ Failed to parse JSON from stderr: {}", e);
                    }
                }
            } else {
                log::warn!("No JSON array found in stderr");
            }
        }
        
        // 策略3: 所有解析都失败，返回详细的错误信息
        log::error!("All JSON parsing attempts failed");
        
        // 构建详细的错误消息
        let mut error_details = String::new();
        
        // 优先显示 stderr 中的实际错误信息
        if !stderr.is_empty() {
            error_details.push_str("【CodeFree CLI 错误详情】\n");
            error_details.push_str(&stderr);
            error_details.push_str("\n\n");
        }
        
        if !stdout.is_empty() {
            error_details.push_str("【标准输出】\n");
            error_details.push_str(&stdout.chars().take(1000).collect::<String>());
            error_details.push_str("\n\n");
        }
        
        error_details.push_str(&format!("退出码: {}", output.status));
        
        Err(AIError {
            message: error_details,
        })
    }

    /// 解析 CodeFree CLI 的 JSON 响应（数组格式）
    fn parse_codefree_json_response(&self, json_str: &str) -> Result<String, AIError> {
        use serde_json::Value;
        
        // 尝试解析 JSON 数组
        let responses: Vec<Value> = serde_json::from_str(json_str)
            .map_err(|e| AIError {
                message: format!("JSON 解析失败：{}", e),
            })?;
        
        // 第一遍遍历：检查是否有错误响应
        for response in &responses {
            if let Some(msg_type) = response.get("type").and_then(|v| v.as_str()) {
                if msg_type == "result" {
                    // 检查是否为错误结果
                    if let Some(is_error) = response.get("is_error").and_then(|v| v.as_bool()) {
                        if is_error {
                            // 提取错误信息
                            if let Some(error_obj) = response.get("error") {
                                if let Some(error_message) = error_obj.get("message").and_then(|v| v.as_str()) {
                                    return Err(AIError {
                                        message: format!("CodeFree CLI 错误：{}", error_message),
                                    });
                                }
                            }
                            // 如果没有详细错误信息，返回通用错误
                            return Err(AIError {
                                message: "CodeFree CLI 执行出错".to_string(),
                            });
                        }
                    }
                }
            }
        }
        
        // 第二遍遍历：查找 type 为 "assistant" 的消息
        for response in &responses {
            if let Some(msg_type) = response.get("type").and_then(|v| v.as_str()) {
                if msg_type == "assistant" {
                    // 提取 assistant 消息中的文本内容
                    if let Some(message) = response.get("message") {
                        if let Some(content_array) = message.get("content").and_then(|v| v.as_array()) {
                            // 拼接所有 text 类型的内容
                            let mut texts = Vec::new();
                            for item in content_array {
                                if let Some(item_type) = item.get("type").and_then(|v| v.as_str()) {
                                    if item_type == "text" {
                                        if let Some(text) = item.get("text").and_then(|v| v.as_str()) {
                                            texts.push(text.to_string());
                                        }
                                    }
                                }
                            }
                            return Ok(texts.join("\n"));
                        }
                    }
                }
            }
        }
        
        // 第三遍遍历：如果没有找到 assistant 消息，尝试从成功的 result 类型中提取
        for response in &responses {
            if let Some(msg_type) = response.get("type").and_then(|v| v.as_str()) {
                if msg_type == "result" {
                    if let Some(result_text) = response.get("result").and_then(|v| v.as_str()) {
                        return Ok(result_text.to_string());
                    }
                }
            }
        }
        
        Err(AIError {
            message: "未在 JSON 响应中找到助手消息".to_string(),
        })
    }

    /// 解析 CodeFree CLI 的 JSONL 响应（每行一个 JSON 对象，用于流式输出）
    fn parse_codefree_jsonl_line(&self, line: &str) -> Result<Option<String>, AIError> {
        use serde_json::Value;
        
        // 尝试解析单个 JSON 对象
        let value: Value = match serde_json::from_str(line) {
            Ok(v) => v,
            Err(_) => {
                // 如果不是有效的JSON，可能是纯文本内容（如Markdown PRD）
                // 直接返回该行作为文本内容
                log::debug!("Line is not valid JSON, treating as plain text (length: {})", line.len());
                return Ok(Some(line.to_string()));
            }
        };
        
        if let Some(msg_type) = value.get("type").and_then(|v| v.as_str()) {
            match msg_type {
                "result" => {
                    // 检查是否为错误结果
                    if let Some(is_error) = value.get("is_error").and_then(|v| v.as_bool()) {
                        if is_error {
                            if let Some(error_obj) = value.get("error") {
                                if let Some(error_msg) = error_obj.get("message").and_then(|v| v.as_str()) {
                                    return Err(AIError {
                                        message: format!("CodeFree CLI 错误：{}", error_msg),
                                    });
                                }
                            }
                            return Err(AIError {
                                message: "CodeFree CLI 执行出错".to_string(),
                            });
                        }
                    }
                    // 提取成功结果
                    if let Some(result) = value.get("result").and_then(|v| v.as_str()) {
                        return Ok(Some(result.to_string()));
                    }
                }
                "assistant" => {
                    // 提取助手消息
                    if let Some(message) = value.get("message") {
                        if let Some(content_array) = message.get("content").and_then(|v| v.as_array()) {
                            let mut texts = Vec::new();
                            for item in content_array {
                                if let Some(item_type) = item.get("type").and_then(|v| v.as_str()) {
                                    if item_type == "text" {
                                        if let Some(text) = item.get("text").and_then(|v| v.as_str()) {
                                            texts.push(text.to_string());
                                        }
                                    }
                                }
                            }
                            if !texts.is_empty() {
                                return Ok(Some(texts.join("\n")));
                            }
                        }
                    }
                }
                _ => {
                    // 其他类型（如 system、error）不处理
                    log::debug!("Skipping {} message type", msg_type);
                }
            }
        }
        
        Ok(None)
    }

    /// CodeFree CLI 流式聊天
    pub(super) async fn stream_chat_codefree<F>(
        &self,
        request: ChatRequest,
        mut on_chunk: F,
    ) -> Result<String, AIError>
    where
        F: FnMut(String) -> Result<(), AIError>,
    {
        // 智能查找 codefree 命令
        let cmd_config = find_codefree_executable().await?;
        
        log::info!("Using CodeFree CLI for streaming: {}, use_shell: {}", cmd_config.executable, cmd_config.use_shell);
        
        // 如果提供了 project_id，获取并切换到项目工作区目录
        let workspace_dir = if let Some(ref project_id) = request.project_id {
            use crate::utils::paths::get_workspaces_dir;
            
            let workspaces_root = get_workspaces_dir();
            let project_workspace = workspaces_root.join(project_id);
            
            if project_workspace.exists() {
                log::info!("[CodeFree] Switching to project workspace: {:?}", project_workspace);
                Some(project_workspace)
            } else {
                log::warn!("[CodeFree] Project workspace does not exist: {:?}", project_workspace);
                None
            }
        } else {
            None
        };
        
        // 构建查询内容
        let query = request.messages.iter()
            .map(|m| format!("{}: {}", m.role, m.content))
            .collect::<Vec<_>>()
            .join("\n\n");
        
        log::info!("Stream query length: {} chars", query.len());
        
        // 构建命令参数
        // 格式: codefree [query...] -o stream-json [-m model] [-y]
        let mut args = vec![query.clone()];
        args.push("-o".to_string());
        args.push("stream-json".to_string());
        
        // 如果指定了模型，添加 -m 参数
        if !request.model.is_empty() && request.model != "codefree-default" {
            args.push("-m".to_string());
            args.push(request.model.clone());
            log::info!("Using model for streaming: {}", request.model);
        }
        
        // 添加 -y 参数（YOLO 模式），自动接受所有操作，无需用户确认
        args.push("-y".to_string());
        log::info!("[CodeFree] YOLO mode enabled: automatically accept all actions");
        
        // 打印完整的 CodeFree CLI 命令（用于调试）
        if cmd_config.use_shell {
            let mut full_cmd = vec![cmd_config.executable.clone()];
            full_cmd.extend(args.clone());
            log::info!("[CodeFree] Full command: cmd.exe /c {}", full_cmd.join(" "));
        } else {
            log::info!("[CodeFree] Full command: {} {}", cmd_config.executable, args.join(" "));
        }
        
        // 启动进程（设置工作目录）
        let mut child = if cmd_config.use_shell {
            log::info!("Executing via cmd.exe /c for streaming");
            // Windows 批处理文件需要通过 cmd.exe 执行
            let mut full_cmd = vec![cmd_config.executable.clone()];
            full_cmd.extend(args.clone());
            
            let mut cmd = Command::new("cmd");
            cmd.args(&["/c"])
                .args(&full_cmd)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped());
            
            // 如果指定了工作目录，设置当前目录
            if let Some(ref dir) = workspace_dir {
                cmd.current_dir(dir);
                log::info!("[CodeFree] Set working directory to: {:?}", dir);
            }
            
            cmd.spawn().map_err(|e| {
                log::error!("Failed to spawn cmd.exe with codefree: {}", e);
                AIError {
                    message: format!("CodeFree CLI 启动失败（cmd）：{}", e),
                }
            })?
        } else {
            log::info!("Executing directly for streaming");
            let mut cmd = Command::new(&cmd_config.executable);
            cmd.args(&args)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped());
            
            // 如果指定了工作目录，设置当前目录
            if let Some(ref dir) = workspace_dir {
                cmd.current_dir(dir);
                log::info!("[CodeFree] Set working directory to: {:?}", dir);
            }
            
            cmd.spawn().map_err(|e| {
                log::error!("Failed to execute codefree command: {}", e);
                AIError {
                    message: format!("CodeFree CLI 启动失败：{}", e),
                }
            })?
        };
        
        let stdout = child.stdout.take().ok_or_else(|| AIError {
            message: "无法获取 CodeFree CLI 输出".to_string(),
        })?;
        
        let stderr = child.stderr.take().ok_or_else(|| AIError {
            message: "无法获取 CodeFree CLI 错误输出".to_string(),
        })?;
        
        // 创建子进程的等待 future
        let child_wait = Box::pin(child.wait());
        
        // 并发读取 stdout 和 stderr
        let stdout_future = async {
            let mut reader = BufReader::new(stdout);
            let mut line = String::new();
            let mut content = String::new();
            
            loop {
                line.clear();
                match reader.read_line(&mut line).await {
                    Ok(0) => break, // EOF
                    Ok(_) => {
                        let trimmed_line = line.trim();
                        if !trimmed_line.is_empty() {
                            content.push_str(trimmed_line);
                            content.push('\n');
                            
                            // 尝试解析当前行的 JSON 并提取文本内容
                            match self.parse_codefree_jsonl_line(trimmed_line) {
                                Ok(Some(text)) => {
                                    // 发送提取的文本内容到前端
                                    if let Err(e) = on_chunk(text) {
                                        log::error!("Error in on_chunk callback: {}", e);
                                    }
                                }
                                Ok(None) => {
                                    // 没有可提取的内容，跳过
                                }
                                Err(e) => {
                                    // 解析错误，记录日志
                                    log::warn!("Failed to parse JSONL line: {}, error: {}", 
                                        trimmed_line.chars().take(100).collect::<String>(), e);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        log::error!("Error reading stdout: {}", e);
                        break;
                    }
                }
            }
            
            content
        };
        
        let stderr_future = async {
            // 使用原始字节读取 stderr，避免 UTF-8 编码问题
            use tokio::io::AsyncReadExt;
            let mut err_reader = stderr;
            let mut buffer = Vec::new();
            
            match err_reader.read_to_end(&mut buffer).await {
                Ok(_) => {
                    let stderr_content = String::from_utf8_lossy(&buffer).to_string();
                    
                    if !stderr_content.trim().is_empty() {
                        log::warn!("CodeFree stderr output ({} bytes): {}", buffer.len(), stderr_content.chars().take(500).collect::<String>());
                    }
                    
                    stderr_content
                }
                Err(e) => {
                    log::error!("Error reading stderr: {}", e);
                    format!("读取 stderr 失败: {}", e)
                }
            }
        };
        
        // ✅ 关键改进：并发执行两个读取任务，它们会一直运行直到 EOF
        let (stdout_content, stderr_content) = tokio::join!(stdout_future, stderr_future);
        
        // ✅ 然后等待进程退出，确保所有输出都已完成
        let status = child_wait.await.map_err(|e| AIError {
            message: format!("CodeFree CLI 进程等待失败：{}", e),
        })?;
        
        if !status.success() {
            log::error!("CodeFree CLI exited with status: {}", status);
            
            // 尝试从已接收的 stdout 内容中解析 JSON 错误信息
            if !stdout_content.is_empty() {
                log::info!("Attempting to parse error from stream output (length: {})", stdout_content.len());
                
                // 逐行解析，尝试提取错误信息
                for line in stdout_content.lines() {
                    let trimmed = line.trim();
                    if !trimmed.is_empty() {
                        match self.parse_codefree_jsonl_line(trimmed) {
                            Err(e) => {
                                if e.message.starts_with("CodeFree CLI 错误：") {
                                    log::info!("✓ Extracted error message from stream: {}", e.message.chars().take(100).collect::<String>());
                                    return Err(e);
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
            
            // 如果所有解析都失败，返回详细的诊断信息
            let mut error_details = String::new();
            
            // 优先显示 stderr 中的实际错误信息
            if !stderr_content.is_empty() {
                error_details.push_str("【CodeFree CLI 错误详情】\n");
                error_details.push_str(&stderr_content);
                error_details.push_str("\n\n");
            }
            
            if !stdout_content.is_empty() {
                error_details.push_str("【已接收的标准输出】\n");
                error_details.push_str(&stdout_content.chars().take(1000).collect::<String>());
                error_details.push_str("\n\n");
            }
            
            error_details.push_str(&format!("退出码: {}", status));
            
            return Err(AIError {
                message: error_details,
            });
        }
        
        Ok(stdout_content)
    }
}
