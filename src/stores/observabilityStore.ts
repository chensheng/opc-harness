/**
 * 智能体可观测性状态管理
 * 使用 Zustand 管理日志、追踪、告警和性能指标
 */

import { create } from 'zustand'
import type {
  LogEntry,
  LogStats,
  LogFilter,
  AgentTrace,
  TraceTreeNode,
  AgentAlert,
  AlertConfig,
  PerformanceMetrics,
} from '@/types/agentObservability'
import { DEFAULT_ALERT_CONFIG } from '@/types/agentObservability'

interface ObservabilityState {
  // ==================== 日志状态 ====================
  // 按智能体 ID 分组的日志缓存
  logsByAgent: Map<string, LogEntry[]>
  // 日志统计
  logStatsByAgent: Map<string, LogStats>

  // ==================== 追踪状态 ====================
  // 按智能体 ID 分组的追踪记录
  tracesByAgent: Map<string, AgentTrace[]>
  // 按智能体 ID 分组的追踪树
  traceTreesByAgent: Map<string, TraceTreeNode[]>

  // ==================== 告警状态 ====================
  // 活跃告警列表
  activeAlerts: AgentAlert[]
  // 告警历史
  alertHistory: AgentAlert[]
  // 告警配置
  alertConfig: AlertConfig

  // ==================== 性能指标状态 ====================
  // 按智能体 ID 分组的性能指标
  metricsByAgent: Map<string, PerformanceMetrics>

  // ==================== 日志操作 ====================
  /** 添加日志条目 */
  addLog: (agentId: string, log: LogEntry) => void
  /** 批量添加日志 */
  addLogsBatch: (agentId: string, logs: LogEntry[]) => void
  /** 获取智能体的日志 */
  getLogs: (agentId: string, filter?: LogFilter) => LogEntry[]
  /** 获取日志统计 */
  getLogStats: (agentId: string) => LogStats
  /** 清空智能体的日志 */
  clearLogs: (agentId: string) => void
  /** 清空所有日志 */
  clearAllLogs: () => void

  // ==================== 追踪操作 ====================
  /** 添加追踪记录 */
  addTrace: (agentId: string, trace: AgentTrace) => void
  /** 批量添加追踪记录 */
  addTracesBatch: (agentId: string, traces: AgentTrace[]) => void
  /** 获取智能体的追踪记录 */
  getTraces: (agentId: string, eventType?: string) => AgentTrace[]
  /** 获取追踪树 */
  getTraceTree: (agentId: string) => TraceTreeNode[]
  /** 清空智能体的追踪 */
  clearTraces: (agentId: string) => void

  // ==================== 告警操作 ====================
  /** 添加告警 */
  addAlert: (alert: AgentAlert) => void
  /** 解决告警 */
  resolveAlert: (alertId: string) => void
  /** 获取活跃告警 */
  getActiveAlerts: () => AgentAlert[]
  /** 获取告警历史 */
  getAlertHistory: () => AgentAlert[]
  /** 更新告警配置 */
  updateAlertConfig: (config: Partial<AlertConfig>) => void
  /** 清空所有告警 */
  clearAllAlerts: () => void

  // ==================== 性能指标操作 ====================
  /** 更新性能指标 */
  updateMetrics: (agentId: string, metrics: Partial<PerformanceMetrics>) => void
  /** 获取性能指标 */
  getMetrics: (agentId: string) => PerformanceMetrics | undefined
  /** 清空所有性能指标 */
  clearAllMetrics: () => void
}

// 日志缓存限制
const LOG_CACHE_LIMIT = 100

export const useObservabilityStore = create<ObservabilityState>((set, get) => ({
  // ==================== 初始状态 ====================
  logsByAgent: new Map(),
  logStatsByAgent: new Map(),
  tracesByAgent: new Map(),
  traceTreesByAgent: new Map(),
  activeAlerts: [],
  alertHistory: [],
  alertConfig: DEFAULT_ALERT_CONFIG,
  metricsByAgent: new Map(),

  // ==================== 日志操作 ====================
  addLog: (agentId, log) => {
    set(state => {
      const logs = state.logsByAgent.get(agentId) || []
      const newLogs = [...logs, log]

      // 限制缓存大小
      if (newLogs.length > LOG_CACHE_LIMIT) {
        newLogs.shift()
      }

      // 更新统计
      const stats = { ...getLogStatsFromLogs(newLogs) }

      return {
        logsByAgent: new Map(state.logsByAgent.set(agentId, newLogs)),
        logStatsByAgent: new Map(state.logStatsByAgent.set(agentId, stats)),
      }
    })
  },

  addLogsBatch: (agentId, logs) => {
    set(state => {
      const existingLogs = state.logsByAgent.get(agentId) || []
      const newLogs = [...existingLogs, ...logs]

      // 限制缓存大小
      while (newLogs.length > LOG_CACHE_LIMIT) {
        newLogs.shift()
      }

      const stats = getLogStatsFromLogs(newLogs)

      return {
        logsByAgent: new Map(state.logsByAgent.set(agentId, newLogs)),
        logStatsByAgent: new Map(state.logStatsByAgent.set(agentId, stats)),
      }
    })
  },

  getLogs: (agentId, filter) => {
    const logs = get().logsByAgent.get(agentId) || []

    if (!filter) {
      return logs
    }

    return logs.filter(log => {
      if (filter.level && filter.level !== 'all' && log.level !== filter.level) {
        return false
      }
      if (filter.source && log.source !== filter.source) {
        return false
      }
      if (filter.keyword) {
        const keyword = filter.keyword.toLowerCase()
        if (
          !log.message.toLowerCase().includes(keyword) &&
          !log.source.toLowerCase().includes(keyword)
        ) {
          return false
        }
      }
      if (filter.startTime && log.timestamp < filter.startTime) {
        return false
      }
      if (filter.endTime && log.timestamp > filter.endTime) {
        return false
      }
      return true
    })
  },

  getLogStats: agentId => {
    return (
      get().logStatsByAgent.get(agentId) || {
        total: 0,
        info: 0,
        warn: 0,
        error: 0,
        debug: 0,
        success: 0,
      }
    )
  },

  clearLogs: agentId => {
    set(state => {
      const newLogsByAgent = new Map(state.logsByAgent)
      newLogsByAgent.delete(agentId)
      const newLogStatsByAgent = new Map(state.logStatsByAgent)
      newLogStatsByAgent.delete(agentId)
      return {
        logsByAgent: newLogsByAgent,
        logStatsByAgent: newLogStatsByAgent,
      }
    })
  },

  clearAllLogs: () => {
    set({
      logsByAgent: new Map(),
      logStatsByAgent: new Map(),
    })
  },

  // ==================== 追踪操作 ====================
  addTrace: (agentId, trace) => {
    set(state => {
      const traces = state.tracesByAgent.get(agentId) || []
      const newTraces = [...traces, trace]

      return {
        tracesByAgent: new Map(state.tracesByAgent.set(agentId, newTraces)),
      }
    })
  },

  addTracesBatch: (agentId, traces) => {
    set(state => {
      const existingTraces = state.tracesByAgent.get(agentId) || []
      const newTraces = [...existingTraces, ...traces]

      return {
        tracesByAgent: new Map(state.tracesByAgent.set(agentId, newTraces)),
      }
    })
  },

  getTraces: (agentId, eventType) => {
    const traces = get().tracesByAgent.get(agentId) || []

    if (!eventType) {
      return traces
    }

    return traces.filter(trace => trace.eventType === eventType)
  },

  getTraceTree: agentId => {
    const traces = get().tracesByAgent.get(agentId) || []

    // 构建追踪树
    const nodeMap = new Map<string, TraceTreeNode>()
    const roots: TraceTreeNode[] = []

    // 首先创建所有节点
    traces.forEach(trace => {
      nodeMap.set(trace.id, {
        ...trace,
        children: [],
        depth: 0,
      })
    })

    // 建立父子关系
    traces.forEach(trace => {
      const node = nodeMap.get(trace.id)
      if (!node) return

      if (trace.parentId) {
        const parent = nodeMap.get(trace.parentId)
        if (parent) {
          parent.children.push(node)
          node.depth = parent.depth + 1
        } else {
          roots.push(node)
        }
      } else {
        roots.push(node)
      }
    })

    // 按时间排序
    roots.sort((a, b) => a.timestamp.getTime() - b.timestamp.getTime())

    return roots
  },

  clearTraces: agentId => {
    set(state => {
      const newTracesByAgent = new Map(state.tracesByAgent)
      newTracesByAgent.delete(agentId)
      const newTraceTreesByAgent = new Map(state.traceTreesByAgent)
      newTraceTreesByAgent.delete(agentId)
      return {
        tracesByAgent: newTracesByAgent,
        traceTreesByAgent: newTraceTreesByAgent,
      }
    })
  },

  // ==================== 告警操作 ====================
  addAlert: alert => {
    set(state => ({
      activeAlerts: [...state.activeAlerts, alert],
      alertHistory: [...state.alertHistory, alert],
    }))
  },

  resolveAlert: alertId => {
    set(state => ({
      activeAlerts: state.activeAlerts.filter(a => a.id !== alertId),
      alertHistory: state.alertHistory.map(a =>
        a.id === alertId ? { ...a, status: 'resolved', resolvedAt: new Date() } : a
      ),
    }))
  },

  getActiveAlerts: () => {
    return get().activeAlerts
  },

  getAlertHistory: () => {
    return get().alertHistory
  },

  updateAlertConfig: config => {
    set(state => ({
      alertConfig: { ...state.alertConfig, ...config },
    }))
  },

  clearAllAlerts: () => {
    set({
      activeAlerts: [],
      alertHistory: [],
    })
  },

  // ==================== 性能指标操作 ====================
  updateMetrics: (agentId, metrics) => {
    set(state => {
      const existing = state.metricsByAgent.get(agentId)
      const newMetrics = existing
        ? { ...existing, ...metrics }
        : {
            agentId,
            cpuUsage: 0,
            memoryUsage: 0,
            apiCalls: {
              totalCalls: 0,
              successfulCalls: 0,
              failedCalls: 0,
              responseTimes: { p50: 0, p90: 0, p99: 0 },
            },
            lastUpdated: new Date(),
            ...metrics,
          }

      return {
        metricsByAgent: new Map(state.metricsByAgent.set(agentId, newMetrics)),
      }
    })
  },

  getMetrics: agentId => {
    return get().metricsByAgent.get(agentId)
  },

  clearAllMetrics: () => {
    set({
      metricsByAgent: new Map(),
    })
  },
}))

// 辅助函数：从日志列表计算统计
function getLogStatsFromLogs(logs: LogEntry[]): LogStats {
  const stats: LogStats = {
    total: logs.length,
    info: 0,
    warn: 0,
    error: 0,
    debug: 0,
    success: 0,
  }

  logs.forEach(log => {
    switch (log.level) {
      case 'info':
        stats.info++
        break
      case 'warn':
        stats.warn++
        break
      case 'error':
        stats.error++
        break
      case 'debug':
        stats.debug++
        break
      case 'success':
        stats.success++
        break
    }
  })

  return stats
}
