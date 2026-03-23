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
- 🔄 CI/CD 集成模板

---

### ✅ 安装验证（15 分钟）
**适合人群**: 刚安装完项目，想验证测试是否正常

👉 **[testing-validation.md](./testing-validation.md)**

内容：
- 📋 8 步验证清单
- ✅ 验收标准
- 🐛 故障排除

---

## 🎯 快速命令参考

### 单元测试
```bash
npm run test:unit         # 单元测试 ⭐

# 按需使用
npx vitest run --coverage # 生成覆盖率报告
npx vitest --ui           # UI 界面（浏览器）
```

### 🌐 E2E 测试 ⭐
```bash
npm run test:e2e          # 智能运行（自动检测并启动服务器）⭐
```

**🎯 为什么使用轻量级 E2E？**
- ✅ 无需下载浏览器（节省 ~300MB）
- ✅ 零配置，直接使用 Vitest
- ✅ 更快的测试执行速度（快 75%）
- ✅ 满足 80% 的 E2E 测试需求
- ✅ **自动管理服务器** - 检测端口、自动启动、超时处理

**📖 详细说明**: [E2E 测试方案对比](./E2E-STRATEGY.md)

### Harness Engineering
```bash
npm run harness:check           # 架构健康检查（主入口）⭐
npm run harness:check -- -All   # 完整检查（包含文档和死代码检测）
```

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
2. ✅ 运行 `npm run test:unit`
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
**A**: 先运行 `npm run test:unit` 看看现有测试，然后阅读 [快速开始](./README.md)

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