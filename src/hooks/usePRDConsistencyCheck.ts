/**
 * PRD 一致性检查 Hook（使用 Rust 后端）
 *
 * 调用 Rust 实现的 PRD 一致性检查器
 */

import { useState, useCallback } from 'react'
import { invoke } from '@tauri-apps/api/core'
import type { PRDConsistencyReport } from '@/types/prd-quality'
import type { PRD } from '@/types'

interface UsePRDConsistencyCheckReturn {
  /** 一致性检查报告 */
  report: PRDConsistencyReport | null
  /** 是否正在检查 */
  isChecking: boolean
  /** 错误信息 */
  error: string | null
  /** 执行一致性检查 */
  checkConsistency: (prd: PRD) => Promise<void>
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
 * PRD 一致性检查 Hook
 */
export function usePRDConsistencyCheck(): UsePRDConsistencyCheckReturn {
  const [report, setReport] = useState<PRDConsistencyReport | null>(null)
  const [isChecking, setIsChecking] = useState(false)
  const [error, setError] = useState<string | null>(null)

  /**
   * 执行一致性检查
   */
  const checkConsistency = useCallback(async (prd: PRD) => {
    setIsChecking(true)
    setError(null)

    try {
      // 将 PRD 转换为 Markdown
      const markdownContent = prdToMarkdown(prd)

      // 调用 Rust 后端进行一致性检查
      const result = await invoke<PRDConsistencyReport>('check_prd_consistency', {
        request: {
          prd_content: markdownContent,
        },
      })

      setReport(result)
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : '一致性检查失败'
      setError(errorMessage)
      console.error('PRD consistency check failed:', err)
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
    checkConsistency,
    reset,
  }
}
