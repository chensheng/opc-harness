/**
 * Agent 追踪视图组件
 *
 * 展示智能体的思考链、工具调用和执行轨迹
 * 支持树形结构展示、时间线视图和回放功能
 */

import { useState, useEffect, useRef } from 'react'
// import { useParams } from 'react-router-dom'
// import { invoke } from '@tauri-apps/api/core'
import {
  Brain,
  Wrench,
  CheckCircle,
  GitCommit,
  Clock,
  Play,
  Pause,
  FastForward,
  StepForward,
  Filter,
} from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Tabs, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { Checkbox } from '@/components/ui/checkbox'
import type { TraceEventType, TraceTreeNode } from '@/types/agentObservability'
import { useObservabilityStore } from '@/stores/observabilityStore'

interface AgentTracingProps {
  agentId?: string
}

export function AgentTracing({ agentId }: AgentTracingProps) {
  // const { projectId } = useParams<{ projectId: string }>()

  const [viewMode, setViewMode] = useState<'tree' | 'timeline'>('tree')
  const [isPlaying, setIsPlaying] = useState(false)
  const [playbackSpeed, setPlaybackSpeed] = useState(1)
  const [selectedEventTypes, setSelectedEventTypes] = useState<TraceEventType[]>([
    'thought',
    'tool_call',
    'tool_result',
    'decision',
  ])

  const traces = useObservabilityStore(state => state.getTraces(agentId || 'default'))
  const traceTree = useObservabilityStore(state => state.getTraceTree(agentId || 'default'))

  // 回放状态
  const [_currentEventIndex, setCurrentEventIndex] = useState(0)
  const intervalRef = useRef<NodeJS.Timeout | null>(null)

  // 过滤事件类型
  const filteredTraces = traces.filter(trace => selectedEventTypes.includes(trace.eventType))

  // 回放逻辑
  useEffect(() => {
    if (isPlaying && filteredTraces.length > 0) {
      const interval = setInterval(() => {
        setCurrentEventIndex(prev => {
          if (prev >= filteredTraces.length - 1) {
            setIsPlaying(false)
            return 0
          }
          return prev + 1
        })
      }, 1000 / playbackSpeed)

      intervalRef.current = interval

      return () => {
        if (intervalRef.current) {
          clearInterval(intervalRef.current)
        }
      }
    }
  }, [isPlaying, playbackSpeed, filteredTraces.length])

  // 获取事件类型图标
  const getEventTypeIcon = (eventType: TraceEventType) => {
    switch (eventType) {
      case 'thought':
        return <Brain className="w-4 h-4" />
      case 'tool_call':
        return <Wrench className="w-4 h-4" />
      case 'tool_result':
        return <CheckCircle className="w-4 h-4" />
      case 'decision':
        return <GitCommit className="w-4 h-4" />
      default:
        return <Brain className="w-4 h-4" />
    }
  }

  // 获取事件类型标签
  const getEventTypeLabel = (eventType: TraceEventType) => {
    switch (eventType) {
      case 'thought':
        return '思考'
      case 'tool_call':
        return '工具调用'
      case 'tool_result':
        return '工具结果'
      case 'decision':
        return '决策'
      default:
        return eventType
    }
  }

  // 渲染树形节点
  const renderTreeNode = (node: TraceTreeNode, _index: number) => {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const data = node.data as any

    return (
      <div key={node.id} style={{ marginLeft: node.depth * 24 }}>
        <Card className="p-3 mb-2">
          <div className="flex items-start gap-3">
            <div className="p-2 bg-muted rounded-lg">{getEventTypeIcon(node.eventType)}</div>
            <div className="flex-1">
              <div className="flex items-center gap-2 mb-1">
                <Badge variant="secondary">{getEventTypeLabel(node.eventType)}</Badge>
                <span className="text-xs text-muted-foreground">
                  {node.timestamp.toLocaleTimeString()}
                </span>
              </div>

              {node.eventType === 'thought' && data.content && (
                <p className="text-sm">{data.content}</p>
              )}

              {node.eventType === 'tool_call' && (
                <div className="text-sm">
                  <p className="font-medium">工具：{data.toolName}</p>
                  {data.parameters && (
                    <pre className="text-xs bg-muted p-2 rounded mt-1 overflow-auto">
                      {JSON.stringify(data.parameters, null, 2)}
                    </pre>
                  )}
                </div>
              )}

              {node.eventType === 'tool_result' && (
                <div className="text-sm">
                  <Badge variant={data.success ? 'default' : 'destructive'}>
                    {data.success ? '成功' : '失败'}
                  </Badge>
                  {data.durationMs && (
                    <span className="text-xs text-muted-foreground ml-2">
                      耗时：{data.durationMs}ms
                    </span>
                  )}
                  {data.result && (
                    <pre className="text-xs bg-muted p-2 rounded mt-1 overflow-auto">
                      {JSON.stringify(data.result, null, 2)}
                    </pre>
                  )}
                </div>
              )}

              {node.eventType === 'decision' && (
                <div className="text-sm">
                  <p className="font-medium">决策：{data.decision}</p>
                  {data.context && (
                    <p className="text-xs text-muted-foreground mt-1">上下文：{data.context}</p>
                  )}
                  {data.reason && (
                    <p className="text-xs text-muted-foreground mt-1">原因：{data.reason}</p>
                  )}
                </div>
              )}
            </div>
          </div>
        </Card>

        {/* 递归渲染子节点 */}
        {node.children.map((child, childIndex) => renderTreeNode(child, _index * 100 + childIndex))}
      </div>
    )
  }

  // 渲染时间线视图
  const renderTimeline = () => {
    return (
      <div className="space-y-2">
        {filteredTraces.map((trace, _index) => {
          // eslint-disable-next-line @typescript-eslint/no-explicit-any
          const data = trace.data as any

          return (
            <Card key={trace.id} className="p-3">
              <div className="flex items-center gap-3">
                <div className="flex items-center justify-center w-8 h-8 rounded-full bg-muted">
                  {getEventTypeIcon(trace.eventType)}
                </div>
                <div className="flex-1">
                  <div className="flex items-center gap-2 mb-1">
                    <Badge variant="secondary">{getEventTypeLabel(trace.eventType)}</Badge>
                    <span className="text-xs text-muted-foreground">
                      {trace.timestamp.toLocaleTimeString()}
                    </span>
                  </div>

                  {trace.eventType === 'thought' && data.content && (
                    <p className="text-sm">{data.content}</p>
                  )}

                  {trace.eventType === 'tool_call' && (
                    <p className="text-sm">
                      调用工具：<span className="font-medium">{data.toolName}</span>
                    </p>
                  )}

                  {trace.eventType === 'tool_result' && (
                    <p className="text-sm">
                      <Badge variant={data.success ? 'default' : 'destructive'}>
                        {data.success ? '成功' : '失败'}
                      </Badge>
                      {data.durationMs && (
                        <span className="text-xs text-muted-foreground ml-2">
                          {data.durationMs}ms
                        </span>
                      )}
                    </p>
                  )}

                  {trace.eventType === 'decision' && (
                    <p className="text-sm">
                      决策：<span className="font-medium">{data.decision}</span>
                    </p>
                  )}
                </div>
              </div>
            </Card>
          )
        })}
      </div>
    )
  }

  // 事件类型过滤选项
  const eventTypeOptions: { value: TraceEventType; label: string }[] = [
    { value: 'thought', label: '思考' },
    { value: 'tool_call', label: '工具调用' },
    { value: 'tool_result', label: '工具结果' },
    { value: 'decision', label: '决策' },
  ]

  const toggleEventType = (eventType: TraceEventType) => {
    setSelectedEventTypes(prev =>
      prev.includes(eventType) ? prev.filter(t => t !== eventType) : [...prev, eventType]
    )
  }

  return (
    <Card className="p-4">
      <div className="flex items-center justify-between mb-4">
        <h2 className="text-lg font-semibold flex items-center gap-2">
          <Brain className="w-5 h-5" />
          智能体追踪
        </h2>

        <div className="flex items-center gap-2">
          <Tabs value={viewMode} onValueChange={v => setViewMode(v as 'tree' | 'timeline')}>
            <TabsList>
              <TabsTrigger value="tree">树形视图</TabsTrigger>
              <TabsTrigger value="timeline">时间线</TabsTrigger>
            </TabsList>
          </Tabs>
        </div>
      </div>

      {/* 事件类型过滤 */}
      <div className="flex items-center gap-4 mb-4 p-3 bg-muted rounded-lg">
        <Filter className="w-4 h-4" />
        <span className="text-sm font-medium">事件类型:</span>
        {eventTypeOptions.map(option => (
          <label key={option.value} className="flex items-center gap-2 text-sm">
            <Checkbox
              checked={selectedEventTypes.includes(option.value)}
              onChange={() => toggleEventType(option.value)}
            />
            {option.label}
          </label>
        ))}
      </div>

      {/* 回放控制（预留） */}
      <div className="flex items-center justify-center gap-2 mb-4 p-3 bg-muted rounded-lg">
        <Button
          variant="ghost"
          size="sm"
          onClick={() => setIsPlaying(!isPlaying)}
          disabled={traces.length === 0}
        >
          {isPlaying ? <Pause className="w-4 h-4" /> : <Play className="w-4 h-4" />}
        </Button>
        <Button
          variant="ghost"
          size="sm"
          onClick={() => setPlaybackSpeed(2)}
          disabled={playbackSpeed === 2}
        >
          <FastForward className="w-4 h-4" /> 2x
        </Button>
        <Button
          variant="ghost"
          size="sm"
          onClick={() => setPlaybackSpeed(1)}
          disabled={playbackSpeed === 1}
        >
          1x
        </Button>
        <Button variant="ghost" size="sm" disabled>
          <StepForward className="w-4 h-4" />
        </Button>
      </div>

      {/* 统计信息 */}
      <div className="flex items-center gap-4 mb-4 text-sm text-muted-foreground">
        <span className="flex items-center gap-1">
          <Clock className="w-4 h-4" />
          总事件数：{traces.length}
        </span>
        <span>思考：{traces.filter(t => t.eventType === 'thought').length}</span>
        <span>工具调用：{traces.filter(t => t.eventType === 'tool_call').length}</span>
        <span>决策：{traces.filter(t => t.eventType === 'decision').length}</span>
      </div>

      {/* 内容区域 */}
      <div className="max-h-[600px] overflow-auto">
        {viewMode === 'tree' ? (
          <div className="space-y-1">
            {traceTree.length === 0 ? (
              <div className="text-center py-8 text-muted-foreground">暂无追踪数据</div>
            ) : (
              traceTree.map((node, index) => renderTreeNode(node, index))
            )}
          </div>
        ) : filteredTraces.length === 0 ? (
          <div className="text-center py-8 text-muted-foreground">暂无匹配的事件记录</div>
        ) : (
          renderTimeline()
        )}
      </div>
    </Card>
  )
}
