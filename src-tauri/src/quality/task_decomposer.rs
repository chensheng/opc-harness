#![allow(dead_code)]

/// US-032: 智能任务分解与依赖识别
/// 
/// 基于 PRD 功能点列表，智能分解为技术任务并识别依赖关系：
/// - 任务分解（前端/后端/数据库/测试）
/// - 依赖识别（模块依赖/技术依赖/数据流依赖）
/// - 优先级排序（基于依赖和复杂度）
/// - 工时估算（详细到每个任务）
/// - 关键路径计算

use serde::{Deserialize, Serialize};

/// 任务类型枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskType {
    /// 前端开发
    Frontend,
    /// 后端开发
    Backend,
    /// 数据库开发
    Database,
    /// 测试
    Testing,
    /// 文档
    Documentation,
    /// 部署
    Deployment,
}

/// 技术任务结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechnicalTask {
    /// 任务 ID
    pub id: String,
    /// 任务标题
    pub title: String,
    /// 任务描述
    pub description: String,
    /// 任务类型
    pub task_type: TaskType,
    /// 预估工时（小时）
    pub estimated_hours: f32,
    /// 依赖的任务 ID 列表
    pub dependencies: Vec<String>,
    /// 优先级 (1-10)
    pub priority: u8,
    /// 关联的功能 ID
    pub feature_id: String,
    /// 复杂度评分 (1-5)
    pub complexity: u8,
    /// 所需技能标签
    pub skills: Vec<String>,
}

/// 依赖边结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyEdge {
    /// 源任务 ID
    pub from_task: String,
    /// 目标任务 ID
    pub to_task: String,
    /// 依赖类型
    pub dependency_type: String,
    /// 依赖强度
    pub strength: String,
}

/// 任务依赖图
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskDependencyGraph {
    /// 所有任务列表
    pub tasks: Vec<TechnicalTask>,
    /// 依赖边列表
    pub edges: Vec<DependencyEdge>,
    /// 关键路径（任务 ID 序列）
    pub critical_path: Vec<String>,
    /// 总预估工时
    pub total_estimated_hours: f32,
    /// 任务统计
    pub statistics: TaskStatistics,
}

/// 任务统计结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskStatistics {
    /// 总任务数
    pub total_tasks: u32,
    /// 前端任务数
    pub frontend_tasks: u32,
    /// 后端任务数
    pub backend_tasks: u32,
    /// 数据库任务数
    pub database_tasks: u32,
    /// 测试任务数
    pub testing_tasks: u32,
    /// 平均工时
    pub average_hours: f32,
    /// 平均复杂度
    pub average_complexity: f32,
}

impl TaskDependencyGraph {
    /// 创建空的任务依赖图
    pub fn empty() -> Self {
        Self {
            tasks: Vec::new(),
            edges: Vec::new(),
            critical_path: Vec::new(),
            total_estimated_hours: 0.0,
            statistics: TaskStatistics {
                total_tasks: 0,
                frontend_tasks: 0,
                backend_tasks: 0,
                database_tasks: 0,
                testing_tasks: 0,
                average_hours: 0.0,
                average_complexity: 0.0,
            },
        }
    }

    /// 计算统计数据
    pub fn calculate_statistics(&mut self) {
        self.statistics.total_tasks = self.tasks.len() as u32;
        
        self.statistics.frontend_tasks = self.tasks.iter()
            .filter(|t| matches!(t.task_type, TaskType::Frontend))
            .count() as u32;
        
        self.statistics.backend_tasks = self.tasks.iter()
            .filter(|t| matches!(t.task_type, TaskType::Backend))
            .count() as u32;
        
        self.statistics.database_tasks = self.tasks.iter()
            .filter(|t| matches!(t.task_type, TaskType::Database))
            .count() as u32;
        
        self.statistics.testing_tasks = self.tasks.iter()
            .filter(|t| matches!(t.task_type, TaskType::Testing))
            .count() as u32;
        
        if !self.tasks.is_empty() {
            self.statistics.average_hours = self.tasks.iter()
                .map(|t| t.estimated_hours)
                .sum::<f32>() / self.tasks.len() as f32;
            
            self.statistics.average_complexity = self.tasks.iter()
                .map(|t| t.complexity as f32)
                .sum::<f32>() / self.tasks.len() as f32;
        }
        
        self.total_estimated_hours = self.tasks.iter()
            .map(|t| t.estimated_hours)
            .sum();
    }

    /// 计算关键路径（简化版）
    pub fn calculate_critical_path(&mut self) {
        // TODO: 实现完整的关路径算法（CPM）
        // 这里使用简化版本：按依赖关系排序
        
        let mut visited = std::collections::HashSet::new();
        let mut path = Vec::new();
        
        for task in &self.tasks {
            if task.dependencies.is_empty() && !visited.contains(&task.id) {
                self.dfs_critical_path(task.id.clone(), &mut visited, &mut path);
            }
        }
        
        self.critical_path = path;
    }

    fn dfs_critical_path(
        &self,
        task_id: String,
        visited: &mut std::collections::HashSet<String>,
        path: &mut Vec<String>,
    ) {
        if visited.contains(&task_id) {
            return;
        }
        
        visited.insert(task_id.clone());
        path.push(task_id.clone());
        
        // 查找依赖当前任务的所有任务
        for edge in &self.edges {
            if edge.from_task == task_id {
                self.dfs_critical_path(edge.to_task.clone(), visited, path);
            }
        }
    }
}

/// 任务分解器
pub struct TaskDecomposer;

impl TaskDecomposer {
    /// 创建新的分解器
    pub fn new() -> Self {
        Self
    }

    /// 分解功能点为技术任务
    pub async fn decompose_features(
        &self,
        features: &[crate::quality::prd_deep_analyzer::Feature],
    ) -> Result<TaskDependencyGraph, Box<dyn std::error::Error>> {
        let mut graph = TaskDependencyGraph::empty();
        
        // 为每个功能点生成技术任务
        for feature in features {
            let tasks = self.generate_tasks_for_feature(feature);
            graph.tasks.extend(tasks);
        }
        
        // 识别依赖关系
        graph.edges = self.identify_dependencies(&graph.tasks);
        
        // 计算统计数据
        graph.calculate_statistics();
        
        // 计算关键路径
        graph.calculate_critical_path();
        
        Ok(graph)
    }

    /// 为单个功能点生成技术任务
    fn generate_tasks_for_feature(
        &self,
        feature: &crate::quality::prd_deep_analyzer::Feature,
    ) -> Vec<TechnicalTask> {
        let mut tasks = Vec::new();
        let mut task_counter = 1;
        
        // 根据功能类型生成不同类型的任务
        match feature.feature_type {
            crate::quality::prd_deep_analyzer::FeatureType::Core => {
                // 核心功能：完整的开发流程
                
                // 数据库任务
                tasks.push(TechnicalTask {
                    id: format!("T{}-DB", task_counter),
                    title: format!("{} - 数据库设计", feature.name),
                    description: format!("设计与{}相关的数据库表结构", feature.description),
                    task_type: TaskType::Database,
                    estimated_hours: feature.complexity as f32 * 1.5,
                    dependencies: vec![],
                    priority: 9,
                    feature_id: feature.id.clone(),
                    complexity: feature.complexity,
                    skills: vec!["SQL".to_string(), "Database Design".to_string()],
                });
                task_counter += 1;
                
                // 后端任务
                tasks.push(TechnicalTask {
                    id: format!("T{}-BE-API", task_counter),
                    title: format!("{} - API 开发", feature.name),
                    description: format!("实现{}相关的 RESTful API", feature.description),
                    task_type: TaskType::Backend,
                    estimated_hours: feature.complexity as f32 * 2.0,
                    dependencies: vec![format!("T{}-DB", task_counter - 1)],
                    priority: 8,
                    feature_id: feature.id.clone(),
                    complexity: feature.complexity,
                    skills: vec!["Rust".to_string(), "API Design".to_string()],
                });
                task_counter += 1;
                
                // 前端任务
                tasks.push(TechnicalTask {
                    id: format!("T{}-FE-UI", task_counter),
                    title: format!("{} - 界面开发", feature.name),
                    description: format!("实现{}相关的前端界面", feature.description),
                    task_type: TaskType::Frontend,
                    estimated_hours: feature.complexity as f32 * 1.8,
                    dependencies: vec![format!("T{}-BE-API", task_counter - 1)],
                    priority: 7,
                    feature_id: feature.id.clone(),
                    complexity: feature.complexity,
                    skills: vec!["React".to_string(), "TypeScript".to_string()],
                });
                task_counter += 1;
                
                // 测试任务
                tasks.push(TechnicalTask {
                    id: format!("T{}-TEST", task_counter),
                    title: format!("{} - 测试", feature.name),
                    description: format!("编写{}相关的单元测试和集成测试", feature.description),
                    task_type: TaskType::Testing,
                    estimated_hours: feature.complexity as f32 * 1.2,
                    dependencies: vec![
                        format!("T{}-BE-API", task_counter - 2),
                        format!("T{}-FE-UI", task_counter - 1),
                    ],
                    priority: 6,
                    feature_id: feature.id.clone(),
                    complexity: feature.complexity,
                    skills: vec!["Testing".to_string(), "Vitest".to_string()],
                });
            }
            
            crate::quality::prd_deep_analyzer::FeatureType::Auxiliary => {
                // 辅助功能：简化的开发流程
                
                // 后端任务
                tasks.push(TechnicalTask {
                    id: format!("T{}-BE", task_counter),
                    title: format!("{} - 后端实现", feature.name),
                    description: format!("实现{}相关的后端逻辑", feature.description),
                    task_type: TaskType::Backend,
                    estimated_hours: feature.complexity as f32 * 1.5,
                    dependencies: vec![],
                    priority: 6,
                    feature_id: feature.id.clone(),
                    complexity: feature.complexity,
                    skills: vec!["Rust".to_string()],
                });
                task_counter += 1;
                
                // 前端任务
                tasks.push(TechnicalTask {
                    id: format!("T{}-FE", task_counter),
                    title: format!("{} - 前端实现", feature.name),
                    description: format!("实现{}相关的前端界面", feature.description),
                    task_type: TaskType::Frontend,
                    estimated_hours: feature.complexity as f32 * 1.3,
                    dependencies: vec![format!("T{}-BE", task_counter - 1)],
                    priority: 5,
                    feature_id: feature.id.clone(),
                    complexity: feature.complexity,
                    skills: vec!["React".to_string()],
                });
            }
            
            crate::quality::prd_deep_analyzer::FeatureType::Enhanced => {
                // 增强功能：最简化的开发流程
                
                tasks.push(TechnicalTask {
                    id: format!("T{}-ENH", task_counter),
                    title: format!("{} - 增强实现", feature.name),
                    description: format!("实现{}增强功能", feature.description),
                    task_type: TaskType::Frontend,
                    estimated_hours: feature.complexity as f32,
                    dependencies: vec![],
                    priority: 4,
                    feature_id: feature.id.clone(),
                    complexity: feature.complexity,
                    skills: vec!["CSS".to_string(), "UX".to_string()],
                });
            }
        }
        
        tasks
    }

    /// 识别任务间的依赖关系
    fn identify_dependencies(&self, tasks: &[TechnicalTask]) -> Vec<DependencyEdge> {
        let mut edges = Vec::new();
        
        // 基于任务类型的依赖规则
        let db_tasks: Vec<_> = tasks.iter()
            .filter(|t| matches!(t.task_type, TaskType::Database))
            .collect();
        
        let be_tasks: Vec<_> = tasks.iter()
            .filter(|t| matches!(t.task_type, TaskType::Backend))
            .collect();
        
        let fe_tasks: Vec<_> = tasks.iter()
            .filter(|t| matches!(t.task_type, TaskType::Frontend))
            .collect();
        
        // 数据库 → 后端依赖
        for db_task in db_tasks {
            for be_task in &be_tasks {
                if db_task.feature_id == be_task.feature_id {
                    edges.push(DependencyEdge {
                        from_task: db_task.id.clone(),
                        to_task: be_task.id.clone(),
                        dependency_type: "technical".to_string(),
                        strength: "strong".to_string(),
                    });
                }
            }
        }
        
        // 后端 → 前端依赖
        for be_task in be_tasks {
            for fe_task in &fe_tasks {
                if be_task.feature_id == fe_task.feature_id {
                    edges.push(DependencyEdge {
                        from_task: be_task.id.clone(),
                        to_task: fe_task.id.clone(),
                        dependency_type: "technical".to_string(),
                        strength: "strong".to_string(),
                    });
                }
            }
        }
        
        edges
    }
}

impl Default for TaskDecomposer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quality::prd_deep_analyzer::{Feature, FeatureType};

    #[test]
    fn test_empty_graph() {
        let graph = TaskDependencyGraph::empty();
        
        assert_eq!(graph.tasks.len(), 0);
        assert_eq!(graph.edges.len(), 0);
        assert_eq!(graph.statistics.total_tasks, 0);
    }

    #[tokio::test]
    async fn test_decompose_core_feature() {
        let decomposer = TaskDecomposer::new();
        
        let features = vec![
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
        ];
        
        let graph = decomposer.decompose_features(&features).await.unwrap();
        
        // 核心功能应该生成至少 4 个任务（DB/BE/FE/TEST）
        assert!(graph.tasks.len() >= 4);
        assert!(graph.statistics.backend_tasks > 0);
        assert!(graph.statistics.frontend_tasks > 0);
    }

    #[tokio::test]
    async fn test_decompose_mixed_features() {
        let decomposer = TaskDecomposer::new();
        
        let features = vec![
            Feature {
                id: "F001".to_string(),
                name: "核心功能".to_string(),
                description: "测试".to_string(),
                feature_type: FeatureType::Core,
                complexity: 3,
                estimated_hours: 6.0,
                priority: 8,
                dependencies: vec![],
            },
            Feature {
                id: "F002".to_string(),
                name: "辅助功能".to_string(),
                description: "测试".to_string(),
                feature_type: FeatureType::Auxiliary,
                complexity: 2,
                estimated_hours: 4.0,
                priority: 5,
                dependencies: vec![],
            },
        ];
        
        let mut graph = decomposer.decompose_features(&features).await.unwrap();
        graph.calculate_statistics();
        
        assert!(graph.tasks.len() >= 6); // Core(4) + Auxiliary(2)
        assert!(graph.statistics.total_tasks >= 6);
    }

    #[test]
    fn test_calculate_statistics() {
        let mut graph = TaskDependencyGraph::empty();
        
        graph.tasks.push(TechnicalTask {
            id: "T1".to_string(),
            title: "Backend Task".to_string(),
            description: "Test".to_string(),
            task_type: TaskType::Backend,
            estimated_hours: 4.0,
            dependencies: vec![],
            priority: 8,
            feature_id: "F001".to_string(),
            complexity: 3,
            skills: vec!["Rust".to_string()],
        });
        
        graph.tasks.push(TechnicalTask {
            id: "T2".to_string(),
            title: "Frontend Task".to_string(),
            description: "Test".to_string(),
            task_type: TaskType::Frontend,
            estimated_hours: 3.0,
            dependencies: vec!["T1".to_string()],
            priority: 7,
            feature_id: "F001".to_string(),
            complexity: 2,
            skills: vec!["React".to_string()],
        });
        
        graph.calculate_statistics();
        
        assert_eq!(graph.statistics.total_tasks, 2);
        assert_eq!(graph.statistics.backend_tasks, 1);
        assert_eq!(graph.statistics.frontend_tasks, 1);
        assert_eq!(graph.total_estimated_hours, 7.0);
    }
}
