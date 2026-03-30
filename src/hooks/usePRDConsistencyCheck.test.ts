/**
 * usePRDConsistencyCheck Hook 单元测试
 */

import { describe, it, expect, vi, beforeEach } from 'vitest'
import { renderHook, act } from '@testing-library/react'
import { usePRDConsistencyCheck } from './usePRDConsistencyCheck'
import type { PRD } from '@/types'

// Mock Tauri invoke
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}))

describe('usePRDConsistencyCheck', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should initialize with empty state', () => {
    const { result } = renderHook(() => usePRDConsistencyCheck())

    expect(result.current.report).toBeNull()
    expect(result.current.isChecking).toBe(false)
    expect(result.current.error).toBeNull()
  })

  it('should execute consistency check successfully', async () => {
    const mockReport = {
      overall_score: 85,
      dimensions: {
        user_feature_alignment: 90,
        tech_feature_alignment: 85,
        effort_reasonableness: 80,
        terminology_consistency: 90,
        logical_consistency: 80,
      },
      inconsistencies: [],
      suggestions: [],
    }

    const { invoke } = await import('@tauri-apps/api/core')
    vi.mocked(invoke).mockResolvedValue(mockReport)

    const { result } = renderHook(() => usePRDConsistencyCheck())

    const testPRD: PRD = {
      title: '测试产品',
      overview: '这是一个测试产品',
      targetUsers: ['用户 A', '用户 B'],
      coreFeatures: ['功能 1', '功能 2', '功能 3'],
      techStack: ['React', 'Rust'],
      estimatedEffort: '2 周',
    }

    await act(async () => {
      await result.current.checkConsistency(testPRD)
    })

    expect(result.current.report).toEqual(mockReport)
    expect(result.current.isChecking).toBe(false)
    expect(result.current.error).toBeNull()
    expect(invoke).toHaveBeenCalledWith('check_prd_consistency', {
      request: {
        prd_content: expect.any(String),
      },
    })
  })

  it('should handle consistency check error', async () => {
    const { invoke } = await import('@tauri-apps/api/core')
    vi.mocked(invoke).mockRejectedValue(new Error('检查失败'))

    const { result } = renderHook(() => usePRDConsistencyCheck())

    const testPRD: PRD = {
      title: '测试产品',
      overview: '这是一个测试产品',
      targetUsers: ['用户 A'],
      coreFeatures: ['功能 1'],
      techStack: ['React'],
      estimatedEffort: '2 周',
    }

    await act(async () => {
      await result.current.checkConsistency(testPRD)
    })

    expect(result.current.report).toBeNull()
    expect(result.current.isChecking).toBe(false)
    expect(result.current.error).toBe('检查失败')
  })

  it('should reset state correctly', async () => {
    const mockReport = {
      overall_score: 85,
      dimensions: {
        user_feature_alignment: 90,
        tech_feature_alignment: 85,
        effort_reasonableness: 80,
        terminology_consistency: 90,
        logical_consistency: 80,
      },
      inconsistencies: [],
      suggestions: [],
    }

    const { invoke } = await import('@tauri-apps/api/core')
    vi.mocked(invoke).mockResolvedValue(mockReport)

    const { result } = renderHook(() => usePRDConsistencyCheck())

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
      await result.current.checkConsistency(testPRD)
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
      overall_score: 85,
      dimensions: {
        user_feature_alignment: 90,
        tech_feature_alignment: 85,
        effort_reasonableness: 80,
        terminology_consistency: 90,
        logical_consistency: 80,
      },
      inconsistencies: [],
      suggestions: [],
    }

    const { invoke } = await import('@tauri-apps/api/core')
    vi.mocked(invoke).mockResolvedValue(mockReport)

    const { result } = renderHook(() => usePRDConsistencyCheck())

    const testPRD: PRD = {
      title: '测试产品',
      overview: '这是一个测试产品',
      targetUsers: ['用户 A', '用户 B'],
      coreFeatures: ['功能 1', '功能 2'],
      techStack: ['React', 'Rust'],
      estimatedEffort: '2 周',
    }

    await act(async () => {
      await result.current.checkConsistency(testPRD)
    })

    // 验证 invoke 被调用且参数包含 markdown 内容
    expect(invoke).toHaveBeenCalled()
    const callArgs = vi.mocked(invoke).mock.calls[0]
    expect(callArgs[1]).toHaveProperty('request')
    const request = (callArgs[1] as { request: { prd_content: string } }).request
    expect(request.prd_content).toContain('# 测试产品')
    expect(request.prd_content).toContain('## 产品概述')
    expect(request.prd_content).toContain('## 目标用户')
  })

  it('should detect inconsistencies in inconsistent PRD', async () => {
    const mockReport = {
      overall_score: 45,
      dimensions: {
        user_feature_alignment: 40,
        tech_feature_alignment: 50,
        effort_reasonableness: 30,
        terminology_consistency: 60,
        logical_consistency: 45,
      },
      inconsistencies: [
        {
          inconsistency_type: {
            type: 'user_not_served',
            user: '设计师',
          },
          severity: 'major',
          description: "目标用户 '设计师' 没有明确对应的功能支持",
          suggestion: "添加针对 '设计师' 用户的专属功能",
        },
        {
          inconsistency_type: {
            type: 'tech_stack_mismatch',
            feature: '实时消息推送',
            required_techs: ['WebSocket'],
            missing_techs: ['WebSocket'],
          },
          severity: 'major',
          description: "功能 '实时消息推送' 需要技术栈 WebSocket，但未在技术栈列表中找到",
          suggestion: '在技术栈中添加：WebSocket',
        },
      ],
      suggestions: [
        "为 '设计师' 用户添加明确的功能支持",
        "为功能 '实时消息推送' 添加必要的技术支持",
      ],
    }

    const { invoke } = await import('@tauri-apps/api/core')
    vi.mocked(invoke).mockResolvedValue(mockReport)

    const { result } = renderHook(() => usePRDConsistencyCheck())

    const testPRD: PRD = {
      title: '复杂系统',
      overview: '一个简单的系统',
      targetUsers: ['普通用户', '管理员', '访客', '设计师'],
      coreFeatures: ['用户管理', '权限控制', '数据可视化', '实时消息推送', '报表生成'],
      techStack: ['HTML', 'CSS'],
      estimatedEffort: '1 天',
    }

    await act(async () => {
      await result.current.checkConsistency(testPRD)
    })

    expect(result.current.report).toEqual(mockReport)
    expect(result.current.report?.overall_score).toBeLessThan(60)
    expect(result.current.report?.inconsistencies.length).toBeGreaterThan(0)
  })
})
