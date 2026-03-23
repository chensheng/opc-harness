# 📖 OPC-HARNESS 完整测试指南

> **详细的测试体系文档**  
> 基于 OpenAI Harness Engineering 最佳实践

---

## 🎯 测试金字塔

```
           /\
          / E2E \       ~10% (用户工作流)
         /--------\     
        /Integration\    ~20% (模块间通信)
       /------------\   
      /   Unit Tests  \  ~70% (组件、函数、逻辑)
     /------------------\
    / Architecture Tests \ (约束验证、依赖检查)
   /----------------------\
  /  Documentation Checks  \ (一致性、注释漂移)
 /--------------------------\
/    Dead Code Detection     \ (垃圾回收、熵减)
------------------------------
```

### 核心原则

1. **自动化反馈回路** - 每次代码变更都触发自动验证
2. **确定性优先** - 使用 Linter 和规则强制执行，而非依赖 AI 判断
3. **分层隔离** - 下层测试快速定位问题，上层测试验证整体行为
4. **持续清理** - 定期运行垃圾回收，对抗代码腐烂

---

## 🧪 单元测试

### 技术栈

- **框架**: [Vitest](https://vitest.dev/) (与 Vite 深度集成)
- **React 测试**: [@testing-library/react](https://testing-library.com/react)
- **断言**: Jest DOM
- **覆盖率**: V8 coverage

### 编写测试示例

#### 组件测试

```typescript
// src/components/ui/button.test.tsx
import { describe, it, expect } from 'vitest'
import { render, screen, fireEvent } from '@testing-library/react'
import { Button } from '../button'

describe('Button', () => {
  it('should render button with text', () => {
    render(<Button>Click Me</Button>)
    const button = screen.getByRole('button', { name: /click me/i })
    expect(button).toBeInTheDocument()
  })

  it('should handle click events', () => {
    const handleClick = vi.fn()
    render(<Button onClick={handleClick}>Click Me</Button>)
    
    fireEvent.click(screen.getByRole('button'))
    expect(handleClick).toHaveBeenCalledTimes(1)
  })
})
```

#### Store 测试

```typescript
// src/stores/appStore.test.ts
import { describe, it, expect, beforeEach } from 'vitest'
import { act, renderHook } from '@testing-library/react'
import { useAppStore } from './appStore'

describe('useAppStore', () => {
  beforeEach(() => {
    // 重置状态
    const { result } = renderHook(() => useAppStore())
    act(() => {
      result.current.setSettings({ theme: 'system' })
    })
  })

  it('should update settings', () => {
    const { result } = renderHook(() => useAppStore())
    
    act(() => {
      result.current.setSettings({ theme: 'dark' })
    })
    
    expect(result.current.settings.theme).toBe('dark')
  })
})
```

### 最佳实践

✅ **推荐**:
- 使用 `describe` 组织相关测试
- 测试文件名使用 `.test.ts/.tsx`
- 每个测试只验证一个行为
- 使用有意义的测试名称：`it('should return empty array when no items exist')`

❌ **避免**:
- 测试之间相互依赖
- 过于复杂的 mock
- 测试实现细节而非行为

---

## 🌐 E2E 测试

### 技术栈

- **框架**: [Playwright](https://playwright.dev/)
- **浏览器**: Chromium, Firefox, WebKit
- **报告**: HTML + JSON

### 编写 E2E 测试

```typescript
// e2e/app.spec.ts
import { test, expect } from '@playwright/test'

test.describe('OPC-HARNESS Application', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/')
  })

  test('should load successfully', async ({ page }) => {
    await expect(page).toHaveTitle(/OPC-HARNESS/)
    await expect(page.getByText(/Dashboard/i)).toBeVisible()
  })

  test('should navigate to Idea page', async ({ page }) => {
    await page.getByText(/Idea/i).click()
    await expect(page).toHaveURL(/idea/)
    await expect(page.locator('textarea')).toBeVisible()
  })
})
```

### 调试 E2E 测试

```bash
# UI 模式（推荐调试）
npm run test:e2e:ui

# 有头模式（看到浏览器）
npm run test:e2e:headed

# 查看测试报告
npm run test:e2e:report
```

---

## 🏛️ 架构约束测试

### 目的

基于 OpenAI Harness Engineering 实践，**强制执行架构规则**，防止：
- 循环依赖
- 跨层调用
- 模块边界违规

### 运行检查

```bash
# 作为单元测试的一部分运行
npm run test:run tests/architecture

# 或在 harness:check 中自动运行
npm run harness:check
```

### 检查项

#### 1. 分层依赖规则

```
✅ 允许：Frontend → Commands → Services → Database
❌ 禁止：
  - 前端直接访问数据库
  - Service 依赖 UI 组件
  - 循环依赖
```

#### 2. 模块边界

- `components/` - 纯 UI，无业务逻辑
- `stores/` - 状态管理，无直接 API 调用
- `commands/` - 仅参数转发，无复杂逻辑
- `services/` - 业务逻辑，无 UI 依赖

---

## 📚 文档一致性检查

### 目的

防止"注释漂移"（Documentation Drift）- 代码已改，注释未改。

### 运行检查

```bash
npm run harness:doc:check
```

### 检查内容

1. **链接有效性** - AGENTS.md 中的内部链接是否断裂
2. **TODO/FIXME追踪** - 识别过期的技术债务
3. **ADR 状态** - 架构决策记录是否更新
4. **注释同步** - 代码注释是否与实现一致

### 输出示例

```
========================================
  Documentation Consistency Check
========================================

[1/4] Checking AGENTS.md Links...
  [WARN] Broken link in AGENTS.md : ./docs/missing-file.md

[2/4] Checking Code Comments...
  [INFO] appStore.ts:45 - TODO: Add error handling

[3/4] Checking Architecture Decision Records...
  [INFO] Found 5 ADRs

[4/4] Checking Product Documentation Sync...
  [INFO] MVP版本规划.md - Last updated 120 days ago
```

---

## 🗑️ 垃圾回收

### 目的

基于 OpenAI Harness Engineering 的"反熵增"理念，定期清理：
- 未使用的导入
- 死代码（未使用的函数、组件、类型）
- 过期的模拟数据
- 陈旧的技术债务

### 运行垃圾回收

```bash
# 预览模式（推荐先使用）
npm run harness:dead:code:dry

# 实际清理
npm run harness:dead:code

# 完整 GC（包含文档清理）
npm run harness:gc
```

### 检测内容

1. **Unused Imports** - 导入但未使用的标识符
2. **Unused Functions** - 定义但未调用的函数
3. **Unused Types** - 声明但未使用的类型接口
4. **Stale TODOs** - 长期未处理的 TODO 注释

---

## 🔄 CI/CD 集成

### GitHub Actions 模板

```yaml
name: Test Suite

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Node.js
        uses: actions/setup-node@v3
        with:
          node-version: '18'
      
      - name: Install dependencies
        run: npm ci
      
      - name: Run architecture health check
        run: npm run harness:check
      
      - name: Run unit tests with coverage
        run: npm run test:coverage
      
      - name: Run E2E tests
        run: npm run test:e2e
      
      - name: Upload coverage report
        uses: codecov/codecov-action@v3
        with:
          files: ./coverage/lcov.info
```

### 质量门禁

| 指标 | 阈值 | 执行动作 |
|------|------|----------|
| 单元测试覆盖率 | >= 70% | 低于阈值警告 |
| E2E 测试通过率 | 100% | 失败则阻断 |
| 架构健康评分 | >= 90 | 低于 70 阻断 |
| 文档一致性 | 无断裂链接 | 警告但不阻断 |
| 死代码数量 | <= 10 | 超过阈值警告 |

---

## 📖 最佳实践

### Arrange-Act-Assert 模式

```typescript
it('should update counter on increment', () => {
  // Arrange
  const { result } = renderHook(() => useCounter(0))
  
  // Act
  act(() => {
    result.current.increment()
  })
  
  // Assert
  expect(result.current.count).toBe(1)
})
```

### Mock 外部依赖

```typescript
// Mock Tauri APIs
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn().mockResolvedValue({ data: 'mocked' }),
}))

// Mock API calls
vi.mock('axios', () => ({
  default: {
    get: vi.fn().mockResolvedValue({ data: [] }),
  },
}))
```

### 测试隔离

```typescript
beforeEach(() => {
  // 重置状态
  localStorage.clear()
})

afterEach(() => {
  // 清理资源
  vi.clearAllMocks()
})
```

---

## 🎯 持续改进计划

### 每周检查清单

- [ ] 运行 `npm run harness:test:full`
- [ ] 审查新增的 TODO/FIXME
- [ ] 清理死代码 (`npm run harness:dead:code`)
- [ ] 更新过期的 ADR
- [ ] 补充缺失的测试

### 每月目标

- 提高测试覆盖率 5%
- 减少技术债务 10%
- 优化测试执行速度
- 审查并更新架构约束

---

## 📚 参考资料

- [Vitest 官方文档](https://vitest.dev/)
- [Playwright 文档](https://playwright.dev/)
- [Testing Library 最佳实践](https://testing-library.com/docs/react-testing-library/intro/)
- [Rust 测试指南](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [OpenAI Harness Engineering](https://openai.com/index/harness-engineering/)

---

**返回**: [🏠 测试主页](./README.md)
