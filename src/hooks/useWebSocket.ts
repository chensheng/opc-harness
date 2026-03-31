import { useEffect, useRef, useCallback, useState } from 'react'

export interface WebSocketMessage {
  type: 'subscribe' | 'unsubscribe' | 'ping' | 'log' | 'notification' | 'pong' | 'error'
  topic?: string
  payload?: unknown
  timestamp?: number
  message?: string
}

interface UseWebSocketOptions {
  /** WebSocket 服务器 URL */
  url: string
  /** 是否自动连接 */
  autoConnect?: boolean
  /** 自动重连间隔（毫秒） */
  reconnectInterval?: number
  /** 最大重连次数 */
  maxReconnectAttempts?: number
  /** 心跳间隔（毫秒） */
  heartbeatInterval?: number
  /** 收到消息的回调 */
  onMessage?: (message: WebSocketMessage) => void
  /** 连接成功的回调 */
  onOpen?: () => void
  /** 连接关闭的回调 */
  onClose?: () => void
  /** 错误的回调 */
  onError?: (error: Event) => void
}

interface UseWebSocketReturn {
  /** WebSocket 连接状态 */
  connected: boolean
  /** 连接状态：connecting | connected | disconnected | error */
  status: string
  /** 发送消息 */
  sendMessage: (message: WebSocketMessage) => void
  /** 订阅主题 */
  subscribe: (topic: string) => void
  /** 取消订阅 */
  unsubscribe: (topic: string) => void
  /** 手动连接 */
  connect: () => void
  /** 断开连接 */
  disconnect: () => void
  /** 错误信息 */
  error: string | null
}

/**
 * WebSocket Hook - 提供实时通信能力
 */
export function useWebSocket(options: UseWebSocketOptions): UseWebSocketReturn {
  const {
    url,
    autoConnect = true,
    reconnectInterval = 3000,
    maxReconnectAttempts = 5,
    heartbeatInterval = 30000,
    onMessage,
    onOpen,
    onClose,
    onError,
  } = options

  const wsRef = useRef<WebSocket | null>(null)
  const reconnectCountRef = useRef(0)
  const heartbeatTimerRef = useRef<NodeJS.Timeout | null>(null)
  const [connected, setConnected] = useState(false)
  const [status, setStatus] = useState('disconnected')
  const [error, setError] = useState<string | null>(null)

  /**
   * 发送心跳
   */
  const sendHeartbeat = useCallback(() => {
    if (wsRef.current?.readyState === WebSocket.OPEN) {
      wsRef.current.send(JSON.stringify({ type: 'ping' }))
    }
  }, [])

  /**
   * 清理连接
   */
  const cleanup = useCallback(() => {
    if (heartbeatTimerRef.current) {
      clearInterval(heartbeatTimerRef.current)
      heartbeatTimerRef.current = null
    }

    if (wsRef.current) {
      wsRef.current.onopen = null
      wsRef.current.onclose = null
      wsRef.current.onerror = null
      wsRef.current.onmessage = null
      wsRef.current.close()
      wsRef.current = null
    }
  }, [])

  /**
   * 连接到 WebSocket 服务器
   */
  const connect = useCallback(() => {
    if (
      wsRef.current?.readyState === WebSocket.OPEN ||
      wsRef.current?.readyState === WebSocket.CONNECTING
    ) {
      return
    }

    setStatus('connecting')
    setError(null)

    try {
      const ws = new WebSocket(url)

      ws.onopen = () => {
        console.log('[WebSocket] Connected')
        setConnected(true)
        setStatus('connected')
        reconnectCountRef.current = 0
        onOpen?.()

        // 启动心跳
        heartbeatTimerRef.current = setInterval(sendHeartbeat, heartbeatInterval)
      }

      ws.onclose = () => {
        console.log('[WebSocket] Disconnected')
        setConnected(false)
        setStatus('disconnected')
        onClose?.()
        cleanup()

        // 尝试重连
        if (reconnectCountRef.current < maxReconnectAttempts) {
          reconnectCountRef.current++
          console.log(
            `[WebSocket] Reconnecting... (${reconnectCountRef.current}/${maxReconnectAttempts})`
          )
          setTimeout(connect, reconnectInterval)
        } else {
          console.error('[WebSocket] Max reconnect attempts reached')
          setError('连接失败，已达最大重试次数')
        }
      }

      ws.onerror = event => {
        console.error('[WebSocket] Error:', event)
        setStatus('error')
        onError?.(event)
        setError('WebSocket 连接错误')
      }

      ws.onmessage = event => {
        try {
          const message = JSON.parse(event.data) as WebSocketMessage
          console.log('[WebSocket] Message received:', message)
          onMessage?.(message)
        } catch (err) {
          console.error('[WebSocket] Failed to parse message:', err)
        }
      }

      wsRef.current = ws
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : '连接失败'
      console.error('[WebSocket] Connection error:', err)
      setError(errorMessage)
      setStatus('error')
    }
  }, [
    url,
    maxReconnectAttempts,
    reconnectInterval,
    sendHeartbeat,
    heartbeatInterval,
    onOpen,
    onClose,
    onError,
    onMessage,
    cleanup,
  ])

  /**
   * 断开连接
   */
  const disconnect = useCallback(() => {
    reconnectCountRef.current = maxReconnectAttempts + 1 // 阻止自动重连
    cleanup()
    setConnected(false)
    setStatus('disconnected')
  }, [maxReconnectAttempts, cleanup])

  /**
   * 发送消息
   */
  const sendMessage = useCallback((message: WebSocketMessage) => {
    if (wsRef.current?.readyState === WebSocket.OPEN) {
      wsRef.current.send(JSON.stringify(message))
    } else {
      console.warn('[WebSocket] Cannot send message: not connected')
    }
  }, [])

  /**
   * 订阅主题
   */
  const subscribe = useCallback(
    (topic: string) => {
      sendMessage({ type: 'subscribe', topic })
    },
    [sendMessage]
  )

  /**
   * 取消订阅
   */
  const unsubscribe = useCallback(
    (topic: string) => {
      sendMessage({ type: 'unsubscribe', topic })
    },
    [sendMessage]
  )

  // 自动连接
  useEffect(() => {
    if (autoConnect) {
      connect()
    }

    return () => {
      disconnect()
    }
  }, [autoConnect, connect, disconnect])

  return {
    connected,
    status,
    sendMessage,
    subscribe,
    unsubscribe,
    connect,
    disconnect,
    error,
  }
}
