import React from 'react'
import ReactMarkdown from 'react-markdown'
import remarkGfm from 'remark-gfm'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Textarea } from '@/components/ui/textarea'
import { ScrollArea } from '@/components/ui/scroll-area'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Sparkles, FileText, MessageSquare, Loader2, AlertCircle } from 'lucide-react'

interface DecomposeDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  prdContent: string
  prompt: string
  onPromptChange: (prompt: string) => void
  onDecompose: () => Promise<void>
  isStreaming: boolean
  markdownContent: string
  error?: string
}

// PRD 预览的自定义 Markdown 组件
const PRDPreviewComponents = {
  h1: ({ ...props }) => (
    <h1
      className="text-2xl font-bold mb-4 mt-6 pb-2 border-b border-border text-primary"
      {...props}
    />
  ),
  h2: ({ ...props }) => (
    <h2
      className="text-xl font-semibold mb-3 mt-5 pb-1 border-b border-border/50 text-foreground"
      {...props}
    />
  ),
  h3: ({ ...props }) => (
    <h3 className="text-lg font-medium mb-2 mt-4 text-foreground/90" {...props} />
  ),
  p: ({ ...props }) => (
    <p className="text-base leading-relaxed mb-3 text-foreground/90" {...props} />
  ),
  ul: ({ ...props }) => <ul className="list-disc list-outside pl-6 mb-3 space-y-1.5" {...props} />,
  ol: ({ ...props }) => (
    <ol className="list-decimal list-outside pl-6 mb-3 space-y-1.5" {...props} />
  ),
  li: ({ ...props }) => <li className="text-sm leading-relaxed text-foreground/85" {...props} />,
  strong: ({ ...props }) => <strong className="font-semibold text-foreground" {...props} />,
  em: ({ ...props }) => <em className="italic text-foreground/80" {...props} />,
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  code: ({ inline, ...props }: any) =>
    inline ? (
      <code
        className="bg-muted/80 px-1.5 py-0.5 rounded text-xs font-mono text-primary"
        {...props}
      />
    ) : (
      <code
        className="block bg-muted p-2.5 rounded-md my-2 overflow-x-auto text-xs font-mono"
        {...props}
      />
    ),
  pre: ({ ...props }) => (
    <pre
      className="bg-muted/50 p-3 rounded-md my-3 overflow-x-auto border border-border/30"
      {...props}
    />
  ),
  blockquote: ({ ...props }) => (
    <blockquote
      className="border-l-4 border-primary/50 pl-4 py-2 my-3 bg-muted/20 italic text-foreground/75"
      {...props}
    />
  ),
  table: ({ ...props }) => (
    <div className="overflow-x-auto my-6 first:mt-4 last:mb-4">
      <table className="w-full border-collapse border border-border" {...props} />
    </div>
  ),
  th: ({ ...props }) => (
    <th
      className="border border-border px-4 py-3 bg-muted/80 text-left font-semibold text-sm"
      {...props}
    />
  ),
  td: ({ ...props }) => (
    <td className="border border-border px-4 py-3 text-left text-sm" {...props} />
  ),
  tr: ({ ...props }) => (
    <tr className="even:bg-muted/30 hover:bg-muted/50 transition-colors" {...props} />
  ),
}

export function DecomposeDialog({
  open,
  onOpenChange,
  prdContent,
  prompt,
  onPromptChange,
  onDecompose,
  isStreaming,
  markdownContent,
  error,
}: DecomposeDialogProps) {
  // 调试日志：当 error prop 变化时输出
  React.useEffect(() => {
    if (error) {
      console.log('[DecomposeDialog] Error prop received:', error)
    }
  }, [error])

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-6xl h-[85vh] flex flex-col">
        <DialogHeader className="flex-shrink-0">
          <DialogTitle className="flex items-center gap-2">
            <Sparkles className="w-5 h-5" />
            拆分用户故事
          </DialogTitle>
          <DialogDescription>AI 将基于 PRD 内容自动拆分为用户故事</DialogDescription>
        </DialogHeader>

        {/* 主体内容区域 - 使用flex布局确保高度正确传递 */}
        <div className="flex-1 min-h-0 grid grid-cols-2 gap-4">
          {/* PRD Preview - 左侧（可滚动） */}
          <Card className="h-full flex flex-col min-h-0">
            <CardHeader className="pb-2 flex-shrink-0">
              <CardTitle className="flex items-center gap-2 text-base">
                <FileText className="w-4 h-4" />
                项目 PRD
              </CardTitle>
              <CardDescription className="text-xs">AI 将基于此内容拆分</CardDescription>
            </CardHeader>
            <CardContent className="flex-1 min-h-0 overflow-hidden p-0">
              <div className="h-full px-4 pb-4">
                <ScrollArea className="h-full w-full rounded-md border p-3 bg-card" type="always">
                  <div className="min-h-full">
                    {prdContent ? (
                      <div className="prose prose-sm dark:prose-invert max-w-none pr-2">
                        <ReactMarkdown
                          remarkPlugins={[remarkGfm]}
                          components={PRDPreviewComponents}
                        >
                          {prdContent}
                        </ReactMarkdown>
                      </div>
                    ) : (
                      <div className="flex items-center justify-center h-full text-muted-foreground">
                        <div className="text-center">
                          <FileText className="w-8 h-8 mx-auto mb-2 opacity-50" />
                          <p className="text-xs">暂无 PRD 内容</p>
                        </div>
                      </div>
                    )}
                  </div>
                </ScrollArea>
              </div>
            </CardContent>
          </Card>

          {/* Prompt Input - 右侧（固定不滚动） */}
          <Card className="h-full flex flex-col min-h-0">
            <CardHeader className="pb-2 flex-shrink-0">
              <CardTitle className="flex items-center gap-2 text-base">
                <MessageSquare className="w-4 h-4" />
                拆分要求
              </CardTitle>
              <CardDescription className="text-xs">可选的额外要求</CardDescription>
            </CardHeader>
            <CardContent className="flex-1 flex flex-col space-y-3 px-4 pb-4">
              <Textarea
                placeholder="例如：&#10;- 重点关注用户认证&#10;- 优先核心业务流程&#10;- 考虑技术债务..."
                value={prompt}
                onChange={e => onPromptChange(e.target.value)}
                className="min-h-[120px] resize-none text-sm flex-shrink-0"
              />

              {error && (
                <div className="flex items-start gap-2 p-2 bg-red-50 dark:bg-red-950/20 border border-red-200 dark:border-red-800 rounded-md flex-shrink-0">
                  <AlertCircle className="w-4 h-4 text-red-500 mt-0.5 flex-shrink-0" />
                  <div className="text-xs text-red-700 dark:text-red-400">{error}</div>
                </div>
              )}

              {/* 流式响应实时显示 */}
              {isStreaming && (
                <Card className="border-primary/50 bg-gradient-to-br from-primary/5 to-purple-500/5 flex-shrink-0">
                  <CardHeader className="pb-2 pt-2 px-3">
                    <CardTitle className="flex items-center gap-2 text-sm">
                      <Loader2 className="w-4 h-4 animate-spin text-primary" />
                      AI 生成中...
                    </CardTitle>
                  </CardHeader>
                  <CardContent className="px-3 pb-3">
                    <ScrollArea className="h-[200px] w-full rounded-md border p-2 bg-background/50">
                      {markdownContent ? (
                        <div className="prose prose-sm dark:prose-invert max-w-none">
                          <ReactMarkdown
                            remarkPlugins={[remarkGfm]}
                            components={PRDPreviewComponents}
                          >
                            {markdownContent}
                          </ReactMarkdown>
                        </div>
                      ) : (
                        <div className="flex items-center justify-center h-20 text-muted-foreground">
                          <div className="text-center space-y-1">
                            <Loader2 className="w-6 h-6 animate-spin mx-auto" />
                            <p className="text-xs">正在连接 AI...</p>
                          </div>
                        </div>
                      )}
                    </ScrollArea>
                  </CardContent>
                </Card>
              )}

              <Button
                onClick={onDecompose}
                disabled={!prdContent || isStreaming}
                className="w-full flex-shrink-0"
              >
                {isStreaming ? (
                  <>
                    <Sparkles className="w-4 h-4 mr-2 animate-spin" />
                    AI 拆分中...
                  </>
                ) : (
                  <>
                    <Sparkles className="w-4 h-4 mr-2" />
                    开始拆分用户故事
                  </>
                )}
              </Button>

              <div className="text-xs text-muted-foreground space-y-0.5 flex-shrink-0">
                <p>💡 提示：</p>
                <ul className="list-disc list-inside space-y-0.5 ml-1 text-[10px]">
                  <li>AI 自动基于 PRD 拆分</li>
                  <li>可输入额外要求指导拆分</li>
                  <li>遵循 INVEST 原则</li>
                  <li>包含验收标准和优先级</li>
                </ul>
              </div>
            </CardContent>
          </Card>
        </div>
      </DialogContent>
    </Dialog>
  )
}
