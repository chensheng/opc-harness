/**
 * PRD 反馈和重新生成 Hook
 */

import { useState, useCallback } from 'react'
import { invoke } from '@tauri-apps/api/core'
import type {
  SubmitFeedbackRequest,
  SubmitFeedbackResponse,
  FeedbackState,
} from '../types/prd-feedback'

const initialState: FeedbackState = {
  isLoading: false,
  error: null,
  feedbacks: [],
  iterationCount: 0,
  lastResult: null,
}

export function usePRDFeedback(prdId: string) {
  const [state, setState] = useState<FeedbackState>(initialState)

  /**
   * 提交反馈并重新生成 PRD
   */
  const submitFeedback = useCallback(
    async (
      prdContent: string,
      feedbackContent: string,
      section?: string
    ): Promise<SubmitFeedbackResponse | null> => {
      setState(prev => ({ ...prev, isLoading: true, error: null }))

      try {
        const request: SubmitFeedbackRequest = {
          prd_id: prdId,
          prd_content: prdContent,
          feedback_content: feedbackContent,
          section,
          iteration_count: state.iterationCount,
        }

        const result = await invoke<SubmitFeedbackResponse>('submit_feedback_and_regenerate', {
          request,
        })

        // 更新状态
        setState(prev => ({
          ...prev,
          isLoading: false,
          iterationCount: result.iteration_number,
          lastResult: result,
          feedbacks: [
            ...prev.feedbacks,
            {
              id: `fb_${Date.now()}`,
              prd_id: prdId,
              section,
              content: feedbackContent,
              sentiment: 'suggestion',
              priority: 'medium',
              timestamp: new Date().toISOString(),
            },
          ],
        }))

        return result
      } catch (error) {
        const errorMessage = error instanceof Error ? error.message : '提交反馈失败'
        setState(prev => ({
          ...prev,
          isLoading: false,
          error: errorMessage,
        }))
        return null
      }
    },
    [prdId, state.iterationCount]
  )

  /**
   * 重置状态
   */
  const reset = useCallback(() => {
    setState(initialState)
  }, [])

  /**
   * 清除错误
   */
  const clearError = useCallback(() => {
    setState(prev => ({ ...prev, error: null }))
  }, [])

  return {
    ...state,
    submitFeedback,
    reset,
    clearError,
  }
}
