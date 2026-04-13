import { useState, useEffect, useMemo } from 'react'
import { Dialog, DialogContent, DialogHeader, DialogTitle } from '@/components/ui/dialog'
import { Input } from '@/components/ui/input'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import { Checkbox } from '@/components/ui/checkbox'
import { ScrollArea } from '@/components/ui/scroll-area'
import { Search, X, Users } from 'lucide-react'
import type { Sprint, UserStory } from '@/types'

interface ManageStoriesDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  sprint: Sprint
  availableStories: UserStory[]
  onSave: (sprint: Sprint, selectedStoryIds: string[]) => Promise<void>
}

export function ManageStoriesDialog({
  open,
  onOpenChange,
  sprint,
  availableStories,
  onSave,
}: ManageStoriesDialogProps) {
  const [selectedStoryIds, setSelectedStoryIds] = useState<string[]>([])
  const [storyFilterKeyword, setStoryFilterKeyword] = useState('')
  const [showOnlyUnassigned, setShowOnlyUnassigned] = useState(false)
  const [isSaving, setIsSaving] = useState(false)

  // 初始化已选故事（通过查询 user_stories.sprint_id 获取）
  useEffect(() => {
    if (open && sprint) {
      // 从 availableStories 中筛选出属于当前 Sprint 的故事
      const assignedStoryIds = availableStories
        .filter(story => story.sprintId === sprint.id)
        .map(story => story.id)

      setSelectedStoryIds(assignedStoryIds)
      setStoryFilterKeyword('')
      setShowOnlyUnassigned(false)
    }
  }, [open, sprint, availableStories])

  // 计算总故事点
  const calculateTotalStoryPoints = () => {
    return availableStories
      .filter(story => selectedStoryIds.includes(story.id))
      .reduce((sum, story) => sum + (story.storyPoints || 0), 0)
  }

  // 切换故事选择
  const toggleStorySelection = (storyId: string) => {
    setSelectedStoryIds(prev =>
      prev.includes(storyId) ? prev.filter(id => id !== storyId) : [...prev, storyId]
    )
  }

  // 全选/取消全选
  const handleSelectAll = () => {
    if (filteredStories.length === selectedStoryIds.length) {
      setSelectedStoryIds([])
    } else {
      setSelectedStoryIds(filteredStories.map(s => s.id))
    }
  }

  // 筛选后的故事列表
  const filteredStories = useMemo(() => {
    let stories = availableStories

    // 关键词筛选
    if (storyFilterKeyword.trim()) {
      const keyword = storyFilterKeyword.toLowerCase()
      stories = stories.filter(
        story =>
          story.title.toLowerCase().includes(keyword) ||
          story.storyNumber.toLowerCase().includes(keyword) ||
          story.role.toLowerCase().includes(keyword)
      )
    }

    // 只显示未分配的故事
    if (showOnlyUnassigned) {
      stories = stories.filter(story => !selectedStoryIds.includes(story.id))
    }

    return stories
  }, [availableStories, storyFilterKeyword, showOnlyUnassigned, selectedStoryIds])

  // 处理保存（只更新 Sprint 的基本信息，用户故事的 sprintId 由 SprintManager 处理）
  const handleSave = async () => {
    setIsSaving(true)
    try {
      const updatedSprint: Sprint = {
        ...sprint,
        totalStoryPoints: calculateTotalStoryPoints(),
        updatedAt: new Date().toISOString(),
      }

      await onSave(updatedSprint, selectedStoryIds)
      onOpenChange(false)
    } catch (error) {
      console.error('Failed to save managed stories:', error)
    } finally {
      setIsSaving(false)
    }
  }

  // 处理取消
  const handleCancel = () => {
    onOpenChange(false)
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-4xl max-h-[90vh] overflow-y-auto">
        <DialogHeader>
          <DialogTitle>管理用户故事 - {sprint.name}</DialogTitle>
        </DialogHeader>

        {/* 顶部操作和统计区域 */}
        <div className="flex items-center justify-between py-2 border-b">
          {/* 统计信息 */}
          <div className="flex items-center gap-2">
            <Users className="w-4 h-4 text-muted-foreground" />
            <span className="text-sm font-medium">
              已选择 <span className="text-primary">{selectedStoryIds.length}</span> 个故事
            </span>
            <Badge variant="secondary" className="text-xs">
              总故事点: {calculateTotalStoryPoints()}
            </Badge>
          </div>

          {/* 操作按钮 */}
          <div className="flex gap-2">
            <Button variant="outline" onClick={handleCancel} disabled={isSaving}>
              取消
            </Button>
            <Button onClick={handleSave} disabled={isSaving}>
              {isSaving ? '保存中...' : '保存'}
            </Button>
          </div>
        </div>

        <div className="space-y-4 py-4">
          {/* 搜索和筛选工具栏 */}
          <div className="flex gap-2">
            <div className="relative flex-1">
              <Search className="absolute left-2 top-1/2 -translate-y-1/2 w-3 h-3 text-muted-foreground" />
              <Input
                type="text"
                placeholder="搜索故事编号、标题、角色..."
                value={storyFilterKeyword}
                onChange={e => setStoryFilterKeyword(e.target.value)}
                className="pl-7 h-8 text-xs"
              />
              {storyFilterKeyword && (
                <button
                  onClick={() => setStoryFilterKeyword('')}
                  className="absolute right-2 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground"
                >
                  <X className="w-3 h-3" />
                </button>
              )}
            </div>
            <Button
              variant={showOnlyUnassigned ? 'default' : 'outline'}
              size="sm"
              onClick={() => setShowOnlyUnassigned(!showOnlyUnassigned)}
              className="h-8 text-xs"
            >
              {showOnlyUnassigned ? '显示全部' : '未分配'}
            </Button>
            <Button variant="outline" size="sm" onClick={handleSelectAll} className="h-8 text-xs">
              {filteredStories.length > 0 &&
              filteredStories.every(s => selectedStoryIds.includes(s.id))
                ? '取消全选'
                : '全选'}
            </Button>
          </div>

          {/* 故事列表 */}
          <ScrollArea className="h-[400px] w-full rounded-md border p-3">
            {filteredStories.length > 0 ? (
              <div className="space-y-2">
                {filteredStories.map(story => (
                  <div
                    key={story.id}
                    className={`flex items-start space-x-3 p-2 rounded transition-colors ${
                      selectedStoryIds.includes(story.id)
                        ? 'bg-primary/5 border border-primary/20'
                        : 'hover:bg-muted/50'
                    }`}
                  >
                    <Checkbox
                      id={`story-${story.id}`}
                      checked={selectedStoryIds.includes(story.id)}
                      onChange={() => toggleStorySelection(story.id)}
                    />
                    <div
                      className="flex-1 space-y-1 cursor-pointer"
                      onClick={() => toggleStorySelection(story.id)}
                    >
                      <div className="text-sm font-medium leading-none">
                        {story.storyNumber} - {story.title}
                      </div>
                      <div className="flex items-center gap-2 text-xs text-muted-foreground">
                        <span>{story.role}</span>
                        {story.storyPoints && (
                          <Badge variant="outline" className="text-[10px] h-4 px-1">
                            {story.storyPoints} 点
                          </Badge>
                        )}
                        <Badge
                          className={`${
                            story.priority === 'P0'
                              ? 'bg-red-100 text-red-700'
                              : story.priority === 'P1'
                                ? 'bg-orange-100 text-orange-700'
                                : story.priority === 'P2'
                                  ? 'bg-yellow-100 text-yellow-700'
                                  : 'bg-gray-100 text-gray-700'
                          } text-[10px] h-4 px-1`}
                        >
                          {story.priority}
                        </Badge>
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            ) : (
              <div className="flex flex-col items-center justify-center h-full text-muted-foreground py-8">
                <Users className="w-8 h-8 mb-2 opacity-50" />
                <p className="text-sm">
                  {storyFilterKeyword || showOnlyUnassigned
                    ? '没有符合条件的用户故事'
                    : '暂无可用的用户故事'}
                </p>
              </div>
            )}
          </ScrollArea>

          {/* 已选故事标签 */}
          {selectedStoryIds.length > 0 && (
            <div className="p-3 bg-muted/30 rounded-md space-y-2">
              <div className="flex items-center justify-between text-xs">
                <span className="font-medium">已选择的故事</span>
                <Button
                  variant="ghost"
                  size="sm"
                  onClick={() => setSelectedStoryIds([])}
                  className="h-6 text-[10px] px-2"
                >
                  <X className="w-3 h-3 mr-1" />
                  清空
                </Button>
              </div>
              <div className="flex flex-wrap gap-1">
                {availableStories
                  .filter(s => selectedStoryIds.includes(s.id))
                  .map(story => (
                    <Badge
                      key={story.id}
                      variant="secondary"
                      className="text-[10px] h-5 px-2 flex items-center gap-1"
                    >
                      {story.storyNumber}
                      <button
                        onClick={() => toggleStorySelection(story.id)}
                        className="hover:text-destructive"
                      >
                        <X className="w-2.5 h-2.5" />
                      </button>
                    </Badge>
                  ))}
              </div>
            </div>
          )}
        </div>
      </DialogContent>
    </Dialog>
  )
}
