# OPC-HARNESS 系统架构

> **文档版本**: v3.1  
> **最后更新**: 2026-03-22  
> **架构类型**: Tauri v2 混合架构 (本地桌面 + 云端 AI)

## 🎯 架构概览

### 核心原则

| 原则 | 说明 | 实现策略 |
|------|------|---------|
| **AI 驱动** | AI Agent 自主完成编码任务，人类仅关键决策点审查 | 多会话编排 + HITL 检查点 |
| **多云支持** | 支持多家 AI厂商，Agent自主选择最优模型 | 统一 AI 适配层 + 多厂商 API |
| **本地优先** | 项目数据、代码、日志存储在本地文件系统 | SQLite + Git + 本地存储 |
| **质量门禁** | 多层次质量保障确保代码符合生产标准 | 代码检查 + 类型检查 + 单元测试 |
| **安全隔离** | API密钥本地加密存储，前后端严格分离 | OS Keychain + Tauri Commands |
| **透明可控** | AI 执行过程实时可见，关键决策需人工批准 | 实时日志 + 检查点审查 |

### 技术栈

```
前端：React 18 + TypeScript 5 + Tailwind CSS 3 + Zustand 4
后端：Rust + Tauri v2 + tokio + rusqlite
数据库：SQLite (元数据) + Git (代码版本控制)
AI 服务：OpenAI / Anthropic / Kimi / GLM
构建工具：Vite 5 (前端) + Cargo (Rust)
代码质量：ESLint + Prettier + cargo clippy
```

## 🏗️ 整体架构

```
┌─────────────────────────────────────────────────────────────┐
│                  🖥️ 桌面应用层 (Frontend)                    │
│  ┌──────────────────────────────────────────────────────┐   │
│  │  React + TypeScript + Tailwind CSS                   │   │
│  │  ├─ UI 界面与用户交互                                  │   │
│  │  ├─ 状态管理 (Zustand)                                │   │
│  │  ├─ 路由导航 (React Router)                           │   │
│  │  ├─ 代码编辑器 (Monaco Editor)                        │   │
│  │  └─ Vibe Coding 监控界面                              │   │
│  └──────────────────────────────────────────────────────┘   │
├─────────────────────────────────────────────────────────────┤
│                  🔧 Tauri Commands 层                        │
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐       │
│  │ AI Commands  │ │DB Commands   │ │CLI Commands  │       │
│  └──────────────┘ └──────────────┘ └──────────────┘       │
├─────────────────────────────────────────────────────────────┤
│                  🤖 Rust Services 层                        │
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐       │
│  │ AI Service   │ │DB Service    │ │Agent Service │       │
│  └──────────────┘ └──────────────┘ └──────────────┘       │
├─────────────────────────────────────────────────────────────┤
│                  ☁️ 云端 AI 服务层                            │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐      │
│  │ OpenAI   │ │Anthropic │ │ Kimi     │ │ GLM      │      │
│  └──────────┘ └──────────┘ └──────────┘ └──────────┘      │
├─────────────────────────────────────────────────────────────┤
│                  💾 本地数据存储层                          │
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐       │
│  │ SQLite DB    │ │ 项目文件     │ │ 执行日志     │       │
│  │ (元数据)     │ │ (代码/资源)  │ │ (Agent Logs) │       │
│  └──────────────┘ └──────────────┘ └──────────────┘       │
│  ┌──────────────────────────────────────────────┐          │
│  │ Git 仓库 (版本控制 + 回滚保障)                │          │
│  └──────────────────────────────────────────────┘          │
└─────────────────────────────────────────────────────────────┘
```

## 📐 分层架构详解

### 1. 前端层 (src/)

#### 组件分层
```
src/
├── components/
│   ├── ui/              # shadcn/ui基础组件 (不可修改)
│   ├── common/          # 通用组件 (Layout, Header, Sidebar)
│   ├── vibe-design/     # Vibe Design 业务组件
│   ├── vibe-coding/     # Vibe Coding 业务组件
│   └── vibe-marketing/  # Vibe Marketing 业务组件
├── hooks/               # 自定义 Hooks (封装可复用逻辑)
├── stores/              # Zustand Stores (全局状态)
├── types/               # TypeScript 类型定义
└── lib/                 # 工具函数 (纯函数)
```

#### 数据流规则
```typescript
// ✅ 推荐：单向数据流
Component → Store → Tauri Commands → Rust Backend
     ↑                                       |
     └─────────── State Update ←─────────────┘

// ❌ 禁止：
- 组件直接调用 invoke() - 必须通过 stores 封装
- Store 直接操作 DOM
- 循环依赖
```

### 2. Tauri Commands 层 (src-tauri/src/commands/)

#### 职责
- 参数验证和转发
- 错误处理和格式化
- **不包含业务逻辑**

#### 示例
```rust
#[tauri::command]
pub async fn create_project(
    name: String,
    description: Option<String>,
) -> Result<Project, String> {
    // 参数验证
    if name.is_empty() {
        return Err("项目名称不能为空".to_string());
    }
    
    // 委托给 Service 层
    services::create_project(name, description)
        .await
        .map_err(|e| format!("创建失败：{}", e))
}
```

### 3. Services 层 (src-tauri/src/services/)

#### 核心服务
- **AI Service**: AI厂商集成和调用
- **DB Service**: 数据库 CRUD 操作
- **Agent Service**: AI Agent 编排和执行

#### 业务逻辑
```rust
pub async fn create_project(
    name: String,
    description: Option<String>,
) -> Result<Project, AppError> {
    // 业务验证
    validate_project_name(&name)?;
    
    // 创建项目
    let project = Project {
        id: generate_uuid(),
        name,
        description,
        status: "idea".to_string(),
        created_at: now(),
        ..Default::default()
    };
    
    // 保存到数据库
    db::save_project(&project).await?;
    
    Ok(project)
}
```

### 4. Models 层 (src-tauri/src/models/)

#### 数据模型
```rust
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub progress: i32,
    pub created_at: String,
    pub updated_at: String,
}
```

#### 序列化规范
- Rust 字段：`snake_case`
- JSON 输出：`camelCase`
- 使用 `#[serde(rename_all = "camelCase")]`

## 🔄 核心流程

### Vibe Design 流程
```
用户输入想法
    ↓
选择 AI厂商和模型
    ↓
生成 PRD 文档
    ↓
生成用户画像
    ↓
生成竞品分析
    ↓
保存到项目
```

### Vibe Coding 流程
```
读取 PRD 文档
    ↓
Initializer Agent 分解任务
    ↓
HITL CP-002: Issue 质量审查
    ↓
Coding Agents 并行开发 (4+ 并发)
    ↓
HITL CP-006: Issue 完成审查
    ↓
MR Creation Agent 创建合并请求
    ↓
质量门禁检查
    ↓
HITL CP-010: MR 审查
    ↓
合并到主分支
```

## 🔒 安全架构

### API密钥管理
```rust
// 使用 OS Keychain 安全存储
use keyring::Entry;

pub fn save_api_key(provider: &str, key: &str) -> Result<(), Error> {
    let entry = Entry::new("opc-harness", provider)?;
    entry.set_password(key)?;
    Ok(())
}

pub fn get_api_key(provider: &str) -> Result<String, Error> {
    let entry = Entry::new("opc-harness", provider)?;
    entry.get_password()
}
```

### 前后端隔离
- 前端无法直接访问数据库
- 所有数据访问通过 Tauri Commands
- Rust 后端完全控制资源访问

## 📊 数据库设计

### 核心表结构

#### projects 表
```sql
CREATE TABLE projects (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL DEFAULT 'idea',
    progress INTEGER NOT NULL DEFAULT 0,
    prd_document TEXT,
    user_personas TEXT,
    competitor_analysis TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
```

#### ai_configs 表
```sql
CREATE TABLE ai_configs (
    id TEXT PRIMARY KEY,
    provider TEXT NOT NULL UNIQUE,
    model TEXT NOT NULL,
    api_key_encrypted TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
```

## 🧪 质量保障

### 多层质量门禁

#### 1. 代码规范检查
```bash
npm run lint           # ESLint
cargo clippy           # Rust lint
```

#### 2. 类型检查
```bash
tsc --noEmit          # TypeScript
cargo check           # Rust 编译检查
```

#### 3. 测试覆盖
```bash
npm run test          # Vitest (前端)
cargo test            # Rust 测试
```

#### 4. 格式化检查
```bash
npm run format:check  # Prettier
cargo fmt --check     # rustfmt
```

## 🚀 性能优化

### 启动时间优化
- **目标**: < 3 秒
- **策略**: 
  - Rust 后端预编译
  - 前端代码分割
  - 懒加载组件

### AI 响应优化
- **目标**: < 100ms 延迟
- **策略**:
  - SSE 流式输出
  - 请求缓存
  - 智能预加载

### 内存优化
- **目标**: < 500MB
- **策略**:
  - 组件按需加载
  - 状态持久化
  - 垃圾回收机制

## 📈 可扩展性

### 插件系统 (规划中)
```rust
pub trait Plugin {
    fn name(&self) -> &str;
    fn initialize(&mut self) -> Result<(), Error>;
    fn execute(&mut self, context: &Context) -> Result<(), Error>;
}
```

### AI厂商扩展
```rust
// 新增 AI厂商只需实现 AIProvider trait
impl AIProvider for NewProvider {
    fn chat(&self, request: ChatRequest) -> Result<ChatResponse, Error> {
        // 实现具体调用逻辑
    }
}
```

## 📖 参考文档

- [前端开发规范](./src/AGENTS.md)
- [Rust 后端规范](./src-tauri/AGENTS.md)
- [设计文档索引](./docs/design-docs/)
- [产品规范索引](./docs/product-specs/)

---

**最后更新**: 2026-03-22  
**维护者**: OPC-HARNESS Team  
**架构状态**: ✅ MVP 阶段完成
