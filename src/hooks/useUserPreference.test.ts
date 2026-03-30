/**
 * useUserPreference Hook 单元测试
 */

import { describe, it, expect, vi, beforeEach } from 'vitest'
import { renderHook, act } from '@testing-library/react'
import { useUserPreference } from './useUserPreference'
import type { PreferenceModel, Feedback } from '@/types/user-preference'

// Mock Tauri invoke
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}))

describe('useUserPreference', () => {
  const mockPreferences: PreferenceModel = {
    preferredStructure: ['overview', 'features', 'tech'],
    preferredTechStack: ['React', 'Rust'],
    preferredFeatureComplexity: 0.7,
    preferredDetailLevel: 0.6,
    commonModifications: [],
    feedbackKeywords: ['添加', '详细'],
  }

  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should initialize with empty state', () => {
    const { result } = renderHook(() => useUserPreference())

    expect(result.current.preferences).toBeNull()
    expect(result.current.isLoading).toBe(false)
    expect(result.current.error).toBeNull()
  })

  it('should load preferences successfully', async () => {
    const { invoke } = await import('@tauri-apps/api/core')
    vi.mocked(invoke).mockResolvedValue(mockPreferences)

    const { result } = renderHook(() => useUserPreference())

    let loaded: PreferenceModel | undefined
    await act(async () => {
      loaded = await result.current.loadPreferences()
    })

    expect(loaded).toEqual(mockPreferences)
    expect(result.current.preferences).toEqual(mockPreferences)
    expect(result.current.isLoading).toBe(false)
    expect(result.current.error).toBeNull()
    expect(invoke).toHaveBeenCalledWith('get_user_preferences', {
      request: {},
    })
  })

  it('should handle load preferences error', async () => {
    const { invoke } = await import('@tauri-apps/api/core')
    vi.mocked(invoke).mockRejectedValue(new Error('加载失败'))

    const { result } = renderHook(() => useUserPreference())

    await act(async () => {
      try {
        await result.current.loadPreferences()
      } catch (err) {
        // Expected error
      }
    })

    expect(result.current.preferences).toBeNull()
    expect(result.current.error).toBe('加载失败')
    expect(result.current.isLoading).toBe(false)
  })

  it('should update preferences successfully', async () => {
    const { invoke } = await import('@tauri-apps/api/core')
    vi.mocked(invoke).mockResolvedValue(undefined)

    const { result } = renderHook(() => useUserPreference())

    await act(async () => {
      await result.current.updatePreferences(mockPreferences)
    })

    expect(result.current.preferences).toEqual(mockPreferences)
    expect(result.current.isLoading).toBe(false)
    expect(invoke).toHaveBeenCalledWith('update_user_preferences', {
      model: mockPreferences,
    })
  })

  it('should analyze from feedback successfully', async () => {
    const { invoke } = await import('@tauri-apps/api/core')
    vi.mocked(invoke).mockResolvedValue(mockPreferences)

    const { result } = renderHook(() => useUserPreference())

    const feedbacks: Feedback[] = [
      {
        content: '请添加更多功能',
        timestamp: Date.now(),
        feedbackType: 'manual',
      },
    ]

    let analyzed: PreferenceModel | undefined
    await act(async () => {
      analyzed = await result.current.analyzeFromFeedback(feedbacks)
    })

    expect(analyzed).toEqual(mockPreferences)
    expect(result.current.preferences).toEqual(mockPreferences)
    expect(invoke).toHaveBeenCalledWith('analyze_preference_from_feedback', {
      feedbackHistory: feedbacks,
    })
  })

  it('should apply preferences to PRD', async () => {
    const { invoke } = await import('@tauri-apps/api/core')

    const testPrd = {
      title: '测试产品',
      overview: '这是一个测试产品',
      targetUsers: ['用户 A'],
      coreFeatures: ['功能 1'],
      techStack: ['Old Tech'],
      estimatedEffort: '2 周',
    }

    const optimizedPrd = {
      ...testPrd,
      techStack: ['React', 'Rust'],
      coreFeatures: ['功能 1', '基于偏好的功能'],
    }

    vi.mocked(invoke).mockResolvedValue(JSON.stringify(optimizedPrd))

    const { result } = renderHook(() => useUserPreference())

    let appliedPrd: typeof testPrd | undefined
    await act(async () => {
      appliedPrd = await result.current.applyToPrd(testPrd)
    })

    expect(appliedPrd).toEqual(optimizedPrd)
    expect(invoke).toHaveBeenCalledWith('apply_preference_to_prd', {
      prdJson: JSON.stringify(testPrd),
    })
  })

  it('should reset state correctly', async () => {
    const { invoke } = await import('@tauri-apps/api/core')
    vi.mocked(invoke).mockResolvedValue(mockPreferences)

    const { result } = renderHook(() => useUserPreference())

    // 先加载偏好
    await act(async () => {
      await result.current.loadPreferences()
    })

    expect(result.current.preferences).not.toBeNull()

    // 重置
    act(() => {
      result.current.reset()
    })

    expect(result.current.preferences).toBeNull()
    expect(result.current.isLoading).toBe(false)
    expect(result.current.error).toBeNull()
  })

  it('should support multiple preference updates', async () => {
    const { invoke } = await import('@tauri-apps/api/core')
    vi.mocked(invoke).mockResolvedValue(undefined)

    const { result } = renderHook(() => useUserPreference())

    const pref1: PreferenceModel = {
      ...mockPreferences,
      preferredFeatureComplexity: 0.5,
    }

    const pref2: PreferenceModel = {
      ...mockPreferences,
      preferredFeatureComplexity: 0.8,
    }

    // 第一次更新
    await act(async () => {
      await result.current.updatePreferences(pref1)
    })

    expect(result.current.preferences?.preferredFeatureComplexity).toBe(0.5)

    // 第二次更新
    await act(async () => {
      await result.current.updatePreferences(pref2)
    })

    expect(result.current.preferences?.preferredFeatureComplexity).toBe(0.8)
    expect(invoke).toHaveBeenCalledTimes(2)
  })
})
