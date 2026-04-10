//! Debug Agent 核心实现
//! 
//! 负责协调错误收集、解析和诊断的完整流程

use super::types::{DebugAgentConfig, DebugResult, DebugStatus, ErrorInfo};
use super::parsers::get_parser_for_source;
use super::diagnoser::Diagnoser;

/// Debug Agent 结构体
#[derive(Debug, Clone)]
pub struct DebugAgent {
    /// 配置信息
    pub config: DebugAgentConfig,
    /// 当前状态
    pub status: DebugStatus,
    /// 会话 ID
    pub session_id: String,
}

impl DebugAgent {
    /// 创建新的 Debug Agent
    pub fn new(config: DebugAgentConfig) -> Self {
        Self {
            config,
            status: DebugStatus::Pending,
            session_id: uuid::Uuid::new_v4().to_string(),
        }
    }

    /// 执行完整的调试流程
    pub async fn run_debug(&mut self) -> Result<DebugResult, String> {
        log::info!("开始执行 Debug Agent - Session: {}", self.session_id);

        // 1. 收集错误信息
        self.status = DebugStatus::CollectingErrors;
        log::info!("步骤 1/4: 收集错误信息");
        
        let errors = self.collect_errors().await?;
        
        if errors.is_empty() {
            log::info!("未检测到错误");
            self.status = DebugStatus::Completed;
            return Ok(DebugResult::success(vec![]));
        }

        log::info!("检测到 {} 个错误", errors.len());

        // 2. 解析错误
        self.status = DebugStatus::ParsingErrors;
        log::info!("步骤 2/4: 解析错误信息");
        
        let mut diagnoses = Vec::new();
        
        for error in errors {
            // 3. AI 诊断
            self.status = DebugStatus::Diagnosing;
            log::info!("步骤 3/4: 诊断错误：{}", error.message);
            
            match self.diagnose_error(&error).await {
                Ok(diagnosis) => {
                    log::info!("诊断完成，置信度：{:.2}", diagnosis.confidence);
                    diagnoses.push(diagnosis);
                }
                Err(e) => {
                    log::error!("诊断失败：{}", e);
                    // 继续处理下一个错误
                }
            }
        }

        // 4. 生成修复建议
        self.status = DebugStatus::GeneratingFixes;
        log::info!("步骤 4/4: 生成修复建议");
        
        // 已经完成在 diagnose_error 中
        
        // 完成
        self.status = DebugStatus::Completed;
        log::info!("Debug Agent 执行完成，共诊断 {} 个错误", diagnoses.len());
        
        Ok(DebugResult::success(diagnoses))
    }

    /// 收集错误信息
    async fn collect_errors(&self) -> Result<Vec<ErrorInfo>, String> {
        // 根据错误来源解析错误信息
        let parser = get_parser_for_source(&self.config.error_source);
        parser.parse_errors(&self.config.error_output)
    }

    /// AI 诊断错误
    async fn diagnose_error(&self, error: &ErrorInfo) -> Result<super::types::Diagnosis, String> {
        let diagnoser = Diagnoser::new();
        diagnoser.diagnose_error(error).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::types::ErrorSource;

    #[test]
    fn test_debug_agent_creation() {
        let config = DebugAgentConfig {
            project_path: "/tmp/test".to_string(),
            error_source: ErrorSource::TypeScript,
            auto_fix: false,
            max_suggestions: 5,
            error_output: "test error".to_string(),
        };

        let agent = DebugAgent::new(config);

        assert_eq!(agent.config.project_path, "/tmp/test");
        assert_eq!(agent.status, DebugStatus::Pending);
        assert!(!agent.session_id.is_empty());
    }
}
