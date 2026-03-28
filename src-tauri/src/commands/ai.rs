use crate::ai::{
    AIProvider, AIProviderType, ChatRequest, Message as AIMessage, StreamChunk, StreamComplete, StreamError,
};
use crate::prompts::prd_template;
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
    _request: GeneratePRDRequest,
) -> Result<Vec<UserPersonaResponse>, String> {
    // TODO: Implement actual persona generation
    Ok(vec![UserPersonaResponse {
        id: "1".to_string(),
        name: "Alex".to_string(),
        age: "28".to_string(),
        occupation: "Full-stack Developer".to_string(),
        background: "Experienced developer working on side projects".to_string(),
        goals: vec!["Build passive income".to_string()],
        pain_points: vec!["Limited time".to_string()],
        behaviors: vec!["Active on Twitter".to_string()],
        quote: Some("I want to focus on creative work.".to_string()),
    }])
}

#[tauri::command]
pub async fn generate_competitor_analysis(
    _request: GeneratePRDRequest,
) -> Result<CompetitorAnalysisResponse, String> {
    // TODO: Implement actual competitor analysis
    Ok(CompetitorAnalysisResponse {
        competitors: vec![CompetitorResponse {
            name: "Competitor A".to_string(),
            strengths: vec!["Brand recognition".to_string()],
            weaknesses: vec!["High price".to_string()],
            market_share: Some("35%".to_string()),
        }],
        differentiation: "Our unique value proposition.".to_string(),
        opportunities: vec!["Growing market".to_string()],
    })
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

// ========== 单元测试 ==========

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_first_heading_with_h1() {
        let content = "# 产品需求文档 - Test Product\n\nSome content...";
        let result = extract_first_heading(content);
        
        assert_eq!(result, Some("产品需求文档 - Test Product".to_string()));
    }

    #[test]
    fn test_extract_first_heading_without_h1() {
        let content = "Some content without heading";
        let result = extract_first_heading(content);
        
        assert_eq!(result, None);
    }

    #[test]
    fn test_extract_first_heading_multiline() {
        let content = r#"# First Heading
Some text
## Second Heading
More text"#;
        let result = extract_first_heading(content);
        
        assert_eq!(result, Some("First Heading".to_string()));
    }

    #[test]
    fn test_extract_section_found() {
        let content = r#"## 1. 产品概述
这是产品概述的内容。
包含多行描述。

## 2. 目标用户
这是目标用户章节。"#;
        
        let result = extract_section(content, "产品概述");
        
        assert!(result.is_some());
        assert!(result.unwrap().contains("这是产品概述的内容"));
    }

    #[test]
    fn test_extract_section_not_found() {
        let content = r#"## 1. 产品概述
这是产品概述的内容。

## 2. 目标用户
这是目标用户章节。"#;
        
        let result = extract_section(content, "不存在的章节");
        
        assert!(result.is_none());
    }

    #[test]
    fn test_extract_section_stops_at_next_heading() {
        let content = r#"## 1. 产品概述
这是第一节的内容。
不应该包含第二节的内容。

## 2. 其他章节
这是第二节的内容。"#;
        
        let result = extract_section(content, "产品概述");
        
        assert!(result.is_some());
        let section = result.unwrap();
        assert!(section.contains("这是第一节的内容"));
        assert!(!section.contains("这是第二节的内容"));
    }

    #[test]
    fn test_extract_list_items_basic() {
        let content = r#"## 目标用户
- 独立开发者
- 自由职业者
- 小团队

## 其他内容
一些其他内容。"#;
        
        let result = extract_list_items(content, "目标用户");
        
        assert!(result.is_some());
        let items = result.unwrap();
        assert_eq!(items.len(), 3);
        assert_eq!(items[0], "独立开发者");
        assert_eq!(items[1], "自由职业者");
        assert_eq!(items[2], "小团队");
    }

    #[test]
    fn test_extract_list_items_empty() {
        let content = r#"## 目标用户
没有列表项。

## 其他内容
一些其他内容。"#;
        
        let result = extract_list_items(content, "目标用户");
        
        assert!(result.is_none());
    }

    #[test]
    fn test_extract_list_items_with_continuation() {
        let content = r#"## 核心功能
- 功能 1：详细描述
  这个描述跨越多行
- 功能 2

## 其他
内容。"#;
        
        let result = extract_list_items(content, "核心功能");
        
        assert!(result.is_some());
        let items = result.unwrap();
        assert!(items.len() >= 2);
        assert!(items[0].contains("功能 1"));
        assert!(items[0].contains("详细描述"));
    }

    #[test]
    fn test_parse_prd_from_markdown_complete() {
        let content = r#"# 产品需求文档 - Test Product

## 1. 产品概述
这是一个测试产品的概述描述。

## 2. 目标用户
- 用户群体 A
- 用户群体 B

## 3. 核心功能
- 功能点一
- 功能点二

## 4. 技术栈
- React
- Rust
- Tauri

## 5. 开发计划
### 时间估算
预计 2-4 周完成。

## 6. 商业模式
采用订阅制收费。

## 7. 定价策略
基础版免费，专业版$9/月。"#;
        
        let result = parse_prd_from_markdown(content);
        
        assert!(result.is_ok());
        let prd = result.unwrap();
        
        assert_eq!(prd.title, "产品需求文档 - Test Product");
        assert!(prd.overview.contains("测试产品"));
        assert_eq!(prd.target_users.len(), 2);
        assert_eq!(prd.core_features.len(), 2);
        assert_eq!(prd.tech_stack.len(), 3);
        assert!(prd.estimated_effort.contains("2-4 周"));
        assert!(prd.business_model.is_some());
        assert!(prd.pricing.is_some());
    }

    #[test]
    fn test_parse_prd_from_markdown_minimal() {
        let content = "# Minimal Product\n\nSome minimal content.";
        
        let result = parse_prd_from_markdown(content);
        
        assert!(result.is_ok());
        let prd = result.unwrap();
        
        assert_eq!(prd.title, "Minimal Product");
        // 其他字段应该使用默认值
        assert_eq!(prd.target_users[0], "Target users to be defined");
    }

    #[test]
    fn test_parse_prd_from_markdown_with_asterisk_list() {
        let content = r#"## 目标用户
* 用户 A
* 用户 B
* 用户 C"#;
        
        let result = extract_list_items(content, "目标用户");
        
        assert!(result.is_some());
        let items = result.unwrap();
        assert_eq!(items.len(), 3);
    }

    #[test]
    fn test_parse_prd_from_markdown_with_plus_list() {
        let content = r#"## 目标用户
+ 用户 A
+ 用户 B
+ 用户 C"#;
        
        let result = extract_list_items(content, "目标用户");
        
        assert!(result.is_some());
        let items = result.unwrap();
        assert_eq!(items.len(), 3);
    }
}
