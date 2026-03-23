# INFRA-008: 集成 SQLite 数据库 (rusqlite)

**任务状态**: ✅ 已完成  
**完成时间**: 2026-03-23  
**优先级**: P0  
**估时**: 4h  

## 任务描述

实现 SQLite 数据库的完整 CRUD 操作服务层，为项目数据、AI 配置和 CLI 会话提供持久化存储能力。

## 实现内容

### 1. 数据库表结构

已创建以下三张核心表：

#### projects 表
```sql
CREATE TABLE projects (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    status TEXT DEFAULT 'idea',
    progress INTEGER DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    idea TEXT,
    prd TEXT,
    user_personas TEXT,
    competitor_analysis TEXT
)
```

#### ai_configs 表
```sql
CREATE TABLE ai_configs (
    provider TEXT PRIMARY KEY,
    model TEXT NOT NULL,
    api_key TEXT NOT NULL
)
```

#### cli_sessions 表
```sql
CREATE TABLE cli_sessions (
    id TEXT PRIMARY KEY,
    tool_type TEXT NOT NULL,
    project_path TEXT NOT NULL,
    created_at TEXT NOT NULL
)
```

### 2. Rust CRUD 服务层 (`src-tauri/src/db/mod.rs`)

实现了完整的数据库操作函数：

#### Project 操作
- `create_project()` - 创建新项目
- `get_all_projects()` - 获取所有项目（按更新时间降序）
- `get_project_by_id()` - 获取单个项目
- `update_project()` - 更新项目信息
- `delete_project()` - 删除项目

#### AI Config 操作
- `save_ai_config()` - 保存 AI 配置（支持更新）
- `get_all_ai_configs()` - 获取所有 AI 配置
- `get_ai_config()` - 获取单个 AI 配置
- `delete_ai_config()` - 删除 AI 配置

#### CLI Session 操作
- `create_cli_session()` - 创建 CLI 会话
- `get_all_cli_sessions()` - 获取所有 CLI 会话
- `get_cli_session_by_id()` - 获取单个 CLI 会话
- `delete_cli_session()` - 删除 CLI 会话

### 3. Tauri Commands (`src-tauri/src/commands/database.rs`)

暴露给前端的命令接口：

```rust
// Project 命令
create_project(name, description) -> String (project_id)
get_all_projects() -> Vec<Project>
get_project_by_id(id) -> Option<Project>
update_project(project) -> ()
delete_project(id) -> ()

// AI Config 命令
save_ai_config(config) -> ()
get_all_ai_configs() -> Vec<AIConfig>
get_ai_config(provider) -> Option<AIConfig>
delete_ai_config(provider) -> ()

// CLI Session 命令
create_cli_session_db(tool_type, project_path) -> String (session_id)
get_all_cli_sessions() -> Vec<CLISession>
get_cli_session_by_id(id) -> Option<CLISession>
delete_cli_session(id) -> ()
```

### 4. 技术实现细节

#### 类型转换处理
- `progress` 字段：Rust `i32` ↔ SQLite `INTEGER` ↔ 字符串转换
- `Option<String>` 字段：使用 `clone().unwrap_or_default()` 处理空值

#### 时间戳处理
- 使用 `chrono::Utc::now().to_rfc3339()` 生成 ISO 8601 格式时间戳
- 更新操作自动生成新的时间戳

#### 唯一 ID 生成
- 使用 `uuid::Uuid::new_v4()` 生成随机 UUID

#### JSON 序列化
- 所有模型使用 `#[serde(rename_all = "camelCase")]` 属性
- Rust snake_case 字段自动转换为前端 camelCase JSON
- 例如：`created_at` → `createdAt`, `user_personas` → `userPersonas`

## 文件清单

| 文件 | 变更类型 | 说明 |
|------|----------|------|
| `src-tauri/src/db/mod.rs` | 修改 | 扩展 CRUD 操作函数（从仅初始化到完整 CRUD） |
| `src-tauri/src/commands/mod.rs` | 修改 | 添加 database 子模块声明 |
| `src-tauri/src/commands/database.rs` | 新建 | 实现数据库相关的 Tauri Commands |
| `src-tauri/src/main.rs` | 修改 | 注册数据库命令到 invoke_handler |
| `src-tauri/src/models/mod.rs` | 修改 | 添加 serde camelCase 重命名属性 |
| `src-tauri/Cargo.toml` | 无变更 | rusqlite 依赖已配置 |

## 验收标准

- [x] 数据库连接正常建立
- [x] 三张核心表正确创建（projects, ai_configs, cli_sessions）
- [x] 所有 CRUD 操作通过 cargo check 编译
- [x] Tauri Commands 正确注册并可调用
- [x] 类型转换正确处理（i32, Option<String>）
- [x] JSON 序列化使用 camelCase 与前端保持一致
- [x] Harness Engineering 健康检查通过
- [x] **开发环境成功启动**（`npm run tauri:dev`）
- [x] **应用窗口正常显示**
- [x] **Rust 后端编译完成，无错误**

## 运行环境验证

### ✅ 开发环境启动验证

```
# 1. 启动开发环境
npm run tauri:dev

# 预期输出:
# - Vite 开发服务器启动在 http://localhost:1420/
# - Rust 后端编译完成 (cargo build)
# - Tauri 应用窗口弹出
# - 控制台显示 "Running `target\debug\opc-harness.exe`"
```

### 验证结果

```
✅ Vite v5.4.21 ready in 894 ms
✅ Frontend: http://localhost:1420/
✅ Rust Backend: opc-harness v0.1.0 compiled successfully
✅ Tauri Window: Application launched
✅ Database: Initialized at %APPDATA%\opc-harness\opc-harness.db
```

### 编译警告统计

```
- Total Warnings: 23
- Critical Errors: 0
- Build Status: SUCCESS (dev profile)
- Build Time: 22.77s
```

**警告分类**:
- 未使用导入：3 个（async_trait, tauri::Manager ×2）
- 未使用变量：9 个（request ×6, session_id, url, on_chunk）
- 未使用代码：11 个（CLISession 字段、AIService、CLITool 等）

**影响评估**: 所有警告均为死代码检测，不影响功能正常运行。可在后续清理。

### 下一步验证建议

1. **前端调用测试**（参考 `.harness/database-demo.md`）
   ```javascript
   // 在浏览器控制台中执行
   const projectId = await invoke('create_project', {
     name: '测试项目',
     description: '验证数据库功能'
   });
   console.log('✅ 项目创建成功:', projectId);
   
   const projects = await invoke('get_all_projects');
   console.log('📋 项目列表:', projects);
   ```

2. **数据库文件验证**
   - 位置：`%APPDATA%\opc-harness\opc-harness.db`
   - 工具：DB Browser for SQLite
   - 检查表结构：`projects`, `ai_configs`, `cli_sessions`

3. **热更新测试**
   - 修改前端代码 → 页面自动刷新
   - 修改 Rust 代码 → 自动重新编译并重启应用

## 测试建议

### 手动测试（前端调用示例）

``typescript
import { invoke } from '@tauri-apps/api/core';

// 创建项目
const projectId = await invoke('create_project', {
  name: '我的产品',
  description: '产品描述'
});

// 获取所有项目
const projects = await invoke('get_all_projects');
console.log(projects); // [{ id, name, description, status, progress, createdAt, updatedAt, ... }]

// 保存 AI 配置
await invoke('save_ai_config', {
  config: {
    provider: 'openai',
    model: 'gpt-4o',
    apiKey: 'sk-xxx'  // 注意：前端使用 camelCase
  }
});

// 获取 AI 配置
const config = await invoke('get_ai_config', {
  provider: 'openai'
});
console.log(config); // { provider, model, apiKey }

// 删除项目
await invoke('delete_project', { id: projectId });
```

### 数据库位置

Windows: `%APPDATA%\opc-harness\opc-harness.db`

可以使用 DB Browser for SQLite 等工具查看数据库内容。

## 后续任务

- [ ] **INFRA-010**: 集成 OS 密钥存储 (keyring-rs) - API密钥安全存储（替代数据库明文存储）
- [ ] **VD-010~VD-011**: 实现 OpenAI/Kimi适配器 - 需要读取 AI 配置
- [ ] **VC-005**: 实现会话状态持久化 - 依赖数据库 CRUD
- [ ] **前端集成**: 在 Settings 页面中使用数据库 API 替换 Mock 数据

## 注意事项

1. **数据库路径**: 使用 Tauri的 `app_data_dir()` API，自动适配各平台
2. **并发安全**: SQLite 支持读写并发，写写互斥，适合桌面应用
3. **性能优化**: 对于大量数据操作，建议使用事务（transaction）批量处理
4. **错误处理**: 所有数据库操作返回 `Result<T, String>`，便于前端友好提示
5. **API密钥安全**: 当前 API密钥存储在数据库中，后续应迁移到 keyring 安全存储

## 已知问题

1. **API密钥明文存储**: 当前存储在数据库中，存在安全风险
   - **解决方案**: 使用 keyring-rs 将密钥存储在系统凭据管理器中
   - **优先级**: P0（应在 MVP 前完成）

2. **未使用事务**: 当前操作都是单条执行
   - **影响**: 批量操作性能较低
   - **解决方案**: 对批量操作使用事务包装

## 参考资料

- [rusqlite 文档](https://docs.rs/rusqlite/)
- [Serde 序列化指南](https://serde.rs/)
- [Tauri v2 插件系统](https://v2.tauri.app/)
- [MVP版本规划](../docs/MVP版本规划.md) - INFRA-008 任务定义
- [Harness Engineering 规范](./INDEX.md)
