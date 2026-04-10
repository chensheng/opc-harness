use crate::commands::quality::types::{AnalyzePRDDepthRequest, AnalyzePRDDepthResponse};

/// PRD 深度分析
#[tauri::command]
pub async fn analyze_prd_depth(request: AnalyzePRDDepthRequest) -> Result<AnalyzePRDDepthResponse, String> {
    log::info!("Starting PRD deep analysis...");
    
    let analyzer = crate::quality::prd_deep_analyzer::PrdDeepAnalyzer::new();
    
    match analyzer.analyze(&request.prd_content).await {
        Ok(mut analysis) => {
            // 如果有 API key，使用 AI 进行更深度的分析
            if let Some(api_key) = request.api_key {
                if !api_key.is_empty() {
                    match analyzer.analyze_with_ai(&request.prd_content, &api_key).await {
                        Ok(ai_analysis) => {
                            analysis = ai_analysis;
                        }
                        Err(e) => {
                            log::warn!("AI analysis failed, using basic analysis: {}", e);
                        }
                    }
                }
            }
            
            log::info!("PRD deep analysis completed. Found {} features", analysis.estimates.total_features);
            
            Ok(AnalyzePRDDepthResponse {
                success: true,
                analysis,
                error_message: None,
            })
        }
        Err(e) => {
            log::error!("PRD deep analysis failed: {}", e);
            
            Ok(AnalyzePRDDepthResponse {
                success: false,
                analysis: crate::quality::prd_deep_analyzer::PrdAnalysis::empty(),
                error_message: Some(e.to_string()),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_analyze_prd_depth_basic() {
        let request = AnalyzePRDDepthRequest {
            prd_content: "# PRD\n这是一个用户管理系统，需要数据分析和报告功能".to_string(),
            api_key: None,
        };
        
        let result = analyze_prd_depth(request).await;
        assert!(result.is_ok());
        
        let response = result.unwrap();
        assert!(response.success);
        assert!(response.analysis.estimates.total_features > 0);
    }
}
