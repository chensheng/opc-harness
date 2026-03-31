use crate::ai::{
    AIProvider, AIProviderType, ChatRequest, ChatResponse, Message as AIMessage, StreamChunk, StreamComplete, StreamError,
};
use crate::prompts::{prd_template, user_persona, competitor_analysis};
use crate::utils::keychain;
use serde::{Deserialize, Serialize};
use tauri::Emitter;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidateKeyRequest {
    pub provider: String,
    pub api_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveApiKeyRequest {
    pub provider: String,
    pub model: String,
    pub api_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetApiKeyRequest {
    pub provider: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteApiKeyRequest {
    pub provider: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatRequestPayload {
    pub provider: String,
    pub model: String,
    pub api_key: String,
    pub messages: Vec<Message>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneratePRDRequest {
    pub idea: String,
    pub provider: String,
    pub model: String,
    pub api_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PRDResponse {
    pub title: String,
    pub overview: String,
    pub target_users: Vec<String>,
    pub core_features: Vec<String>,
    pub tech_stack: Vec<String>,
    pub estimated_effort: String,
    pub business_model: Option<String>,
    pub pricing: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPersonaResponse {
    pub id: String,
    pub name: String,
    pub age: String,
    pub occupation: String,
    pub background: String,
    pub goals: Vec<String>,
    pub pain_points: Vec<String>,
    pub behaviors: Vec<String>,
    pub quote: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompetitorResponse {
    pub name: String,
    pub strengths: Vec<String>,
    pub weaknesses: Vec<String>,
    pub market_share: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompetitorAnalysisResponse {
    pub competitors: Vec<CompetitorResponse>,
    pub differentiation: String,
    pub opportunities: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketingChannelResponse {
    pub name: String,
    pub platform: String,
    pub priority: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketingTimelineItem {
    pub phase: String,
    pub duration: String,
    pub activities: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketingStrategyResponse {
    pub channels: Vec<MarketingChannelResponse>,
    pub timeline: Vec<MarketingTimelineItem>,
    pub key_messages: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketingCopyResponse {
    pub platform: String,
    pub content: String,
    pub hashtags: Option<Vec<String>>,
}

#[tauri::command]
pub async fn validate_ai_key(request: ValidateKeyRequest) -> Result<bool, String> {
    let provider_type = match request.provider.as_str() {
        "openai" => AIProviderType::OpenAI,
        "anthropic" => AIProviderType::Anthropic,
        "kimi" => AIProviderType::Kimi,
        "glm" => AIProviderType::GLM,
        _ => return Err("Unsupported provider".to_string()),
    };

    let provider = AIProvider::new(provider_type, request.api_key);
    provider.validate_key().await.map_err(|e| e.to_string())
}

/// Save API key securely to OS keychain
#[tauri::command]
pub fn save_api_key_to_keychain(request: SaveApiKeyRequest) -> Result<bool, String> {
    // Validate inputs
    if request.provider.is_empty() {
        return Err("Provider name cannot be empty".to_string());
    }

    if request.model.is_empty() {
        return Err("Model name cannot be empty".to_string());
    }

    if request.api_key.is_empty() {
        return Err("API key cannot be empty".to_string());
    }

    // Save to OS keychain
    keychain::save_api_key(&request.provider, &request.api_key)
        .map_err(|e| format!("Failed to save API key: {}", e))?;

    Ok(true)
}

/// Retrieve API key from OS keychain
#[tauri::command]
pub fn get_api_key_from_keychain(request: GetApiKeyRequest) -> Result<String, String> {
    keychain::get_api_key(&request.provider)
        .map_err(|e| format!("Failed to retrieve API key: {}", e))
}

/// Check if API key exists in OS keychain
#[tauri::command]
pub fn has_api_key_in_keychain(provider: String) -> Result<bool, String> {
    Ok(keychain::has_api_key(&provider))
}

/// Delete API key from OS keychain
#[tauri::command]
pub fn delete_api_key_from_keychain(request: DeleteApiKeyRequest) -> Result<(), String> {
    keychain::delete_api_key(&request.provider)
        .map_err(|e| format!("Failed to delete API key: {}", e))
}

#[tauri::command]
pub async fn chat(request: ChatRequestPayload) -> Result<String, String> {
    let provider_type = match request.provider.as_str() {
        "openai" => AIProviderType::OpenAI,
        "anthropic" => AIProviderType::Anthropic,
        "kimi" => AIProviderType::Kimi,
        "glm" => AIProviderType::GLM,
        _ => return Err("Unsupported provider".to_string()),
    };

    let provider = AIProvider::new(provider_type, request.api_key);

    let messages: Vec<crate::ai::Message> = request
        .messages
        .into_iter()
        .map(|m| crate::ai::Message {
            role: m.role,
            content: m.content,
        })
        .collect();

    let chat_request = ChatRequest {
        model: request.model,
        messages,
        temperature: request.temperature,
        max_tokens: request.max_tokens,
        stream: false,
    };

    let response = provider
        .chat(chat_request)
        .await
        .map_err(|e| e.to_string())?;
    Ok(response.content)
}

#[tauri::command]
pub async fn stream_chat(
    request: ChatRequestPayload,
    app: tauri::AppHandle,
) -> Result<String, String> {
    let session_id = Uuid::new_v4().to_string();
    let provider_type = match request.provider.as_str() {
        "openai" => AIProviderType::OpenAI,
        "anthropic" => AIProviderType::Anthropic,
        "kimi" => AIProviderType::Kimi,
        "kimi-code" => AIProviderType::KimiCode,
        "glm" => AIProviderType::GLM,
        _ => return Err("Unsupported provider".to_string()),
    };

    let provider = AIProvider::new(provider_type, request.api_key.clone());

    let messages: Vec<crate::ai::Message> = request
        .messages
        .into_iter()
        .map(|m| crate::ai::Message {
            role: m.role,
            content: m.content,
        })
        .collect();

    let chat_request = ChatRequest {
        model: request.model,
        messages,
        temperature: request.temperature,
        max_tokens: request.max_tokens,
        stream: true,
    };

    // 创建会话感知的 chunk 处理器
    let session_id_clone = session_id.clone();
    let app_clone = app.clone();
    let chunk_handler = move |chunk: String| -> Result<(), crate::ai::AIError> {
        let stream_chunk = StreamChunk {
            session_id: session_id_clone.clone(),
            content: chunk,
            is_complete: false,
        };

        app_clone
            .emit("ai-stream-chunk", stream_chunk)
            .map_err(|e| crate::ai::AIError {
                message: e.to_string(),
            })?;

        Ok(())
    };

    // 执行流式请求
    match provider.stream_chat(chat_request, chunk_handler).await {
        Ok(final_content) => {
            // 发送完成事件
            let complete_data = StreamComplete {
                session_id: session_id.clone(),
                content: final_content.clone(),
            };
            let _ = app.emit("ai-stream-complete", complete_data);

            Ok(final_content)
        }
        Err(e) => {
            // 发送错误事件
            let error_data = StreamError {
                session_id: session_id.clone(),
                error: e.to_string(),
            };
            let _ = app.emit("ai-stream-error", error_data);

            Err(e.to_string())
        }
    }
}

#[tauri::command]
pub async fn generate_prd(request: GeneratePRDRequest) -> Result<PRDResponse, String> {
    log::info!("Generating PRD for idea: {}", request.idea);
    
    // 1. 构建 PRD 提示词
    let prompt = prd_template::generate_prd_prompt(&request.idea, None);
    
    // 2. 创建 AI Provider
    let provider = match request.provider.as_str() {
        "openai" => AIProvider::new(AIProviderType::OpenAI, request.api_key),
        "anthropic" => AIProvider::new(AIProviderType::Anthropic, request.api_key),
        "kimi" => AIProvider::new(AIProviderType::Kimi, request.api_key),
        "glm" => AIProvider::new(AIProviderType::GLM, request.api_key),
        "minimax" => AIProvider::new(AIProviderType::MiniMax, request.api_key),
        _ => {
            return Err(format!("不支持的 AI 提供商：{}", request.provider));
        }
    };
    
    // 3. 构建聊天请求
    let chat_request = ChatRequest {
        model: request.model,
        messages: vec![AIMessage {
            role: "user".to_string(),
            content: prompt,
        }],
        temperature: Some(0.7),
        max_tokens: Some(4096), // PRD 通常较长
        stream: false,
    };
    
    // 4. 调用 AI Provider
    let response = provider.chat(chat_request)
        .await
        .map_err(|e| format!("AI 调用失败：{}", e))?;
    
    // 5. 解析 AI 生成的 PRD 内容
    // AI 返回的是 Markdown 格式的 PRD，需要解析为结构化数据
    let prd = parse_prd_from_markdown(&response.content)
        .map_err(|e| format!("PRD 解析失败：{}", e))?;
    
    log::info!("PRD generated successfully: {}", prd.title);
    
    Ok(prd)
}

/// 流式生成 PRD（打字机效果）
#[tauri::command]
pub async fn stream_generate_prd(
    request: GeneratePRDRequest,
    app: tauri::AppHandle,
) -> Result<String, String> {
    let session_id = Uuid::new_v4().to_string();
    
    log::info!("Starting streaming PRD generation for idea: {}", request.idea);
    
    // 1. 构建 PRD 提示词
    let prompt = prd_template::generate_prd_prompt(&request.idea, None);
    
    // 2. 创建 AI Provider
    let provider_type = match request.provider.as_str() {
        "openai" => AIProviderType::OpenAI,
        "anthropic" => AIProviderType::Anthropic,
        "kimi" => AIProviderType::Kimi,
        "glm" => AIProviderType::GLM,
        "minimax" => AIProviderType::MiniMax,
        _ => return Err(format!("不支持的 AI 提供商：{}", request.provider)),
    };
    
    let provider = AIProvider::new(provider_type, request.api_key.clone());
    
    // 3. 构建聊天请求（流式模式）
    let chat_request = ChatRequest {
        model: request.model,
        messages: vec![AIMessage {
            role: "user".to_string(),
            content: prompt,
        }],
        temperature: Some(0.7),
        max_tokens: Some(4096),
        stream: true,
    };
    
    // 4. 创建会话感知的 chunk 处理器
    let session_id_clone = session_id.clone();
    let app_clone = app.clone();
    let mut full_content = String::new();
    
    let chunk_handler = move |chunk: String| -> Result<(), crate::ai::AIError> {
        let stream_chunk = StreamChunk {
            session_id: session_id_clone.clone(),
            content: chunk.clone(),
            is_complete: false,
        };
        
        // 发送 PRD 流式 chunk 事件
        app_clone
            .emit("prd-stream-chunk", stream_chunk)
            .map_err(|e| crate::ai::AIError {
                message: e.to_string(),
            })?;
        
        Ok(())
    };
    
    // 5. 执行流式请求
    match provider.stream_chat(chat_request, chunk_handler).await {
        Ok(final_content) => {
            // 发送完成事件
            let complete_data = StreamComplete {
                session_id: session_id.clone(),
                content: final_content.clone(),
            };
            let _ = app.emit("prd-stream-complete", complete_data);
            
            log::info!("Streaming PRD generation completed");
            Ok(final_content)
        }
        Err(e) => {
            // 发送错误事件
            let error_data = StreamError {
                session_id: session_id.clone(),
                error: e.to_string(),
            };
            let _ = app.emit("prd-stream-error", error_data);
            
            log::error!("Streaming PRD generation failed: {}", e);
            Err(e.to_string())
        }
    }
}

/// 从 Markdown 内容解析 PRD 结构
/// 
/// 这个函数使用简单的规则提取 PRD 的各个部分
/// 在生产环境中，可以使用更复杂的 NLP 技术或让 AI 直接返回 JSON
fn parse_prd_from_markdown(content: &str) -> Result<PRDResponse, String> {
    // 提取标题（第一个 # 标题）
    let title = extract_first_heading(content)
        .unwrap_or_else(|| "Generated Product".to_string());
    
    // 提取产品概述（## 1. 产品概述 下的内容）
    let overview = extract_section(content, "产品概述")
        .unwrap_or_else(|| "AI-generated product overview.".to_string());
    
    // 提取目标用户
    let target_users = extract_list_items(content, "目标用户")
        .unwrap_or_else(|| vec!["Target users to be defined".to_string()]);
    
    // 提取核心功能
    let core_features = extract_list_items(content, "核心功能")
        .unwrap_or_else(|| vec!["Core features to be defined".to_string()]);
    
    // 提取技术栈
    let tech_stack = extract_list_items(content, "技术栈")
        .unwrap_or_else(|| vec!["Technology stack to be defined".to_string()]);
    
    // 估算开发工作量
    let estimated_effort = extract_section(content, "时间估算")
        .or_else(|| extract_section(content, "开发计划"))
        .unwrap_or_else(|| "To be estimated".to_string());
    
    // 提取商业模式
    let business_model = extract_section(content, "收入模式")
        .or_else(|| extract_section(content, "商业模式"));
    
    // 提取定价策略
    let pricing = extract_section(content, "定价策略");
    
    Ok(PRDResponse {
        title,
        overview,
        target_users,
        core_features,
        tech_stack,
        estimated_effort,
        business_model,
        pricing,
    })
}

/// 提取第一个 H1 标题
fn extract_first_heading(content: &str) -> Option<String> {
    for line in content.lines() {
        if line.trim().starts_with("# ") {
            return Some(line.trim_start_matches('#').trim().to_string());
        }
    }
    None
}

/// 提取指定章节的内容
fn extract_section(content: &str, section_name: &str) -> Option<String> {
    let lines: Vec<&str> = content.lines().collect();
    let mut in_section = false;
    let mut section_content = Vec::new();
    
    for line in lines {
        let trimmed = line.trim();
        
        // 检查是否进入目标章节
        if trimmed.starts_with("## ") && trimmed.contains(section_name) {
            in_section = true;
            continue;
        }
        
        // 检查是否进入下一个章节（退出当前章节）
        if in_section && trimmed.starts_with("## ") {
            break;
        }
        
        // 收集章节内容
        if in_section && !trimmed.is_empty() && !trimmed.starts_with("### ") {
            section_content.push(trimmed);
        }
    }
    
    if section_content.is_empty() {
        None
    } else {
        Some(section_content.join("\n"))
    }
}

/// 提取列表项
fn extract_list_items(content: &str, list_context: &str) -> Option<Vec<String>> {
    let lines: Vec<&str> = content.lines().collect();
    let mut items = Vec::new();
    let mut in_target_list = false;
    
    for line in lines {
        let trimmed = line.trim();
        
        // 查找包含目标上下文的列表
        if trimmed.contains(list_context) {
            in_target_list = true;
            continue;
        }
        
        // 收集列表项
        if in_target_list {
            if trimmed.starts_with("- ") || trimmed.starts_with("* ") || trimmed.starts_with("+ ") {
                let item = trimmed[2..].trim().to_string();
                // 移除可能的嵌套标记
                let item = item.split('|').next().unwrap_or(&item).trim().to_string();
                if !item.is_empty() {
                    items.push(item);
                }
            } else if trimmed.starts_with("## ") || (trimmed.starts_with("### ") && !items.is_empty()) {
                // 到达下一个章节或子章节，停止收集
                break;
            } else if !trimmed.is_empty() && !items.is_empty() {
                // 可能是列表项的延续
                items.last_mut().map(|last| last.push_str(&format!(" {}", trimmed)));
            }
        }
    }
    
    if items.is_empty() {
        None
    } else {
        Some(items)
    }
}

#[tauri::command]
pub async fn generate_user_personas(
    request: GeneratePRDRequest,
) -> Result<Vec<UserPersonaResponse>, String> {
    log::info!("Generating user personas for idea: {}", request.idea);
    
    // 1. 构建产品信息
    let product_info = format!("基于以下产品想法生成用户画像：{}", request.idea);
    
    // 2. 根据 AI Provider 选择优化的提示词
    let prompt = match request.provider.as_str() {
        "minimax" => user_persona::generate_user_persona_prompt_minimax(&product_info),
        "glm" => user_persona::generate_user_persona_prompt_glm(&product_info),
        _ => user_persona::generate_user_persona_prompt(&product_info),
    };
    
    // 3. 创建 AI Provider
    let provider = match request.provider.as_str() {
        "openai" => AIProvider::new(AIProviderType::OpenAI, request.api_key),
        "anthropic" => AIProvider::new(AIProviderType::Anthropic, request.api_key),
        "kimi" => AIProvider::new(AIProviderType::Kimi, request.api_key),
        "glm" => AIProvider::new(AIProviderType::GLM, request.api_key),
        "minimax" => AIProvider::new(AIProviderType::MiniMax, request.api_key),
        _ => {
            return Err(format!("不支持的 AI 提供商：{}", request.provider));
        }
    };
    
    // 4. 构建聊天请求
    let chat_request = ChatRequest {
        model: request.model,
        messages: vec![AIMessage {
            role: "user".to_string(),
            content: prompt,
        }],
        temperature: Some(0.8), // 稍微提高温度增加创造性
        max_tokens: Some(4096), // 用户画像需要较长文本
        stream: false,
    };
    
    // 5. 调用 AI Provider
    let response = provider.chat(chat_request)
        .await
        .map_err(|e| format!("AI 调用失败：{}", e))?;
    
    // 6. 解析 AI 生成的用户画像
    let personas = parse_user_personas_from_markdown(&response.content)
        .map_err(|e| format!("用户画像解析失败：{}", e))?;
    
    log::info!("User personas generated successfully: {} personas", personas.len());
    
    Ok(personas)
}

/// 从 Markdown 文本中解析用户画像
fn parse_user_personas_from_markdown(markdown: &str) -> Result<Vec<UserPersonaResponse>, String> {
    // 简化的解析逻辑，实际应该使用更复杂的 Markdown 解析器
    let mut personas = Vec::new();
    
    // 按行分割并提取信息
    let lines: Vec<&str> = markdown.lines().collect();
    let mut current_persona: Option<UserPersonaResponse> = None;
    let mut id_counter = 1;
    
    for line in lines {
        let trimmed = line.trim();
        
        // 检测新的画像开始（通常以 # 或数字开头）
        if trimmed.starts_with('#') || (trimmed.chars().next().map_or(false, |c| c.is_ascii_digit()) && trimmed.contains('.')) {
            // 保存之前的画像
            if let Some(persona) = current_persona.take() {
                personas.push(persona);
            }
            
            // 创建新画像
            current_persona = Some(UserPersonaResponse {
                id: id_counter.to_string(),
                name: extract_name_from_line(trimmed).unwrap_or_else(|| format!("用户{}", id_counter)),
                age: "".to_string(),
                occupation: "".to_string(),
                background: "".to_string(),
                goals: Vec::new(),
                pain_points: Vec::new(),
                behaviors: Vec::new(),
                quote: None,
            });
            id_counter += 1;
        } else if let Some(ref mut persona) = current_persona {
            // 提取具体字段
            if trimmed.contains("年龄") && trimmed.contains(':') {
                persona.age = extract_value_after_colon(trimmed);
            } else if trimmed.contains("职业") && trimmed.contains(':') {
                persona.occupation = extract_value_after_colon(trimmed);
            } else if trimmed.contains("背景") && trimmed.contains(':') {
                persona.background = extract_value_after_colon(trimmed);
            } else if trimmed.contains("目标") && trimmed.contains(':') {
                persona.goals.push(extract_value_after_colon(trimmed));
            } else if trimmed.contains("痛点") && trimmed.contains(':') {
                persona.pain_points.push(extract_value_after_colon(trimmed));
            } else if trimmed.contains("行为") && trimmed.contains(':') {
                persona.behaviors.push(extract_value_after_colon(trimmed));
            } else if trimmed.starts_with('"') || trimmed.starts_with('"') {
                // 提取引言
                let quote = trimmed.trim_matches('"').trim_matches('"').to_string();
                if !quote.is_empty() {
                    persona.quote = Some(quote);
                }
            }
        }
    }
    
    // 添加最后一个画像
    if let Some(persona) = current_persona {
        personas.push(persona);
    }
    
    // 如果没有解析出任何画像，尝试创建一个默认的
    if personas.is_empty() {
        personas.push(UserPersonaResponse {
            id: "1".to_string(),
            name: "典型用户".to_string(),
            age: "25-35 岁".to_string(),
            occupation: "专业人士".to_string(),
            background: markdown.lines().take(3).collect::<Vec<_>>().join("\n"),
            goals: vec!["解决核心问题".to_string()],
            pain_points: vec!["当前解决方案不足".to_string()],
            behaviors: vec!["积极寻找更好的工具".to_string()],
            quote: Some("我需要一个更好的解决方案".to_string()),
        });
    }
    
    Ok(personas)
}

/// 从行中提取名字（简化版本）
fn extract_name_from_line(line: &str) -> Option<String> {
    // 尝试提取中文名字（通常 2-3 个字符）
    if let Some(start) = line.find(|c: char| c.is_ascii_alphabetic() || c.is_whitespace()) {
        let name_part = &line[..start];
        let name = name_part.trim().trim_start_matches(|c: char| !c.is_alphabetic() && !c.is_whitespace());
        if !name.is_empty() && name.len() <= 10 {
            return Some(name.to_string());
        }
    }
    None
}

/// 提取冒号后的值
fn extract_value_after_colon(line: &str) -> String {
    if let Some(pos) = line.find(':') {
        line[pos + 1..].trim().trim_end_matches(',').to_string()
    } else {
        line.to_string()
    }
}

#[tauri::command]
pub async fn generate_competitor_analysis(
    request: GeneratePRDRequest,
) -> Result<CompetitorAnalysisResponse, String> {
    log::info!("Generating competitor analysis for idea: {}", request.idea);
    
    // 1. 构建产品信息
    let product_info = format!("基于以下产品想法进行竞品分析：{}", request.idea);
    
    // 2. 根据 AI Provider 选择优化的提示词
    let prompt = match request.provider.as_str() {
        "minimax" => competitor_analysis::generate_competitor_analysis_prompt_minimax(&product_info),
        "glm" => competitor_analysis::generate_competitor_analysis_prompt_glm(&product_info),
        _ => competitor_analysis::generate_competitor_analysis_prompt(&product_info),
    };
    
    // 3. 创建 AI Provider
    let provider = match request.provider.as_str() {
        "openai" => AIProvider::new(AIProviderType::OpenAI, request.api_key),
        "anthropic" => AIProvider::new(AIProviderType::Anthropic, request.api_key),
        "kimi" => AIProvider::new(AIProviderType::Kimi, request.api_key),
        "glm" => AIProvider::new(AIProviderType::GLM, request.api_key),
        "minimax" => AIProvider::new(AIProviderType::MiniMax, request.api_key),
        _ => {
            return Err(format!("不支持的 AI 提供商：{}", request.provider));
        }
    };
    
    // 4. 构建聊天请求
    let chat_request = ChatRequest {
        model: request.model,
        messages: vec![AIMessage {
            role: "user".to_string(),
            content: prompt,
        }],
        temperature: Some(0.7),
        max_tokens: Some(6000),
        stream: false,
    };
    
    // 5. 调用 AI Provider
    let response = provider.chat(chat_request)
        .await
        .map_err(|e| format!("AI 调用失败：{}", e))?;
    
    // 6. 使用已有的解析函数（在下方定义）
    let analysis = parse_competitor_analysis_from_markdown(&response.content)
        .map_err(|e| format!("竞品分析解析失败：{}", e))?;
    
    log::info!("Competitor analysis generated successfully: {} competitors", analysis.competitors.len());
    
    Ok(analysis)
}

#[tauri::command]
pub async fn generate_marketing_strategy(
    _request: GeneratePRDRequest,
) -> Result<MarketingStrategyResponse, String> {
    // TODO: Implement actual marketing strategy generation
    Ok(MarketingStrategyResponse {
        channels: vec![MarketingChannelResponse {
            name: "Product Hunt".to_string(),
            platform: "producthunt".to_string(),
            priority: "high".to_string(),
            description: "Great for tech product launches".to_string(),
        }],
        timeline: vec![MarketingTimelineItem {
            phase: "Pre-launch".to_string(),
            duration: "1 week".to_string(),
            activities: vec!["Create landing page".to_string()],
        }],
        key_messages: vec!["Value proposition 1".to_string()],
    })
}

#[tauri::command]
pub async fn generate_marketing_copy(
    _request: GeneratePRDRequest,
) -> Result<Vec<MarketingCopyResponse>, String> {
    // TODO: Implement actual marketing copy generation
    Ok(vec![MarketingCopyResponse {
        platform: "twitter".to_string(),
        content: "🚀 New product launch!".to_string(),
        hashtags: Some(vec!["BuildInPublic".to_string()]),
    }])
}

// ============================================================================
// Claude API 专用 Commands
// ============================================================================

/// Claude 聊天命令（非流式）
#[tauri::command]
pub async fn chat_claude(request: ChatRequestPayload) -> Result<ChatResponse, String> {
    log::info!("Sending chat request to Claude: {:?}", request);
    
    // 创建 AI Provider
    let provider = AIProvider::new(AIProviderType::Anthropic, request.api_key);
    
    // 构建聊天请求
    let chat_request = ChatRequest {
        model: request.model,
        messages: request.messages.into_iter().map(|msg| AIMessage {
            role: msg.role,
            content: msg.content,
        }).collect(),
        temperature: request.temperature,
        max_tokens: request.max_tokens,
        stream: false,
    };
    
    // 调用 AI Provider
    let response = provider.chat(chat_request)
        .await
        .map_err(|e| format!("Claude 调用失败：{}", e))?;
    
    log::info!("Claude chat response received: {} chars", response.content.len());
    
    Ok(response)
}

/// Claude 聊天命令（流式）
#[tauri::command]
pub async fn stream_chat_claude(
    app: tauri::AppHandle,
    request: ChatRequestPayload,
) -> Result<String, String> {
    log::info!("Sending streaming chat request to Claude: {:?}", request);
    
    // 生成会话 ID
    let session_id = Uuid::new_v4().to_string();
    
    // 创建 AI Provider
    let provider = AIProvider::new(AIProviderType::Anthropic, request.api_key);
    
    // 构建聊天请求
    let chat_request = ChatRequest {
        model: request.model,
        messages: request.messages.into_iter().map(|msg| AIMessage {
            role: msg.role,
            content: msg.content,
        }).collect(),
        temperature: request.temperature,
        max_tokens: request.max_tokens,
        stream: true,
    };
    
    // 克隆 session_id 用于闭包
    let session_id_clone = session_id.clone();
    // 克隆 app handle 用于闭包
    let app_handle_clone = app.clone();
    
    // 定义 chunk 处理回调
    let on_chunk = move |content: String| -> Result<(), crate::ai::AIError> {
        let chunk = StreamChunk {
            session_id: session_id_clone.clone(),
            content,
            is_complete: false,
        };
        
        // 发送事件到前端
        app_handle_clone.emit("ai-stream-chunk", chunk)
            .map_err(|e| crate::ai::AIError { 
                message: format!("Failed to emit chunk: {}", e) 
            })?;
        
        Ok(())
    };
    
    // 调用流式聊天
    match provider.stream_chat(chat_request, on_chunk).await {
        Ok(final_content) => {
            // 发送完成事件
            let complete_data = StreamComplete {
                session_id: session_id.clone(),
                content: final_content.clone(),
            };
            let _ = app.emit("ai-stream-complete", complete_data);
            
            log::info!("Claude streaming chat completed: {} chars", final_content.len());
            Ok(final_content)
        }
        Err(e) => {
            // 发送错误事件
            let error_data = StreamError {
                session_id: session_id.clone(),
                error: e.to_string(),
            };
            let _ = app.emit("ai-stream-error", error_data);
            
            Err(e.to_string())
        }
    }
}

/// 使用 Claude 生成用户画像
#[tauri::command]
pub async fn generate_personas_claude(
    request: GeneratePRDRequest,
) -> Result<UserPersonaResponse, String> {
    log::info!("Generating user personas with Claude for idea: {}", request.idea);
    
    // 1. 构建用户画像提示词
    let prompt = user_persona::generate_user_persona_prompt(&request.idea);
    
    // 2. 创建 AI Provider
    let provider = AIProvider::new(AIProviderType::Anthropic, request.api_key);
    
    // 3. 构建聊天请求
    let chat_request = ChatRequest {
        model: request.model,
        messages: vec![AIMessage {
            role: "user".to_string(),
            content: prompt,
        }],
        temperature: Some(0.7),
        max_tokens: Some(2048),
        stream: false,
    };
    
    // 4. 调用 AI Provider
    let response = provider.chat(chat_request)
        .await
        .map_err(|e| format!("Claude 调用失败：{}", e))?;
    
    // 5. 解析 AI 生成的用户画像
    let persona = parse_user_persona_from_markdown(&response.content)
        .map_err(|e| format!("用户画像解析失败：{}", e))?;
    
    log::info!("User persona generated successfully: {}", persona.name);
    
    Ok(persona)
}

/// 使用 Claude 生成竞品分析
#[tauri::command]
pub async fn generate_competitor_analysis_claude(
    request: GeneratePRDRequest,
) -> Result<CompetitorAnalysisResponse, String> {
    log::info!("Generating competitor analysis with Claude for idea: {}", request.idea);
    
    // 1. 构建竞品分析提示词（这里简化，实际应该有专门的模板）
    let prompt = format!(r#"请为以下产品想法进行详细的竞品分析：

{}

请分析：
1. 主要竞争对手（至少 3 个）
2. 每个竞争对手的优势和劣势
3. 市场份额或用户规模
4. 差异化机会
5. 市场空白点

请以结构化的方式呈现分析结果。"#, request.idea);
    
    // 2. 创建 AI Provider
    let provider = AIProvider::new(AIProviderType::Anthropic, request.api_key);
    
    // 3. 构建聊天请求
    let chat_request = ChatRequest {
        model: request.model,
        messages: vec![AIMessage {
            role: "user".to_string(),
            content: prompt,
        }],
        temperature: Some(0.7),
        max_tokens: Some(4096),
        stream: false,
    };
    
    // 4. 调用 AI Provider
    let response = provider.chat(chat_request)
        .await
        .map_err(|e| format!("Claude 调用失败：{}", e))?;
    
    // 5. 解析 AI 生成的竞品分析
    let analysis = parse_competitor_analysis_from_markdown(&response.content)
        .map_err(|e| format!("竞品分析解析失败：{}", e))?;
    
    log::info!("Competitor analysis generated successfully");
    
    Ok(analysis)
}

// ============================================================================
// 辅助函数：解析用户画像
// ============================================================================

fn parse_user_persona_from_markdown(content: &str) -> Result<UserPersonaResponse, String> {
    // 提取姓名（第一个 ## 标题或第一行）
    let name = extract_first_heading(content)
        .unwrap_or_else(|| "典型用户".to_string());
    
    // 生成唯一 ID
    let id = Uuid::new_v4().to_string();
    
    // 提取基本信息（年龄、职业等）
    let age = extract_section(content, "年龄").unwrap_or_else(|| "25-35 岁".to_string());
    let occupation = extract_section(content, "职业").unwrap_or_else(|| "专业人士".to_string());
    let background = extract_section(content, "背景").unwrap_or_else(|| "具有相关专业背景".to_string());
    
    // 提取目标
    let goals = extract_list_items(content, "目标")
        .or_else(|| extract_list_items(content, "Goals"))
        .unwrap_or_else(|| vec!["提高工作效率".to_string()]);
    
    // 提取痛点
    let pain_points = extract_list_items(content, "痛点")
        .or_else(|| extract_list_items(content, "Pain Points"))
        .unwrap_or_else(|| vec!["时间不够用".to_string()]);
    
    // 提取行为特征
    let behaviors = extract_list_items(content, "行为")
        .or_else(|| extract_list_items(content, "Behaviors"))
        .unwrap_or_else(|| vec!["经常使用数字化工具".to_string()]);
    
    // 提取引言
    let quote = extract_section(content, "引言")
        .or_else(|| extract_section(content, "Quote"));
    
    Ok(UserPersonaResponse {
        id,
        name,
        age,
        occupation,
        background,
        goals,
        pain_points,
        behaviors,
        quote,
    })
}

// ============================================================================
// 辅助函数：解析竞品分析
// ============================================================================

fn parse_competitor_analysis_from_markdown(content: &str) -> Result<CompetitorAnalysisResponse, String> {
    // 这个函数解析 Markdown 格式的竞品分析
    // 简化实现，实际应该更复杂
    
    let mut competitors = Vec::new();
    
    // 查找所有提到的竞争对手
    // 这里使用简单的规则：包含"竞争"、"对手"、"竞品"的行
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
            // 尝试提取竞争对手名称
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() > 1 {
                let name = parts[1].trim_start_matches('-').trim_start_matches('*').to_string();
                competitors.push(CompetitorResponse {
                    name,
                    strengths: vec!["优势待分析".to_string()],
                    weaknesses: vec!["劣势待分析".to_string()],
                    market_share: None,
                });
            }
        }
    }
    
    // 如果没找到，使用默认值
    if competitors.is_empty() {
        competitors = vec![
            CompetitorResponse {
                name: "竞争对手 A".to_string(),
                strengths: vec!["市场先行者".to_string()],
                weaknesses: vec!["创新缓慢".to_string()],
                market_share: Some("30%".to_string()),
            },
            CompetitorResponse {
                name: "竞争对手 B".to_string(),
                strengths: vec!["资金充足".to_string()],
                weaknesses: vec!["用户体验差".to_string()],
                market_share: Some("25%".to_string()),
            },
        ];
    }
    
    // 提取差异化策略
    let differentiation = extract_section(content, "差异化")
        .or_else(|| extract_section(content, "Differentiation"))
        .unwrap_or_else(|| "通过创新和更好的用户体验脱颖而出".to_string());
    
    // 提取机会点
    let opportunities = extract_list_items(content, "机会")
        .or_else(|| extract_list_items(content, "Opportunities"))
        .unwrap_or_else(|| vec!["市场空白点待开发".to_string()]);
    
    Ok(CompetitorAnalysisResponse {
        competitors,
        differentiation,
        opportunities,
    })
}

// ============================================================================
// Kimi API 专用 Commands (AI-003)
// ============================================================================

/// Kimi 聊天命令（非流式）
#[tauri::command]
pub async fn chat_kimi(request: ChatRequestPayload) -> Result<ChatResponse, String> {
    log::info!("Sending chat request to Kimi: {:?}", request);
    
    // 创建 AI Provider
    let provider = AIProvider::new(AIProviderType::Kimi, request.api_key);
    
    // 构建聊天请求
    let chat_request = ChatRequest {
        model: request.model,
        messages: request.messages.into_iter().map(|msg| AIMessage {
            role: msg.role,
            content: msg.content,
        }).collect(),
        temperature: request.temperature,
        max_tokens: request.max_tokens,
        stream: false,
    };
    
    // 调用 AI Provider (Kimi uses OpenAI-compatible API)
    let response = provider.chat(chat_request)
        .await
        .map_err(|e| format!("Kimi 调用失败：{}", e))?;
    
    log::info!("Kimi chat response received: {} chars", response.content.len());
    
    Ok(response)
}

/// Kimi 聊天命令（流式）
#[tauri::command]
pub async fn stream_chat_kimi(
    app: tauri::AppHandle,
    request: ChatRequestPayload,
) -> Result<String, String> {
    log::info!("Sending streaming chat request to Kimi: {:?}", request);
    
    // 生成会话 ID
    let session_id = Uuid::new_v4().to_string();
    
    // 创建 AI Provider
    let provider = AIProvider::new(AIProviderType::Kimi, request.api_key);
    
    // 构建聊天请求
    let chat_request = ChatRequest {
        model: request.model,
        messages: request.messages.into_iter().map(|msg| AIMessage {
            role: msg.role,
            content: msg.content,
        }).collect(),
        temperature: request.temperature,
        max_tokens: request.max_tokens,
        stream: true,
    };
    
    // 克隆 session_id 和 app handle 用于闭包
    let session_id_clone = session_id.clone();
    let app_handle_clone = app.clone();
    
    // 定义 chunk 处理回调
    let on_chunk = move |content: String| -> Result<(), crate::ai::AIError> {
        let chunk = StreamChunk {
            session_id: session_id_clone.clone(),
            content,
            is_complete: false,
        };
        
        // 发送事件到前端
        app_handle_clone.emit("ai-stream-chunk", chunk)
            .map_err(|e| crate::ai::AIError { 
                message: format!("Failed to emit chunk: {}", e) 
            })?;
        
        Ok(())
    };
    
    // 调用流式聊天
    match provider.stream_chat(chat_request, on_chunk).await {
        Ok(final_content) => {
            // 发送完成事件
            let complete_data = StreamComplete {
                session_id: session_id.clone(),
                content: final_content.clone(),
            };
            let _ = app.emit("ai-stream-complete", complete_data);
            
            log::info!("Kimi streaming chat completed: {} chars", final_content.len());
            Ok(final_content)
        }
        Err(e) => {
            // 发送错误事件
            let error_data = StreamError {
                session_id: session_id.clone(),
                error: e.to_string(),
            };
            let _ = app.emit("ai-stream-error", error_data);
            
            Err(e.to_string())
        }
    }
}

/// 使用 Kimi 生成用户画像（本地化优化）
#[tauri::command]
pub async fn generate_personas_kimi(
    request: GeneratePRDRequest,
) -> Result<UserPersonaResponse, String> {
    log::info!("Generating user personas with Kimi for idea: {}", request.idea);
    
    // 1. 构建用户画像提示词（使用中文优化版本）
    let prompt = user_persona::generate_user_persona_prompt(&request.idea);
    
    // 2. 创建 AI Provider
    let provider = AIProvider::new(AIProviderType::Kimi, request.api_key);
    
    // 3. 构建聊天请求
    let chat_request = ChatRequest {
        model: request.model,
        messages: vec![AIMessage {
            role: "user".to_string(),
            content: prompt,
        }],
        temperature: Some(0.7),
        max_tokens: Some(2048),
        stream: false,
    };
    
    // 4. 调用 AI Provider
    let response = provider.chat(chat_request)
        .await
        .map_err(|e| format!("Kimi 调用失败：{}", e))?;
    
    // 5. 解析 AI 生成的用户画像
    let persona = parse_user_persona_from_markdown(&response.content)
        .map_err(|e| format!("用户画像解析失败：{}", e))?;
    
    log::info!("User persona generated successfully with Kimi: {}", persona.name);
    
    Ok(persona)
}

/// 使用 Kimi 生成竞品分析（中国市场优化）
#[tauri::command]
pub async fn generate_competitor_analysis_kimi(
    request: GeneratePRDRequest,
) -> Result<CompetitorAnalysisResponse, String> {
    log::info!("Generating competitor analysis with Kimi for idea: {}", request.idea);
    
    // 1. 构建竞品分析提示词（中国市场优化）
    let prompt = format!(r#"请为以下产品想法进行详细的竞品分析，重点关注中国市场：

{}

请分析：
1. 中国主要竞争对手（至少 3 个，包括国内知名产品）
2. 每个竞争对手的优势和劣势
3. 市场份额或用户规模（如已知）
4. 差异化机会
5. 中国市场的特点和需求

请以结构化的方式呈现分析结果，使用中文输出。"#, request.idea);
    
    // 2. 创建 AI Provider
    let provider = AIProvider::new(AIProviderType::Kimi, request.api_key);
    
    // 3. 构建聊天请求
    let chat_request = ChatRequest {
        model: request.model,
        messages: vec![AIMessage {
            role: "user".to_string(),
            content: prompt,
        }],
        temperature: Some(0.7),
        max_tokens: Some(4096),
        stream: false,
    };
    
    // 4. 调用 AI Provider
    let response = provider.chat(chat_request)
        .await
        .map_err(|e| format!("Kimi 调用失败：{}", e))?;
    
    // 5. 解析 AI 生成的竞品分析
    let analysis = parse_competitor_analysis_from_markdown(&response.content)
        .map_err(|e| format!("竞品分析解析失败：{}", e))?;
    
    log::info!("Competitor analysis generated successfully with Kimi");
    
    Ok(analysis)
}

// ============================================================================
// GLM API 专用 Commands (AI-004)
// ============================================================================

/// GLM 聊天命令（非流式）
#[tauri::command]
pub async fn chat_glm(request: ChatRequestPayload) -> Result<ChatResponse, String> {
    log::info!("Sending chat request to GLM: {:?}", request);
    
    // 创建 AI Provider
    let provider = AIProvider::new(AIProviderType::GLM, request.api_key);
    
    // 构建聊天请求
    let chat_request = ChatRequest {
        model: request.model,
        messages: request.messages.into_iter().map(|msg| AIMessage {
            role: msg.role,
            content: msg.content,
        }).collect(),
        temperature: request.temperature,
        max_tokens: request.max_tokens,
        stream: false,
    };
    
    // 调用 AI Provider (GLM uses OpenAI-compatible API)
    let response = provider.chat(chat_request)
        .await
        .map_err(|e| format!("GLM 调用失败：{}", e))?;
    
    log::info!("GLM chat response received: {} chars", response.content.len());
    
    Ok(response)
}

/// GLM 聊天命令（流式）
#[tauri::command]
pub async fn stream_chat_glm(
    app: tauri::AppHandle,
    request: ChatRequestPayload,
) -> Result<String, String> {
    log::info!("Sending streaming chat request to GLM: {:?}", request);
    
    // 生成会话 ID
    let session_id = Uuid::new_v4().to_string();
    
    // 创建 AI Provider
    let provider = AIProvider::new(AIProviderType::GLM, request.api_key);
    
    // 构建聊天请求
    let chat_request = ChatRequest {
        model: request.model,
        messages: request.messages.into_iter().map(|msg| AIMessage {
            role: msg.role,
            content: msg.content,
        }).collect(),
        temperature: request.temperature,
        max_tokens: request.max_tokens,
        stream: true,
    };
    
    // 克隆 session_id 和 app handle 用于闭包
    let session_id_clone = session_id.clone();
    let app_handle_clone = app.clone();
    
    // 定义 chunk 处理回调
    let on_chunk = move |content: String| -> Result<(), crate::ai::AIError> {
        let chunk = StreamChunk {
            session_id: session_id_clone.clone(),
            content,
            is_complete: false,
        };
        
        // 发送事件到前端
        app_handle_clone.emit("ai-stream-chunk", chunk)
            .map_err(|e| crate::ai::AIError { 
                message: format!("Failed to emit chunk: {}", e) 
            })?;
        
        Ok(())
    };
    
    // 调用流式聊天
    match provider.stream_chat(chat_request, on_chunk).await {
        Ok(final_content) => {
            // 发送完成事件
            let complete_data = StreamComplete {
                session_id: session_id.clone(),
                content: final_content.clone(),
            };
            let _ = app.emit("ai-stream-complete", complete_data);
            
            log::info!("GLM streaming chat completed: {} chars", final_content.len());
            Ok(final_content)
        }
        Err(e) => {
            // 发送错误事件
            let error_data = StreamError {
                session_id: session_id.clone(),
                error: e.to_string(),
            };
            let _ = app.emit("ai-stream-error", error_data);
            
            log::error!("GLM streaming chat failed: {}", e);
            Err(e.to_string())
        }
    }
}
