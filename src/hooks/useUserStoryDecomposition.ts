import { useState, useCallback } from 'react'
import { invoke } from '@tauri-apps/api/core'
import type { DecomposeUserStoriesRequest, DecomposeUserStoriesResponse, UserStory } from '../types'
import { useAIConfigStore } from '@/stores/aiConfigStore'
import { useUserStoryStore } from '@/stores/userStoryStore'

interface UseUserStoryDecompositionReturn {
  /** 用户故事列表 */
  userStories: UserStory[]
  /** 是否正在拆分 */
  loading: boolean
  /** 错误信息 */
  error: string | null
  /** 执行用户故事拆分，返回拆分后的用户故事数组 */
  decompose: (prdContent: string, projectId?: string) => Promise<UserStory[]>
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
  const aiConfigStore = useAIConfigStore()
  const userStoryStore = useUserStoryStore()

  /**
   * 执行用户故事拆分
   * @param prdContent - PRD 内容或功能描述
   * @param projectId - 可选的项目ID，用于获取已有用户故事避免重复
   * @returns 拆分后的用户故事数组
   */
  const decompose = useCallback(
    async (prdContent: string, projectId?: string): Promise<UserStory[]> => {
      setLoading(true)
      setError(null)

      try {
        // 从 AI 配置 Store 获取当前激活的配置
        const activeConfig = aiConfigStore.getActiveConfig()

        if (!activeConfig?.apiKey) {
          throw new Error('未配置 AI API Key，请先在设置中配置')
        }

        // 如果提供了 projectId，获取已有的用户故事
        let existingStories: Array<{ title: string; role: string; feature: string }> | undefined
        
        if (projectId) {
          const stories = userStoryStore.getProjectStories(projectId)
          if (stories.length > 0) {
            // 提取关键信息用于避免重复
            existingStories = stories.map(story => ({
              title: story.title,
              role: story.role,
              feature: story.feature,
            }))
            console.log(`[useUserStoryDecomposition] Found ${existingStories.length} existing stories to avoid duplication`)
          }
        }

        const request: DecomposeUserStoriesRequest = {
          prdContent,
          provider: activeConfig.provider,
          model: activeConfig.model,
          apiKey: activeConfig.apiKey,
          projectId,
          existingStories,
        }

        const response = await invoke<DecomposeUserStoriesResponse>('decompose_user_stories', {
          request,
        })

        if (response.success) {
          setUserStories(response.userStories)
          return response.userStories
        } else {
          setError(response.errorMessage || '拆分失败')
          return []
        }
      } catch (err) {
        const errorMessage = err instanceof Error ? err.message : '未知错误'
        setError(errorMessage)
        console.error('User story decomposition failed:', err)
        return []
      } finally {
        setLoading(false)
      }
    },
    [aiConfigStore, userStoryStore]
  )

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