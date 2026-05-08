import { useState, useEffect, useCallback } from 'react'
import { useWebSocket } from './useWebSocket'

// Checkpoint 类型定义（内联，避免架构违规）
export interface CheckpointData {
  id: string
  agent_id: string
  story_id: string
  checkpoint_type:
    | 'code_generation'
    | 'dependency_installation'
    | 'test_execution'
    | 'commit_review'
  status: 'pending' | 'approved' | 'rejected' | 'timed_out'
  data: {
    title: string
    description: string
    payload?: Record<string, unknown>
  }
  user_decision?: 'approve' | 'reject'
  user_feedback?: string
  created_at: string
  resolved_at?: string
  expires_at?: string
}

/**
 * Checkpoint Hook - 管理 HITL Checkpoint 的实时通信和 API 调用
 */
export function useCheckpoint(agentId?: string) {
  const [pendingCheckpoints, setPendingCheckpoints] = useState<CheckpointData[]>([])
  const [currentCheckpoint, setCurrentCheckpoint] = useState<CheckpointData | null>(null)
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  // WebSocket 连接（假设后端 WebSocket 服务器地址）
  const wsUrl = import.meta.env.VITE_WS_URL || 'ws://localhost:8080/ws'
  const { connected, subscribe } = useWebSocket({
    url: wsUrl,
    autoConnect: !!agentId,
    onMessage: message => {
      if (message.type === 'notification') {
        handleNotification(message.payload as Record<string, unknown>)
      }
    },
  })

  /**
   * 处理 WebSocket 通知
   */
  const handleNotification = useCallback(
    (payload: Record<string, unknown>) => {
      switch (payload.event) {
        case 'checkpoint_created': {
          // 收到新的 checkpoint
          const newCheckpoint = payload.data as CheckpointData
          setPendingCheckpoints(prev => [...prev, newCheckpoint])
          // 如果当前没有正在处理的 checkpoint，设置为当前
          if (!currentCheckpoint) {
            setCurrentCheckpoint(newCheckpoint)
          }
          break
        }

        case 'checkpoint_resolved': {
          // checkpoint 已解决，从待处理列表中移除
          const resolvedId = payload.checkpoint_id
          setPendingCheckpoints(prev => prev.filter(cp => cp.id !== resolvedId))
          if (currentCheckpoint?.id === resolvedId) {
            setCurrentCheckpoint(null)
          }
          break
        }

        case 'checkpoint_timeout': {
          // checkpoint 超时
          const timeoutId = payload.checkpoint_id
          setPendingCheckpoints(prev =>
            prev.map(cp => (cp.id === timeoutId ? { ...cp, status: 'timed_out' } : cp))
          )
          break
        }

        default:
          console.log('[Checkpoint] Unknown notification:', payload)
      }
    },
    [currentCheckpoint]
  )

  /**
   * 订阅 checkpoint 事件
   */
  useEffect(() => {
    if (connected && agentId) {
      subscribe(`agent:${agentId}:checkpoints`)
      console.log('[Checkpoint] Subscribed to checkpoints for agent:', agentId)
    }
  }, [connected, agentId, subscribe])

  /**
   * 获取待处理的 checkpoints
   */
  const fetchPendingCheckpoints = useCallback(async () => {
    if (!agentId) return

    setIsLoading(true)
    setError(null)

    try {
      // TODO: 调用 Tauri command 或 HTTP API 获取待处理 checkpoints
      // const response = await invoke('get_pending_checkpoints', { agentId })
      // setPendingCheckpoints(response)
      console.log('[Checkpoint] Fetching pending checkpoints for agent:', agentId)
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : '获取 checkpoints 失败'
      setError(errorMessage)
      console.error('[Checkpoint] Failed to fetch pending checkpoints:', err)
    } finally {
      setIsLoading(false)
    }
  }, [agentId])

  /**
   * 批准 checkpoint
   */
  const approveCheckpoint = useCallback(
    async (checkpointId: string, feedback?: string) => {
      setIsLoading(true)
      setError(null)

      try {
        // TODO: 调用 Tauri command 或 HTTP API
        // await invoke('resolve_checkpoint', {
        //   checkpointId,
        //   decision: 'approve',
        //   feedback,
        // })
        console.log('[Checkpoint] Approving checkpoint:', checkpointId, feedback)

        // 模拟 API 调用
        await new Promise(resolve => setTimeout(resolve, 500))

        // 从待处理列表中移除
        setPendingCheckpoints(prev => prev.filter(cp => cp.id !== checkpointId))
        if (currentCheckpoint?.id === checkpointId) {
          setCurrentCheckpoint(null)
        }
      } catch (err) {
        const errorMessage = err instanceof Error ? err.message : '批准失败'
        setError(errorMessage)
        console.error('[Checkpoint] Failed to approve checkpoint:', err)
        throw err
      } finally {
        setIsLoading(false)
      }
    },
    [currentCheckpoint]
  )

  /**
   * 拒绝 checkpoint
   */
  const rejectCheckpoint = useCallback(
    async (checkpointId: string, feedback?: string) => {
      setIsLoading(true)
      setError(null)

      try {
        // TODO: 调用 Tauri command 或 HTTP API
        // await invoke('resolve_checkpoint', {
        //   checkpointId,
        //   decision: 'reject',
        //   feedback,
        // })
        console.log('[Checkpoint] Rejecting checkpoint:', checkpointId, feedback)

        // 模拟 API 调用
        await new Promise(resolve => setTimeout(resolve, 500))

        // 从待处理列表中移除
        setPendingCheckpoints(prev => prev.filter(cp => cp.id !== checkpointId))
        if (currentCheckpoint?.id === checkpointId) {
          setCurrentCheckpoint(null)
        }
      } catch (err) {
        const errorMessage = err instanceof Error ? err.message : '拒绝失败'
        setError(errorMessage)
        console.error('[Checkpoint] Failed to reject checkpoint:', err)
        throw err
      } finally {
        setIsLoading(false)
      }
    },
    [currentCheckpoint]
  )

  /**
   * 批量批准所有待处理 checkpoints
   */
  const approveAllCheckpoints = useCallback(async () => {
    setIsLoading(true)
    setError(null)

    try {
      // 逐个批准
      for (const checkpoint of pendingCheckpoints) {
        await approveCheckpoint(checkpoint.id)
      }
      console.log('[Checkpoint] Approved all checkpoints')
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : '批量批准失败'
      setError(errorMessage)
      console.error('[Checkpoint] Failed to approve all checkpoints:', err)
      throw err
    } finally {
      setIsLoading(false)
    }
  }, [pendingCheckpoints, approveCheckpoint])

  /**
   * 显示下一个 checkpoint
   */
  const showNextCheckpoint = useCallback(() => {
    if (pendingCheckpoints.length > 0 && !currentCheckpoint) {
      setCurrentCheckpoint(pendingCheckpoints[0])
    }
  }, [pendingCheckpoints, currentCheckpoint])

  // 初始加载
  useEffect(() => {
    if (agentId) {
      fetchPendingCheckpoints()
    }
  }, [agentId, fetchPendingCheckpoints])

  return {
    // 状态
    connected,
    pendingCheckpoints,
    currentCheckpoint,
    isLoading,
    error,
    pendingCount: pendingCheckpoints.length,

    // 操作
    approveCheckpoint,
    rejectCheckpoint,
    approveAllCheckpoints,
    showNextCheckpoint,
    fetchPendingCheckpoints,
    setCurrentCheckpoint,
  }
}
