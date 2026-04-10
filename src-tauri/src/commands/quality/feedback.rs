use crate::commands::quality::types::{SubmitFeedbackRequest, SubmitFeedbackResponse};

/// 提交反馈并重新生成 PRD
#[tauri::command]
pub async fn submit_feedback_and_regenerate(
    request: SubmitFeedbackRequest,
) -> Result<SubmitFeedbackResponse, String> {
    use crate::quality::feedback_processor::PRDFeedbackProcessor;
    
    // 创建反馈处理器
    let processor = PRDFeedbackProcessor::new();
    
    // 解析用户反馈
    let parsed_feedback = processor.parse_feedback(&request.feedback_content)?;
    
    // 保存需要克隆的字段
    let sentiment = parsed_feedback.sentiment.clone();
    let priority = parsed_feedback.priority.clone();
    
    // 创建反馈对象
    let feedback = crate::quality::feedback_processor::Feedback {
        id: format!("fb_{}", chrono::Utc::now().timestamp()),
        prd_id: request.prd_id.clone(),
        section: request.section.clone(),
        content: request.feedback_content.clone(),
        sentiment,
        priority,
        timestamp: chrono::Utc::now().to_rfc3339(),
    };
    
    // 识别受影响的章节
    let prd_structure = crate::quality::feedback_processor::PRDStructure::default();
    let affected_sections = processor.identify_affected_sections(
        &parsed_feedback,
        &prd_structure,
    );
    
    // 构建再生成请求
    let regen_request = crate::quality::feedback_processor::RegenerateRequest {
        prd_content: request.prd_content,
        feedbacks: vec![feedback],
        sections_to_regenerate: if let Some(section) = request.section {
            vec![section]
        } else {
            affected_sections
        },
        iteration_count: request.iteration_count,
    };
    
    // 执行再生成
    let result = processor.regenerate_with_feedback(&regen_request)?;
    
    Ok(SubmitFeedbackResponse {
        new_prd_content: result.new_prd_content,
        changed_sections: result.changed_sections,
        quality_score_before: result.quality_score_before,
        quality_score_after: result.quality_score_after,
        iteration_number: result.iteration_number,
        success: result.success,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_submit_feedback_basic() {
        let request = SubmitFeedbackRequest {
            prd_id: "test_prd".to_string(),
            prd_content: "# Test PRD\n\nContent here.".to_string(),
            feedback_content: "用户画像部分需要更详细".to_string(),
            section: Some("用户画像".to_string()),
            iteration_count: 0,
        };
        
        let result = submit_feedback_and_regenerate(request).await;
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.success);
        assert!(response.iteration_number == 1);
        assert!(response.quality_score_after > response.quality_score_before);
    }

    #[tokio::test]
    async fn test_submit_feedback_overall() {
        let request = SubmitFeedbackRequest {
            prd_id: "test_prd".to_string(),
            prd_content: "# Test PRD".to_string(),
            feedback_content: "整体不错，但有些小问题".to_string(),
            section: None,
            iteration_count: 0,
        };
        
        let result = submit_feedback_and_regenerate(request).await;
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.success);
    }

    #[tokio::test]
    async fn test_max_iteration_exceeded() {
        let request = SubmitFeedbackRequest {
            prd_id: "test_prd".to_string(),
            prd_content: "# Test PRD".to_string(),
            feedback_content: "继续改进".to_string(),
            section: None,
            iteration_count: 10, // 超过最大限制
        };
        
        let result = submit_feedback_and_regenerate(request).await;
        assert!(result.is_err());
    }
}
