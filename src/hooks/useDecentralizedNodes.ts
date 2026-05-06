import { useState, useCallback } from 'react'
import { invoke } from '@tauri-apps/api/core'

/**
 * 去中心化 Agent Node Hook
 *
 * 提供对单机去中心化智能体系统的控制能力:
 * - 启动/停止独立的 Agent Node 实例
 * - 查询所有运行中的 Nodes
 * - 监控节点状态
 */
export function useDecentralizedNodes() {
  const [nodes, setNodes] = useState<Array<{ node_id: string; is_running: boolean }>>([])
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  /** 启动新的去中心化 Agent Node */
  const startNode = useCallback(
    async (options?: { node_id?: string; redis_url?: string; max_concurrent?: number }) => {
      setIsLoading(true)
      setError(null)

      try {
        const nodeId = await invoke<string>('start_decentralized_node', {
          nodeId: options?.node_id,
          redisUrl: options?.redis_url || 'redis://127.0.0.1:6379',
          maxConcurrent: options?.max_concurrent || 3,
        })

        console.log('[useDecentralizedNodes] Started node:', nodeId)

        // 刷新节点列表
        await refreshNodes()

        return nodeId
      } catch (err) {
        const errorMsg = err instanceof Error ? err.message : String(err)
        setError(errorMsg)
        console.error('[useDecentralizedNodes] Failed to start node:', errorMsg)
        throw err
      } finally {
        setIsLoading(false)
      }
    },
    // eslint-disable-next-line react-hooks/exhaustive-deps
    []
  )

  /** 停止指定的 Agent Node */
  const stopNode = useCallback(async (nodeId: string) => {
    setIsLoading(true)
    setError(null)

    try {
      await invoke('stop_decentralized_node', { nodeId })

      console.log('[useDecentralizedNodes] Stopped node:', nodeId)

      // 刷新节点列表
      await refreshNodes()
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : String(err)
      setError(errorMsg)
      console.error('[useDecentralizedNodes] Failed to stop node:', errorMsg)
      throw err
    } finally {
      setIsLoading(false)
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [])

  /** 刷新节点列表 */
  const refreshNodes = useCallback(async () => {
    try {
      const nodesList = await invoke<Array<{ node_id: string; is_running: boolean }>>(
        'list_decentralized_nodes'
      )
      setNodes(nodesList)
      return nodesList
    } catch (err) {
      console.error('[useDecentralizedNodes] Failed to list nodes:', err)
      return []
    }
  }, [])

  /** 获取运行中的节点数量 */
  const getRunningCount = useCallback(() => {
    return nodes.filter(node => node.is_running).length
  }, [nodes])

  return {
    nodes,
    isLoading,
    error,
    startNode,
    stopNode,
    refreshNodes,
    getRunningCount,
  }
}
