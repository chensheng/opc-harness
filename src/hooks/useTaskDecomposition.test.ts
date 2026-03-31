import { describe, it, expect, beforeEach, vi } from 'vitest'
import { renderHook, waitFor } from '@testing-library/react'
import { useTaskDecomposition } from './useTaskDecomposition'
import type { PrdAnalysis } from '../types'
import { FeatureType } from '../types'

// Mock Tauri invoke
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}))

const { invoke } = await import('@tauri-apps/api/core')

describe('useTaskDecomposition', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should initialize with null taskGraph', () => {
    const { result } = renderHook(() => useTaskDecomposition())

    expect(result.current.taskGraph).toBeNull()
    expect(result.current.loading).toBe(false)
    expect(result.current.error).toBeNull()
  })

  it('should decompose PRD analysis successfully', async () => {
    const mockAnalysis: PrdAnalysis = {
      features: [
        {
          id: 'F001',
          name: '用户管理',
          description: '用户 CRUD 操作',
          featureType: FeatureType.CORE,
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

    const mockTaskGraph = {
      tasks: [
        {
          id: 'T1-DB',
          title: '数据库设计',
          description: '设计数据库表',
          taskType: 'database',
          estimatedHours: 4.5,
          dependencies: [],
          priority: 9,
          featureId: 'F001',
          complexity: 3,
          skills: ['SQL', 'Database Design'],
        },
        {
          id: 'T1-BE-API',
          title: 'API 开发',
          description: '实现 API',
          taskType: 'backend',
          estimatedHours: 6,
          dependencies: ['T1-DB'],
          priority: 8,
          featureId: 'F001',
          complexity: 3,
          skills: ['Rust', 'API Design'],
        },
      ],
      edges: [
        {
          fromTask: 'T1-DB',
          toTask: 'T1-BE-API',
          dependencyType: 'technical',
          strength: 'strong',
        },
      ],
      criticalPath: ['T1-DB', 'T1-BE-API'],
      totalEstimatedHours: 10.5,
      statistics: {
        totalTasks: 2,
        frontendTasks: 0,
        backendTasks: 1,
        databaseTasks: 1,
        testingTasks: 0,
        averageHours: 5.25,
        averageComplexity: 3,
      },
    }

    ;(invoke as unknown as jest.Mock).mockResolvedValue({
      success: true,
      taskGraph: mockTaskGraph,
    })

    const { result } = renderHook(() => useTaskDecomposition())

    await waitFor(() => {
      result.current.decompose(mockAnalysis)
    })

    await waitFor(() => {
      expect(result.current.loading).toBe(false)
    })

    expect(result.current.taskGraph).toEqual(mockTaskGraph)
    expect(result.current.error).toBeNull()
  })

  it('should handle decomposition error', async () => {
    const mockAnalysis: PrdAnalysis = {
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
    }

    ;(invoke as unknown as jest.Mock).mockResolvedValue({
      success: false,
      taskGraph: {
        tasks: [],
        edges: [],
        criticalPath: [],
        totalEstimatedHours: 0,
        statistics: {
          totalTasks: 0,
          frontendTasks: 0,
          backendTasks: 0,
          databaseTasks: 0,
          testingTasks: 0,
          averageHours: 0,
          averageComplexity: 0,
        },
      },
      errorMessage: '分解失败',
    })

    const { result } = renderHook(() => useTaskDecomposition())

    await waitFor(() => {
      result.current.decompose(mockAnalysis)
    })

    await waitFor(() => {
      expect(result.current.loading).toBe(false)
    })

    expect(result.current.taskGraph).toBeDefined()
    expect(result.current.error).toBe('分解失败')
  })

  it('should handle invoke exception', async () => {
    const mockAnalysis: PrdAnalysis = {
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
    }

    ;(invoke as unknown as jest.Mock).mockRejectedValue(new Error('Network error'))

    const { result } = renderHook(() => useTaskDecomposition())

    await waitFor(() => {
      result.current.decompose(mockAnalysis)
    })

    await waitFor(() => {
      expect(result.current.loading).toBe(false)
    })

    expect(result.current.taskGraph).toBeNull()
    expect(result.current.error).toBe('Network error')
  })

  it('should reset state', () => {
    const { result } = renderHook(() => useTaskDecomposition())

    result.current.reset()

    expect(result.current.taskGraph).toBeNull()
    expect(result.current.loading).toBe(false)
    expect(result.current.error).toBeNull()
  })
})
