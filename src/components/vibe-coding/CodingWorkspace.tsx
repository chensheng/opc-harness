import { useState, useEffect, useRef } from 'react'
import { useParams, useNavigate } from 'react-router-dom'
import {
  Play,
  Square,
  Send,
  FolderTree,
  FileCode,
  ExternalLink,
  RefreshCw,
  ChevronRight,
  ChevronDown,
  File,
  Folder,
  CheckCircle,
  AlertCircle,
  Loader2,
  Terminal,
  GitBranch,
  FileText,
  ListTodo,
  Activity,
  Cpu,
  HardDrive,
  Pause,
  Eye,
} from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card } from '@/components/ui/card'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { Badge } from '@/components/ui/badge'
import { useProjectStore } from '@/stores'
import type { FileNode, CLIOutputLine } from '@/types'

interface AgentInfo {
  agentId: string
  type: 'initializer' | 'coding' | 'mr_creation'
  status: 'idle' | 'running' | 'paused' | 'completed' | 'failed' | 'stopped'
  currentTask?: string
  progress: number
  cpuUsage: number
  memoryUsage: number
  logs: string[]
  sessionId: string
}

// Mock file tree
const mockFileTree: FileNode[] = [
  {
    name: 'src',
    path: '/src',
    type: 'directory',
    isExpanded: true,
    children: [
      {
        name: 'components',
        path: '/src/components',
        type: 'directory',
        children: [
          { name: 'Button.tsx', path: '/src/components/Button.tsx', type: 'file' },
          { name: 'Card.tsx', path: '/src/components/Card.tsx', type: 'file' },
        ],
      },
      { name: 'App.tsx', path: '/src/App.tsx', type: 'file' },
      { name: 'main.tsx', path: '/src/main.tsx', type: 'file' },
    ],
  },
  { name: 'package.json', path: '/package.json', type: 'file' },
  { name: 'README.md', path: '/README.md', type: 'file' },
]

// Mock CLI output
const mockCLIOutput = [
  { type: 'stdout' as const, content: '> Starting development server...', timestamp: '10:00:01' },
  { type: 'stdout' as const, content: '> Ready on http://localhost:3000', timestamp: '10:00:03' },
  { type: 'stdout' as const, content: '> Compiling...', timestamp: '10:00:05' },
  { type: 'stdout' as const, content: '> Compiled successfully', timestamp: '10:00:08' },
]

interface InitializerStep {
  id: string
  name: string
  description: string
  icon: React.ReactNode
  status: 'pending' | 'running' | 'completed' | 'failed'
  logs: string[]
  error?: string
}

export function AgentMonitor() {
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  const { projectId: _projectId } = useParams<{ projectId: string }>()
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
            <Eye className="w-4 h-4 mr-2" />
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
                    <Eye className="w-3 h-3" />
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
                    <div className="grid grid-cols-2 gap-4">
                      <div>
                        <p className="text-sm text-muted-foreground">Agent ID</p>
                        <p className="font-mono">{agent.agentId}</p>
                      </div>
                      <div>
                        <p className="text-sm text-muted-foreground">会话 ID</p>
                        <p className="font-mono">{agent.sessionId}</p>
                      </div>
                      <div>
                        <p className="text-sm text-muted-foreground">类型</p>
                        <div className="flex items-center gap-2">
                          {getAgentTypeIcon(agent.type)}
                          <span className="capitalize">{agent.type}</span>
                        </div>
                      </div>
                      <div>
                        <p className="text-sm text-muted-foreground">状态</p>
                        {getStatusBadge(agent.status)}
                      </div>
                    </div>

                    <div>
                      <p className="text-sm text-muted-foreground mb-2">完整日志</p>
                      <div className="bg-black/5 dark:bg-white/5 rounded-md p-3 font-mono text-xs max-h-48 overflow-y-auto">
                        {agent.logs.map((log, idx) => (
                          <div key={idx} className="text-gray-700 dark:text-gray-300">
                            [{new Date().toLocaleTimeString()}] {log}
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

export function InitializerWorkflow() {
  const { projectId } = useParams<{ projectId: string }>()
  const navigate = useNavigate()
  const [steps, setSteps] = useState<InitializerStep[]>([
    {
      id: 'prd-parsing',
      name: 'PRD 解析',
      description: '解析产品需求文档，提取核心功能',
      icon: <FileText className="w-5 h-5" />,
      status: 'pending',
      logs: [],
    },
    {
      id: 'env-check',
      name: '环境检查',
      description: '验证开发环境和工具链',
      icon: <Terminal className="w-5 h-5" />,
      status: 'pending',
      logs: [],
    },
    {
      id: 'git-init',
      name: 'Git 初始化',
      description: '创建 Git 仓库和配置文件',
      icon: <GitBranch className="w-5 h-5" />,
      status: 'pending',
      logs: [],
    },
    {
      id: 'task-decomposition',
      name: '任务分解',
      description: '将 PRD 分解为可执行的 Issues',
      icon: <ListTodo className="w-5 h-5" />,
      status: 'pending',
      logs: [],
    },
  ])
  const [isRunning, setIsRunning] = useState(false)
  const [currentStepIndex, setCurrentStepIndex] = useState(0)
  const [overallProgress, setOverallProgress] = useState(0)

  const startInitialization = async () => {
    setIsRunning(true)
    setOverallProgress(0)

    // Mock initialization flow
    for (let i = 0; i < steps.length; i++) {
      setCurrentStepIndex(i)
      await executeStep(i)
      setOverallProgress(((i + 1) / steps.length) * 100)
    }

    setIsRunning(false)
    // Navigate to CP-002 checkpoint after completion
    setTimeout(() => {
      navigate(`/checkpoint/${projectId}/CP-002`)
    }, 1000)
  }

  const executeStep = async (stepIndex: number) => {
    const step = steps[stepIndex]

    // Update step status to running
    setSteps(prev => prev.map((s, idx) => (idx === stepIndex ? { ...s, status: 'running' } : s)))

    // Mock logs for each step
    const mockLogs: Record<string, string[]> = {
      'prd-parsing': [
        '正在读取 PRD 文档...',
        '分析产品需求...',
        '提取功能列表...',
        '识别技术栈...',
        'PRD 解析完成！共识别 12 个核心功能',
      ],
      'env-check': [
        '检查 Git 版本...',
        '✓ Git 2.40.0 已安装',
        '检查 Node.js 版本...',
        '✓ Node.js 20.10.0 已安装',
        '检查 npm 版本...',
        '✓ npm 10.2.3 已安装',
        '检查 Rust 版本...',
        '✓ Rust 1.75.0 已安装',
        '环境检查通过！',
      ],
      'git-init': [
        '初始化 Git 仓库...',
        '创建 .gitignore 文件...',
        '配置 Git 用户信息...',
        '创建初始提交...',
        'Git 仓库初始化成功！',
      ],
      'task-decomposition': [
        '分析 PRD 功能列表...',
        '设计系统架构...',
        '创建 Milestones...',
        '分解任务为 Issues...',
        '评估优先级和依赖关系...',
        '估算工时...',
        '任务分解完成！共生成 15 个 Issues',
      ],
    }

    // Simulate step execution with logs
    const logs = mockLogs[step.id] || []
    for (const log of logs) {
      await new Promise(resolve => setTimeout(resolve, 300))
      setSteps(prev =>
        prev.map((s, idx) => (idx === stepIndex ? { ...s, logs: [...s.logs, log] } : s))
      )
    }

    // Mark step as completed
    setSteps(prev => prev.map((s, idx) => (idx === stepIndex ? { ...s, status: 'completed' } : s)))
  }

  const stopInitialization = () => {
    setIsRunning(false)
    setSteps(prev =>
      prev.map((s, idx) => (idx > currentStepIndex ? { ...s, status: 'pending' } : s))
    )
  }

  const retryStep = (stepIndex: number) => {
    executeStep(stepIndex)
  }

  const getStepColor = (status: InitializerStep['status']) => {
    switch (status) {
      case 'completed':
        return 'border-green-500 bg-green-50 dark:bg-green-950/20'
      case 'running':
        return 'border-blue-500 bg-blue-50 dark:bg-blue-950/20'
      case 'failed':
        return 'border-red-500 bg-red-50 dark:bg-red-950/20'
      default:
        return 'border-gray-200 dark:border-gray-800'
    }
  }

  const getStepIcon = (status: InitializerStep['status'], icon: React.ReactNode) => {
    if (status === 'completed') return <CheckCircle className="w-5 h-5 text-green-500" />
    if (status === 'running') return <Loader2 className="w-5 h-5 text-blue-500 animate-spin" />
    if (status === 'failed') return <AlertCircle className="w-5 h-5 text-red-500" />
    return icon
  }

  const allCompleted = steps.every(s => s.status === 'completed')

  return (
    <div className="h-full flex flex-col p-6 space-y-6 overflow-auto">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-2xl font-bold">Initializer Agent 工作流</h2>
          <p className="text-muted-foreground mt-1">AI 正在初始化项目环境和任务分解</p>
        </div>
        <div className="flex gap-2">
          {!isRunning && !allCompleted && (
            <Button onClick={startInitialization}>
              <Play className="w-4 h-4 mr-2" />
              启动初始化
            </Button>
          )}
          {isRunning && (
            <Button variant="destructive" onClick={stopInitialization}>
              <Square className="w-4 h-4 mr-2" />
              停止
            </Button>
          )}
          {allCompleted && (
            <Button onClick={() => navigate(`/checkpoint/${projectId}/CP-002`)}>
              <ChevronRight className="w-4 h-4 mr-2" />
              前往审查任务分解
            </Button>
          )}
        </div>
      </div>

      {/* Progress Bar */}
      <Card className="p-4">
        <div className="flex items-center justify-between mb-2">
          <span className="text-sm font-medium">总体进度</span>
          <span className="text-sm text-muted-foreground">{Math.round(overallProgress)}%</span>
        </div>
        <div className="w-full bg-gray-200 dark:bg-gray-800 rounded-full h-2">
          <div
            className="bg-blue-500 h-2 rounded-full transition-all duration-500"
            style={{ width: `${overallProgress}%` }}
          />
        </div>
      </Card>

      {/* Steps Timeline */}
      <div className="space-y-4">
        {steps.map((step, index) => (
          <Card key={step.id} className={`border-l-4 ${getStepColor(step.status)} p-4`}>
            <div className="flex items-start gap-4">
              <div className="flex-shrink-0">{getStepIcon(step.status, step.icon)}</div>
              <div className="flex-1 space-y-2">
                <div className="flex items-center justify-between">
                  <div>
                    <h3 className="font-semibold">{step.name}</h3>
                    <p className="text-sm text-muted-foreground">{step.description}</p>
                  </div>
                  <Badge variant={step.status === 'completed' ? 'default' : 'secondary'}>
                    {step.status === 'completed'
                      ? '已完成'
                      : step.status === 'running'
                        ? '执行中'
                        : step.status === 'failed'
                          ? '失败'
                          : '等待中'}
                  </Badge>
                </div>

                {/* Logs */}
                {step.logs.length > 0 && (
                  <div className="bg-black/5 dark:bg-white/5 rounded-md p-3 font-mono text-xs space-y-1 max-h-48 overflow-y-auto">
                    {step.logs.map((log, logIndex) => (
                      <div key={logIndex} className="text-gray-700 dark:text-gray-300">
                        <span className="text-muted-foreground mr-2">
                          [{new Date().toLocaleTimeString()}]
                        </span>
                        {log}
                      </div>
                    ))}
                  </div>
                )}

                {/* Error Message */}
                {step.error && (
                  <div className="flex items-center gap-2 text-red-500 text-sm">
                    <AlertCircle className="w-4 h-4" />
                    <span>{step.error}</span>
                  </div>
                )}

                {/* Retry Button */}
                {step.status === 'failed' && (
                  <Button size="sm" variant="outline" onClick={() => retryStep(index)}>
                    <RefreshCw className="w-3 h-3 mr-2" />
                    重试此步骤
                  </Button>
                )}
              </div>
            </div>
          </Card>
        ))}
      </div>

      {/* Summary Card */}
      {allCompleted && (
        <Card className="p-6 bg-gradient-to-r from-green-50 to-blue-50 dark:from-green-950/20 dark:to-blue-950/20">
          <div className="flex items-center gap-4">
            <CheckCircle className="w-12 h-12 text-green-500" />
            <div className="flex-1">
              <h3 className="text-lg font-bold text-green-700 dark:text-green-400">
                ✨ 初始化完成！
              </h3>
              <p className="text-sm text-green-600 dark:text-green-500 mt-1">
                项目环境已准备就绪，共生成 15 个开发任务。请前往下一步审查任务分解结果。
              </p>
            </div>
            <Button onClick={() => navigate(`/checkpoint/${projectId}/CP-002`)}>
              前往审查
              <ChevronRight className="w-4 h-4 ml-2" />
            </Button>
          </div>
        </Card>
      )}
    </div>
  )
}

export function CodingWorkspace() {
  const { projectId } = useParams<{ projectId: string }>()
  const navigate = useNavigate()
  const { getProjectById, updateProjectStatus, updateProjectProgress } = useProjectStore()

  const [fileTree, setFileTree] = useState<FileNode[]>(mockFileTree)
  const [selectedFile, setSelectedFile] = useState<string | null>(null)
  const [cliOutput, setCliOutput] = useState<CLIOutputLine[]>(mockCLIOutput)
  const [cliInput, setCliInput] = useState('')
  const [isRunning, setIsRunning] = useState(false)
  const [activeTab, setActiveTab] = useState('code')
  const outputEndRef = useRef<HTMLDivElement>(null)

  const project = projectId ? getProjectById(projectId) : undefined

  useEffect(() => {
    if (project && project.status === 'design') {
      updateProjectStatus(projectId!, 'coding')
      updateProjectProgress(projectId!, 50)
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [project])

  useEffect(() => {
    outputEndRef.current?.scrollIntoView({ behavior: 'smooth' })
  }, [cliOutput])

  const toggleFolder = (path: string) => {
    const toggleNode = (nodes: FileNode[]): FileNode[] => {
      return nodes.map(node => {
        if (node.path === path) {
          return { ...node, isExpanded: !node.isExpanded }
        }
        if (node.children) {
          return { ...node, children: toggleNode(node.children) }
        }
        return node
      })
    }
    setFileTree(toggleNode(fileTree))
  }

  const renderFileTree = (nodes: FileNode[], depth = 0) => {
    return nodes.map(node => (
      <div key={node.path} style={{ paddingLeft: depth * 16 }}>
        <button
          onClick={() =>
            node.type === 'directory' ? toggleFolder(node.path) : setSelectedFile(node.path)
          }
          className={`flex items-center gap-1 w-full px-2 py-1 text-sm rounded hover:bg-accent ${
            selectedFile === node.path ? 'bg-accent' : ''
          }`}
        >
          {node.type === 'directory' ? (
            <>
              {node.isExpanded ? (
                <ChevronDown className="w-4 h-4" />
              ) : (
                <ChevronRight className="w-4 h-4" />
              )}
              <Folder className="w-4 h-4 text-yellow-500" />
            </>
          ) : (
            <>
              <span className="w-4" />
              <File className="w-4 h-4 text-blue-500" />
            </>
          )}
          <span className="truncate">{node.name}</span>
        </button>
        {node.type === 'directory' &&
          node.isExpanded &&
          node.children &&
          renderFileTree(node.children, depth + 1)}
      </div>
    ))
  }

  const handleSendCommand = () => {
    if (!cliInput.trim()) return

    setCliOutput(prev => [
      ...prev,
      { type: 'input' as const, content: cliInput, timestamp: new Date().toLocaleTimeString() },
    ])

    // Simulate response
    setTimeout(() => {
      setCliOutput(prev => [
        ...prev,
        {
          type: 'stdout',
          content: `Executing: ${cliInput}`,
          timestamp: new Date().toLocaleTimeString(),
        },
        { type: 'stdout', content: 'Done!', timestamp: new Date().toLocaleTimeString() },
      ])
    }, 500)

    setCliInput('')
  }

  const handleStartServer = () => {
    setIsRunning(!isRunning)
    if (!isRunning) {
      setCliOutput(prev => [
        ...prev,
        {
          type: 'stdout',
          content: '> Starting server...',
          timestamp: new Date().toLocaleTimeString(),
        },
      ])
    }
  }

  if (!project) {
    return (
      <div className="text-center py-12">
        <p className="text-muted-foreground">项目不存在</p>
        <Button onClick={() => navigate('/')} className="mt-4">
          返回首页
        </Button>
      </div>
    )
  }

  return (
    <div className="h-[calc(100vh-8rem)] flex flex-col">
      <div className="flex items-center justify-between mb-4">
        <div>
          <h1 className="text-xl font-bold">💻 Vibe Coding</h1>
          <p className="text-sm text-muted-foreground">{project.name}</p>
        </div>
        <div className="flex gap-2">
          <Button variant="outline" size="sm" onClick={handleStartServer}>
            {isRunning ? (
              <>
                <Square className="w-4 h-4 mr-2" />
                停止
              </>
            ) : (
              <>
                <Play className="w-4 h-4 mr-2" />
                运行
              </>
            )}
          </Button>
          <Button variant="outline" size="sm">
            <ExternalLink className="w-4 h-4 mr-2" />
            预览
          </Button>
        </div>
      </div>

      <div className="flex-1 grid grid-cols-12 gap-4 min-h-0">
        {/* File Tree */}
        <Card className="col-span-3 overflow-hidden flex flex-col">
          <div className="p-3 border-b flex items-center gap-2">
            <FolderTree className="w-4 h-4" />
            <span className="text-sm font-medium">文件</span>
          </div>
          <div className="flex-1 overflow-auto p-2">{renderFileTree(fileTree)}</div>
        </Card>

        {/* Editor / Preview */}
        <Card className="col-span-6 overflow-hidden flex flex-col">
          <Tabs value={activeTab} onValueChange={setActiveTab} className="flex flex-col h-full">
            <div className="border-b px-3">
              <TabsList className="h-10">
                <TabsTrigger value="code" className="flex items-center gap-2">
                  <FileCode className="w-4 h-4" />
                  代码
                </TabsTrigger>
                <TabsTrigger value="preview" className="flex items-center gap-2">
                  <ExternalLink className="w-4 h-4" />
                  预览
                </TabsTrigger>
              </TabsList>
            </div>

            <TabsContent value="code" className="flex-1 m-0 p-4 overflow-auto">
              {selectedFile ? (
                <div className="font-mono text-sm">
                  <div className="text-muted-foreground mb-4">{selectedFile}</div>
                  <pre className="text-muted-foreground">
                    {`// Example code for ${selectedFile}
import React from 'react'

export function Component() {
  return (
    <div className="p-4">
      <h1>Hello World</h1>
    </div>
  )
}`}
                  </pre>
                </div>
              ) : (
                <div className="flex items-center justify-center h-full text-muted-foreground">
                  选择一个文件开始编辑
                </div>
              )}
            </TabsContent>

            <TabsContent value="preview" className="flex-1 m-0 p-0 overflow-hidden">
              <iframe src="about:blank" className="w-full h-full border-0" title="Preview" />
            </TabsContent>
          </Tabs>
        </Card>

        {/* CLI Console */}
        <Card className="col-span-3 overflow-hidden flex flex-col bg-slate-950">
          <div className="p-3 border-b border-slate-800 flex items-center justify-between">
            <div className="flex items-center gap-2">
              <Badge variant={isRunning ? 'default' : 'secondary'} className="text-xs">
                {isRunning ? '运行中' : '已停止'}
              </Badge>
            </div>
            <Button variant="ghost" size="icon" className="h-6 w-6">
              <RefreshCw className="w-3 h-3" />
            </Button>
          </div>

          <div className="flex-1 overflow-auto p-3 font-mono text-xs space-y-1">
            {cliOutput.map((line, index) => (
              <div
                key={index}
                className={`${
                  line.type === 'stderr'
                    ? 'text-red-400'
                    : line.type === 'input'
                      ? 'text-blue-400'
                      : 'text-slate-300'
                }`}
              >
                <span className="text-slate-600 mr-2">[{line.timestamp}]</span>
                {line.content}
              </div>
            ))}
            <div ref={outputEndRef} />
          </div>

          <div className="p-3 border-t border-slate-800 flex gap-2">
            <input
              type="text"
              value={cliInput}
              onChange={e => setCliInput(e.target.value)}
              onKeyDown={e => e.key === 'Enter' && handleSendCommand()}
              placeholder="输入命令..."
              className="flex-1 bg-slate-900 border-slate-700 rounded px-3 py-2 text-sm text-slate-200 outline-none focus:ring-1 focus:ring-primary"
            />
            <Button size="sm" onClick={handleSendCommand}>
              <Send className="w-4 h-4" />
            </Button>
          </div>
        </Card>
      </div>
    </div>
  )
}
