/**
 * usePRDVersionHistory Hook 单元测试
 */

import { describe, it, expect, vi, beforeEach } from 'vitest'
import { renderHook, act } from '@testing-library/react'
import { usePRDVersionHistory } from './usePRDVersionHistory'
import type { IterationHistory, PRDVersion } from '@/types/prd-iteration'
import type { PRD } from '@/types'

// Mock Tauri invoke
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}))

describe('usePRDVersionHistory', () => {
  const mockHistory: IterationHistory = {
    currentVersionId: 'version-3',
    versions: [
      {
        versionId: 'version-1',
        timestamp: 1000000000,
        prd: { title: 'V1' } as PRD,
        iterationNumber: 0,
        feedback: undefined,
      },
      {
        versionId: 'version-2',
        timestamp: 1000000100,
        prd: { title: 'V2' } as PRD,
        iterationNumber: 1,
        feedback: '添加更多功能',
      },
      {
        versionId: 'version-3',
        timestamp: 1000000200,
        prd: { title: 'V3' } as PRD,
        iterationNumber: 2,
        feedback: '优化用户体验',
      },
    ] as PRDVersion[],
  }

  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should initialize with empty state', () => {
    const { result } = renderHook(() => usePRDVersionHistory())

    expect(result.current.history).toBeNull()
    expect(result.current.isLoading).toBe(false)
    expect(result.current.error).toBeNull()
  })

  it('should load history successfully', async () => {
    const { invoke } = await import('@tauri-apps/api/core')
    vi.mocked(invoke).mockResolvedValue(mockHistory)

    const { result } = renderHook(() => usePRDVersionHistory())

    let loadedHistory: IterationHistory | undefined
    await act(async () => {
      loadedHistory = await result.current.loadHistory()
    })

    expect(loadedHistory).toEqual(mockHistory)
    expect(result.current.history).toEqual(mockHistory)
    expect(result.current.isLoading).toBe(false)
    expect(result.current.error).toBeNull()
    expect(invoke).toHaveBeenCalledWith('get_iteration_history', {
      request: {},
    })
  })

  it('should handle load history error', async () => {
    const { invoke } = await import('@tauri-apps/api/core')
    vi.mocked(invoke).mockRejectedValue(new Error('加载失败'))

    const { result } = renderHook(() => usePRDVersionHistory())

    await act(async () => {
      try {
        await result.current.loadHistory()
      } catch {
        // Expected error
      }
    })

    expect(result.current.history).toBeNull()
    expect(result.current.error).toBe('加载失败')
    expect(result.current.isLoading).toBe(false)
  })

  it('should compare two versions successfully', async () => {
    const mockDiff = {
      addedFeatures: ['功能 3'],
      removedFeatures: [] as string[],
      modifiedFieldsCount: 1,
    }

    const { invoke } = await import('@tauri-apps/api/core')
    vi.mocked(invoke).mockResolvedValue(mockDiff)

    const { result } = renderHook(() => usePRDVersionHistory())

    // 先加载历史
    vi.mocked(invoke).mockResolvedValueOnce(mockHistory)
    await act(async () => {
      await result.current.loadHistory()
    })

    let diff: typeof mockDiff | undefined
    await act(async () => {
      diff = await result.current.compareVersions('version-1', 'version-2')
    })

    expect(diff).toEqual(mockDiff)
    expect(invoke).toHaveBeenCalledWith('compare_versions', {
      request: {
        version_id_1: 'version-1',
        version_id_2: 'version-2',
      },
    })
  })

  it('should handle compare versions without loaded history', async () => {
    const { invoke } = await import('@tauri-apps/api/core')
    vi.mocked(invoke).mockRejectedValue(new Error('版本历史未加载'))

    const { result } = renderHook(() => usePRDVersionHistory())

    await act(async () => {
      try {
        await result.current.compareVersions('version-1', 'version-2')
      } catch {
        // Expected error
      }
    })

    expect(result.current.error).toBe('版本历史未加载')
  })

  it('should rollback to version successfully', async () => {
    const mockVersion: PRDVersion = {
      versionId: 'version-1',
      timestamp: 1000000000,
      prd: { title: 'V1' } as PRD,
      iterationNumber: 0,
      feedback: undefined,
    }

    const { invoke } = await import('@tauri-apps/api/core')
    vi.mocked(invoke)
      .mockResolvedValueOnce({ version: mockVersion }) // rollback response
      .mockResolvedValueOnce(mockHistory) // refresh history

    const { result } = renderHook(() => usePRDVersionHistory())

    let rolledBackVersion: PRDVersion | undefined
    await act(async () => {
      rolledBackVersion = await result.current.rollbackToVersion('version-1')
    })

    expect(rolledBackVersion).toEqual(mockVersion)
    expect(invoke).toHaveBeenCalledTimes(2)
    expect(invoke).toHaveBeenCalledWith('rollback_to_version', {
      request: {
        version_id: 'version-1',
      },
    })
    expect(invoke).toHaveBeenCalledWith('get_iteration_history', {
      request: {},
    })
  })

  it('should refresh history', async () => {
    const { invoke } = await import('@tauri-apps/api/core')
    vi.mocked(invoke).mockResolvedValue(mockHistory)

    const { result } = renderHook(() => usePRDVersionHistory())

    await act(async () => {
      await result.current.refresh()
    })

    expect(result.current.history).toEqual(mockHistory)
    expect(invoke).toHaveBeenCalledWith('get_iteration_history', {
      request: {},
    })
  })

  it('should reset state correctly', async () => {
    const { invoke } = await import('@tauri-apps/api/core')
    vi.mocked(invoke).mockResolvedValue(mockHistory)

    const { result } = renderHook(() => usePRDVersionHistory())

    // 先加载历史
    await act(async () => {
      await result.current.loadHistory()
    })

    expect(result.current.history).not.toBeNull()

    // 重置
    act(() => {
      result.current.reset()
    })

    expect(result.current.history).toBeNull()
    expect(result.current.isLoading).toBe(false)
    expect(result.current.error).toBeNull()
  })

  it('should support multiple version comparisons', async () => {
    const { invoke } = await import('@tauri-apps/api/core')
    vi.mocked(invoke)
      .mockResolvedValueOnce(mockHistory)
      .mockResolvedValueOnce({
        addedFeatures: ['功能 A'],
        removedFeatures: [],
        modifiedFieldsCount: 0,
      })
      .mockResolvedValueOnce({
        addedFeatures: ['功能 B'],
        removedFeatures: [],
        modifiedFieldsCount: 1,
      })

    const { result } = renderHook(() => usePRDVersionHistory())

    // 加载历史
    await act(async () => {
      await result.current.loadHistory()
    })

    // 对比 version-1 和 version-2
    await act(async () => {
      await result.current.compareVersions('version-1', 'version-2')
    })

    // 对比 version-2 和 version-3
    await act(async () => {
      await result.current.compareVersions('version-2', 'version-3')
    })

    expect(invoke).toHaveBeenCalledTimes(3)
  })
})
