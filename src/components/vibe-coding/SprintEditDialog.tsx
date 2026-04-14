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
import type { Sprint } from '@/types'
import { Badge } from '@/components/ui/badge'

interface SprintEditDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  sprint: Sprint | null
  onSave: (updatedSprint: Sprint) => Promise<void>
}

const statusColors: Record<Sprint['status'], string> = {
  planning: 'bg-gray-100 text-gray-700 dark:bg-gray-800 dark:text-gray-300',
  active: 'bg-blue-100 text-blue-700 dark:bg-blue-900/30 dark:text-blue-400',
  completed: 'bg-green-100 text-green-700 dark:bg-green-900/30 dark:text-green-400',
  cancelled: 'bg-red-100 text-red-700 dark:bg-red-900/30 dark:text-red-400',
}

export function SprintEditDialog({ open, onOpenChange, sprint, onSave }: SprintEditDialogProps) {
  const [formData, setFormData] = useState<Partial<Sprint>>({})
  const [errors, setErrors] = useState<Record<string, string>>({})
  const [isSaving, setIsSaving] = useState(false)

  // 初始化表单数据
  useEffect(() => {
    console.log('[SprintEditDialog] sprint prop changed:', sprint)
    if (sprint) {
      console.log('[SprintEditDialog] Setting form data from sprint:', {
        name: sprint.name,
        goal: sprint.goal,
        startDate: sprint.startDate,
        endDate: sprint.endDate,
        status: sprint.status,
      })
      setFormData({
        name: sprint.name,
        goal: sprint.goal,
        startDate: sprint.startDate,
        endDate: sprint.endDate,
        status: sprint.status,
      })
      setErrors({})
    } else {
      // 新建Sprint时的默认值
      const today = new Date().toISOString().split('T')[0]
      const nextWeek = new Date(Date.now() + 7 * 24 * 60 * 60 * 1000).toISOString().split('T')[0]
      console.log('[SprintEditDialog] Setting default form data for new sprint')
      setFormData({
        name: '',
        goal: '',
        startDate: today,
        endDate: nextWeek,
        status: 'planning',
      })
      setErrors({})
    }
  }, [sprint])

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
      const newSprint: Sprint = {
        id: sprint?.id || crypto.randomUUID(),
        name: formData.name || '',
        goal: formData.goal || '',
        startDate: formData.startDate || '',
        endDate: formData.endDate || '',
        status: formData.status || 'planning',
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

  if (!sprint && !open) return null

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-2xl">
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
