/**
 * 用户偏好学习 Hook
 */

import { useState, useCallback } from 'react'
import { invoke } from '@tauri-apps/api/core'
import type { PreferenceModel, Feedback } from '@/types/user-preference'
import type { PRD } from '@/types'

interface UseUserPreferenceReturn {
  /** 当前偏好模型 */
  preferences: PreferenceModel | null
  /** 是否正在加载 */
  isLoading: boolean
  /** 错误信息 */
  error: string | null
  /** 获取用户偏好 */
  loadPreferences: () => Promise<PreferenceModel>
  /** 更新用户偏好 */
  updatePreferences: (model: PreferenceModel) => Promise<void>
  /** 从反馈中分析偏好 */
  analyzeFromFeedback: (feedbackHistory: Feedback[]) => Promise<PreferenceModel>
  /** 应用偏好到 PRD */
  applyToPrd: (prd: PRD) => Promise<PRD>
  /** 重置 */
  reset: () => void
}

/**
 * 用户偏好学习 Hook
 */
export function useUserPreference(): UseUserPreferenceReturn {
  const [preferences, setPreferences] = useState<PreferenceModel | null>(null)
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  /**
   * 获取用户偏好
   */
  const loadPreferences = useCallback(async (): Promise<PreferenceModel> => {
    setIsLoading(true)
    setError(null)

    try {
      const response = await invoke<PreferenceModel>('get_user_preferences', {
        request: {},
      })
      setPreferences(response)
      return response
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : '获取用户偏好失败'
      setError(errorMessage)
      console.error('Failed to load user preferences:', err)
      throw err
    } finally {
      setIsLoading(false)
    }
  }, [])

  /**
   * 更新用户偏好
   */
  const updatePreferences = useCallback(async (model: PreferenceModel): Promise<void> => {
    setIsLoading(true)
    setError(null)

    try {
      await invoke('update_user_preferences', {
        model,
      })
      setPreferences(model)
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : '更新用户偏好失败'
      setError(errorMessage)
      console.error('Failed to update user preferences:', err)
      throw err
    } finally {
      setIsLoading(false)
    }
  }, [])

  /**
   * 从反馈中分析偏好
   */
  const analyzeFromFeedback = useCallback(
    async (feedbackHistory: Feedback[]): Promise<PreferenceModel> => {
      setIsLoading(true)
      setError(null)

      try {
        const response = await invoke<PreferenceModel>('analyze_preference_from_feedback', {
          feedbackHistory,
        })
        setPreferences(response)
        return response
      } catch (err) {
        const errorMessage = err instanceof Error ? err.message : '分析用户偏好失败'
        setError(errorMessage)
        console.error('Failed to analyze user preferences:', err)
        throw err
      } finally {
        setIsLoading(false)
      }
    },
    []
  )

  /**
   * 应用偏好到 PRD
   */
  const applyToPrd = useCallback(async (prd: PRD): Promise<PRD> => {
    setIsLoading(true)
    setError(null)

    try {
      const prdJson = JSON.stringify(prd)
      const result = await invoke<string>('apply_preference_to_prd', {
        prdJson,
      })
      return JSON.parse(result)
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : '应用偏好失败'
      setError(errorMessage)
      console.error('Failed to apply preferences to PRD:', err)
      throw err
    } finally {
      setIsLoading(false)
    }
  }, [])

  /**
   * 重置
   */
  const reset = useCallback(() => {
    setPreferences(null)
    setIsLoading(false)
    setError(null)
  }, [])

  return {
    preferences,
    isLoading,
    error,
    loadPreferences,
    updatePreferences,
    analyzeFromFeedback,
    applyToPrd,
    reset,
  }
}
