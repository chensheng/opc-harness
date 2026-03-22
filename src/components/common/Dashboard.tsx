import { useNavigate } from 'react-router-dom'
import { Plus, Folder, TrendingUp, Code, Lightbulb } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Progress } from '@/components/ui/progress'
import { Badge } from '@/components/ui/badge'
import { useProjectStore } from '@/stores'
import { formatDate } from '@/lib/utils'

const statusLabels: Record<string, string> = {
  idea: '构思中',
  design: '设计中',
  coding: '开发中',
  marketing: '运营中',
  completed: '已完成',
}

const statusColors: Record<string, string> = {
  idea: 'bg-yellow-500',
  design: 'bg-blue-500',
  coding: 'bg-purple-500',
  marketing: 'bg-green-500',
  completed: 'bg-gray-500',
}

export function Dashboard() {
  const navigate = useNavigate()
  const { projects } = useProjectStore()

  const stats = {
    total: projects.length,
    inProgress: projects.filter(p => p.status !== 'completed').length,
    completed: projects.filter(p => p.status === 'completed').length,
  }

  return (
    <div className="space-y-6">
      {/* Stats Cards */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        <Card>
          <CardHeader className="flex flex-row items-center justify-between pb-2">
            <CardTitle className="text-sm font-medium">总项目</CardTitle>
            <Folder className="w-4 h-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{stats.total}</div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between pb-2">
            <CardTitle className="text-sm font-medium">进行中</CardTitle>
            <Code className="w-4 h-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{stats.inProgress}</div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between pb-2">
            <CardTitle className="text-sm font-medium">已完成</CardTitle>
            <TrendingUp className="w-4 h-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{stats.completed}</div>
          </CardContent>
        </Card>
      </div>

      {/* Quick Actions */}
      <Card>
        <CardHeader>
          <CardTitle>快速开始</CardTitle>
        </CardHeader>
        <CardContent className="flex gap-4">
          <Button onClick={() => navigate('/idea')} className="flex-1">
            <Lightbulb className="w-4 h-4 mr-2" />
            创建新项目
          </Button>
        </CardContent>
      </Card>

      {/* Recent Projects */}
      <Card>
        <CardHeader>
          <CardTitle>最近项目</CardTitle>
        </CardHeader>
        <CardContent>
          {projects.length === 0 ? (
            <div className="text-center py-8 text-muted-foreground">
              <p>还没有项目，点击上方按钮创建你的第一个项目</p>
            </div>
          ) : (
            <div className="space-y-4">
              {projects.slice(0, 5).map(project => (
                <div
                  key={project.id}
                  className="flex items-center justify-between p-4 border rounded-lg hover:bg-accent cursor-pointer"
                  onClick={() => navigate(`/prd/${project.id}`)}
                >
                  <div className="flex-1">
                    <div className="flex items-center gap-2">
                      <h3 className="font-medium">{project.name}</h3>
                      <Badge
                        variant="secondary"
                        className={`${statusColors[project.status]} text-white`}
                      >
                        {statusLabels[project.status]}
                      </Badge>
                    </div>
                    <p className="text-sm text-muted-foreground mt-1">
                      {project.description}
                    </p>
                    <div className="flex items-center gap-4 mt-2 text-xs text-muted-foreground">
                      <span>创建于 {formatDate(project.createdAt)}</span>
                      <span>进度: {project.progress}%</span>
                    </div>
                  </div>
                  <div className="w-32">
                    <Progress value={project.progress} />
                  </div>
                </div>
              ))}
            </div>
          )}
        </CardContent>
      </Card>
    </div>
  )
}
