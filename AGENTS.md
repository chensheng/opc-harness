# AI Agent 导航地图

> **核心理念**: "人类掌舵，Agent 执行" (Humans steer. Agents execute.)
> 
> **重要提示**: 本文档仅为导航入口，详细规则请参阅各模块的 AGENTS.md

## 📍 快速定位

### 项目类型
**OPC-HARNESS** - AI 驱动的一人公司操作系统 MVP
- **技术栈**: React 18 + TypeScript 5 + Tauri v2 + Rust + SQLite
- **目标用户**: 独立开发者、个人创业者
- **核心功能**: Vibe Design → Vibe Coding → Vibe Marketing

### 关键文档索引

#### 📋 架构与规范
- [`AGENTS.md`](./src/AGENTS.md) - 前端代码规范
- [`AGENTS.md`](./src-tauri/AGENTS.md) - Rust 后端规范
- [`ARCHITECTURE.md`](./ARCHITECTURE.md) - 系统架构设计
- [架构约束](#🏗️-架构约束) - 全局依赖规则

#### 🧪 测试体系 ⭐
- [`docs/testing/`](./docs/testing/) - 测试文档中心
  - [README.md](./docs/testing/README.md) - 5 分钟快速开始
  - [testing-full.md](./docs/testing/testing-full.md) - 完整测试指南
  - [testing-validation.md](./docs/testing/testing-validation.md) - 安装验证清单

#### 📚 知识库 (docs/)
- [设计文档](./docs/design-docs/) - 技术方案和设计决策
- [执行计划](./docs/exec-plans/) - 活跃/已完成的执行计划
- [产品规范](./docs/product-specs/) - 产品需求说明
- [参考资料](./docs/references/) - 外部参考文档
- [生成文档](./docs/generated/) - 自动生成的文档

#### 🔧 Harness 工具

**架构健康检查**:
```bash
# 基础检查（推荐）
npm run harness:check

# 完整检查（包含文档和死代码）
npm run harness:check -- -All
```

**测试套件**:
```bash
# 单元测试
npm run test:unit

# E2E 测试（智能运行，自动管理服务器）
npm run test:e2e
```

详细使用指南请参考：
- [scripts/README.md](./scripts/README.md) - 自动化脚本说明
- [docs/testing/README.md](./docs/testing/README.md) - 测试体系导航
- [docs/references/harness-user-guide.md](./docs/references/harness-user-guide.md) - Harness 用户指南