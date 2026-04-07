import { useState } from 'react'
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

interface ExportDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  isExporting: boolean
  exportProgress: number
  exportStatus: 'success' | 'error'
  exportMessage: string
  onRetry: () => void
}

export function ExportDialog({
  open,
  onOpenChange,
  isExporting,
  exportProgress,
  exportStatus,
  exportMessage,
  onRetry,
}: ExportDialogProps) {
  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle>
            {exportStatus === 'success' ? '📦 导出产品需求文档' : '❌ 导出失败'}
          </DialogTitle>
          <DialogDescription>
            {exportStatus === 'success'
              ? isExporting
                ? '正在导出您的 PRD 文档...'
                : exportMessage
              : exportMessage}
          </DialogDescription>
        </DialogHeader>

        {isExporting && (
          <div className="py-4">
            <div className="flex items-center justify-between mb-2">
              <span className="text-sm text-muted-foreground">导出进度</span>
              <span className="text-sm text-muted-foreground">{exportProgress}%</span>
            </div>
            <Progress value={exportProgress} className="w-full" />

            <div className="mt-4 flex items-center justify-center">
              <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary" />
            </div>
          </div>
        )}

        {!isExporting && exportStatus === 'error' && (
          <DialogFooter>
            <Button onClick={() => onOpenChange(false)}>关闭</Button>
            <Button onClick={onRetry} variant="outline">
              重试
            </Button>
          </DialogFooter>
        )}

        {!isExporting && exportStatus === 'success' && (
          <DialogFooter>
            <Button onClick={() => onOpenChange(false)}>确定</Button>
          </DialogFooter>
        )}
      </DialogContent>
    </Dialog>
  )
}
