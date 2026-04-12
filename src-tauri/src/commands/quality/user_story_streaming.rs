// 用户故事流式处理模块
// 负责流式分解用户故事并实时推送结果

use crate::commands::quality::types::DecomposeUserStoriesRequest;
use tauri::Emitter;

/// 分解用户故事(流式版本)
pub async fn decompose_user_stories_streaming(
    request: DecomposeUserStoriesRequest,
    app: tauri::AppHandle,
) -> Result<String, String> {
    use uuid::Uuid;
    
    let session_id = Uuid::new_v4().to_string();
    
    log::info!("Starting streaming user story decomposition (provider: {}, model: {})", 
               request.provider, request.model);
    
    // 1. 如果是 CodeFree，需要写入 AGENTS.md 和 PRD.md 文件
    let user_message = if request.provider == "codefree" {
        log::info!("[decompose_user_stories_streaming] 🎯 CodeFree provider detected!");
        
        if let Some(ref pid) = request.project_id {
            use crate::utils::paths::get_workspaces_dir;
            use std::fs;
            
            let workspaces_root = get_workspaces_dir();
            let workspace_path = workspaces_root.join(pid);
            let context_dir = workspace_path.join(".opc-harness");
            
            log::info!("[decompose_user_stories_streaming] 📁 Workspace path: {:?}", workspace_path);
            log::info!("[decompose_user_stories_streaming] 📁 Context directory: {:?}", context_dir);
            
            // 确保 .opc-harness 目录存在
            fs::create_dir_all(&context_dir).map_err(|e| {
                log::error!("[decompose_user_stories_streaming] Failed to create context directory: {}", e);
                format!("Failed to create context directory: {}", e)
            })?;
            
            // 写入 PRD.md
            let prd_md_path = context_dir.join("PRD.md");
            fs::write(&prd_md_path, &request.prd_content).map_err(|e| {
                log::error!("[decompose_user_stories_streaming] Failed to write PRD.md: {}", e);
                format!("Failed to write PRD.md: {}", e)
            })?;
            log::info!("[decompose_user_stories_streaming] ✅ PRD.md written to: {:?}", prd_md_path);
            
            // 写入 AGENTS.md 作为系统提示词（使用与其他 AI 厂商统一的提示词）
            let agents_md_path = context_dir.join("AGENTS.md");
            
            // 创建专门用于 CodeFree 的用户故事分解提示词模板（不包含具体 PRD 内容）
            let agents_content = r#"# AI Assistant - User Story Decomposition

## Role
你是一位经验丰富的敏捷开发专家和产品经理，擅长将产品需求拆分为符合 INVEST 原则的用户故事。

## Task
请分析 PRD（产品需求文档），将其拆分为符合 INVEST 原则的用户故事列表。

## ⚠️ 输出格式要求（非常重要）
**你必须以 Markdown 表格格式输出用户故事，不要输出 JSON！**

表格必须包含以下列：
- **序号**: 故事编号（US-001, US-002...）
- **标题**: 简洁的故事标题
- **角色**: 用户角色（As a...）
- **功能**: 想要的功能（I want...）
- **价值**: 业务价值（So that...）
- **优先级**: P0/P1/P2/P3
- **故事点**: 1/2/3/5/8/13
- **验收标准**: 3-5条验收标准，用分号分隔
- **模块**: 功能模块名称
- **标签**: 标签列表，用逗号分隔
- **依赖**: 依赖的故事编号，无则填"无"

### 表格示例

| 序号 | 标题 | 角色 | 功能 | 价值 | 优先级 | 故事点 | 验收标准 | 模块 | 标签 | 依赖 |
|------|------|------|------|------|--------|--------|----------|------|------|------|
| US-001 | 用户注册与登录 | 新用户 | 能够通过邮箱或手机号注册账号并登录系统 | 我可以访问系统的核心功能并保存我的个人数据 | P0 | 5 | 用户可以通过邮箱注册，收到验证邮件；用户可以通过手机号注册，收到短信验证码；登录成功后跳转到首页；密码强度验证（至少8位，包含字母和数字） | 用户认证 | 认证,核心功能 | 无 |
| US-002 | 创建任务 | 注册用户 | 能够创建新的任务并设置基本信息 | 我可以记录和管理我需要完成的工作 | P0 | 3 | 用户可以输入任务标题；用户可以设置任务描述；用户可以设置截止日期；创建后任务显示在任务列表中 | 任务管理 | 任务,核心功能 | US-001 |

---

## INVEST 原则说明
- **I**ndependent（独立的）：每个故事应尽可能独立
- **N**egotiable（可协商的）：故事细节可以讨论和调整
- **V**aluable（有价值的）：对用户或客户有价值
- **E**stimable（可估算的）：工作量可以估算
- **S**mall（小的）：故事应该足够小，能在一个迭代内完成
- **T**estable（可测试的）：有明确的验收标准

## 用户故事标准格式
- **角色（Role）**：As a [具体用户角色]
- **功能（Feature）**：I want [具体功能或行为]
- **价值（Benefit）**：So that [业务价值或用户收益]

## 优先级定义
- **P0**：核心功能，必须实现，影响产品基本可用性
- **P1**：重要功能，应该在第一个版本中实现
- **P2**：增强功能，可以在后续版本中实现
- **P3**：锦上添花，低优先级功能

## 故事点估算参考
- **1-3 点**：简单任务，1-2 天完成
- **5 点**：中等复杂度，3-5 天完成
- **8 点**：较复杂，1-2 周完成
- **13 点**：非常复杂，需要分解为更小的故事

## 验收标准要求
每个故事必须包含 3-5 条具体的、可测试的验收标准。

## 注意事项
1. **角色具体化**：避免使用泛泛的"用户"，而是使用具体的角色如"新用户"、"管理员"、"付费用户"等
2. **功能明确**：功能描述应该清晰、具体，避免模糊表述
3. **价值导向**：强调业务价值，而不仅仅是技术实现
4. **合理拆分**：确保故事大小适中，既不过大也不过小
5. **依赖关系**：准确识别故事间的依赖关系
6. **模块划分**：合理划分功能模块，便于团队分工
7. **标签系统**：使用有意义的标签进行分类

---

## 最终要求

**请严格按照上述表格格式输出，确保：**
1. 表格包含所有必需的列
2. 每行代表一个完整的用户故事
3. 验收标准用分号（；）分隔多条
4. 标签用逗号（，）分隔多个
5. 没有依赖时填写"无"
6. 故事编号从 US-001 开始连续编号

**现在，请读取 @.opc-harness/PRD.md 获取 PRD 内容，分析后以 Markdown 表格格式输出用户故事列表，并将结果保存到 @.opc-harness/US.md 文件中。**"#;
            
            fs::write(&agents_md_path, &agents_content).map_err(|e| {
                log::error!("[decompose_user_stories_streaming] Failed to write AGENTS.md: {}", e);
                format!("Failed to write AGENTS.md: {}", e)
            })?;
            log::info!("[decompose_user_stories_streaming] ✅ AGENTS.md written to: {:?}", agents_md_path);
            log::info!("[decompose_user_stories_streaming] 📝 AGENTS.md content length: {} bytes", agents_content.len());
            
            // 构建简短的用户消息，通过 @ 引用文件
            // ⚠️ 注意：移除换行符，避免 cmd.exe /c 解析错误
            format!(
                "请读取 @.opc-harness/AGENTS.md 了解任务规则，读取 @.opc-harness/PRD.md 获取 PRD 内容，然后将拆分的用户故事结果保存到 @.opc-harness/US.md 文件中。"
            )
        } else {
            log::warn!("[decompose_user_stories_streaming] ❌ CodeFree provider requires project_id but got None");
            // 如果没有 project_id，回退到完整提示词
            crate::prompts::user_story_decomposition::generate_user_story_decomposition_prompt(&request.prd_content)
        }
    } else {
        // 非 CodeFree 提供商，使用完整的提示词
        crate::prompts::user_story_decomposition::generate_user_story_decomposition_prompt(&request.prd_content)
    };
    
    // 2. 创建 AI Provider
    let provider_type = match request.provider.as_str() {
        "openai" => crate::ai::AIProviderType::OpenAI,
        "anthropic" => crate::ai::AIProviderType::Anthropic,
        "kimi" => crate::ai::AIProviderType::Kimi,
        "glm" => crate::ai::AIProviderType::GLM,
        "minimax" => crate::ai::AIProviderType::MiniMax,
        "codefree" => crate::ai::AIProviderType::CodeFree,
        _ => return Err(format!("不支持的 AI 提供商：{}", request.provider)),
    };
    
    // 获取 API Key - 优先使用传入的 key，否则从环境变量读取
    let api_key = request.api_key
        .or_else(|| std::env::var("OPENAI_API_KEY").ok())
        .or_else(|| std::env::var("ANTHROPIC_API_KEY").ok())
        .or_else(|| std::env::var("MOONSHOT_API_KEY").ok())
        .or_else(|| std::env::var("ZHIPU_API_KEY").ok())
        .or_else(|| std::env::var("KIMI_API_KEY").ok())
        .or_else(|| std::env::var("GLM_API_KEY").ok())
        .unwrap_or_default(); // 如果都没有，使用空字符串（AI Provider会处理）
    
    let provider = crate::ai::AIProvider::new(provider_type, api_key);
    
    // 3. 构建聊天请求（流式模式）
    let chat_request = crate::ai::ChatRequest {
        model: request.model,
        messages: vec![
            crate::ai::Message {
                role: "user".to_string(),
                content: user_message,
            },
        ],
        temperature: Some(0.7),
        max_tokens: Some(4096),
        stream: true,
        project_id: request.project_id.clone(),
    };
    
    // 4. 创建会话感知的 chunk 处理器
    let session_id_clone = session_id.clone();
    let app_clone = app.clone();
    let provider_clone = request.provider.clone();
    let project_id_clone = request.project_id.clone();
    
    let chunk_handler = move |chunk: String| -> Result<(), crate::ai::AIError> {
        let stream_chunk = crate::ai::StreamChunk {
            session_id: session_id_clone.clone(),
            content: chunk.clone(),
            is_complete: false,
        };
        
        // 发送用户故事流式 chunk 事件
        app_clone
            .emit("user-story-stream-chunk", &stream_chunk)
            .map_err(|e| crate::ai::AIError {
                message: e.to_string(),
            })?;
        
        Ok(())
    };
    
    // 5. 执行流式请求
    match provider.stream_chat(chat_request, chunk_handler).await {
        Ok(final_content) => {
            // 如果是 CodeFree，需要从文件读取最终内容
            let us_content = if provider_clone == "codefree" {
                if let Some(ref pid) = project_id_clone {
                    use crate::utils::paths::get_workspaces_dir;
                    use std::fs;
                    
                    let workspaces_root = get_workspaces_dir();
                    let workspace_path = workspaces_root.join(pid);
                    let context_dir = workspace_path.join(".opc-harness");
                    let us_md_path = context_dir.join("US.md");
                    
                    log::info!("[decompose_user_stories_streaming] 📖 Reading generated user stories from: {:?}", us_md_path);
                    
                    // 等待一下确保文件写入完成
                    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                    
                    // 尝试读取 US.md 文件
                    match fs::read_to_string(&us_md_path) {
                        Ok(content) => {
                            log::info!("[decompose_user_stories_streaming] ✅ Successfully read US.md, length: {} bytes", content.len());
                            content
                        }
                        Err(e) => {
                            log::warn!("[decompose_user_stories_streaming] ⚠️ Failed to read US.md: {}, using streamed content", e);
                            final_content
                        }
                    }
                } else {
                    log::warn!("[decompose_user_stories_streaming] ⚠️ CodeFree completed but no project_id, using streamed content");
                    final_content
                }
            } else {
                // 非 CodeFree 提供商，直接使用流式内容
                final_content
            };
            
            // 发送完成事件（使用从文件读取的内容或流式内容）
            let complete_data = crate::ai::StreamComplete {
                session_id: session_id.clone(),
                content: us_content.clone(),
            };
            let _ = app.emit("user-story-stream-complete", &complete_data);
            
            log::info!("Streaming user story decomposition completed");
            Ok(us_content)
        }
        Err(e) => {
            // 发送错误事件
            let error_data = crate::ai::StreamError {
                session_id: session_id.clone(),
                error: e.to_string(),
            };
            let _ = app.emit("user-story-stream-error", &error_data);
            
            log::error!("Streaming user story decomposition failed: {}", e);
            Err(e.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_streaming_module_structure() {
        // 简单的结构测试，确保模块可以正常导入
        assert!(true);
    }
}
