# Vibe Coding 智能体创建功能

## 概述

Vibe Coding 模块的智能体监控面板现已支持创建新的 AI 智能体。用户可以通过直观的界面配置并启动不同类型的智能体来执行特定任务。

## 功能特性

### 1. 智能体类型支持

- **初始化智能体 (Initializer)** 🚀
  - 负责项目初始化和环境配置
  - 自动分解任务为 Issues
  - 检查开发环境依赖

- **编码智能体 (Coding)** 💻
  - 代码生成和修改
  - 实现具体功能需求
  - 遵循最佳实践和编码规范

- **MR 创建智能体 (MR Creation)** 🔀
  - 自动创建合并请求
  - 管理分支策略
  - 生成 MR 描述和变更总结

### 2. 用户界面

#### 创建对话框
- 智能体类型选择（带图标和描述）
- 会话 ID 输入（用于关联同一会话的多个智能体）
- 项目路径输入（智能体工作目录）
- 实时验证和错误提示
- 加载状态显示

#### 监控面板
- 新增"创建 Agent"按钮（位于右上角）
- 创建成功后自动刷新智能体列表
- 显示新创建智能体的初始状态

## 使用方法

### 步骤 1: 进入智能体监控面板
1. 打开 Vibe Coding 模块
2. 点击顶部标签栏的"智能体"标签

### 步骤 2: 创建智能体
1. 点击右上角的 **"创建 Agent"** 按钮
2. 在弹出的对话框中填写配置信息：
   - **智能体类型**: 从下拉列表选择需要的智能体类型
   - **会话 ID**: 输入会话标识（例如: `session-001`）
   - **项目路径**: 输入项目的绝对路径（例如: `/home/user/my-project`）
3. 点击 **"创建智能体"** 按钮

### 步骤 3: 查看和管理
- 创建成功后，新智能体会出现在监控面板中
- 可以查看智能体的状态、进度和资源使用情况
- 支持暂停、恢复、停止等操作

## 技术实现

### 前端组件

#### CreateAgentDialog.tsx
```typescript
interface CreateAgentDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  onSuccess?: (agentId: string) => void
}
```

**主要功能**:
- 表单验证（必填字段检查）
- 调用 Tauri `create_agent` 命令
- 错误处理和用户反馈
- 成功回调通知父组件

#### AgentMonitor.tsx
**新增内容**:
- `showCreateDialog` 状态管理
- `handleAgentCreated` 回调函数
- "创建 Agent"按钮
- CreateAgentDialog 组件集成

### 后端接口

**Tauri Command**: `create_agent`
```rust
pub async fn create_agent(
    state: State<'_, Arc<RwLock<AgentManager>>>,
    agent_type: String,
    session_id: String,
    project_path: String,
) -> Result<String, String>
```

**参数说明**:
- `agent_type`: 智能体类型 (`initializer` | `coding` | `mr_creation`)
- `session_id`: 会话 ID
- `project_path`: 项目路径

**返回值**: 创建的智能体 ID

## 示例场景

### 场景 1: 创建编码智能体
```
智能体类型: coding
会话 ID: session-feature-auth
项目路径: /projects/my-app
```
→ 用于实现用户认证功能

### 场景 2: 创建初始化智能体
```
智能体类型: initializer
会话 ID: session-new-project
项目路径: /projects/new-startup
```
→ 用于新项目的环境初始化和任务分解

### 场景 3: 创建 MR 智能体
```
智能体类型: mr_creation
会话 ID: session-release-v1
项目路径: /projects/production-app
```
→ 用于创建版本发布的合并请求

## 注意事项

1. **项目路径**: 必须是有效的文件系统路径，智能体将在该目录下工作
2. **会话 ID**: 建议使用有意义的命名，便于追踪和管理
3. **智能体类型**: 根据实际需求选择合适的智能体类型
4. **资源限制**: 同时运行的智能体数量受系统资源限制

## 故障排除

### 问题 1: 创建失败 - "项目路径不存在"
**解决方案**: 
- 确认输入的路径是绝对路径
- 检查路径是否存在且有访问权限
- Windows 用户使用反斜杠或双反斜杠（如 `C:\\Projects\\my-app`）

### 问题 2: 创建失败 - "未知的智能体类型"
**解决方案**:
- 确保从下拉列表中选择，不要手动输入
- 支持的类型: `initializer`, `coding`, `mr_creation`

### 问题 3: 智能体创建后未显示
**解决方案**:
- 刷新页面或点击"刷新状态"按钮
- 检查浏览器控制台是否有错误信息
- 确认后端服务正常运行

## 相关文件

- **前端组件**:
  - `src/components/vibe-coding/CreateAgentDialog.tsx` - 创建对话框
  - `src/components/vibe-coding/AgentMonitor.tsx` - 监控面板
  
- **后端命令**:
  - `src-tauri/src/agent/agent_manager_commands.rs` - Tauri 命令定义
  
- **类型定义**:
  - `src/components/vibe-coding/CodingWorkspaceTypes.ts` - AgentInfo 类型

## 未来改进

- [ ] 支持批量创建多个智能体
- [ ] 添加智能体模板功能（预设配置）
- [ ] 支持克隆已有智能体配置
- [ ] 增加智能体优先级和调度策略
- [ ] 提供智能体性能分析报告

---

**更新日期**: 2026-04-14  
**版本**: v0.1.0
