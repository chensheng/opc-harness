# .harness Directory

本目录包含 Harness Engineering 的核心配置和验证工具。

## 📁 目录结构

```
.harness/
├── README.md                      # 本文件
├── cli-browser-verify/           # CLI 浏览器验证模块
│   ├── README.md                 # 使用文档
│   ├── USAGE.md                  # 详细用法
│   ├── cli_detector.ps1          # CLI 检测脚本
│   ├── verify_runner.ps1         # 验证运行器
│   ├── tasks/                    # 验证任务定义
│   ├── reports/                  # 生成的报告
│   └── screenshots/              # 截图输出
├── constraints/                   # (可选) 架构约束规则
└── context-engineering/          # (可选) 上下文工程数据
```

## 🔄 变更说明

**2024-03-23**: 脚本目录迁移
- ✅ 所有开发脚本已迁移至根目录的 [`scripts/`](../scripts/) 文件夹
- ✅ `package.json` 中的 npm 命令已更新
- ✅ 文档引用已更新

**保留内容：**
- `cli-browser-verify/` - CLI 浏览器验证模块（独立功能）

**已迁移：**
- `scripts/*.ps1` → `../scripts/*.ps1`

## 🛠️ 核心组件

### CLI Browser 验证

利用 AI CLI工具（Kimi、Claude Code 等）的浏览器能力进行自动化验证，**无需配置 API Key**。

**主要功能：**
- 页面加载验证
- UI 元素检查
- 导航流程测试
- 错误检测

**使用方式：**
```bash
# 运行 CLI 浏览器验证
npm run harness:verify:cli
```

详细说明请查看 [CLI Browser 验证文档](./cli-browser-verify/README.md)。

## 📋 历史背景

"Harness" 代表 **Human-Augmenting Reverse-engineering Neural System**，是本项目的方法论核心：

1. **架构健康检查** - 通过自动化脚本保持代码质量
2. **上下文工程** - 为 AI Agent 提供结构化知识
3. **约束驱动开发** - 使用 Linter 规则强制执行架构规范

## 🔗 相关链接

- [Scripts 目录](../scripts/) - 开发脚本集合
- [项目架构](../ARCHITECTURE.md) - 系统架构说明
- [AGENTS.md](../AGENTS.md) - AI Agent 使用指南
