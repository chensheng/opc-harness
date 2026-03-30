/**
 * PRD 质量检查 Hook（使用 Rust 后端）
 *
 * 调用 Rust 实现的 PRD 质量检查器
 */

import { useState, useCallback } from 'react'
import { invoke } from '@tauri-apps/api/core'
import type { PRDQualityReport } from '@/types/prd-quality'
import type { PRD } from '@/types'

interface UsePRDQualityCheckReturn {
  /** 质量检查报告 */
  report: PRDQualityReport | null
  /** 是否正在检查 */
  isChecking: boolean
  /** 错误信息 */
  error: string | null
  /** 执行质量检查 */
  checkQuality: (prd: PRD) => Promise<void>
  /** 重置检查结果 */
  reset: () => void
}

/**
 * 将 PRD 对象转换为 Markdown 字符串
 */
function prdToMarkdown(prd: PRD): string {
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

  if (prd.estimatedEffort) {
    markdown += `## 预估工作量\n\n${prd.estimatedEffort}\n\n`
  }

  return markdown
}

/**
 * PRD 质量检查 Hook
 */
export function usePRDQualityCheck(): UsePRDQualityCheckReturn {
  const [report, setReport] = useState<PRDQualityReport | null>(null)
  const [isChecking, setIsChecking] = useState(false)
  const [error, setError] = useState<string | null>(null)

  /**
   * 执行质量检查
   */
  const checkQuality = useCallback(async (prd: PRD) => {
    setIsChecking(true)
    setError(null)

    try {
      // 将 PRD 转换为 Markdown
      const markdownContent = prdToMarkdown(prd)

      // 调用 Rust 后端进行质量检查
      const result = await invoke<PRDQualityReport>('check_prd_quality', {
        prdContent: markdownContent,
      })

      setReport(result)
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : '质量检查失败'
      setError(errorMessage)
      console.error('PRD quality check failed:', err)
    } finally {
      setIsChecking(false)
    }
  }, [])

  /**
   * 重置检查结果
   */
  const reset = useCallback(() => {
    setReport(null)
    setIsChecking(false)
    setError(null)
  }, [])

  return {
    report,
    isChecking,
    error,
    checkQuality,
    reset,
  }
}
