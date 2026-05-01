import { useState, useCallback } from 'react'
import { invoke } from '@tauri-apps/api/core'

/**
 * 完全去中心化 Agent Worker Hook
 * 
 * 提供对完全去中心化智能体系统的控制能力:
 * - 启动/停止独立的 Agent Worker 实例（每个 Worker 拥有完整的 Agent Loop）
 * - 查询所有运行中的 Workers
 * - 监控 Worker 状态和当前处理的 Story
 */
export function useAgentWorkers() {
  const [workers, setWorkers] = useState<Array<{
    worker_id: string
    is_running: boolean
    current_story_id?: string
  }>>([])
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  /** 启动新的完全去中心化 Agent Worker */
  const startWorker = useCallback(async (options?: {
    worker_id?: string
    project_id: string
    check_interval?: number
  }) => {
    if (!options?.project_id) {
      throw new Error('project_id is required')
    }

    setIsLoading(true)
    setError(null)

    try {
      const workerId = await invoke<string>('start_agent_worker', {
        workerId: options?.worker_id,
        projectId: options.project_id,
        checkInterval: options?.check_interval || 30,
      })
      
      console.log('[useAgentWorkers] Started worker:', workerId)
      
      // 刷新 Worker 列表
      await refreshWorkers()
      
      return workerId
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : String(err)
      setError(errorMsg)
      console.error('[useAgentWorkers] Failed to start worker:', errorMsg)
      throw err
    } finally {
      setIsLoading(false)
    }
  }, [])

  /** 停止指定的 Agent Worker */
  const stopWorker = useCallback(async (workerId: string) => {
    setIsLoading(true)
    setError(null)

    try {
      await invoke('stop_agent_worker', { workerId })
      
      console.log('[useAgentWorkers] Stopped worker:', workerId)
      
      // 刷新 Worker 列表
      await refreshWorkers()
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : String(err)
      setError(errorMsg)
      console.error('[useAgentWorkers] Failed to stop worker:', errorMsg)
      throw err
    } finally {
      setIsLoading(false)
    }
  }, [])

  /** 刷新 Worker 列表 */
  const refreshWorkers = useCallback(async () => {
    try {
      const workersList = await invoke<Array<{
        worker_id: string
        is_running: boolean
        current_story_id?: string
      }>>('list_agent_workers')
      setWorkers(workersList)
      return workersList
    } catch (err) {
      console.error('[useAgentWorkers] Failed to list workers:', err)
      return []
    }
  }, [])

  /** 获取运行中的 Worker 数量 */
  const getRunningCount = useCallback(() => {
    return workers.filter(worker => worker.is_running).length
  }, [workers])

  /** 获取正在处理 Story 的 Worker 数量 */
  const getBusyCount = useCallback(() => {
    return workers.filter(worker => worker.current_story_id).length
  }, [workers])

  return {
    workers,
    isLoading,
    error,
    startWorker,
    stopWorker,
    refreshWorkers,
    getRunningCount,
    getBusyCount,
  }
}
