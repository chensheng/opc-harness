import { useState, useCallback } from 'react'
import { invoke } from '@tauri-apps/api/core'
import type { PrdAnalysis, AnalyzePRDDepthRequest, AnalyzePRDDepthResponse } from '../types'

interface UsePRDAnalysisReturn {
  /** 分析结果 */
  analysis: PrdAnalysis | null
  /** 是否正在分析 */
  loading: boolean
  /** 错误信息 */
  error: string | null
  /** 执行深度分析 */
  analyze: (prdContent: string, apiKey?: string) => Promise<void>
  /** 重置状态 */
  reset: () => void
}

/**
 * PRD 深度分析 Hook
 */
export function usePRDAnalysis(): UsePRDAnalysisReturn {
  const [analysis, setAnalysis] = useState<PrdAnalysis | null>(null)
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  /**
   * 执行深度分析
   */
  const analyze = useCallback(async (prdContent: string, apiKey?: string) => {
    setLoading(true)
    setError(null)

    try {
      const response = await invoke<AnalyzePRDDepthResponse>('analyze_prd_depth', {
        prdContent,
        apiKey,
      })

      if (response.success) {
        setAnalysis(response.analysis)
      } else {
        setError(response.errorMessage || '分析失败')
      }
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : '未知错误'
      setError(errorMessage)
      console.error('PRD deep analysis failed:', err)
    } finally {
      setLoading(false)
    }
  }, [])

  /**
   * 重置状态
   */
  const reset = useCallback(() => {
    setAnalysis(null)
    setLoading(false)
    setError(null)
  }, [])

  return {
    analysis,
    loading,
    error,
    analyze,
    reset,
  }
}
