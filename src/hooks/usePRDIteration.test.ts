/**
 * usePRDIteration Hook 单元测试
 */

import { describe, it, expect, vi, beforeEach } from 'vitest'
import { renderHook, act } from '@testing-library/react'
import { usePRDIteration } from './usePRDIteration'
import type { PRD } from '@/types'

// Mock Tauri invoke
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}))

describe('usePRDIteration', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should initialize with empty state', () => {
    const { result } = renderHook(() => usePRDIteration())

    expect(result.current.currentVersionId).toBeNull()
    expect(result.current.history).toBeNull()
    expect(result.current.isIterating).toBe(false)
    expect(result.current.error).toBeNull()
  })

  it('should create initial version successfully', async () => {
    const mockVersionId = 'test-version-123'
    const { invoke } = await import('@tauri-apps/api/core')
    vi.mocked(invoke).mockResolvedValue({ version_id: mockVersionId })

    const { result } = renderHook(() => usePRDIteration())

    const testPRD: PRD = {
      title: '测试产品',
      overview: '这是一个测试产品',
      targetUsers: ['用户 A'],
      coreFeatures: ['功能 1'],
      techStack: ['React'],
      estimatedEffort: '2 周',
    }

    let versionId: string | undefined
    await act(async () => {
      versionId = await result.current.createInitialVersion(testPRD)
    })

    expect(versionId).toBe(mockVersionId)
    expect(result.current.currentVersionId).toBe(mockVersionId)
    expect(result.current.isIterating).toBe(false)
    expect(result.current.error).toBeNull()
    expect(invoke).toHaveBeenCalledWith('create_initial_version', {
      request: {
        prd_json: JSON.stringify(testPRD),
      },
    })
  })

  it('should handle create initial version error', async () => {
    const { invoke } = await import('@tauri-apps/api/core')
    vi.mocked(invoke).mockRejectedValue(new Error('创建失败'))

    const { result } = renderHook(() => usePRDIteration())

    const testPRD: PRD = {
      title: '测试产品',
      overview: '这是一个测试产品',
      targetUsers: ['用户 A'],
      coreFeatures: ['功能 1'],
      techStack: ['React'],
      estimatedEffort: '2 周',
    }

    await act(async () => {
      try {
        await result.current.createInitialVersion(testPRD)
      } catch (err) {
        // Expected error
      }
    })

    expect(result.current.error).toBe('创建失败')
    expect(result.current.isIterating).toBe(false)
  })

  it('should iterate with feedback successfully', async () => {
    const mockResponse = {
      newVersionId: 'new-version-456',
      optimizedPrd: {
        title: '优化后的产品',
        overview: '这是一个优化后的测试产品',
        targetUsers: ['用户 A', '用户 B'],
        coreFeatures: ['功能 1', '功能 2', '基于用户反馈新增的功能'],
        techStack: ['React', 'Rust'],
        estimatedEffort: '3 周',
      },
      diff: {
        addedFeatures: ['功能 2', '基于用户反馈新增的功能'],
        removedFeatures: [],
        modifiedFieldsCount: 2,
      },
      iterationNumber: 1,
    }

    const { invoke } = await import('@tauri-apps/api/core')
    vi.mocked(invoke).mockResolvedValue(mockResponse)

    const { result } = renderHook(() => usePRDIteration())

    const testPRD: PRD = {
      title: '测试产品',
      overview: '这是一个测试产品',
      targetUsers: ['用户 A'],
      coreFeatures: ['功能 1'],
      techStack: ['React'],
      estimatedEffort: '2 周',
    }

    let response: typeof mockResponse | undefined
    await act(async () => {
      response = await result.current.iterateWithFeedback(
        testPRD,
        '请添加更多功能',
        '完整性评分较低'
      )
    })

    expect(response).toEqual(mockResponse)
    expect(result.current.currentVersionId).toBe('new-version-456')
    expect(result.current.isIterating).toBe(false)
    expect(invoke).toHaveBeenCalledWith('iterate_prd', {
      request: {
        current_prd_json: JSON.stringify(testPRD),
        user_feedback: '请添加更多功能',
        quality_summary: '完整性评分较低',
      },
    })
  })

  it('should reset state correctly', async () => {
    const { invoke } = await import('@tauri-apps/api/core')
    vi.mocked(invoke).mockResolvedValue({ version_id: 'test-id' })

    const { result } = renderHook(() => usePRDIteration())

    const testPRD: PRD = {
      title: '测试产品',
      overview: '这是一个测试产品',
      targetUsers: ['用户 A'],
      coreFeatures: ['功能 1'],
      techStack: ['React'],
      estimatedEffort: '2 周',
    }

    // 先创建版本
    await act(async () => {
      await result.current.createInitialVersion(testPRD)
    })

    expect(result.current.currentVersionId).not.toBeNull()

    // 重置
    act(() => {
      result.current.reset()
    })

    expect(result.current.currentVersionId).toBeNull()
    expect(result.current.history).toBeNull()
    expect(result.current.isIterating).toBe(false)
    expect(result.current.error).toBeNull()
  })

  it('should support multiple iterations', async () => {
    const { invoke } = await import('@tauri-apps/api/core')

    const testPRD: PRD = {
      title: '测试产品',
      overview: '这是一个测试产品',
      targetUsers: ['用户 A'],
      coreFeatures: ['功能 1'],
      techStack: ['React'],
      estimatedEffort: '2 周',
    }

    const { result } = renderHook(() => usePRDIteration())

    // 执行 3 轮迭代
    for (let i = 1; i <= 3; i++) {
      vi.mocked(invoke).mockResolvedValue({
        newVersionId: `version-${i}`,
        optimizedPrd: {
          ...testPRD,
          coreFeatures: [...testPRD.coreFeatures, `功能 ${i + 1}`],
        },
        diff: {
          addedFeatures: [`功能 ${i + 1}`],
          removedFeatures: [],
          modifiedFieldsCount: 0,
        },
        iterationNumber: i as number,
      })

      await act(async () => {
        await result.current.iterateWithFeedback(testPRD, `第 ${i} 轮反馈`)
      })

      expect(result.current.currentVersionId).toBe(`version-${i}`)
    }

    // 验证调用了 3 次
    expect(invoke).toHaveBeenCalledTimes(3)
  })
})
