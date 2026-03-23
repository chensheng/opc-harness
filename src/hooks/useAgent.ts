import { useState, useCallback, useRef } from 'react'
import { invoke } from '@tauri-apps/api/core'
import type {
  AgentConfig,
  AgentMessage,
  AgentRequest,
  AgentResponse,
  DaemonState,
  UseAgentReturn,
} from '@/types/agent'

/**
 * Agent 通信和管理 Hook
 * 
 * 提供与 Agent 守护进程的完整通信能力:
 * - WebSocket 实时连接
 * - Agent 请求/响应
 * - 消息订阅/取消订阅
 * - 状态管理
 */
export function useAgent(): UseAgentReturn {
  const [agents, setAgents] = useState<AgentConfig[]>([])
  const [messages, setMessages] = useState<AgentMessage[]>([])
  const [daemonState, setDaemonState] = useState<DaemonState | null>(null)
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)
  
  const wsRef = useRef<WebSocket | null>(null)
  const sessionIdRef = useRef<string>('')

  /** 连接 WebSocket */
  const connectWebSocket = useCallback(async (sessionId: string) => {
    setIsLoading(true)
    setError(null)
    
    try {
      // TODO: 实现真实的 WebSocket 连接
      // 当前使用 Mock 实现
      sessionIdRef.current = sessionId
      
      // 模拟连接成功
      console.log('[useAgent] WebSocket connected:', sessionId)
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : 'Failed to connect WebSocket'
      setError(errorMsg)
      throw err
    } finally {
      setIsLoading(false)
    }
  }, [])

  /** 断开 WebSocket 连接 */
  const disconnectWebSocket = useCallback(() => {
    if (wsRef.current) {
      wsRef.current.close()
      wsRef.current = null
    }
    sessionIdRef.current = ''
    console.log('[useAgent] WebSocket disconnected')
  }, [])

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
      } catch (err) {
        const errorMsg = err instanceof Error ? err.message : 'Failed to send agent request'
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
    [],
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

  return {
    agents,
    messages,
    daemonState,
    isLoading,
    error,
    connectWebSocket,
    disconnectWebSocket,
    sendAgentRequest,
    subscribeAgent,
    unsubscribeAgent,
  }
}
