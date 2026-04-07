import React from 'react'
import ReactMarkdown from 'react-markdown'
import remarkGfm from 'remark-gfm'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Sparkles } from 'lucide-react'
import { StreamingComponents } from './PRDDisplayMarkdownComponents'

interface StreamingViewProps {
  markdownContent: string
  error: string | null
  onStopGeneration: () => void
  onRetry: () => void
}

export function PRDDisplayStreamingView({
  markdownContent,
  error,
  onStopGeneration,
  onRetry,
}: StreamingViewProps) {
  return (
    <div className="max-w-4xl mx-auto space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold flex items-center gap-2">
            <Sparkles className="w-6 h-6 text-primary animate-pulse" />
            AI 正在创作 PRD...
          </h1>
          <p className="text-muted-foreground">产品需求文档生成中</p>
        </div>
        <Button variant="destructive" onClick={onStopGeneration}>
          停止生成
        </Button>
      </div>

      {/* 实时内容预览 */}
      <Card>
        <CardHeader>
          <CardTitle>实时生成预览</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="prose prose-sm max-w-none">
            {markdownContent ? (
              <div className="text-sm leading-relaxed">
                <ReactMarkdown remarkPlugins={[remarkGfm]} components={StreamingComponents}>
                  {markdownContent}
                </ReactMarkdown>
                <span className="inline-block w-2 h-4 bg-primary ml-1 animate-pulse" />
              </div>
            ) : (
              <div className="flex items-center justify-center py-12">
                <div className="text-center">
                  <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary mx-auto" />
                  <p className="mt-4 text-muted-foreground">正在连接 AI...</p>
                </div>
              </div>
            )}
          </div>
        </CardContent>
      </Card>

      {/* 进度提示 */}
      <Card>
        <CardContent className="py-6">
          <div className="space-y-2">
            <div className="flex items-center justify-between text-sm">
              <span className="text-muted-foreground">生成进度</span>
              <span className="text-muted-foreground">
                {Math.min((markdownContent.length / 2000) * 100, 100).toFixed(0)}%
              </span>
            </div>
            <div className="h-2 bg-accent rounded-full overflow-hidden">
              <div
                className="h-full bg-primary transition-all duration-300"
                style={{
                  width: `${Math.min((markdownContent.length / 2000) * 100, 100)}%`,
                }}
              />
            </div>
          </div>
        </CardContent>
      </Card>

      {error && (
        <Card className="border-destructive">
          <CardContent className="py-6">
            <p className="text-destructive">{error}</p>
            <Button onClick={onRetry} className="mt-4" variant="outline">
              重试
            </Button>
          </CardContent>
        </Card>
      )}
    </div>
  )
}
