/**
 * PRD 可行性评估 Hook（使用 Rust 后端）
 *
 * 调用 Rust 实现的 PRD 可行性评估器
 */

import { useState, useCallback } from 'react'
import { invoke } from '@tauri-apps/api/core'
import type { PRDFeasibilityReport } from '@/types/prd-quality'
import type { PRD } from '@/types'

interface UsePRDFeasibilityCheckReturn {
  /** 可行性评估报告 */
  report: PRDFeasibilityReport | null
  /** 是否正在评估 */
  isAssessing: boolean
  /** 错误信息 */
  error: string | null
  /** 执行可行性评估 */
  assessFeasibility: (prd: PRD) => Promise<void>
  /** 重置评估结果 */
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
 * PRD 可行性评估 Hook
 */
export function usePRDFeasibilityCheck(): UsePRDFeasibilityCheckReturn {
  const [report, setReport] = useState<PRDFeasibilityReport | null>(null)
  const [isAssessing, setIsAssessing] = useState(false)
  const [error, setError] = useState<string | null>(null)

  /**
   * 执行可行性评估
   */
  const assessFeasibility = useCallback(async (prd: PRD) => {
    setIsAssessing(true)
    setError(null)

    try {
      // 将 PRD 转换为 Markdown
      const markdownContent = prdToMarkdown(prd)

      // 调用 Rust 后端进行可行性评估
      const result = await invoke<PRDFeasibilityReport>('assess_prd_feasibility', {
        request: {
          prd_content: markdownContent,
        },
      })

      setReport(result)
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : '可行性评估失败'
      setError(errorMessage)
      console.error('PRD feasibility assessment failed:', err)
    } finally {
      setIsAssessing(false)
    }
  }, [])

  /**
   * 重置评估结果
   */
  const reset = useCallback(() => {
    setReport(null)
    setIsAssessing(false)
    setError(null)
  }, [])

  return {
    report,
    isAssessing,
    error,
    assessFeasibility,
    reset,
  }
}
