# TD-004: 缺少架构护栏测试

## 📋 基本信息

- **创建日期**: 2026-03-22
- **优先级**: P3 (轻微)
- **状态**: ✅ 已偿还
- **影响范围**: 代码质量、架构一致性
- **负责人**: OPC-HARNESS Team
- **偿还计划**: 2026-03-28 周
- **完成日期**: 2026-03-28

---

## 📝 问题描述

尚未实现自定义 ESLint 规则检测架构违规，依赖人工 Code Review 发现架构问题。

### 当前状态

- ✅ 已有基础架构约束测试 (`tests/architecture/constraints.test.ts`)
- ✅ 已实现自动化 ESLint 规则
- ✅ 已在开发时实时检测架构违规

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

- [x] 实现至少 1 个架构 ESLint 规则 → **实际完成 3 个** ✅
- [x] 规则在 CI 中运行 → **集成到 `npm run lint`** ✅
- [x] 能够检测常见的架构违规 → **覆盖 9 种场景** ✅
- [x] 提供清晰的错误信息 → **每个规则都有详细消息** ✅

---

## 📊 实施计划

### 第一阶段：调研（2026-03-25）✅

- [x] 评估 eslint-plugin-boundaries
- [x] 评估 eslint-plugin-import
- [x] 确定最佳方案

### 第二阶段：实现（2026-03-28）✅

- [x] 安装并配置插件
- [x] 定义架构规则
- [x] 编写测试用例

### 第三阶段：集成（2026-03-28）✅

- [x] 集成到 CI 流程
- [x] 更新开发文档
- [x] 团队培训

---

## 🎯 实施成果

### 已实现的规则

1. **architecture-constraint** - 架构分层约束规则
   - 检测 Store 层导入组件层
   - 检测 Hook 层导入业务组件层
   - 检测组件层直接导入 Rust 代码
   - 强制执行分层依赖关系矩阵

2. **ui-component-purity** - UI 组件纯度检查规则
   - 禁止 UI 组件调用 Tauri invoke
   - 禁止 UI 组件直接 HTTP 请求（axios/fetch）
   - 禁止 UI 组件包含复杂异步操作
   - 禁止 UI 组件直接导入 stores

3. **store-api-check** - Store 层 API 调用检查规则
   - 禁止 Store 层使用 axios
   - 禁止 Store 层使用 fetch 访问 HTTP 端点
   - 禁止 Store 层导入 HTTP 客户端库

### 文件结构

```
eslint-rules/
├── architecture-constraint.cjs (129 行)
├── ui-component-purity.cjs (114 行)
├── store-api-check.cjs (103 行)
├── index.cjs (15 行)
└── README.md (完整文档)

tests/eslint-rules/
└── architecture-rules.test.ts (200+ 行单元测试)
```

### 集成状态

- ✅ ESLint 配置文件已更新 (`eslint.config.js`)
- ✅ 自定义插件已注册 (`architecture` plugin)
- ✅ 3 个规则全部启用（error 级别）
- ✅ 单元测试已编写并验证
- ✅ 集成到 `npm run lint` 命令
- ✅ 集成到 `npm run harness:check` 流程

### 使用方法

```bash
# 开发时实时检测
npm run lint

# 自动修复可修复的问题
npm run lint:fix

# 完整的架构健康检查
npm run harness:check
```

### 单元测试覆盖

```bash
# 运行自定义规则测试
npm run test:unit -- tests/eslint-rules/architecture-rules.test.ts
```

测试覆盖场景：
- ✅ 允许的导入模式（valid cases）
- ✅ 禁止的导入模式（invalid cases）
- ✅ 错误消息准确性验证
- ✅ 所有规则的 create 函数逻辑

---

## 📚 相关资源

- [ESLint 自定义规则](https://eslint.org/docs/latest/extend/custom-rules)
- [RuleTester API](https://eslint.org/docs/latest/extend/custom-rules#testing-rules)
- [`Harness Engineering 流程`](../../docs/HARNESS_ENGINEERING.md)
- [`自定义 ESLint 规则文档`](../../eslint-rules/README.md)

---

**最后更新**: 2026-03-28  
**偿还完成**: 2026-03-28  
**Health Score**: 100/100 ✅
