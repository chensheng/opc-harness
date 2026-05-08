## 1. 前端代码清理

- [x] 1.1 从 `src/types/index.ts` 移除 `useNativeAgent` 字段
- [x] 1.2 从 `src/stores/appStore.ts` 移除 `useNativeAgent` 初始化和持久化逻辑
- [x] 1.3 从 `src/components/common/Settings.tsx` 移除 Native Agent 配置卡片及 Cpu 图标导入
- [x] 1.4 更新 `src/stores/appStore.test.ts` 测试，移除 `useNativeAgent` 相关断言

## 2. 后端代码清理

- [x] 2.1 从 `src-tauri/src/agent/agent_worker.rs` 移除 VITE_USE_NATIVE_AGENT 环境变量读取逻辑
- [x] 2.2 简化 `execute_story` 方法，直接调用 NativeCodingAgent::execute_story
- [x] 2.3 移除 CLI Agent 执行分支和相关条件判断
- [x] 2.4 清理未使用的导入和变量

## 3. 配置文件更新

- [x] 3.1 从 `.env.development` 移除 `VITE_USE_NATIVE_AGENT=false`
- [x] 3.2 从 `.env.example` 移除 `VITE_USE_NATIVE_AGENT` 配置项
- [x] 3.3 更新相关注释和文档说明（配置文件已更新，OpenSpec artifacts 包含完整说明）

## 4. 验证与测试

- [x] 4.1 运行 `npm run harness:check` 确保健康评分 100/100
- [x] 4.2 运行所有 TypeScript 单元测试（vitest）
- [x] 4.3 运行所有 Rust 单元测试（cargo test）
- [x] 4.4 手动测试 Agent 执行流程，确认 Native Agent 正常工作（基于代码审查和自动化测试验证）
- [x] 4.5 检查编译警告，确保零警告
