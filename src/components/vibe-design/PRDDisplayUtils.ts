import type { PRD } from '@/types'

// 将 PRD 对象转换为 Markdown 格式
export function convertPRDToMarkdown(prdData: PRD): string {
  let markdown = `# ${prdData.title}\n\n`

  markdown += `## 产品概述\n\n${prdData.overview}\n\n`

  markdown += `## 目标用户\n\n`
  prdData.targetUsers.forEach(user => {
    markdown += `- ${user}\n`
  })
  markdown += `\n`

  markdown += `## 核心功能\n\n`
  prdData.coreFeatures.forEach(feature => {
    markdown += `- ${feature}\n`
  })
  markdown += `\n`

  markdown += `## 技术栈\n\n`
  prdData.techStack.forEach(tech => {
    markdown += `- ${tech}\n`
  })
  markdown += `\n`

  markdown += `## 预估工作量\n\n${prdData.estimatedEffort}\n\n`

  if (prdData.businessModel) {
    markdown += `## 商业模式\n\n${prdData.businessModel}\n\n`
  }

  if (prdData.pricing) {
    markdown += `## 定价策略\n\n${prdData.pricing}\n\n`
  }

  return markdown.trim()
}

// 从 Markdown 内容解析回 PRD 对象
export function parseMarkdownToPRD(markdown: string): PRD {
  const lines = markdown.split('\n')
  const prd: Partial<PRD> = {
    title: '',
    overview: '',
    targetUsers: [],
    coreFeatures: [],
    techStack: [],
    estimatedEffort: '',
    businessModel: '',
    pricing: '',
  }

  let currentSection = ''
  let currentContent: string[] = []

  const saveCurrentSection = () => {
    const content = currentContent.join('\n').trim()
    if (!content) return

    switch (currentSection.toLowerCase()) {
      case '产品概述':
        prd.overview = content
        break
      case '目标用户':
        prd.targetUsers = content
          .split('\n')
          .filter(line => line.startsWith('-'))
          .map(line => line.replace(/^- /, '').trim())
        break
      case '核心功能':
        prd.coreFeatures = content
          .split('\n')
          .filter(line => line.startsWith('-'))
          .map(line => line.replace(/^- /, '').trim())
        break
      case '技术栈':
        prd.techStack = content
          .split('\n')
          .filter(line => line.startsWith('-'))
          .map(line => line.replace(/^- /, '').trim())
        break
      case '预估工作量':
        prd.estimatedEffort = content
        break
      case '商业模式':
        prd.businessModel = content
        break
      case '定价策略':
        prd.pricing = content
        break
    }
  }

  for (const line of lines) {
    const trimmedLine = line.trim()

    // 检测标题 (# 开头)
    if (trimmedLine.startsWith('# ') && !currentSection) {
      // 第一个 # 标题是产品标题
      prd.title = trimmedLine.replace(/^# /, '').trim()
      continue
    }

    // 检测章节 (## 开头)
    if (trimmedLine.startsWith('## ')) {
      // 保存之前的章节内容
      saveCurrentSection()

      // 开始新章节
      currentSection = trimmedLine.replace(/^## /, '').trim()
      currentContent = []
      continue
    }

    // 收集内容（保留空行，只跳过标题）
    if (!trimmedLine.startsWith('#')) {
      currentContent.push(line)
    }
  }

  // 保存最后一个章节
  if (currentSection) {
    saveCurrentSection()
  }

  // 如果标题为空，使用默认值
  if (!prd.title) {
    prd.title = '产品需求文档'
  }

  return prd as PRD
}

// Simulated AI-generated PRD (fallback)
export function generateMockPRD(idea: string): PRD {
  return {
    title: idea.slice(0, 30) + (idea.length > 30 ? '...' : ''),
    overview: `这是一个基于用户想法「${idea.slice(0, 50)}...」的产品。该产品旨在解决目标用户的核心痛点，提供简洁高效的解决方案。`,
    targetUsers: ['独立开发者', '自由职业者', '技术型创业者', '小型团队负责人'],
    coreFeatures: [
      '直观的用户界面，降低学习成本',
      '核心功能模块化，按需使用',
      '数据同步和备份机制',
      '多平台支持（Web、移动端）',
      'API 接口开放，支持第三方集成',
    ],
    techStack: ['React', 'Node.js', 'PostgreSQL', 'Redis', 'Docker'],
    estimatedEffort: '2-4 周',
    businessModel: 'Freemium 模式，基础功能免费，高级功能订阅制',
    pricing: '免费版：基础功能；Pro 版：$9/月；Team 版：$29/月',
  }
}
