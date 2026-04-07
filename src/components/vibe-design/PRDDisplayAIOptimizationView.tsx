import { useEffect, useRef, useState } from 'react'
import ReactMarkdown from 'react-markdown'
import remarkGfm from 'remark-gfm'
import { Button } from '@/components/ui/button'
import { Card, CardContent } from '@/components/ui/card'
import { Textarea } from '@/components/ui/textarea'
import { Sparkles, StopCircle, ArrowLeft, Check, Send } from 'lucide-react'
import { StreamingComponents } from './PRDDisplayMarkdownComponents'
import { usePRDAIChat } from '@/hooks/usePRDAIChat'

interface AIOptimizationViewProps {
  currentPRDContent: string
  onApplyOptimization: (content: string) => void
  onBack: () => void
  onSaveToDatabase?: (content: string) => Promise<void>
}

// 快捷提示词列表
const QUICK_PROMPTS = [
  '完善目标用户画像',
  '补充技术实现细节',
  '优化产品功能描述',
  '细化用户体验流程',
  '补充竞品分析内容',
]

export function PRDDisplayAIOptimizationView({
  currentPRDContent,
  onApplyOptimization,
  onBack,
  onSaveToDatabase,
}: AIOptimizationViewProps) {
  const [inputMessage, setInputMessage] = useState('')
  const contentEndRef = useRef<HTMLDivElement>(null)
  const scrollContainerRef = useRef<HTMLDivElement>(null)
  const textareaRef = useRef<HTMLTextAreaElement>(null)

  const {
    messages,
    isStreaming,
    error,
    sendMessage,
    stopStream,
    reset,
  } = usePRDAIChat()

  // 自动滚动到底部
  useEffect(() => {
    if (contentEndRef.current && scrollContainerRef.current) {
      requestAnimationFrame(() => {
        contentEndRef.current!.scrollIntoView({ behavior: 'smooth', block: 'end' })
      })
    }
  }, [messages])

  // 专门监听最后一条消息的内容变化(用于流式输出时的实时滚动)
  useEffect(() => {
    if (isStreaming && messages.length > 0) {
      const lastMessage = messages[messages.length - 1]
      if (lastMessage?.role === 'assistant' && lastMessage.content) {
        requestAnimationFrame(() => {
          if (scrollContainerRef.current) {
            scrollContainerRef.current.scrollTop = scrollContainerRef.current.scrollHeight
          }
        })
      }
    }
  }, [messages[messages.length - 1]?.content, isStreaming])

  // 自动聚焦输入框
  useEffect(() => {
    if (textareaRef.current && !isStreaming) {
      textareaRef.current.focus()
    }
  }, [isStreaming])

  const handleSend = async () => {
    if (!inputMessage.trim() || isStreaming) return
    await sendMessage(inputMessage.trim(), currentPRDContent)
    setInputMessage('')
  }

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault()
      handleSend()
    }
  }

  const handleQuickPrompt = (prompt: string) => {
    if (isStreaming) return
    setInputMessage(prompt)
    // 自动聚焦到输入框
    if (textareaRef.current) {
      textareaRef.current.focus()
    }
  }

  const handleApplyOptimization = async () => {
    // 获取最后一条助手消息
    const lastAssistantMessage = [...messages].reverse().find(msg => msg.role === 'assistant')
    if (lastAssistantMessage?.content) {
      // 先应用优化内容
      onApplyOptimization(lastAssistantMessage.content)
      
      // 如果提供了保存函数，则自动保存到数据库
      if (onSaveToDatabase) {
        try {
          await onSaveToDatabase(lastAssistantMessage.content)
          console.log('[AI Optimization] Content saved to database successfully')
        } catch (error) {
          console.error('[AI Optimization] Failed to save to database:', error)
          // 即使保存失败，也不影响应用优化内容
        }
      }
    }
  }

  // 获取最后一条助手消息
  const lastAssistantMessage = [...messages].reverse().find(msg => msg.role === 'assistant')
  const hasOptimizedContent = lastAssistantMessage?.content && !isStreaming

  return (
    <div className="max-w-4xl mx-auto space-y-6">
      {/* 顶部标题栏和操作按钮 */}
      <div className="flex items-center justify-between sticky top-0 bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60 z-10 py-4 border-b gap-4">
        <div className="flex-1">
          <h1 className="text-2xl font-bold flex items-center gap-2">
            <Sparkles className="w-6 h-6 text-primary animate-pulse" />
            AI 优化助手
          </h1>
          <p className="text-muted-foreground text-sm mt-1">
            {lastAssistantMessage?.content 
              ? `已生成 ${lastAssistantMessage.content.length} 字符` 
              : '输入优化需求，AI 将帮您完善 PRD'}
          </p>
        </div>
        <div className="flex gap-2 shrink-0">
          {isStreaming ? (
            <Button
              variant="destructive"
              onClick={stopStream}
              size="lg"
              className="gap-2"
            >
              <StopCircle className="w-4 h-4" />
              停止生成
            </Button>
          ) : (
            <Button
              variant="outline"
              onClick={onBack}
              size="lg"
              className="gap-2"
            >
              <ArrowLeft className="w-4 h-4" />
              返回编辑
            </Button>
          )}
          {/* 应用优化按钮 - 有优化内容时始终显示 */}
          {hasOptimizedContent && !isStreaming && (
            <Button
              onClick={handleApplyOptimization}
              size="lg"
              className="gap-2"
            >
              <Check className="w-4 h-4" />
              应用优化
            </Button>
          )}
        </div>
      </div>

      {/* 实时内容预览 - 可滚动区域 */}
      <Card className="min-h-[70vh]">
        <CardContent className="py-6">
          <div
            ref={scrollContainerRef}
            className="prose prose-sm max-w-none overflow-y-auto max-h-[calc(100vh-280px)] scroll-smooth"
          >
            {lastAssistantMessage?.content ? (
              <div className="text-sm leading-relaxed space-y-4">
                <ReactMarkdown remarkPlugins={[remarkGfm]} components={StreamingComponents}>
                  {lastAssistantMessage.content}
                </ReactMarkdown>
                {/* 流式光标 */}
                {isStreaming && (
                  <span className="inline-block w-2 h-4 bg-primary ml-1 animate-pulse" />
                )}
                {/* 用于自动滚动的锚点 */}
                <div ref={contentEndRef} />
              </div>
            ) : (
              <div className="flex items-center justify-center py-12">
                <div className="text-center">
                  <Sparkles className="w-16 h-16 mx-auto mb-4 text-primary opacity-30" />
                  <p className="text-muted-foreground">输入优化需求，开始优化您的 PRD</p>
                  <p className="text-sm text-muted-foreground mt-2">
                    例如：完善目标用户画像、补充技术实现细节等
                  </p>
                </div>
              </div>
            )}
          </div>
        </CardContent>
      </Card>

      {/* 错误提示 */}
      {error && (
        <Card className="border-destructive">
          <CardContent className="py-6">
            <p className="text-destructive">{error}</p>
          </CardContent>
        </Card>
      )}

      {/* 底部输入区域 */}
      <div className="sticky bottom-0 bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60 border-t py-4">
        <div className="max-w-4xl mx-auto">
          {/* 快捷提示词按钮 */}
          {!lastAssistantMessage?.content && (
            <div className="mb-3">
              <p className="text-xs text-muted-foreground mb-2">💡 快捷优化建议：</p>
              <div className="flex flex-wrap gap-2">
                {QUICK_PROMPTS.map((prompt, index) => (
                  <Button
                    key={index}
                    variant="outline"
                    size="sm"
                    className="text-xs h-7"
                    onClick={() => handleQuickPrompt(prompt)}
                    disabled={isStreaming}
                  >
                    {prompt}
                  </Button>
                ))}
              </div>
            </div>
          )}

          <div className="flex gap-2">
            <Textarea
              ref={textareaRef}
              value={inputMessage}
              onChange={e => setInputMessage(e.target.value)}
              onKeyDown={handleKeyDown}
              placeholder="输入优化需求，如：请帮我完善目标用户部分..."
              className="min-h-[45px] resize-none text-sm"
              disabled={isStreaming}
            />
            <Button
              size="lg"
              onClick={handleSend}
              disabled={!inputMessage.trim() || isStreaming}
              className="h-[45px] px-6"
            >
              <Send className="w-5 h-5" />
            </Button>
          </div>
          <p className="text-xs text-muted-foreground mt-2">
            💡 按 Enter 发送，Shift + Enter 换行 | AI 将基于当前 PRD 生成优化后的完整文档
          </p>
        </div>
      </div>
    </div>
  )
}
