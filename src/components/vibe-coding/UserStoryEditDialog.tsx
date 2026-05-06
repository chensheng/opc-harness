import { useState, useEffect } from 'react'
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
import type { UserStory, Sprint } from '@/types'
import { X, Plus } from 'lucide-react'

interface UserStoryEditDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  story: UserStory | null
  onSave: (updatedStory: UserStory) => Promise<void>
  sprints?: Sprint[] // 可选的 Sprint 列表
}

export function UserStoryEditDialog({
  open,
  onOpenChange,
  story,
  onSave,
  sprints = [],
}: UserStoryEditDialogProps) {
  const [formData, setFormData] = useState<Partial<UserStory>>({})
  const [errors, setErrors] = useState<Record<string, string>>({})
  const [isSaving, setIsSaving] = useState(false)

  // 初始化表单数据
  useEffect(() => {
    if (story) {
      setFormData({
        title: story.title,
        role: story.role,
        feature: story.feature,
        benefit: story.benefit,
        description: story.description,
        acceptanceCriteria: [...story.acceptanceCriteria],
        priority: story.priority,
        status: story.status,
        storyPoints: story.storyPoints,
        featureModule: story.featureModule,
        labels: [...story.labels],
      })
      setErrors({})
    }
  }, [story])

  // 验证表单
  const validateForm = (): boolean => {
    const newErrors: Record<string, string> = {}

    if (!formData.title?.trim()) {
      newErrors.title = '标题不能为空'
    }
    if (!formData.role?.trim()) {
      newErrors.role = '角色不能为空'
    }
    if (!formData.feature?.trim()) {
      newErrors.feature = '功能描述不能为空'
    }
    if (!formData.benefit?.trim()) {
      newErrors.benefit = '价值描述不能为空'
    }
    if (!formData.description?.trim()) {
      newErrors.description = '详细描述不能为空'
    }

    setErrors(newErrors)
    return Object.keys(newErrors).length === 0
  }

  // 处理保存
  const handleSave = async () => {
    if (!validateForm() || !story) return

    setIsSaving(true)
    try {
      const updatedStory: UserStory = {
        ...story,
        ...formData,
        acceptanceCriteria: formData.acceptanceCriteria || [],
        labels: formData.labels || [],
        updatedAt: new Date().toISOString(),
      } as UserStory

      await onSave(updatedStory)
      onOpenChange(false)
    } catch (error) {
      console.error('Failed to save story:', error)
      setErrors({ submit: '保存失败，请重试' })
    } finally {
      setIsSaving(false)
    }
  }

  // 处理取消
  const handleCancel = () => {
    onOpenChange(false)
    setFormData({})
    setErrors({})
  }

  // 更新字段值
  const updateField = (field: keyof UserStory, value: string | number | string[] | undefined) => {
    setFormData(prev => {
      const newData = { ...prev, [field]: value }

      // 当角色、功能或价值改变时，自动生成完整描述
      if (field === 'role' || field === 'feature' || field === 'benefit') {
        const role = field === 'role' ? (value as string) : prev.role
        const feature = field === 'feature' ? (value as string) : prev.feature
        const benefit = field === 'benefit' ? (value as string) : prev.benefit

        if (role && feature && benefit) {
          newData.description = `As a ${role}, I want ${feature}, so that ${benefit}`
        }
      }

      return newData
    })

    // 清除该字段的错误
    if (errors[field]) {
      setErrors(prev => {
        const newErrors = { ...prev }
        delete newErrors[field]
        return newErrors
      })
    }
  }

  // 添加验收标准
  const addAcceptanceCriterion = () => {
    const currentCriteria = formData.acceptanceCriteria || []
    updateField('acceptanceCriteria', [...currentCriteria, ''])
  }

  // 更新验收标准
  const updateAcceptanceCriterion = (index: number, value: string) => {
    const currentCriteria = formData.acceptanceCriteria || []
    const newCriteria = [...currentCriteria]
    newCriteria[index] = value
    updateField('acceptanceCriteria', newCriteria)
  }

  // 删除验收标准
  const removeAcceptanceCriterion = (index: number) => {
    const currentCriteria = formData.acceptanceCriteria || []
    const newCriteria = currentCriteria.filter((_, i) => i !== index)
    updateField('acceptanceCriteria', newCriteria)
  }

  // 处理标签输入（逗号分隔）
  const handleLabelsChange = (value: string) => {
    const labels = value
      .split(',')
      .map(label => label.trim())
      .filter(Boolean)
    updateField('labels', labels)
  }

  if (!story) return null

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-4xl max-h-[90vh] overflow-y-auto">
        <DialogHeader>
          <DialogTitle>编辑用户故事：{story.storyNumber}</DialogTitle>
        </DialogHeader>

        <div className="space-y-6 py-4">
          {/* 基本信息组 */}
          <div className="space-y-4">
            <h3 className="text-sm font-semibold text-muted-foreground">基本信息</h3>
            <div className="grid grid-cols-2 gap-4">
              <div className="space-y-2">
                <Label htmlFor="title">
                  标题 <span className="text-red-500">*</span>
                </Label>
                <Input
                  id="title"
                  value={formData.title || ''}
                  onChange={e => updateField('title', e.target.value)}
                  placeholder="简短的故事标题"
                  className={errors.title ? 'border-red-500' : ''}
                />
                {errors.title && <p className="text-xs text-red-500">{errors.title}</p>}
              </div>

              <div className="space-y-2">
                <Label htmlFor="role">
                  角色 <span className="text-red-500">*</span>
                </Label>
                <Input
                  id="role"
                  value={formData.role || ''}
                  onChange={e => updateField('role', e.target.value)}
                  placeholder="例如：管理员、普通用户"
                  className={errors.role ? 'border-red-500' : ''}
                />
                {errors.role && <p className="text-xs text-red-500">{errors.role}</p>}
              </div>

              <div className="space-y-2">
                <Label htmlFor="feature">
                  功能 <span className="text-red-500">*</span>
                </Label>
                <Input
                  id="feature"
                  value={formData.feature || ''}
                  onChange={e => updateField('feature', e.target.value)}
                  placeholder="想要实现的功能"
                  className={errors.feature ? 'border-red-500' : ''}
                />
                {errors.feature && <p className="text-xs text-red-500">{errors.feature}</p>}
              </div>

              <div className="space-y-2">
                <Label htmlFor="benefit">
                  价值 <span className="text-red-500">*</span>
                </Label>
                <Input
                  id="benefit"
                  value={formData.benefit || ''}
                  onChange={e => updateField('benefit', e.target.value)}
                  placeholder="带来的业务价值"
                  className={errors.benefit ? 'border-red-500' : ''}
                />
                {errors.benefit && <p className="text-xs text-red-500">{errors.benefit}</p>}
              </div>
            </div>
          </div>

          {/* 详细描述组 */}
          <div className="space-y-4">
            <h3 className="text-sm font-semibold text-muted-foreground">详细描述</h3>
            <div className="space-y-4">
              <div className="space-y-2">
                <Label htmlFor="description">完整描述（自动生成）</Label>
                <Textarea
                  id="description"
                  value={formData.description || ''}
                  readOnly
                  placeholder="As a [role], I want [feature], so that [benefit]..."
                  rows={4}
                  className="bg-muted/50 cursor-not-allowed"
                />
                <p className="text-xs text-muted-foreground">根据角色、功能、价值自动生成</p>
              </div>

              <div className="space-y-2">
                <Label>验收标准</Label>
                <div className="space-y-2">
                  {(formData.acceptanceCriteria || []).map((criterion, index) => (
                    <div key={index} className="flex gap-2">
                      <Textarea
                        value={criterion}
                        onChange={e => updateAcceptanceCriterion(index, e.target.value)}
                        placeholder={`验收标准 ${index + 1}`}
                        rows={2}
                        className="flex-1"
                      />
                      <Button
                        type="button"
                        variant="ghost"
                        size="sm"
                        onClick={() => removeAcceptanceCriterion(index)}
                        className="h-9 w-9 p-0"
                      >
                        <X className="w-4 h-4" />
                      </Button>
                    </div>
                  ))}
                  <Button
                    type="button"
                    variant="outline"
                    size="sm"
                    onClick={addAcceptanceCriterion}
                    className="w-full"
                  >
                    <Plus className="w-4 h-4 mr-2" />
                    添加验收标准
                  </Button>
                </div>
              </div>
            </div>
          </div>

          {/* 元数据组 */}
          <div className="space-y-4">
            <h3 className="text-sm font-semibold text-muted-foreground">元数据</h3>
            <div className="grid grid-cols-2 gap-4">
              <div className="space-y-2">
                <Label htmlFor="priority">优先级</Label>
                <Select
                  value={formData.priority}
                  onValueChange={value => updateField('priority', value as UserStory['priority'])}
                >
                  <SelectTrigger>
                    <SelectValue placeholder="选择优先级" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="P0">P0 - 最高</SelectItem>
                    <SelectItem value="P1">P1 - 高</SelectItem>
                    <SelectItem value="P2">P2 - 中</SelectItem>
                    <SelectItem value="P3">P3 - 低</SelectItem>
                  </SelectContent>
                </Select>
              </div>

              <div className="space-y-2">
                <Label htmlFor="status">状态</Label>
                <Select
                  value={formData.status}
                  onValueChange={value => updateField('status', value as UserStory['status'])}
                >
                  <SelectTrigger>
                    <SelectValue placeholder="选择状态" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="draft">草稿</SelectItem>
                    <SelectItem value="refined">已细化</SelectItem>
                    <SelectItem value="approved">已批准</SelectItem>
                    <SelectItem value="in_development">开发中</SelectItem>
                    <SelectItem value="completed">已完成</SelectItem>
                    <SelectItem value="failed">失败</SelectItem>
                  </SelectContent>
                </Select>
              </div>

              <div className="space-y-2">
                <Label htmlFor="storyPoints">故事点</Label>
                <Input
                  id="storyPoints"
                  type="number"
                  min="0"
                  value={formData.storyPoints || ''}
                  onChange={e => updateField('storyPoints', parseInt(e.target.value) || undefined)}
                  placeholder="估算的故事点"
                />
              </div>

              <div className="space-y-2">
                <Label htmlFor="featureModule">功能模块</Label>
                <Input
                  id="featureModule"
                  value={formData.featureModule || ''}
                  onChange={e => updateField('featureModule', e.target.value)}
                  placeholder="关联的功能模块"
                />
              </div>

              {sprints.length > 0 && (
                <div className="space-y-2">
                  <Label htmlFor="sprintId">所属 Sprint</Label>
                  <Select
                    value={formData.sprintId || 'none'}
                    onValueChange={value =>
                      updateField('sprintId', value === 'none' ? undefined : value)
                    }
                  >
                    <SelectTrigger>
                      <SelectValue placeholder="选择 Sprint" />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="none">无</SelectItem>
                      {sprints.map(sprint => (
                        <SelectItem key={sprint.id} value={sprint.id}>
                          {sprint.name}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                </div>
              )}

              <div className="space-y-2 col-span-2">
                <Label htmlFor="labels">标签（逗号分隔）</Label>
                <Input
                  id="labels"
                  value={(formData.labels || []).join(', ')}
                  onChange={e => handleLabelsChange(e.target.value)}
                  placeholder="例如：前端, 认证, 核心功能"
                />
              </div>
            </div>
          </div>

          {/* 失败信息展示区域（仅在状态为 failed 时显示） */}
          {story.status === 'failed' && (
            <div className="space-y-4">
              <h3 className="text-sm font-semibold text-muted-foreground">失败信息</h3>
              <div className="p-4 bg-red-50 dark:bg-red-950/20 border border-red-200 dark:border-red-800 rounded-md space-y-3">
                {story.errorMessage && (
                  <div>
                    <Label className="text-xs text-red-700 dark:text-red-400">失败原因</Label>
                    <p className="text-sm text-red-800 dark:text-red-300 mt-1">
                      {story.errorMessage}
                    </p>
                  </div>
                )}
                <div className="grid grid-cols-2 gap-4">
                  <div>
                    <Label className="text-xs text-red-700 dark:text-red-400">重试次数</Label>
                    <p className="text-sm text-red-800 dark:text-red-300 mt-1">
                      {story.retryCount || 0}
                    </p>
                  </div>
                  {story.failedAt && (
                    <div>
                      <Label className="text-xs text-red-700 dark:text-red-400">失败时间</Label>
                      <p className="text-sm text-red-800 dark:text-red-300 mt-1">
                        {new Date(story.failedAt).toLocaleString('zh-CN')}
                      </p>
                    </div>
                  )}
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
