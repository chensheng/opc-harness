import React from 'react'
import { useTaskDecomposition } from '../hooks/useTaskDecomposition'
import { Card, CardContent, CardHeader, CardTitle } from './ui/card'
import { Badge } from './ui/badge'
import { Button } from './ui/button'
import type { PrdAnalysis, TechnicalTask, TaskDependencyGraph } from '../types'

interface TaskDependencyGraphPanelProps {
  /** PRD 分析结果 */
  analysis: PrdAnalysis
}

/**
 * 任务依赖图面板组件
 */
export function TaskDependencyGraphPanel({ analysis }: TaskDependencyGraphPanelProps) {
  const { taskGraph, loading, error, decompose, reset } = useTaskDecomposition()

  // 自动执行分解
  React.useEffect(() => {
    if (analysis && !taskGraph) {
      decompose(analysis)
    }
  }, [analysis])

  if (loading) {
    return (
      <div className="w-full h-64 flex items-center justify-center">
        <div className="text-center">
          <div className="animate-spin mb-4">🔄</div>
          <p className="text-muted-foreground">正在分解任务...</p>
        </div>
      </div>
    )
  }

  if (error) {
    return (
      <Card className="border-red-500">
        <CardHeader>
          <CardTitle className="text-red-500">❌ 分解失败</CardTitle>
        </CardHeader>
        <CardContent>
          <p className="text-destructive">{error}</p>
          <Button onClick={() => decompose(analysis)} className="mt-4">
            重试
          </Button>
        </CardContent>
      </Card>
    )
  }

  if (!taskGraph) {
    return (
      <div className="w-full h-64 flex items-center justify-center">
        <Button onClick={() => decompose(analysis)}>开始分解</Button>
      </div>
    )
  }

  return (
    <div className="space-y-6">
      {/* 统计概览 */}
      <Card>
        <CardHeader>
          <CardTitle>📊 任务分解概览</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-2 md:grid-cols-5 gap-4">
            <StatCard title="总任务数" value={taskGraph.statistics.totalTasks} icon="🎯" />
            <StatCard title="前端任务" value={taskGraph.statistics.frontendTasks} icon="🎨" />
            <StatCard title="后端任务" value={taskGraph.statistics.backendTasks} icon="⚙️" />
            <StatCard title="数据库任务" value={taskGraph.statistics.databaseTasks} icon="💾" />
            <StatCard
              title="预估工时"
              value={`${taskGraph.totalEstimatedHours.toFixed(1)}h`}
              icon="⏱️"
            />
          </div>
        </CardContent>
      </Card>

      {/* 关键路径 */}
      {taskGraph.criticalPath.length > 0 && (
        <Card>
          <CardHeader>
            <CardTitle>🔑 关键路径</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="flex items-center gap-2 flex-wrap">
              {taskGraph.criticalPath.map((taskId, index) => (
                <React.Fragment key={taskId}>
                  <Badge variant="default" className="text-sm py-1 px-3">
                    {taskId}
                  </Badge>
                  {index < taskGraph.criticalPath.length - 1 && (
                    <span className="text-muted-foreground">→</span>
                  )}
                </React.Fragment>
              ))}
            </div>
          </CardContent>
        </Card>
      )}

      {/* 任务列表 */}
      <Card>
        <CardHeader>
          <CardTitle>📋 任务清单 ({taskGraph.tasks.length})</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="space-y-2 max-h-[600px] overflow-y-auto">
            {taskGraph.tasks.map(task => (
              <TaskItem key={task.id} task={task} />
            ))}
          </div>
        </CardContent>
      </Card>

      {/* 依赖关系边 */}
      {taskGraph.edges.length > 0 && (
        <Card>
          <CardHeader>
            <CardTitle>🔗 依赖关系 ({taskGraph.edges.length})</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="space-y-2">
              {taskGraph.edges.map((edge, index) => (
                <div key={index} className="flex items-center gap-2">
                  <Badge variant="outline">{edge.fromTask}</Badge>
                  <span className="text-muted-foreground">→</span>
                  <Badge variant="outline">{edge.toTask}</Badge>
                  <span className="text-sm text-muted-foreground">
                    ({edge.dependencyType}, {edge.strength})
                  </span>
                </div>
              ))}
            </div>
          </CardContent>
        </Card>
      )}

      {/* 操作按钮 */}
      <div className="flex justify-end gap-2">
        <Button variant="outline" onClick={reset}>
          重置
        </Button>
        <Button onClick={() => decompose(analysis)}>重新分解</Button>
      </div>
    </div>
  )
}

/**
 * 统计卡片组件
 */
function StatCard({ title, value, icon }: { title: string; value: number | string; icon: string }) {
  return (
    <div className="bg-muted/50 rounded-lg p-4 text-center">
      <div className="text-2xl mb-2">{icon}</div>
      <div className="text-2xl font-bold">{value}</div>
      <div className="text-sm text-muted-foreground">{title}</div>
    </div>
  )
}

/**
 * 任务项组件
 */
function TaskItem({ task }: { task: TechnicalTask }) {
  const getTypeColor = (type: string): string => {
    switch (type) {
      case 'frontend':
        return 'bg-blue-500'
      case 'backend':
        return 'bg-green-500'
      case 'database':
        return 'bg-purple-500'
      case 'testing':
        return 'bg-yellow-500'
      case 'documentation':
        return 'bg-gray-500'
      case 'deployment':
        return 'bg-red-500'
      default:
        return 'bg-gray-500'
    }
  }

  const getTypeLabel = (type: string): string => {
    switch (type) {
      case 'frontend':
        return '前端'
      case 'backend':
        return '后端'
      case 'database':
        return '数据库'
      case 'testing':
        return '测试'
      case 'documentation':
        return '文档'
      case 'deployment':
        return '部署'
      default:
        return '未知'
    }
  }

  return (
    <div className="border rounded-lg p-3 hover:bg-muted/50">
      <div className="flex items-start justify-between mb-2">
        <div className="flex-1">
          <div className="font-semibold mb-1">{task.title}</div>
          <div className="text-sm text-muted-foreground">{task.description}</div>
        </div>
        <div className="flex gap-2 ml-4">
          <Badge variant="secondary" className={getTypeColor(task.taskType)}>
            {getTypeLabel(task.taskType)}
          </Badge>
          <Badge variant="outline">复杂度：{task.complexity}</Badge>
        </div>
      </div>
      <div className="flex flex-wrap gap-4 text-sm">
        <span>优先级：{task.priority}</span>
        <span>预估：{task.estimatedHours}h</span>
        <span>ID: {task.id}</span>
        {task.dependencies.length > 0 && <span>依赖：{task.dependencies.join(', ')}</span>}
      </div>
      {task.skills.length > 0 && (
        <div className="flex gap-1 mt-2 flex-wrap">
          {task.skills.map((skill, index) => (
            <Badge key={index} variant="secondary" className="text-xs">
              {skill}
            </Badge>
          ))}
        </div>
      )}
    </div>
  )
}
