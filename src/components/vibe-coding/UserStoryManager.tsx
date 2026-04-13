import { useState } from 'react'
import React from 'react'
import ReactMarkdown from 'react-markdown'
import remarkGfm from 'remark-gfm'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import { Textarea } from '@/components/ui/textarea'
import { ScrollArea } from '@/components/ui/scroll-area'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import {
  Sparkles,
  FileText,
  CheckCircle2,
  Clock,
  AlertCircle,
  Edit2,
  Trash2,
  MessageSquare,
  Loader2,
  ChevronLeft,
  ChevronRight,
  ArrowUpDown,
  ArrowUp,
  ArrowDown,
  Search,
  X,
  Users,
} from 'lucide-react'
import type { UserStory } from '@/types'
import { useUserStoryDecomposition } from '@/hooks/useUserStoryDecomposition'
import { useUserStoryStream } from '@/hooks/useUserStoryStream'
import { useProjectStore } from '@/stores/projectStore'
import { useUserStoryStore } from '@/stores/userStoryStore'
import { useSprintStore } from '@/stores/sprintStore'
import { UserStoryEditDialog } from './UserStoryEditDialog'

/**
 * 拆分配置对话框组件
 */
interface DecomposeDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  prdContent: string
  prompt: string
  onPromptChange: (prompt: string) => void
  onDecompose: () => Promise<void>
  isStreaming: boolean
  markdownContent: string
  error?: string
}

function DecomposeDialog({
  open,
  onOpenChange,
  prdContent,
  prompt,
  onPromptChange,
  onDecompose,
  isStreaming,
  markdownContent,
  error,
}: DecomposeDialogProps) {
  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-6xl h-[85vh] flex flex-col">
        <DialogHeader className="flex-shrink-0">
          <DialogTitle className="flex items-center gap-2">
            <Sparkles className="w-5 h-5" />
            拆分用户故事
          </DialogTitle>
          <DialogDescription>AI 将基于 PRD 内容自动拆分为用户故事</DialogDescription>
        </DialogHeader>

        {/* 主体内容区域 - 使用flex布局确保高度正确传递 */}
        <div className="flex-1 min-h-0 grid grid-cols-2 gap-4">
          {/* PRD Preview - 左侧（可滚动） */}
          <Card className="h-full flex flex-col min-h-0">
            <CardHeader className="pb-2 flex-shrink-0">
              <CardTitle className="flex items-center gap-2 text-base">
                <FileText className="w-4 h-4" />
                项目 PRD
              </CardTitle>
              <CardDescription className="text-xs">AI 将基于此内容拆分</CardDescription>
            </CardHeader>
            <CardContent className="flex-1 min-h-0 overflow-hidden p-0">
              <div className="h-full px-4 pb-4">
                <ScrollArea className="h-full w-full rounded-md border p-3 bg-card" type="always">
                  <div className="min-h-full">
                    {prdContent ? (
                      <div className="prose prose-sm dark:prose-invert max-w-none pr-2">
                        <ReactMarkdown
                          remarkPlugins={[remarkGfm]}
                          components={PRDPreviewComponents}
                        >
                          {prdContent}
                        </ReactMarkdown>
                      </div>
                    ) : (
                      <div className="flex items-center justify-center h-full text-muted-foreground">
                        <div className="text-center">
                          <FileText className="w-8 h-8 mx-auto mb-2 opacity-50" />
                          <p className="text-xs">暂无 PRD 内容</p>
                        </div>
                      </div>
                    )}
                  </div>
                </ScrollArea>
              </div>
            </CardContent>
          </Card>

          {/* Prompt Input - 右侧（固定不滚动） */}
          <Card className="h-full flex flex-col min-h-0">
            <CardHeader className="pb-2 flex-shrink-0">
              <CardTitle className="flex items-center gap-2 text-base">
                <MessageSquare className="w-4 h-4" />
                拆分要求
              </CardTitle>
              <CardDescription className="text-xs">可选的额外要求</CardDescription>
            </CardHeader>
            <CardContent className="flex-1 flex flex-col space-y-3 px-4 pb-4">
              <Textarea
                placeholder="例如：&#10;- 重点关注用户认证&#10;- 优先核心业务流程&#10;- 考虑技术债务..."
                value={prompt}
                onChange={e => onPromptChange(e.target.value)}
                className="min-h-[120px] resize-none text-sm flex-shrink-0"
              />

              {error && (
                <div className="flex items-start gap-2 p-2 bg-red-50 dark:bg-red-950/20 border border-red-200 dark:border-red-800 rounded-md flex-shrink-0">
                  <AlertCircle className="w-4 h-4 text-red-500 mt-0.5 flex-shrink-0" />
                  <div className="text-xs text-red-700 dark:text-red-400">{error}</div>
                </div>
              )}

              {/* 流式响应实时显示 */}
              {isStreaming && (
                <Card className="border-primary/50 bg-gradient-to-br from-primary/5 to-purple-500/5 flex-shrink-0">
                  <CardHeader className="pb-2 pt-2 px-3">
                    <CardTitle className="flex items-center gap-2 text-sm">
                      <Loader2 className="w-4 h-4 animate-spin text-primary" />
                      AI 生成中...
                    </CardTitle>
                  </CardHeader>
                  <CardContent className="px-3 pb-3">
                    <ScrollArea className="h-[200px] w-full rounded-md border p-2 bg-background/50">
                      {markdownContent ? (
                        <div className="prose prose-sm dark:prose-invert max-w-none">
                          <ReactMarkdown
                            remarkPlugins={[remarkGfm]}
                            components={PRDPreviewComponents}
                          >
                            {markdownContent}
                          </ReactMarkdown>
                        </div>
                      ) : (
                        <div className="flex items-center justify-center h-20 text-muted-foreground">
                          <div className="text-center space-y-1">
                            <Loader2 className="w-6 h-6 animate-spin mx-auto" />
                            <p className="text-xs">正在连接 AI...</p>
                          </div>
                        </div>
                      )}
                    </ScrollArea>
                  </CardContent>
                </Card>
              )}

              <Button
                onClick={onDecompose}
                disabled={!prdContent || isStreaming}
                className="w-full flex-shrink-0"
              >
                {isStreaming ? (
                  <>
                    <Sparkles className="w-4 h-4 mr-2 animate-spin" />
                    AI 拆分中...
                  </>
                ) : (
                  <>
                    <Sparkles className="w-4 h-4 mr-2" />
                    开始拆分用户故事
                  </>
                )}
              </Button>

              <div className="text-xs text-muted-foreground space-y-0.5 flex-shrink-0">
                <p>💡 提示：</p>
                <ul className="list-disc list-inside space-y-0.5 ml-1 text-[10px]">
                  <li>AI 自动基于 PRD 拆分</li>
                  <li>可输入额外要求指导拆分</li>
                  <li>遵循 INVEST 原则</li>
                  <li>包含验收标准和优先级</li>
                </ul>
              </div>
            </CardContent>
          </Card>
        </div>
      </DialogContent>
    </Dialog>
  )
}

// PRD 预览的自定义 Markdown 组件
const PRDPreviewComponents = {
  h1: ({ ...props }) => (
    <h1
      className="text-2xl font-bold mb-4 mt-6 pb-2 border-b border-border text-primary"
      {...props}
    />
  ),
  h2: ({ ...props }) => (
    <h2
      className="text-xl font-semibold mb-3 mt-5 pb-1 border-b border-border/50 text-foreground"
      {...props}
    />
  ),
  h3: ({ ...props }) => (
    <h3 className="text-lg font-medium mb-2 mt-4 text-foreground/90" {...props} />
  ),
  p: ({ ...props }) => (
    <p className="text-base leading-relaxed mb-3 text-foreground/90" {...props} />
  ),
  ul: ({ ...props }) => <ul className="list-disc list-outside pl-6 mb-3 space-y-1.5" {...props} />,
  ol: ({ ...props }) => (
    <ol className="list-decimal list-outside pl-6 mb-3 space-y-1.5" {...props} />
  ),
  li: ({ ...props }) => <li className="text-sm leading-relaxed text-foreground/85" {...props} />,
  strong: ({ ...props }) => <strong className="font-semibold text-foreground" {...props} />,
  em: ({ ...props }) => <em className="italic text-foreground/80" {...props} />,
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  code: ({ inline, ...props }: any) =>
    inline ? (
      <code
        className="bg-muted/80 px-1.5 py-0.5 rounded text-xs font-mono text-primary"
        {...props}
      />
    ) : (
      <code
        className="block bg-muted p-2.5 rounded-md my-2 overflow-x-auto text-xs font-mono"
        {...props}
      />
    ),
  pre: ({ ...props }) => (
    <pre
      className="bg-muted/50 p-3 rounded-md my-3 overflow-x-auto border border-border/30"
      {...props}
    />
  ),
  blockquote: ({ ...props }) => (
    <blockquote
      className="border-l-4 border-primary/50 pl-4 py-2 my-3 bg-muted/20 italic text-foreground/75"
      {...props}
    />
  ),
  table: ({ ...props }) => (
    <div className="overflow-x-auto my-6 first:mt-4 last:mb-4">
      <table className="w-full border-collapse border border-border" {...props} />
    </div>
  ),
  th: ({ ...props }) => (
    <th
      className="border border-border px-4 py-3 bg-muted/80 text-left font-semibold text-sm"
      {...props}
    />
  ),
  td: ({ ...props }) => (
    <td className="border border-border px-4 py-3 text-left text-sm" {...props} />
  ),
  tr: ({ ...props }) => (
    <tr className="even:bg-muted/30 hover:bg-muted/50 transition-colors" {...props} />
  ),
}

interface UserStoryManagerProps {
  /** 项目 PRD 内容（必需） */
  prdContent: string
  /** AI API Key（可选） */
  apiKey?: string
  /** 拆分完成后的回调 */
  onStoriesGenerated?: (stories: UserStory[]) => void
}

const priorityColors: Record<string, string> = {
  P0: 'bg-red-500 text-white',
  P1: 'bg-orange-500 text-white',
  P2: 'bg-yellow-500 text-white',
  P3: 'bg-gray-500 text-white',
}

const statusColors: Record<string, string> = {
  draft: 'bg-gray-100 text-gray-700 border border-gray-300',
  refined: 'bg-blue-50 text-blue-700 border border-blue-200',
  approved: 'bg-green-50 text-green-700 border border-green-200',
  in_development: 'bg-purple-50 text-purple-700 border border-purple-200',
  completed: 'bg-emerald-50 text-emerald-700 border border-emerald-200',
}

const statusLabels: Record<string, string> = {
  draft: '草稿',
  refined: '已细化',
  approved: '已批准',
  in_development: '开发中',
  completed: '已完成',
}

const _statusIcons: Record<string, React.ReactNode> = {
  draft: <Clock className="w-3 h-3" />,
  refined: <Edit2 className="w-3 h-3" />,
  approved: <CheckCircle2 className="w-3 h-3" />,
  in_development: <Loader2 className="w-3 h-3 animate-spin" />,
  completed: <CheckCircle2 className="w-3 h-3" />,
}

/**
 * 用户故事管理组件
 *
 * 提供通过 AI 拆分 PRD 为用户故事的功能，并支持故事的查看、编辑和管理
 */
export function UserStoryManager({
  prdContent,
  apiKey: _apiKey, // 保留参数用于未来扩展,当前从 AI 配置 Store 自动获取
  onStoriesGenerated,
}: UserStoryManagerProps) {
  const [prompt, setPrompt] = useState('')
  const [showDecomposeDialog, setShowDecomposeDialog] = useState(false)

  // 分页状态
  const [currentPage, setCurrentPage] = useState(1)
  const [pageSize, setPageSize] = useState(10) // 每页显示10个故事

  // 排序状态（支持多条件排序）
  type SortField = 'priority' | 'storyPoints'
  const [sortConfigs, setSortConfigs] = useState<
    Array<{ field: SortField; order: 'asc' | 'desc' }>
  >([])

  // 筛选状态
  const [filterKeyword, setFilterKeyword] = useState('') // 关键词筛选
  const [filterStatus, setFilterStatus] = useState<string>('') // 状态筛选
  const [filterPriority, setFilterPriority] = useState<string>('') // 优先级筛选
  const [filterSprint, setFilterSprint] = useState<string>('') // Sprint筛选

  // 编辑对话框状态
  const [editingStory, setEditingStory] = useState<UserStory | null>(null)
  const [showEditDialog, setShowEditDialog] = useState(false)

  // 删除确认对话框状态
  const [deletingStory, setDeletingStory] = useState<UserStory | null>(null)
  const [showDeleteConfirm, setShowDeleteConfirm] = useState(false)
  const [isDeleting, setIsDeleting] = useState(false)

  // 快速编辑 Sprint 的状态
  const [editingSprintStoryId, setEditingSprintStoryId] = useState<string | null>(null)

  // 使用流式 Hook
  const {
    markdownContent,
    userStories: streamUserStories,
    isStreaming,
    isComplete: _isComplete,
    error: streamError,
    startStream,
    reset: resetStream,
  } = useUserStoryStream()

  // 使用非流式 Hook (作为后备)
  const {
    userStories: _userStories,
    loading: _loading,
    error: _error,
    decompose: _decompose,
    reset: _reset,
  } = useUserStoryDecomposition()

  // 获取当前项目ID和用户故事Store
  const currentProjectId = useProjectStore(state => state.currentProjectId)
  const loadProjectStories = useUserStoryStore(state => state.loadProjectStories)
  const updateStory = useUserStoryStore(state => state.updateStory)
  const deleteStory = useUserStoryStore(state => state.deleteStory)
  const isLoadingFromDB = useUserStoryStore(state => state.isLoading)

  // 获取Sprint Store
  const loadProjectSprints = useSprintStore(state => state.loadProjectSprints)
  const sprints = useSprintStore(state =>
    currentProjectId ? state.sprintsByProject[currentProjectId] || [] : []
  )

  // 直接从 Store 订阅用户故事(响应式)
  const savedStories = useUserStoryStore(state =>
    currentProjectId ? state.storiesByProject[currentProjectId] || [] : []
  )

  // 组件挂载时加载用户故事和Sprints
  React.useEffect(() => {
    if (currentProjectId) {
      loadProjectStories(currentProjectId)
      loadProjectSprints(currentProjectId)
    }
  }, [currentProjectId, loadProjectStories, loadProjectSprints])

  // 优先使用流式的用户故事，否则使用保存的故事
  const displayError = streamError || _error || undefined
  const displayStories =
    streamUserStories.length > 0
      ? streamUserStories
      : _userStories.length > 0
        ? _userStories
        : savedStories

  const handleDecompose = async () => {
    // 组合 PRD 内容和用户提示词
    const fullContent = prompt.trim() ? `${prdContent}\n\n---\n\n用户要求：${prompt}` : prdContent

    // 从 AI 配置 Store 获取配置
    const { useAIConfigStore } = await import('@/stores/aiConfigStore')
    const store = useAIConfigStore.getState()
    const activeConfig = store.getActiveConfig()

    // CodeFree CLI 不需要 API Key，其他 provider 需要检查
    if (activeConfig?.provider !== 'codefree' && !activeConfig?.apiKey) {
      alert('未配置 AI API Key，请先在设置中配置')
      return
    }

    // 获取已有的用户故事，用于避免重复生成
    let existingStories: Array<{ title: string; role: string; feature: string }> | undefined
    if (currentProjectId && savedStories.length > 0) {
      existingStories = savedStories.map(story => ({
        title: story.title,
        role: story.role,
        feature: story.feature,
      }))
      console.log(
        `[UserStoryManager] Found ${existingStories.length} existing stories to avoid duplication`
      )
    }

    // 隐藏对话框
    setShowDecomposeDialog(false)

    // 开始流式拆分，并在完成后自动保存
    await startStream(
      {
        prdContent: fullContent,
        provider: store.defaultProvider, // ✅ 使用默认厂商而非activeConfig.provider
        model: activeConfig.model,
        apiKey: activeConfig.apiKey || '', // codefree 传空字符串
        projectId: currentProjectId || undefined, // 传递项目 ID 用于 CodeFree 文件写入
        existingStories, // 传递已有用户故事信息
      },
      stories => {
        // 流式完成后自动调用保存回调
        if (onStoriesGenerated) {
          onStoriesGenerated(stories)
        }
      }
    )
  }

  const _handleReset = () => {
    _reset()
    resetStream()
    setPrompt('')
  }

  const _getStoryStats = () => {
    const stories = displayStories || []
    const total = stories.length
    const p0 = stories.filter((s: UserStory) => s.priority === 'P0').length
    const p1 = stories.filter((s: UserStory) => s.priority === 'P1').length
    const avgPoints = stories.reduce((sum, s) => sum + (s.storyPoints || 0), 0) / total || 0

    return { total, p0, p1, avgPoints }
  }

  // 编辑用户故事
  const handleEditStory = (story: UserStory) => {
    setEditingStory(story)
    setShowEditDialog(true)
  }

  // 保存编辑后的用户故事
  const handleSaveStory = async (updatedStory: UserStory) => {
    if (currentProjectId && editingStory) {
      await updateStory(currentProjectId, editingStory.id, updatedStory)
      setShowEditDialog(false)
      setEditingStory(null)
    }
  }

  // 打开删除确认对话框
  const handleDeleteStory = (story: UserStory) => {
    setDeletingStory(story)
    setShowDeleteConfirm(true)
  }

  // 执行删除操作
  const confirmDelete = async () => {
    if (!deletingStory || !currentProjectId) return

    setIsDeleting(true)
    try {
      await deleteStory(currentProjectId, deletingStory.id)
      setShowDeleteConfirm(false)
      setDeletingStory(null)
    } catch (error) {
      console.error('删除用户故事失败:', error)
      alert('删除失败，请重试')
    } finally {
      setIsDeleting(false)
    }
  }

  // 取消删除
  const cancelDelete = () => {
    setShowDeleteConfirm(false)
    setDeletingStory(null)
  }

  // 快速更新用户故事的 Sprint
  const handleQuickUpdateSprint = async (storyId: string, sprintId: string | undefined) => {
    if (!currentProjectId) return

    try {
      await updateStory(currentProjectId, storyId, { sprintId })
      // 关闭编辑状态
      setEditingSprintStoryId(null)
    } catch (error) {
      console.error('更新 Sprint 失败:', error)
      alert('更新失败，请重试')
    }
  }

  // 取消快速编辑
  const cancelQuickEdit = () => {
    setEditingSprintStoryId(null)
  }

  // 筛选逻辑（在排序之前执行）
  const filteredStories = React.useMemo(() => {
    let result = displayStories

    // 关键词筛选：匹配标题、功能描述、角色
    if (filterKeyword.trim()) {
      const keyword = filterKeyword.toLowerCase()
      result = result.filter(
        (story: UserStory) =>
          story.title.toLowerCase().includes(keyword) ||
          story.feature.toLowerCase().includes(keyword) ||
          story.role.toLowerCase().includes(keyword)
      )
    }

    // 状态筛选
    if (filterStatus) {
      result = result.filter((story: UserStory) => story.status === filterStatus)
    }

    // 优先级筛选
    if (filterPriority) {
      result = result.filter((story: UserStory) => story.priority === filterPriority)
    }

    // Sprint筛选
    if (filterSprint) {
      if (filterSprint === 'unassigned') {
        // 筛选未分配Sprint的故事
        result = result.filter((story: UserStory) => !story.sprintId)
      } else {
        // 筛选特定Sprint的故事
        result = result.filter((story: UserStory) => story.sprintId === filterSprint)
      }
    }

    return result
  }, [displayStories, filterKeyword, filterStatus, filterPriority, filterSprint])

  // 计算分页数据（先筛选，再排序，最后分页）
  const sortedStories = React.useMemo(() => {
    if (sortConfigs.length === 0) return filteredStories

    return [...filteredStories].sort((a: UserStory, b: UserStory) => {
      let comparison = 0

      for (const config of sortConfigs) {
        if (config.field === 'priority') {
          // 优先级排序：P0 < P1 < P2 < P3
          const priorityOrder: Record<string, number> = { P0: 0, P1: 1, P2: 2, P3: 3 }
          const aPriority = priorityOrder[a.priority] ?? 999
          const bPriority = priorityOrder[b.priority] ?? 999
          comparison = aPriority - bPriority
        } else if (config.field === 'storyPoints') {
          // 故事点排序
          const aPoints = a.storyPoints || 0
          const bPoints = b.storyPoints || 0
          comparison = aPoints - bPoints
        }

        if (comparison !== 0) {
          return config.order === 'asc' ? comparison : -comparison
        }
      }

      return 0
    })
  }, [filteredStories, sortConfigs])

  const totalPages = Math.ceil(sortedStories.length / pageSize)
  const startIndex = (currentPage - 1) * pageSize
  const endIndex = startIndex + pageSize
  const paginatedStories = sortedStories.slice(startIndex, endIndex)

  // 处理排序切换（支持多条件排序）
  const handleSort = (field: SortField, _event?: React.MouseEvent) => {
    const existingConfig = sortConfigs.find(config => config.field === field)

    if (existingConfig) {
      // 如果字段已存在，切换排序方向
      const newOrder = existingConfig.order === 'asc' ? 'desc' : 'asc'
      setSortConfigs(prevConfigs =>
        prevConfigs.map(config =>
          config.field === field ? { ...config, order: newOrder } : config
        )
      )
    } else {
      // 如果字段不存在，添加到排序条件列表
      setSortConfigs(prevConfigs => [...prevConfigs, { field, order: 'asc' }])
    }

    // 重置到第一页
    setCurrentPage(1)
  }

  // 清除所有排序条件
  const clearSort = () => {
    setSortConfigs([])
    setCurrentPage(1)
  }

  // 获取字段的排序配置
  const getSortConfig = (field: SortField) => {
    return sortConfigs.find(config => config.field === field)
  }

  // 获取字段在排序列表中的序号（用于多条件排序显示）
  const getSortIndex = (field: SortField) => {
    const index = sortConfigs.findIndex(config => config.field === field)
    return index >= 0 ? index + 1 : null
  }

  // 清除所有筛选条件
  const clearFilters = () => {
    setFilterKeyword('')
    setFilterStatus('')
    setFilterPriority('')
    setFilterSprint('')
    setCurrentPage(1)
  }

  // 获取所有唯一的筛选选项
  const uniqueStatuses = React.useMemo(() => {
    const statuses = new Set(displayStories.map((s: UserStory) => s.status))
    return Array.from(statuses).sort()
  }, [displayStories])

  const uniquePriorities = React.useMemo(() => {
    const priorities = new Set(displayStories.map((s: UserStory) => s.priority))
    return Array.from(priorities).sort()
  }, [displayStories])

  // 重置分页（当故事列表变化时）
  React.useEffect(() => {
    setCurrentPage(1)
  }, [displayStories.length])

  const handlePageChange = (page: number) => {
    setCurrentPage(page)
  }

  const handlePageSizeChange = (size: number) => {
    setPageSize(size)
    setCurrentPage(1) // 重置到第一页
  }

  return (
    <div className="space-y-3">
      {/* Header with Action Buttons */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-2">
          <Users className="w-5 h-5" />
          <h2 className="text-lg font-semibold">用户故事</h2>
          {displayStories.length > 0 && (
            <Badge variant="secondary" className="ml-2">
              {displayStories.length}
            </Badge>
          )}
        </div>
        <div className="flex gap-2">
          <Button size="sm" onClick={() => setShowDecomposeDialog(true)} disabled={!prdContent}>
            <Sparkles className="w-4 h-4 mr-2" />
            拆分用户故事
          </Button>
        </div>
      </div>

      {/* Loading State */}
      {isLoadingFromDB && (
        <Card>
          <CardContent className="flex items-center justify-center py-12">
            <div className="text-center space-y-3">
              <Loader2 className="w-8 h-8 animate-spin mx-auto text-primary" />
              <p className="text-sm text-muted-foreground">正在加载用户故事...</p>
            </div>
          </CardContent>
        </Card>
      )}

      {/* Empty State */}
      {!isLoadingFromDB && (!displayStories || displayStories.length === 0) && (
        <Card>
          <CardContent className="flex flex-col items-center justify-center py-16 space-y-4">
            <div className="w-16 h-16 rounded-full bg-muted/50 flex items-center justify-center">
              <FileText className="w-8 h-8 text-muted-foreground" />
            </div>
            <div className="text-center space-y-2">
              <h3 className="text-lg font-semibold">暂无用户故事</h3>
              <p className="text-sm text-muted-foreground max-w-md">
                {prdContent
                  ? '点击右上角"拆分用户故事"按钮，AI 将基于 PRD 内容自动拆分为用户故事'
                  : '请先在项目设计中生成 PRD，然后再拆分用户故事'}
              </p>
            </div>
            <Button onClick={() => setShowDecomposeDialog(true)} disabled={!prdContent}>
              <Sparkles className="w-4 h-4 mr-2" />
              开始拆分
            </Button>
          </CardContent>
        </Card>
      )}

      {/* Story List - 当有故事时显示 */}
      {displayStories && displayStories.length > 0 && (
        <div className="space-y-2">
          {/* Filter Bar - 筛选栏 */}
          <Card>
            <CardContent className="p-2">
              <div className="flex items-center gap-2">
                {/* 关键词搜索 */}
                <div className="relative flex-1 max-w-xs">
                  <Search className="absolute left-2 top-1/2 -translate-y-1/2 w-3 h-3 text-muted-foreground" />
                  <input
                    type="text"
                    placeholder="搜索标题、功能、角色..."
                    value={filterKeyword}
                    onChange={e => {
                      setFilterKeyword(e.target.value)
                      setCurrentPage(1)
                    }}
                    className="w-full pl-7 pr-7 py-1 text-[10px] border rounded bg-background focus:outline-none focus:ring-1 focus:ring-primary"
                  />
                  {filterKeyword && (
                    <button
                      onClick={() => setFilterKeyword('')}
                      className="absolute right-1.5 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground"
                    >
                      <X className="w-3 h-3" />
                    </button>
                  )}
                </div>

                {/* 状态筛选 */}
                <select
                  value={filterStatus}
                  onChange={e => {
                    setFilterStatus(e.target.value)
                    setCurrentPage(1)
                  }}
                  className="px-2 py-1 text-[10px] border rounded bg-background focus:outline-none focus:ring-1 focus:ring-primary"
                >
                  <option value="">所有状态</option>
                  {uniqueStatuses.map(status => (
                    <option key={status} value={status}>
                      {statusLabels[status] || status}
                    </option>
                  ))}
                </select>

                {/* 优先级筛选 */}
                <select
                  value={filterPriority}
                  onChange={e => {
                    setFilterPriority(e.target.value)
                    setCurrentPage(1)
                  }}
                  className="px-2 py-1 text-[10px] border rounded bg-background focus:outline-none focus:ring-1 focus:ring-primary"
                >
                  <option value="">所有优先级</option>
                  {uniquePriorities.map(priority => (
                    <option key={priority} value={priority}>
                      {priority}
                    </option>
                  ))}
                </select>

                {/* Sprint筛选 */}
                <select
                  value={filterSprint}
                  onChange={e => {
                    setFilterSprint(e.target.value)
                    setCurrentPage(1)
                  }}
                  className="px-2 py-1 text-[10px] border rounded bg-background focus:outline-none focus:ring-1 focus:ring-primary max-w-[150px]"
                  title="按Sprint筛选"
                >
                  <option value="">所有Sprint</option>
                  <option value="unassigned">未分配</option>
                  {sprints.map(sprint => (
                    <option key={sprint.id} value={sprint.id}>
                      {sprint.name}
                    </option>
                  ))}
                </select>

                {/* 清除筛选按钮 */}
                {(filterKeyword || filterStatus || filterPriority || filterSprint) && (
                  <Button
                    variant="ghost"
                    size="sm"
                    onClick={clearFilters}
                    className="h-6 px-2 text-[10px]"
                  >
                    <X className="w-3 h-3 mr-1" />
                    清除
                  </Button>
                )}

                {/* 筛选结果统计 */}
                <span className="text-[10px] text-muted-foreground ml-auto">
                  {filteredStories.length} / {displayStories.length} 条
                </span>
              </div>
            </CardContent>
          </Card>

          {/* Story List - 表格形式（超紧凑版） */}
          <Card>
            <CardContent className="p-0">
              <ScrollArea className="h-[calc(100vh-350px)]">
                <table className="w-full border-collapse text-xs">
                  <thead className="sticky top-0 bg-muted/90 backdrop-blur-sm z-10">
                    <tr className="border-b border-border">
                      <th className="text-left py-1.5 px-2 font-semibold text-[10px] w-16">序号</th>
                      <th
                        className="text-left py-1.5 px-2 font-semibold text-[10px] w-16 cursor-pointer hover:bg-muted/50 transition-colors select-none"
                        onClick={e => handleSort('priority', e)}
                        title="点击切换排序，多次点击可添加多条件排序"
                      >
                        <div className="flex items-center gap-0.5">
                          <span>优先</span>
                          {(() => {
                            const config = getSortConfig('priority')
                            const index = getSortIndex('priority')
                            if (!config) {
                              return <ArrowUpDown className="w-2.5 h-2.5 text-muted-foreground" />
                            }
                            return (
                              <div className="flex items-center gap-0.5">
                                {config.order === 'asc' ? (
                                  <ArrowUp className="w-2.5 h-2.5" />
                                ) : (
                                  <ArrowDown className="w-2.5 h-2.5" />
                                )}
                                {sortConfigs.length > 1 && index && (
                                  <span className="text-[8px] bg-primary text-primary-foreground rounded-full w-3 h-3 flex items-center justify-center">
                                    {index}
                                  </span>
                                )}
                              </div>
                            )
                          })()}
                        </div>
                      </th>
                      <th className="text-left py-1.5 px-2 font-semibold text-[10px]">标题</th>
                      <th className="text-left py-1.5 px-2 font-semibold text-[10px] w-24">角色</th>
                      <th
                        className="text-left py-1.5 px-2 font-semibold text-[10px] w-16 cursor-pointer hover:bg-muted/50 transition-colors select-none"
                        onClick={e => handleSort('storyPoints', e)}
                        title="点击切换排序，多次点击可添加多条件排序"
                      >
                        <div className="flex items-center gap-0.5">
                          <span>点数</span>
                          {(() => {
                            const config = getSortConfig('storyPoints')
                            const index = getSortIndex('storyPoints')
                            if (!config) {
                              return <ArrowUpDown className="w-2.5 h-2.5 text-muted-foreground" />
                            }
                            return (
                              <div className="flex items-center gap-0.5">
                                {config.order === 'asc' ? (
                                  <ArrowUp className="w-2.5 h-2.5" />
                                ) : (
                                  <ArrowDown className="w-2.5 h-2.5" />
                                )}
                                {sortConfigs.length > 1 && index && (
                                  <span className="text-[8px] bg-primary text-primary-foreground rounded-full w-3 h-3 flex items-center justify-center">
                                    {index}
                                  </span>
                                )}
                              </div>
                            )
                          })()}
                        </div>
                      </th>
                      <th className="text-left py-1.5 px-2 font-semibold text-[10px] w-16">状态</th>
                      <th className="text-left py-1.5 px-2 font-semibold text-[10px] w-24">
                        Sprint
                      </th>
                      <th className="text-left py-1.5 px-2 font-semibold text-[10px] w-16">操作</th>
                    </tr>
                  </thead>
                  <tbody>
                    {paginatedStories.map((story: UserStory, index: number) => (
                      <tr
                        key={story.id}
                        className={`border-b border-border/30 hover:bg-muted/20 transition-colors ${
                          index % 2 === 0 ? 'bg-background' : 'bg-muted/5'
                        }`}
                      >
                        <td className="py-1.5 px-2 align-middle">
                          <span className="font-mono text-[10px]">{story.storyNumber}</span>
                        </td>
                        <td className="py-1.5 px-2 align-middle">
                          <Badge
                            className={`${priorityColors[story.priority]} text-[9px] px-1 py-0 h-4`}
                          >
                            {story.priority}
                          </Badge>
                        </td>
                        <td className="py-1.5 px-2 align-middle">
                          <div className="space-y-0.5">
                            <div className="font-medium text-xs leading-tight">{story.title}</div>
                            <div className="text-[10px] text-muted-foreground line-clamp-1">
                              {story.feature}
                            </div>
                          </div>
                        </td>
                        <td className="py-1.5 px-2 align-middle">
                          <div className="text-[10px] truncate" title={story.role}>
                            {story.role}
                          </div>
                        </td>
                        <td className="py-1.5 px-2 align-middle text-center">
                          {story.storyPoints && (
                            <span className="text-[10px]">{story.storyPoints}</span>
                          )}
                        </td>
                        <td className="py-1.5 px-2 align-middle">
                          <Badge
                            className={`${statusColors[story.status] || 'bg-gray-100 text-gray-700'} text-[9px] px-1.5 py-0 h-4 font-medium`}
                          >
                            {statusLabels[story.status] || story.status}
                          </Badge>
                        </td>
                        <td className="py-1.5 px-2 align-middle">
                          {editingSprintStoryId === story.id ? (
                            // 快速编辑模式：显示下拉选择器
                            <Select
                              value={story.sprintId || 'none'}
                              onValueChange={value => {
                                const newSprintId = value === 'none' ? undefined : value
                                handleQuickUpdateSprint(story.id, newSprintId)
                              }}
                              onOpenChange={open => {
                                if (!open) {
                                  cancelQuickEdit()
                                }
                              }}
                            >
                              <SelectTrigger className="h-6 text-[10px] px-2">
                                <SelectValue placeholder="选择 Sprint" />
                              </SelectTrigger>
                              <SelectContent>
                                <SelectItem value="none" className="text-[10px]">
                                  无
                                </SelectItem>
                                {sprints.map(sprint => (
                                  <SelectItem
                                    key={sprint.id}
                                    value={sprint.id}
                                    className="text-[10px]"
                                  >
                                    {sprint.name}
                                  </SelectItem>
                                ))}
                              </SelectContent>
                            </Select>
                          ) : (
                            // 显示模式：可点击切换到编辑
                            <div
                              className="text-[10px] truncate cursor-pointer hover:bg-muted/50 rounded px-1 py-0.5 transition-colors"
                              title={sprints.find(s => s.id === story.sprintId)?.name || ''}
                              onClick={() => setEditingSprintStoryId(story.id)}
                            >
                              {story.sprintId ? (
                                sprints.find(s => s.id === story.sprintId)?.name || '-'
                              ) : (
                                <span className="text-muted-foreground">未分配</span>
                              )}
                            </div>
                          )}
                        </td>
                        <td className="py-1.5 px-2 align-middle">
                          <div className="flex gap-0.5">
                            <Button
                              variant="ghost"
                              size="sm"
                              className="h-5 w-5 p-0"
                              onClick={() => handleEditStory(story)}
                              title="编辑用户故事"
                            >
                              <Edit2 className="w-2.5 h-2.5" />
                            </Button>
                            <Button
                              variant="ghost"
                              size="sm"
                              className="h-5 w-5 p-0 hover:bg-destructive/10 hover:text-destructive"
                              onClick={() => handleDeleteStory(story)}
                              title="删除用户故事"
                            >
                              <Trash2 className="w-2.5 h-2.5" />
                            </Button>
                          </div>
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </ScrollArea>
            </CardContent>
          </Card>

          {/* Pagination Controls - 超紧凑版 */}
          <div className="flex items-center justify-between py-1 px-1.5 border-t">
            <div className="flex items-center gap-1 text-[10px]">
              <span className="text-muted-foreground">每页:</span>
              <div className="flex gap-0.5">
                {[10, 20, 50].map(size => (
                  <Button
                    key={size}
                    variant={pageSize === size ? 'default' : 'outline'}
                    size="sm"
                    onClick={() => handlePageSizeChange(size)}
                    className="h-5 px-1.5 text-[9px]"
                  >
                    {size}
                  </Button>
                ))}
              </div>

              {/* 排序状态显示和清除按钮 */}
              {sortConfigs.length > 0 && (
                <>
                  <span className="text-muted-foreground ml-2">|</span>
                  <div className="flex items-center gap-1">
                    <span className="text-muted-foreground">排序:</span>
                    <div className="flex gap-0.5">
                      {sortConfigs.map((config, idx) => (
                        <Badge
                          key={config.field}
                          variant="secondary"
                          className="text-[9px] h-4 px-1 flex items-center gap-0.5"
                        >
                          <span>{config.field === 'priority' ? '优先' : '点数'}</span>
                          {config.order === 'asc' ? '↑' : '↓'}
                          {sortConfigs.length > 1 && (
                            <span className="text-[8px] opacity-70">{idx + 1}</span>
                          )}
                        </Badge>
                      ))}
                    </div>
                    <Button
                      variant="ghost"
                      size="sm"
                      onClick={clearSort}
                      className="h-4 px-1 text-[9px] hover:bg-destructive/10 hover:text-destructive"
                      title="清除所有排序"
                    >
                      ✕
                    </Button>
                  </div>
                </>
              )}
            </div>

            {totalPages > 1 && (
              <div className="flex items-center gap-0.5">
                <Button
                  variant="outline"
                  size="sm"
                  onClick={() => handlePageChange(currentPage - 1)}
                  disabled={currentPage === 1}
                  className="gap-0.5 h-5 px-1.5 text-[9px]"
                >
                  <ChevronLeft className="w-2.5 h-2.5" />
                  上页
                </Button>

                <div className="flex items-center gap-0.5">
                  {Array.from({ length: Math.min(5, totalPages) }, (_, i) => {
                    let pageNum
                    if (totalPages <= 5) {
                      pageNum = i + 1
                    } else if (currentPage <= 3) {
                      pageNum = i + 1
                    } else if (currentPage >= totalPages - 2) {
                      pageNum = totalPages - 4 + i
                    } else {
                      pageNum = currentPage - 2 + i
                    }

                    return (
                      <Button
                        key={pageNum}
                        variant={currentPage === pageNum ? 'default' : 'outline'}
                        size="sm"
                        onClick={() => handlePageChange(pageNum)}
                        className="w-5 h-5 p-0 text-[9px]"
                      >
                        {pageNum}
                      </Button>
                    )
                  })}
                </div>

                <Button
                  variant="outline"
                  size="sm"
                  onClick={() => handlePageChange(currentPage + 1)}
                  disabled={currentPage === totalPages}
                  className="gap-0.5 h-5 px-1.5 text-[9px]"
                >
                  下页
                  <ChevronRight className="w-2.5 h-2.5" />
                </Button>
              </div>
            )}
          </div>
        </div>
      )}

      {/* Decompose Dialog */}
      <DecomposeDialog
        open={showDecomposeDialog}
        onOpenChange={setShowDecomposeDialog}
        prdContent={prdContent}
        prompt={prompt}
        onPromptChange={setPrompt}
        onDecompose={handleDecompose}
        isStreaming={isStreaming}
        markdownContent={markdownContent}
        error={displayError}
      />

      {/* 用户故事编辑对话框 */}
      <UserStoryEditDialog
        open={showEditDialog}
        onOpenChange={setShowEditDialog}
        story={editingStory}
        onSave={handleSaveStory}
      />

      {/* 删除确认对话框 */}
      <Dialog open={showDeleteConfirm} onOpenChange={setShowDeleteConfirm}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>确认删除</DialogTitle>
            <DialogDescription>
              确定要删除用户故事"{deletingStory?.title}"吗？此操作不可恢复。
            </DialogDescription>
          </DialogHeader>
          <DialogFooter>
            <Button variant="outline" onClick={cancelDelete} disabled={isDeleting}>
              取消
            </Button>
            <Button variant="destructive" onClick={confirmDelete} disabled={isDeleting}>
              {isDeleting ? (
                <>
                  <Loader2 className="w-4 h-4 mr-2 animate-spin" />
                  删除中...
                </>
              ) : (
                '删除'
              )}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  )
}

/**
 * 用户故事卡片组件（已废弃，保留供未来使用）
 */
interface UserStoryCardProps {
  story: UserStory
  compact?: boolean // 紧凑模式
}

function _UserStoryCard({ story: _story, compact: _compact = false }: UserStoryCardProps) {
  // 当前版本不使用卡片组件，直接使用表格展示
  return null
}
