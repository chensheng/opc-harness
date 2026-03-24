# TD-005: CLI Browser 验证场景不足

## 📋 基本信息

- **创建日期**: 2026-03-22
- **优先级**: P3 (轻微)
- **状态**: 📋 待开始
- **影响范围**: CLI 功能、测试覆盖率
- **负责人**: 未分配
- **偿还计划**: 2026-04 周

---

## 📝 问题描述

CLI Browser 功能仅有冒烟测试，缺少关键路径测试，部分功能回归无法自动发现。

### 当前测试覆盖

- ✅ 基本启动测试
- ✅ 页面加载测试
- ❌ 缺少用户交互测试
- ❌ 缺少边缘场景测试
- ❌ 缺少性能回归测试

### 缺失的测试场景

1. **导航功能**: 前进/后退/刷新
2. **网络请求**: 离线/慢速网络处理
3. **截图功能**: 全屏/元素截图
4. **控制台消息**: 错误捕获
5. **内存泄漏**: 长时间运行测试

---

## 🎯 解决方案

### 步骤 1: 定义测试矩阵

| 功能模块 | 测试场景 | 优先级 |
|----------|----------|--------|
| 导航 | URL 跳转、历史导航 | P0 |
| 截图 | 页面截图、元素截图 | P0 |
| 控制台 | 日志捕获、错误检测 | P1 |
| 网络 | 请求拦截、响应修改 | P1 |
| 性能 | 内存使用、加载时间 | P2 |

### 步骤 2: 实现关键测试

```typescript
// e2e/browser/navigation.test.ts
import { test, expect } from '@playwright/test';

test.describe('Browser Navigation', () => {
  test('should navigate to URL', async ({ page }) => {
    await page.goto('https://example.com');
    await expect(page).toHaveTitle(/Example/);
  });

  test('should handle browser history', async ({ page }) => {
    await page.goto('https://example.com/page1');
    await page.goto('https://example.com/page2');
    await page.goBack();
    await expect(page).toHaveURL(/page1/);
  });
});
```

### 步骤 3: 添加视觉回归测试

```typescript
// e2e/browser/screenshot.test.ts
test('should capture full page screenshot', async ({ page }) => {
  await page.goto('https://example.com');
  const screenshot = await page.screenshot({ fullPage: true });
  expect(screenshot).toMatchSnapshot('fullpage.png');
});
```

---

## ✅ 验收标准

- [ ] 关键路径测试覆盖率 > 80%
- [ ] 至少 10 个自动化测试用例
- [ ] 测试在 CI 中稳定运行
- [ ] 无假阳性/假阴性

---

## 📊 实施计划

### 第一阶段：基础测试（2026-04-01）

- [ ] 导航功能测试
- [ ] 截图功能测试
- [ ] 控制台消息测试

### 第二阶段：高级测试（2026-04-05）

- [ ] 网络条件模拟
- [ ] 性能基准测试
- [ ] 内存泄漏检测

### 第三阶段：集成（2026-04-10）

- [ ] 集成到 CI 流程
- [ ] 添加测试报告
- [ ] 文档更新

---

## 📚 相关资源

- [Playwright 测试最佳实践](https://playwright.dev/docs/best-practices)
- [E2E 测试规范](file://d:\workspace\opc-harness\e2e\app.spec.ts)
- [harness-e2e.ps1 脚本](file://d:\workspace\opc-harness\scripts\harness-e2e.ps1)

---

**最后更新**: 2026-03-24
