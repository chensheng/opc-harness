/**
 * usePRDQualityCheck Hook 单元测试
 */

import { describe, it, expect, vi, beforeEach } from 'vitest'
import { renderHook, act } from '@testing-library/react'
import { usePRDQualityCheck } from './usePRDQualityCheck'
import type { PRD } from '@/types'

// Mock Tauri invoke
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}))

describe('usePRDQualityCheck', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should initialize with empty state', () => {
    const { result } = renderHook(() => usePRDQualityCheck())

    expect(result.current.report).toBeNull()
    expect(result.current.isChecking).toBe(false)
    expect(result.current.error).toBeNull()
  })

  it('should execute quality check successfully', async () => {
    const mockReport = {
      overallScore: 85,
      completeness: {
        completeness_score: 90,
        missing_sections: [],
        sections: {},
      },
      issues: [],
      suggestions: [],
    }

    const { invoke } = await import('@tauri-apps/api/core')
    vi.mocked(invoke).mockResolvedValue(mockReport)

    const { result } = renderHook(() => usePRDQualityCheck())

    const testPRD: PRD = {
      title: '测试产品',
      overview: '这是一个测试产品',
      targetUsers: ['用户 A', '用户 B'],
      coreFeatures: ['功能 1', '功能 2', '功能 3'],
      techStack: ['React', 'Rust'],
      estimatedEffort: '2 周',
    }

    await act(async () => {
      await result.current.checkQuality(testPRD)
    })

    expect(result.current.report).toEqual(mockReport)
    expect(result.current.isChecking).toBe(false)
    expect(result.current.error).toBeNull()
    expect(invoke).toHaveBeenCalledWith('check_prd_quality', {
      prdContent: expect.any(String),
    })
  })

  it('should handle quality check error', async () => {
    const { invoke } = await import('@tauri-apps/api/core')
    vi.mocked(invoke).mockRejectedValue(new Error('检查失败'))

    const { result } = renderHook(() => usePRDQualityCheck())

    const testPRD: PRD = {
      title: '测试产品',
      overview: '这是一个测试产品',
      targetUsers: ['用户 A'],
      coreFeatures: ['功能 1'],
      techStack: ['React'],
      estimatedEffort: '2 周',
    }

    await act(async () => {
      await result.current.checkQuality(testPRD)
    })

    expect(result.current.report).toBeNull()
    expect(result.current.isChecking).toBe(false)
    expect(result.current.error).toBe('检查失败')
  })

  it('should reset state correctly', async () => {
    const mockReport = {
      overallScore: 85,
      completeness: {
        completeness_score: 90,
        missing_sections: [],
        sections: {},
      },
      issues: [],
      suggestions: [],
    }

    const { invoke } = await import('@tauri-apps/api/core')
    vi.mocked(invoke).mockResolvedValue(mockReport)

    const { result } = renderHook(() => usePRDQualityCheck())

    const testPRD: PRD = {
      title: '测试产品',
      overview: '这是一个测试产品',
      targetUsers: ['用户 A'],
      coreFeatures: ['功能 1'],
      techStack: ['React'],
      estimatedEffort: '2 周',
    }

    // 先执行检查
    await act(async () => {
      await result.current.checkQuality(testPRD)
    })

    expect(result.current.report).not.toBeNull()

    // 重置
    act(() => {
      result.current.reset()
    })

    expect(result.current.report).toBeNull()
    expect(result.current.isChecking).toBe(false)
    expect(result.current.error).toBeNull()
  })

  it('should convert PRD to markdown correctly', async () => {
    const mockReport = {
      overallScore: 85,
      completeness: {
        completeness_score: 90,
        missing_sections: [],
        sections: {},
      },
      issues: [],
      suggestions: [],
    }

    const { invoke } = await import('@tauri-apps/api/core')
    vi.mocked(invoke).mockResolvedValue(mockReport)

    const { result } = renderHook(() => usePRDQualityCheck())

    const testPRD: PRD = {
      title: '测试产品',
      overview: '这是一个测试产品',
      targetUsers: ['用户 A', '用户 B'],
      coreFeatures: ['功能 1', '功能 2'],
      techStack: ['React', 'Rust'],
      estimatedEffort: '2 周',
    }

    await act(async () => {
      await result.current.checkQuality(testPRD)
    })

    // 验证 invoke 被调用且第一个参数包含 markdown 内容
    expect(invoke).toHaveBeenCalled()
    const callArgs = vi.mocked(invoke).mock.calls[0]
    expect(callArgs[1]).toHaveProperty('prdContent')
    expect((callArgs[1] as { prdContent: string }).prdContent).toContain('# 测试产品')
    expect((callArgs[1] as { prdContent: string }).prdContent).toContain('## 产品概述')
    expect((callArgs[1] as { prdContent: string }).prdContent).toContain('## 目标用户')
  })

  it('should handle empty PRD fields', async () => {
    const mockReport = {
      overallScore: 30,
      completeness: {
        completeness_score: 40,
        missing_sections: ['目标用户', '核心功能'],
        sections: {},
      },
      issues: [
        {
          severity: 'critical',
          section: '目标用户',
          description: '缺少目标用户',
          suggestion: '定义至少 2 个目标用户群体',
        },
      ],
      suggestions: ['添加缺失的章节：目标用户'],
    }

    const { invoke } = await import('@tauri-apps/api/core')
    vi.mocked(invoke).mockResolvedValue(mockReport)

    const { result } = renderHook(() => usePRDQualityCheck())

    const testPRD: PRD = {
      title: '测试',
      overview: '简短概述',
      targetUsers: [],
      coreFeatures: [],
      techStack: ['React'],
      estimatedEffort: '',
    }

    await act(async () => {
      await result.current.checkQuality(testPRD)
    })

    expect(result.current.report).toEqual(mockReport)
  })
})
