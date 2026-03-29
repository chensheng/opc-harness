import { useState, useCallback } from 'react'
import type { PRD } from '@/types'
import type { PRDQualityReport } from '@/types/prd-quality'
import { defaultQualityChecker } from '@/lib/prd-quality-checker'

/**
 * PRD 质量检查 Hook 的返回值
 */
interface UsePRDQualityReturn {
  /** 是否正在检查 */
  isChecking: boolean
  /** 检查进度 (0-100) */
  progress: number
  /** 错误信息 */
  error: string | null
  /** 质量报告 */
  qualityReport: PRDQualityReport | null
  /** 总体评分 */
  overallScore: number | null
  /** 开始质量检查 */
  checkQuality: (prd: PRD) => Promise<void>
  /** 重置状态 */
  reset: () => void
}

/**
 * PRD 质量检查 Hook
 *
 * 用于对 PRD 进行自动化的完整性检查和质量评估
 *
 * @example
 * ```tsx
 * const { qualityReport, overallScore, checkQuality } = usePRDQuality()
 *
 * await checkQuality(prd)
 * console.log(`PRD 评分：${overallScore}`)
 * ```
 */
export function usePRDQuality(): UsePRDQualityReturn {
  // 状态管理
  const [isChecking, setIsChecking] = useState(false)
  const [progress, setProgress] = useState(0)
  const [error, setError] = useState<string | null>(null)
  const [qualityReport, setQualityReport] = useState<PRDQualityReport | null>(null)
  const [overallScore, setOverallScore] = useState<number | null>(null)

  /**
   * 开始质量检查
   */
  const checkQuality = useCallback(async (prd: PRD) => {
    try {
      // 重置状态
      setIsChecking(true)
      setProgress(0)
      setError(null)
      setQualityReport(null)
      setOverallScore(null)

      // 模拟进度更新（实际检查很快，但为了 UX 添加进度动画）
      const progressInterval = setInterval(() => {
        setProgress(prev => {
          if (prev >= 90) {
            clearInterval(progressInterval)
            return 90
          }
          return prev + 10
        })
      }, 100)

      // 执行质量检查
      const report = await defaultQualityChecker.checkQuality(prd)

      // 清除进度定时器
      clearInterval(progressInterval)
      setProgress(100)

      // 更新结果
      setQualityReport(report)
      setOverallScore(report.overallScore)
      setIsChecking(false)
    } catch (err) {
      // 错误处理
      const errorMessage = err instanceof Error ? err.message : '质量检查失败'
      setError(errorMessage)
      setIsChecking(false)
      setProgress(0)
    }
  }, [])

  /**
   * 重置状态
   */
  const reset = useCallback(() => {
    setIsChecking(false)
    setProgress(0)
    setError(null)
    setQualityReport(null)
    setOverallScore(null)
  }, [])

  return {
    isChecking,
    progress,
    error,
    qualityReport,
    overallScore,
    checkQuality,
    reset,
  }
}
