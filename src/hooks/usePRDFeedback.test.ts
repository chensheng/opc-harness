/**
 * usePRDFeedback Hook 单元测试
 */

import { renderHook, act } from '@testing-library/react'
import { describe, it, expect, vi, beforeEach } from 'vitest'
import { usePRDFeedback } from './usePRDFeedback'
import type { SubmitFeedbackResponse } from '../types/prd-feedback'

// Mock invoke
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}))

const { invoke } = await import('@tauri-apps/api/core')

describe('usePRDFeedback', () => {
  const mockPrdId = 'test-prd-123'
  const mockPrdContent = '# Test PRD\n\nContent here.'
  const mockFeedbackContent = '用户画像部分需要更详细'

  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should initialize with empty state', () => {
    const { result } = renderHook(() => usePRDFeedback(mockPrdId))

    expect(result.current.isLoading).toBe(false)
    expect(result.current.error).toBeNull()
    expect(result.current.feedbacks).toEqual([])
    expect(result.current.iterationCount).toBe(0)
    expect(result.current.lastResult).toBeNull()
  })

  it('should submit feedback successfully', async () => {
    const mockResponse = {
      new_prd_content: '# Optimized PRD',
      changed_sections: ['用户画像'],
      quality_score_before: 85.0,
      quality_score_after: 90.0,
      iteration_number: 1,
      success: true,
    }

    ;(invoke as any).mockResolvedValueOnce(mockResponse)

    const { result } = renderHook(() => usePRDFeedback(mockPrdId))

    let responseResult: SubmitFeedbackResponse | null = null
    await act(async () => {
      responseResult = await result.current.submitFeedback(
        mockPrdContent,
        mockFeedbackContent,
        '用户画像'
      )
    })

    expect(invoke).toHaveBeenCalledWith('submit_feedback_and_regenerate', {
      request: {
        prd_id: mockPrdId,
        prd_content: mockPrdContent,
        feedback_content: mockFeedbackContent,
        section: '用户画像',
        iteration_count: 0,
      },
    })

    expect(responseResult).toEqual(mockResponse)
    expect(result.current.isLoading).toBe(false)
    expect(result.current.iterationCount).toBe(1)
    expect(result.current.lastResult).toEqual(mockResponse)
    expect(result.current.feedbacks.length).toBe(1)
  })

  it('should handle feedback submission error', async () => {
    const mockError = new Error('评估失败')
    ;(invoke as any).mockRejectedValueOnce(mockError)

    const { result } = renderHook(() => usePRDFeedback(mockPrdId))

    let responseResult: SubmitFeedbackResponse | null = null
    await act(async () => {
      responseResult = await result.current.submitFeedback(mockPrdContent, mockFeedbackContent)
    })

    expect(responseResult).toBeNull()
    expect(result.current.isLoading).toBe(false)
    expect(result.current.error).toBe('评估失败')
  })

  it('should reset state correctly', async () => {
    const mockResponse = {
      new_prd_content: '# Optimized PRD',
      changed_sections: [],
      quality_score_before: 85.0,
      quality_score_after: 90.0,
      iteration_number: 1,
      success: true,
    }

    ;(invoke as any).mockResolvedValueOnce(mockResponse)

    const { result } = renderHook(() => usePRDFeedback(mockPrdId))

    await act(async () => {
      await result.current.submitFeedback(mockPrdContent, mockFeedbackContent)
    })

    // 验证状态已更新
    expect(result.current.iterationCount).toBe(1)
    expect(result.current.feedbacks.length).toBe(1)

    // 重置状态
    act(() => {
      result.current.reset()
    })

    expect(result.current.isLoading).toBe(false)
    expect(result.current.error).toBeNull()
    expect(result.current.feedbacks).toEqual([])
    expect(result.current.iterationCount).toBe(0)
    expect(result.current.lastResult).toBeNull()
  })

  it('should clear error correctly', async () => {
    const mockError = new Error('测试错误')
    ;(invoke as any).mockRejectedValueOnce(mockError)

    const { result } = renderHook(() => usePRDFeedback(mockPrdId))

    await act(async () => {
      await result.current.submitFeedback(mockPrdContent, mockFeedbackContent)
    })

    expect(result.current.error).toBe('测试错误')

    act(() => {
      result.current.clearError()
    })

    expect(result.current.error).toBeNull()
  })

  it('should track multiple iterations', async () => {
    const mockResponses = [
      {
        new_prd_content: '# Version 1',
        changed_sections: ['用户画像'],
        quality_score_before: 85.0,
        quality_score_after: 88.0,
        iteration_number: 1,
        success: true,
      },
      {
        new_prd_content: '# Version 2',
        changed_sections: ['功能需求'],
        quality_score_before: 88.0,
        quality_score_after: 92.0,
        iteration_number: 2,
        success: true,
      },
    ]

    ;(invoke as any).mockResolvedValueOnce(mockResponses[0]).mockResolvedValueOnce(mockResponses[1])

    const { result } = renderHook(() => usePRDFeedback(mockPrdId))

    // 第一次迭代
    await act(async () => {
      await result.current.submitFeedback(mockPrdContent, '第一次反馈')
    })

    expect(result.current.iterationCount).toBe(1)
    expect(result.current.feedbacks.length).toBe(1)

    // 第二次迭代
    await act(async () => {
      await result.current.submitFeedback(mockResponses[0].new_prd_content, '第二次反馈')
    })

    expect(result.current.iterationCount).toBe(2)
    expect(result.current.feedbacks.length).toBe(2)
  })
})
