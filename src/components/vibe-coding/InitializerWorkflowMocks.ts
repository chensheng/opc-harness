/**
 * Initializer Agent 工作流的 Mock 数据
 */

export const MOCK_STEP_LOGS = {
  'prd-parsing': [
    '正在读取 PRD 文档...',
    '分析产品需求...',
    '提取功能列表...',
    '识别技术栈...',
    'PRD 解析完成！共识别 12 个核心功能',
  ],
  'env-check': [
    '检查 Git 版本...',
    '✓ Git 2.40.0 已安装',
    '检查 Node.js 版本...',
    '✓ Node.js 20.10.0 已安装',
    '检查 npm 版本...',
    '✓ npm 10.2.3 已安装',
    '检查 Rust 版本...',
    '✓ Rust 1.75.0 已安装',
    '环境检查通过！',
  ],
  'git-init': [
    '初始化 Git 仓库...',
    '创建 .gitignore 文件...',
    '配置 Git 用户信息...',
    '创建初始提交...',
    'Git 仓库初始化成功！',
  ],
  'task-decomposition': [
    '分析 PRD 功能列表...',
    '设计系统架构...',
    '创建 Milestones...',
    '分解任务为 Issues...',
    '评估优先级和依赖关系...',
    '估算工时...',
    '任务分解完成！共生成 15 个 Issues',
  ],
} as const
