import { useState, useCallback } from 'react'
import { invoke } from '@tauri-apps/api/core'
import type { DecomposeUserStoriesRequest, DecomposeUserStoriesResponse, UserStory } from '../types'

interface UseUserStoryDecompositionReturn {
  /** 用户故事列表 */
  userStories: UserStory[]
  /** 是否正在拆分 */
  loading: boolean
  /** 错误信息 */
  error: string | null
  /** 执行用户故事拆分 */
  decompose: (prdContent: string, apiKey?: string) => Promise<void>
  /** 重置状态 */
  reset: () => void
}

/**
 * 用户故事拆分 Hook
 *
 * 通过 AI 将 PRD 或功能描述拆分为符合 INVEST 原则的用户故事
 */
export function useUserStoryDecomposition(): UseUserStoryDecompositionReturn {
  const [userStories, setUserStories] = useState<UserStory[]>([])
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  /**
   * 执行用户故事拆分
   * @param prdContent - PRD 内容或功能描述
   * @param apiKey - 可选的 AI API Key
   */
  const decompose = useCallback(async (prdContent: string, apiKey?: string) => {
    setLoading(true)
    setError(null)

    try {
      const request: DecomposeUserStoriesRequest = {
        prdContent,
        apiKey,
      }

      const response = await invoke<DecomposeUserStoriesResponse>('decompose_user_stories', {
        request,
      })

      if (response.success) {
        setUserStories(response.userStories)
      } else {
        setError(response.errorMessage || '拆分失败')
      }
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : '未知错误'
      setError(errorMessage)
      console.error('User story decomposition failed:', err)
    } finally {
      setLoading(false)
    }
  }, [])

  /**
   * 重置状态
   */
  const reset = useCallback(() => {
    setUserStories([])
    setLoading(false)
    setError(null)
  }, [])

  return {
    userStories,
    loading,
    error,
    decompose,
    reset,
  }
}
