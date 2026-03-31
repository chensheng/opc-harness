/**
 * 流式输出展示组件
 *
 * 用于实时展示 AI 生成的内容，支持打字机效果和进度显示
 */
import React, { useEffect, useRef } from 'react'
import { useStreaming } from '../hooks/useStreaming'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Progress } from '@/components/ui/progress'
import { ScrollArea } from '@/components/ui/scroll-area'
import { Loader2, StopCircle, RefreshCw } from 'lucide-react'

export interface StreamingDisplayProps {
  /** 提示词 */
  prompt: string
  /** 自动开始 */
  autoStart?: boolean
  /** 流式完成回调 */
  onComplete?: (content: string) => void
  /** 流式错误回调 */
  onError?: (error: string) => void
}

/**
 * 流式输出展示组件
 */
export const StreamingDisplay: React.FC<StreamingDisplayProps> = ({
  prompt,
  autoStart = false,
  onComplete,
  onError,
}) => {
  const { isStreaming, content, progress, error, startStream, stopStream, reset } = useStreaming()
  const scrollRef = useRef<HTMLDivElement>(null)
  const prevContentRef = useRef('')

  // 自动滚动到底部
  useEffect(() => {
    if (scrollRef.current && content.length > prevContentRef.current.length) {
      scrollRef.current.scrollTop = scrollRef.current.scrollHeight
      prevContentRef.current = content
    }
  }, [content])

  // 自动开始
  useEffect(() => {
    if (autoStart && prompt && !isStreaming) {
      startStream(prompt)
    }
  }, [autoStart, prompt, isStreaming, startStream])

  // 完成回调
  useEffect(() => {
    if (!isStreaming && content && onComplete) {
      onComplete(content)
    }
  }, [isStreaming, content, onComplete])

  // 错误回调
  useEffect(() => {
    if (error && onError) {
      onError(error)
    }
  }, [error, onError])

  const handleManualStart = () => {
    if (prompt) {
      startStream(prompt)
    }
  }

  const handleManualStop = () => {
    stopStream()
  }

  const handleReset = () => {
    reset()
  }

  return (
    <Card className="w-full">
      <CardHeader>
        <CardTitle className="flex items-center justify-between">
          <span>AI 流式输出</span>
          <div className="flex gap-2">
            {!isStreaming ? (
              <Button
                variant="outline"
                size="sm"
                onClick={handleManualStart}
                disabled={!prompt || isStreaming}
              >
                <RefreshCw className="w-4 h-4 mr-2" />
                重新生成
              </Button>
            ) : (
              <Button variant="destructive" size="sm" onClick={handleManualStop}>
                <StopCircle className="w-4 h-4 mr-2" />
                停止
              </Button>
            )}
          </div>
        </CardTitle>
      </CardHeader>
      <CardContent className="space-y-4">
        {/* 进度条 */}
        {isStreaming && (
          <div className="space-y-2">
            <div className="flex justify-between text-sm text-muted-foreground">
              <span>生成进度</span>
              <span>{Math.round(progress)}%</span>
            </div>
            <Progress value={progress} className="h-2" />
          </div>
        )}

        {/* 内容显示区域 */}
        <ScrollArea className="h-[400px] w-full rounded-md border p-4">
          <div ref={scrollRef} className="space-y-2">
            {isStreaming && content.length === 0 && (
              <div className="flex items-center gap-2 text-muted-foreground">
                <Loader2 className="w-4 h-4 animate-spin" />
                <span>正在生成...</span>
              </div>
            )}

            {content && (
              <div className="prose dark:prose-invert max-w-none">
                <div className="whitespace-pre-wrap">{content}</div>
              </div>
            )}

            {error && (
              <div className="text-destructive p-4 bg-destructive/10 rounded-md">
                <p className="font-semibold">生成失败</p>
                <p className="text-sm">{error}</p>
              </div>
            )}
          </div>
        </ScrollArea>

        {/* 状态信息 */}
        <div className="flex justify-between text-sm text-muted-foreground">
          <span>
            {isStreaming ? (
              <span className="flex items-center gap-2">
                <Loader2 className="w-3 h-3 animate-spin" />
                生成中...
              </span>
            ) : content ? (
              '生成完成'
            ) : (
              '等待生成'
            )}
          </span>
          <span>{content.length} 字符</span>
        </div>
      </CardContent>
    </Card>
  )
}
