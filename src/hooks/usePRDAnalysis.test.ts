import { describe, it, expect, beforeEach, vi } from 'vitest'
import { renderHook, waitFor } from '@testing-library/react'
import { usePRDAnalysis } from './usePRDAnalysis'

// Mock Tauri invoke
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}))

const { invoke } = await import('@tauri-apps/api/core')

describe('usePRDAnalysis', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should initialize with null analysis', () => {
    const { result } = renderHook(() => usePRDAnalysis())

    expect(result.current.analysis).toBeNull()
    expect(result.current.loading).toBe(false)
    expect(result.current.error).toBeNull()
  })

  it('should analyze PRD content successfully', async () => {
    const mockAnalysis = {
      features: [
        {
          id: 'F001',
          name: '用户管理',
          description: '用户相关功能',
          featureType: 'core',
          complexity: 3,
          estimatedHours: 6,
          priority: 8,
          dependencies: [],
        },
      ],
      dependencies: [],
      risks: [],
      estimates: {
        totalFeatures: 1,
        coreFeatures: 1,
        auxiliaryFeatures: 0,
        enhancedFeatures: 0,
        averageComplexity: 3,
        totalEstimatedHours: 6,
        highRisksCount: 0,
      },
    }

    ;(invoke as unknown as jest.Mock).mockResolvedValue({
      success: true,
      analysis: mockAnalysis,
    })

    const { result } = renderHook(() => usePRDAnalysis())

    await waitFor(() => {
      result.current.analyze('# Test PRD', 'test-key')
    })

    await waitFor(() => {
      expect(result.current.loading).toBe(false)
    })

    expect(result.current.analysis).toEqual(mockAnalysis)
    expect(result.current.error).toBeNull()
  })

  it('should handle analysis error', async () => {
    ;(invoke as unknown as jest.Mock).mockResolvedValue({
      success: false,
      analysis: {
        features: [],
        dependencies: [],
        risks: [],
        estimates: {
          totalFeatures: 0,
          coreFeatures: 0,
          auxiliaryFeatures: 0,
          enhancedFeatures: 0,
          averageComplexity: 0,
          totalEstimatedHours: 0,
          highRisksCount: 0,
        },
      },
      errorMessage: '分析失败',
    })

    const { result } = renderHook(() => usePRDAnalysis())

    await waitFor(() => {
      result.current.analyze('# Test PRD')
    })

    await waitFor(() => {
      expect(result.current.loading).toBe(false)
    })

    expect(result.current.analysis).toBeDefined()
    expect(result.current.error).toBe('分析失败')
  })

  it('should handle invoke exception', async () => {
    ;(invoke as unknown as jest.Mock).mockRejectedValue(new Error('Network error'))

    const { result } = renderHook(() => usePRDAnalysis())

    await waitFor(() => {
      result.current.analyze('# Test PRD')
    })

    await waitFor(() => {
      expect(result.current.loading).toBe(false)
    })

    expect(result.current.analysis).toBeNull()
    expect(result.current.error).toBe('Network error')
  })

  it('should reset state', () => {
    const { result } = renderHook(() => usePRDAnalysis())

    // Set some state
    result.current.reset()

    expect(result.current.analysis).toBeNull()
    expect(result.current.loading).toBe(false)
    expect(result.current.error).toBeNull()
  })
})
