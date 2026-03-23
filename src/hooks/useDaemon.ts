import { useState, useCallback } from 'react'
import { invoke } from '@tauri-apps/api/core'
import type { DaemonSnapshot, DaemonStatusType, DaemonConfig, UseDaemonReturn } from '@/types/agent'

/**
 * 守护进程管理 Hook
 *
 * 提供完整的守护进程生命周期管理能力:
 * - 启动/停止/暂停/恢复
 * - Agent 进程管理
 * - 状态快照获取
 * - 资源监控
 */
export function useDaemon(): UseDaemonReturn {
  const [snapshot, setSnapshot] = useState<DaemonSnapshot | null>(null)
  const [status, setStatus] = useState<DaemonStatusType | null>(null)
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  /** 启动守护进程 */
  const startDaemon = useCallback(async (config: Partial<DaemonConfig>) => {
    setIsLoading(true)
    setError(null)

    try {
      const result = await invoke<DaemonSnapshot>('start_daemon', {
        sessionId: config.sessionId || crypto.randomUUID(),
        projectPath: config.projectPath || '',
        logLevel: config.logLevel || 'info',
        maxConcurrentAgents: config.maxConcurrentAgents || 5,
      })

      setSnapshot(result)
      setStatus(result.status)
      console.log('[useDaemon] Daemon started:', result.daemonId)
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : 'Failed to start daemon'
      setError(errorMsg)
      console.error('[useDaemon] Start failed:', errorMsg)
      throw new Error(errorMsg)
    } finally {
      setIsLoading(false)
    }
  }, [])

  /** 停止守护进程 */
  const stopDaemon = useCallback(async (graceful: boolean = true) => {
    setIsLoading(true)
    setError(null)

    try {
      await invoke<void>('stop_daemon', { graceful })
      setStatus('stopped')
      setSnapshot(null)
      console.log('[useDaemon] Daemon stopped')
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : 'Failed to stop daemon'
      setError(errorMsg)
      console.error('[useDaemon] Stop failed:', errorMsg)
      throw new Error(errorMsg)
    } finally {
      setIsLoading(false)
    }
  }, [])

  /** 暂停守护进程 */
  const pauseDaemon = useCallback(async () => {
    setIsLoading(true)
    setError(null)

    try {
      await invoke<void>('pause_daemon')
      setStatus('paused')
      console.log('[useDaemon] Daemon paused')
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : 'Failed to pause daemon'
      setError(errorMsg)
      console.error('[useDaemon] Pause failed:', errorMsg)
      throw new Error(errorMsg)
    } finally {
      setIsLoading(false)
    }
  }, [])

  /** 恢复守护进程 */
  const resumeDaemon = useCallback(async () => {
    setIsLoading(true)
    setError(null)

    try {
      await invoke<void>('resume_daemon')
      setStatus('running')
      console.log('[useDaemon] Daemon resumed')
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : 'Failed to resume daemon'
      setError(errorMsg)
      console.error('[useDaemon] Resume failed:', errorMsg)
      throw new Error(errorMsg)
    } finally {
      setIsLoading(false)
    }
  }, [])

  /** 生成新的 Agent */
  const spawnAgent = useCallback(async (agentType: string): Promise<string> => {
    setIsLoading(true)
    setError(null)

    try {
      const agentId = await invoke<string>('spawn_agent', { agentType })
      console.log('[useDaemon] Agent spawned:', agentId)
      return agentId
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : 'Failed to spawn agent'
      setError(errorMsg)
      console.error('[useDaemon] Spawn failed:', errorMsg)
      throw new Error(errorMsg)
    } finally {
      setIsLoading(false)
    }
  }, [])

  /** 终止指定 Agent */
  const killAgent = useCallback(async (agentId: string) => {
    setIsLoading(true)
    setError(null)

    try {
      await invoke<void>('kill_agent', { agentId })
      console.log('[useDaemon] Agent killed:', agentId)
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : 'Failed to kill agent'
      setError(errorMsg)
      console.error('[useDaemon] Kill failed:', errorMsg)
      throw new Error(errorMsg)
    } finally {
      setIsLoading(false)
    }
  }, [])

  /** 刷新快照 */
  const refreshSnapshot = useCallback(async () => {
    setError(null)

    try {
      const result = await invoke<DaemonSnapshot>('get_daemon_snapshot')
      setSnapshot(result)
      setStatus(result.status)
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : 'Failed to get snapshot'
      setError(errorMsg)
      console.error('[useDaemon] Refresh snapshot failed:', errorMsg)
    }
  }, [])

  return {
    snapshot,
    status,
    isLoading,
    error,
    startDaemon,
    stopDaemon,
    pauseDaemon,
    resumeDaemon,
    spawnAgent,
    killAgent,
    refreshSnapshot,
  }
}
