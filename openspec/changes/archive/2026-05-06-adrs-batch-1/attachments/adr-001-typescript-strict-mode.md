# ADR-001: 启用 TypeScript 严格模式

**状态**: 已采纳  
**日期**: 2026-03-22  
**作者**: OPC-HARNESS Team  
**优先级**: 高

## 背景与问题

在 TypeScript 项目中，默认的宽松类型检查会导致：

1. 运行时错误无法在编译时发现
2. `any`类型滥用降低类型安全性
3. 未初始化的属性可能导致 undefined 错误
4. AI Agent 生成代码时可能忽略边界情况

## 决策内容

启用 TypeScript 严格模式（strict mode），包括以下配置：

```json
{
  "compilerOptions": {
    "strict": true, // 启用所有严格类型检查选项
    "noImplicitAny": true, // 禁止隐式 any 类型
    "strictNullChecks": true, // 严格 null 检查
    "strictFunctionTypes": true, // 严格函数类型检查
    "strictBindCallApply": true, // 严格 bind/call/apply 检查
    "strictPropertyInitialization": true, // 严格属性初始化检查
    "noImplicitThis": true, // 禁止隐式 this
    "useUnknownInCatchVariables": true // catch 变量默认为 unknown
  }
}
```

## 技术影响

### ✅ 优势

- **提前发现错误**: 编译时发现更多潜在问题
- **更好的文档**: 类型定义更清晰，减少注释需求
- **AI 友好**: AI Agent 能更好地理解代码意图
- **重构安全**: 大规模重构时有更强的类型保障

### ⚠️ 劣势

- **学习曲线**: 新成员需要适应更严格的类型要求
- **开发速度**: 初期开发速度可能略慢
- **样板代码**: 需要更多的类型声明

### 📊 权衡分析

| 方案            | 类型安全   | 开发效率   | 维护成本   | AI 理解度  |
| --------------- | ---------- | ---------- | ---------- | ---------- |
| 严格模式 (推荐) | ⭐⭐⭐⭐⭐ | ⭐⭐⭐     | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| 宽松模式        | ⭐⭐       | ⭐⭐⭐⭐⭐ | ⭐⭐       | ⭐⭐⭐     |

## 实施策略

### 阶段 1: 配置文件更新

更新 `tsconfig.json`:

```json
{
  "compilerOptions": {
    "strict": true
  }
}
```

### 阶段 2: 现有代码迁移

对现有代码进行渐进式改造：

1. 优先修复核心业务逻辑的类型错误
2. 逐步消除 `any` 类型使用
3. 为可选属性添加明确的 `undefined` 处理

### 阶段 3: 持续集成

在 CI/CD 流水线中添加类型检查：

```bash
npx tsc --noEmit
```

## 最佳实践

### ✅ 推荐模式

```typescript
// 1. 明确的类型定义
interface User {
  id: string
  name: string
  email?: string // 可选属性明确标记
}

// 2. 类型守卫处理联合类型
function processValue(value: string | number) {
  if (typeof value === 'string') {
    return value.toUpperCase()
  }
  return value.toFixed(2)
}

// 3. 非空断言仅在确定时使用
const element = document.getElementById('app')!

// 4. 使用类型推断减少冗余
const users: User[] = [] // 显式类型注解
```

### ❌ 避免模式

```typescript
// 1. 滥用 any 类型
function processData(data: any) {
  // ❌
  return data.value
}

// 2. 忽略可选链
const email = user.contact.email // ❌ 可能抛出错误

// 3. 类型断言过度
const element = document.getElementById('app') as HTMLElement // 冗余

// 4. 隐式 any
const callback = item => item.id // ❌ 参数缺少类型
```

## 验证方法

运行以下命令验证配置是否正确：

```bash
npm run lint
npx tsc --noEmit
npm run harness:check
```

## 相关链接

- [TypeScript Strict Mode 官方文档](https://www.typescriptlang.org/tsconfig#strict)
- [ADR-002: Zustand 状态管理](./adr-002-zustand-state-management.md)
- [架构约束规则](../../constraints/architecture-rules.md)

---

**变更记录**:

- 2026-03-22: 初始版本
