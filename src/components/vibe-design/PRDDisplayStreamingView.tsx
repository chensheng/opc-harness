import { useEffect, useRef } from 'react'
import ReactMarkdown from 'react-markdown'
import remarkGfm from 'remark-gfm'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Sparkles, StopCircle } from 'lucide-react'
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
  const contentEndRef = useRef<HTMLDivElement>(null)
  const scrollContainerRef = useRef<HTMLDivElement>(null)

  // 自动滚动到底部
  useEffect(() => {
    if (contentEndRef.current && scrollContainerRef.current) {
      // 平滑滚动到底部
      contentEndRef.current.scrollIntoView({ behavior: 'smooth', block: 'end' })
    }
  }, [markdownContent])

  return (
    <div className="max-w-4xl mx-auto space-y-6">
      {/* 顶部标题栏和操作按钮 */}
      <div className="flex items-center justify-between sticky top-0 bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60 z-10 py-4 border-b gap-4">
        <div className="flex-1">
          <h1 className="text-2xl font-bold flex items-center gap-2">
            <Sparkles className="w-6 h-6 text-primary animate-pulse" />
            AI 正在创作 PRD...
          </h1>
          <p className="text-muted-foreground text-sm mt-1">
            {markdownContent ? `已生成 ${markdownContent.length} 字符` : '产品需求文档生成中'}
          </p>
        </div>
        <Button 
          variant="destructive" 
          onClick={onStopGeneration}
          size="lg"
          className="gap-2 shrink-0"
        >
          <StopCircle className="w-4 h-4" />
          停止生成
        </Button>
      </div>

      {/* 实时内容预览 - 可滚动区域 */}
      <Card className="min-h-[70vh]">
        <CardHeader>
          <CardTitle>实时生成预览</CardTitle>
        </CardHeader>
        <CardContent>
          <div 
            ref={scrollContainerRef}
            className="prose prose-sm max-w-none overflow-y-auto max-h-[calc(100vh-280px)] scroll-smooth"
          >
            {markdownContent ? (
              <div className="text-sm leading-relaxed space-y-4">
                <ReactMarkdown remarkPlugins={[remarkGfm]} components={StreamingComponents}>
                  {markdownContent}
                </ReactMarkdown>
                <span className="inline-block w-2 h-4 bg-primary ml-1 animate-pulse" />
                {/* 用于自动滚动的锚点 */}
                <div ref={contentEndRef} />
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
