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

#### 📚 知识库 (docs/)
- [设计文档](./docs/design-docs/) - 技术方案和设计决策
- [执行计划](./docs/exec-plans/) - 活跃/已完成的执行计划
- [产品规范](./docs/product-specs/) - 产品需求说明
- [参考资料](./docs/references/) - 外部参考文档
- [生成文档](./docs/generated/) - 自动生成的文档

#### 🔧 Harness 工具
- [Harness 脚本](./scripts/) - 自动化验证脚本
- [CLI 浏览器验证](./.harness/cli-browser-verify/) - 自动化测试
- [架构守卫](./.harness/constraints/) - Linter 规则定义

## 🛠️ 可用工具

### 开发命令
```bash
# 前端开发
npm run dev              # 启动 Vite 开发服务器
npm run build            # 构建生产版本
npm run preview          # 预览构建结果

# Tauri 开发
npm run tauri:dev        # 启动 Tauri 开发环境 (前后端一起)
npm run tauri:build      # 构建生产安装包

# 代码质量
npm run lint             # ESLint 检查
npm run lint:fix         # 自动修复 ESLint 问题
npm run format           # Prettier 格式化
npm run format:check     # 检查格式规范

# Harness Engineering ⭐️
npm run harness:check          # 架构健康检查
npm run harness:gc             # 垃圾回收 (清理过时文档、死代码)
npm run harness:fix            # 代码质量自动修复 (格式化 + ESLint fix)
npm run harness:fix:dry        # 预览修复操作但不实际执行
npm run harness:quick          # 快速验证 (类型 + ESLint + Rust 检查)
npm run harness:verify:tauri   # Tauri 应用验证
npm run harness:verify:cli     # CLI 浏览器验证
```

### 环境要求
- **Node.js**: >= 18.0.0
- **Rust**: >= 1.70.0
- **必需工具**: cargo, npm/pnpm

## 🏗️ 架构约束

### 分层规则
```
✅ 允许的数据流:
Frontend (React) → Tauri Commands → Rust Services → Database
                      ↑                                    ↓
                      └────────── Response ──────────────┘

❌ 禁止的模式:
- 前端直接访问数据库
- Rust 服务直接调用前端代码
- 循环依赖
```

### 依赖约束
```typescript
// ✅ 推荐：使用路径别名
import { Button } from '@/components/ui/button';
import { useAppStore } from '@/stores/appStore';

// ❌ 避免：相对路径过深
import { Button } from '../../../components/ui/button';
```

### 模块边界
- **src/components/** - UI 组件层，不可包含业务逻辑
- **src/stores/** - 状态管理层，不可直接调用 API
- **src-tauri/src/commands/** - Tauri 命令层，仅做参数转发
- **src-tauri/src/services/** - 业务逻辑层，不可依赖 UI 组件

### 性能限制
- **启动时间**: < 3 秒 (开发模式)
- **响应延迟**: < 100ms (Tauri 命令)
- **内存使用**: < 500MB (正常运行)

## 🔄 反馈回路

### 架构健康检查
运行 `npm run harness:check` 将验证：
- ✅ TypeScript 类型安全
- ✅ ESLint 代码规范
- ✅ Prettier 格式规范
- ✅ Rust 编译检查
- ✅ 依赖完整性

### CLI Browser 验证
利用当前 AI CLI（Kimi / Claude / OpenCode）内置的浏览器能力进行验证，**无需配置 API Key**。

**使用方式**:

1. **直接对话（推荐）**:
```
请帮我验证 http://localhost:1420：
1. 页面是否正常加载 OPC-HARNESS 标题
2. 导航菜单是否包含 Dashboard、Idea、Coding
3. 点击 Idea 是否能进入输入页面
```

2. **使用浏览器命令**:
```
@browser http://localhost:1420
请告诉我你看到了什么？
```

3. **自动化脚本**:
```bash
npm run harness:verify:cli
```

**验证流程**:
```
1. 前置健康检查 → harness:check
2. 执行开发任务 → 代码修改
3. 启动开发环境 → npm run tauri:dev
4. CLI 浏览器验证 → 在 Kimi CLI 中直接验证
5. 后置健康检查 → harness:check
6. 生成执行报告
```

### 错误处理模式
```rust
// Rust 端错误处理
match result {
    Ok(data) => Ok(data),
    Err(e) => Err(format!("操作失败：{}", e)), // 中文错误提示
}
```

```typescript
// 前端错误处理
try {
  await invoke('some_command');
} catch (error) {
  console.error('[Error] 具体操作:', error);
  // 显示用户友好的错误提示
}
```

## 🗑️ 垃圾回收

### 自动清理
运行 `npm run harness:gc` 将：
- 🗑️ 删除 >30 天未更新的临时文档
- 🔍 扫描未使用的导入和死代码
- 🧹 清理过时的模拟数据
- ✅ 验证配置一致性

### 手动清理清单
```bash
# 清理构建产物
rm -rf target/
rm -rf node_modules/
rm -rf dist/

# 重置开发环境
cargo clean
npm ci
```

## 📚 知识库

### 决策记录 (ADRs)
位置：[docs/design-docs/decision-records/](./docs/design-docs/decision-records/)

已记录的架构决策：
- [ADR-001](./docs/design-docs/decision-records/adr-001-typescript-strict-mode.md) - 启用 TypeScript 严格模式
- [ADR-002](./docs/design-docs/decision-records/adr-002-zustand-state-management.md) - 使用 Zustand 进行状态管理
- [ADR-003](./docs/design-docs/decision-records/adr-003-tauri-v2-architecture.md) - Tauri v2 前后端分离架构
- [ADR-004](./docs/design-docs/decision-records/adr-004-sqlite-integration.md) - SQLite 数据库集成
- [ADR-005](./docs/design-docs/decision-records/adr-005-sse-streaming.md) - SSE 流式输出实现方案

### 执行日志
位置：[docs/exec-plans/active/](./docs/exec-plans/active/)

记录重要操作的执行结果和调试信息。

### 最佳实践
位置：[docs/references/best-practices.md](./docs/references/best-practices.md)

收录团队积累的开发经验和常见问题解决方案。

## 🎯 修改规范

### 新增功能流程
1. **创建决策记录**: 在 `docs/design-docs/` 中记录设计思路
2. **实现代码**: 遵循架构约束和代码规范
3. **更新文档**: 同步更新相关文档和注释
4. **运行检查**: 执行 `npm run harness:check` 验证
5. **提交代码**: 确保所有测试通过

### 代码审查清单
- [ ] TypeScript 类型定义完整
- [ ] Rust 代码通过 `cargo check`
- [ ] ESLint 无警告
- [ ] Prettier 格式化通过
- [ ] 添加了必要的错误处理
- [ ] 更新了相关文档

## 🚨 常见问题

### Q: 如何调试 Tauri 命令？
A: 在 `src-tauri/src/main.rs` 中启用日志：
```rust
.use_plugin(tauri_plugin_log::Builder::default().build())
```

### Q: 前端如何调用 Rust 函数？
A: 使用 Tauri 的 `invoke` API：
```typescript
import { invoke } from '@tauri-apps/api/core';
const result = await invoke('command_name', { param: 'value' });
```

### Q: 如何添加新的 AI厂商支持？
A: 
1. 在 `src/types/index.ts` 中添加类型定义
2. 在 `src-tauri/src/models/mod.rs` 中添加 Rust 模型
3. 在 `src-tauri/src/commands/ai.rs` 中实现命令
4. 更新 `src/components/common/AIConfig.tsx` UI

### Q: 如何验证界面功能？
A: 使用 CLI Browser 验证：
1. 启动开发环境：`npm run tauri:dev`
2. 在 Kimi CLI 中直接请求：`@browser http://localhost:1420`
3. 或使用指令：`npm run harness:verify:cli`

## 📖 学习资源

- [Tauri v2 文档](https://v2.tauri.app/)
- [Zustand 文档](https://zustand-demo.pmnd.rs/)
- [shadcn/ui 文档](https://ui.shadcn.com/)
- [Rust 编程语言](https://www.rust-lang.org/learn)

---

**最后更新**: 2026-03-22  
**维护者**: OPC-HARNESS Team  
**文档状态**: ✅ 符合 OpenAI Harness Engineering 最佳实践
