import { useState, useEffect, useRef } from 'react'
import { useParams } from 'react-router-dom'
import {
  Activity,
  AlertCircle,
  Terminal,
  CheckCircle,
  FileText,
  Trash2,
  Download,
  Filter,
  Search,
  EyeIcon,
} from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { useObservabilityStore } from '@/stores/observabilityStore'
import type { LogEntry, LogStats } from '@/types/agentObservability'

export function LogTerminal() {
  const { projectId } = useParams<{ projectId: string }>()

  // 使用 observability store 获取真实数据
  const getLogs = useObservabilityStore(state => state.getLogs)
  const getLogStats = useObservabilityStore(state => state.getLogStats)
  const clearLogsAction = useObservabilityStore(state => state.clearLogs)

  // 假设当前查看的是某个智能体的日志（实际应用中可能需要从上下文获取 agentId）
  const [currentAgentId] = useState<string>('default-agent')

  const [filter, setFilter] = useState<'all' | 'info' | 'warn' | 'error' | 'debug' | 'success'>(
    'all'
  )
  const [searchText, setSearchText] = useState('')
  const [isAutoScroll, setIsAutoScroll] = useState(true)
  const logsEndRef = useRef<HTMLDivElement>(null)

  // 从 store 获取日志和统计
  const logs = getLogs(currentAgentId, {
    level: filter === 'all' ? undefined : filter,
    keyword: searchText || undefined,
  })

  const stats: LogStats = getLogStats(currentAgentId)

  // Auto-scroll to bottom when new logs arrive
  useEffect(() => {
    if (isAutoScroll && logsEndRef.current) {
      logsEndRef.current.scrollIntoView({ behavior: 'smooth' })
    }
  }, [logs.length, isAutoScroll])

  const getLevelColor = (level: LogEntry['level']) => {
    switch (level) {
      case 'info':
        return 'text-blue-600 dark:text-blue-400'
      case 'warn':
        return 'text-yellow-600 dark:text-yellow-400'
      case 'error':
        return 'text-red-600 dark:text-red-400'
      case 'debug':
        return 'text-gray-600 dark:text-gray-400'
      case 'success':
        return 'text-green-600 dark:text-green-400'
    }
  }

  const getLevelIcon = (level: LogEntry['level']) => {
    switch (level) {
      case 'info':
        return <Activity className="w-3 h-3" />
      case 'warn':
        return <AlertCircle className="w-3 h-3" />
      case 'error':
        return <AlertCircle className="w-3 h-3" />
      case 'debug':
        return <Terminal className="w-3 h-3" />
      case 'success':
        return <CheckCircle className="w-3 h-3" />
    }
  }

  const handleClearLogs = () => {
    clearLogsAction(currentAgentId)
  }

  const exportLogs = () => {
    const logContent = logs
      .map(
        log =>
          `[${log.timestamp.toISOString()}] [${log.level.toUpperCase()}] [${log.source}] ${log.message}`
      )
      .join('\n')

    const blob = new Blob([logContent], { type: 'text/plain' })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = `logs-${projectId}-${new Date().toISOString()}.txt`
    document.body.appendChild(a)
    a.click()
    document.body.removeChild(a)
    URL.revokeObjectURL(url)
  }

  return (
    <div className="h-full flex flex-col p-6 space-y-6 overflow-hidden">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-2xl font-bold">实时日志终端</h2>
          <p className="text-muted-foreground mt-1">监控 AI Agent 执行过程和 CLI 工具输出</p>
        </div>
        <div className="flex gap-2">
          <Button variant="outline" size="sm" onClick={handleClearLogs}>
            <Trash2 className="w-4 h-4 mr-2" />
            清空
          </Button>
          <Button variant="outline" size="sm" onClick={exportLogs}>
            <Download className="w-4 h-4 mr-2" />
            导出
          </Button>
        </div>
      </div>

      {/* Statistics Cards */}
      <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-6 gap-4">
        <Card className="p-4">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm text-muted-foreground">总日志数</p>
              <p className="text-2xl font-bold">{stats.total}</p>
            </div>
            <FileText className="w-8 h-8 text-gray-500" />
          </div>
        </Card>

        <Card className="p-4">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm text-muted-foreground">信息</p>
              <p className="text-2xl font-bold text-blue-600">{stats.info}</p>
            </div>
            <Activity className="w-8 h-8 text-blue-500" />
          </div>
        </Card>

        <Card className="p-4">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm text-muted-foreground">警告</p>
              <p className="text-2xl font-bold text-yellow-600">{stats.warn}</p>
            </div>
            <AlertCircle className="w-8 h-8 text-yellow-500" />
          </div>
        </Card>

        <Card className="p-4">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm text-muted-foreground">错误</p>
              <p className="text-2xl font-bold text-red-600">{stats.error}</p>
            </div>
            <AlertCircle className="w-8 h-8 text-red-500" />
          </div>
        </Card>

        <Card className="p-4">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm text-muted-foreground">调试</p>
              <p className="text-2xl font-bold text-gray-600">{stats.debug}</p>
            </div>
            <Terminal className="w-8 h-8 text-gray-500" />
          </div>
        </Card>

        <Card className="p-4">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm text-muted-foreground">成功</p>
              <p className="text-2xl font-bold text-green-600">{stats.success}</p>
            </div>
            <CheckCircle className="w-8 h-8 text-green-500" />
          </div>
        </Card>
      </div>

      {/* Filter and Search */}
      <div className="flex items-center gap-4">
        <div className="flex items-center gap-2">
          <Filter className="w-4 h-4 text-muted-foreground" />
          <div className="flex gap-1">
            {(['all', 'info', 'warn', 'error', 'debug', 'success'] as const).map(level => (
              <Button
                key={level}
                variant={filter === level ? 'default' : 'outline'}
                size="sm"
                onClick={() => setFilter(level)}
                className="text-xs"
              >
                {level === 'all'
                  ? '全部'
                  : level === 'info'
                    ? '信息'
                    : level === 'warn'
                      ? '警告'
                      : level === 'error'
                        ? '错误'
                        : level === 'debug'
                          ? '调试'
                          : '成功'}
              </Button>
            ))}
          </div>
        </div>

        <div className="flex-1 relative">
          <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 w-4 h-4 text-muted-foreground" />
          <input
            type="text"
            placeholder="搜索日志..."
            value={searchText}
            onChange={e => setSearchText(e.target.value)}
            className="w-full pl-10 pr-4 py-2 border rounded-md bg-background text-sm focus:outline-none focus:ring-2 focus:ring-ring"
          />
        </div>

        <Button
          variant="outline"
          size="sm"
          onClick={() => setIsAutoScroll(!isAutoScroll)}
          className={isAutoScroll ? 'bg-green-50 dark:bg-green-950' : ''}
        >
          <EyeIcon className="w-4 h-4 mr-2" />
          自动滚动：{isAutoScroll ? '开' : '关'}
        </Button>
      </div>

      {/* Log Terminal */}
      <Card className="flex-1 overflow-hidden flex flex-col bg-slate-950 text-slate-50 font-mono text-sm">
        {/* Terminal Header */}
        <div className="flex items-center justify-between p-4 border-b border-slate-800 bg-slate-900">
          <div className="flex items-center gap-2">
            <Terminal className="w-4 h-4 text-green-500" />
            <span className="font-semibold">Console Output</span>
            <Badge variant="secondary" className="text-xs">
              {logs.length} / {stats.total} 条日志
            </Badge>
          </div>
          <div className="flex items-center gap-2 text-xs text-slate-400">
            <span>Project: {projectId}</span>
            <span>•</span>
            <span>{new Date().toLocaleString('zh-CN')}</span>
          </div>
        </div>

        {/* Log Content */}
        <div className="flex-1 overflow-auto p-4 space-y-1">
          {logs.length === 0 ? (
            <div className="flex items-center justify-center h-full text-muted-foreground">
              <div className="text-center">
                <Terminal className="w-12 h-12 mx-auto mb-2 opacity-50" />
                <p>暂无日志</p>
                <p className="text-xs mt-1">AI Agent 开始执行后将在此显示日志</p>
              </div>
            </div>
          ) : (
            logs.map((log: LogEntry) => (
              <div
                key={log.id}
                className="flex items-start gap-3 hover:bg-slate-900/50 px-2 py-1 rounded transition-colors"
              >
                {/* Timestamp */}
                <span className="text-slate-500 whitespace-nowrap text-xs">
                  {log.timestamp.toLocaleTimeString('zh-CN', { hour12: false })}
                </span>

                {/* Level Icon */}
                <span className={`${getLevelColor(log.level)} flex-shrink-0`}>
                  {getLevelIcon(log.level)}
                </span>

                {/* Source */}
                <span className="text-cyan-400 font-semibold whitespace-nowrap text-xs">
                  [{log.source}]
                </span>

                {/* Message */}
                <span className={`${getLevelColor(log.level)} flex-1 break-all`}>
                  {log.message}
                </span>
              </div>
            ))
          )}
          <div ref={logsEndRef} />
        </div>

        {/* Terminal Footer */}
        <div className="flex items-center justify-between p-3 border-t border-slate-800 bg-slate-900 text-xs text-slate-400">
          <div className="flex items-center gap-4">
            <span>UTF-8</span>
            <span>Lines: {logs.length}</span>
            <span>Size: ~{Math.round(logs.length * 0.1)}KB</span>
          </div>
          <div className="flex items-center gap-2">
            <span className={isAutoScroll ? 'text-green-500' : 'text-slate-500'}>
              ● {isAutoScroll ? '实时滚动' : '已暂停'}
            </span>
          </div>
        </div>
      </Card>
    </div>
  )
}
