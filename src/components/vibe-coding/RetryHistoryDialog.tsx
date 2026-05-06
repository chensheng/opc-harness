import React, { useEffect } from 'react'
import { Dialog, DialogContent, DialogHeader, DialogTitle } from '@/components/ui/dialog'
import { RetryHistoryTimeline } from './RetryHistoryTimeline'
import { useRetryHistory } from '@/hooks/useRetryHistory'
import type { UserStory } from '@/types'

interface RetryHistoryDialogProps {
  story: UserStory | null
  open: boolean
  onOpenChange: (open: boolean) => void
}

export function RetryHistoryDialog({ story, open, onOpenChange }: RetryHistoryDialogProps) {
  const { loadHistory, getHistory, isLoading } = useRetryHistory()

  // 当对话框打开时加载重试历史
  useEffect(() => {
    if (open && story) {
      loadHistory(story.id)
    }
  }, [open, story, loadHistory])

  if (!story) return null

  const histories = getHistory(story.id)
  const loading = isLoading(story.id)

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-3xl max-h-[80vh] overflow-hidden flex flex-col">
        <DialogHeader>
          <DialogTitle className="flex items-center gap-2">
            <span>重试历史</span>
            <span className="text-sm font-normal text-muted-foreground">
              - {story.storyNumber}: {story.title}
            </span>
          </DialogTitle>
        </DialogHeader>

        <div className="flex-1 overflow-y-auto mt-4">
          {loading ? (
            <div className="flex items-center justify-center py-12">
              <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary"></div>
            </div>
          ) : (
            <RetryHistoryTimeline histories={histories} />
          )}
        </div>
      </DialogContent>
    </Dialog>
  )
}
