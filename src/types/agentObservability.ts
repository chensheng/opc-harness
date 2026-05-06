/**
 * 智能体可观测性相关类型定义
 */

// ==================== 日志相关类型 ====================

export type LogLevel = 'info' | 'warn' | 'error' | 'debug' | 'success'

export interface LogEntry {
  id: string
  timestamp: Date
  level: LogLevel
  source: string
  message: string
  agentId: string
  sessionId: string
  createdAt?: Date
}

export interface LogStats {
  total: number
  info: number
  warn: number
  error: number
  debug: number
  success: number
}

export interface LogFilter {
  level?: LogLevel | 'all'
  source?: string
  keyword?: string
  startTime?: Date
  endTime?: Date
}

// ==================== 追踪相关类型 ====================

export type TraceEventType = 'thought' | 'tool_call' | 'tool_result' | 'decision'

export interface AgentTrace {
  id: string
  agentId: string
  sessionId: string
  eventType: TraceEventType
  timestamp: Date
  data: TraceData
  parentId?: string
  createdAt?: Date
}

export interface TraceData {
  // Thought 类型
  content?: string
  // ToolCall 类型
  toolName?: string
  parameters?: Record<string, unknown>
  // ToolResult 类型
  success?: boolean
  result?: unknown
  durationMs?: number
  // Decision 类型
  context?: string
  decision?: string
  reason?: string
  // 通用字段
  [key: string]: unknown
}

export interface TraceTreeNode extends AgentTrace {
  children: TraceTreeNode[]
  depth: number
}

// ==================== 告警相关类型 ====================

export type AlertLevel = 'warning' | 'critical'
export type AlertStatus = 'active' | 'resolved'
export type AlertType = 'no_response' | 'error_rate' | 'cpu_high' | 'memory_high'

export interface AgentAlert {
  id: string
  agentId: string
  level: AlertLevel
  alertType: AlertType
  message: string
  status: AlertStatus
  createdAt: Date
  resolvedAt?: Date
}

export interface AlertConfig {
  enabled: boolean
  thresholds: {
    noResponseWarningMinutes: number
    noResponseCriticalMinutes: number
    errorRateWarningPerMinute: number
    errorRateCriticalPerMinute: number
    cpuHighPercent: number
    memoryHighPercent: number
  }
}

export const DEFAULT_ALERT_CONFIG: AlertConfig = {
  enabled: true,
  thresholds: {
    noResponseWarningMinutes: 5,
    noResponseCriticalMinutes: 10,
    errorRateWarningPerMinute: 10,
    errorRateCriticalPerMinute: 30,
    cpuHighPercent: 90,
    memoryHighPercent: 95,
  },
}

// ==================== 性能指标类型 ====================

export interface PerformanceMetrics {
  agentId: string
  cpuUsage: number // 0-100
  memoryUsage: number // MB
  apiCalls: ApiCallStats
  lastUpdated: Date
}

export interface ApiCallStats {
  totalCalls: number
  successfulCalls: number
  failedCalls: number
  responseTimes: {
    p50: number // 毫秒
    p90: number
    p99: number
  }
}

// ==================== WebSocket 消息类型 ====================

export interface AgentMessage {
  type: 'log' | 'status' | 'progress' | 'error' | 'trace'
  sessionId: string
  timestamp: string
  content: string
  metadata?: {
    level?: LogLevel
    source?: string
    progress?: number
    currentTask?: string
    trace?: TraceData
  }
}

// ==================== 工具函数 ====================

/**
 * 将后端返回的日志级别转换为前端类型
 */
export function parseLogLevel(level: string): LogLevel {
  const validLevels: LogLevel[] = ['info', 'warn', 'error', 'debug', 'success']
  const lowerLevel = level.toLowerCase() as LogLevel
  return validLevels.includes(lowerLevel) ? lowerLevel : 'info'
}

/**
 * 将后端返回的事件类型转换为前端类型
 */
export function parseTraceEventType(eventType: string): TraceEventType {
  const validTypes: TraceEventType[] = ['thought', 'tool_call', 'tool_result', 'decision']
  const lowerType = eventType.toLowerCase() as TraceEventType
  return validTypes.includes(lowerType) ? lowerType : 'thought'
}

/**
 * 将 ISO 时间字符串转换为 Date 对象
 */
export function parseTimestamp(timestamp: string): Date {
  return new Date(timestamp)
}
