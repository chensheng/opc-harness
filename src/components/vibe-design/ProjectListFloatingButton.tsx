import { useState, useMemo } from 'react'
import { useNavigate } from 'react-router-dom'
import { FolderOpen, ChevronRight, FileText, Calendar, TrendingUp, Search } from 'lucide-react'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog'
import { ScrollArea } from '@/components/ui/scroll-area'
import { Badge } from '@/components/ui/badge'
import { Card, CardContent } from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { useProjectStore } from '@/stores'
import type { Project } from '@/types'

interface ProjectListFloatingButtonProps {
  className?: string
}

export function ProjectListFloatingButton({ className = '' }: ProjectListFloatingButtonProps) {
  const navigate = useNavigate()
  const { projects, loadProjectsFromDatabase } = useProjectStore()
  const [open, setOpen] = useState(false)
  
  // 筛选和分页状态
  const [searchKeyword, setSearchKeyword] = useState('')
  const [statusFilter, setStatusFilter] = useState<string>('all')
  const [currentPage, setCurrentPage] = useState(1)
  const pageSize = 10

  // 当对话框打开时，加载最新的项目列表
  const handleOpenChange = async (newOpen: boolean) => {
    setOpen(newOpen)
    if (newOpen) {
      await loadProjectsFromDatabase()
      // 重置筛选和分页状态
      setSearchKeyword('')
      setStatusFilter('all')
      setCurrentPage(1)
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

  // 筛选和搜索项目
  const filteredProjects = useMemo(() => {
    return projects.filter(project => {
      // 状态筛选
      if (statusFilter !== 'all' && project.status !== statusFilter) {
        return false
      }
      
      // 关键词搜索
      if (searchKeyword.trim()) {
        const keyword = searchKeyword.toLowerCase()
        const matchName = project.name.toLowerCase().includes(keyword)
        const matchDescription = project.description?.toLowerCase().includes(keyword)
        return matchName || matchDescription
      }
      
      return true
    })
  }, [projects, statusFilter, searchKeyword])

  // 计算分页
  const totalPages = Math.ceil(filteredProjects.length / pageSize)
  const startIndex = (currentPage - 1) * pageSize
  const endIndex = startIndex + pageSize
  const paginatedProjects = filteredProjects.slice(startIndex, endIndex)

  // 重置到第一页
  const handleFilterChange = () => {
    setCurrentPage(1)
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
              共 {filteredProjects.length} 个项目
            </Badge>
          </div>
        </DialogHeader>

        {/* 筛选和搜索区域 */}
        <div className="px-6 py-2 border-b space-y-2">
          <div className="flex items-center gap-2">
            {/* 搜索框 */}
            <div className="relative flex-1">
              <Search className="absolute left-2.5 top-1/2 transform -translate-y-1/2 h-3.5 w-3.5 text-muted-foreground" />
              <Input
                placeholder="搜索项目..."
                value={searchKeyword}
                onChange={e => {
                  setSearchKeyword(e.target.value)
                  handleFilterChange()
                }}
                className="pl-8 h-8 text-sm"
              />
            </div>
            
            {/* 状态筛选 */}
            <Select
              value={statusFilter}
              onValueChange={value => {
                setStatusFilter(value)
                handleFilterChange()
              }}
            >
              <SelectTrigger className="w-[140px] h-8 text-sm">
                <SelectValue placeholder="全部状态" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="all">全部状态</SelectItem>
                <SelectItem value="idea">创意阶段</SelectItem>
                <SelectItem value="design">设计阶段</SelectItem>
                <SelectItem value="coding">开发阶段</SelectItem>
                <SelectItem value="marketing">营销阶段</SelectItem>
                <SelectItem value="completed">已完成</SelectItem>
              </SelectContent>
            </Select>
          </div>
        </div>

        <ScrollArea className="max-h-[calc(80vh-220px)] px-6 py-4">
          {paginatedProjects.length === 0 ? (
            <div className="text-center py-12">
              <FileText className="h-12 w-12 mx-auto text-muted-foreground mb-4" />
              <p className="text-muted-foreground">
                {projects.length === 0 ? '暂无项目' : '没有找到匹配的项目'}
              </p>
              {projects.length === 0 && (
                <Button
                  onClick={() => {
                    setOpen(false)
                    navigate('/')
                  }}
                  className="mt-4"
                >
                  创建新项目
                </Button>
              )}
            </div>
          ) : (
            <div className="space-y-3">
              {paginatedProjects.map(project => {
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

        {/* 分页控件 */}
        {totalPages > 1 && (
          <div className="px-6 py-3 border-t flex items-center justify-between">
            <div className="text-sm text-muted-foreground">
              第 {currentPage} / {totalPages} 页
            </div>
            <div className="flex items-center gap-2">
              <Button
                variant="outline"
                size="sm"
                onClick={() => setCurrentPage(prev => Math.max(1, prev - 1))}
                disabled={currentPage === 1}
              >
                上一页
              </Button>
              <Button
                variant="outline"
                size="sm"
                onClick={() => setCurrentPage(prev => Math.min(totalPages, prev + 1))}
                disabled={currentPage === totalPages}
              >
                下一页
              </Button>
            </div>
          </div>
        )}
      </DialogContent>
    </Dialog>
  )
}
