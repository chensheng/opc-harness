## MODIFIED Requirements

### Requirement: 产品规格层级体系
产品规格 MUST 按以下层级组织:
- **产品级规格**: 全局产品定义,跨变更共享 (如 vibe-coding, vibe-design, vibe-marketing)
- **Capability 级规格**: 特定功能的能力定义
- **Change 级规格**: 变更引入的新需求

#### Scenario: 查找 Vibe Coding 规格
- **WHEN** 开发者需要了解 Vibe Coding 模块的整体设计
- **THEN** 应访问 `openspec/specs/vibe-coding/spec.md` 获取完整产品规格

### Requirement: 产品设计文档整合
系统 SHALL 将产品设计文档整合到 product-specification capability 中,包括产品愿景、用户故事、功能列表等。

#### Scenario: 查看产品设计
- **WHEN** 产品经理查看产品整体设计
- **THEN** 应看到产品愿景、目标用户、核心功能和验收标准
