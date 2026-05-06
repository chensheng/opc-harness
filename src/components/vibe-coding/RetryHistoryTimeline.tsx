import React from 'react'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { ScrollArea } from '@/components/ui/scroll-area'
import type { UserStoryRetryHistory } from '@/types'
import { Clock, CheckCircle2, XCircle, AlertCircle, RefreshCw } from 'lucide-react'

interface RetryHistoryTimelineProps {
  histories: UserStoryRetryHistory[]
}

export function RetryHistoryTimeline({ histories }: RetryHistoryTimelineProps) {
  if (histories.length === 0) {
    return (
      <div className="text-center py-8 text-muted-foreground">
        <RefreshCw className="w-12 h-12 mx-auto mb-2 opacity-50" />
        <p>暂无重试记录</p>
      </div>
    )
  }

  // 按时间排序（最新的在前）
  const sortedHistories = [...histories].sort(
    (a, b) => new Date(b.triggeredAt).getTime() - new Date(a.triggeredAt).getTime()
  )

  return (
    <ScrollArea className="h-[400px] pr-4">
      <div className="space-y-4">
        {sortedHistories.map((history, index) => (
          <RetryHistoryItem
            key={history.id}
            history={history}
            isLast={index === sortedHistories.length - 1}
          />
        ))}
      </div>
    </ScrollArea>
  )
}

interface RetryHistoryItemProps {
  history: UserStoryRetryHistory
  isLast: boolean
}

function RetryHistoryItem({ history, isLast }: RetryHistoryItemProps) {
  const getStatusIcon = () => {
    switch (history.result) {
      case 'success':
        return <CheckCircle2 className="w-5 h-5 text-green-500" />
      case 'failed':
        return <XCircle className="w-5 h-5 text-red-500" />
      case 'pending':
        return <Clock className="w-5 h-5 text-yellow-500" />
      default:
        return <AlertCircle className="w-5 h-5 text-gray-500" />
    }
  }

  const getStatusBadge = () => {
    const variants = {
      success: 'default' as const,
      failed: 'destructive' as const,
      pending: 'secondary' as const,
    }

    const labels = {
      success: '成功',
      failed: '失败',
      pending: '待处理',
    }

    const variant = variants[history.result || 'pending']
    const label = labels[history.result || 'pending']

    return <Badge variant={variant}>{label}</Badge>
  }

  const getDecisionBadge = () => {
    if (history.decision === 'retry') {
      return (
        <Badge variant="outline" className="border-blue-500 text-blue-600">
          重试
        </Badge>
      )
    } else {
      return (
        <Badge variant="outline" className="border-red-500 text-red-600">
          终止
        </Badge>
      )
    }
  }

  const formatTime = (timeStr: string) => {
    try {
      const date = new Date(timeStr)
      return date.toLocaleString('zh-CN', {
        year: 'numeric',
        month: '2-digit',
        day: '2-digit',
        hour: '2-digit',
        minute: '2-digit',
        second: '2-digit',
      })
    } catch {
      return timeStr
    }
  }

  return (
    <div className="relative">
      {/* 时间线连接线 */}
      {!isLast && <div className="absolute left-6 top-12 bottom-0 w-0.5 bg-border" />}

      <Card className="relative">
        <CardHeader className="pb-3">
          <div className="flex items-start justify-between">
            <div className="flex items-center gap-3">
              {getStatusIcon()}
              <div>
                <CardTitle className="text-base">第 {history.retryNumber} 次重试</CardTitle>
                <p className="text-sm text-muted-foreground">{formatTime(history.triggeredAt)}</p>
              </div>
            </div>
            <div className="flex gap-2">
              {getStatusBadge()}
              {getDecisionBadge()}
            </div>
          </div>
        </CardHeader>
        <CardContent className="pt-0 space-y-3">
          {/* 错误信息 */}
          {history.errorMessage && (
            <div>
              <p className="text-sm font-medium text-muted-foreground mb-1">错误信息：</p>
              <p className="text-sm text-red-600 dark:text-red-400 bg-red-50 dark:bg-red-950/20 p-2 rounded">
                {history.errorMessage}
              </p>
            </div>
          )}

          {/* 错误类型 */}
          {history.errorType && (
            <div>
              <p className="text-sm font-medium text-muted-foreground mb-1">错误类型：</p>
              <Badge variant={history.errorType === 'temporary' ? 'outline' : 'destructive'}>
                {history.errorType === 'temporary' ? '临时错误' : '永久错误'}
              </Badge>
            </div>
          )}

          {/* 下次重试时间 */}
          {history.nextRetryAt && history.decision === 'retry' && (
            <div>
              <p className="text-sm font-medium text-muted-foreground mb-1">预计下次重试：</p>
              <p className="text-sm text-blue-600 dark:text-blue-400">
                {formatTime(history.nextRetryAt)}
              </p>
            </div>
          )}

          {/* 完成时间 */}
          {history.completedAt && (
            <div>
              <p className="text-sm font-medium text-muted-foreground mb-1">完成时间：</p>
              <p className="text-sm">{formatTime(history.completedAt)}</p>
            </div>
          )}
        </CardContent>
      </Card>
    </div>
  )
}
