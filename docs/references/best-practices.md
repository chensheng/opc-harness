# Harness Engineering 最佳实践

本文档收录了 OPC-HARNESS 项目开发过程中的最佳实践和经验教训。

## 📋 目录

1. [AI Agent 协作](#ai-agent-协作)
2. [上下文工程](#上下文工程)
3. [代码质量](#代码质量)
4. [调试技巧](#调试技巧)
5. [性能优化](#性能优化)

---

## AI Agent 协作

### 如何向 AI 提问

#### ✅ 好的提问方式
```markdown
**任务**: 实现用户认证功能

**上下文**:
- 位置：src-tauri/src/commands/auth.rs
- 已有：数据库连接、User 模型
- 需要：登录、注册、登出命令

**约束**:
- 使用 bcrypt 加密密码
- JWT token 有效期 7 天
- 错误信息使用中文

**示例代码风格**:
参考 src-tauri/src/commands/project.rs 的实现模式
```

#### ❌ 避免的提问方式
```
帮我写个登录功能
```

### AI 生成代码验证流程

```bash
# 1. 类型检查
npx tsc --noEmit

# 2. 代码规范检查
npm run lint

# 3. 格式化
npm run format

# 4. 架构健康检查
npm run harness:check

# 5. 手动审查关键点
- [ ] 错误处理是否完整
- [ ] 边界条件是否考虑
- [ ] 是否符合项目代码风格
```

### Prompt 模板

#### 功能开发模板
```markdown
## 任务描述
[清晰描述要实现的功能]

## 技术栈
- 前端：React + TypeScript
- 后端：Rust + Tauri
- 状态管理：Zustand

## 文件位置
- 组件：src/components/[模块]/[组件名].tsx
- 命令：src-tauri/src/commands/[模块].rs
- 类型：src/types/index.ts

## 接口定义
TypeScript 类型：
```typescript
interface DataType {
  // 定义数据结构
}
```

Rust 模型：
```rust
pub struct DataModel {
    // 定义数据结构
}
```

## 特殊要求
- [性能要求]
- [安全要求]
- [用户体验要求]
```

#### Bug 修复模板
```markdown
## 问题描述
[描述遇到的问题]

## 错误信息
```
粘贴完整的错误堆栈
```

## 重现步骤
1. [第一步]
2. [第二步]
3. [结果]

## 已尝试方案
- [x] 尝试过方案 A
- [x] 尝试过方案 B

## 环境信息
- Node.js: v18.x
- Rust: v1.70+
- 操作系统：Windows/macOS/Linux
```

---

## 上下文工程

### 决策记录 (ADR) 编写指南

每个重要架构决策都应记录 ADR，包含以下要素：

```markdown
# ADR-XXX: [标题]

**状态**: [提议 | 已采纳 | 废弃]
**日期**: YYYY-MM-DD
**作者**: [姓名]

## 背景与问题
说明为什么需要做这个决策

## 决策内容
具体决策是什么

## 技术影响
- 优势
- 劣势
- 权衡分析

## 实施策略
如何落地这个决策

## 最佳实践
- ✅ 推荐做法
- ❌ 避免做法

## 验证方法
如何验证决策是否正确执行
```

### 执行日志记录

关键操作的执行日志应包含：

```typescript
// ✅ 完整的日志记录
console.log('[INFO] 开始处理项目创建请求');
console.log('[PARAMS] 项目名称:', projectName, '| 描述长度:', description.length);

try {
  const result = await invoke('create_project', { name, description });
  console.log('[SUCCESS] 项目创建成功:', result.id);
  return result;
} catch (error) {
  console.error('[ERROR] 项目创建失败:', error);
  throw error;
}

// ❌ 避免：信息不足的日志
console.log('创建项目...');
const result = await invoke('create_project');
```

### 知识库维护

定期更新 `.harness/context-engineering/knowledge-base/` 目录：

1. **新增功能后**: 更新相关文档
2. **解决问题后**: 记录解决方案到 FAQ
3. **性能优化后**: 更新性能基准数据
4. **遇到陷阱后**: 添加注意事项

---

## 代码质量

### TypeScript 空值安全

#### ✅ 推荐模式
```tsx
// 1. 显式检查后操作
{(() => {
  const provider = providers.find(p => p.id === config.provider);
  if (!provider?.supportedModels) return null;
  return provider.supportedModels.map(item => (
    <Component key={item} />
  ));
})()}

// 2. 类型守卫
function isDefined<T>(value: T | undefined | null): value is T {
  return value !== undefined && value !== null;
}

const validItems = items.filter(isDefined);

// 3. 默认值
const count = items?.length ?? 0;
```

#### ❌ 避免模式
```tsx
// 仅依赖可选链（可能返回 undefined）
{providers.find(p => p.id === id)?.supportedModels.map(...)}

// 无条件渲染
{items.map(item => <Item key={item.id} {...item} />)}
// items 可能为 undefined
```

### Rust 错误处理

#### ✅ 推荐模式
```rust
// 1. 使用 Result 传递错误
#[tauri::command]
pub async fn create_project(
    name: String,
    description: String,
) -> Result<Project, String> {
    if name.trim().is_empty() {
        return Err("项目名称不能为空".to_string());
    }
    
    let project = Project::new(&name, &description)
        .map_err(|e| format!("创建项目失败：{}", e))?;
    
    Ok(project)
}

// 2. 提供友好的错误信息
match api_call.await {
    Ok(response) => Ok(response),
    Err(e) => Err(format!(
        "AI 服务调用失败：{}。请检查 API Key 配置和网络连接",
        e
    )),
}

// 3. 使用 ? 操作符简化
let file = File::open(path)?;
let content = read_to_string(file)?;
Ok(content)
```

#### ❌ 避免模式
```rust
// 1.  unwrap() 滥用
let file = File::open(path).unwrap(); // 可能 panic

// 2. 模糊的错误信息
Err("Error occurred".to_string()) // 用户不知道如何解决

// 3. 忽略错误
let _ = some_operation(); // 错误被丢弃
```

### ESLint 规则遵守

#### 常见警告及修复

**未使用的变量**
```typescript
// ❌ 警告
function processData(data: any, unusedParam: string) {
  return data.value;
}

// ✅ 修复
function processData(data: any, _unusedParam: string) {
  return data.value;
}
```

**缺少 await**
```typescript
// ❌ 警告
async function fetchData() {
  const result = fetchApi(); // Promise 未等待
  return result;
}

// ✅ 修复
async function fetchData() {
  const result = await fetchApi();
  return result;
}
```

---

## 调试技巧

### Tauri 命令调试

#### 前端日志
```typescript
import { invoke } from '@tauri-apps/api/core';

async function debugCommand() {
  console.group('[DEBUG] 调用 Tauri 命令');
  console.log('[REQUEST] command:', 'get_projects');
  console.log('[REQUEST] params:', { page: 1 });
  
  try {
    const result = await invoke('get_projects', { page: 1 });
    console.log('[RESPONSE]', result);
    console.groupEnd();
  } catch (error) {
    console.error('[ERROR]', error);
    console.groupEnd();
    throw error;
  }
}
```

#### 后端日志
```rust
use log::{info, error, debug};

#[tauri::command]
pub async fn get_projects(page: i32) -> Result<Vec<Project>, String> {
    debug!("接收请求：page = {}", page);
    
    match database.query(page).await {
        Ok(projects) => {
            info!("查询成功：返回 {} 条记录", projects.len());
            Ok(projects)
        }
        Err(e) => {
            error!("查询失败：{}", e);
            Err(format!("查询失败：{}", e))
        }
    }
}
```

### 性能问题分析

#### 前端性能
```typescript
// 使用 Performance API
const start = performance.now();
await heavyComputation();
const end = performance.now();
console.log(`耗时：${end - start}ms`);
```

#### 后端性能
```rust
use std::time::Instant;

let start = Instant::now();
let result = heavy_operation().await;
let duration = start.elapsed();
info!("操作耗时：{:?}", duration);
```

---

## 性能优化

### 前端优化

#### 1. 组件懒加载
```typescript
// 路由级别代码分割
const CodingWorkspace = lazy(() => 
  import('@/components/vibe-coding/CodingWorkspace')
);

function App() {
  return (
    <Suspense fallback={<LoadingSpinner />}>
      <Routes>
        <Route path="/coding" element={<CodingWorkspace />} />
      </Routes>
    </Suspense>
  );
}
```

#### 2. 列表虚拟化
```typescript
// 大列表使用虚拟滚动
import { useVirtualizer } from '@tanstack/react-virtual';

function VirtualList({ items }) {
  const parentRef = useRef(null);
  const virtualizer = useVirtualizer({
    count: items.length,
    getScrollElement: () => parentRef.current,
    estimateSize: () => 50,
  });
  
  return (
    <div ref={parentRef}>
      {virtualizer.getVirtualItems().map(item => (
        <div key={item.key} style={{ height: item.size }}>
          {renderItem(items[item.index])}
        </div>
      ))}
    </div>
  );
}
```

### 后端优化

#### 1. 数据库查询优化
```rust
// 使用索引
CREATE INDEX idx_project_status ON projects(status);

// 批量查询
let projects = sqlx::query_as::<_, Project>(
    "SELECT * FROM projects WHERE status = ? LIMIT ? OFFSET ?"
)
.bind(status)
.bind(limit)
.bind(offset)
.fetch_all(&pool)
.await?;
```

#### 2. 并发控制
```rust
use tokio::sync::Semaphore;
use std::sync::Arc;

let semaphore = Arc::new(Semaphore::new(10)); // 最多 10 个并发

let mut tasks = vec![];
for i in 0..100 {
    let permit = semaphore.clone().acquire_owned().await.unwrap();
    tasks.push(tokio::spawn(async move {
        process_item(i).await;
        drop(permit); // 释放许可
    }));
}
```

---

## 检查清单

### 提交前检查清单

```markdown
## 代码质量
- [ ] TypeScript 无类型错误
- [ ] ESLint 无警告
- [ ] Prettier 格式化通过
- [ ] Rust cargo check 通过

## 功能完整性
- [ ] 核心功能已测试
- [ ] 边界情况已处理
- [ ] 错误提示友好（中文）
- [ ] 加载状态正确显示

## 文档
- [ ] 更新了 AGENTS.md（如需要）
- [ ] 添加了必要的注释
- [ ] 记录了 ADR（架构变更时）

## 性能
- [ ] 无明显性能问题
- [ ] 大数据量测试通过
- [ ] 内存泄漏检查

## 安全
- [ ] 敏感数据加密存储
- [ ] API Key 不硬编码
- [ ] 输入验证完整
```

---

**维护者**: OPC-HARNESS Team  
**最后更新**: 2026-03-22  
**贡献**: 欢迎提交 PR 补充更多最佳实践
