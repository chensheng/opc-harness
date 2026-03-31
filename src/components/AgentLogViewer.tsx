import React, { useEffect, useRef } from 'react'
import { useAgentLogs, LogLevel } from '../hooks/useAgentLogs'
import { ScrollArea } from './ui/scroll-area'
import { Button } from './ui/button'
import { Badge } from './ui/badge'
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from './ui/select'

interface AgentLogViewerProps {
  /** WebSocket 服务器 URL */
  wsUrl: string
  /** 最大显示日志条数 */
  maxLogs?: number
  /** 是否显示控制按钮 */
  showControls?: boolean
}

/**
 * Agent 日志查看器组件
 */
export function AgentLogViewer({
  wsUrl,
  maxLogs = 1000,
  showControls = true,
}: AgentLogViewerProps) {
  const scrollRef = useRef<HTMLDivElement>(null)

  const { logs, connected, paused, pause, resume, clear, filterLevel, setFilterLevel } =
    useAgentLogs({
      wsUrl,
      maxLogs,
    })

  // 自动滚动到最新日志
  useEffect(() => {
    if (scrollRef.current && !paused) {
      scrollRef.current.scrollTop = scrollRef.current.scrollHeight
    }
  }, [logs, paused])

  /**
   * 获取日志级别对应的颜色
   */
  const getLevelColor = (level: LogLevel): string => {
    switch (level) {
      case LogLevel.INFO:
        return 'bg-blue-500'
      case LogLevel.WARN:
        return 'bg-yellow-500'
      case LogLevel.ERROR:
        return 'bg-red-500'
      case LogLevel.DEBUG:
        return 'bg-gray-500'
      default:
        return 'bg-gray-500'
    }
  }

  /**
   * 格式化时间戳
   */
  const formatTimestamp = (timestamp: number): string => {
    const date = new Date(timestamp)
    return date.toLocaleTimeString('zh-CN', {
      hour12: false,
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
    } as any)
  }

  return (
    <div className="w-full h-full flex flex-col border rounded-lg bg-background">
      {/* 控制栏 */}
      {showControls && (
        <div className="flex items-center justify-between p-2 border-b gap-2">
          <div className="flex items-center gap-2">
            <Badge variant={connected ? 'default' : 'secondary'}>
              {connected ? '🟢 已连接' : '⚪ 未连接'}
            </Badge>

            <Select
              value={filterLevel || 'all'}
              onValueChange={value => setFilterLevel(value === 'all' ? null : (value as LogLevel))}
            >
              <SelectTrigger className="w-[120px]">
                <SelectValue placeholder="所有级别" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="all">所有级别</SelectItem>
                <SelectItem value="info">INFO</SelectItem>
                <SelectItem value="warn">WARN</SelectItem>
                <SelectItem value="error">ERROR</SelectItem>
                <SelectItem value="debug">DEBUG</SelectItem>
              </SelectContent>
            </Select>
          </div>

          <div className="flex items-center gap-2">
            <span className="text-sm text-muted-foreground">{logs.length} 条日志</span>

            {paused ? (
              <Button size="sm" onClick={resume} variant="outline">
                ▶️ 恢复
              </Button>
            ) : (
              <Button size="sm" onClick={pause} variant="outline">
                ⏸️ 暂停
              </Button>
            )}

            <Button size="sm" onClick={clear} variant="ghost">
              🗑️ 清空
            </Button>
          </div>
        </div>
      )}

      {/* 日志列表 */}
      <ScrollArea ref={scrollRef} className="flex-1 p-4 font-mono text-sm">
        <div className="space-y-1">
          {logs.length === 0 ? (
            <div className="text-muted-foreground text-center py-8">
              {paused ? '⏸️ 已暂停' : connected ? '等待日志...' : '正在连接...'}
            </div>
          ) : (
            logs.map(log => (
              <div
                key={log.id}
                className="flex items-start gap-2 hover:bg-muted/50 px-2 py-1 rounded"
              >
                <span className="text-muted-foreground shrink-0">
                  {formatTimestamp(log.timestamp)}
                </span>

                <Badge
                  variant="secondary"
                  className={`${getLevelColor(log.level)} text-white shrink-0`}
                >
                  {log.level}
                </Badge>

                {log.source && (
                  <span className="text-muted-foreground shrink-0">[{log.source}]</span>
                )}

                <span className="break-all">{log.message}</span>
              </div>
            ))
          )}
        </div>
      </ScrollArea>
    </div>
  )
}
