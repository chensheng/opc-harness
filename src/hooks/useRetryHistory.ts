import { useState, useCallback } from 'react'
import { invoke } from '@tauri-apps/api/core'
import type { UserStoryRetryHistory } from '@/types'

/**
 * 重试历史 Hook
 * 提供用户故事重试历史的查询和管理功能
 */
export function useRetryHistory() {
  const [histories, setHistories] = useState<Record<string, UserStoryRetryHistory[]>>({})
  const [loading, setLoading] = useState<Record<string, boolean>>({})
  const [error, setError] = useState<Record<string, string | null>>({})

  /**
   * 加载指定用户故事的重试历史
   */
  const loadHistory = useCallback(async (storyId: string) => {
    setLoading(prev => ({ ...prev, [storyId]: true }))
    setError(prev => ({ ...prev, [storyId]: null }))

    try {
      const response = await invoke<UserStoryRetryHistory[]>('get_user_story_retry_history', {
        storyId,
      })

      setHistories(prev => ({ ...prev, [storyId]: response }))
      console.log(`[useRetryHistory] Loaded ${response.length} retry records for story ${storyId}`)
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : 'Failed to load retry history'
      setError(prev => ({ ...prev, [storyId]: errorMsg }))
      console.error('[useRetryHistory] Failed to load retry history:', err)
    } finally {
      setLoading(prev => ({ ...prev, [storyId]: false }))
    }
  }, [])

  /**
   * 获取指定用户故事的重试历史
   */
  const getHistory = useCallback(
    (storyId: string): UserStoryRetryHistory[] => {
      return histories[storyId] || []
    },
    [histories]
  )

  /**
   * 清除指定用户故事的重试历史缓存
   */
  const clearHistory = useCallback((storyId: string) => {
    setHistories(prev => {
      const newHistories = { ...prev }
      delete newHistories[storyId]
      return newHistories
    })
    setError(prev => {
      const newErrors = { ...prev }
      delete newErrors[storyId]
      return newErrors
    })
  }, [])

  /**
   * 检查是否有加载错误
   */
  const hasError = useCallback(
    (storyId: string): boolean => {
      return !!error[storyId]
    },
    [error]
  )

  /**
   * 获取错误信息
   */
  const getError = useCallback(
    (storyId: string): string | null => {
      return error[storyId] || null
    },
    [error]
  )

  /**
   * 检查是否正在加载
   */
  const isLoading = useCallback(
    (storyId: string): boolean => {
      return loading[storyId] || false
    },
    [loading]
  )

  return {
    histories,
    loading,
    error,
    loadHistory,
    getHistory,
    clearHistory,
    hasError,
    getError,
    isLoading,
  }
}
