import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Button } from '@/components/ui/button'
import { Loader2 } from 'lucide-react'
import type { UserStory } from '@/types'

interface UserStoryDeleteDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  story: UserStory | null
  isDeleting: boolean
  onConfirm: () => Promise<void>
  onCancel: () => void
}

export function UserStoryDeleteDialog({
  open,
  onOpenChange,
  story,
  isDeleting,
  onConfirm,
  onCancel,
}: UserStoryDeleteDialogProps) {
  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>确认删除</DialogTitle>
          <DialogDescription>
            确定要删除用户故事"{story?.title}"吗？此操作不可恢复。
          </DialogDescription>
        </DialogHeader>
        <DialogFooter>
          <Button variant="outline" onClick={onCancel} disabled={isDeleting}>
            取消
          </Button>
          <Button variant="destructive" onClick={onConfirm} disabled={isDeleting}>
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
  )
}
