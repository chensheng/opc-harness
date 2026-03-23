# Harness Engineering 最佳实践

> **基于 OpenAI 官方最佳实践**  
> 适用范围：OPC-HARNESS 项目全体开发者和 AI Agent  
> 最后更新：2026-03-23

## 🎯 核心理念

**Harness Engineering** 是一套让 AI Agent 更好地协助你开发项目的工程实践体系。

**核心理念**: "人类掌舵，Agent 执行" (Humans steer. Agents execute.)

### 三大支柱

1. **上下文工程 (Context Engineering)** - 帮助 AI 快速理解项目
2. **架构约束 (Architectural Constraints)** - 确保代码符合规范
3. **反馈回路 (Feedback Loops)** - 快速发现问题并持续改进

---

## 📖 如何向 AI 提问

### ✅ 好的提问方式

```markdown
**任务**: 实现用户登录功能

**上下文**:
- 位置：src/components/auth/Login.tsx
- 已有：数据库连接、User 模型
- 需要：登录、注册、登出组件

**约束**:
- 使用 bcrypt 加密密码
- JWT token 有效期 7 天
- 错误信息使用中文
- 遵循 src/AGENTS.md 规范

**参考示例**:
参考 src/components/common/Settings.tsx 的实现模式
```

### ❌ 避免的提问方式

```
帮我写个登录功能
```

---

## 🧪 AI 生成代码验证流程

```bash
# 1. 类型检查
npx tsc --noEmit

# 2. 代码规范检查
npm run lint

# 3. 格式化
npm run format

# 4. 架构健康检查（强烈推荐）
npm run harness:check

# 5. 运行单元测试
npm run test:unit

# 6. 手动审查关键点
- [ ] 错误处理是否完整
- [ ] 边界条件是否考虑
- [ ] 是否符合项目代码风格
```

---

## 📝 代码质量最佳实践

### TypeScript 空值安全

#### ✅ 推荐模式

```typescript
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

// 3. 使用默认值
const count = items?.length ?? 0;
```

#### ❌ 避免模式

```typescript
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
    description: Option<String>,
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
// 1. unwrap() 滥用
let file = File::open(path).unwrap(); // 可能 panic

// 2. 模糊的错误信息
Err("Error occurred".to_string()) // 用户不知道如何解决

// 3. 忽略错误
let _ = some_operation(); // 错误被丢弃
```

---

## 🏗️ 架构约束遵守

### 前端数据流规则

```typescript
// ✅ 推荐：单向数据流
Component → Store → Commands → Services → DB
     ↑                                       |
     └─────────── State Update ←─────────────┘

// ❌ 禁止：
- 组件直接调用 invoke() - 必须通过 stores 封装
- Store 直接操作 DOM
- 循环依赖：stores → components → stores
```

### 后端分层规则

```rust
// ✅ 允许：
Commands → Services → Models → DB
                ↑
            Business Logic

// ❌ 禁止：
- Commands 包含复杂业务逻辑
- Services 直接返回前端（必须通过 Commands）
- 循环依赖：commands ↔ services
```

---

## 📋 提交前检查清单

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

## 测试覆盖
- [ ] 新增代码有对应测试
- [ ] 单元测试覆盖率 >= 70%
- [ ] E2E 测试通过

## 文档
- [ ] 更新了必要的注释
- [ ] 记录了架构决策（如需要）
- [ ] 更新了最佳实践（如有新经验）

## 性能和安全
- [ ] 无明显性能问题
- [ ] 敏感数据加密存储
- [ ] API Key 不硬编码
- [ ] 输入验证完整
```

---

## 🔧 常见陷阱与解决方案

### 陷阱 1: 在组件中直接调用 Tauri API

```typescript
// ❌ 错误
function MyComponent() {
  const handleSave = async () => {
    await invoke('save_project', { data });
  };
}

// ✅ 正确
// stores/projectStore.ts
export const useProjectStore = create((set) => ({
  saveProject: async (data) => {
    await invoke('save_project', { data });
  }
}));

// components/MyComponent.tsx
function MyComponent() {
  const { saveProject } = useProjectStore();
  const handleSave = () => saveProject(data);
}
```

### 陷阱 2: 在 Commands 中包含业务逻辑

```rust
// ❌ 错误
#[tauri::command]
pub fn create_project(name: String) -> Result<Project, String> {
    // 直接在命令中操作数据库
    let conn = get_connection()?;
    let project = Project { name, .. };
    conn.execute(...)  // 复杂的 SQL 逻辑
}

// ✅ 正确
#[tauri::command]
pub fn create_project(name: String) -> Result<Project, String> {
    // 仅做参数验证和错误处理
    services::create_project(name).await
}
```

### 陷阱 3: 过度使用 useEffect

```typescript
// ❌ 错误：不必要的 useEffect
const [count, setCount] = useState(0);
useEffect(() => {
  document.title = `Count: ${count}`;
}, [count]);

// ✅ 正确：直接在事件处理中更新
const handleClick = () => {
  setCount(c => c + 1);
  document.title = `Count: ${count + 1}`;
};
```

---

## 🎓 学习路径

### 新手入门（1 小时）
1. ✅ 阅读根目录 AGENTS.md - 10 分钟
2. ✅ 浏览 src/AGENTS.md 或 src-tauri/AGENTS.md - 20 分钟
3. ✅ 运行 `npm run harness:check` 并理解输出 - 10 分钟
4. ✅ 阅读本文档 - 20 分钟

### 进阶提升（1 天）
1. 📖 精读 ARCHITECTURE.md
2. 📝 学习所有架构决策记录
3. 🔧 尝试编写自定义检查规则
4. 📚 贡献新的最佳实践

### 专家级别（1 周）
1. 🏗️ 深入理解 Harness Engineering 理念
2. 🤖 优化 AI 协作流程
3. 📊 建立团队的质量文化
4. 🌟 向社区分享经验

---

## 🔗 相关资源

### 官方文档
- [OpenAI Harness Engineering](https://openai.com/index/harness-engineering/)
- [本项目导航地图](../AGENTS.md)
- [前端开发规范](../src/AGENTS.md)
- [Rust 后端规范](../src-tauri/AGENTS.md)

### 学习材料
- [架构决策记录 (ADR) 指南](https://adr.github.io/)
- [TypeScript 严格模式](https://www.typescriptlang.org/tsconfig#strict)
- [Rust 编码规范](https://rust-lang.github.io/api-guidelines/)

### 工具链
- [ESLint - 代码规范检查](https://eslint.org/)
- [Prettier - 代码格式化](https://prettier.io/)
- [cargo - Rust 包管理](https://doc.rust-lang.org/cargo/)
- [Vitest - 单元测试框架](https://vitest.dev/)

---

**维护者**: OPC-HARNESS Team  
**版本**: 2.0.0 (基于 OpenAI Harness Engineering 最佳实践重构)  
**最后更新**: 2026-03-23  
**贡献**: 欢迎提交 PR 补充更多最佳实践
