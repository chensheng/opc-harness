## Context

AgentMonitor.tsx 组件当前包含大量 console.log 调试日志,主要用于:
1. WebSocket 消息处理流程追踪
2. 智能体匹配逻辑调试
3. 状态更新监控

这些日志在代码行 154-260 之间密集分布,每次 WebSocket 消息到达都会触发多次日志输出,导致控制台信息过载。

**当前问题**:
- 每条 WebSocket 消息触发 5-7 条 console.log
- 智能体列表刷新时输出所有智能体的详细信息
- 消息匹配失败时输出完整的智能体列表用于调试
- 生产环境也会输出这些日志,影响性能

**约束**:
- 必须保留错误日志(console.error)用于问题排查
- 必须保留关键警告(console.warn)用于异常情况
- 不能改变现有功能逻辑
- 需要保持代码可读性和可维护性

## Goals / Non-Goals

**Goals:**
- 移除冗余的 console.log 调试日志(约 10+ 处)
- 保留关键的 console.error 和 console.warn
- 减少控制台输出量 80% 以上
- 保持代码结构清晰,便于未来调试时按需添加日志

**Non-Goals:**
- 不引入新的日志框架或库
- 不改变现有的日志级别分类逻辑
- 不修改后端日志系统
- 不影响现有的 WebSocket 消息处理逻辑

## Decisions

### Decision 1: 移除策略 - 按日志重要性分级清理

**选择**: 根据日志用途分为三类处理:
1. **完全移除**: 纯调试用途的 console.log (如消息内容打印、智能体列表遍历)
2. **保留但降级**: 关键流程日志改为条件输出(仅在开发模式或特定条件下)
3. **完全保留**: 错误处理和异常情况的 console.error/console.warn

**理由**: 
- 简单直接,不需要引入新依赖
- 符合项目现有的日志实践
- 易于维护和后续调整

**替代方案考虑**:
- 方案 A: 引入日志级别控制系统 → 过度设计,增加复杂度
- 方案 B: 使用环境变量控制 → 需要额外配置,不够灵活
- 方案 C: 全部保留,仅在生产环境禁用 → 无法解决开发环境的噪音问题

### Decision 2: 保留的关键日志点

**保留以下日志**:
1. Line 197: `console.warn` - 智能体匹配失败(异常情况)
2. Line 312: `console.warn` - Project ID 不可用(配置错误)
3. Line 382: `console.error` - 加载智能体失败(错误处理)
4. Line 415: `console.error` - 启动智能体失败(错误处理)
5. Line 448: `console.error` - 启动失败的详细错误
6. Line 479: `console.error` - 暂停失败的详细错误
7. Line 515: `console.error` - 恢复失败的详细错误
8. Line 536: `console.error` - 停止失败的详细错误
9. Line 556: `console.error` - 删除失败的详细错误

**移除的日志**:
1. Line 154: 跳过系统连接消息的提示
2. Line 158-164: WebSocket 消息详情打印
3. Line 167-178: 智能体列表遍历输出
4. Line 180: 消息 sessionId 匹配尝试
5. Line 210-214: 成功匹配的智能体信息
6. Line 240-244: 添加日志到智能体的确认
7. Line 292-297: WebSocket 连接建立提示
8. Line 300-303: WebSocket 连接失败错误(已有更高层的错误处理)
9. Line 334: Running workers 列表输出
10. Line 336: 获取 workers 失败的警告(非关键)
11. Line 345: 智能体状态修正提示
12. Line 411: 启动智能体的开始提示
13. Line 419-423: 启动参数打印
14. Line 433: 启动成功的确认
15. Line 446: 启动成功的 emoji 提示
16. Line 455: 暂停智能体的开始提示
17. Line 463: 暂停成功的确认
18. Line 477: 暂停成功的 emoji 提示
19. Line 486: 恢复智能体的开始提示
20. Line 501: 恢复成功的确认
21. Line 513: 恢复成功的 emoji 提示
22. Line 547-552: 删除智能体后的列表更新确认
23. Line 564: 智能体创建成功的简单提示
24. Line 570: 智能体编辑成功的简单提示

### Decision 3: 代码注释保留策略

**选择**: 对于移除的日志,在关键位置添加简短注释说明原日志用途,便于未来调试时快速定位。

**示例**:
```typescript
// Removed debug log: Processing WebSocket message
if (messages.length === 0) return
```

**理由**: 
- 保持代码可追溯性
- 不增加运行时开销
- 便于团队协作和未来维护

## Risks / Trade-offs

### Risk 1: 调试难度增加
**风险**: 移除日志后,出现问题时难以快速定位
**缓解**: 
- 保留所有 error 和关键 warn 日志
- 开发者可在需要时临时添加 console.log
- 使用浏览器 DevTools 的 Network 面板查看 WebSocket 消息

### Risk 2: 遗漏重要日志
**风险**: 可能误删了某些重要的调试信息
**缓解**: 
- 仔细审查每个日志点的用途
- 保留所有错误处理和异常情况日志
- 通过测试验证功能完整性

### Trade-off: 开发便利性 vs 控制台清洁度
- **牺牲**: 开发时实时查看消息流的便利性
- **获得**: 清洁的控制台,更快的页面响应,更好的用户体验
- **平衡**: 保留关键流程日志,开发者可按需临时添加

## Migration Plan

1. **实施阶段**: 
   - 移除 identified 的 console.log 语句
   - 保留所有 console.error 和 console.warn
   - 运行测试确保功能正常

2. **验证阶段**:
   - 手动测试 AgentMonitor 的所有功能
   - 检查控制台输出是否显著减少
   - 确认错误日志仍然正常工作

3. **回滚策略**: 
   - Git 版本控制,可随时回退
   - 无数据库变更,无 API 变更,回滚风险低

## Open Questions

None - 此变更为纯粹的代码清理,不涉及架构决策或未知因素。
