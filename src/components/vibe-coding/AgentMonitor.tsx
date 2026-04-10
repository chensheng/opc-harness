import { useState, useEffect } from 'react'
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
} from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import type { AgentInfo } from './CodingWorkspaceTypes'

export function AgentMonitor() {
  const [agents, setAgents] = useState<AgentInfo[]>([
    {
      agentId: 'agent-001',
      type: 'initializer',
      status: 'completed',
      currentTask: '任务分解完成',
      progress: 100,
      cpuUsage: 0,
      memoryUsage: 0,
      logs: ['初始化完成', '生成 15 个 Issues'],
      sessionId: 'session-001',
    },
    {
      agentId: 'agent-002',
      type: 'coding',
      status: 'running',
      currentTask: '实现用户认证功能',
      progress: 65,
      cpuUsage: 45.3,
      memoryUsage: 512,
      logs: ['正在读取 Issue #1', '分析代码结构', '生成代码中...'],
      sessionId: 'session-002',
    },
    {
      agentId: 'agent-003',
      type: 'coding',
      status: 'running',
      currentTask: '实现项目管理模块',
      progress: 40,
      cpuUsage: 38.7,
      memoryUsage: 428,
      logs: ['创建数据库模型', '实现 API 接口'],
      sessionId: 'session-003',
    },
    {
      agentId: 'agent-004',
      type: 'coding',
      status: 'paused',
      currentTask: '等待检查点审查',
      progress: 80,
      cpuUsage: 0,
      memoryUsage: 256,
      logs: ['CP-006 检查点触发', '等待用户审查'],
      sessionId: 'session-004',
    },
  ])
  const [selectedAgent, setSelectedAgent] = useState<string | null>(null)
  const [isAutoScroll, setIsAutoScroll] = useState(true)

  // Mock real-time updates
  useEffect(() => {
    const interval = setInterval(() => {
      setAgents(prev =>
        prev.map(agent => {
          if (agent.status === 'running') {
            return {
              ...agent,
              progress: Math.min(100, agent.progress + Math.random() * 2),
              cpuUsage: 30 + Math.random() * 30,
              memoryUsage: 400 + Math.random() * 200,
            }
          }
          return agent
        })
      )
    }, 2000)

    return () => clearInterval(interval)
  }, [])

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
          <Button>
            <RefreshCw className="w-4 h-4 mr-2" />
            刷新状态
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
        {agents.map(agent => (
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
                    <span className="font-semibold">{agent.agentId}</span>
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
                  <div key={idx} className="text-gray-700 dark:text-gray-300 truncate">
                    {log}
                  </div>
                ))}
              </div>
            </div>
          </Card>
        ))}
      </div>

      {/* Selected Agent Detail Modal */}
      {selectedAgent && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
          <Card className="max-w-2xl w-full max-h-[80vh] overflow-hidden">
            <div className="p-6 border-b">
              <div className="flex items-center justify-between">
                <h3 className="text-lg font-bold">Agent 详情：{selectedAgent}</h3>
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
                          <div key={idx} className="text-gray-700 dark:text-gray-300">
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
    </div>
  )
}
