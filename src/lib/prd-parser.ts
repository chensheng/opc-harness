import type { PRD } from '@/types'

/**
 * 从 Markdown 内容解析回 PRD 对象
 */
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

    // 检测章节标题 (## 开头)
    if (trimmedLine.startsWith('## ')) {
      // 保存上一个章节
      saveCurrentSection()

      // 开始新章节
      currentSection = trimmedLine.replace(/^## /, '').trim()
      currentContent = []
      continue
    }

    // 收集章节内容
    if (currentSection && trimmedLine && !trimmedLine.startsWith('#')) {
      currentContent.push(trimmedLine)
    }
  }

  // 保存最后一个章节
  saveCurrentSection()

  return prd as PRD
}
