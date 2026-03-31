import { useState, useEffect, useCallback, useRef } from 'react'
import { useWebSocket, type WebSocketMessage } from './useWebSocket'

/**
 * 日志级别枚举
 */
export enum LogLevel {
  INFO = 'INFO',
  WARN = 'WARN',
  ERROR = 'ERROR',
  DEBUG = 'DEBUG',
}

/**
 * 日志条目接口
 */
export interface LogEntry {
  id: string
  timestamp: number
  level: LogLevel
  message: string
  source?: string
}

interface UseAgentLogsOptions {
  /** WebSocket 服务器 URL */
  wsUrl: string
  /** 是否自动连接 */
  autoConnect?: boolean
  /** 最大日志条数 */
  maxLogs?: number
  /** 收到新日志的回调 */
  onNewLog?: (log: LogEntry) => void
}

interface UseAgentLogsReturn {
  /** 日志列表 */
  logs: LogEntry[]
  /** 连接状态 */
  connected: boolean
  /** 是否暂停 */
  paused: boolean
  /** 暂停日志显示 */
  pause: () => void
  /** 恢复日志显示 */
  resume: () => void
  /** 清空日志 */
  clear: () => void
  /** 按级别过滤 */
  filterLevel: LogLevel | null
  /** 设置过滤级别 */
  setFilterLevel: (level: LogLevel | null) => void
}

/**
 * Agent 日志 Hook - 实时接收和显示 Agent 执行日志
 */
export function useAgentLogs(options: UseAgentLogsOptions): UseAgentLogsReturn {
  const { wsUrl, autoConnect = true, maxLogs = 1000, onNewLog } = options

  const [logs, setLogs] = useState<LogEntry[]>([])
  const [paused, setPaused] = useState(false)
  const [filterLevel, setFilterLevel] = useState<LogLevel | null>(null)

  const logBuffer = useRef<LogEntry[]>([])

  // 使用 WebSocket Hook
  const { connected, subscribe } = useWebSocket({
    url: wsUrl,
    autoConnect,
    onMessage: useCallback(
      (message: WebSocketMessage) => {
        if (message.type === 'log' && message.topic === 'agent-logs') {
          const logEntry: LogEntry = {
            id: `${message.timestamp}-${Math.random()}`,
            timestamp: message.timestamp || Date.now(),
            level: message.payload?.level || LogLevel.INFO,
            message: message.payload?.message || '',
            source: message.payload?.source,
          }

          if (!paused) {
            setLogs(prev => {
              const newLogs = [...prev, logEntry]
              // 限制日志数量
              if (newLogs.length > maxLogs) {
                return newLogs.slice(newLogs.length - maxLogs)
              }
              return newLogs
            })

            onNewLog?.(logEntry)
          } else {
            // 暂停时缓存日志
            logBuffer.current.push(logEntry)
          }
        }
      },
      [paused, maxLogs, onNewLog]
    ),
  })

  // 连接后订阅 agent-logs 主题
  useEffect(() => {
    if (connected) {
      subscribe('agent-logs')
    }
  }, [connected, subscribe])

  /**
   * 暂停日志显示
   */
  const pause = useCallback(() => {
    setPaused(true)
  }, [])

  /**
   * 恢复日志显示
   */
  const resume = useCallback(() => {
    setPaused(false)

    // 添加缓存的日志
    if (logBuffer.current.length > 0) {
      setLogs(prev => {
        const newLogs = [...prev, ...logBuffer.current]
        if (newLogs.length > maxLogs) {
          return newLogs.slice(newLogs.length - maxLogs)
        }
        return newLogs
      })
      logBuffer.current = []
    }
  }, [maxLogs])

  /**
   * 清空日志
   */
  const clear = useCallback(() => {
    setLogs([])
    logBuffer.current = []
  }, [])

  /**
   * 按级别过滤日志
   */
  const filteredLogs = filterLevel ? logs.filter(log => log.level === filterLevel) : logs

  return {
    logs: filteredLogs,
    connected,
    paused,
    pause,
    resume,
    clear,
    filterLevel,
    setFilterLevel,
  }
}
