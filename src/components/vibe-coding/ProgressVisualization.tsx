import { useState } from 'react'
import {
  RefreshCw,
  Target,
  CheckCircle,
  Clock,
  EyeIcon,
  TrendingUp,
  Calendar,
  BarChart3,
  PieChart,
} from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import type { Milestone, TaskStats } from './CodingWorkspaceTypes'

export function ProgressVisualization() {
  // Mock data - will be replaced with real data from Backend
  const [milestones] = useState<Milestone[]>([
    {
      id: 'm-001',
      title: '环境初始化',
      progress: 100,
      totalTasks: 5,
      completedTasks: 5,
      status: 'completed',
      dueDate: '2026-03-20',
    },
    {
      id: 'm-002',
      title: '用户认证模块',
      progress: 75,
      totalTasks: 8,
      completedTasks: 6,
      status: 'in_progress',
      dueDate: '2026-03-25',
    },
    {
      id: 'm-003',
      title: '项目管理功能',
      progress: 40,
      totalTasks: 10,
      completedTasks: 4,
      status: 'in_progress',
      dueDate: '2026-03-28',
    },
    {
      id: 'm-004',
      title: '数据可视化',
      progress: 0,
      totalTasks: 7,
      completedTasks: 0,
      status: 'pending',
      dueDate: '2026-04-02',
    },
  ])

  const [taskStats] = useState<TaskStats>({
    total: 30,
    todo: 10,
    inProgress: 8,
    review: 4,
    done: 8,
  })

  const overallProgress = Math.round(
    milestones.reduce((sum, m) => sum + m.progress, 0) / milestones.length
  )

  const getStatusColor = (status: Milestone['status']) => {
    switch (status) {
      case 'completed':
        return 'bg-green-500'
      case 'in_progress':
        return 'bg-blue-500'
      case 'pending':
        return 'bg-gray-300 dark:bg-gray-700'
    }
  }

  const getTaskStatusColor = (stage: keyof TaskStats) => {
    const colors = {
      total: 'bg-gray-500',
      todo: 'bg-gray-400',
      inProgress: 'bg-blue-500',
      review: 'bg-yellow-500',
      done: 'bg-green-500',
    }
    return colors[stage] || 'bg-gray-400'
  }

  return (
    <div className="h-full flex flex-col p-6 space-y-6 overflow-auto">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-2xl font-bold">进度可视化</h2>
          <p className="text-muted-foreground mt-1">实时追踪项目整体进度和里程碑完成情况</p>
        </div>
        <Button variant="outline">
          <RefreshCw className="w-4 h-4 mr-2" />
          刷新进度
        </Button>
      </div>

      {/* Overall Progress */}
      <Card className="p-6">
        <div className="space-y-4">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-3">
              <Target className="w-6 h-6 text-blue-500" />
              <span className="text-lg font-semibold">总体进度</span>
            </div>
            <span className="text-3xl font-bold text-blue-600">{overallProgress}%</span>
          </div>

          <div className="relative pt-1">
            <div className="flex mb-2 items-center justify-between">
              <div>
                <span className="text-xs font-semibold inline-block py-1 px-2 uppercase rounded-full text-blue-600 bg-blue-200 dark:bg-blue-900 dark:text-blue-300">
                  项目完成度
                </span>
              </div>
              <div className="text-right">
                <span className="text-xs font-semibold inline-block text-blue-600 dark:text-blue-300">
                  {milestones.filter(m => m.status === 'completed').length}/{milestones.length}{' '}
                  里程碑
                </span>
              </div>
            </div>
            <div className="overflow-hidden h-4 mb-4 text-xs flex rounded bg-blue-100 dark:bg-blue-900">
              <div
                style={{ width: `${overallProgress}%` }}
                className="shadow-none flex flex-col text-center whitespace-nowrap text-white justify-center bg-blue-500 transition-all duration-500"
              />
            </div>

            {/* Progress Breakdown */}
            <div className="grid grid-cols-3 gap-4 mt-4">
              <div className="text-center">
                <div className="text-2xl font-bold text-green-600">
                  {milestones.filter(m => m.status === 'completed').length}
                </div>
                <div className="text-xs text-muted-foreground">已完成</div>
              </div>
              <div className="text-center">
                <div className="text-2xl font-bold text-blue-600">
                  {milestones.filter(m => m.status === 'in_progress').length}
                </div>
                <div className="text-xs text-muted-foreground">进行中</div>
              </div>
              <div className="text-center">
                <div className="text-2xl font-bold text-gray-600">
                  {milestones.filter(m => m.status === 'pending').length}
                </div>
                <div className="text-xs text-muted-foreground">待开始</div>
              </div>
            </div>
          </div>
        </div>
      </Card>

      {/* Task Statistics */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-5 gap-4">
        {(Object.keys(taskStats) as Array<keyof TaskStats>).map(stage => (
          <Card key={stage} className="p-4">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm text-muted-foreground capitalize">
                  {stage === 'todo'
                    ? '待办'
                    : stage === 'inProgress'
                      ? '进行中'
                      : stage === 'review'
                        ? '审查中'
                        : stage === 'done'
                          ? '已完成'
                          : '总计'}
                </p>
                <p className="text-2xl font-bold mt-1">{taskStats[stage]}</p>
              </div>
              <div
                className={`w-12 h-12 rounded-full ${getTaskStatusColor(stage)} flex items-center justify-center`}
              >
                {stage === 'done' ? (
                  <CheckCircle className="w-6 h-6 text-white" />
                ) : stage === 'inProgress' ? (
                  <Clock className="w-6 h-6 text-white" />
                ) : stage === 'review' ? (
                  <EyeIcon className="w-6 h-6 text-white" />
                ) : (
                  <Target className="w-6 h-6 text-white" />
                )}
              </div>
            </div>
          </Card>
        ))}
      </div>

      {/* Milestone Timeline */}
      <Card className="p-6">
        <div className="space-y-4">
          <div className="flex items-center gap-3">
            <Calendar className="w-6 h-6 text-blue-500" />
            <h3 className="text-lg font-semibold">里程碑时间线</h3>
          </div>

          <div className="space-y-4">
            {milestones.map((milestone, index) => (
              <div key={milestone.id} className="relative">
                {/* Timeline connector */}
                {index < milestones.length - 1 && (
                  <div className="absolute left-6 top-12 bottom-0 w-0.5 bg-gray-200 dark:bg-gray-700" />
                )}

                <div className="flex items-start gap-4">
                  {/* Status indicator */}
                  <div
                    className={`w-12 h-12 rounded-full ${getStatusColor(milestone.status)} flex items-center justify-center flex-shrink-0 z-10`}
                  >
                    {milestone.status === 'completed' ? (
                      <CheckCircle className="w-6 h-6 text-white" />
                    ) : milestone.status === 'in_progress' ? (
                      <TrendingUp className="w-6 h-6 text-white" />
                    ) : (
                      <Clock className="w-6 h-6 text-white" />
                    )}
                  </div>

                  {/* Milestone card */}
                  <div className="flex-1">
                    <Card className="p-4">
                      <div className="space-y-3">
                        <div className="flex items-center justify-between">
                          <div>
                            <h4 className="font-semibold">{milestone.title}</h4>
                            {milestone.dueDate && (
                              <p className="text-sm text-muted-foreground">
                                截止日期：{new Date(milestone.dueDate).toLocaleDateString('zh-CN')}
                              </p>
                            )}
                          </div>
                          <Badge
                            variant={
                              milestone.status === 'completed'
                                ? 'default'
                                : milestone.status === 'in_progress'
                                  ? 'secondary'
                                  : 'outline'
                            }
                          >
                            {milestone.status === 'completed'
                              ? '已完成'
                              : milestone.status === 'in_progress'
                                ? '进行中'
                                : '待开始'}
                          </Badge>
                        </div>

                        {/* Progress bar */}
                        <div>
                          <div className="flex items-center justify-between mb-1">
                            <span className="text-xs text-muted-foreground">
                              任务进度：{milestone.completedTasks}/{milestone.totalTasks}
                            </span>
                            <span className="text-xs font-medium">{milestone.progress}%</span>
                          </div>
                          <div className="w-full bg-gray-200 dark:bg-gray-800 rounded-full h-2">
                            <div
                              className={`h-2 rounded-full transition-all ${
                                milestone.progress === 100
                                  ? 'bg-green-500'
                                  : milestone.progress > 50
                                    ? 'bg-blue-500'
                                    : 'bg-yellow-500'
                              }`}
                              style={{ width: `${milestone.progress}%` }}
                            />
                          </div>
                        </div>

                        {/* Stats */}
                        <div className="grid grid-cols-3 gap-2 text-xs">
                          <div className="text-center">
                            <div className="font-semibold">{milestone.totalTasks}</div>
                            <div className="text-muted-foreground">总任务</div>
                          </div>
                          <div className="text-center">
                            <div className="font-semibold text-green-600">
                              {milestone.completedTasks}
                            </div>
                            <div className="text-muted-foreground">已完成</div>
                          </div>
                          <div className="text-center">
                            <div className="font-semibold text-blue-600">
                              {milestone.totalTasks - milestone.completedTasks}
                            </div>
                            <div className="text-muted-foreground">剩余</div>
                          </div>
                        </div>
                      </div>
                    </Card>
                  </div>
                </div>
              </div>
            ))}
          </div>
        </div>
      </Card>

      {/* Charts Section */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* Burn-down Chart Placeholder */}
        <Card className="p-6">
          <div className="space-y-4">
            <div className="flex items-center gap-3">
              <BarChart3 className="w-6 h-6 text-blue-500" />
              <h3 className="text-lg font-semibold">燃尽图</h3>
            </div>
            <div className="h-64 flex items-center justify-center bg-gray-50 dark:bg-gray-900 rounded-md border-2 border-dashed border-gray-200 dark:border-gray-700">
              <div className="text-center text-muted-foreground">
                <BarChart3 className="w-12 h-12 mx-auto mb-2 opacity-50" />
                <p className="text-sm">燃尽图即将上线</p>
                <p className="text-xs mt-1">展示剩余工作量随时间的变化趋势</p>
              </div>
            </div>
          </div>
        </Card>

        {/* Task Distribution */}
        <Card className="p-6">
          <div className="space-y-4">
            <div className="flex items-center gap-3">
              <PieChart className="w-6 h-6 text-blue-500" />
              <h3 className="text-lg font-semibold">任务分布</h3>
            </div>
            <div className="h-64 flex items-center justify-center bg-gray-50 dark:bg-gray-900 rounded-md border-2 border-dashed border-gray-200 dark:border-gray-700">
              <div className="text-center text-muted-foreground">
                <PieChart className="w-12 h-12 mx-auto mb-2 opacity-50" />
                <p className="text-sm">任务分布图即将上线</p>
                <p className="text-xs mt-1">展示各阶段任务的占比情况</p>
              </div>
            </div>
          </div>
        </Card>
      </div>
    </div>
  )
}
