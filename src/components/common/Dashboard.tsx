import { useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { Folder, TrendingUp, Code, Lightbulb, Trash2 } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Progress } from '@/components/ui/progress'
import { Badge } from '@/components/ui/badge'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { useProjectStore } from '@/stores'
import { formatDate } from '@/lib/utils'
import { ProjectListFloatingButton } from '@/components/vibe-design/ProjectListFloatingButton'

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
  const { projects, deleteProject } = useProjectStore()

  // 删除确认状态
  const [deleteDialogOpen, setDeleteDialogOpen] = useState(false)
  const [projectToDelete, setProjectToDelete] = useState<{ id: string; name: string } | null>(null)

  const stats = {
    total: projects.length,
    inProgress: projects.filter(p => p.status !== 'completed').length,
    completed: projects.filter(p => p.status === 'completed').length,
  }

  // 打开删除确认对话框
  const handleDeleteClick = (e: React.MouseEvent, projectId: string, projectName: string) => {
    e.stopPropagation() // 阻止触发卡片点击事件
    setProjectToDelete({ id: projectId, name: projectName })
    setDeleteDialogOpen(true)
  }

  // 确认删除
  const handleConfirmDelete = async () => {
    if (projectToDelete) {
      try {
        await deleteProject(projectToDelete.id)
        console.log(`[Dashboard] Project "${projectToDelete.name}" deleted successfully`)
      } catch (error) {
        console.error('[Dashboard] Failed to delete project:', error)
      } finally {
        setDeleteDialogOpen(false)
        setProjectToDelete(null)
      }
    }
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
                  className="flex items-center justify-between p-4 border rounded-lg hover:bg-accent cursor-pointer group"
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
                    <p className="text-sm text-muted-foreground mt-1">{project.description}</p>
                    <div className="flex items-center gap-4 mt-2 text-xs text-muted-foreground">
                      <span>创建于 {formatDate(project.createdAt)}</span>
                      <span>进度: {project.progress}%</span>
                    </div>
                  </div>
                  <div className="flex items-center gap-4">
                    <div className="w-32">
                      <Progress value={project.progress} />
                    </div>
                    <Button
                      variant="ghost"
                      size="icon"
                      className="opacity-0 group-hover:opacity-100 transition-opacity text-destructive hover:text-destructive hover:bg-destructive/10"
                      onClick={e => handleDeleteClick(e, project.id, project.name)}
                      title="删除项目"
                    >
                      <Trash2 className="w-4 h-4" />
                    </Button>
                  </div>
                </div>
              ))}
            </div>
          )}
        </CardContent>
      </Card>

      {/* 删除确认对话框 */}
      <Dialog open={deleteDialogOpen} onOpenChange={setDeleteDialogOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>确认删除项目</DialogTitle>
            <DialogDescription>
              您确定要删除项目 "{projectToDelete?.name}"
              吗？此操作不可撤销，所有相关数据都将被永久删除。
            </DialogDescription>
          </DialogHeader>
          <DialogFooter>
            <Button variant="outline" onClick={() => setDeleteDialogOpen(false)}>
              取消
            </Button>
            <Button variant="destructive" onClick={handleConfirmDelete}>
              删除
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      {/* 项目列表悬浮按钮 */}
      <ProjectListFloatingButton />
    </div>
  )
}
