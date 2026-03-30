/**
 * PRD 迭代优化 Hook
 */

import { useState, useCallback } from 'react'
import { invoke } from '@tauri-apps/api/core'
import type { PRD } from '@/types'
import type { IterationResponse, IterationHistory } from '@/types/prd-iteration'

interface UsePRDIterationReturn {
  /** 当前版本 ID */
  currentVersionId: string | null
  /** 迭代历史 */
  history: IterationHistory | null
  /** 是否正在迭代 */
  isIterating: boolean
  /** 错误信息 */
  error: string | null
  /** 创建初始版本 */
  createInitialVersion: (prd: PRD) => Promise<string>
  /** 执行迭代优化 */
  iterateWithFeedback: (
    prd: PRD,
    feedback: string,
    qualitySummary?: string
  ) => Promise<IterationResponse>
  /** 获取迭代历史 */
  getHistory: () => Promise<IterationHistory>
  /** 重置 */
  reset: () => void
}

/**
 * 将 PRD 对象转换为 JSON 字符串
 */
function prdToJson(prd: PRD): string {
  return JSON.stringify(prd)
}

/**
 * 将 JSON 字符串解析为 PRD 对象
 */
function jsonToPrd(json: string): PRD {
  return JSON.parse(json)
}

/**
 * PRD 迭代优化 Hook
 */
export function usePRDIteration(): UsePRDIterationReturn {
  const [currentVersionId, setCurrentVersionId] = useState<string | null>(null)
  const [history, setHistory] = useState<IterationHistory | null>(null)
  const [isIterating, setIsIterating] = useState(false)
  const [error, setError] = useState<string | null>(null)

  /**
   * 创建初始版本
   */
  const createInitialVersion = useCallback(async (prd: PRD): Promise<string> => {
    setIsIterating(true)
    setError(null)

    try {
      const prdJson = prdToJson(prd)
      const response = await invoke<{ version_id: string }>('create_initial_version', {
        request: {
          prd_json: prdJson,
        },
      })

      setCurrentVersionId(response.version_id)
      return response.version_id
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : '创建初始版本失败'
      setError(errorMessage)
      console.error('Failed to create initial version:', err)
      throw err
    } finally {
      setIsIterating(false)
    }
  }, [])

  /**
   * 执行迭代优化
   */
  const iterateWithFeedback = useCallback(
    async (prd: PRD, feedback: string, qualitySummary?: string): Promise<IterationResponse> => {
      setIsIterating(true)
      setError(null)

      try {
        const prdJson = prdToJson(prd)
        const response = await invoke<IterationResponse>('iterate_prd', {
          request: {
            current_prd_json: prdJson,
            user_feedback: feedback,
            quality_summary: qualitySummary || null,
          },
        })

        // 更新当前版本 ID
        setCurrentVersionId(response.newVersionId)

        return response
      } catch (err) {
        const errorMessage = err instanceof Error ? err.message : '迭代优化失败'
        setError(errorMessage)
        console.error('Failed to iterate PRD:', err)
        throw err
      } finally {
        setIsIterating(false)
      }
    },
    []
  )

  /**
   * 获取迭代历史
   */
  const getHistory = useCallback(async (): Promise<IterationHistory> => {
    try {
      const response = await invoke<IterationHistory>('get_iteration_history', {
        request: {},
      })
      setHistory(response)
      return response
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : '获取迭代历史失败'
      setError(errorMessage)
      console.error('Failed to get iteration history:', err)
      throw err
    }
  }, [])

  /**
   * 重置
   */
  const reset = useCallback(() => {
    setCurrentVersionId(null)
    setHistory(null)
    setIsIterating(false)
    setError(null)
  }, [])

  return {
    currentVersionId,
    history,
    isIterating,
    error,
    createInitialVersion,
    iterateWithFeedback,
    getHistory,
    reset,
  }
}
