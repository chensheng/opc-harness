import { useState } from 'react'
import React from 'react'
import { Card, CardContent } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import { Loader2, Users, FileText, Sparkles } from 'lucide-react'
import type { UserStory } from '@/types'
import { useUserStoryStream } from '@/hooks/useUserStoryStream'
import { useProjectStore } from '@/stores/projectStore'
import { useUserStoryStore } from '@/stores/userStoryStore'
import { useSprintStore } from '@/stores/sprintStore'
import { DecomposeDialog } from './DecomposeDialog'
import { UserStoryEditDialog } from './UserStoryEditDialog'
import { UserStoryFilterBar } from './UserStoryFilterBar'
import { UserStoryTable } from './UserStoryTable'
import { UserStoryPagination } from './UserStoryPagination'
import { UserStoryDeleteDialog } from './UserStoryDeleteDialog'
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogFooter,
} from '@/components/ui/dialog'

interface UserStoryManagerProps {
  /** 项目 PRD 内容（必需） */
  prdContent: string
  /** AI API Key（可选） */
  apiKey?: string
  /** 拆分完成后的回调 */
  onStoriesGenerated?: (stories: UserStory[]) => void
}

/**
 * 用户故事管理组件
 *
 * 提供通过 AI 拆分 PRD 为用户故事的功能，并支持故事的查看、编辑和管理
 */
export function UserStoryManager({
  prdContent,
  apiKey: _apiKey,
  onStoriesGenerated,
}: UserStoryManagerProps) {
  const [prompt, setPrompt] = useState('')
  const [showDecomposeDialog, setShowDecomposeDialog] = useState(false)

  // 分页状态
  const [currentPage, setCurrentPage] = useState(1)
  const [pageSize, setPageSize] = useState(10)

  // 排序状态（支持多条件排序）
  type SortField = 'priority' | 'storyPoints'
  const [sortConfigs, setSortConfigs] = useState<
    Array<{ field: SortField; order: 'asc' | 'desc' }>
  >([])

  // 筛选状态
  const [filterKeyword, setFilterKeyword] = useState('')
  const [filterStatus, setFilterStatus] = useState<string>('')
  const [filterPriority, setFilterPriority] = useState<string>('')
  const [filterSprint, setFilterSprint] = useState<string>('')

  // 编辑对话框状态
  const [editingStory, setEditingStory] = useState<UserStory | null>(null)
  const [showEditDialog, setShowEditDialog] = useState(false)

  // 删除确认对话框状态
  const [deletingStory, setDeletingStory] = useState<UserStory | null>(null)
  const [showDeleteConfirm, setShowDeleteConfirm] = useState(false)
  const [isDeleting, setIsDeleting] = useState(false)

  // 重试确认对话框状态
  const [retryingStory, setRetryingStory] = useState<UserStory | null>(null)
  const [showRetryConfirm, setShowRetryConfirm] = useState(false)
  const [isRetrying, setIsRetrying] = useState(false)

  // 快速编辑 Sprint 的状态
  const [editingSprintStoryId, setEditingSprintStoryId] = useState<string | null>(null)

  // 使用流式 Hook
  const {
    markdownContent,
    userStories: streamUserStories,
    isStreaming,
    error: streamError,
    startStream,
  } = useUserStoryStream()

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

  // 调试日志：当错误状态变化时输出
  const displayError = streamError || undefined
  React.useEffect(() => {
    if (displayError) {
      console.log('[UserStoryManager] Display error changed:', displayError)
    }
  }, [displayError])

  const displayStories = streamUserStories.length > 0 ? streamUserStories : savedStories

  const handleDecompose = async () => {
    const fullContent = prompt.trim() ? `${prdContent}\n\n---\n\n用户要求：${prompt}` : prdContent

    const { useAIConfigStore } = await import('@/stores/aiConfigStore')
    const store = useAIConfigStore.getState()
    const activeConfig = store.getActiveConfig()

    if (activeConfig?.provider !== 'codefree' && !activeConfig?.apiKey) {
      alert('未配置 AI API Key，请先在设置中配置')
      return
    }

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

    await startStream(
      {
        prdContent: fullContent,
        provider: store.defaultProvider,
        model: activeConfig.model,
        apiKey: activeConfig.apiKey || '',
        projectId: currentProjectId || undefined,
        existingStories,
      },
      stories => {
        if (onStoriesGenerated) {
          onStoriesGenerated(stories)
        }
        setShowDecomposeDialog(false)
      }
    )
  }

  const handleEditStory = (story: UserStory) => {
    setEditingStory(story)
    setShowEditDialog(true)
  }

  const handleSaveStory = async (updatedStory: UserStory) => {
    if (currentProjectId && editingStory) {
      await updateStory(currentProjectId, editingStory.id, updatedStory)
      setShowEditDialog(false)
      setEditingStory(null)
    }
  }

  const handleDeleteStory = (story: UserStory) => {
    setDeletingStory(story)
    setShowDeleteConfirm(true)
  }

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

  const cancelDelete = () => {
    setShowDeleteConfirm(false)
    setDeletingStory(null)
  }

  // 重试失败的用户故事
  const handleRetryStory = (story: UserStory) => {
    setRetryingStory(story)
    setShowRetryConfirm(true)
  }

  const confirmRetry = async () => {
    if (!retryingStory || !currentProjectId) return

    setIsRetrying(true)
    try {
      await updateStory(currentProjectId, retryingStory.id, {
        status: 'draft',
        updatedAt: new Date().toISOString(),
      })
      setShowRetryConfirm(false)
      setRetryingStory(null)
    } catch (error) {
      console.error('重试用户故事失败:', error)
      alert('重试失败，请重试')
    } finally {
      setIsRetrying(false)
    }
  }

  const cancelRetry = () => {
    setShowRetryConfirm(false)
    setRetryingStory(null)
  }

  const handleQuickUpdateSprint = async (storyId: string, sprintId: string | undefined) => {
    if (!currentProjectId) return

    try {
      await updateStory(currentProjectId, storyId, { sprintId })
      setEditingSprintStoryId(null)
    } catch (error) {
      console.error('更新 Sprint 失败:', error)
      alert('更新失败，请重试')
    }
  }

  const cancelQuickEdit = () => {
    setEditingSprintStoryId(null)
  }

  // 筛选逻辑
  const filteredStories = React.useMemo(() => {
    let result = displayStories

    if (filterKeyword.trim()) {
      const keyword = filterKeyword.toLowerCase()
      result = result.filter(
        (story: UserStory) =>
          story.title.toLowerCase().includes(keyword) ||
          story.feature.toLowerCase().includes(keyword) ||
          story.role.toLowerCase().includes(keyword)
      )
    }

    if (filterStatus) {
      result = result.filter((story: UserStory) => story.status === filterStatus)
    }

    if (filterPriority) {
      result = result.filter((story: UserStory) => story.priority === filterPriority)
    }

    if (filterSprint) {
      if (filterSprint === 'unassigned') {
        result = result.filter((story: UserStory) => !story.sprintId)
      } else {
        result = result.filter((story: UserStory) => story.sprintId === filterSprint)
      }
    }

    return result
  }, [displayStories, filterKeyword, filterStatus, filterPriority, filterSprint])

  // 排序逻辑
  const sortedStories = React.useMemo(() => {
    if (sortConfigs.length === 0) return filteredStories

    return [...filteredStories].sort((a: UserStory, b: UserStory) => {
      let comparison = 0

      for (const config of sortConfigs) {
        if (config.field === 'priority') {
          const priorityOrder: Record<string, number> = { P0: 0, P1: 1, P2: 2, P3: 3 }
          const aPriority = priorityOrder[a.priority] ?? 999
          const bPriority = priorityOrder[b.priority] ?? 999
          comparison = aPriority - bPriority
        } else if (config.field === 'storyPoints') {
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

  const handleSort = (field: SortField, _event?: React.MouseEvent) => {
    const existingConfig = sortConfigs.find(config => config.field === field)

    if (existingConfig) {
      const newOrder = existingConfig.order === 'asc' ? 'desc' : 'asc'
      setSortConfigs(prevConfigs =>
        prevConfigs.map(config =>
          config.field === field ? { ...config, order: newOrder } : config
        )
      )
    } else {
      setSortConfigs(prevConfigs => [...prevConfigs, { field, order: 'asc' }])
    }

    setCurrentPage(1)
  }

  const clearSort = () => {
    setSortConfigs([])
    setCurrentPage(1)
  }

  const getSortConfig = (field: SortField) => {
    return sortConfigs.find(config => config.field === field)
  }

  const getSortIndex = (field: SortField) => {
    const index = sortConfigs.findIndex(config => config.field === field)
    return index >= 0 ? index + 1 : null
  }

  const clearFilters = () => {
    setFilterKeyword('')
    setFilterStatus('')
    setFilterPriority('')
    setFilterSprint('')
    setCurrentPage(1)
  }

  React.useEffect(() => {
    setCurrentPage(1)
  }, [displayStories.length])

  const handlePageChange = (page: number) => {
    setCurrentPage(page)
  }

  const handlePageSizeChange = (size: number) => {
    setPageSize(size)
    setCurrentPage(1)
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
          {/* Filter Bar */}
          <UserStoryFilterBar
            filterKeyword={filterKeyword}
            onFilterKeywordChange={setFilterKeyword}
            filterStatus={filterStatus}
            onFilterStatusChange={setFilterStatus}
            filterPriority={filterPriority}
            onFilterPriorityChange={setFilterPriority}
            filterSprint={filterSprint}
            onFilterSprintChange={setFilterSprint}
            displayStories={displayStories}
            filteredStories={filteredStories}
            sprints={sprints}
            onClearFilters={clearFilters}
          />

          {/* Story Table */}
          <UserStoryTable
            stories={paginatedStories}
            sortConfigs={sortConfigs}
            onSort={handleSort}
            getSortConfig={getSortConfig}
            getSortIndex={getSortIndex}
            editingSprintStoryId={editingSprintStoryId}
            onEditSprint={setEditingSprintStoryId}
            onCancelQuickEdit={cancelQuickEdit}
            onQuickUpdateSprint={handleQuickUpdateSprint}
            sprints={sprints}
            onEditStory={handleEditStory}
            onDeleteStory={handleDeleteStory}
            onRetryStory={handleRetryStory}
          />

          {/* Pagination */}
          <UserStoryPagination
            currentPage={currentPage}
            totalPages={totalPages}
            pageSize={pageSize}
            sortConfigs={sortConfigs}
            onPageChange={handlePageChange}
            onPageSizeChange={handlePageSizeChange}
            onClearSort={clearSort}
          />
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

      {/* User Story Edit Dialog */}
      <UserStoryEditDialog
        open={showEditDialog}
        onOpenChange={setShowEditDialog}
        story={editingStory}
        onSave={handleSaveStory}
      />

      {/* Delete Confirmation Dialog */}
      <UserStoryDeleteDialog
        open={showDeleteConfirm}
        onOpenChange={setShowDeleteConfirm}
        story={deletingStory}
        isDeleting={isDeleting}
        onConfirm={confirmDelete}
        onCancel={cancelDelete}
      />

      {/* Retry Confirmation Dialog */}
      <Dialog open={showRetryConfirm} onOpenChange={setShowRetryConfirm}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>重试用户故事</DialogTitle>
            <DialogDescription>
              确定要重试 "{retryingStory?.title}" 吗？
              <br />
              <span className="text-orange-600">
                该操作会将状态重置为"草稿"，Agent Worker 将重新处理此故事。
              </span>
            </DialogDescription>
          </DialogHeader>
          <DialogFooter>
            <Button variant="outline" onClick={cancelRetry} disabled={isRetrying}>
              取消
            </Button>
            <Button onClick={confirmRetry} disabled={isRetrying}>
              {isRetrying ? '重试中...' : '确认重试'}
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
