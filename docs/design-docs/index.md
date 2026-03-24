# 设计文档索引

> 本目录包含所有技术方案、架构决策和系统设计文档

## 📋 架构决策记录 (ADRs)

位置：[`decision-records/`](./decision-records/)

| 编号 | 标题 | 状态 | 日期 |
|------|------|------|------|
| [ADR-001](./decision-records/adr-001-typescript-strict-mode.md) | 启用 TypeScript 严格模式 | ✅ 已采纳 | 2026-03-15 |
| [ADR-002](./decision-records/adr-002-zustand-state-management.md) | 使用 Zustand 进行状态管理 | ✅ 已采纳 | 2026-03-15 |
| [ADR-003](./decision-records/adr-003-tauri-v2-architecture.md) | Tauri v2 前后端分离架构 | ✅ 已采纳 | 2026-03-15 |
| [ADR-004](./decision-records/adr-004-sqlite-integration.md) | SQLite 数据库集成方案 | ✅ 已采纳 | 2026-03-16 |
| [ADR-005](./decision-records/adr-005-sse-streaming.md) | SSE 流式输出实现方案 | ✅ 已采纳 | 2026-03-17 |

## 🏗️ 核心设计文档

- [`core-beliefs.md`](./core-beliefs.md) - 核心设计理念
- [`system-architecture.md`](./system-architecture.md) - 系统架构设计
- [`database-schema.md`](./database-schema.md) - 数据库设计

## 📐 技术规范

- [`frontend-guidelines.md`](./frontend-guidelines.md) - 前端开发规范
- [`rust-guidelines.md`](./rust-guidelines.md) - Rust 后端规范
- [`api-design.md`](./api-design.md) - Tauri Commands 设计规范
- [`architecture-rules.md`](./architecture-rules.md) - 架构约束规则（含测试约束）🔥

## 🔄 更新记录

- **2026-03-22**: 按照 OpenAI Harness Engineering 最佳实践重组目录结构
- **2026-03-17**: 添加 ADR-005 SSE 流式输出
- **2026-03-16**: 添加 ADR-004 SQLite 集成
