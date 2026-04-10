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
        error_details.push_str("CodeFree CLI 执行失败\n\n");
        
        if !stdout.is_empty() {
            error_details.push_str("【标准输出】\n");
            error_details.push_str(&stdout.chars().take(1000).collect::<String>());
            error_details.push_str("\n\n");
        }
        
        if !stderr.is_empty() {
            error_details.push_str("【标准错误】\n");
            error_details.push_str(&stderr.chars().take(1000).collect::<String>());
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
        mut on_chunk: F,
    ) -> Result<String, AIError>
    where
        F: FnMut(String) -> Result<(), AIError>,
    {
        // 智能查找 codefree 命令
        let (command_path, use_cmd) = find_and_prepare_codefree().await?;
        
        log::info!("Using CodeFree CLI path: {}, use_cmd: {}", command_path, use_cmd);
        
        // 构建查询内容（将消息合并为单个查询字符串）
        let query = request.messages.iter()
            .map(|m| format!("{}: {}", m.role, m.content))
            .collect::<Vec<_>>()
            .join("\n\n");
        
        // 根据是否需要 cmd.exe 来构建命令
        // 流式输出使用 -o stream-json 格式
        let mut child = if use_cmd {
            log::info!("Executing via cmd.exe /c");
            log::info!("Full command: cmd.exe /c \"{}\" \"{}\" -o stream-json", command_path, query.replace('\n', "\\n").chars().take(200).collect::<String>());
            Command::new("cmd")
                .args(&["/c", &command_path, &query, "-o", "stream-json"])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .map_err(|e| {
                    log::error!("Failed to spawn cmd.exe with codefree: {}", e);
                    AIError {
                        message: format!("CodeFree CLI 启动失败（cmd）：{}", e),
                    }
                })?
        } else {
            log::info!("Executing directly");
            log::info!("Full command: {} \"{}\" -o stream-json", command_path, query.replace('\n', "\\n").chars().take(200).collect::<String>());
            Command::new(&command_path)
                .arg(&query)
                .arg("-o")
                .arg("stream-json")
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .map_err(|e| {
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
        
        let mut full_content = String::new();
        let mut reader = BufReader::new(stdout);
        let mut err_reader = BufReader::new(stderr);
        
        // 用于存储从 stderr 提取的错误信息
        let _stderr_error = std::sync::Arc::new(tokio::sync::Mutex::new(None::<String>));
        
        // 异步读取 stderr（不阻塞主流程）
        let stderr_handle = tokio::spawn(async move {
            let mut line = String::new();
            let mut full_stderr = String::new();
            
            loop {
                line.clear();
                match err_reader.read_line(&mut line).await {
                    Ok(0) => break,
                    Ok(_) => {
                        if !line.trim().is_empty() {
                            log::warn!("CodeFree stderr: {}", line.trim());
                            full_stderr.push_str(&line);
                        }
                    }
                    Err(e) => {
                        log::error!("Error reading stderr: {}", e);
                        break;
                    }
                }
            }
            
            // 尝试从 stderr 中提取 JSON 错误信息
            if let Some(json_start) = full_stderr.rfind('[') {
                let _json_content = &full_stderr[json_start..];
                // 这里我们只记录，实际解析会在主线程中进行
                log::info!("Found potential JSON in stderr for later parsing");
            }
        });
        
        // 逐行读取 stdout 并流式输出
        let mut line = String::new();
        loop {
            line.clear();
            match reader.read_line(&mut line).await {
                Ok(0) => break, // EOF
                Ok(_) => {
                    if !line.trim().is_empty() {
                        let chunk = line.clone();
                        full_content.push_str(&chunk);
                        
                        // 发送 chunk 到回调
                        if let Err(e) = on_chunk(chunk) {
                            log::error!("Error in on_chunk callback: {}", e);
                        }
                    }
                }
                Err(e) => {
                    log::error!("Error reading stdout: {}", e);
                    break;
                }
            }
        }
        
        // 等待进程结束
        let status = child.wait().await.map_err(|e| AIError {
            message: format!("CodeFree CLI 进程等待失败：{}", e),
        })?;
        
        // 等待 stderr 读取完成
        let _ = stderr_handle.await;
        
        if !status.success() {
            log::error!("CodeFree CLI exited with status: {}", status);
            
            // 尝试从已接收的内容中解析 JSON 错误信息
            if !full_content.is_empty() {
                log::info!("Attempting to parse error from stream output (length: {})", full_content.len());
                
                // 策略1: 尝试解析为 JSONL 格式（每行一个 JSON 对象）
                match self.parse_codefree_jsonl_response(&full_content) {
                    Ok(_) => {
                        // 如果解析成功但没有返回错误，说明是正常的 assistant 消息
                        // 这种情况不应该发生（因为 exit code 不为 0）
                        log::warn!("JSONL parsed successfully but exit code is non-zero");
                    }
                    Err(e) => {
                        // 提取到了具体的错误信息，直接返回
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
            error_details.push_str("CodeFree CLI 执行失败\n\n");
            
            if !full_content.is_empty() {
                error_details.push_str("【已接收的内容】\n");
                error_details.push_str(&full_content.chars().take(1000).collect::<String>());
                error_details.push_str("\n\n");
            }
            
            error_details.push_str("【说明】\n");
            error_details.push_str("CLI 将输出发送到了标准错误(stderr)，可能包含警告或错误信息。\n");
            error_details.push_str("请检查 CodeFree CLI 配置和认证设置。\n\n");
            
            error_details.push_str(&format!("退出码: {}", status));
            
            return Err(AIError {
                message: error_details,
            });
        }
        
        log::info!(
            "CodeFree stream finished, content length: {}",
            full_content.len()
        );
        
        Ok(full_content)
    }
}
