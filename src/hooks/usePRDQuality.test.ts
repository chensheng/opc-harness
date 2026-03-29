import { describe, it, expect, beforeEach, vi } from 'vitest'
import { renderHook, act } from '@testing-library/react'
import { usePRDQuality } from './usePRDQuality'
import type { PRD } from '@/types'

/**
 * 创建完整的 PRD 测试数据
 */
function createTestPRD(overrides: Partial<PRD> = {}): PRD {
  return {
    title: '测试产品',
    overview: '这是一个完整的产品概述，描述了产品的核心价值和功能定位。',
    targetUsers: ['产品经理', '开发者'],
    coreFeatures: ['功能 1', '功能 2', '功能 3'],
    techStack: ['React', 'TypeScript', 'Node.js'],
    estimatedEffort: '2 周',
    ...overrides,
  }
}

describe('usePRDQuality', () => {
  beforeEach(() => {
    // 清理 mock
    vi.clearAllMocks()
  })

  it('should initialize with default state', () => {
    const { result } = renderHook(() => usePRDQuality())

    expect(result.current.isChecking).toBe(false)
    expect(result.current.progress).toBe(0)
    expect(result.current.error).toBe(null)
    expect(result.current.qualityReport).toBe(null)
    expect(result.current.overallScore).toBe(null)
  })

  it('should check quality and update state', async () => {
    const testPRD = createTestPRD()

    const { result } = renderHook(() => usePRDQuality())

    // 开始检查
    await act(async () => {
      await result.current.checkQuality(testPRD)
    })

    // 验证状态更新
    expect(result.current.isChecking).toBe(false)
    expect(result.current.progress).toBe(100)
    expect(result.current.error).toBe(null)
    expect(result.current.qualityReport).not.toBeNull()
    expect(result.current.overallScore).not.toBeNull()
    expect(result.current.overallScore!).toBeGreaterThanOrEqual(0)
    expect(result.current.overallScore!).toBeLessThanOrEqual(100)
  })

  it('should show progress during check', async () => {
    const testPRD = createTestPRD({ overview: '简短概述' })

    const { result } = renderHook(() => usePRDQuality())

    // 开始检查
    await act(async () => {
      await result.current.checkQuality(testPRD)
    })

    // 检查完成后应该不是 checking 状态
    expect(result.current.isChecking).toBe(false)
    expect(result.current.progress).toBe(100)
  })

  it('should handle errors gracefully', async () => {
    const testPRD = createTestPRD()

    const { result } = renderHook(() => usePRDQuality())

    // 即使检查出错也不应该崩溃
    await act(async () => {
      await result.current.checkQuality(testPRD)
    })

    // 应该恢复到稳定状态
    expect(result.current.isChecking).toBe(false)
  })

  it('should reset state correctly', async () => {
    const testPRD = createTestPRD()

    const { result } = renderHook(() => usePRDQuality())

    // 先执行一次检查
    await act(async () => {
      await result.current.checkQuality(testPRD)
    })

    // 验证有检查结果
    expect(result.current.qualityReport).not.toBeNull()
    expect(result.current.overallScore).not.toBeNull()

    // 重置
    await act(async () => {
      result.current.reset()
    })

    // 验证状态已重置
    expect(result.current.isChecking).toBe(false)
    expect(result.current.progress).toBe(0)
    expect(result.current.error).toBe(null)
    expect(result.current.qualityReport).toBe(null)
    expect(result.current.overallScore).toBe(null)
  })

  it('should allow multiple checks', async () => {
    const prd1 = createTestPRD({ overview: '第一个版本' })
    const prd2 = createTestPRD({
      overview: '第二个更完整的版本，描述更加详细和充分。',
      targetUsers: ['用户 A', '用户 B'],
    })

    const { result } = renderHook(() => usePRDQuality())

    // 第一次检查
    await act(async () => {
      await result.current.checkQuality(prd1)
    })
    const firstScore = result.current.overallScore

    // 第二次检查
    await act(async () => {
      await result.current.checkQuality(prd2)
    })
    const secondScore = result.current.overallScore

    // 两次检查都应该有结果
    expect(firstScore).not.toBeNull()
    expect(secondScore).not.toBeNull()
    expect(result.current.qualityReport).not.toBeNull()
  })

  it('should provide detailed quality report', async () => {
    const completePRD = createTestPRD({
      overview: '这是一个非常详细的产品概述，充分描述了产品的各个方面。',
      targetUsers: ['用户 A', '用户 B'],
      coreFeatures: ['功能 1', '功能 2', '功能 3', '功能 4'],
    })

    const { result } = renderHook(() => usePRDQuality())

    await act(async () => {
      await result.current.checkQuality(completePRD)
    })

    const report = result.current.qualityReport
    expect(report).not.toBeNull()
    // 评分应该是正数
    expect(report!.overallScore).toBeGreaterThanOrEqual(0)
    expect(report!.completeness).toBeDefined()
    expect(report!.issues).toBeDefined()
    expect(report!.suggestions).toBeDefined()
  })
})
