# 🧪 测试体系文档中心

> **基于 OpenAI Harness Engineering 最佳实践**  
> 最后更新：2026-03-23

---

## 📖 文档导航

### 🚀 快速开始（5 分钟）
**适合人群**: 第一次接触测试的开发者

👉 **[README.md](./README.md)** - 从这里开始！

内容：
- ⚡ 安装和运行第一个测试
- 🎯 常用命令速查
- 📁 测试文件位置
- 🐛 常见问题解答

---

### 📚 完整指南（30 分钟）
**适合人群**: 想深入了解测试体系的开发者

👉 **[testing-full.md](./testing-full.md)**

内容：
- 🎯 测试金字塔详解
- 🧪 单元测试最佳实践
- 🌐 E2E 测试编写指南
- 🏛️ 架构约束测试
- 📚 文档一致性检查
- 🗑️ 垃圾回收机制
- 🔄 CI/CD 集成模板

---

### ✅ 安装验证（15 分钟）
**适合人群**: 刚安装完项目，想验证测试是否正常

👉 **[testing-validation.md](./testing-validation.md)**

内容：
- 📋 8 步验证清单
- ✅ 验收标准
- 🐛 故障排除
- 📊 质量门禁

---

## 🎯 快速命令参考

### 单元测试
```
npm run test:unit         # 单元测试 ⭐

# 按需使用
npx vitest run --coverage # 生成覆盖率报告
npx vitest --ui           # UI 界面（浏览器）
npx vitest                # 监视模式（开发时用，可选）
```

### 🌐 E2E 测试 ⭐ (轻量级方案)
```
npm run test:e2e          # 智能运行（自动检测并启动服务器）⭐
```

**🎯 为什么使用轻量级 E2E？**
- ✅ 无需下载浏览器（节省 ~300MB）
- ✅ 零配置，直接使用 Vitest
- ✅ 更快的测试执行速度（快 75%）
- ✅ 满足 80% 的 E2E 测试需求
- ✅ 内存占用降低 90%
- ✅ **自动管理服务器** - 检测端口、自动启动、超时处理

**📖 详细说明**: [E2E 测试方案对比](./E2E-STRATEGY.md)

### Harness Engineering
```
npm run harness:check           # 架构健康检查（主入口）⭐
npm run harness:check -- -All   # 完整检查（包含文档和死代码检测）
```

**💡 提示**: 其他 Harness 功能已整合到 `harness:check` 中：
- 文档一致性检查 → `npm run harness:check -- -DocCheck`
- 死代码检测 → `npm run harness:check -- -DeadCode`
- 快速验证、垃圾回收等功能已移除，推荐使用 Git diff 和手动组合命令

---

## 📊 测试金字塔

```
           /\
          / E2E \       ~10% (用户工作流)
         /--------\     
        /Integration\    ~20% (模块间通信)
       /------------\   
      /   Unit Tests  \  ~70% (组件、函数、逻辑)
     /------------------\
    / Architecture Tests \ (约束验证)
   /----------------------\
  /  Documentation Checks  \ (一致性检查)
 /--------------------------\
/    Dead Code Detection     \ (垃圾回收)
------------------------------
```

---

## 🎓 学习路径

### Week 1: 熟悉与上手
1. ✅ 阅读 [快速开始](./README.md)
2. ✅ 运行 `npm run test:run`
3. ✅ 查看示例测试代码

### Week 2: 补充测试覆盖
1. ✅ 阅读 [完整指南](./testing-full.md)
2. ✅ 为现有组件添加测试
3. ✅ 达到 70% 覆盖率

### Month 1: 建立流程
1. ✅ 集成到 CI/CD
2. ✅ 每周运行完整测试
3. ✅ 优化测试速度

---

## 🙋 常见问题

### Q: 我应该从哪里开始？
**A**: 先运行 `npm run test:run` 看看现有测试，然后阅读 [快速开始](./README.md)

### Q: 如何编写第一个测试？
**A**: 参考 [完整指南](./testing-full.md) 中的"编写测试示例"部分

### Q: 测试运行很慢怎么办？
**A**: 使用并行模式 `npm run test:run -- --pool=forks`

### Q: E2E 测试失败但手动测试正常？
**A**: 使用有头模式调试 `npm run test:e2e:headed`

更多问题请查看 [完整指南](./testing-full.md) 或 [故障排除](./testing-validation.md#-故障排除)

---

## 📚 外部资源

- [Vitest 官方文档](https://vitest.dev/)
- [Playwright 文档](https://playwright.dev/)
- [Testing Library 指南](https://testing-library.com/)
- [OpenAI Harness Engineering](https://openai.com/index/harness-engineering/)

---

**需要帮助？** 在 GitHub Issues 提问或联系维护者

## 📚 相关文档

### 核心文档

- [README.md](../README.md) - 项目主文档
- [AGENTS.md](../AGENTS.md) - AI Agent 导航地图
- [ARCHITECTURE.md](../ARCHITECTURE.md) - 架构设计文档

### 测试和 Harness 文档

- [docs/testing/README.md](./testing/README.md) - 测试体系导航 ⭐
- [docs/testing/COMMANDS-REFERENCE.md](./testing/COMMANDS-REFERENCE.md) - 完整命令参考
- [docs/testing/HARNESS-COMMANDS.md](./testing/HARNESS-COMMANDS.md) - Harness 命令精简说明
- [docs/testing/HARNESS-STRUCTURE.md](./testing/HARNESS-STRUCTURE.md) - 目录结构说明
- [docs/testing/E2E-STRATEGY.md](./testing/E2E-STRATEGY.md) - E2E 测试方案
- [docs/references/harness-user-guide.md](../references/harness-user-guide.md) - Harness 用户指南
- [docs/references/best-practices.md](../references/best-practices.md) - 最佳实践
- [docs/references/architecture-rules.json](../references/architecture-rules.json) - 架构规则配置

```
# 🧪 OPC-HARNESS 测试体系

> **基于 OpenAI Harness Engineering 最佳实践**  
> 最后更新：2026-03-23

## 📋 快速导航

- [🚀 5 分钟快速开始](#-5-分钟快速开始)
- [📖 完整文档](./testing-full.md) - 详细的测试指南和最佳实践
- [📊 安装验证](./testing-validation.md) - 8 步验证清单

---

## ⚡ 5 分钟快速开始

### 1️⃣ 安装依赖

```bash
npm install
```

### 2️⃣ 运行测试

```bash
# 单元测试（推荐开发时使用）
npm run test

# 或一次性运行
npm run test:run

# 查看 UI 界面
npm run test:ui
```

### 3️⃣ E2E 测试

```bash
# 启动开发服务器（终端 1）
npm run dev

# 运行 E2E 测试（终端 2）
npm run test:e2e
```

### 4️⃣ 完整验证

```bash
# 推荐：完整测试套件
npm run harness:test:full
```

这将执行：
- ✅ 架构健康检查（类型、编译、规范）
- ✅ 单元测试 + 覆盖率报告
- ✅ E2E 端到端测试

---

## 🎯 常用命令

### 单元测试
```
npm run test              # Vitest 监视模式
npm run test:run          # 运行所有单元测试
npm run test:coverage     # 生成覆盖率报告
npm run test:ui           # Vitest UI 界面
```

### E2E 测试
```
# 注意：首次运行前需要安装 Playwright 浏览器
# 详见：docs/testing/PLAYWRIGHT-INSTALL.md

npm run test:e2e          # Playwright E2E
npm run test:e2e:ui       # Playwright UI 模式
npm run test:e2e:headed   # E2E 有头模式（使用系统浏览器）
npm run test:e2e:report   # 查看 E2E 报告
```

**🔧 浏览器未安装？**  
如果看到 `browserType.launch` 错误，请查看 [Playwright 安装指南](./PLAYWRIGHT-INSTALL.md)

### Harness Engineering
```
npm run harness:check           # 架构健康检查（主入口）⭐
npm run harness:check -- -All   # 完整检查（包含文档和死代码检测）
```

**💡 提示**: 其他 Harness 功能已整合到 `harness:check` 中：
- 文档一致性检查 → `npm run harness:check -- -DocCheck`
- 死代码检测 → `npm run harness:check -- -DeadCode`
- 快速验证、垃圾回收等功能已移除，推荐使用 Git diff 和手动组合命令

---

## 📁 测试文件位置

```
opc-harness/
├── src/
│   ├── components/ui/
│   │   └── button.test.tsx      # 组件测试
│   └── stores/
│       └── appStore.test.ts     # Store 测试
├── tests/
│   └── architecture/
│       └── constraints.test.ts  # 架构约束测试
├── e2e/
│   └── app.spec.ts              # E2E 测试
└── src-tauri/
    └── tests/
        ├── integration_test.rs  # Rust 集成测试
        └── common/mod.rs        # Rust 测试工具
```

---

## 📊 质量门禁

| 指标 | 阈值 | 状态 |
|------|------|------|
| 单元测试覆盖率 | >= 70% | ⏳ 待运行 |
| E2E 测试通过率 | 100% | ⏳ 待运行 |
| 架构健康评分 | >= 90 | ⏳ 待运行 |
| 死代码数量 | <= 10 | ⏳ 待运行 |

---

## 🐛 常见问题

### Q: 测试运行很慢？
```bash
# 并行执行
npm run test:run -- --pool=forks

# 只运行变更的测试
npm run test:run -- --changed
```

### Q: E2E 测试失败但手动测试正常？
```bash
# 使用有头模式查看浏览器行为
npm run test:e2e:headed

# 增加超时时间
npm run test:e2e -- --timeout=10000
```

### Q: PowerShell 脚本报权限错误？
```powershell
# Windows: 以管理员身份运行
Set-ExecutionPolicy RemoteSigned -Scope CurrentUser
```

---

## 📚 学习资源

- [Vitest 官方文档](https://vitest.dev/)
- [Playwright 文档](https://playwright.dev/)
- [Testing Library 指南](https://testing-library.com/)
- [OpenAI Harness Engineering](https://openai.com/index/harness-engineering/)

---

**需要详细文档？** 查看 [📖 完整测试指南](./testing-full.md)
