/**
 * Agent Office Component (智能体统一办公室场景)
 *
 * 参考 Star-Office-UI 设计，将所有智能体整合到一个俯视角办公室中
 * 智能体根据状态在办公室的不同区域间自由移动
 *
 * @module components/vibe-coding/AgentOffice
 */

import React, { useState, useEffect } from 'react'
import { Card } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { ScrollArea } from '@/components/ui/scroll-area'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import {
  Play,
  Pause,
  Square,
  Terminal,
  Cpu,
  HardDrive,
  Activity,
  Clock,
  Calendar,
  Users,
  Star,
} from 'lucide-react'
import type { AgentInfo } from './CodingWorkspaceTypes'
import { PixelAvatar } from './PixelAvatar'
import { OfficeBackground } from './OfficeBackground'

/**
 * AgentOffice 组件 Props
 */
export interface AgentOfficeProps {
  agents: AgentInfo[]
  loading?: boolean
  _onStartAgent?: (agentId: string) => void // eslint-disable-line @typescript-eslint/no-unused-vars
  onPauseAgent?: (agentId: string) => void
  onResumeAgent?: (agentId: string) => void
  onStopAgent?: (agentId: string) => void
  _onRefresh?: () => void // eslint-disable-line @typescript-eslint/no-unused-vars
}

/**
 * 办公室区域定义
 */
type OfficeZone = 'work' | 'rest' | 'debug' | 'sync'

/**
 * 智能体在办公室中的精确位置 (基于 1200x800 画布)
 * Star-Office-UI 风格：角色在统一场景中自由移动
 */
interface AgentPosition {
  x: number // 绝对像素坐标
  y: number // 绝对像素坐标
  zone: OfficeZone
}

/**
 * 根据智能体状态和索引计算其在办公室中的位置
 * 参考 Star-Office-UI 的俯视角布局
 */
const getAgentPosition = (agent: AgentInfo, index: number, _totalAgents: number): AgentPosition => {
  // 同区域内多个智能体的偏移量 (避免重叠)
  const _offset = index * 80

  switch (agent.status) {
    case 'running':
      // 工作区 (左侧) - 坐在办公桌前
      return {
        zone: 'work',
        x: 100 + (index % 3) * 80,
        y: 200 + Math.floor(index / 3) * 100,
      }
    case 'idle':
    case 'stopped':
      // 休息区 (中间) - 沙发区域
      return {
        zone: 'rest',
        x: 480 + (index % 3) * 80,
        y: 150 + Math.floor(index / 3) * 100,
      }
    case 'paused':
      // 休息区 (中间) - 咖啡机旁
      return {
        zone: 'rest',
        x: 520 + (index % 3) * 80,
        y: 280 + Math.floor(index / 3) * 100,
      }
    case 'failed':
      // 调试区 (右上) - 服务器机柜旁
      return {
        zone: 'debug',
        x: 830 + (index % 3) * 80,
        y: 100 + Math.floor(index / 3) * 100,
      }
    case 'completed':
      // 同步区 (右下) - 文件柜旁
      return {
        zone: 'sync',
        x: 880 + (index % 3) * 80,
        y: 500 + Math.floor(index / 3) * 100,
      }
    default:
      return {
        zone: 'rest',
        x: 480,
        y: 150,
      }
  }
}

/**
 * 区域标签映射
 */
const ZONE_LABELS: Record<OfficeZone, { label: string; x: number; y: number }> = {
  work: { label: '🖥️ 工作区', x: 60, y: 120 },
  rest: { label: '☕ 休息区', x: 440, y: 80 },
  debug: { label: '🐛 调试区', x: 800, y: 30 },
  sync: { label: '💾 同步区', x: 800, y: 400 },
}

/**
 * 生成"昨日小记"内容
 */
const generateYesterdayNotes = (agents: AgentInfo[]): string[] => {
  const notes: string[] = []
  const completedAgents = agents.filter(a => a.status === 'completed')

  if (completedAgents.length > 0) {
    notes.push(`✅ ${completedAgents.length} 个智能体完成任务`)
  }

  const failedAgents = agents.filter(a => a.status === 'failed')
  if (failedAgents.length > 0) {
    notes.push(`⚠️ ${failedAgents.length} 个智能体遇到错误`)
  }

  const runningAgents = agents.filter(a => a.status === 'running')
  if (runningAgents.length > 0) {
    notes.push(`🔄 ${runningAgents.length} 个智能体仍在工作中`)
  }

  if (notes.length === 0) {
    notes.push('📝 暂无昨日记录')
  }

  return notes
}

/**
 * AgentOffice 组件实现 (Star-Office-UI 统一办公室风格)
 */
export function AgentOffice({
  agents,
  loading = false,
  onPauseAgent,
  onResumeAgent,
  onStopAgent,
}: AgentOfficeProps) {
  const [selectedAgent, setSelectedAgent] = useState<AgentInfo | null>(null)
  const [showLogDialog, setShowLogDialog] = useState(false)
  const [currentTime, setCurrentTime] = useState(new Date())
  const [hoveredAgent, setHoveredAgent] = useState<string | null>(null)

  // 更新时间 (每秒)
  useEffect(() => {
    const timer = setInterval(() => setCurrentTime(new Date()), 1000)
    return () => clearInterval(timer)
  }, [])

  // 计算每个智能体的位置
  const agentPositions = agents.map((agent, index) => ({
    ...agent,
    position: getAgentPosition(agent, index, agents.length),
  }))

  // 处理角色点击
  const handleAvatarClick = (agent: AgentInfo) => {
    setSelectedAgent(agent)
    setShowLogDialog(true)
  }

  // 生成昨日小记
  const yesterdayNotes = generateYesterdayNotes(agents)

  if (loading) {
    return (
      <Card className="p-8">
        <div className="flex items-center justify-center space-x-2">
          <Activity className="w-6 h-6 animate-spin text-muted-foreground" />
          <span className="text-muted-foreground">正在布置办公室...</span>
        </div>
      </Card>
    )
  }

  if (agents.length === 0) {
    return (
      <Card className="p-12">
        <div className="flex flex-col items-center justify-center text-center space-y-4">
          <div className="text-6xl">🏢</div>
          <div>
            <h3 className="text-lg font-semibold mb-2">办公室空空如也</h3>
            <p className="text-muted-foreground mb-4">
              还没有智能体在工作，点击"创建 Agent"邀请新同事加入吧！
            </p>
          </div>
        </div>
      </Card>
    )
  }

  return (
    <div className="space-y-4">
      {/* 顶部信息栏 (Star-Office-UI 风格) */}
      <Card className="p-4 bg-gradient-to-r from-blue-50 via-purple-50 to-pink-50 dark:from-gray-900 dark:via-gray-800 dark:to-gray-900">
        <div className="flex items-center justify-between flex-wrap gap-4">
          {/* 左侧：时间和统计 */}
          <div className="flex items-center gap-6">
            <div className="flex items-center gap-2">
              <Clock className="w-5 h-5 text-blue-500" />
              <div>
                <div className="text-sm font-bold">
                  {currentTime.toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit' })}
                </div>
                <div className="text-xs text-muted-foreground">
                  {currentTime.toLocaleDateString('zh-CN', {
                    month: 'long',
                    day: 'numeric',
                    weekday: 'long',
                  })}
                </div>
              </div>
            </div>

            <div className="flex items-center gap-2">
              <Users className="w-5 h-5 text-purple-500" />
              <div>
                <div className="text-sm font-bold">{agents.length} 个智能体</div>
                <div className="text-xs text-muted-foreground">
                  {agents.filter(a => a.status === 'running').length} 工作中
                </div>
              </div>
            </div>
          </div>

          {/* 右侧：办公室名称和昨日小记 */}
          <div className="flex items-center gap-4">
            <div className="flex items-center gap-2 bg-amber-100 dark:bg-amber-900/30 px-4 py-2 rounded-lg">
              <Star className="w-4 h-4 text-amber-500 fill-amber-500" />
              <span className="text-sm font-bold text-amber-800 dark:text-amber-200">
                Vibe Coding 智能体办公室
              </span>
              <Star className="w-4 h-4 text-amber-500 fill-amber-500" />
            </div>

            <div className="flex items-center gap-2 bg-white/50 dark:bg-gray-800/50 backdrop-blur-sm rounded-lg px-4 py-2">
              <Calendar className="w-4 h-4 text-orange-500" />
              <div className="text-xs">
                <div className="font-bold mb-1">📝 昨日小记</div>
                <div className="text-muted-foreground space-y-0.5">
                  {yesterdayNotes.slice(0, 2).map((note, idx) => (
                    <div key={idx}>{note}</div>
                  ))}
                </div>
              </div>
            </div>
          </div>
        </div>
      </Card>

      {/* 统一办公室场景 (Star-Office-UI 风格) */}
      <Card className="relative overflow-hidden bg-gray-900">
        {/* 办公室背景画布 */}
        <div className="relative w-full" style={{ aspectRatio: '1200/800' }}>
          <OfficeBackground width={1200} height={800} />

          {/* 区域标签 (悬浮在背景上) */}
          {Object.entries(ZONE_LABELS).map(([zone, config]) => (
            <div
              key={zone}
              className="absolute bg-white/90 dark:bg-gray-800/90 backdrop-blur-sm px-3 py-1.5 rounded-full shadow-lg border-2 border-gray-300 dark:border-gray-600"
              style={{
                left: `${(config.x / 1200) * 100}%`,
                top: `${(config.y / 800) * 100}%`,
                transform: 'translate(-50%, -50%)',
              }}
            >
              <span className="text-xs font-bold">{config.label}</span>
            </div>
          ))}

          {/* 智能体角色 (绝对定位在办公室中) */}
          {agentPositions.map(agent => (
            <div
              key={agent.agentId}
              className="absolute transition-all duration-700 ease-in-out group cursor-pointer"
              style={{
                left: `${(agent.position.x / 1200) * 100}%`,
                top: `${(agent.position.y / 800) * 100}%`,
                transform: 'translate(-50%, -50%)',
                zIndex: hoveredAgent === agent.agentId ? 50 : 10,
              }}
              onClick={() => handleAvatarClick(agent)}
              onMouseEnter={() => setHoveredAgent(agent.agentId)}
              onMouseLeave={() => setHoveredAgent(null)}
            >
              {/* 像素角色 */}
              <div className="relative">
                <PixelAvatar
                  agent={agent}
                  size={64}
                  showBubble={true}
                  className="transition-transform hover:scale-110 drop-shadow-2xl"
                />

                {/* 悬停信息卡片 */}
                {hoveredAgent === agent.agentId && (
                  <div className="absolute bottom-full left-1/2 transform -translate-x-1/2 mb-3 pointer-events-none z-50">
                    <div className="bg-white dark:bg-gray-800 border-2 border-gray-300 dark:border-gray-600 rounded-lg p-3 shadow-2xl min-w-[200px]">
                      <div className="font-bold text-sm mb-1">{agent.name || agent.type}</div>
                      <div className="text-xs text-muted-foreground mb-2 line-clamp-2">
                        {agent.currentTask}
                      </div>
                      <div className="flex items-center justify-between text-xs mb-1">
                        <span>进度:</span>
                        <span className="font-bold">{Math.round(agent.progress)}%</span>
                      </div>
                      <div className="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
                        <div
                          className="bg-blue-500 h-2 rounded-full transition-all"
                          style={{ width: `${agent.progress}%` }}
                        />
                      </div>
                      <div className="mt-2 text-xs text-gray-500">
                        区域: {ZONE_LABELS[agent.position.zone].label}
                      </div>
                    </div>
                  </div>
                )}
              </div>

              {/* 快速操作按钮 (悬停显示) */}
              <div className="absolute -bottom-10 left-1/2 transform -translate-x-1/2 flex gap-1 opacity-0 group-hover:opacity-100 transition-opacity bg-white dark:bg-gray-800 rounded-lg shadow-xl p-1.5 border-2 border-gray-300 dark:border-gray-600 z-50">
                {agent.status === 'running' && onPauseAgent && (
                  <Button
                    size="sm"
                    variant="ghost"
                    className="h-7 w-7 p-0 hover:bg-yellow-100 dark:hover:bg-yellow-900"
                    onClick={e => {
                      e.stopPropagation()
                      onPauseAgent(agent.agentId)
                    }}
                    title="暂停"
                  >
                    <Pause className="w-3.5 h-3.5 text-yellow-600" />
                  </Button>
                )}
                {agent.status === 'paused' && onResumeAgent && (
                  <Button
                    size="sm"
                    variant="ghost"
                    className="h-7 w-7 p-0 hover:bg-green-100 dark:hover:bg-green-900"
                    onClick={e => {
                      e.stopPropagation()
                      onResumeAgent(agent.agentId)
                    }}
                    title="恢复"
                  >
                    <Play className="w-3.5 h-3.5 text-green-600" />
                  </Button>
                )}
                {(agent.status === 'running' || agent.status === 'paused') && onStopAgent && (
                  <Button
                    size="sm"
                    variant="ghost"
                    className="h-7 w-7 p-0 hover:bg-red-100 dark:hover:bg-red-900"
                    onClick={e => {
                      e.stopPropagation()
                      onStopAgent(agent.agentId)
                    }}
                    title="停止"
                  >
                    <Square className="w-3.5 h-3.5 text-red-600" />
                  </Button>
                )}
              </div>
            </div>
          ))}
        </div>
      </Card>

      {/* 详细日志对话框 */}
      <Dialog
        open={showLogDialog}
        onOpenChange={(open: boolean) => {
          if (!open) {
            setShowLogDialog(false)
            setSelectedAgent(null)
          }
        }}
      >
        <DialogContent className="max-w-4xl max-h-[80vh]">
          {selectedAgent && (
            <>
              <DialogHeader>
                <DialogTitle className="flex items-center gap-3">
                  <PixelAvatar agent={selectedAgent} size={48} showBubble={false} />
                  <div>
                    <div className="flex items-center gap-2">
                      <h3 className="text-lg font-bold">
                        {selectedAgent.name || selectedAgent.type}
                      </h3>
                      <Badge variant={selectedAgent.status === 'running' ? 'default' : 'secondary'}>
                        {selectedAgent.status}
                      </Badge>
                    </div>
                    <p className="text-sm text-muted-foreground font-normal mt-1">
                      {selectedAgent.currentTask}
                    </p>
                  </div>
                </DialogTitle>
                <DialogDescription>查看智能体的详细运行日志和状态信息</DialogDescription>
              </DialogHeader>

              <Tabs defaultValue="logs" className="w-full">
                <TabsList className="grid w-full grid-cols-3 mb-4">
                  <TabsTrigger value="logs">
                    <Terminal className="w-4 h-4 mr-2" />
                    运行日志
                  </TabsTrigger>
                  <TabsTrigger value="resources">
                    <Cpu className="w-4 h-4 mr-2" />
                    资源使用
                  </TabsTrigger>
                  <TabsTrigger value="info">
                    <Activity className="w-4 h-4 mr-2" />
                    基本信息
                  </TabsTrigger>
                </TabsList>

                <TabsContent value="logs">
                  <ScrollArea className="h-[300px] w-full rounded-md border p-4 bg-gray-50 dark:bg-gray-900">
                    <div className="space-y-2 font-mono text-xs">
                      {selectedAgent.logs && selectedAgent.logs.length > 0 ? (
                        selectedAgent.logs.map((log, index) => (
                          <div key={index} className="text-gray-700 dark:text-gray-300">
                            <span className="text-gray-400 mr-2">
                              [{String(index + 1).padStart(3, '0')}]
                            </span>
                            {log}
                          </div>
                        ))
                      ) : (
                        <div className="text-center text-gray-400 py-8">暂无日志记录</div>
                      )}
                    </div>
                  </ScrollArea>
                </TabsContent>

                <TabsContent value="resources">
                  <div className="space-y-4 p-4">
                    <div>
                      <div className="flex items-center justify-between mb-2">
                        <div className="flex items-center gap-2">
                          <Cpu className="w-4 h-4 text-blue-500" />
                          <span className="text-sm">CPU 使用率</span>
                        </div>
                        <span className="text-sm font-bold">
                          {selectedAgent.cpuUsage.toFixed(1)}%
                        </span>
                      </div>
                      <div className="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
                        <div
                          className="bg-blue-500 h-2 rounded-full transition-all"
                          style={{ width: `${Math.min(selectedAgent.cpuUsage, 100)}%` }}
                        />
                      </div>
                    </div>

                    <div>
                      <div className="flex items-center justify-between mb-2">
                        <div className="flex items-center gap-2">
                          <HardDrive className="w-4 h-4 text-green-500" />
                          <span className="text-sm">内存使用</span>
                        </div>
                        <span className="text-sm font-bold">
                          {(selectedAgent.memoryUsage / 1024).toFixed(1)} GB
                        </span>
                      </div>
                      <div className="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
                        <div
                          className="bg-green-500 h-2 rounded-full transition-all"
                          style={{
                            width: `${Math.min((selectedAgent.memoryUsage / 1024 / 16) * 100, 100)}%`,
                          }}
                        />
                      </div>
                    </div>

                    <div>
                      <div className="flex items-center justify-between mb-2">
                        <span className="text-sm">任务进度</span>
                        <span className="text-sm font-bold">
                          {Math.round(selectedAgent.progress)}%
                        </span>
                      </div>
                      <div className="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-3">
                        <div
                          className="bg-gradient-to-r from-blue-500 to-purple-500 h-3 rounded-full transition-all"
                          style={{ width: `${selectedAgent.progress}%` }}
                        />
                      </div>
                    </div>
                  </div>
                </TabsContent>

                <TabsContent value="info">
                  <div className="grid grid-cols-2 gap-4 p-4">
                    <div>
                      <div className="text-xs text-muted-foreground mb-1">智能体 ID</div>
                      <div className="font-mono text-xs bg-gray-100 dark:bg-gray-800 p-2 rounded">
                        {selectedAgent.agentId}
                      </div>
                    </div>
                    <div>
                      <div className="text-xs text-muted-foreground mb-1">类型</div>
                      <div className="text-sm capitalize">{selectedAgent.type}</div>
                    </div>
                    <div>
                      <div className="text-xs text-muted-foreground mb-1">会话 ID</div>
                      <div className="font-mono text-xs bg-gray-100 dark:bg-gray-800 p-2 rounded">
                        {selectedAgent.sessionId || 'N/A'}
                      </div>
                    </div>
                    <div>
                      <div className="text-xs text-muted-foreground mb-1">状态</div>
                      <div className="text-sm">{selectedAgent.status}</div>
                    </div>
                  </div>
                </TabsContent>
              </Tabs>
            </>
          )}
        </DialogContent>
      </Dialog>
    </div>
  )
}
