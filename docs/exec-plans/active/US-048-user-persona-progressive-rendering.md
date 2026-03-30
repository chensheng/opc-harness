# US-048: 用户画像渐进式渲染

**状态**: 🔄 进行中  
**优先级**: P1  
**任务类型**: User Story  
**开始日期**: 2026-03-30  
**预计完成**: 2026-03-30  
**负责人**: OPC-HARNESS Team  
**关联需求**: Sprint 2 - EPIC-01 Feature-01.5

---

## 📋 任务概述

### 背景
在 Vibe Design 场景中，用户需要查看 AI 生成的用户画像。传统的整页渲染方式缺乏参与感，用户无法感知 AI 的生成过程。通过渐进式渲染，可以让用户逐步构建认知，提升参与感和掌控感。

### 目标
从 Sprint 2 规划中拆解的核心目标：
- [ ] **业务目标**: 提升用户对 AI 生成内容的参与感和信任度
- [ ] **功能目标**: 实现用户画像的流式生成和渐进式渲染
- [ ] **技术目标**: 复用 US-047 的流式生成架构，保持一致性

### 范围
明确包含和不包含的内容：
- ✅ **In Scope**: 
  - 用户画像流式生成 Hook (usePersonaStream)
  - 渐进式渲染组件 (UserPersonasDisplay)
  - Markdown 解析为结构化用户画像
  - 打字机效果和动画
  - 单元测试覆盖
- ❌ **Out of Scope**: 
  - 用户画像编辑功能（后续迭代）
  - 自定义样式配置（US-060）

### 关键结果 (Key Results)
可量化的成功标准：
- [ ] KR1: Health Score 100/100
- [ ] KR2: 测试覆盖率 ≥80%
- [ ] KR3: 首字延迟 < 1s
- [ ] KR4: 渐进式渲染无闪烁
- [ ] KR5: E2E 测试通过

---

## 💡 解决方案设计

### 架构设计
```
数据流:
AI Service → Stream API → usePersonaStream Hook → UserPersonasDisplay Component → Progressive Rendering

组件关系:
- usePersonaStream: 流式数据获取和状态管理
- UserPersonasDisplay: 渐进式渲染 UI 组件
- PersonaCard: 单个用户画像卡片
```

### 核心接口/API
```typescript
// Hook 接口
interface UsePersonaStreamReturn {
  personas: UserPersona[] // 已生成的用户画像
  isLoading: boolean
  error: string | null
  progress: number // 生成进度 0-100
  startGeneration: () => Promise<void>
}

// 数据结构
interface UserPersona {
  id: string
  name: string
  avatar?: string
  demographics: {
    age: string
    occupation: string
    location: string
  }
  goals: string[]
  painPoints: string[]
  behaviors: string[]
  quote: string
}
```

### 数据结构
```typescript
// 类型定义（复用在 types/index.ts）
type UserPersona = {
  id: string
  name: string
  role: string
  description: string
  needs: string[]
  frustrations: string[]
  motivations: string[]
}
```

### 技术选型
- **流式处理**: 复用 US-047 的 ReadableStream API
- **状态管理**: React Hooks (useState, useCallback, useRef)
- **UI 框架**: React + TypeScript
- **样式**: Tailwind CSS
- **动画**: CSS Transitions + Keyframes
- **测试**: Vitest + React Testing Library

---

## 📝 实施计划

### Phase 1: 类型定义 (10%)
- [ ] 在 `src/types/index.ts` 添加 UserPersona 类型
- [ ] 在 `src/types/prd-quality.ts` 添加相关质量指标

### Phase 2: Hook 实现 (30%)
- [ ] 创建 `src/hooks/usePersonaStream.ts`
- [ ] 实现流式数据处理逻辑
- [ ] 实现渐进式状态管理
- [ ] 添加错误处理和重试机制

### Phase 3: 组件实现 (30%)
- [ ] 创建 `src/components/UserPersonasDisplay.tsx`
- [ ] 实现 PersonaCard 子组件
- [ ] 实现渐进式渲染逻辑
- [ ] 添加动画和过渡效果

### Phase 4: 测试覆盖 (20%)
- [ ] 编写 Hook 单元测试（≥5 个测试用例）
- [ ] 编写组件单元测试（≥5 个测试用例）
- [ ] 确保测试覆盖率 ≥80%

### Phase 5: 质量验证 (10%)
- [ ] 运行 `harness:check` 确保 100/100
- [ ] 性能测试（首字延迟、渲染流畅度）
- [ ] 代码审查和优化

---

## ✅ 验收标准

根据 Sprint 2 规划的验收标准：
- [ ] 渐进式显示，无闪烁
- [ ] 首字延迟 < 1s
- [ ] 支持至少 3-5 个用户画像
- [ ] 每个画像包含完整信息（姓名、角色、描述等）
- [ ] 打字机效果流畅
- [ ] 错误处理完善
- [ ] 单元测试覆盖
- [ ] Health Score 100/100

---

## 📊 进度追踪

| 阶段 | 计划完成 | 实际完成 | 状态 | 备注 |
|------|---------|---------|------|------|
| Phase 1: 类型定义 | D0 | - | 📋 待开始 | - |
| Phase 2: Hook 实现 | D0+1 | - | 📋 待开始 | - |
| Phase 3: 组件实现 | D0+2 | - | 📋 待开始 | - |
| Phase 4: 测试覆盖 | D0+3 | - | 📋 待开始 | - |
| Phase 5: 质量验证 | D0+4 | - | 📋 待开始 | - |

---

## 🚧 风险与问题

| 风险 | 概率 | 影响 | 缓解措施 | 状态 |
|------|------|------|---------|------|
| 流式解析复杂 | 中 | 中 | 复用 US-047 的解析逻辑 | 📋 监控中 |
| 动画性能问题 | 低 | 中 | 使用 CSS transform 优化 | 📋 监控中 |
| 浏览器兼容性 | 低 | 低 | 降级方案支持 | 📋 监控中 |

---

## 📝 执行日志

### Day 0 (2026-03-30)
- [ ] 创建执行计划
- [ ] 完成类型定义
- [ ] 开始 Hook 实现

---

## 🔗 参考文档

- [`US-047 PRD 流式生成`](./US-047-PRD-流式生成.md) - 参考实现
- [`src/AGENTS.md`](../../src/AGENTS.md) - 前端开发规范
- [`ARCHITECTURE.md`](../../docs/design-docs/system-architecture.md) - 系统架构
