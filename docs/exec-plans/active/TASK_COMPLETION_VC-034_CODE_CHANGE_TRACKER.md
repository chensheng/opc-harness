# 执行计划：VC-034 - 实现代码变更追踪 Agent

> **状态**: 🔄 进行中  
> **优先级**: P0  
> **开始日期**: 2026-03-28  
> **预计完成**: 2026-03-28  
> **负责人**: OPC-HARNESS Team  
> **文档版本**: v1.0  
> **Harness Health Score**: 待验证  

---

## 📋 任务概述

### 背景
在 Vibe Coding 流程中，AI Agent 会生成大量的代码变更。目前缺少一个机制来：
- 追踪和汇总这些变更
- 分析变更的影响范围
- 为 HITL 检查点提供结构化的变更报告
- 支持变更回滚和版本对比

### 目标
实现 CodeChangeTracker Agent，能够：
1. 自动检测工作区的文件变更
2. 生成结构化的变更摘要（按文件/按功能分组）
3. 分析变更影响（识别依赖文件）
4. 提供变更统计信息（新增/修改/删除行数）
5. 支持变更历史查询和对比

### 范围
**包含**:
- ✅ Rust 后端实现（CodeChangeTracker Agent）
- ✅ Tauri Commands 集成
- ✅ 单元测试（覆盖率≥95%）
- ✅ 类型定义和数据结构
- ✅ Git diff 解析和统计

**不包含**:
- ❌ 前端 UI 展示（后续任务）
- ❌ 变更可视化对比（后续任务）
- ❌ 自动回滚功能（后续任务）

### 关键结果
- [ ] CodeChangeTracker Agent 完整实现
- [ ] 支持 Git diff 解析和统计
- [ ] 支持变更影响分析
- [ ] 15+ 单元测试，覆盖率≥95%
- [ ] Harness Health Score ≥90
- [ ] E2E 测试通过

---

## 🏗️ 解决方案设计

### 架构设计

```
┌─────────────────────────────────────┐
│   Tauri Commands                    │
│   - get_workspace_changes()         │
│   - get_file_diff(file_path)        │
│   - get_change_statistics()         │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│   CodeChangeTracker Agent           │
│   ├─ detect_changes()               │
│   ├─ analyze_impact()               │
│   ├─ generate_summary()             │
│   └─ calculate_statistics()         │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│   Git Integration                   │
│   - git diff                        │
│   - git status                      │
│   - git history                     │
└─────────────────────────────────────┘
```

### 核心数据结构

```rust
/// 变更类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeType {
    Added,      // 新增文件
    Modified,   // 修改文件
    Deleted,    // 删除文件
    Renamed,    // 重命名文件
}

/// 单个文件的变更信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileChange {
    pub file_path: String,
    pub change_type: ChangeType,
    pub additions: u32,      // 新增行数
    pub deletions: u32,      // 删除行数
    pub diff: String,        // Git diff 输出
    pub impacted_files: Vec<String>,  // 受影响的依赖文件
}

/// 变更统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeStatistics {
    pub total_files_changed: u32,
    pub total_additions: u32,
    pub total_deletions: u32,
    pub net_change: i32,  // additions - deletions
    pub files_by_type: HashMap<ChangeType, u32>,
}

/// 变更摘要
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeSummary {
    pub statistics: ChangeStatistics,
    pub changes: Vec<FileChange>,
    pub impacted_files: Vec<String>,
    pub generated_at: String,
}

/// CodeChangeTracker Agent
pub struct CodeChangeTracker {
    workspace_root: PathBuf,
    git_repo: Option<GitRepository>,
}
```

### 核心接口

```rust
impl CodeChangeTracker {
    /// 创建新的变更追踪器
    pub fn new(workspace_root: PathBuf) -> Result<Self>;
    
    /// 检测工作区的所有变更
    pub async fn detect_changes(&self) -> Result<Vec<FileChange>>;
    
    /// 获取单个文件的 diff
    pub async fn get_file_diff(&self, file_path: &str) -> Result<String>;
    
    /// 分析变更影响（识别依赖文件）
    pub async fn analyze_impact(&self, changes: &[FileChange]) -> Result<Vec<String>>;
    
    /// 生成变更摘要
    pub async fn generate_summary(&self) -> Result<ChangeSummary>;
    
    /// 计算变更统计
    pub fn calculate_statistics(&self, changes: &[FileChange]) -> ChangeStatistics;
}
```

### Tauri Commands

```rust
#[tauri::command]
async fn get_workspace_changes() -> Result<ChangeSummary, String>;

#[tauri::command]
async fn get_file_diff(file_path: String) -> Result<String, String>;

#[tauri::command]
async fn get_change_statistics() -> Result<ChangeStatistics, String>;
```

---

## 🧪 测试策略

### 单元测试覆盖

#### CodeChangeTracker 测试
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_tracker_creation()
    #[test]
    fn test_detect_added_files()
    #[test]
    fn test_detect_modified_files()
    #[test]
    fn test_detect_deleted_files()
    #[test]
    fn test_parse_git_diff()
    #[test]
    fn test_calculate_statistics()
    #[test]
    fn test_analyze_imports_typescript()
    #[test]
    fn test_analyze_imports_rust()
    #[test]
    fn test_generate_summary()
    #[test]
    fn test_empty_workspace()
    #[test]
    fn test_non_git_project()
}
```

### 测试数据准备
- 创建临时 Git 仓库
- 模拟各种变更场景（新增/修改/删除）
- 准备带有 import/require 语句的测试文件

---

## 📊 质量指标

| 指标 | 目标 | 实际 | 状态 |
|------|------|------|------|
| TypeScript 编译 | 通过 | - | ⏳ |
| ESLint 检查 | 通过 | - | ⏳ |
| Prettier 格式化 | 一致 | - | ⏳ |
| Rust cargo check | 通过 | - | ⏳ |
| 单元测试覆盖率 | ≥95% | - | ⏳ |
| Harness Health Score | ≥90 | - | ⏳ |

---

## 🚀 执行日志

### 2026-03-28 12:30 - 任务启动
- ✅ 任务选择完成
- ✅ 执行计划创建
- 🔄 等待架构学习

### 下一步计划
1. 阅读架构约束文档
2. 实现 CodeChangeTracker Agent
3. 编写单元测试
4. 运行 harness:check 验证
5. 更新文档并归档

---

## 🔗 相关文件

- 实现文件：`src-tauri/src/agent/code_change_tracker.rs`
- 模块导出：`src-tauri/src/agent/mod.rs`
- 命令注册：`src-tauri/src/commands/mod.rs`
- 类型定义：`src-tauri/src/agent/types.rs`
