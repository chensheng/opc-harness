use crate::commands::quality::types::{DecomposeTasksRequest, DecomposeTasksResponse};

/// 分解任务
#[tauri::command]
pub async fn decompose_tasks(request: DecomposeTasksRequest) -> Result<DecomposeTasksResponse, String> {
    log::info!("Starting task decomposition...");
    
    let decomposer = crate::quality::task_decomposer::TaskDecomposer::new();
    
    match decomposer.decompose_features(&request.analysis.features).await {
        Ok(task_graph) => {
            log::info!("Task decomposition completed. Generated {} tasks", task_graph.statistics.total_tasks);
            
            Ok(DecomposeTasksResponse {
                success: true,
                task_graph,
                error_message: None,
            })
        }
        Err(e) => {
            log::error!("Task decomposition failed: {}", e);
            
            Ok(DecomposeTasksResponse {
                success: false,
                task_graph: crate::quality::task_decomposer::TaskDependencyGraph::empty(),
                error_message: Some(e.to_string()),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quality::prd_deep_analyzer::{Feature, FeatureType, PrdAnalysis, Estimates};

    #[tokio::test]
    async fn test_decompose_tasks_basic() {
        let analysis = PrdAnalysis {
            features: vec![
                Feature {
                    id: "F001".to_string(),
                    name: "用户管理".to_string(),
                    description: "用户 CRUD 操作".to_string(),
                    feature_type: FeatureType::Core,
                    complexity: 3,
                    estimated_hours: 6.0,
                    priority: 8,
                    dependencies: vec![],
                }
            ],
            dependencies: vec![],
            risks: vec![],
            estimates: Estimates {
                total_features: 1,
                core_features: 1,
                auxiliary_features: 0,
                enhanced_features: 0,
                average_complexity: 3.0,
                total_estimated_hours: 6.0,
                high_risks_count: 0,
            },
        };
        
        let request = DecomposeTasksRequest { analysis };
        let result = decompose_tasks(request).await;
        assert!(result.is_ok());
        
        let response = result.unwrap();
        assert!(response.success);
        assert!(response.task_graph.statistics.total_tasks > 0);
    }
}
