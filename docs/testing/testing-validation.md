# ✅ 测试体系安装验证指南

> **8 步完成验证** | 预计时间：15 分钟

---

## 📋 验证清单

### Step 1: 运行单元测试 ⏱️ 2 分钟

```bash
npm run test:run
```

**预期输出**:
```
 ✓ src/stores/appStore.test.ts  (6 tests) 45ms
 ✓ src/components/ui/button.test.tsx  (6 tests) 78ms

 Test Files  2 passed (2)
      Tests  12 passed (12)
```

✅ **通过标准**: 所有测试通过（12/12）

---

### Step 2: 查看测试 UI ⏱️ 1 分钟

```bash
npm run test:ui
```

**预期行为**:
- 浏览器自动打开
- 显示 Vitest UI 界面
- 可以看到测试结果

✅ **通过标准**: UI 正常显示

---

### Step 3: 运行 E2E 测试 ⏱️ 5 分钟

```bash
# 终端 1: 启动开发服务器
npm run dev

# 终端 2: 运行 E2E 测试
npm run test:e2e
```

**预期输出**:
```
Running 9 tests using 4 workers

  ✓  1 app.spec.ts:10:3 › should load successfully (1.2s)
  ✓  2 app.spec.ts:15:3 › should display navigation menu (856ms)
  ...

  9 passed (3.5s)
```

✅ **通过标准**: 9 个 E2E 测试全部通过

---

### Step 4: 生成覆盖率报告 ⏱️ 2 分钟

```bash
npm run test:coverage
```

**预期输出**:
```
 % Coverage report from v8
----------|---------|----------|---------|---------|-------------------
File      | % Stmts | % Branch | % Funcs | % Lines | Uncovered Line    
----------|---------|----------|---------|---------|-------------------
All files |   75.2  |    68.4  |   72.1  |   76.8  |                   
----------|---------|----------|---------|---------|-------------------
```

✅ **通过标准**: 生成 HTML 报告，覆盖率 > 70%

**查看报告**:
```bash
# Windows
start coverage/index.html

# macOS/Linux
open coverage/index.html
```

---

### Step 5: 架构健康检查 ⏱️ 2 分钟

```bash
npm run harness:check
```

**预期输出**:
```
========================================
  OPC-HARNESS Architecture Health Check
========================================

[1/6] TypeScript Type Checking...
  [PASS] TypeScript type checking passed

[2/6] ESLint Code Quality Check...
  [PASS] ESLint check passed

[3/6] Prettier Formatting Check...
  [PASS] Prettier formatting passed

[4/6] Rust Compilation Check...
  [PASS] Rust compilation check passed

[5/6] Dependency Integrity Check...
  [PASS] Dependency files intact

[6/6] Directory Structure Check...
  [PASS] Directory structure complete

========================================
  Health Score: 100/100
  Status: Excellent
========================================
```

✅ **通过标准**: 健康评分 >= 90

---

### Step 6: 文档一致性检查 ⏱️ 1 分钟

```bash
npm run harness:doc:check
```

**预期输出**:
```
========================================
  Documentation Consistency Check
========================================

[1/4] Checking AGENTS.md Links...
  Found 0 broken links

[2/4] Checking Code Comments...
  Found 3 TODO comments

[3/4] Checking Architecture Decision Records...
  Found 5 ADRs

[4/4] Checking Product Documentation Sync...
  All docs updated recently

========================================
  Documentation Health Score: 100/100
========================================
```

✅ **通过标准**: 无断裂链接

---

### Step 7: 死代码检测 ⏱️ 1 分钟

```bash
# 预览模式
npm run harness:dead:code:dry
```

**预期输出**:
```
========================================
  Dead Code Detection
========================================

Unused Imports: 3
Unused Functions: 2
Unused Types: 1
TODO Comments: 5

Total Issues: 11
```

✅ **通过标准**: 死代码数量 <= 10

---

### Step 8: 完整测试套件 ⏱️ 8 分钟

```bash
npm run harness:test:full
```

**执行内容**:
1. ✅ 架构健康检查
2. ✅ 单元测试 + 覆盖率
3. ✅ E2E 测试

✅ **通过标准**: 所有检查项通过

---

## 📊 验收标准

### 必须满足 ✅

- [ ] 所有单元测试通过（12/12）
- [ ] 所有 E2E 测试通过（9/9）
- [ ] 架构健康评分 >= 90
- [ ] 无 TypeScript 类型错误
- [ ] 无 ESLint 错误
- [ ] Rust 编译通过

### 推荐满足 🌟

- [ ] 测试覆盖率 >= 70%
- [ ] 无断裂的文档链接
- [ ] 死代码数量 <= 10

---

## 🐛 故障排除

### Q: `npm run test:run` 报错 "Cannot find module 'vitest'"

**解决方案**:
```bash
npm install vitest --save-dev
```

### Q: E2E 测试超时失败

**解决方案**:
```bash
# 增加超时时间
npm run test:e2e -- --timeout=30000
```

### Q: PowerShell 脚本报"权限错误"

**解决方案**:
```powershell
# Windows: 以管理员身份运行 PowerShell
Set-ExecutionPolicy RemoteSigned -Scope CurrentUser
```

### Q: 覆盖率报告显示 0%

**解决方案**:
1. 确保 `vite.config.ts` 中有 coverage 配置
2. 检查 `include` 和 `exclude` 模式
3. 运行 `npm run test:coverage -- --reporter=verbose`

---

## 📚 下一步

### ✅ 验证完成！接下来：

1. 📖 阅读 [完整测试指南](./testing-full.md)
2. 🧪 开始编写新功能的测试
3. 📊 每周运行一次 `npm run harness:test:full`
4. 🔄 持续改进测试覆盖率

---

**返回**: [🏠 测试主页](./README.md)
