#![allow(dead_code)]

use crate::ai::{AIProvider, AIProviderType};
use std::collections::HashMap;

pub mod observability_service;

pub use observability_service::ObservabilityService;

pub struct AIService {
    providers: HashMap<String, AIProvider>,
}

impl AIService {
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
        }
    }

    pub fn add_provider(&mut self, name: String, provider_type: AIProviderType, api_key: String) {
        let provider = AIProvider::new(provider_type, api_key);
        self.providers.insert(name, provider);
    }

    pub fn get_provider(&self, name: &str) -> Option<&AIProvider> {
        self.providers.get(name)
    }
}

impl Default for AIService {
    fn default() -> Self {
        Self::new()
    }
}
