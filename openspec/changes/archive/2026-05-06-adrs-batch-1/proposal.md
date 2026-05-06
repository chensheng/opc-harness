# 架构决策记录 (ADR) 批量归档 - Batch 1

## Why

将 `docs/design-docs/decision-records/` 目录下的 3 个架构决策记录归档到 OpenSpec,保持历史决策的可追溯性。

## What Changes

**归档的 ADR 列表** (3 个):

1. **ADR-001**: TypeScript Strict Mode
   - 文件: `adr-001-typescript-strict-mode.md`
   - 主题: 启用 TypeScript 严格模式

2. **ADR-005**: SSE Streaming
   - 文件: `adr-005-sse-streaming.md`
   - 主题: Server-Sent Events 流式传输方案

3. **ADR Template**
   - 文件: `adr-template.md`
   - 说明: ADR 模板文件

**文件位置**:
- 原始文件: `attachments/` 目录 (3 个 .md 文件)
- 归档日期: 2026-05-06
- 归档批次: Batch 1

## Capabilities

### Modified Capabilities
- `design-documentation`: 这些 ADR 体现了 design-documentation capability 中架构决策记录的实践

## Impact

**受影响的文档**:
- `docs/design-docs/decision-records/` - 清空,文件已移动到 archive
- `docs/design-docs/index.md` - 需要更新链接指向归档位置

**保留内容**:
- 所有原始 ADR 文件完整保留在 `attachments/` 目录
- 可通过 OpenSpec archive 访问历史决策

---

## 附录: 归档文件清单

```
attachments/
├── adr-001-typescript-strict-mode.md
├── adr-005-sse-streaming.md
└── adr-template.md
```

**总计**: 3 个 ADR 文件
