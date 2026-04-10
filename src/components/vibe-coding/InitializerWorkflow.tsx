/**
 * Initializer Agent UI 组件
 *
 * 展示 Initializer Agent 的四步工作流和实时日志输出
 *
 * @author AI Agent
 * @date 2026-03-25
 */

import { useState, useEffect } from 'react'
import { useNavigate, useParams } from 'react-router-dom'
import {
  FileText,
  Terminal,
  GitBranch,
  ListTodo,
  CheckCircle,
  AlertCircle,
  Loader2,
  ChevronRight,
  Play,
  Square,
} from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import type { InitializerStep } from './CodingWorkspaceTypes'
import { MOCK_STEP_LOGS } from './InitializerWorkflowMocks'

interface InitializerWorkflowProps {
  autoStart?: boolean
}

export function InitializerWorkflow({ autoStart = false }: InitializerWorkflowProps) {
  const { projectId } = useParams<{ projectId: string }>()
  const navigate = useNavigate()
  const [isRunning, setIsRunning] = useState(autoStart)
  const [currentStepIndex, setCurrentStepIndex] = useState(0)

  // 如果 autoStart 为 true，自动开始执行
  useEffect(() => {
    if (autoStart && !isRunning) {
      startInitialization()
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [autoStart])

  const steps: InitializerStep[] = [
    {
      id: 'prd-parsing',
      name: 'PRD 解析',
      description: '分析产品需求文档，提取功能列表和技术栈',
      icon: <FileText className="w-5 h-5" />,
      status: 'pending',
      logs: [],
    },
    {
      id: 'env-check',
      name: '环境检查',
      description: '验证开发环境依赖（Git、Node.js、Rust等）',
      icon: <Terminal className="w-5 h-5" />,
      status: 'pending',
      logs: [],
    },
    {
      id: 'git-init',
      name: 'Git 初始化',
      description: '创建 Git 仓库并配置基本信息',
      icon: <GitBranch className="w-5 h-5" />,
      status: 'pending',
      logs: [],
    },
    {
      id: 'task-decomposition',
      name: '任务分解',
      description: '将 PRD 分解为 Milestones 和 Issues',
      icon: <ListTodo className="w-5 h-5" />,
      status: 'pending',
      logs: [],
    },
  ]

  const [stepsState, setSteps] = useState<InitializerStep[]>(steps)

  const startInitialization = async () => {
    setIsRunning(true)
    setCurrentStepIndex(0)

    for (let i = 0; i < steps.length; i++) {
      await executeStep(i)
      if (i < steps.length - 1) {
        setCurrentStepIndex(i + 1)
      }
    }

    setIsRunning(false)
  }

  const executeStep = async (stepIndex: number) => {
    const step = steps[stepIndex]

    // Update step status to running
    setSteps(prev => prev.map((s, idx) => (idx === stepIndex ? { ...s, status: 'running' } : s)))

    // Simulate step execution with logs
    const logs = MOCK_STEP_LOGS[step.id as keyof typeof MOCK_STEP_LOGS] || []
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

  const allCompleted = stepsState.every(s => s.status === 'completed')

  return (
    <div className="h-full flex flex-col p-6 space-y-6 overflow-auto">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-2xl font-bold">Initializer Agent 工作流</h2>
          <p className="text-muted-foreground mt-1">AI 正在初始化项目环境和任务分解</p>
        </div>
        <div className="flex gap-2">
          {!autoStart && !isRunning && !allCompleted && (
            <Button onClick={startInitialization}>
              <Play className="w-4 h-4 mr-2" />
              开始初始化
            </Button>
          )}
          {isRunning && (
            <Button variant="destructive" onClick={stopInitialization}>
              <Square className="w-4 h-4 mr-2" />
              停止
            </Button>
          )}
        </div>
      </div>

      {/* Steps */}
      <div className="space-y-4">
        {stepsState.map((step, index) => (
          <Card key={step.id} className={`border-l-4 ${getStepColor(step.status)} p-4`}>
            <div className="flex items-start gap-4">
              {/* Step Icon */}
              <div className="flex-shrink-0">{getStepIcon(step.status, step.icon)}</div>

              {/* Step Content */}
              <div className="flex-1 min-w-0">
                <div className="flex items-center justify-between mb-2">
                  <div>
                    <h3 className="font-semibold">{step.name}</h3>
                    <p className="text-sm text-muted-foreground">{step.description}</p>
                  </div>
                  <Badge
                    variant={
                      step.status === 'completed'
                        ? 'default'
                        : step.status === 'running'
                          ? 'secondary'
                          : step.status === 'failed'
                            ? 'destructive'
                            : 'outline'
                    }
                  >
                    {step.status === 'completed'
                      ? '已完成'
                      : step.status === 'running'
                        ? '执行中'
                        : step.status === 'failed'
                          ? '失败'
                          : '待执行'}
                  </Badge>
                </div>

                {/* Logs */}
                {step.logs.length > 0 && (
                  <div className="mt-3 bg-black/5 dark:bg-white/5 rounded-md p-3 font-mono text-xs max-h-32 overflow-y-auto space-y-1">
                    {step.logs.map((log, idx) => (
                      <div key={idx} className="text-gray-700 dark:text-gray-300">
                        {log}
                      </div>
                    ))}
                  </div>
                )}

                {/* Retry Button */}
                {step.status === 'failed' && (
                  <Button
                    size="sm"
                    variant="outline"
                    className="mt-3"
                    onClick={() => retryStep(index)}
                  >
                    重试
                  </Button>
                )}
              </div>
            </div>
          </Card>
        ))}
      </div>

      {/* Completion Card */}
      {allCompleted && (
        <Card className="border-2 border-green-500 bg-green-50 dark:bg-green-950/20 p-6">
          <div className="flex items-center justify-between">
            <div>
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

export default InitializerWorkflow
