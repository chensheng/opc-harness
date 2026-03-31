import { useState, useCallback } from 'react'
import { invoke } from '@tauri-apps/api/core'
import type {
  PrdAnalysis,
  TaskDependencyGraph,
  DecomposeTasksRequest,
  DecomposeTasksResponse,
} from '../types'

interface UseTaskDecompositionReturn {
  /** 任务依赖图 */
  taskGraph: TaskDependencyGraph | null
  /** 是否正在分解 */
  loading: boolean
  /** 错误信息 */
  error: string | null
  /** 执行任务分解 */
  decompose: (analysis: PrdAnalysis) => Promise<void>
  /** 重置状态 */
  reset: () => void
}

/**
 * 任务分解 Hook
 */
export function useTaskDecomposition(): UseTaskDecompositionReturn {
  const [taskGraph, setTaskGraph] = useState<TaskDependencyGraph | null>(null)
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  /**
   * 执行任务分解
   */
  const decompose = useCallback(async (analysis: PrdAnalysis) => {
    setLoading(true)
    setError(null)

    try {
      const request: DecomposeTasksRequest = {
        analysis,
      }

      const response = await invoke<DecomposeTasksResponse>('decompose_tasks', request)

      if (response.success) {
        setTaskGraph(response.taskGraph)
      } else {
        setError(response.errorMessage || '分解失败')
      }
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : '未知错误'
      setError(errorMessage)
      console.error('Task decomposition failed:', err)
    } finally {
      setLoading(false)
    }
  }, [])

  /**
   * 重置状态
   */
  const reset = useCallback(() => {
    setTaskGraph(null)
    setLoading(false)
    setError(null)
  }, [])

  return {
    taskGraph,
    loading,
    error,
    decompose,
    reset,
  }
}
