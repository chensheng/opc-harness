import { useState, useEffect } from 'react'
import { useParams } from 'react-router-dom'
import { invoke } from '@tauri-apps/api/core'
import {
  Play,
  Square,
  RefreshCw,
  EyeIcon,
  Activity,
  Loader2,
  Cpu,
  HardDrive,
  Pause,
  FileText,
  Terminal,
  GitBranch,
  Plus,
  Trash2,
} from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import type { AgentInfo } from './CodingWorkspaceTypes'
import { CreateAgentDialog } from './CreateAgentDialog'

// 后端 AgentSession 类型定义
interface AgentSession {
  // 驼峰命名（实际返回的格式）
  sessionId?: string
  agentId?: string
  agentType?: string
  projectId?: string
  name?: string
  createdAt?: string
  updatedAt?: string
  stdioChannelId?: string
  registeredToDaemon?: boolean

  session_id: string
  agent_id: string
  agent_type: string
  project_id: string
  status: string
  phase: string
  created_at: string
  updated_at: string
  stdio_channel_id?: string
  registered_to_daemon: boolean
  metadata?: string
}

// 将后端 AgentSession 转换为前端 AgentInfo
const convertSessionToAgentInfo = (session: AgentSession): AgentInfo => {
  // 映射 agent_type
  const typeMap: Record<string, AgentInfo['type']> = {
    initializer: 'initializer',
    coding: 'coding',
    mr_creation: 'mr_creation',
  }

  // 映射 status
  const statusMap: Record<string, AgentInfo['status']> = {
    idle: 'idle',
    running: 'running',
    paused: 'paused',
    completed: 'completed',
    failed: 'failed',
    stopped: 'stopped',
  }

  // 解析 metadata 获取额外信息（如果有）
  let currentTask = '等待启动'
  let progress = 0
  let logs: string[] = ['智能体已创建']

  if (session.metadata) {
    try {
      const metadata = JSON.parse(session.metadata)
      currentTask = metadata.current_task || currentTask
      progress = metadata.progress || progress
      logs = metadata.logs || logs
    } catch {
      // 忽略解析错误
    }
  }

  return {
    agentId: session.agentId || session.agent_id || '',
    type: typeMap[session.agentType || session.agent_type] || 'coding',
    name: session.name,
    status: statusMap[session.status || 'idle'] || 'idle',
    currentTask,
    progress,
    cpuUsage: 0, // 需要从实时监控获取
    memoryUsage: 0, // 需要从实时监控获取
    logs,
    sessionId: session.sessionId || session.session_id || '',
  }
}

export function AgentMonitor() {
  const { projectId } = useParams<{ projectId: string }>()
  const [agents, setAgents] = useState<AgentInfo[]>([])
  const [loading, setLoading] = useState(true)
  const [selectedAgent, setSelectedAgent] = useState<string | null>(null)
  const [isAutoScroll, setIsAutoScroll] = useState(true)
  const [showCreateDialog, setShowCreateDialog] = useState(false)
  const [agentToDelete, setAgentToDelete] = useState<string | null>(null)

  // 监听 agentToDelete 状态变化
  useEffect(() => {}, [agentToDelete])

  // 从数据库加载智能体列表
  const loadAgents = async () => {
    if (!projectId) {
      console.warn('[AgentMonitor] Project ID is not available')
      setLoading(false)
      return
    }

    try {
      setLoading(true)
      const sessions = await invoke<AgentSession[]>('get_sessions_by_project', {
        projectId,
      })

      const agentInfos = sessions.map(convertSessionToAgentInfo)

      setAgents(agentInfos)
    } catch (error) {
      console.error('[AgentMonitor] Failed to load agents:', error)
      // 失败时显示空列表，不阻塞用户操作
      setAgents([])
    } finally {
      setLoading(false)
    }
  }

  // 组件挂载时加载数据
  useEffect(() => {
    loadAgents()
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [projectId])

  // 定时刷新数据（每5秒）
  useEffect(() => {
    const interval = setInterval(() => {
      loadAgents()
    }, 5000)

    return () => clearInterval(interval)
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [projectId])

  const handlePauseAgent = (agentId: string) => {
    setAgents(prev =>
      prev.map(a => (a.agentId === agentId ? { ...a, status: 'paused', cpuUsage: 0 } : a))
    )
  }

  const handleResumeAgent = (agentId: string) => {
    setAgents(prev => prev.map(a => (a.agentId === agentId ? { ...a, status: 'running' } : a)))
  }

  const handleStopAgent = (agentId: string) => {
    setAgents(prev =>
      prev.map(a =>
        a.agentId === agentId ? { ...a, status: 'stopped', progress: 0, cpuUsage: 0 } : a
      )
    )
  }

  const handleDeleteAgent = async (agentId: string) => {
    try {
      await invoke('delete_agent_session', { agentId })
      // 从列表中移除已删除的智能体
      setAgents(prev => {
        const newAgents = prev.filter(a => a.agentId !== agentId)
        console.log(
          '[AgentMonitor] Updated agents list, removed agent:',
          agentId,
          'remaining:',
          newAgents.length
        )
        return newAgents
      })
    } catch (error) {
      console.error('[AgentMonitor] Failed to delete agent:', error)
      alert(`删除智能体失败: ${error}`)
    } finally {
      setAgentToDelete(null)
    }
  }

  const handleAgentCreated = (agentId: string) => {
    console.log('智能体创建成功:', agentId)
    // 重新加载智能体列表
    loadAgents()
  }

  const handleRefresh = () => {
    loadAgents()
  }

  const getStatusColor = (status: AgentInfo['status']) => {
    switch (status) {
      case 'running':
        return 'bg-green-500'
      case 'paused':
        return 'bg-yellow-500'
      case 'completed':
        return 'bg-blue-500'
      case 'failed':
        return 'bg-red-500'
      default:
        return 'bg-gray-500'
    }
  }

  const getStatusBadge = (status: AgentInfo['status']) => {
    const variants = {
      running: 'default',
      paused: 'secondary',
      completed: 'outline',
      failed: 'destructive',
      stopped: 'secondary',
      idle: 'secondary',
    } as const

    const labels = {
      running: '运行中',
      paused: '已暂停',
      completed: '已完成',
      failed: '失败',
      stopped: '已停止',
      idle: '空闲',
    } as const

    return <Badge variant={variants[status]}>{labels[status]}</Badge>
  }

  const getAgentTypeIcon = (type: AgentInfo['type']) => {
    switch (type) {
      case 'initializer':
        return <FileText className="w-4 h-4" />
      case 'coding':
        return <Terminal className="w-4 h-4" />
      case 'mr_creation':
        return <GitBranch className="w-4 h-4" />
    }
  }

  const totalAgents = agents.length
  const runningAgents = agents.filter(a => a.status === 'running').length
  const avgProgress = agents.reduce((sum, a) => sum + a.progress, 0) / totalAgents
  const totalCpuUsage = agents.reduce((sum, a) => sum + a.cpuUsage, 0)
  const totalMemoryUsage = agents.reduce((sum, a) => sum + a.memoryUsage, 0)

  return (
    <div className="h-full flex flex-col p-6 space-y-6 overflow-auto">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-2xl font-bold">Agent 监控面板</h2>
          <p className="text-muted-foreground mt-1">实时监控多个 AI Agent 的运行状态和资源使用</p>
        </div>
        <div className="flex gap-2">
          <Button variant="outline" onClick={() => setIsAutoScroll(!isAutoScroll)}>
            <EyeIcon className="w-4 h-4 mr-2" />
            {isAutoScroll ? '自动滚动：开' : '自动滚动：关'}
          </Button>
          <Button variant="outline" onClick={handleRefresh} disabled={loading}>
            <RefreshCw className={`w-4 h-4 mr-2 ${loading ? 'animate-spin' : ''}`} />
            {loading ? '加载中...' : '刷新状态'}
          </Button>
          <Button onClick={() => setShowCreateDialog(true)}>
            <Plus className="w-4 h-4 mr-2" />
            创建 Agent
          </Button>
        </div>
      </div>

      {/* Statistics Cards */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        <Card className="p-4">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm text-muted-foreground">总 Agent 数</p>
              <p className="text-2xl font-bold">{totalAgents}</p>
            </div>
            <Activity className="w-8 h-8 text-blue-500" />
          </div>
        </Card>

        <Card className="p-4">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm text-muted-foreground">运行中</p>
              <p className="text-2xl font-bold text-green-600">{runningAgents}</p>
            </div>
            <Play className="w-8 h-8 text-green-500" />
          </div>
        </Card>

        <Card className="p-4">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm text-muted-foreground">平均进度</p>
              <p className="text-2xl font-bold">{Math.round(avgProgress)}%</p>
            </div>
            <Loader2 className="w-8 h-8 text-orange-500" />
          </div>
        </Card>

        <Card className="p-4">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm text-muted-foreground">CPU 使用率</p>
              <p className="text-2xl font-bold">{totalCpuUsage.toFixed(1)}%</p>
            </div>
            <Cpu className="w-8 h-8 text-purple-500" />
          </div>
          <div className="mt-2 text-xs text-muted-foreground">
            内存：{(totalMemoryUsage / 1024).toFixed(1)} GB
          </div>
        </Card>
      </div>

      {/* Agent List */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
        {loading ? (
          <div className="col-span-full flex items-center justify-center py-12">
            <Loader2 className="w-8 h-8 animate-spin text-muted-foreground" />
            <span className="ml-2 text-muted-foreground">加载中...</span>
          </div>
        ) : agents.length === 0 ? (
          <div className="col-span-full flex flex-col items-center justify-center py-12 text-center">
            <Activity className="w-12 h-12 text-muted-foreground mb-4" />
            <h3 className="text-lg font-semibold mb-2">暂无智能体</h3>
            <p className="text-muted-foreground mb-4">点击"创建 Agent"按钮开始创建新的智能体</p>
            <Button onClick={() => setShowCreateDialog(true)}>
              <Plus className="w-4 h-4 mr-2" />
              创建 Agent
            </Button>
          </div>
        ) : (
          agents.map(agent => (
            <Card
              key={agent.agentId}
              className={`border-l-4 ${
                agent.status === 'running'
                  ? 'border-l-green-500'
                  : agent.status === 'paused'
                    ? 'border-l-yellow-500'
                    : agent.status === 'completed'
                      ? 'border-l-blue-500'
                      : 'border-l-gray-500'
              }`}
            >
              <div className="p-4 space-y-3">
                {/* Header */}
                <div className="flex items-center justify-between">
                  <div className="flex items-center gap-3">
                    <div className={`w-3 h-3 rounded-full ${getStatusColor(agent.status)}`} />
                    <div className="flex items-center gap-2">
                      {getAgentTypeIcon(agent.type)}
                      <div className="flex flex-col">
                        <span className="font-semibold">{agent.name || agent.agentId}</span>
                        {agent.name && (
                          <span className="text-xs text-muted-foreground font-mono">
                            {agent.agentId}
                          </span>
                        )}
                      </div>
                      {getStatusBadge(agent.status)}
                    </div>
                  </div>
                  <div className="flex gap-2">
                    {agent.status === 'running' && (
                      <Button
                        size="sm"
                        variant="outline"
                        onClick={() => handlePauseAgent(agent.agentId)}
                      >
                        <Pause className="w-3 h-3" />
                      </Button>
                    )}
                    {agent.status === 'paused' && (
                      <Button
                        size="sm"
                        variant="outline"
                        onClick={() => handleResumeAgent(agent.agentId)}
                      >
                        <Play className="w-3 h-3" />
                      </Button>
                    )}
                    <Button
                      size="sm"
                      variant="outline"
                      onClick={() => handleStopAgent(agent.agentId)}
                    >
                      <Square className="w-3 h-3" />
                    </Button>
                    <Button
                      size="sm"
                      variant="outline"
                      onClick={() => setSelectedAgent(agent.agentId)}
                    >
                      <EyeIcon className="w-3 h-3" />
                    </Button>
                    <Button
                      size="sm"
                      variant="destructive"
                      onClick={() => {
                        setAgentToDelete(agent.agentId)
                      }}
                      title="删除智能体"
                    >
                      <Trash2 className="w-3 h-3" />
                    </Button>
                  </div>
                </div>

                {/* Current Task */}
                {agent.currentTask && (
                  <div className="text-sm">
                    <span className="text-muted-foreground">当前任务：</span>
                    <span>{agent.currentTask}</span>
                  </div>
                )}

                {/* Progress Bar */}
                <div>
                  <div className="flex items-center justify-between mb-1">
                    <span className="text-xs text-muted-foreground">进度</span>
                    <span className="text-xs font-medium">{Math.round(agent.progress)}%</span>
                  </div>
                  <div className="w-full bg-gray-200 dark:bg-gray-800 rounded-full h-2">
                    <div
                      className={`h-2 rounded-full transition-all ${
                        agent.progress === 100
                          ? 'bg-blue-500'
                          : agent.status === 'failed'
                            ? 'bg-red-500'
                            : 'bg-green-500'
                      }`}
                      style={{ width: `${agent.progress}%` }}
                    />
                  </div>
                </div>

                {/* Resource Usage */}
                <div className="grid grid-cols-2 gap-2 text-xs">
                  <div className="flex items-center gap-2">
                    <Cpu className="w-3 h-3 text-purple-500" />
                    <span>CPU: {agent.cpuUsage.toFixed(1)}%</span>
                  </div>
                  <div className="flex items-center gap-2">
                    <HardDrive className="w-3 h-3 text-blue-500" />
                    <span>内存：{(agent.memoryUsage / 1024).toFixed(1)} MB</span>
                  </div>
                </div>

                {/* Recent Logs */}
                <div className="bg-black/5 dark:bg-white/5 rounded-md p-2 font-mono text-xs max-h-24 overflow-y-auto">
                  {agent.logs.slice(-5).map((log, idx) => (
                    <div
                      key={`${agent.agentId}-log-${idx}`}
                      className="text-gray-700 dark:text-gray-300 truncate"
                    >
                      {log}
                    </div>
                  ))}
                </div>
              </div>
            </Card>
          ))
        )}
      </div>

      {/* Selected Agent Detail Modal */}
      {selectedAgent && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
          <Card className="max-w-2xl w-full max-h-[80vh] overflow-hidden">
            <div className="p-6 border-b">
              <div className="flex items-center justify-between">
                <div>
                  {(() => {
                    const agent = agents.find(a => a.agentId === selectedAgent)
                    return (
                      <>
                        <h3 className="text-lg font-bold">
                          Agent 详情：{agent?.name || selectedAgent}
                        </h3>
                        {agent?.name && (
                          <p className="text-xs text-muted-foreground font-mono mt-1">
                            {selectedAgent}
                          </p>
                        )}
                      </>
                    )
                  })()}
                </div>
                <Button variant="ghost" size="sm" onClick={() => setSelectedAgent(null)}>
                  ✕
                </Button>
              </div>
            </div>
            <div className="p-6 overflow-y-auto max-h-96 space-y-4">
              {agents
                .filter(a => a.agentId === selectedAgent)
                .map(agent => (
                  <div key={agent.agentId} className="space-y-4">
                    <div>
                      <h4 className="font-semibold mb-2">基本信息</h4>
                      <div className="grid grid-cols-2 gap-2 text-sm">
                        {agent.name && (
                          <div className="col-span-2">
                            <span className="text-muted-foreground">名称：</span>
                            <span className="font-medium">{agent.name}</span>
                          </div>
                        )}
                        <div>
                          <span className="text-muted-foreground">类型：</span>
                          <span>{agent.type}</span>
                        </div>
                        <div>
                          <span className="text-muted-foreground">状态：</span>
                          {getStatusBadge(agent.status)}
                        </div>
                        <div>
                          <span className="text-muted-foreground">进度：</span>
                          <span>{Math.round(agent.progress)}%</span>
                        </div>
                        <div>
                          <span className="text-muted-foreground">会话 ID：</span>
                          <span className="font-mono text-xs">{agent.sessionId}</span>
                        </div>
                      </div>
                    </div>

                    <div>
                      <h4 className="font-semibold mb-2">完整日志</h4>
                      <div className="bg-black/5 dark:bg-white/5 rounded-md p-3 font-mono text-xs max-h-48 overflow-y-auto space-y-1">
                        {agent.logs.map((log, idx) => (
                          <div
                            key={`${agent.agentId}-full-log-${idx}`}
                            className="text-gray-700 dark:text-gray-300"
                          >
                            {log}
                          </div>
                        ))}
                      </div>
                    </div>
                  </div>
                ))}
            </div>
          </Card>
        </div>
      )}

      {/* Delete Confirmation Dialog */}
      <Dialog
        key="delete-dialog"
        open={!!agentToDelete}
        onOpenChange={open => {
          if (!open) {
            setAgentToDelete(null)
          }
        }}
      >
        <DialogContent>
          <DialogHeader>
            <DialogTitle>确认删除智能体</DialogTitle>
            <DialogDescription>
              您确定要删除智能体 <strong>{agentToDelete}</strong>{' '}
              吗？此操作不可恢复，智能体的所有数据和日志将被永久删除。
            </DialogDescription>
          </DialogHeader>
          <DialogFooter>
            <Button variant="outline" onClick={() => setAgentToDelete(null)}>
              取消
            </Button>
            <Button
              variant="destructive"
              onClick={() => {
                if (agentToDelete) {
                  handleDeleteAgent(agentToDelete)
                }
              }}
            >
              删除
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      {/* Create Agent Dialog */}
      <CreateAgentDialog
        key="create-agent-dialog"
        open={showCreateDialog}
        onOpenChange={setShowCreateDialog}
        onSuccess={handleAgentCreated}
        projectId={projectId}
      />
    </div>
  )
}
