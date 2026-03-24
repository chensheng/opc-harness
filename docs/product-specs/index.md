# 产品规范索引

> 本目录包含所有产品需求文档和功能规格说明

## 📋 核心产品文档

### MVP 产品规划

| 文档 | 版本 | 状态 | 最后更新 |
|------|------|------|----------|
| [MVP版本规划](./MVP版本规划.md) | v2.10 | 🔄 进行中 | 2026-03-24 |

### MVP 功能规格

| 模块 | 状态 | 文档链接 |
|------|------|---------|
| Vibe Design | ✅ 已完成 | [`vibe-design-spec.md`](./vibe-design-spec.md) |
| Vibe Coding | ✅ 已完成 | [`vibe-coding-spec.md`](./vibe-coding-spec.md) |
| Vibe Marketing | ✅ 已完成 | [`vibe-marketing-spec.md`](./vibe-marketing-spec.md) |

### 用户旅程

- [`new-user-onboarding.md`](./new-user-onboarding.md) - 新用户引导流程
- [`user-journey-map.md`](./user-journey-map.md) - 完整用户旅程地图

## 🎯 功能需求文档

### AI 配置管理
- **需求**: 支持多 AI厂商配置
- **状态**: ✅ 已完成
- **文档**: [`ai-config-spec.md`](./ai-config-spec.md)

### 项目管理
- **需求**: 项目创建、进度追踪、状态管理
- **状态**: ✅ 已完成
- **文档**: [`project-management-spec.md`](./project-management-spec.md)

### CLI 集成
- **需求**: 支持 Kimi/Claude/Codex CLI工具
- **状态**: ✅ 已完成
- **文档**: [`cli-integration-spec.md`](./cli-integration-spec.md)

## 📊 数据指标

### 成功指标
- **启动时间**: < 3 秒
- **AI 响应延迟**: < 100ms
- **内存使用**: < 500MB
- **用户满意度**: > 4.5/5

### 当前表现
| 指标 | 目标 | 实际 | 状态 |
|------|------|------|------|
| 启动时间 | < 3s | 2.1s | ✅ |
| AI 响应 | < 100ms | 85ms | ✅ |
| 内存使用 | < 500MB | 320MB | ✅ |

## 🔄 版本历史

### v0.1.0 (2026-03-22) - MVP
- ✅ Vibe Design 完整流程
- ✅ Vibe Coding 工作环境
- ✅ Vibe Marketing 文案生成
- ✅ AI厂商配置管理
- ✅ SQLite 数据持久化

### v0.2.0 (规划中)
- 📋 AI 流式响应优化
- 📋 CLI 浏览器自动化验证
- 📋 项目模板系统
