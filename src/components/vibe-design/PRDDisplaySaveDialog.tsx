import { Button } from '@/components/ui/button'
import { Progress } from '@/components/ui/progress'
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogFooter,
} from '@/components/ui/dialog'

interface SaveDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  isSaving: boolean
  saveProgress: number
  saveStatus: 'success' | 'error'
  saveMessage: string
  onSave: () => void
}

export function SaveDialog({
  open,
  onOpenChange,
  isSaving,
  saveProgress,
  saveStatus,
  saveMessage,
  onSave,
}: SaveDialogProps) {
  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle>
            {saveStatus === 'success' ? '💾 保存产品需求文档' : '❌ 保存失败'}
          </DialogTitle>
          <DialogDescription>{saveMessage}</DialogDescription>
        </DialogHeader>

        {isSaving && (
          <div className="py-4">
            <div className="flex items-center justify-between mb-2">
              <span className="text-sm text-muted-foreground">保存进度</span>
              <span className="text-sm text-muted-foreground">{saveProgress}%</span>
            </div>
            <Progress value={saveProgress} className="w-full" />

            <div className="mt-4 flex items-center justify-center">
              <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary" />
            </div>
          </div>
        )}

        {!isSaving && saveStatus === 'error' && (
          <DialogFooter>
            <Button onClick={() => onOpenChange(false)}>关闭</Button>
            <Button onClick={onSave} variant="outline">
              重试
            </Button>
          </DialogFooter>
        )}

        {!isSaving && saveStatus === 'success' && (
          <DialogFooter>
            <Button onClick={() => onOpenChange(false)}>确定</Button>
          </DialogFooter>
        )}
      </DialogContent>
    </Dialog>
  )
}
