import { useState, useCallback, useRef, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { listen, UnlistenFn } from '@tauri-apps/api/event'
import type {
  AgentConfig,
  AgentMessage,
  AgentRequest,
  AgentResponse,
  DaemonState,
  UseAgentReturn,
} from '@/types/agent'

// WebSocket 消息类型定义
interface WsMessage {
  id: string
  session_id: string
  type: 'Log' | 'Progress' | 'Status' | 'AgentResponse' | 'Error' | 'Heartbeat'
  payload: any
  timestamp: number
}

interface WsLogPayload {
  level: string
  message: string
  source?: string
}

interface WsProgressPayload {
  phase: string
  current: number
  total: number
  description?: string
}

interface WsStatusPayload {
  status: string
  details?: string
}

/**
 * Agent 通信和管理 Hook
 *
 * 提供与 Agent 守护进程的完整通信能力:
 * - Tauri Events 实时连接（替代 WebSocket）
 * - Agent 请求/响应
 * - 消息订阅/取消订阅
 * - 状态管理
 */
export function useAgent(): UseAgentReturn {
  const [_agents] = useState<AgentConfig[]>([])
  const [messages, setMessages] = useState<AgentMessage[]>([])
  const [_daemonState] = useState<DaemonState | null>(null)
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const sessionIdRef = useRef<string>('')
  const connectionIdRef = useRef<string>('')
  const unlistenRef = useRef<UnlistenFn | null>(null)

  /** 处理接收到的 WebSocket 消息 */
  const handleWsMessage = useCallback((wsMessage: WsMessage) => {
    console.log('[useAgent] Received WebSocket message:', wsMessage)

    // 根据消息类型转换为 AgentMessage
    let agentMessage: AgentMessage | null = null

    switch (wsMessage.type) {
      case 'Log': {
        const logPayload = wsMessage.payload as WsLogPayload
        agentMessage = {
          id: wsMessage.id,
          sessionId: wsMessage.session_id,
          type: 'log',
          content: logPayload.message,
          metadata: {
            level: logPayload.level,
            source: logPayload.source,
            timestamp: wsMessage.timestamp,
          },
          timestamp: new Date(wsMessage.timestamp).toISOString(),
        }
        break
      }

      case 'Progress': {
        const progressPayload = wsMessage.payload as WsProgressPayload
        agentMessage = {
          id: wsMessage.id,
          sessionId: wsMessage.session_id,
          type: 'progress',
          content: progressPayload.description || `${progressPayload.phase}: ${progressPayload.current}/${progressPayload.total}`,
          metadata: {
            phase: progressPayload.phase,
            current: progressPayload.current,
            total: progressPayload.total,
            timestamp: wsMessage.timestamp,
          },
          timestamp: new Date(wsMessage.timestamp).toISOString(),
        }
        break
      }

      case 'Status': {
        const statusPayload = wsMessage.payload as WsStatusPayload
        agentMessage = {
          id: wsMessage.id,
          sessionId: wsMessage.session_id,
          type: 'status',
          content: statusPayload.details || statusPayload.status,
          metadata: {
            status: statusPayload.status,
            timestamp: wsMessage.timestamp,
          },
          timestamp: new Date(wsMessage.timestamp).toISOString(),
        }
        break
      }

      case 'AgentResponse': {
        agentMessage = {
          id: wsMessage.id,
          sessionId: wsMessage.session_id,
          type: 'response',
          content: JSON.stringify(wsMessage.payload.data || wsMessage.payload.error),
          metadata: {
            requestId: wsMessage.payload.request_id,
            success: wsMessage.payload.success,
            timestamp: wsMessage.timestamp,
          },
          timestamp: new Date(wsMessage.timestamp).toISOString(),
        }
        break
      }

      case 'Error': {
        agentMessage = {
          id: wsMessage.id,
          sessionId: wsMessage.session_id,
          type: 'error',
          content: wsMessage.payload.message,
          metadata: {
            code: wsMessage.payload.code,
            details: wsMessage.payload.details,
            timestamp: wsMessage.timestamp,
          },
          timestamp: new Date(wsMessage.timestamp).toISOString(),
        }
        break
      }

      case 'Heartbeat': {
        // 心跳消息不添加到消息列表，仅用于连接健康检查
        console.log('[useAgent] Heartbeat received:', wsMessage.timestamp)
        return
      }

      default:
        console.warn('[useAgent] Unknown message type:', wsMessage.type)
        return
    }

    if (agentMessage) {
      setMessages((prev) => [...prev, agentMessage])
    }
  }, [])

  /** 连接 Tauri Events（替代 WebSocket） */
  const connectWebSocket = useCallback(async (sessionId: string) => {
    setIsLoading(true)
    setError(null)

    try {
      sessionIdRef.current = sessionId

      // Step 1: 注册连接
      console.log('[useAgent] Registering WebSocket connection for session:', sessionId)
      const connectionId = await invoke<string>('ws_register_connection', { sessionId })
      connectionIdRef.current = connectionId
      console.log('[useAgent] Connection registered:', connectionId)

      // Step 2: 建立事件监听器
      const eventName = `ws:${sessionId}`
      console.log('[useAgent] Listening to event:', eventName)

      const unlisten = await listen(eventName, (event) => {
        const wsMessage = event.payload as WsMessage
        handleWsMessage(wsMessage)
      })

      unlistenRef.current = unlisten
      console.log('[useAgent] ✓ Event listener established')

      // Step 3: 发送初始状态消息
      await invoke('ws_send_status', {
        sessionId,
        status: 'connected',
        details: 'Frontend connected to Agent system',
      })
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : 'Failed to connect to Agent system'
      setError(errorMsg)
      console.error('[useAgent] Connection failed:', err)
      throw new Error(errorMsg)
    } finally {
      setIsLoading(false)
    }
  }, [handleWsMessage])

  /** 断开 Tauri Events 连接 */
  const disconnectWebSocket = useCallback(async () => {
    // Step 1: 移除事件监听器
    if (unlistenRef.current) {
      unlistenRef.current()
      unlistenRef.current = null
      console.log('[useAgent] Event listener removed')
    }

    // Step 2: 注销连接
    if (connectionIdRef.current && sessionIdRef.current) {
      try {
        await invoke('ws_unregister_connection', {
          sessionId: sessionIdRef.current,
          connectionId: connectionIdRef.current,
        })
        console.log('[useAgent] Connection unregistered')
      } catch (err) {
        console.error('[useAgent] Failed to unregister connection:', err)
      }
    }

    sessionIdRef.current = ''
    connectionIdRef.current = ''
    console.log('[useAgent] ✓ Disconnected from Agent system')
  }, [])

  /** 清理函数：组件卸载时自动断开连接 */
  useEffect(() => {
    return () => {
      if (sessionIdRef.current) {
        disconnectWebSocket()
      }
    }
  }, [disconnectWebSocket])

  /** 发送 Agent 请求 */
  const sendAgentRequest = useCallback(
    async (agentId: string, action: string, payload: unknown): Promise<AgentResponse> => {
      setIsLoading(true)
      setError(null)

      try {
        const request: AgentRequest = {
          requestId: crypto.randomUUID(),
          agentId,
          action,
          payload,
        }

        // TODO: 实现真实的 invoke 调用
        // const response = await invoke<AgentResponse>('send_agent_request', { request })

        // Mock 响应
        const response: AgentResponse = {
          responseId: crypto.randomUUID(),
          requestId: request.requestId,
          success: true,
          data: { message: 'Mock response' },
        }

        return response
      } catch {
        const errorMsg = 'Failed to send agent request'
        setError(errorMsg)
        return {
          responseId: crypto.randomUUID(),
          requestId: crypto.randomUUID(),
          success: false,
          error: errorMsg,
        }
      } finally {
        setIsLoading(false)
      }
    },
    []
  )

  /** 订阅 Agent 消息 */
  const subscribeAgent = useCallback((agentId: string) => {
    console.log('[useAgent] Subscribed to agent:', agentId)
    // TODO: 实现真实的订阅逻辑
  }, [])

  /** 取消订阅 Agent 消息 */
  const unsubscribeAgent = useCallback((agentId: string) => {
    console.log('[useAgent] Unsubscribed from agent:', agentId)
    // TODO: 实现真实的取消订阅逻辑
  }, [])

  /** 清空消息列表 */
  const clearMessages = useCallback(() => {
    setMessages([])
  }, [])

  return {
    agents: _agents,
    messages,
    daemonState: _daemonState,
    isLoading,
    error,
    connectWebSocket,
    disconnectWebSocket,
    sendAgentRequest,
    subscribeAgent,
    unsubscribeAgent,
    clearMessages,
  }
}
