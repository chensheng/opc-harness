/**
 * usePRDFeasibilityCheck Hook 单元测试
 */

import { describe, it, expect, vi, beforeEach } from 'vitest'
import { renderHook, act } from '@testing-library/react'
import { usePRDFeasibilityCheck } from './usePRDFeasibilityCheck'
import type { PRD } from '@/types'

// Mock Tauri invoke
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}))

describe('usePRDFeasibilityCheck', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should initialize with empty state', () => {
    const { result } = renderHook(() => usePRDFeasibilityCheck())

    expect(result.current.report).toBeNull()
    expect(result.current.isAssessing).toBe(false)
    expect(result.current.error).toBeNull()
  })

  it('should execute feasibility assessment successfully', async () => {
    const mockReport = {
      overall_score: 75,
      feasibility_level: 'high' as const,
      technical: {
        complexity: 0.3,
        team_skill_match: 0.9,
        feasibility_score: 85,
        technical_challenges: ['前端状态管理'],
      },
      resource: {
        required_people_months: 2.5,
        available_team_size: 3,
        resource_adequacy: 0.9,
        critical_skills: ['Frontend Development'],
      },
      timeline: {
        estimated_weeks: 4,
        reasonable_min_weeks: 3.2,
        reasonable_max_weeks: 6,
        reasonableness_score: 90,
      },
      risks: [],
      recommendations: ['项目可行性较高，建议按计划推进'],
    }

    const { invoke } = await import('@tauri-apps/api/core')
    vi.mocked(invoke).mockResolvedValue(mockReport)

    const { result } = renderHook(() => usePRDFeasibilityCheck())

    const testPRD: PRD = {
      title: '测试产品',
      overview: '这是一个测试产品',
      targetUsers: ['用户 A', '用户 B'],
      coreFeatures: ['功能 1', '功能 2', '功能 3'],
      techStack: ['React', 'Rust'],
      estimatedEffort: '2 周',
    }

    await act(async () => {
      await result.current.assessFeasibility(testPRD)
    })

    expect(result.current.report).toEqual(mockReport)
    expect(result.current.isAssessing).toBe(false)
    expect(result.current.error).toBeNull()
    expect(invoke).toHaveBeenCalledWith('assess_prd_feasibility', {
      request: {
        prd_content: expect.any(String),
      },
    })
  })

  it('should handle feasibility assessment error', async () => {
    const { invoke } = await import('@tauri-apps/api/core')
    vi.mocked(invoke).mockRejectedValue(new Error('评估失败'))

    const { result } = renderHook(() => usePRDFeasibilityCheck())

    const testPRD: PRD = {
      title: '测试产品',
      overview: '这是一个测试产品',
      targetUsers: ['用户 A'],
      coreFeatures: ['功能 1'],
      techStack: ['React'],
      estimatedEffort: '2 周',
    }

    await act(async () => {
      await result.current.assessFeasibility(testPRD)
    })

    expect(result.current.report).toBeNull()
    expect(result.current.isAssessing).toBe(false)
    expect(result.current.error).toBe('评估失败')
  })

  it('should reset state correctly', async () => {
    const mockReport = {
      overall_score: 75,
      feasibility_level: 'high' as const,
      technical: {
        complexity: 0.3,
        team_skill_match: 0.9,
        feasibility_score: 85,
        technical_challenges: [],
      },
      resource: {
        required_people_months: 2.5,
        available_team_size: 3,
        resource_adequacy: 0.9,
        critical_skills: [],
      },
      timeline: {
        estimated_weeks: 4,
        reasonable_min_weeks: 3.2,
        reasonable_max_weeks: 6,
        reasonableness_score: 90,
      },
      risks: [],
      recommendations: [],
    }

    const { invoke } = await import('@tauri-apps/api/core')
    vi.mocked(invoke).mockResolvedValue(mockReport)

    const { result } = renderHook(() => usePRDFeasibilityCheck())

    const testPRD: PRD = {
      title: '测试产品',
      overview: '这是一个测试产品',
      targetUsers: ['用户 A'],
      coreFeatures: ['功能 1'],
      techStack: ['React'],
      estimatedEffort: '2 周',
    }

    // 先执行评估
    await act(async () => {
      await result.current.assessFeasibility(testPRD)
    })

    expect(result.current.report).not.toBeNull()

    // 重置
    act(() => {
      result.current.reset()
    })

    expect(result.current.report).toBeNull()
    expect(result.current.isAssessing).toBe(false)
    expect(result.current.error).toBeNull()
  })

  it('should convert PRD to markdown correctly', async () => {
    const mockReport = {
      overall_score: 75,
      feasibility_level: 'medium' as const,
      technical: {
        complexity: 0.5,
        team_skill_match: 0.7,
        feasibility_score: 70,
        technical_challenges: [],
      },
      resource: {
        required_people_months: 3,
        available_team_size: 3,
        resource_adequacy: 0.8,
        critical_skills: [],
      },
      timeline: {
        estimated_weeks: 4,
        reasonable_min_weeks: 3,
        reasonable_max_weeks: 6,
        reasonableness_score: 80,
      },
      risks: [],
      recommendations: [],
    }

    const { invoke } = await import('@tauri-apps/api/core')
    vi.mocked(invoke).mockResolvedValue(mockReport)

    const { result } = renderHook(() => usePRDFeasibilityCheck())

    const testPRD: PRD = {
      title: '测试产品',
      overview: '这是一个测试产品',
      targetUsers: ['用户 A', '用户 B'],
      coreFeatures: ['功能 1', '功能 2'],
      techStack: ['React', 'Rust'],
      estimatedEffort: '2 周',
    }

    await act(async () => {
      await result.current.assessFeasibility(testPRD)
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

  it('should identify risks in low feasibility PRD', async () => {
    const mockReport = {
      overall_score: 35,
      feasibility_level: 'low' as const,
      technical: {
        complexity: 0.9,
        team_skill_match: 0.3,
        feasibility_score: 30,
        technical_challenges: ['高复杂度技术栈', '实时通信架构'],
      },
      resource: {
        required_people_months: 10,
        available_team_size: 3,
        resource_adequacy: 0.3,
        critical_skills: ['Backend Development', 'Database Design'],
      },
      timeline: {
        estimated_weeks: 2,
        reasonable_min_weeks: 8,
        reasonable_max_weeks: 15,
        reasonableness_score: 25,
      },
      risks: [
        {
          risk_type: {
            type: 'technical_capability_gap',
            required_techs: ['高级技术'],
            team_skill_level: 0.3,
          },
          level: 'critical',
          description: '团队技能匹配度较低 (30%)，可能存在技术能力缺口',
          impact: '可能导致开发进度延迟、代码质量下降、技术债务累积',
          mitigation: '建议进行技术培训、招聘专业人才或寻求外部技术支持',
        },
        {
          risk_type: { type: 'resource_shortage', required_people: 10, available_team_size: 3 },
          level: 'high',
          description: '所需人力 (10.0 人月) 超过可用团队规模 (3 人)',
          impact: '可能导致项目延期、团队成员过度工作、产品质量下降',
          mitigation: '建议增加团队规模、调整项目范围或延长交付时间',
        },
      ],
      recommendations: ['建议定期回顾风险评估，及时调整项目策略'],
    }

    const { invoke } = await import('@tauri-apps/api/core')
    vi.mocked(invoke).mockResolvedValue(mockReport)

    const { result } = renderHook(() => usePRDFeasibilityCheck())

    const testPRD: PRD = {
      title: '复杂系统',
      overview: '一个企业级复杂系统',
      targetUsers: ['所有员工'],
      coreFeatures: Array(10).fill('复杂功能'),
      techStack: ['Kubernetes', 'Microservices', 'Machine Learning', 'Blockchain'],
      estimatedEffort: '1 周',
    }

    await act(async () => {
      await result.current.assessFeasibility(testPRD)
    })

    expect(result.current.report).toEqual(mockReport)
    expect(result.current.report?.overall_score).toBeLessThan(50)
    expect(result.current.report?.feasibility_level).toBe('low')
    expect(result.current.report?.risks.length).toBeGreaterThan(0)
  })
})
