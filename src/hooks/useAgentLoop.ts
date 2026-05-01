import { useState, useCallback, useEffect, useRef } from 'react'
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
  const [logs, setLogs] = useState<Array<{timestamp: Date, message: string, type: 'info' | 'error' | 'success'}>>([])
  const logsEndRef = useRef<HTMLDivElement>(null)

  // 自动滚动到最新日志
  useEffect(() => {
    logsEndRef.current?.scrollIntoView({ behavior: 'smooth' })
  }, [logs])

  /** 添加日志 */
  const addLog = useCallback((message: string, type: 'info' | 'error' | 'success' = 'info') => {
    setLogs(prev => [...prev, { timestamp: new Date(), message, type }])
    console.log(`[AgentLoop ${type.toUpperCase()}] ${message}`)
  }, [])

  /** 启动 Agent Loop 持续运行 */
  const startAgentLoop = useCallback(async (projectId: string, intervalSecs?: number) => {
    setIsLoading(true)
    setError(null)
    addLog(`正在启动 Agent Loop... (项目: ${projectId}, 间隔: ${intervalSecs || 60}秒)`, 'info')

    try {
      await invoke('start_agent_loop', {
        projectId,
        intervalSecs: intervalSecs || 60, // 默认 60 秒
      })
      
      setIsRunning(true)
      addLog(`✓ Agent Loop 已成功启动`, 'success')
      addLog(`📊 系统将每 ${intervalSecs || 60} 秒自动检测活跃的 Sprint 并执行待处理的用户故事`, 'info')
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : String(err)
      setError(errorMsg)
      addLog(`✗ 启动失败: ${errorMsg}`, 'error')
      throw err
    } finally {
      setIsLoading(false)
    }
  }, [addLog])

  /** 执行一次 Agent Loop(用于测试或手动触发) */
  const executeOnce = useCallback(async (projectId: string) => {
    setIsLoading(true)
    setError(null)
    addLog(`正在执行单次 Agent Loop... (项目: ${projectId})`, 'info')

    try {
      const startedCount = await invoke<number>('execute_agent_loop_once', {
        projectId,
      })
      
      addLog(`✓ 执行完成! 启动了 ${startedCount} 个 Agent`, 'success')
      if (startedCount > 0) {
        addLog(`📝 已为 ${startedCount} 个用户故事创建独立的 Worktree 并启动 Coding Agent`, 'info')
      } else {
        addLog(`ℹ️ 当前没有待处理的用户故事或没有活跃的 Sprint`, 'info')
      }
      return startedCount
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : String(err)
      setError(errorMsg)
      addLog(`✗ 执行失败: ${errorMsg}`, 'error')
      throw err
    } finally {
      setIsLoading(false)
    }
  }, [addLog])

  /** 停止 Agent Loop */
  const stopAgentLoop = useCallback(async () => {
    setIsLoading(true)
    setError(null)
    addLog(`正在停止 Agent Loop...`, 'info')

    try {
      await invoke('stop_agent_loop')
      
      setIsRunning(false)
      addLog(`✓ Agent Loop 已停止`, 'success')
      addLog(`⚠️ 系统将不再自动检测和执行业务故事`, 'info')
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : String(err)
      setError(errorMsg)
      addLog(`✗ 停止失败: ${errorMsg}`, 'error')
      throw err
    } finally {
      setIsLoading(false)
    }
  }, [addLog])

  /** 检查 Agent Loop 是否正在运行 */
  const checkStatus = useCallback(async () => {
    try {
      const running = await invoke<boolean>('is_agent_loop_running')
      const wasRunning = isRunning
      setIsRunning(running)
      
      // 状态变化时记录日志
      if (!wasRunning && running) {
        addLog('🔄 检测到 Agent Loop 正在运行', 'success')
      } else if (wasRunning && !running) {
        addLog('⏸️ 检测到 Agent Loop 已停止', 'info')
      }
      
      return running
    } catch (err) {
      console.error('[useAgentLoop] Failed to check Agent Loop status:', err)
      return false
    }
  }, [isRunning, addLog])

  /** 清空日志 */
  const clearLogs = useCallback(() => {
    setLogs([])
    addLog('日志已清空', 'info')
  }, [addLog])

  return {
    isRunning,
    isLoading,
    error,
    logs,
    logsEndRef,
    startAgentLoop,
    executeOnce,
    stopAgentLoop,
    checkStatus,
    clearLogs,
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

export function useWorktreeManager(projectId?: string) {
  const [worktrees, setWorktrees] = useState<WorktreeInfo[]>([])
  const [diskUsage, setDiskUsage] = useState<number>(0)
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  /** 创建 Worktree */
  const createWorktree = useCallback(async (agentId: string, storyId: string, branchName: string) => {
    if (!projectId) {
      throw new Error('Project ID is required for worktree operations')
    }
    
    setIsLoading(true)
    setError(null)

    try {
      const path = await invoke<string>('create_worktree', {
        projectId,
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
  }, [projectId])

  /** 删除 Worktree */
  const removeWorktree = useCallback(async (agentId: string) => {
    if (!projectId) {
      throw new Error('Project ID is required for worktree operations')
    }
    
    setIsLoading(true)
    setError(null)

    try {
      await invoke('remove_worktree', {
        projectId,
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
  }, [projectId])

  /** 列出所有 Worktrees */
  const listWorktrees = useCallback(async () => {
    setIsLoading(true)
    setError(null)

    try {
      const wtList = await invoke<WorktreeInfo[]>('list_worktrees', {
        projectId: projectId || null,  // 传递项目 ID，如果未提供则列出所有
      })
      setWorktrees(wtList)
      console.log('[useWorktreeManager] Listed', wtList.length, 'worktrees for project:', projectId || 'all')
      
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
  }, [projectId])

  /** 清理孤立的 Worktrees */
  const cleanupOrphaned = useCallback(async () => {
    if (!projectId) {
      throw new Error('Project ID is required for worktree cleanup')
    }
    
    setIsLoading(true)
    setError(null)

    try {
      const count = await invoke<number>('cleanup_orphaned_worktrees', {
        projectId,
      })
      console.log('[useWorktreeManager] Cleaned up', count, 'orphaned worktrees for project:', projectId)
      
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
  }, [projectId])

  /** 获取磁盘使用量 */
  const getDiskUsage = useCallback(async () => {
    try {
      const bytes = await invoke<number>('get_worktree_disk_usage', {
        projectId: projectId || null,
      })
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
  }, [projectId])

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
