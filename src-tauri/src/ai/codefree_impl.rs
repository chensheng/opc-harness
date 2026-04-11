//! CodeFree CLI Provider Implementation
//! 
//! 通过调用系统 codefree CLI 工具实现 AI 对话功能

use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

use super::ai_types::*;
use super::provider_core::AIProvider;

/// 智能查找并执行 CodeFree CLI
/// 返回完整的命令路径和是否需要通过 cmd.exe 执行的标志
async fn find_and_prepare_codefree() -> Result<(String, bool), AIError> {
    // 策略1: 直接尝试 "codefree"
    log::info!("Strategy 1: Trying direct 'codefree' command");
    
    // 先检查是否可以直接执行
    let test_cmd = Command::new("codefree")
        .arg("--version")
        .output()
        .await;
    
    if let Ok(output) = &test_cmd {
        if output.status.success() {
            log::info!("Direct execution successful");
            return Ok(("codefree".to_string(), false));
        }
    }
    
    // 策略2: Windows 上使用 where 命令查找
    #[cfg(windows)]
    {
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
                log::info!("Using batch file: {}", batch_path);
                return Ok((batch_path.clone(), true));
            }
            
            // 如果没有批处理文件，尝试其他文件
            if let Some(other_path) = other_files.first() {
                log::info!("Using non-batch file: {}", other_path);
                return Ok((other_path.clone(), false));
            }
        }
    }
    
    // 策略3: Unix 系统使用 which 命令
    #[cfg(not(windows))]
    {
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
            return Ok((path, false));
        }
    }
    
    Err(AIError {
        message: "CodeFree CLI not found in PATH. Please ensure it's installed and accessible.".to_string(),
    })
}

impl AIProvider {
    /// CodeFree CLI 非流式聊天
    pub(super) async fn chat_codefree(&self, request: ChatRequest) -> Result<ChatResponse, AIError> {
        // 智能查找 codefree 命令
        let (command_path, use_cmd) = find_and_prepare_codefree().await?;
        
        log::info!("Using CodeFree CLI path: {}, use_cmd: {}", command_path, use_cmd);
        
        // 构建查询内容（将消息合并为单个查询字符串）
        let query = request.messages.iter()
            .map(|m| format!("{}: {}", m.role, m.content))
            .collect::<Vec<_>>()
            .join("\n\n");
        
        // 根据是否需要 cmd.exe 来执行命令
        // 使用格式: codefree [query...] -o json
        let output = if use_cmd {
            log::info!("Executing via cmd.exe /c");
            log::info!("Full command: cmd.exe /c \"{}\" \"{}\" -o json", command_path, query.replace('\n', "\\n").chars().take(200).collect::<String>());
            Command::new("cmd")
                .args(&["/c", &command_path, &query, "-o", "json"])
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
            log::info!("Full command: {} \"{}\" -o json", command_path, query.replace('\n', "\\n").chars().take(200).collect::<String>());
            Command::new(&command_path)
                .arg(&query)
                .arg("-o")
                .arg("json")
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
                    // 检查错误消息是否包含 "CodeFree CLI 错误：" 前缀，说明已经提取到了具体错误
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
    fn parse_codefree_jsonl_response(&self, jsonl_str: &str) -> Result<String, AIError> {
        use serde_json::Value;
        
        let mut error_message: Option<String> = None;
        let mut assistant_texts: Vec<String> = Vec::new();
        let mut result_text: Option<String> = None;
        
        // 逐行解析 JSON
        for line in jsonl_str.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            
            // 尝试解析单个 JSON 对象
            if let Ok(value) = serde_json::from_str::<Value>(line) {
                if let Some(msg_type) = value.get("type").and_then(|v| v.as_str()) {
                    match msg_type {
                        "result" => {
                            // 检查是否为错误结果
                            if let Some(is_error) = value.get("is_error").and_then(|v| v.as_bool()) {
                                if is_error {
                                    if let Some(error_obj) = value.get("error") {
                                        if let Some(error_msg) = error_obj.get("message").and_then(|v| v.as_str()) {
                                            error_message = Some(error_msg.to_string());
                                        }
                                    }
                                }
                            }
                            // 提取成功结果
                            if let Some(result) = value.get("result").and_then(|v| v.as_str()) {
                                result_text = Some(result.to_string());
                            }
                        }
                        "assistant" => {
                            // 提取助手消息
                            if let Some(message) = value.get("message") {
                                if let Some(content_array) = message.get("content").and_then(|v| v.as_array()) {
                                    for item in content_array {
                                        if let Some(item_type) = item.get("type").and_then(|v| v.as_str()) {
                                            if item_type == "text" {
                                                if let Some(text) = item.get("text").and_then(|v| v.as_str()) {
                                                    assistant_texts.push(text.to_string());
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        
        // 优先级1: 如果有错误信息，返回错误
        if let Some(err_msg) = error_message {
            return Err(AIError {
                message: format!("CodeFree CLI 错误：{}", err_msg),
            });
        }
        
        // 优先级2: 如果有助手消息，返回拼接的文本
        if !assistant_texts.is_empty() {
            return Ok(assistant_texts.join("\n"));
        }
        
        // 优先级3: 如果有结果文本，返回结果
        if let Some(result) = result_text {
            return Ok(result);
        }
        
        Err(AIError {
            message: "未在 JSONL 响应中找到有效内容".to_string(),
        })
    }

    /// CodeFree CLI 流式聊天
    pub(super) async fn stream_chat_codefree<F>(
        &self,
        request: ChatRequest,
        on_chunk: F,
    ) -> Result<String, AIError>
    where
        F: FnMut(String) -> Result<(), AIError>,
    {
        // 智能查找 codefree 命令
        let (command_path, use_cmd) = find_and_prepare_codefree().await?;
        
        log::info!("Using CodeFree CLI path: {}, use_cmd: {}", command_path, use_cmd);
        
        // 构建 JSON 格式的消息数组
        let messages_json = serde_json::to_string(&request.messages)
            .map_err(|e| AIError {
                message: format!("序列化消息为 JSON 失败：{}", e),
            })?;
        
        log::info!("JSON input (first 200 chars): {}", messages_json.chars().take(200).collect::<String>());
        
        // 根据是否需要 cmd.exe 来构建命令
        // 使用 --input-format stream-json 和 -o stream-json
        let mut child = if use_cmd {
            log::info!("Executing via cmd.exe /c with JSON input (using temp file)");
            
            // 创建临时文件存储 JSON
            let temp_dir = std::env::temp_dir();
            let temp_file_path = temp_dir.join(format!("codefree_input_{}.json", std::process::id()));
            let temp_file_path_str = temp_file_path.to_string_lossy().to_string();
            
            // 写入 JSON 到临时文件
            tokio::fs::write(&temp_file_path, &messages_json).await
                .map_err(|e| AIError {
                    message: format!("写入临时 JSON 文件失败：{}", e),
                })?;
            
            log::info!("JSON written to temp file: {}", temp_file_path_str);
            
            // 使用 type 命令读取文件并管道传递（比 echo 更可靠）
            // 注意：在 cmd.exe /c "..." 中，内部命令不需要额外引号
            let full_command = format!("type \"{}\" | {} --input-format stream-json -o stream-json", temp_file_path_str, command_path);
            log::info!("Full command: cmd.exe /c \"{}\"", full_command.chars().take(300).collect::<String>());
            
            let child = Command::new("cmd")
                .args(&["/c", &full_command])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .map_err(|e| {
                    log::error!("Failed to spawn cmd.exe with codefree: {}", e);
                    AIError {
                        message: format!("CodeFree CLI 启动失败（cmd）：{}", e),
                    }
                })?;
            
            // 异步删除临时文件（在进程启动后）
            let temp_file_clone = temp_file_path.clone();
            tokio::spawn(async move {
                // 等待一小段时间确保进程已读取文件
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                let _ = tokio::fs::remove_file(&temp_file_clone).await;
            });
            
            child
        } else {
            log::info!("Executing directly with JSON input via stdin");
            // 直接执行，通过 stdin 传递 JSON
            let mut cmd = Command::new(&command_path);
            cmd.arg("--input-format")
                .arg("stream-json")
                .arg("-o")
                .arg("stream-json")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped());
            
            log::info!("Command: {} --input-format stream-json -o stream-json", command_path);
            
            let mut child = cmd.spawn()
                .map_err(|e| {
                    log::error!("Failed to execute codefree command: {}", e);
                    AIError {
                        message: format!("CodeFree CLI 启动失败：{}", e),
                    }
                })?;
            
            // 写入 JSON 到 stdin
            if let Some(mut stdin) = child.stdin.take() {
                use tokio::io::AsyncWriteExt;
                stdin.write_all(messages_json.as_bytes()).await
                    .map_err(|e| AIError {
                        message: format!("写入 JSON 到 stdin 失败：{}", e),
                    })?;
                stdin.flush().await
                    .map_err(|e| AIError {
                        message: format!("刷新 stdin 失败：{}", e),
                    })?;
                // stdin 会在 drop 时自动关闭
            }
            
            child
        };
        
        let stdout = child.stdout.take().ok_or_else(|| AIError {
            message: "无法获取 CodeFree CLI 输出".to_string(),
        })?;
        
        let stderr = child.stderr.take().ok_or_else(|| AIError {
            message: "无法获取 CodeFree CLI 错误输出".to_string(),
        })?;
        
        // 使用 Arc<Mutex> 包装 on_chunk 以便在多个异步任务中共享
        let on_chunk_shared = std::sync::Arc::new(tokio::sync::Mutex::new(on_chunk));
        
        // 用于收集完整内容的缓冲区
        let full_content_arc = std::sync::Arc::new(tokio::sync::Mutex::new(String::new()));
        let full_stderr_arc = std::sync::Arc::new(tokio::sync::Mutex::new(String::new()));
        
        // 克隆 Arc 用于异步任务
        let on_chunk_stdout = on_chunk_shared.clone();
        let full_content_clone = full_content_arc.clone();
        let full_stderr_clone = full_stderr_arc.clone();
        
        // 创建子进程的等待 future（不立即 await）
        let child_wait = Box::pin(child.wait());
        
        // 并发读取 stdout 和 stderr，同时监控进程退出
        let stdout_future = async {
            let mut reader = BufReader::new(stdout);
            let mut line = String::new();
            let mut content = String::new();
            
            loop {
                line.clear();
                match reader.read_line(&mut line).await {
                    Ok(0) => break, // EOF
                    Ok(_) => {
                        if !line.trim().is_empty() {
                            let raw_chunk = line.clone();
                            content.push_str(&raw_chunk);
                            
                            // 尝试解析当前行的 JSON 并提取文本内容
                            let trimmed_line = line.trim();
                            if let Ok(value) = serde_json::from_str::<serde_json::Value>(trimmed_line) {
                                if let Some(msg_type) = value.get("type").and_then(|v| v.as_str()) {
                                    match msg_type {
                                        "assistant" => {
                                            // 提取助手消息中的文本内容
                                            if let Some(message) = value.get("message") {
                                                if let Some(content_array) = message.get("content").and_then(|v| v.as_array()) {
                                                    for item in content_array {
                                                        if let Some(item_type) = item.get("type").and_then(|v| v.as_str()) {
                                                            if item_type == "text" {
                                                                if let Some(text) = item.get("text").and_then(|v| v.as_str()) {
                                                                    // 发送提取的文本内容到前端
                                                                    let mut callback_guard = on_chunk_stdout.lock().await;
                                                                    if let Err(e) = callback_guard(text.to_string()) {
                                                                        log::error!("Error in on_chunk callback: {}", e);
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        "result" => {
                                            // 提取最终结果
                                            if let Some(result_text) = value.get("result").and_then(|v| v.as_str()) {
                                                let mut callback_guard = on_chunk_stdout.lock().await;
                                                if let Err(e) = callback_guard(format!("\n\n{}", result_text)) {
                                                    log::error!("Error in on_chunk callback: {}", e);
                                                }
                                            }
                                        }
                                        _ => {
                                            // 其他类型（如 system、error）不显示
                                            log::debug!("Skipping {} message type", msg_type);
                                        }
                                    }
                                }
                            } else {
                                // 如果解析失败，记录日志但不发送原始 JSON
                                log::warn!("Failed to parse JSON line: {}", trimmed_line.chars().take(100).collect::<String>());
                            }
                        }
                    }
                    Err(e) => {
                        log::error!("Error reading stdout: {}", e);
                        break;
                    }
                }
            }
            
            // 将结果写入共享状态
            let mut guard = full_content_clone.lock().await;
            *guard = content;
        };
        
        let stderr_future = async {
            // 使用原始字节读取 stderr，避免 UTF-8 编码问题
            use tokio::io::AsyncReadExt;
            let mut err_reader = stderr;
            let mut buffer = Vec::new();
            
            match err_reader.read_to_end(&mut buffer).await {
                Ok(_) => {
                    // 尝试将字节转换为字符串，失败时使用替换字符
                    let stderr_content = String::from_utf8_lossy(&buffer).to_string();
                    
                    if !stderr_content.trim().is_empty() {
                        log::warn!("CodeFree stderr output ({} bytes): {}", buffer.len(), stderr_content.chars().take(500).collect::<String>());
                    }
                    
                    // 将结果写入共享状态
                    let mut guard = full_stderr_clone.lock().await;
                    *guard = stderr_content;
                }
                Err(e) => {
                    log::error!("Error reading stderr: {}", e);
                    let mut guard = full_stderr_clone.lock().await;
                    *guard = format!("读取 stderr 失败: {}", e);
                }
            }
        };
        
        // ✅ 关键改进：并发执行两个读取任务，它们会一直运行直到 EOF
        tokio::join!(stdout_future, stderr_future);
        
        // ✅ 然后等待进程退出，确保所有输出都已完成
        let status = child_wait.await.map_err(|e| AIError {
            message: format!("CodeFree CLI 进程等待失败：{}", e),
        })?;
        
        // 从共享状态获取结果
        let full_content = full_content_arc.lock().await.clone();
        let full_stderr = full_stderr_arc.lock().await.clone();
        
        // tokio::join! 返回的是 ()，因为 async 块没有返回值
        // 错误已经在各自的 async 块中通过 log::error 记录了
        
        if !status.success() {
            log::error!("CodeFree CLI exited with status: {}", status);
            
            // 尝试从已接收的 stdout 内容中解析 JSON 错误信息
            if !full_content.is_empty() {
                log::info!("Attempting to parse error from stream output (length: {})", full_content.len());
                
                // 策略1: 尝试解析为 JSONL 格式（每行一个 JSON 对象）
                match self.parse_codefree_jsonl_response(&full_content) {
                    Ok(_) => {
                        log::warn!("JSONL parsed successfully but exit code is non-zero");
                    }
                    Err(e) => {
                        log::info!("✓ Extracted error message from JSONL: {}", e.message.chars().take(100).collect::<String>());
                        return Err(e);
                    }
                }
                
                // 策略2: 如果 JSONL 解析失败，尝试解析为 JSON 数组格式
                if let Some(json_start) = full_content.rfind('[') {
                    let json_content = &full_content[json_start..];
                    log::info!("Attempting to parse as JSON array format");
                    
                    match self.parse_codefree_json_response(json_content) {
                        Ok(_) => {
                            log::warn!("JSON array parsed successfully but exit code is non-zero");
                        }
                        Err(e) => {
                            log::info!("✓ Extracted error message from JSON array: {}", e.message.chars().take(100).collect::<String>());
                            return Err(e);
                        }
                    }
                }
            }
            
            // 如果所有解析都失败，返回详细的诊断信息
            let mut error_details = String::new();
            
            // 优先显示 stderr 中的实际错误信息
            if !full_stderr.is_empty() {
                error_details.push_str("【CodeFree CLI 错误详情】\n");
                error_details.push_str(&full_stderr);
                error_details.push_str("\n\n");
            }
            
            if !full_content.is_empty() {
                error_details.push_str("【已接收的标准输出】\n");
                error_details.push_str(&full_content.chars().take(1000).collect::<String>());
                error_details.push_str("\n\n");
            }
            
            error_details.push_str(&format!("退出码: {}", status));
            
            return Err(AIError {
                message: error_details,
            });
        }
        
        Ok(full_content)
    }
}
