import { useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { FolderOpen, X, ChevronRight, FileText, Calendar, TrendingUp } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogTrigger } from '@/components/ui/dialog'
import { ScrollArea } from '@/components/ui/scroll-area'
import { Badge } from '@/components/ui/badge'
import { Card, CardContent } from '@/components/ui/card'
import { useProjectStore } from '@/stores'
import type { Project } from '@/types'

interface ProjectListFloatingButtonProps {
  className?: string
}

export function ProjectListFloatingButton({ className = '' }: ProjectListFloatingButtonProps) {
  const navigate = useNavigate()
  const { projects, loadProjectsFromDatabase } = useProjectStore()
  const [open, setOpen] = useState(false)

  // 当对话框打开时，加载最新的项目列表
  const handleOpenChange = async (newOpen: boolean) => {
    setOpen(newOpen)
    if (newOpen) {
      await loadProjectsFromDatabase()
    }
  }

  // 获取状态对应的颜色和文本
  const getStatusInfo = (status: Project['status']) => {
    const statusMap: Record<Project['status'], { color: string; text: string }> = {
      idea: { color: 'bg-blue-500', text: '创意阶段' },
      design: { color: 'bg-purple-500', text: '设计阶段' },
      coding: { color: 'bg-green-500', text: '开发阶段' },
      marketing: { color: 'bg-orange-500', text: '营销阶段' },
      completed: { color: 'bg-emerald-500', text: '已完成' },
    }
    return statusMap[status] || { color: 'bg-gray-500', text: status }
  }

  // 格式化日期
  const formatDate = (dateString: string) => {
    const date = new Date(dateString)
    return date.toLocaleDateString('zh-CN', {
      year: 'numeric',
      month: '2-digit',
      day: '2-digit',
    })
  }

  // 处理项目点击
  const handleProjectClick = (projectId: string) => {
    setOpen(false)
    navigate(`/prd/${projectId}`)
  }

  return (
    <Dialog open={open} onOpenChange={handleOpenChange}>
      <DialogTrigger asChild>
        <Button
          size="icon"
          className={`fixed z-50 rounded-full shadow-lg transition-all duration-300 hover:scale-110 hover:shadow-xl ${className}`}
          style={{
            bottom: 'calc(8rem + env(safe-area-inset-bottom, 0px))',
            right: 'calc(2rem + env(safe-area-inset-right, 0px))',
          }}
          aria-label="项目列表"
        >
          <FolderOpen className="h-5 w-5" />
        </Button>
      </DialogTrigger>
      <DialogContent className="max-w-2xl max-h-[80vh] p-0">
        <DialogHeader className="px-6 pt-6 pb-4 border-b">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <FolderOpen className="h-5 w-5 text-primary" />
              <DialogTitle className="text-xl">项目列表</DialogTitle>
            </div>
            <Badge variant="secondary" className="text-sm">
              共 {projects.length} 个项目
            </Badge>
          </div>
        </DialogHeader>

        <ScrollArea className="max-h-[calc(80vh-120px)] px-6 py-4">
          {projects.length === 0 ? (
            <div className="text-center py-12">
              <FileText className="h-12 w-12 mx-auto text-muted-foreground mb-4" />
              <p className="text-muted-foreground">暂无项目</p>
              <Button
                onClick={() => {
                  setOpen(false)
                  navigate('/')
                }}
                className="mt-4"
              >
                创建新项目
              </Button>
            </div>
          ) : (
            <div className="space-y-3">
              {projects.map(project => {
                const statusInfo = getStatusInfo(project.status)
                return (
                  <Card
                    key={project.id}
                    className="cursor-pointer transition-all duration-200 hover:shadow-md hover:border-primary/50 group"
                    onClick={() => handleProjectClick(project.id)}
                  >
                    <CardContent className="p-4">
                      <div className="flex items-start justify-between gap-4">
                        <div className="flex-1 min-w-0">
                          <div className="flex items-center gap-2 mb-2">
                            <h3 className="font-semibold text-lg truncate group-hover:text-primary transition-colors">
                              {project.name}
                            </h3>
                            <Badge className={`${statusInfo.color} text-white text-xs`}>
                              {statusInfo.text}
                            </Badge>
                          </div>
                          
                          {project.description && (
                            <p className="text-sm text-muted-foreground line-clamp-2 mb-3">
                              {project.description}
                            </p>
                          )}

                          <div className="flex items-center gap-4 text-xs text-muted-foreground">
                            <div className="flex items-center gap-1">
                              <Calendar className="h-3 w-3" />
                              <span>{formatDate(project.createdAt)}</span>
                            </div>
                            <div className="flex items-center gap-1">
                              <TrendingUp className="h-3 w-3" />
                              <span>进度: {project.progress}%</span>
                            </div>
                          </div>

                          {/* 进度条 */}
                          <div className="mt-3 h-1.5 bg-muted rounded-full overflow-hidden">
                            <div
                              className="h-full bg-primary transition-all duration-300"
                              style={{ width: `${project.progress}%` }}
                            />
                          </div>
                        </div>

                        <ChevronRight className="h-5 w-5 text-muted-foreground group-hover:text-primary group-hover:translate-x-1 transition-all flex-shrink-0 mt-1" />
                      </div>
                    </CardContent>
                  </Card>
                )
              })}
            </div>
          )}
        </ScrollArea>
      </DialogContent>
    </Dialog>
  )
}
