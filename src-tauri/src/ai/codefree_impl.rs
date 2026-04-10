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
        
        // 记录 stderr（可能是警告信息）
        if !stderr.is_empty() {
            log::warn!("CodeFree stderr: {}", stderr.replace('\n', " | "));
        }
        
        // 如果 stdout 有内容，即使 exit code 不为 0 也尝试解析
        // CodeFree CLI 可能会在产生有用输出的同时输出一些警告
        if !stdout.is_empty() {
            // 尝试解析 JSON 响应
            let content = match self.parse_codefree_json_response(&stdout) {
                Ok(text) => {
                    log::info!("CodeFree CLI JSON response parsed successfully, length: {}", text.len());
                    text
                }
                Err(e) => {
                    log::warn!("Failed to parse JSON response, using raw output: {}", e);
                    stdout.clone()
                }
            };
            
            return Ok(ChatResponse {
                content,
                model: request.model.clone(),
                usage: None,
            });
        }
        
        // 如果 stdout 为空且 exit code 不为 0，才返回错误
        if !output.status.success() {
            log::error!("CodeFree CLI exited with status: {}", output.status);
            return Err(AIError {
                message: if stderr.is_empty() {
                    format!("CodeFree CLI 执行失败：{}", output.status)
                } else {
                    format!("CodeFree CLI 错误：{}", stderr)
                },
            });
        }
        
        // 兜底：返回空内容
        Ok(ChatResponse {
            content: String::new(),
            model: request.model.clone(),
            usage: None,
        })
    }

    /// 解析 CodeFree CLI 的 JSON 响应
    fn parse_codefree_json_response(&self, json_str: &str) -> Result<String, AIError> {
        use serde_json::Value;
        
        // 尝试解析 JSON 数组
        let responses: Vec<Value> = serde_json::from_str(json_str)
            .map_err(|e| AIError {
                message: format!("JSON 解析失败：{}", e),
            })?;
        
        // 查找 type 为 "assistant" 的消息
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
        
        // 如果没有找到 assistant 消息，尝试从 result 类型中提取
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
        // 使用格式: codefree [query...] -o json
        let mut child = if use_cmd {
            log::info!("Executing via cmd.exe /c");
            log::info!("Full command: cmd.exe /c \"{}\" \"{}\" -o json", command_path, query.replace('\n', "\\n").chars().take(200).collect::<String>());
            Command::new("cmd")
                .args(&["/c", &command_path, &query, "-o", "json"])
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
            log::info!("Full command: {} \"{}\" -o json", command_path, query.replace('\n', "\\n").chars().take(200).collect::<String>());
            Command::new(&command_path)
                .arg(&query)
                .arg("-o")
                .arg("json")
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .map_err(|e| {
                    log::error!("Failed to spawn codefree process: {}", e);
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
        
        // 异步读取 stderr（用于错误日志）
        let stderr_handle = tokio::spawn(async move {
            let mut line = String::new();
            loop {
                line.clear();
                match err_reader.read_line(&mut line).await {
                    Ok(0) => break,
                    Ok(_) => {
                        if !line.trim().is_empty() {
                            log::warn!("CodeFree stderr: {}", line.trim());
                        }
                    }
                    Err(e) => {
                        log::error!("Error reading stderr: {}", e);
                        break;
                    }
                }
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
            return Err(AIError {
                message: format!("CodeFree CLI 异常退出：{}", status),
            });
        }
        
        log::info!(
            "CodeFree stream finished, content length: {}",
            full_content.len()
        );
        
        Ok(full_content)
    }
}
