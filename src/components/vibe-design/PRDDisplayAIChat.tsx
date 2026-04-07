import { useState, useRef, useEffect } from 'react'
import { Send, Bot, User, X, Sparkles, Check } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Textarea } from '@/components/ui/textarea'
import { ScrollArea } from '@/components/ui/scroll-area'
import type { ChatMessage } from '@/hooks/usePRDAIChat'

interface PRDDisplayAIChatProps {
  messages: ChatMessage[]
  isStreaming: boolean
  error: string | null
  onSendMessage: (message: string) => void
  onStopStream: () => void
  onApplyOptimization: (content: string) => void
  onClose: () => void
}

const QUICK_SUGGESTIONS = [
  '完善目标用户画像',
  '补充技术实现细节',
  '优化产品功能描述',
  '增强商业模式说明',
  '细化用户体验流程',
  '补充竞品分析内容',
]

export function PRDDisplayAIChat({
  messages,
  isStreaming,
  error,
  onSendMessage,
  onStopStream,
  onApplyOptimization,
  onClose,
}: PRDDisplayAIChatProps) {
  const [inputMessage, setInputMessage] = useState('')
  const scrollRef = useRef<HTMLDivElement>(null)
  const textareaRef = useRef<HTMLTextAreaElement>(null)

  // 自动滚动到底部 - 在消息内容变化或流式输出时触发
  useEffect(() => {
    if (scrollRef.current) {
      // 使用 requestAnimationFrame 确保 DOM 更新后再滚动
      requestAnimationFrame(() => {
        scrollRef.current!.scrollTop = scrollRef.current!.scrollHeight
      })
    }
  }, [messages, isStreaming])

  // 专门监听最后一条消息的内容变化(用于流式输出时的实时滚动)
  useEffect(() => {
    if (isStreaming && messages.length > 0) {
      const lastMessage = messages[messages.length - 1]
      if (lastMessage?.role === 'assistant' && lastMessage.content) {
        requestAnimationFrame(() => {
          if (scrollRef.current) {
            scrollRef.current.scrollTop = scrollRef.current.scrollHeight
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

  const handleSend = () => {
    if (!inputMessage.trim() || isStreaming) return
    onSendMessage(inputMessage.trim())
    setInputMessage('')
  }

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault()
      handleSend()
    }
  }

  const handleQuickSuggestion = (suggestion: string) => {
    onSendMessage(suggestion)
  }

  const handleApplyLastOptimization = () => {
    // 获取最后一条助手消息
    const lastAssistantMessage = [...messages].reverse().find(msg => msg.role === 'assistant')
    if (lastAssistantMessage?.content) {
      onApplyOptimization(lastAssistantMessage.content)
    }
  }

  return (
    <Card className="h-[600px] max-h-[calc(100vh-200px)] flex flex-col border-l border-border">
      <CardHeader className="py-3 border-b border-border">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            <Sparkles className="w-5 h-5 text-primary" />
            <CardTitle className="text-sm font-medium">AI 优化助手</CardTitle>
          </div>
          <Button variant="ghost" size="sm" onClick={onClose} className="h-7 w-7 p-0">
            <X className="w-4 h-4" />
          </Button>
        </div>
      </CardHeader>

      <CardContent className="flex-1 flex flex-col p-0 overflow-hidden">
        {/* 消息列表 */}
        <ScrollArea className="flex-1 px-4 py-3" ref={scrollRef}>
          <div className="space-y-4">
            {messages.length === 0 && (
              <div className="text-center py-8 text-muted-foreground">
                <Bot className="w-12 h-12 mx-auto mb-3 opacity-30" />
                <p className="text-sm">输入优化需求，AI 将帮您完善 PRD</p>
              </div>
            )}

            {messages.map((msg, index) => (
              <div
                key={index}
                className={`flex gap-3 ${msg.role === 'user' ? 'flex-row-reverse' : 'flex-row'}`}
              >
                <div
                  className={`flex-shrink-0 w-8 h-8 rounded-full flex items-center justify-center ${
                    msg.role === 'user' ? 'bg-primary text-primary-foreground' : 'bg-muted'
                  }`}
                >
                  {msg.role === 'user' ? (
                    <User className="w-4 h-4" />
                  ) : (
                    <Bot className="w-4 h-4" />
                  )}
                </div>

                <div
                  className={`max-w-[80%] rounded-lg px-3 py-2 text-sm ${
                    msg.role === 'user'
                      ? 'bg-primary text-primary-foreground'
                      : 'bg-muted border border-border'
                  }`}
                >
                  <div className="whitespace-pre-wrap break-words">{msg.content}</div>

                  {/* 如果是最后一条助手消息且已完成，显示应用按钮 */}
                  {msg.role === 'assistant' &&
                    index === messages.length - 1 &&
                    !isStreaming &&
                    msg.content && (
                      <Button
                        size="sm"
                        variant="outline"
                        className="mt-2 h-7 text-xs"
                        onClick={handleApplyLastOptimization}
                      >
                        <Check className="w-3 h-3 mr-1" />
                        应用优化
                      </Button>
                    )}
                </div>
              </div>
            ))}

            {/* 流式指示器 */}
            {isStreaming && (
              <div className="flex gap-3">
                <div className="flex-shrink-0 w-8 h-8 rounded-full bg-muted flex items-center justify-center">
                  <Bot className="w-4 h-4" />
                </div>
                <div className="bg-muted border border-border rounded-lg px-3 py-2">
                  <div className="flex items-center gap-2 text-sm text-muted-foreground">
                    <div className="flex gap-1">
                      <span className="animate-bounce">●</span>
                      <span className="animate-bounce" style={{ animationDelay: '0.1s' }}>
                        ●
                      </span>
                      <span className="animate-bounce" style={{ animationDelay: '0.2s' }}>
                        ●
                      </span>
                    </div>
                    AI 正在生成...
                  </div>
                </div>
              </div>
            )}

            {/* 错误提示 */}
            {error && (
              <div className="bg-destructive/10 border border-destructive/20 rounded-lg px-3 py-2 text-sm text-destructive">
                {error}
              </div>
            )}
          </div>
        </ScrollArea>

        {/* 快捷建议 */}
        {messages.length === 0 && (
          <div className="px-4 pb-3">
            <p className="text-xs text-muted-foreground mb-2">快捷建议：</p>
            <div className="flex flex-wrap gap-2">
              {QUICK_SUGGESTIONS.map(suggestion => (
                <Button
                  key={suggestion}
                  variant="outline"
                  size="sm"
                  className="text-xs h-7"
                  onClick={() => handleQuickSuggestion(suggestion)}
                  disabled={isStreaming}
                >
                  {suggestion}
                </Button>
              ))}
            </div>
          </div>
        )}

        {/* 输入区域 */}
        <div className="border-t border-border p-4">
          <div className="flex gap-2">
            <Textarea
              ref={textareaRef}
              value={inputMessage}
              onChange={e => setInputMessage(e.target.value)}
              onKeyDown={handleKeyDown}
              placeholder="输入优化需求，如：请帮我完善目标用户部分..."
              className="min-h-[60px] resize-none text-sm"
              disabled={isStreaming}
            />
            <div className="flex flex-col gap-2">
              <Button
                size="sm"
                onClick={handleSend}
                disabled={!inputMessage.trim() || isStreaming}
                className="h-[30px]"
              >
                <Send className="w-4 h-4" />
              </Button>
              {isStreaming && (
                <Button
                  size="sm"
                  variant="outline"
                  onClick={onStopStream}
                  className="h-[30px] text-xs"
                >
                  停止
                </Button>
              )}
            </div>
          </div>
          <p className="text-xs text-muted-foreground mt-2">
            💡 按 Enter 发送，Shift + Enter 换行
          </p>
        </div>
      </CardContent>
    </Card>
  )
}
