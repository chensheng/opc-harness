# TD-004: 缺少架构护栏测试

## 📋 基本信息

- **创建日期**: 2026-03-22
- **优先级**: P3 (轻微)
- **状态**: 📋 待开始
- **影响范围**: 代码质量、架构一致性
- **负责人**: 未分配
- **偿还计划**: 2026-03-28 周

---

## 📝 问题描述

尚未实现自定义 ESLint 规则检测架构违规，依赖人工 Code Review 发现架构问题。

### 当前状态

- ✅ 已有基础架构约束测试 (`tests/architecture/constraints.test.ts`)
- ❌ 缺少自动化 ESLint 规则
- ❌ 无法在开发时实时检测架构违规

### 影响

1. **架构漂移**: 难以自动阻止违反架构规则的代码
2. **Code Review 负担**: 依赖人工检查架构合规性
3. **反馈延迟**: 问题可能在合并后才被发现

---

## 🎯 解决方案

### 方案 1: 使用 eslint-plugin-boundaries

```bash
npm install --save-dev eslint-plugin-boundaries
```

配置 `.eslintrc.js`:

```javascript
{
  "plugins": ["boundaries"],
  "rules": {
    "boundaries/element-types": [
      "error",
      {
        "default": "disallow",
        "rules": [
          {
            "from": "**/components/**",
            "allow": ["**/hooks/**", "**/lib/**"]
          },
          {
            "from": "**/hooks/**",
            "allow": ["**/lib/**", "**/stores/**"]
          }
        ]
      }
    ]
  }
}
```

### 方案 2: 自定义 ESLint 插件

创建 `eslint-rules/architecture-rule.js`:

```javascript
module.exports = {
  rules: {
    'architecture-constraint': {
      meta: {
        type: 'problem',
        docs: {
          description: 'Enforce architecture constraints'
        }
      },
      create(context) {
        return {
          ImportDeclaration(node) {
            const source = node.source.value;
            const currentFile = context.getFilename();
            
            // 检查导入是否违反架构规则
            if (violatesArchitecture(currentFile, source)) {
              context.report({
                node,
                message: `Import violates architecture constraint`
              });
            }
          }
        };
      }
    }
  }
};
```

---

## ✅ 验收标准

- [ ] 实现至少 1 个架构 ESLint 规则
- [ ] 规则在 CI 中运行
- [ ] 能够检测常见的架构违规
- [ ] 提供清晰的错误信息

---

## 📊 实施计划

### 第一阶段：调研（2026-03-25）

- [ ] 评估 eslint-plugin-boundaries
- [ ] 评估 eslint-plugin-import
- [ ] 确定最佳方案

### 第二阶段：实现（2026-03-26）

- [ ] 安装并配置插件
- [ ] 定义架构规则
- [ ] 编写测试用例

### 第三阶段：集成（2026-03-27）

- [ ] 集成到 CI 流程
- [ ] 更新开发文档
- [ ] 团队培训

---

## 📚 相关资源

- [eslint-plugin-boundaries](https://github.com/javierbrea/eslint-plugin-boundaries)
- [eslint-plugin-import](https://github.com/import-js/eslint-plugin-import)
- [ESLint 自定义规则](https://eslint.org/docs/latest/extend/custom-rules)

---

**最后更新**: 2026-03-24
