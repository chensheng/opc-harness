import { useState, useEffect, useMemo } from 'react'
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from '@/components/ui/dialog'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Textarea } from '@/components/ui/textarea'
import { Label } from '@/components/ui/label'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { Checkbox } from '@/components/ui/checkbox'
import { ScrollArea } from '@/components/ui/scroll-area'
import type { Sprint, UserStory } from '@/types'
import { Badge } from '@/components/ui/badge'
import { Search, X, Users } from 'lucide-react'

interface SprintEditDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  sprint: Sprint | null
  availableStories: UserStory[]
  onSave: (updatedSprint: Sprint) => Promise<void>
}

const statusColors: Record<Sprint['status'], string> = {
  planning: 'bg-gray-100 text-gray-700 dark:bg-gray-800 dark:text-gray-300',
  active: 'bg-blue-100 text-blue-700 dark:bg-blue-900/30 dark:text-blue-400',
  completed: 'bg-green-100 text-green-700 dark:bg-green-900/30 dark:text-green-400',
  cancelled: 'bg-red-100 text-red-700 dark:bg-red-900/30 dark:text-red-400',
}

export function SprintEditDialog({
  open,
  onOpenChange,
  sprint,
  availableStories,
  onSave,
}: SprintEditDialogProps) {
  const [formData, setFormData] = useState<Partial<Sprint>>({})
  const [selectedStoryIds, setSelectedStoryIds] = useState<string[]>([])
  const [errors, setErrors] = useState<Record<string, string>>({})
  const [isSaving, setIsSaving] = useState(false)

  // 用户故事筛选状态
  const [storyFilterKeyword, setStoryFilterKeyword] = useState('')
  const [showOnlyUnassigned, setShowOnlyUnassigned] = useState(false)

  // 初始化表单数据
  useEffect(() => {
    if (sprint) {
      setFormData({
        name: sprint.name,
        goal: sprint.goal,
        startDate: sprint.startDate,
        endDate: sprint.endDate,
        status: sprint.status,
      })
      setSelectedStoryIds([...sprint.storyIds])
      setErrors({})
    } else {
      // 新建Sprint时的默认值
      const today = new Date().toISOString().split('T')[0]
      const nextWeek = new Date(Date.now() + 7 * 24 * 60 * 60 * 1000).toISOString().split('T')[0]
      setFormData({
        name: '',
        goal: '',
        startDate: today,
        endDate: nextWeek,
        status: 'planning',
      })
      setSelectedStoryIds([])
      setErrors({})
    }
  }, [sprint])

  // 计算已选故事的总故事点
  const calculateTotalStoryPoints = () => {
    return availableStories
      .filter(story => selectedStoryIds.includes(story.id))
      .reduce((sum, story) => sum + (story.storyPoints || 0), 0)
  }

  // 验证表单
  const validateForm = (): boolean => {
    const newErrors: Record<string, string> = {}

    if (!formData.name?.trim()) {
      newErrors.name = 'Sprint名称不能为空'
    }

    if (!formData.startDate) {
      newErrors.startDate = '开始日期不能为空'
    }

    if (!formData.endDate) {
      newErrors.endDate = '结束日期不能为空'
    }

    // 验证日期范围
    if (formData.startDate && formData.endDate) {
      const start = new Date(formData.startDate)
      const end = new Date(formData.endDate)
      if (end < start) {
        newErrors.endDate = '结束日期必须晚于开始日期'
      }
    }

    setErrors(newErrors)
    return Object.keys(newErrors).length === 0
  }

  // 处理保存
  const handleSave = async () => {
    if (!validateForm()) return

    setIsSaving(true)
    try {
      const totalStoryPoints = calculateTotalStoryPoints()

      const newSprint: Sprint = {
        id: sprint?.id || crypto.randomUUID(),
        name: formData.name || '',
        goal: formData.goal || '',
        startDate: formData.startDate || '',
        endDate: formData.endDate || '',
        status: formData.status || 'planning',
        storyIds: selectedStoryIds,
        totalStoryPoints,
        completedStoryPoints: 0, // TODO: 根据关联故事的完成状态计算
        createdAt: sprint?.createdAt || new Date().toISOString(),
        updatedAt: new Date().toISOString(),
      }

      await onSave(newSprint)
      onOpenChange(false)
    } catch (error) {
      console.error('Failed to save sprint:', error)
      setErrors({ submit: '保存失败，请重试' })
    } finally {
      setIsSaving(false)
    }
  }

  // 处理取消
  const handleCancel = () => {
    onOpenChange(false)
    setFormData({})
    setSelectedStoryIds([])
    setErrors({})
  }

  // 更新字段值
  const updateField = (field: keyof Sprint, value: string) => {
    setFormData(prev => ({ ...prev, [field]: value }))
    // 清除该字段的错误
    if (errors[field]) {
      setErrors(prev => {
        const newErrors = { ...prev }
        delete newErrors[field]
        return newErrors
      })
    }
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
      // 如果已全部选中，则取消全选
      setSelectedStoryIds([])
    } else {
      // 否则全选当前筛选后的故事
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

  if (!sprint && !open) return null

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-4xl max-h-[90vh] overflow-y-auto">
        <DialogHeader>
          <DialogTitle>{sprint ? `编辑Sprint计划` : '新建Sprint计划'}</DialogTitle>
        </DialogHeader>

        <div className="space-y-6 py-4">
          {/* 基本信息组 */}
          <div className="space-y-4">
            <h3 className="text-sm font-semibold text-muted-foreground">基本信息</h3>
            <div className="space-y-4">
              <div className="space-y-2">
                <Label htmlFor="name">
                  Sprint名称 <span className="text-red-500">*</span>
                </Label>
                <Input
                  id="name"
                  value={formData.name || ''}
                  onChange={e => updateField('name', e.target.value)}
                  placeholder="例如：Sprint 1 - 用户认证模块"
                  className={errors.name ? 'border-red-500' : ''}
                />
                {errors.name && <p className="text-xs text-red-500">{errors.name}</p>}
              </div>

              <div className="space-y-2">
                <Label htmlFor="goal">Sprint目标</Label>
                <Textarea
                  id="goal"
                  value={formData.goal || ''}
                  onChange={e => updateField('goal', e.target.value)}
                  placeholder="描述本次Sprint要达成的目标..."
                  rows={3}
                />
              </div>
            </div>
          </div>

          {/* 时间规划组 */}
          <div className="space-y-4">
            <h3 className="text-sm font-semibold text-muted-foreground">时间规划</h3>
            <div className="grid grid-cols-2 gap-4">
              <div className="space-y-2">
                <Label htmlFor="startDate">
                  开始日期 <span className="text-red-500">*</span>
                </Label>
                <Input
                  id="startDate"
                  type="date"
                  value={formData.startDate || ''}
                  onChange={e => updateField('startDate', e.target.value)}
                  className={errors.startDate ? 'border-red-500' : ''}
                />
                {errors.startDate && <p className="text-xs text-red-500">{errors.startDate}</p>}
              </div>

              <div className="space-y-2">
                <Label htmlFor="endDate">
                  结束日期 <span className="text-red-500">*</span>
                </Label>
                <Input
                  id="endDate"
                  type="date"
                  value={formData.endDate || ''}
                  onChange={e => updateField('endDate', e.target.value)}
                  className={errors.endDate ? 'border-red-500' : ''}
                />
                {errors.endDate && <p className="text-xs text-red-500">{errors.endDate}</p>}
              </div>
            </div>
          </div>

          {/* 状态 */}
          <div className="space-y-2">
            <Label htmlFor="status">状态</Label>
            <Select
              value={formData.status}
              onValueChange={value => updateField('status', value as Sprint['status'])}
            >
              <SelectTrigger>
                <SelectValue placeholder="选择状态" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="planning">
                  <div className="flex items-center gap-2">
                    <Badge className={statusColors.planning}>规划中</Badge>
                  </div>
                </SelectItem>
                <SelectItem value="active">
                  <div className="flex items-center gap-2">
                    <Badge className={statusColors.active}>进行中</Badge>
                  </div>
                </SelectItem>
                <SelectItem value="completed">
                  <div className="flex items-center gap-2">
                    <Badge className={statusColors.completed}>已完成</Badge>
                  </div>
                </SelectItem>
                <SelectItem value="cancelled">
                  <div className="flex items-center gap-2">
                    <Badge className={statusColors.cancelled}>已取消</Badge>
                  </div>
                </SelectItem>
              </SelectContent>
            </Select>
          </div>

          {/* 用户故事分配 */}
          <div className="space-y-4">
            <div className="flex items-center justify-between">
              <h3 className="text-sm font-semibold text-muted-foreground">
                关联用户故事 ({selectedStoryIds.length})
              </h3>
              <Badge variant="secondary" className="text-xs">
                总故事点: {calculateTotalStoryPoints()}
              </Badge>
            </div>

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

            <ScrollArea className="h-[250px] w-full rounded-md border p-3">
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
                      <div className="flex-1 space-y-1">
                        <label
                          htmlFor={`story-${story.id}`}
                          className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70 cursor-pointer"
                        >
                          {story.storyNumber} - {story.title}
                        </label>
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

            {/* 已选故事统计 */}
            {selectedStoryIds.length > 0 && (
              <div className="p-3 bg-muted/30 rounded-md space-y-2">
                <div className="flex items-center justify-between text-xs">
                  <span className="font-medium">已选择 {selectedStoryIds.length} 个故事</span>
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

          {/* 统计信息（只读） */}
          {sprint && (
            <div className="space-y-4">
              <h3 className="text-sm font-semibold text-muted-foreground">统计信息</h3>
              <div className="grid grid-cols-2 gap-4">
                <div className="space-y-2">
                  <Label>总故事点</Label>
                  <Input value={calculateTotalStoryPoints()} readOnly className="bg-muted/50" />
                </div>
                <div className="space-y-2">
                  <Label>已完成故事点</Label>
                  <Input
                    value={sprint.completedStoryPoints || 0}
                    readOnly
                    className="bg-muted/50"
                  />
                </div>
              </div>
            </div>
          )}

          {/* 提交错误提示 */}
          {errors.submit && (
            <div className="p-3 bg-red-50 dark:bg-red-950/20 border border-red-200 dark:border-red-800 rounded-md">
              <p className="text-sm text-red-700 dark:text-red-400">{errors.submit}</p>
            </div>
          )}
        </div>

        <DialogFooter>
          <Button variant="outline" onClick={handleCancel} disabled={isSaving}>
            取消
          </Button>
          <Button onClick={handleSave} disabled={isSaving}>
            {isSaving ? '保存中...' : '保存'}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
