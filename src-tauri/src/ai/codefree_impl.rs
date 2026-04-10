//! CodeFree CLI Provider Implementation
//! 
//! 通过调用系统 codefree CLI 工具实现 AI 对话功能

use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

use super::ai_types::*;
use super::provider_core::AIProvider;

impl AIProvider {
    /// CodeFree CLI 非流式聊天
    pub(super) async fn chat_codefree(&self, request: ChatRequest) -> Result<ChatResponse, AIError> {
        // 构建 codefree 命令参数
        let mut cmd = Command::new("codefree");
        
        // 添加模型参数（如果指定）
        if !request.model.is_empty() && request.model != "default" {
            cmd.arg("-m").arg(&request.model);
        }
        
        // 添加 prompt 参数
        let prompt = request.messages.iter()
            .map(|m| format!("{}: {}", m.role, m.content))
            .collect::<Vec<_>>()
            .join("\n\n");
        
        cmd.arg("--prompt").arg(&prompt);
        
        log::info!("Executing CodeFree CLI command: {:?}", cmd);
        
        // 执行命令并捕获输出
        let output = cmd
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .map_err(|e| {
                log::error!("Failed to execute codefree command: {}", e);
                AIError {
                    message: format!("CodeFree CLI 执行失败：{}", e),
                }
            })?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            log::error!("CodeFree CLI error: {}", stderr);
            return Err(AIError {
                message: format!("CodeFree CLI 错误：{}", stderr),
            });
        }
        
        let content = String::from_utf8_lossy(&output.stdout).to_string();
        
        log::info!("CodeFree CLI response received, length: {}", content.len());
        
        Ok(ChatResponse {
            content,
            model: request.model.clone(),
            usage: None, // CLI 不提供 token 使用统计
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
        // 构建 codefree 命令参数
        let mut cmd = Command::new("codefree");
        
        // 添加模型参数（如果指定）
        if !request.model.is_empty() && request.model != "default" {
            cmd.arg("-m").arg(&request.model);
        }
        
        // 添加 prompt 参数
        let prompt = request.messages.iter()
            .map(|m| format!("{}: {}", m.role, m.content))
            .collect::<Vec<_>>()
            .join("\n\n");
        
        cmd.arg("--prompt").arg(&prompt);
        
        log::info!("Executing CodeFree CLI stream command: {:?}", cmd);
        
        // 启动进程
        let mut child = cmd
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| {
                log::error!("Failed to spawn codefree process: {}", e);
                AIError {
                    message: format!("CodeFree CLI 启动失败：{}", e),
                }
            })?;
        
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
