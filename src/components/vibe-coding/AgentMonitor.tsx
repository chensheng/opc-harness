import { useState, useEffect, useCallback } from 'react'
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
  Edit,
  LayoutGrid,
  List,
  Bot,
} from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
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
import { EditAgentDialog } from './EditAgentDialog'
import { AgentOffice } from './AgentOffice'
import { useAgent } from '@/hooks/useAgent'

// ==================== 配置常量 ====================
const LOG_RETENTION_LIMIT = 50 // 每个智能体保留的最大日志数量
const LOG_DISPLAY_COUNT = 10   // UI 显示的最近日志数量
const LOG_THROTTLE_INTERVAL = 200 // 日志节流间隔（毫秒）- 防止高频日志刷屏

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
  agentsMdContent?: string

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
  agents_md_content?: string
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
    projectId: session.projectId || session.project_id, // 保存项目 ID 用于日志路由
  }
}

export function AgentMonitor() {
  const { projectId } = useParams<{ projectId: string }>()
  const [agents, setAgents] = useState<AgentInfo[]>([])
  const [loading, setLoading] = useState(true)
  const [selectedAgent, setSelectedAgent] = useState<string | null>(null)
  const [showCreateDialog, setShowCreateDialog] = useState(false)
  const [agentToDelete, setAgentToDelete] = useState<string | null>(null)
  const [agentToEdit, setAgentToEdit] = useState<AgentInfo | null>(null)
  const [viewMode, setViewMode] = useState<'office' | 'list'>('office') // 视图模式：办公室或列表
  const [isInitialLoad, setIsInitialLoad] = useState(true) // 标记是否为首次加载

  // 使用 Agent WebSocket Hook（实时通信）
  const { messages, connectWebSocket, disconnectWebSocket, clearMessages } = useAgent()

  // 将 WebSocket 消息分发到对应智能体的日志中
  useEffect(() => {
    if (messages.length === 0) return

    // 获取最后一条消息
    const lastMessage = messages[messages.length - 1]
    
    // 🚫 过滤掉系统级别的连接状态消息，不显示在智能体日志中
    if (lastMessage.type === 'status' && lastMessage.content?.includes('Frontend connected')) {
      console.log('[AgentMonitor] Skipping system connection status message')
      return
    }
    
    console.log('[AgentMonitor] Processing message:', {
      type: lastMessage.type,
      sessionId: lastMessage.sessionId,
      content: lastMessage.content?.substring(0, 100),
      metadata: lastMessage.metadata,
      timestamp: lastMessage.timestamp
    })
    
    // 打印所有智能体的 ID 信息，用于调试
    console.log('[AgentMonitor] Available agents:', agents.map(a => ({
      agentId: a.agentId,
      sessionId: a.sessionId,
      projectId: a.projectId,
      expectedSessionIds: [
        `agent-${a.agentId}`,  // WebSocket 日志使用的格式
        a.sessionId             // 数据库中的 session_id（兼容旧格式）
      ]
    })))
    
    console.log('[AgentMonitor] Trying to match message sessionId:', lastMessage.sessionId)
    
    // 根据 sessionId 找到对应的智能体
    setAgents(prev => {
      // 找到第一个匹配的智能体（避免重复添加）
      const matchedAgentIndex = prev.findIndex(agent => {
        // 精确匹配，避免模糊匹配导致的跨智能体污染
        // 1. agent-{agentId} - WebSocket 日志使用的格式（worker_id = agent_id）
        // 2. 直接匹配 sessionId - 兼容数据库中的 session_id（如 session-xxx）
        // 注意：不再支持 project-{projectId} 格式，日志仅发送到智能体粒度
        return lastMessage.sessionId === `agent-${agent.agentId}` ||
               lastMessage.sessionId === agent.sessionId
      })
      
      if (matchedAgentIndex === -1) {
        console.warn('[AgentMonitor] No agent matched for message:', {
          sessionId: lastMessage.sessionId,
          projectId,
          agents: prev.map(a => ({
            agentId: a.agentId,
            projectId: a.projectId,
            sessionId: a.sessionId,
            expectedSessionId: `agent-${a.agentId}`
          }))
        })
        return prev
      }
      
      console.log('[AgentMonitor] Matched agent:', {
        agentId: prev[matchedAgentIndex].agentId,
        agentProjectId: prev[matchedAgentIndex].projectId,
        messageSessionId: lastMessage.sessionId
      })
      
      // 处理 log、status、progress、error 类型的消息
      if (lastMessage.type === 'log' || lastMessage.type === 'status' || lastMessage.type === 'progress' || lastMessage.type === 'error') {
        // 添加新日志到该智能体
        const date = new Date(lastMessage.timestamp)
        const timeStr = date.toLocaleTimeString('zh-CN', { hour12: false })
        const milliseconds = String(date.getMilliseconds()).padStart(3, '0')
        const timestamp = `${timeStr}.${milliseconds}`
        
        let logContent = `[${timestamp}] ${lastMessage.content}`
        
        // 如果是 status 类型，添加状态前缀
        if (lastMessage.type === 'status') {
          logContent = `[${timestamp}] 📊 ${lastMessage.content}`
        } else if (lastMessage.type === 'progress') {
          logContent = `[${timestamp}] 📈 ${lastMessage.content}`
        } else if (lastMessage.type === 'error') {
          logContent = `[${timestamp}] ❌ ${lastMessage.content}`
        }
        
        console.log('[AgentMonitor] Adding log to agent:', prev[matchedAgentIndex].agentId, logContent)
        
        // 创建新的数组，只更新匹配的智能体
        const newAgents = [...prev]
        const currentAgent = newAgents[matchedAgentIndex]
        
        newAgents[matchedAgentIndex] = {
          ...currentAgent,
          logs: [...currentAgent.logs, logContent].slice(-LOG_RETENTION_LIMIT), // 保留最近 LOG_RETENTION_LIMIT 条
        }
        
        return newAgents
      }
      
      return prev
    })
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [messages, projectId]) // 移除 agents 依赖，避免循环触发

  // 刷新智能体列表（从数据库加载）
  const refreshAgents = useCallback(async () => {
    await loadAgents(true)
  }, [])

  // 计算运行中和处理中的智能体数量
  const runningCount = agents.filter(a => a.status === 'running').length
  const processingCount = agents.filter(a => a.currentTask && a.status === 'running').length

  // 监听 agentToDelete 状态变化
  useEffect(() => {}, [agentToDelete])

  // 当 projectId 可用时，建立 WebSocket 连接
  useEffect(() => {
    if (projectId) {
      const sessionId = `project-${projectId}`
      connectWebSocket(sessionId).catch(err => {
        console.error('[AgentMonitor] Failed to connect WebSocket:', err)
      })
    }

    return () => {
      disconnectWebSocket()
    }
  }, [projectId, connectWebSocket, disconnectWebSocket])

  // 当智能体列表加载完成后，为每个运行中的智能体建立 WebSocket 连接
  useEffect(() => {
    if (agents.length > 0 && projectId) {
      // 为每个运行中的智能体建立 WebSocket 连接
      const runningAgents = agents.filter(a => a.status === 'running')
      
      runningAgents.forEach(agent => {
        const sessionId = `agent-${agent.agentId}`
        console.log('[AgentMonitor] Establishing WebSocket connection for agent:', agent.agentId, 'sessionId:', sessionId)
        
        connectWebSocket(sessionId).catch(err => {
          console.error(`[AgentMonitor] Failed to connect WebSocket for agent ${agent.agentId}:`, err)
        })
      })
    }
  }, [agents, projectId, connectWebSocket])

  // 从数据库加载智能体列表
  const loadAgents = async (silent = false) => {
    if (!projectId) {
      console.warn('[AgentMonitor] Project ID is not available')
      if (!silent) setLoading(false)
      return
    }

    try {
      // 只有非静默模式且是首次加载时才显示 loading
      if (!silent && isInitialLoad) {
        setLoading(true)
      }

      // 1. 从数据库加载智能体会话
      const sessions = await invoke<AgentSession[]>('get_sessions_by_project', {
        projectId,
      })

      // 2. 获取正在运行的 Workers 列表
      let runningWorkers: string[] = []
      try {
        const workers = await invoke<any[]>('list_agent_workers')
        runningWorkers = workers.filter(w => w.is_running).map(w => w.worker_id)
        console.log('[AgentMonitor] Running workers:', runningWorkers)
      } catch (error) {
        console.warn('[AgentMonitor] Failed to get running workers:', error)
      }

      // 3. 转换会话信息，并根据实际运行状态调整 status
      const agentInfos = sessions.map(session => {
        const info = convertSessionToAgentInfo(session)
        
        // 如果数据库中状态为 running，但实际没有对应的 Worker，则修正为 stopped
        if (info.status === 'running' && !runningWorkers.includes(info.agentId)) {
          console.log(`[AgentMonitor] Agent ${info.agentId} marked as stopped (no worker found)`)
          info.status = 'stopped'
        }
        
        return info
      })

      // 静默刷新时，保留现有的日志数据（避免清空 WebSocket 推送的日志）
      if (silent) {
        setAgents(prev => {
          // 创建现有智能体的日志映射
          const logsMap = new Map<string, string[]>()
          prev.forEach(agent => {
            if (agent.logs && agent.logs.length > 0) {
              logsMap.set(agent.agentId, agent.logs)
            }
          })
          
          // 将保留的日志合并到新的智能体列表中
          return agentInfos.map(info => {
            const existingLogs = logsMap.get(info.agentId)
            if (existingLogs) {
              return { ...info, logs: existingLogs }
            }
            return info
          })
        })
      } else {
        // 首次加载时，直接使用新数据（无历史日志）
        setAgents(agentInfos)
      }

      // 首次加载完成后，标记为非首次加载
      if (isInitialLoad) {
        setIsInitialLoad(false)
      }
    } catch (error) {
      console.error('[AgentMonitor] Failed to load agents:', error)
      // 失败时显示空列表，不阻塞用户操作
      setAgents([])
    } finally {
      // 只有非静默模式才更新 loading 状态
      if (!silent || isInitialLoad) {
        setLoading(false)
      }
    }
  }

  // 组件挂载时加载数据（首次加载，显示 loading）
  useEffect(() => {
    loadAgents(false)
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [projectId])

  // 定时刷新数据（每5秒，静默刷新）
  useEffect(() => {
    const interval = setInterval(() => {
      loadAgents(true) // 静默刷新，不显示 loading
    }, 5000)

    return () => clearInterval(interval)
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [projectId])

  const handleStartAgent = async (agentId: string) => {
    try {
      console.log('[AgentMonitor] Starting agent worker:', agentId)
      
      // 获取当前项目 ID
      if (!projectId) {
        console.error('[AgentMonitor] Project ID is not available')
        return
      }
      
      console.log('[AgentMonitor] Calling start_agent_worker with params:', {
        workerId: agentId,
        projectId: projectId,
        checkInterval: 30,
      })
      
      // 调用后端 API 真正启动 Agent Worker
      // 注意：Tauri 2.x 会自动将 Rust 的蛇形命名转换为 JS 的驼峰命名
      const workerId = await invoke<string>('start_agent_worker', {
        workerId: agentId,
        projectId: projectId,
        checkInterval: 30, // 每 30 秒检查一次待处理的故事
      })
      
      console.log('[AgentMonitor] Agent worker started:', workerId)
      
      // 更新数据库中的状态
      await invoke('update_agent_session_status', {
        agentId,
        status: 'running',
        phase: 'processing',
      })

      // 更新前端状态
      setAgents(prev => prev.map(a => (a.agentId === agentId ? { ...a, status: 'running' } : a)))
      
      // 显示成功提示
      console.log('[AgentMonitor] ✅ Agent worker started successfully')
    } catch (error) {
      console.error('[AgentMonitor] Failed to start agent:', error)
      alert(`启动智能体失败: ${error}`)
    }
  }

  const handlePauseAgent = async (agentId: string) => {
    try {
      console.log('[AgentMonitor] Pausing agent worker:', agentId)
      
      // 调用后端 API 真正停止 Agent Worker
      // 注意：Tauri 2.x 会自动将 Rust 的蛇形命名转换为 JS 的驼峰命名
      await invoke('stop_agent_worker', {
        workerId: agentId,
      })
      
      console.log('[AgentMonitor] Agent worker stopped:', agentId)
      
      // 更新数据库中的状态
      await invoke('update_agent_session_status', {
        agentId,
        status: 'paused',
        phase: 'waiting',
      })

      // 更新前端状态
      setAgents(prev =>
        prev.map(a => (a.agentId === agentId ? { ...a, status: 'paused', cpuUsage: 0 } : a))
      )
      
      console.log('[AgentMonitor] ✅ Agent worker paused successfully')
    } catch (error) {
      console.error('[AgentMonitor] Failed to pause agent:', error)
      alert(`暂停智能体失败: ${error}`)
    }
  }

  const handleResumeAgent = async (agentId: string) => {
    try {
      console.log('[AgentMonitor] Resuming agent worker:', agentId)
      
      if (!projectId) {
        console.error('[AgentMonitor] Project ID is not available')
        return
      }
      
      // 调用后端 API 重新启动 Agent Worker
      // 注意：Tauri 2.x 会自动将 Rust 的蛇形命名转换为 JS 的驼峰命名
      const workerId = await invoke<string>('start_agent_worker', {
        workerId: agentId,
        projectId: projectId,
        checkInterval: 30,
      })
      
      console.log('[AgentMonitor] Agent worker resumed:', workerId)
      
      // 更新数据库中的状态
      await invoke('update_agent_session_status', {
        agentId,
        status: 'running',
        phase: 'processing',
      })

      // 更新前端状态
      setAgents(prev => prev.map(a => (a.agentId === agentId ? { ...a, status: 'running' } : a)))
      
      console.log('[AgentMonitor] ✅ Agent worker resumed successfully')
    } catch (error) {
      console.error('[AgentMonitor] Failed to resume agent:', error)
      alert(`恢复智能体失败: ${error}`)
    }
  }

  const handleStopAgent = async (agentId: string) => {
    try {
      // 先调用后端 API 更新状态到数据库
      await invoke('update_agent_session_status', {
        agentId,
        status: 'stopped',
        phase: 'idle',
      })

      // 再更新前端状态
      setAgents(prev =>
        prev.map(a =>
          a.agentId === agentId ? { ...a, status: 'stopped', progress: 0, cpuUsage: 0 } : a
        )
      )
    } catch (error) {
      console.error('Failed to stop agent:', error)
      // 可以添加错误提示
    }
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
    // 重新加载智能体列表（静默刷新）
    loadAgents(true)
  }

  const handleAgentEdited = () => {
    console.log('智能体编辑成功')
    // 重新加载智能体列表（静默刷新）
    loadAgents(true)
  }

  const handleRefresh = () => {
    loadAgents(true) // 手动刷新也使用静默模式
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

  const getAgentTypeLabel = (type: AgentInfo['type']) => {
    switch (type) {
      case 'initializer':
        return '初始化'
      case 'coding':
        return '编码'
      case 'mr_creation':
        return '创建 MR'
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
          <h2 className="text-2xl font-bold">智能体监控面板</h2>
          <p className="text-muted-foreground mt-1">实时监控智能体的运行状态、任务执行和资源使用</p>
        </div>
        <div className="flex gap-2">
          {/* 智能体状态指示器 */}
          <Badge variant="secondary" className="px-3 py-1">
            {runningCount} 运行中 / {agents.length} 总计
          </Badge>

          <Button variant="outline" onClick={handleRefresh} disabled={loading}>
            <RefreshCw className={`w-4 h-4 mr-2 ${loading ? 'animate-spin' : ''}`} />
            {loading ? '加载中...' : '刷新状态'}
          </Button>
          <Button onClick={() => setShowCreateDialog(true)}>
            <Plus className="w-4 h-4 mr-2" />
            创建智能体
          </Button>
        </div>
      </div>

      {/* View Mode Tabs */}
      <Tabs
        value={viewMode}
        onValueChange={v => setViewMode(v as 'office' | 'list')}
        className="w-full"
      >
        <TabsList className="grid w-full max-w-md grid-cols-2">
          <TabsTrigger value="office" className="flex items-center gap-2">
            <LayoutGrid className="w-4 h-4" />
            办公室视图
          </TabsTrigger>
          <TabsTrigger value="list" className="flex items-center gap-2">
            <List className="w-4 h-4" />
            列表视图
          </TabsTrigger>
        </TabsList>

        {/* Office View */}
        <TabsContent value="office" className="mt-6">
          {/* Agent Status Card */}
          {agents.length > 0 && (
            <Card className="p-4 mb-6 bg-gradient-to-r from-blue-50 to-purple-50 dark:from-blue-950/20 dark:to-purple-950/20 border-blue-200 dark:border-blue-800">
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-3">
                  <div className="p-2 bg-blue-100 dark:bg-blue-900 rounded-lg">
                    <Bot className="w-5 h-5 text-blue-600 dark:text-blue-400" />
                  </div>
                  <div>
                    <h3 className="font-semibold text-sm">智能体运行状态</h3>
                    <p className="text-xs text-muted-foreground">
                      {runningCount} 个运行中，{processingCount} 个处理任务 · 每个智能体创建后自动启动并持续监控数据库
                    </p>
                  </div>
                </div>
                <div className="flex gap-2">
                  {agents.slice(0, 5).map(agent => (
                    <Badge
                      key={agent.agentId}
                      variant={agent.status === 'running' ? 'default' : 'secondary'}
                      className={agent.status === 'running' ? 'bg-green-500' : ''}
                    >
                      {agent.name || agent.agentId.substring(0, 8)}
                    </Badge>
                  ))}
                  {agents.length > 5 && (
                    <Badge variant="outline">+{agents.length - 5}</Badge>
                  )}
                </div>
              </div>
            </Card>
          )}

          <AgentOffice
            agents={agents}
            loading={loading}
            _onStartAgent={handleStartAgent}
            onPauseAgent={handlePauseAgent}
            onResumeAgent={handleResumeAgent}
            onStopAgent={handleStopAgent}
            _onRefresh={handleRefresh}
          />
        </TabsContent>

        {/* List View */}
        <TabsContent value="list" className="mt-6">
          {/* Statistics Cards */}
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-5 gap-4 mb-6">
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
                  <p className="text-2xl font-bold">{avgProgress.toFixed(1)}%</p>
                </div>
                <FileText className="w-8 h-8 text-purple-500" />
              </div>
            </Card>

            <Card className="p-4">
              <div className="flex items-center justify-between">
                <div>
                  <p className="text-sm text-muted-foreground">CPU 使用率</p>
                  <p className="text-2xl font-bold">{totalCpuUsage.toFixed(1)}%</p>
                </div>
                <Cpu className="w-8 h-8 text-orange-500" />
              </div>
            </Card>

            <Card className="p-4">
              <div className="flex items-center justify-between">
                <div>
                  <p className="text-sm text-muted-foreground">内存使用</p>
                  <p className="text-2xl font-bold">{totalMemoryUsage.toFixed(1)}%</p>
                </div>
                <HardDrive className="w-8 h-8 text-red-500" />
              </div>
            </Card>
          </div>

          {/* Agents Table */}
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
                <p className="text-muted-foreground mb-4">点击"创建智能体"按钮开始创建新的智能体</p>
                <Button onClick={() => setShowCreateDialog(true)}>
                  <Plus className="w-4 h-4 mr-2" />
                  创建智能体
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
                          <span className="font-semibold">
                            {agent.name || getAgentTypeLabel(agent.type)}
                          </span>
                          {getStatusBadge(agent.status)}
                        </div>
                      </div>
                      <div className="flex gap-2">
                        {/* 运行控制按钮组 */}
                        {agent.status === 'running' ? (
                          // 运行中：显示暂停和停止按钮
                          <>
                            <Button
                              size="sm"
                              variant="outline"
                              onClick={() => handlePauseAgent(agent.agentId)}
                              title="暂停智能体"
                            >
                              <Pause className="w-3 h-3" />
                            </Button>
                            <Button
                              size="sm"
                              variant="destructive"
                              onClick={() => handleStopAgent(agent.agentId)}
                              title="停止智能体"
                            >
                              <Square className="w-3 h-3" />
                            </Button>
                          </>
                        ) : agent.status === 'paused' ? (
                          // 已暂停：显示恢复和停止按钮
                          <>
                            <Button
                              size="sm"
                              variant="outline"
                              onClick={() => handleResumeAgent(agent.agentId)}
                              title="恢复智能体"
                            >
                              <Play className="w-3 h-3" />
                            </Button>
                            <Button
                              size="sm"
                              variant="destructive"
                              onClick={() => handleStopAgent(agent.agentId)}
                              title="停止智能体"
                            >
                              <Square className="w-3 h-3" />
                            </Button>
                          </>
                        ) : (
                          // 已停止/其他状态：显示启动按钮
                          <Button
                            size="sm"
                            variant="default"
                            onClick={() => handleStartAgent(agent.agentId)}
                            title="启动智能体"
                          >
                            <Play className="w-3 h-3" />
                          </Button>
                        )}
                        <Button
                          size="sm"
                          variant="outline"
                          onClick={() => setSelectedAgent(agent.agentId)}
                        >
                          <EyeIcon className="w-3 h-3" />
                        </Button>
                        <Button
                          size="sm"
                          variant="outline"
                          onClick={() => setAgentToEdit(agent)}
                          title="编辑智能体"
                        >
                          <Edit className="w-3 h-3" />
                        </Button>
                        {/* 只有停止状态的智能体才显示删除按钮 */}
                        {agent.status === 'stopped' && (
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
                        )}
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

                    {/* Agent Logs - Enhanced Display */}
                    <div className="border rounded-md bg-gray-50 dark:bg-gray-950/30">
                      <div className="px-3 py-2 border-b bg-gray-100 dark:bg-gray-900/50 flex items-center justify-between">
                        <div className="flex items-center gap-2">
                          <Terminal className="w-3 h-3 text-green-600 dark:text-green-400" />
                          <span className="text-xs font-semibold">运行日志</span>
                          <Badge variant="secondary" className="text-xs px-1.5 py-0">
                            {agent.logs.length}
                          </Badge>
                        </div>
                        <Button
                          size="sm"
                          variant="ghost"
                          className="h-5 px-2 text-xs"
                          onClick={() => setSelectedAgent(agent.agentId)}
                        >
                          查看全部
                        </Button>
                      </div>
                      <div className="p-3 max-h-32 overflow-y-auto font-mono text-xs space-y-1">
                        {agent.logs.length === 0 ? (
                          <div className="text-center py-4 text-muted-foreground">
                            <Activity className="w-8 h-8 mx-auto mb-1 opacity-20" />
                            <p className="text-xs">暂无日志</p>
                          </div>
                        ) : (
                          agent.logs.slice(-LOG_DISPLAY_COUNT).reverse().map((log, idx) => {
                            // 根据日志内容判断类型并着色
                            const isError = log.includes('❌') || log.includes('ERROR') || log.includes('error')
                            const isSuccess = log.includes('✅') || log.includes('SUCCESS') || log.includes('success')
                            const isWarning = log.includes('⚠️') || log.includes('WARNING') || log.includes('warning')
                            const isProgress = log.includes('📊') || log.includes('PROGRESS') || log.includes('progress')
                            
                            return (
                              <div
                                key={`${agent.agentId}-log-${idx}`}
                                className={`break-all ${
                                  isError
                                    ? 'text-red-600 dark:text-red-400'
                                    : isSuccess
                                    ? 'text-green-600 dark:text-green-400'
                                    : isWarning
                                    ? 'text-yellow-600 dark:text-yellow-400'
                                    : isProgress
                                    ? 'text-blue-600 dark:text-blue-400'
                                    : 'text-gray-700 dark:text-gray-300'
                                }`}
                              >
                                {log}
                              </div>
                            )
                          })
                        )}
                      </div>
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
                              Agent 详情：
                              {agent?.name || getAgentTypeLabel(agent?.type || 'coding')}
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
                            {agent.logs.slice().reverse().map((log, idx) => (
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
                  {(() => {
                    const agent = agents.find(a => a.agentId === agentToDelete)
                    if (agent && (agent.status === 'running' || agent.status === 'paused')) {
                      return (
                        <div className="mt-2 p-3 bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-200 dark:border-yellow-800 rounded-md">
                          <p className="text-sm text-yellow-800 dark:text-yellow-200">
                            ⚠️ 该智能体当前处于{' '}
                            <strong>{agent.status === 'running' ? '运行中' : '已暂停'}</strong>{' '}
                            状态，无法删除。请先停止智能体后再尝试删除。
                          </p>
                        </div>
                      )
                    }
                    return null
                  })()}
                </DialogDescription>
              </DialogHeader>
              <DialogFooter>
                <Button variant="outline" onClick={() => setAgentToDelete(null)}>
                  取消
                </Button>
                <Button
                  variant="destructive"
                  disabled={(() => {
                    const agent = agents.find(a => a.agentId === agentToDelete)
                    return agent && (agent.status === 'running' || agent.status === 'paused')
                  })()}
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

          {/* Edit Agent Dialog */}
          <EditAgentDialog
            open={agentToEdit !== null}
            onOpenChange={open => {
              if (!open) {
                setAgentToEdit(null)
              }
            }}
            agent={agentToEdit}
            onSuccess={handleAgentEdited}
            projectId={projectId}
          />
        </TabsContent>
      </Tabs>
    </div>
  )
}
