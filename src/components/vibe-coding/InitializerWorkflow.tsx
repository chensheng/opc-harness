/**
 * Initializer Agent UI 组件
 *
 * 展示 Initializer Agent 的四步工作流和实时日志输出
 *
 * @author AI Agent
 * @date 2026-03-25
 */

import { useState, useEffect } from 'react'
import { CheckCircle2, Circle, Loader2, AlertCircle, Play, Terminal } from 'lucide-react'

/**
 * Initializer Agent 状态
 */
type InitializerStatusType =
  | 'Idle'
  | 'ParsingPRD'
  | 'CheckingEnvironment'
  | 'InitializingGit'
  | 'DecomposingTasks'
  | 'WaitingForHITL'
  | 'Completed'
  | 'Failed'

/**
 * 日志级别
 */
type LogLevel = 'info' | 'warn' | 'error' | 'debug'

/**
 * 日志条目
 */
interface LogEntry {
  id: string
  timestamp: Date
  level: LogLevel
  message: string
  data?: unknown
}

/**
 * 工作流步骤
 */
interface WorkflowStep {
  id: number
  title: string
  description: string
  status: 'pending' | 'active' | 'completed' | 'failed'
}

/**
 * InitializerWorkflow 组件 Props
 */
export interface InitializerWorkflowProps {
  /** 是否自动启动 */
  autoStart?: boolean
  /** 初始化完成回调 */
  onComplete?: (result: InitializerResult) => void
  /** 错误回调 */
  onError?: (error: string) => void
}

/**
 * Initializer 结果
 */
export interface InitializerResult {
  success: boolean
  productName?: string
  productDescription?: string
  issues?: Array<{
    iid: string
    title: string
    priority: string
  }>
  environmentCheck?: {
    passed: boolean
    gitInstalled: boolean
    nodeInstalled: boolean
  }
  error?: string
}

/**
 * Initializer Agent UI 组件
 */
export function InitializerWorkflow({
  autoStart = false,
  onComplete,
  onError,
}: InitializerWorkflowProps) {
  // 状态管理
  const [status, setStatus] = useState<InitializerStatusType>('Idle')
  const [logs, setLogs] = useState<LogEntry[]>([])
  const [progress, setProgress] = useState(0)
  const [isRunning, setIsRunning] = useState(false)

  /**
   * 添加日志
   */
  const addLog = (level: LogLevel, message: string, data?: unknown) => {
    const newLog: LogEntry = {
      id: `${Date.now()}-${Math.random().toString(36).substr(2, 9)}`,
      timestamp: new Date(),
      level,
      message,
      data,
    }
    setLogs(prev => [...prev, newLog])
  }

  /**
   * 启动初始化流程
   */
  const startInitialization = async () => {
    if (isRunning) return

    setIsRunning(true)
    setStatus('ParsingPRD')
    setLogs([])
    setProgress(0)

    addLog('info', '开始执行 Initializer Agent 初始化流程')

    try {
      // 模拟初始化流程（后续会替换为真实的 Backend 调用）
      await simulateInitialization()

      setStatus('Completed')
      setProgress(100)
      addLog('info', 'Initializer Agent 初始化流程完成')

      // 触发完成回调
      const mockResult: InitializerResult = {
        success: true,
        productName: '示例产品',
        productDescription: '这是一个示例产品描述',
        issues: [
          { iid: '1', title: '用户认证模块', priority: 'P0' },
          { iid: '2', title: '登录页面', priority: 'P1' },
          { iid: '3', title: '数据库设计', priority: 'P0' },
        ],
        environmentCheck: {
          passed: true,
          gitInstalled: true,
          nodeInstalled: true,
        },
      }

      onComplete?.(mockResult)
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : '未知错误'
      setStatus('Failed')
      addLog('error', `初始化失败：${errorMessage}`)
      onError?.(errorMessage)
    } finally {
      setIsRunning(false)
    }
  }

  /**
   * 模拟初始化流程（TODO: 替换为真实 Backend 调用）
   */
  const simulateInitialization = async () => {
    // Step 1: PRD 解析
    addLog('info', '步骤 1/4: 解析 PRD 文档')
    await sleep(1500)
    addLog('info', '✓ PRD 解析完成：示例电商平台')
    setProgress(25)

    await sleep(500)

    // Step 2: 环境检查
    addLog('info', '步骤 2/4: 检查开发环境')
    setStatus('CheckingEnvironment')
    await sleep(1000)
    addLog('info', '✓ Git 已安装 (v2.40.0)')
    await sleep(500)
    addLog('info', '✓ Node.js 已安装 (v20.0.0)')
    await sleep(500)
    addLog('info', '✓ Cargo 已安装 (v1.75.0)')
    setProgress(50)

    await sleep(500)

    // Step 3: Git 初始化
    addLog('info', '步骤 3/4: 初始化 Git 仓库')
    setStatus('InitializingGit')
    await sleep(1000)
    addLog('info', '✓ Git 仓库初始化完成')
    addLog('info', '✓ 已创建 .gitignore 文件')
    setProgress(75)

    await sleep(500)

    // Step 4: 任务分解
    addLog('info', '步骤 4/4: 分解任务为 Issues')
    setStatus('DecomposingTasks')
    await sleep(1500)
    addLog('info', '✓ 任务分解完成，共 15 个 Issues')
    addLog('info', '✓ 优先级分布：P0(5), P1(7), P2(3)')
    setProgress(90)

    await sleep(500)

    // HITL 等待
    addLog('info', '等待 HITL 审查...')
    setStatus('WaitingForHITL')
    await sleep(1000)

    setProgress(100)
  }

  /**
   * 辅助函数：休眠
   */
  const sleep = (ms: number) => new Promise(resolve => setTimeout(resolve, ms))

  /**
   * 获取工作流步骤
   */
  const getWorkflowSteps = (): WorkflowStep[] => {
    const steps: WorkflowStep[] = [
      {
        id: 1,
        title: 'PRD 解析',
        description: '解析产品需求文档',
        status: getStatusForStep(1),
      },
      {
        id: 2,
        title: '环境检查',
        description: '验证开发环境配置',
        status: getStatusForStep(2),
      },
      {
        id: 3,
        title: 'Git 初始化',
        description: '创建 Git 仓库',
        status: getStatusForStep(3),
      },
      {
        id: 4,
        title: '任务分解',
        description: '分解为可执行的 Issues',
        status: getStatusForStep(4),
      },
    ]
    return steps
  }

  /**
   * 获取步骤状态
   */
  const getStatusForStep = (stepId: number): 'pending' | 'active' | 'completed' | 'failed' => {
    if (status === 'Failed') return 'failed'

    const currentStep = getCurrentStepNumber()

    if (currentStep > stepId) return 'completed'
    if (currentStep === stepId) return 'active'
    return 'pending'
  }

  /**
   * 获取当前步骤编号
   */
  const getCurrentStepNumber = (): number => {
    switch (status) {
      case 'ParsingPRD':
        return 1
      case 'CheckingEnvironment':
        return 2
      case 'InitializingGit':
        return 3
      case 'DecomposingTasks':
      case 'WaitingForHITL':
        return 4
      case 'Completed':
        return 4
      default:
        return 0
    }
  }

  /**
   * 渲染步骤图标
   */
  const renderStepIcon = (stepStatus: WorkflowStep['status']) => {
    switch (stepStatus) {
      case 'completed':
        return <CheckCircle2 className="w-5 h-5 text-green-500" />
      case 'active':
        return <Loader2 className="w-5 h-5 text-blue-500 animate-spin" />
      case 'failed':
        return <AlertCircle className="w-5 h-5 text-red-500" />
      default:
        return <Circle className="w-5 h-5 text-gray-300" />
    }
  }

  /**
   * 渲染步骤卡片
   */
  const renderStepCard = (step: WorkflowStep) => {
    const isActive = step.status === 'active'
    const isCompleted = step.status === 'completed'
    const isFailed = step.status === 'failed'

    return (
      <div
        key={step.id}
        className={`flex flex-col items-center flex-1 ${isActive ? 'text-blue-600' : isCompleted ? 'text-green-600' : isFailed ? 'text-red-600' : 'text-gray-400'}`}
      >
        <div className="mb-2">{renderStepIcon(step.status)}</div>
        <div className="text-sm font-medium">{step.title}</div>
        <div className="text-xs mt-1 text-center">{step.description}</div>
      </div>
    )
  }

  /**
   * 渲染进度条
   */
  const renderProgressBar = () => (
    <div className="w-full bg-gray-200 rounded-full h-2.5 mb-4">
      <div
        className="bg-blue-600 h-2.5 rounded-full transition-all duration-500"
        style={{ width: `${progress}%` }}
      />
    </div>
  )

  /**
   * 渲染日志面板
   */
  const renderLogPanel = () => (
    <div className="border rounded-lg p-4 bg-gray-50 font-mono text-sm h-64 overflow-y-auto">
      <div className="flex items-center gap-2 mb-2 text-gray-600">
        <Terminal className="w-4 h-4" />
        <span className="font-sans font-medium">运行日志</span>
      </div>
      <div className="space-y-1">
        {logs.length === 0 ? (
          <div className="text-gray-400 italic">暂无日志</div>
        ) : (
          logs.map(log => (
            <div
              key={log.id}
              className={`${
                log.level === 'error'
                  ? 'text-red-600'
                  : log.level === 'warn'
                    ? 'text-orange-600'
                    : 'text-gray-700'
              }`}
            >
              <span className="text-gray-400 text-xs">[{log.timestamp.toLocaleTimeString()}]</span>{' '}
              <span className="font-medium">[{log.level.toUpperCase()}]</span> {log.message}
            </div>
          ))
        )}
      </div>
    </div>
  )

  /**
   * 渲染状态卡片
   */
  const renderStatusCard = () => (
    <div className="flex justify-between items-center p-4 bg-white border rounded-lg">
      <div className="flex items-center gap-3">
        {isRunning ? (
          <>
            <Loader2 className="w-5 h-5 text-blue-500 animate-spin" />
            <span className="text-blue-600 font-medium">运行中...</span>
          </>
        ) : status === 'Completed' ? (
          <>
            <CheckCircle2 className="w-5 h-5 text-green-500" />
            <span className="text-green-600 font-medium">已完成</span>
          </>
        ) : status === 'Failed' ? (
          <>
            <AlertCircle className="w-5 h-5 text-red-500" />
            <span className="text-red-600 font-medium">失败</span>
          </>
        ) : (
          <>
            <Circle className="w-5 h-5 text-gray-400" />
            <span className="text-gray-600 font-medium">未开始</span>
          </>
        )}
      </div>
      <div className="text-sm text-gray-600">进度：{progress}%</div>
    </div>
  )

  /**
   * 渲染开始按钮
   */
  const renderStartButton = () => (
    <button
      onClick={startInitialization}
      disabled={isRunning}
      className={`w-full py-3 px-4 rounded-lg font-medium flex items-center justify-center gap-2 transition-colors ${
        isRunning
          ? 'bg-gray-100 text-gray-400 cursor-not-allowed'
          : 'bg-blue-600 text-white hover:bg-blue-700'
      }`}
    >
      <Play className="w-5 h-5" />
      {isRunning ? '初始化中...' : '开始初始化'}
    </button>
  )

  /**
   * 渲染标题
   */
  const renderTitle = () => <h2 className="text-2xl font-bold text-gray-800">Initializer Agent</h2>

  // 自动启动逻辑 - 使用单独的 effect，避免与手动启动冲突
  useEffect(() => {
    if (autoStart && status === 'Idle') {
      // 延迟一点执行，避免渲染冲突
      const timer = setTimeout(() => {
        startInitialization()
      }, 100)
      return () => clearTimeout(timer)
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [autoStart]) // 故意省略 startInitialization 和 status 依赖

  return (
    <div className="p-6 space-y-4">
      {/* 标题 */}
      {renderTitle()}

      {/* 四步工作流 */}
      <div className="flex justify-between gap-4">{getWorkflowSteps().map(renderStepCard)}</div>

      {/* 进度条 */}
      {isRunning || status === 'Completed' ? renderProgressBar() : null}

      {/* 状态卡片 */}
      {renderStatusCard()}

      {/* 日志面板 */}
      {renderLogPanel()}

      {/* 开始按钮 - 完成后也可以重新开始 */}
      {!autoStart && renderStartButton()}
    </div>
  )
}

/**
 * 默认导出
 */
export default InitializerWorkflow
