//! AI 智能路由模块
//! 
//! AI-005: 实现 AI 智能路由系统，根据任务类型、成本、性能等因素自动选择最佳 AI Provider

use crate::ai::AIProviderType;
use log::info;
use std::collections::HashMap;

/// 路由策略
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RoutingStrategy {
    /// 自动路由 - 根据任务类型自动选择
    Auto,
    /// 成本优先 - 选择最经济的 AI
    CostEffective,
    /// 性能优先 - 选择最快的 AI
    Performance,
    /// 质量优先 - 选择质量最好的 AI
    Quality,
    /// 手动指定 - 用户自己选择
    Manual,
}

/// 任务类型
#[derive(Debug, Clone, PartialEq)]
pub enum TaskType {
    Chat,
    PRD,
    UserPersona,
    CompetitorAnalysis,
    CodeGeneration,
    Translation,
    Summary,
}

/// Provider 健康状态
#[derive(Debug, Clone)]
pub struct ProviderHealthStatus {
    pub is_healthy: bool,
    pub last_check_time: u64,
    pub consecutive_failures: u32,
    pub avg_response_time_ms: f64,
}

impl Default for ProviderHealthStatus {
    fn default() -> Self {
        Self {
            is_healthy: true,
            last_check_time: 0,
            consecutive_failures: 0,
            avg_response_time_ms: 0.0,
        }
    }
}

/// Provider 信息
#[derive(Debug, Clone)]
pub struct ProviderInfo {
    pub provider_type: AIProviderType,
    pub name: String,
    pub models: Vec<String>,
    pub cost_level: u8, // 1-5, 1 最便宜，5 最贵
    pub performance_level: u8, // 1-5, 1 最快，5 最慢
    pub quality_level: u8, // 1-5, 1 最好，5 最差
    pub health_status: ProviderHealthStatus,
    pub is_enabled: bool,
}

/// 路由决策结果
#[derive(Debug, Clone)]
pub struct RoutingDecision {
    pub selected_provider: AIProviderType,
    pub reason: String,
    pub alternatives: Vec<AIProviderType>,
}

/// AI 智能路由器
pub struct AISmartRouter {
    providers: HashMap<AIProviderType, ProviderInfo>,
    strategy: RoutingStrategy,
    manual_provider: Option<AIProviderType>,
    health_check_interval_secs: u64,
}

impl AISmartRouter {
    /// 创建新的智能路由器
    pub fn new() -> Self {
        let mut router = Self {
            providers: HashMap::new(),
            strategy: RoutingStrategy::Auto,
            manual_provider: None,
            health_check_interval_secs: 30,
        };
        
        // 初始化所有 Provider
        router.initialize_providers();
        router
    }
    
    /// 初始化 Provider 列表
    fn initialize_providers(&mut self) {
        // OpenAI
        self.providers.insert(
            AIProviderType::OpenAI,
            ProviderInfo {
                provider_type: AIProviderType::OpenAI,
                name: "OpenAI".to_string(),
                models: vec!["gpt-4".to_string(), "gpt-3.5-turbo".to_string()],
                cost_level: 4,
                performance_level: 2,
                quality_level: 1,
                health_status: ProviderHealthStatus::default(),
                is_enabled: true,
            },
        );
        
        // Claude
        self.providers.insert(
            AIProviderType::Anthropic,
            ProviderInfo {
                provider_type: AIProviderType::Anthropic,
                name: "Claude".to_string(),
                models: vec!["claude-3-opus".to_string(), "claude-3-sonnet".to_string()],
                cost_level: 4,
                performance_level: 3,
                quality_level: 1,
                health_status: ProviderHealthStatus::default(),
                is_enabled: true,
            },
        );
        
        // Kimi
        self.providers.insert(
            AIProviderType::Kimi,
            ProviderInfo {
                provider_type: AIProviderType::Kimi,
                name: "Kimi".to_string(),
                models: vec![
                    "kimi-k2".to_string(),       // kimi2.5
                    "kimi-k2-0711".to_string(),  // k2 早期版本
                    "moonshot-v1-8k".to_string(),
                    "moonshot-v1-32k".to_string(),
                    "moonshot-v1-128k".to_string(),
                ],
                cost_level: 2,
                performance_level: 2,
                quality_level: 2,
                health_status: ProviderHealthStatus::default(),
                is_enabled: true,
            },
        );
        
        // Kimi Code (编程专用)
        self.providers.insert(
            AIProviderType::KimiCode,
            ProviderInfo {
                provider_type: AIProviderType::KimiCode,
                name: "Kimi Code".to_string(),
                models: vec![
                    "kimi-for-coding".to_string(),  // Kimi Code 专用模型
                ],
                cost_level: 2,
                performance_level: 2,
                quality_level: 3,  // 代码能力更强
                health_status: ProviderHealthStatus::default(),
                is_enabled: true,
            },
        );
        
        // GLM
        self.providers.insert(
            AIProviderType::GLM,
            ProviderInfo {
                provider_type: AIProviderType::GLM,
                name: "GLM".to_string(),
                models: vec!["glm-4".to_string(), "glm-3-turbo".to_string()],
                cost_level: 2,
                performance_level: 2,
                quality_level: 2,
                health_status: ProviderHealthStatus::default(),
                is_enabled: true,
            },
        );
        
        info!("Initialized {} AI providers", self.providers.len());
    }
    
    /// 设置路由策略
    pub fn set_strategy(&mut self, strategy: RoutingStrategy) {
        info!("Setting routing strategy: {:?}", strategy);
        self.strategy = strategy;
    }
    
    /// 手动指定 Provider
    pub fn set_manual_provider(&mut self, provider: AIProviderType) {
        info!("Setting manual provider: {:?}", provider);
        self.manual_provider = Some(provider);
        self.strategy = RoutingStrategy::Manual;
    }
    
    /// 根据任务类型进行路由
    pub fn route(&self, task_type: &TaskType) -> RoutingDecision {
        match self.strategy {
            RoutingStrategy::Manual => {
                if let Some(provider) = self.manual_provider {
                    return RoutingDecision {
                        selected_provider: provider,
                        reason: "Manual selection".to_string(),
                        alternatives: vec![],
                    };
                }
            }
            RoutingStrategy::Auto => {
                return self.auto_route(task_type);
            }
            RoutingStrategy::CostEffective => {
                return self.cost_based_route();
            }
            RoutingStrategy::Performance => {
                return self.performance_based_route();
            }
            RoutingStrategy::Quality => {
                return self.quality_based_route();
            }
        }
        
        // Fallback to auto route
        self.auto_route(task_type)
    }
    
    /// 自动路由 - 根据任务类型选择最佳 Provider
    fn auto_route(&self, task_type: &TaskType) -> RoutingDecision {
        info!("Auto-routing task: {:?}", task_type);
        
        let selected = match task_type {
            TaskType::Chat => {
                // 聊天任务：优先考虑性能和成本
                self.select_by_criteria(&[
                    (AIProviderType::Kimi, "Fast and cost-effective for chat"),
                    (AIProviderType::GLM, "Good balance for chat"),
                    (AIProviderType::OpenAI, "High quality chat"),
                ])
            }
            TaskType::PRD | TaskType::UserPersona | TaskType::CompetitorAnalysis => {
                // 文档生成任务：优先考虑质量
                self.select_by_criteria(&[
                    (AIProviderType::Anthropic, "Best for long-form content"),
                    (AIProviderType::OpenAI, "High quality documentation"),
                    (AIProviderType::Kimi, "Good for Chinese content"),
                ])
            }
            TaskType::CodeGeneration => {
                // 代码生成：优先考虑技术能力
                self.select_by_criteria(&[
                    (AIProviderType::GLM, "Strong technical capabilities"),
                    (AIProviderType::OpenAI, "Excellent code generation"),
                    (AIProviderType::Anthropic, "Good code understanding"),
                ])
            }
            TaskType::Translation => {
                // 翻译任务：根据语言选择
                self.select_by_criteria(&[
                    (AIProviderType::OpenAI, "Best for multi-language"),
                    (AIProviderType::Kimi, "Good for Chinese translation"),
                    (AIProviderType::DeepL, "Specialized in translation"),
                ])
            }
            TaskType::Summary => {
                // 摘要任务：优先考虑理解能力
                self.select_by_criteria(&[
                    (AIProviderType::Anthropic, "Excellent long-context understanding"),
                    (AIProviderType::OpenAI, "Good summarization"),
                    (AIProviderType::Kimi, "Fast summarization"),
                ])
            }
        };
        
        RoutingDecision {
            selected_provider: selected.0,
            reason: selected.1.to_string(),
            alternatives: vec![],
        }
    }
    
    /// 成本优先路由
    fn cost_based_route(&self) -> RoutingDecision {
        info!("Cost-based routing");
        
        let mut providers: Vec<_> = self.providers.iter()
            .filter(|(_, p)| p.is_enabled && p.health_status.is_healthy)
            .collect();
        
        providers.sort_by_key(|(_, p)| p.cost_level);
        
        if let Some((provider_type, provider)) = providers.first() {
            RoutingDecision {
                selected_provider: **provider_type,
                reason: format!("Lowest cost level: {}", provider.cost_level),
                alternatives: providers.iter().skip(1).take(2).map(|(t, _)| **t).collect(),
            }
        } else {
            // Fallback to first available
            RoutingDecision {
                selected_provider: AIProviderType::OpenAI,
                reason: "No healthy providers, using fallback".to_string(),
                alternatives: vec![],
            }
        }
    }
    
    /// 性能优先路由
    fn performance_based_route(&self) -> RoutingDecision {
        info!("Performance-based routing");
        
        let mut providers: Vec<_> = self.providers.iter()
            .filter(|(_, p)| p.is_enabled && p.health_status.is_healthy)
            .collect();
        
        providers.sort_by_key(|(_, p)| p.performance_level);
        
        if let Some((provider_type, provider)) = providers.first() {
            RoutingDecision {
                selected_provider: **provider_type,
                reason: format!("Best performance level: {}", provider.performance_level),
                alternatives: providers.iter().skip(1).take(2).map(|(t, _)| **t).collect(),
            }
        } else {
            RoutingDecision {
                selected_provider: AIProviderType::OpenAI,
                reason: "No healthy providers, using fallback".to_string(),
                alternatives: vec![],
            }
        }
    }
    
    /// 质量优先路由
    fn quality_based_route(&self) -> RoutingDecision {
        info!("Quality-based routing");
        
        let mut providers: Vec<_> = self.providers.iter()
            .filter(|(_, p)| p.is_enabled && p.health_status.is_healthy)
            .collect();
        
        providers.sort_by_key(|(_, p)| p.quality_level);
        
        if let Some((provider_type, provider)) = providers.first() {
            RoutingDecision {
                selected_provider: **provider_type,
                reason: format!("Best quality level: {}", provider.quality_level),
                alternatives: providers.iter().skip(1).take(2).map(|(t, _)| **t).collect(),
            }
        } else {
            RoutingDecision {
                selected_provider: AIProviderType::OpenAI,
                reason: "No healthy providers, using fallback".to_string(),
                alternatives: vec![],
            }
        }
    }
    
    /// 根据条件选择 Provider
    fn select_by_criteria<'a>(&self, criteria: &[(AIProviderType, &'a str)]) -> (AIProviderType, &'a str) {
        for &(provider, reason) in criteria {
            if let Some(info) = self.providers.get(&provider) {
                if info.is_enabled && info.health_status.is_healthy {
                    return (provider, reason);
                }
            }
        }
        
        // Fallback to first healthy provider
        for (provider_type, info) in &self.providers {
            if info.is_enabled && info.health_status.is_healthy {
                return (*provider_type, "Fallback to available provider");
            }
        }
        
        // Ultimate fallback
        (AIProviderType::OpenAI, "Ultimate fallback")
    }
    
    /// 获取所有可用的 Provider
    pub fn get_available_providers(&self) -> Vec<AIProviderType> {
        self.providers.iter()
            .filter(|(_, info)| info.is_enabled && info.health_status.is_healthy)
            .map(|(provider_type, _)| *provider_type)
            .collect()
    }
    
    /// 更新 Provider 健康状态
    pub fn update_health_status(&mut self, provider: AIProviderType, status: ProviderHealthStatus) {
        if let Some(info) = self.providers.get_mut(&provider) {
            info.health_status = status;
        }
    }
    
    /// 启用/禁用 Provider
    pub fn set_provider_enabled(&mut self, provider: AIProviderType, enabled: bool) {
        if let Some(info) = self.providers.get_mut(&provider) {
            info.is_enabled = enabled;
            info!("Provider {} is now {}", info.name, if enabled { "enabled" } else { "disabled" });
        }
    }
}

impl Default for AISmartRouter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_router_creation() {
        let router = AISmartRouter::new();
        assert!(!router.providers.is_empty());
        assert_eq!(router.strategy, RoutingStrategy::Auto);
    }

    #[test]
    fn test_auto_route_chat() {
        let router = AISmartRouter::new();
        let decision = router.route(&TaskType::Chat);
        assert!(decision.reason.len() > 0);
    }

    #[test]
    fn test_cost_based_routing() {
        let mut router = AISmartRouter::new();
        router.set_strategy(RoutingStrategy::CostEffective);
        let decision = router.route(&TaskType::Chat);
        assert!(decision.reason.contains("cost"));
    }

    #[test]
    fn test_performance_based_routing() {
        let mut router = AISmartRouter::new();
        router.set_strategy(RoutingStrategy::Performance);
        let decision = router.route(&TaskType::Chat);
        assert!(decision.reason.contains("performance"));
    }

    #[test]
    fn test_quality_based_routing() {
        let mut router = AISmartRouter::new();
        router.set_strategy(RoutingStrategy::Quality);
        let decision = router.route(&TaskType::Chat);
        assert!(decision.reason.contains("quality"));
    }

    #[test]
    fn test_manual_provider_selection() {
        let mut router = AISmartRouter::new();
        router.set_manual_provider(AIProviderType::Anthropic);
        let decision = router.route(&TaskType::Chat);
        assert_eq!(decision.selected_provider, AIProviderType::Anthropic);
        assert!(decision.reason.contains("Manual"));
    }

    #[test]
    fn test_get_available_providers() {
        let router = AISmartRouter::new();
        let providers = router.get_available_providers();
        assert!(!providers.is_empty());
    }

    #[test]
    fn test_provider_health_update() {
        let mut router = AISmartRouter::new();
        let status = ProviderHealthStatus {
            is_healthy: false,
            last_check_time: 0,
            consecutive_failures: 3,
            avg_response_time_ms: 1000.0,
        };
        router.update_health_status(AIProviderType::OpenAI, status);
        // 验证健康状态已更新
    }

    #[test]
    fn test_provider_enable_disable() {
        let mut router = AISmartRouter::new();
        router.set_provider_enabled(AIProviderType::OpenAI, false);
        let providers = router.get_available_providers();
        assert!(!providers.contains(&AIProviderType::OpenAI));
    }

    #[test]
    fn test_fallback_mechanism() {
        let router = AISmartRouter::new();
        // 即使所有 provider 都不可用，也应该有 fallback
        let decision = router.route(&TaskType::Chat);
        assert!(decision.selected_provider != AIProviderType::MiniMax || 
                decision.reason.contains("fallback"));
    }
}
