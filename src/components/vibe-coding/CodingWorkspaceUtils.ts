import type { PRD } from '@/types'

/**
 * 将 PRD 对象转换为 Markdown 字符串
 */
export const prdToMarkdown = (prd: PRD | undefined): string => {
  if (!prd) return ''

  // 优先使用完整的 Markdown 内容
  if (prd.markdownContent && prd.markdownContent.trim().length > 0) {
    return prd.markdownContent
  }

  // 如果没有 markdownContent，则从结构化字段生成
  let markdown = ''

  if (prd.title && prd.title.length > 0) {
    markdown += `# ${prd.title}\n\n`
  }

  if (prd.overview) {
    markdown += `## 产品概述\n\n${prd.overview}\n\n`
  }

  if (prd.targetUsers && prd.targetUsers.length > 0) {
    markdown += `## 目标用户\n\n`
    prd.targetUsers.forEach(user => {
      markdown += `- ${user}\n`
    })
    markdown += '\n'
  }

  if (prd.coreFeatures && prd.coreFeatures.length > 0) {
    markdown += `## 核心功能\n\n`
    prd.coreFeatures.forEach(feature => {
      markdown += `- ${feature}\n`
    })
    markdown += '\n'
  }

  if (prd.techStack && prd.techStack.length > 0) {
    markdown += `## 技术栈\n\n`
    prd.techStack.forEach(tech => {
      markdown += `- ${tech}\n`
    })
    markdown += '\n'
  }

  if (prd.businessModel) {
    markdown += `## 商业模式\n\n${prd.businessModel}\n\n`
  }

  if (prd.pricing) {
    markdown += `## 定价策略\n\n${prd.pricing}\n\n`
  }

  if (prd.estimatedEffort) {
    markdown += `## 预估工作量\n\n${prd.estimatedEffort}\n\n`
  }

  return markdown
}
