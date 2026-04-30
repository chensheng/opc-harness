import { useState, useCallback } from 'react'
import { invoke } from '@tauri-apps/api/core'

/**
 * Agent Loop 自动化执行引擎 Hook
 * 
 * 提供对 Vibe Coding Agent Loop 的控制能力:
 * - 启动/停止持续运行的 Agent Loop
 * - 手动触发单次执行
 * - 查询运行状态
 */
export function useAgentLoop() {
  const [isRunning, setIsRunning] = useState(false)
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  /** 启动 Agent Loop 持续运行 */
  const startAgentLoop = useCallback(async (projectId: string, intervalSecs?: number) => {
    setIsLoading(true)
    setError(null)

    try {
      await invoke('start_agent_loop', {
        projectId,
        intervalSecs: intervalSecs || 60, // 默认 60 秒
      })
      
      setIsRunning(true)
      console.log('[useAgentLoop] Agent Loop started for project:', projectId)
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : String(err)
      setError(errorMsg)
      console.error('[useAgentLoop] Failed to start Agent Loop:', errorMsg)
      throw err
    } finally {
      setIsLoading(false)
    }
  }, [])

  /** 执行一次 Agent Loop(用于测试或手动触发) */
  const executeOnce = useCallback(async (projectId: string) => {
    setIsLoading(true)
    setError(null)

    try {
      const startedCount = await invoke<number>('execute_agent_loop_once', {
        projectId,
      })
      
      console.log(`[useAgentLoop] Executed once for project ${projectId}, started ${startedCount} agents`)
      return startedCount
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : String(err)
      setError(errorMsg)
      console.error('[useAgentLoop] Failed to execute Agent Loop:', errorMsg)
      throw err
    } finally {
      setIsLoading(false)
    }
  }, [])

  /** 停止 Agent Loop */
  const stopAgentLoop = useCallback(async () => {
    setIsLoading(true)
    setError(null)

    try {
      await invoke('stop_agent_loop')
      
      setIsRunning(false)
      console.log('[useAgentLoop] Agent Loop stopped')
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : String(err)
      setError(errorMsg)
      console.error('[useAgentLoop] Failed to stop Agent Loop:', errorMsg)
      throw err
    } finally {
      setIsLoading(false)
    }
  }, [])

  /** 检查 Agent Loop 是否正在运行 */
  const checkStatus = useCallback(async () => {
    try {
      const running = await invoke<boolean>('is_agent_loop_running')
      setIsRunning(running)
      return running
    } catch (err) {
      console.error('[useAgentLoop] Failed to check Agent Loop status:', err)
      return false
    }
  }, [])

  return {
    isRunning,
    isLoading,
    error,
    startAgentLoop,
    executeOnce,
    stopAgentLoop,
    checkStatus,
  }
}

/**
 * Worktree 管理器 Hook
 * 
 * 提供对 Git Worktree 的管理能力:
 * - 创建/删除 Worktree
 * - 列出所有 Worktrees
 * - 清理孤立 Worktrees
 * - 监控磁盘使用量
 */
export interface WorktreeInfo {
  id: string
  path: string
  branch: string
  story_id?: string
  created_at: number
  is_orphaned: boolean
}

export function useWorktreeManager() {
  const [worktrees, setWorktrees] = useState<WorktreeInfo[]>([])
  const [diskUsage, setDiskUsage] = useState<number>(0)
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  /** 创建 Worktree */
  const createWorktree = useCallback(async (agentId: string, storyId: string, branchName: string) => {
    setIsLoading(true)
    setError(null)

    try {
      const path = await invoke<string>('create_worktree', {
        agentId,
        storyId,
        branchName,
      })
      
      console.log('[useWorktreeManager] Created worktree at:', path)
      
      // 刷新列表
      await listWorktrees()
      
      return path
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : String(err)
      setError(errorMsg)
      console.error('[useWorktreeManager] Failed to create worktree:', errorMsg)
      throw err
    } finally {
      setIsLoading(false)
    }
  }, [])

  /** 删除 Worktree */
  const removeWorktree = useCallback(async (agentId: string) => {
    setIsLoading(true)
    setError(null)

    try {
      await invoke('remove_worktree', {
        agentId,
      })
      
      console.log('[useWorktreeManager] Removed worktree for agent:', agentId)
      
      // 刷新列表
      await listWorktrees()
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : String(err)
      setError(errorMsg)
      console.error('[useWorktreeManager] Failed to remove worktree:', errorMsg)
      throw err
    } finally {
      setIsLoading(false)
    }
  }, [])

  /** 列出所有 Worktrees */
  const listWorktrees = useCallback(async () => {
    setIsLoading(true)
    setError(null)

    try {
      const wtList = await invoke<WorktreeInfo[]>('list_worktrees')
      setWorktrees(wtList)
      console.log('[useWorktreeManager] Listed', wtList.length, 'worktrees')
      
      // 同时获取磁盘使用量
      await getDiskUsage()
      
      return wtList
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : String(err)
      // Agent Loop 未初始化时不显示错误,这是正常情况
      if (!errorMsg.includes('Agent Loop not initialized')) {
        setError(errorMsg)
        console.error('[useWorktreeManager] Failed to list worktrees:', errorMsg)
      } else {
        console.debug('[useWorktreeManager] Agent Loop not initialized, skipping worktree list')
      }
      throw err
    } finally {
      setIsLoading(false)
    }
  }, [])

  /** 清理孤立的 Worktrees */
  const cleanupOrphaned = useCallback(async () => {
    setIsLoading(true)
    setError(null)

    try {
      const count = await invoke<number>('cleanup_orphaned_worktrees')
      console.log('[useWorktreeManager] Cleaned up', count, 'orphaned worktrees')
      
      // 刷新列表
      await listWorktrees()
      
      return count
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : String(err)
      // Agent Loop 未初始化时不显示错误
      if (!errorMsg.includes('Agent Loop not initialized')) {
        setError(errorMsg)
        console.error('[useWorktreeManager] Failed to cleanup orphaned worktrees:', errorMsg)
      } else {
        console.debug('[useWorktreeManager] Agent Loop not initialized, skipping cleanup')
      }
      throw err
    } finally {
      setIsLoading(false)
    }
  }, [])

  /** 获取磁盘使用量 */
  const getDiskUsage = useCallback(async () => {
    try {
      const bytes = await invoke<number>('get_worktree_disk_usage')
      setDiskUsage(bytes)
      return bytes
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : String(err)
      // Agent Loop 未初始化时不显示错误
      if (!errorMsg.includes('Agent Loop not initialized')) {
        console.error('[useWorktreeManager] Failed to get disk usage:', errorMsg)
      } else {
        console.debug('[useWorktreeManager] Agent Loop not initialized, skipping disk usage check')
      }
      return 0
    }
  }, [])

  /** 格式化磁盘使用量 */
  const formatDiskUsage = useCallback((bytes: number) => {
    if (bytes < 1024) return `${bytes} B`
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(2)} KB`
    if (bytes < 1024 * 1024 * 1024) return `${(bytes / 1024 / 1024).toFixed(2)} MB`
    return `${(bytes / 1024 / 1024 / 1024).toFixed(2)} GB`
  }, [])

  return {
    worktrees,
    diskUsage,
    formattedDiskUsage: formatDiskUsage(diskUsage),
    isLoading,
    error,
    createWorktree,
    removeWorktree,
    listWorktrees,
    cleanupOrphaned,
    getDiskUsage,
  }
}
