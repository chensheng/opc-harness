import { useEffect } from 'react'
import { Key, MessageSquare, Play, Square, Send, Check, X } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import { Textarea } from '@/components/ui/textarea'
import { formatTokens } from '../utils'

interface ProviderConfiguredProps {
  provider: {
    id: string
    name: string
    models: Array<{ id: string; name: string; maxTokens: number }>
  }
  existingConfig: {
    model: string
    apiKey: string
  }
  isTesting: boolean
  testMessage: string
  streamContent: string
  isStreaming: boolean
  isComplete: boolean
  streamError: string | null
  nonStreamResponse: string
  nonStreamLoading: boolean
  nonStreamError: string | null
  onModelChange: (model: string) => void
  onRemove: () => void
  onTestStream: () => void
  _onStopStream: () => void
  onResetStream: () => void
  onTestMessageChange: (message: string) => void
  onTestNonStream: () => void
  onClearNonStreamResponse: () => void
  onClearNonStreamError: () => void
}

/**
 * AI 提供商已配置组件（已配置状态）
 */
export function ProviderConfigured({
  provider,
  existingConfig,
  isTesting,
  testMessage,
  streamContent,
  isStreaming,
  isComplete,
  streamError,
  nonStreamResponse,
  nonStreamLoading,
  nonStreamError,
  onModelChange,
  onRemove,
  onTestStream,
  _onStopStream,
  onResetStream,
  onTestMessageChange,
  onTestNonStream,
  onClearNonStreamResponse,
  onClearNonStreamError,
}: ProviderConfiguredProps) {
  // 监听流式测试错误，当出现错误时自动重置
  useEffect(() => {
    if (streamError && isTesting) {
      console.log('[useEffect] 检测到流式测试错误，重置状态')
      onResetStream()
    }
  }, [streamError, isTesting, onResetStream])

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between p-3 bg-muted rounded-lg">
        <div className="space-y-2 w-full">
          <div className="flex items-center gap-2">
            <Key className="w-4 h-4 text-muted-foreground" />
            <span className="text-sm font-mono">
              {existingConfig.apiKey.slice(0, 8)}...
              {existingConfig.apiKey.slice(-4)}
            </span>
          </div>

          {/* 模型选择 */}
          <div className="flex items-center gap-2">
            <span className="text-xs text-muted-foreground">当前模型:</span>
            <select
              value={existingConfig.model}
              onChange={e => onModelChange(e.target.value)}
              className="text-xs border rounded px-2 py-1 bg-background hover:bg-accent cursor-pointer"
            >
              {provider.models.map(model => (
                <option key={model.id} value={model.id}>
                  {model.name} ({formatTokens(model.maxTokens)})
                </option>
              ))}
            </select>
          </div>
        </div>
        <Button variant="ghost" size="sm" onClick={onRemove}>
          <X className="w-4 h-4 mr-2" />
          删除
        </Button>
      </div>

      {/* 流式测试区域 */}
      <div className="border-t pt-4">
        <div className="flex items-center gap-2 mb-3">
          <MessageSquare className="w-4 h-4 text-primary" />
          <span className="text-sm font-medium">流式测试</span>
        </div>

        <div className="space-y-2">
          <Textarea
            value={testMessage}
            onChange={e => onTestMessageChange(e.target.value)}
            placeholder="输入测试消息..."
            disabled={isStreaming}
            rows={2}
          />

          <div className="flex gap-2">
            <Button
              onClick={onTestStream}
              disabled={isStreaming && !isTesting}
              variant={isTesting ? 'destructive' : 'default'}
              size="sm"
            >
              {isTesting ? (
                <>
                  <Square className="w-4 h-4 mr-2" />
                  停止
                </>
              ) : (
                <>
                  <Play className="w-4 h-4 mr-2" />
                  测试流式
                </>
              )}
            </Button>

            <Button onClick={onTestNonStream} disabled={nonStreamLoading} variant="outline">
              <Send className="w-4 h-4 mr-2" />
              测试非流式
            </Button>
          </div>

          {/* 流式输出显示 */}
          {(isStreaming || streamContent || streamError) && (
            <div className="mt-3 p-3 bg-muted rounded-lg min-h-[100px]">
              {isStreaming && (
                <div className="mb-2 text-xs text-muted-foreground animate-pulse">
                  AI 正在思考中...
                </div>
              )}
              <div className="text-sm whitespace-pre-wrap">{streamContent || '等待响应...'}</div>
              {isComplete && (
                <Badge className="mt-2 bg-green-500">
                  <Check className="w-3 h-3 mr-1" />
                  完成
                </Badge>
              )}
              {streamError && (
                <div className="mt-3 p-4 bg-red-50 border-2 border-red-300 rounded-lg">
                  <div className="flex items-start justify-between gap-3">
                    <div className="flex items-start gap-3 flex-1">
                      <X className="w-5 h-5 text-red-600 flex-shrink-0 mt-0.5" />
                      <div className="flex-1">
                        <div className="font-semibold text-red-900 mb-2">流式测试失败</div>
                        <div className="text-sm text-red-800 bg-white/70 p-3 rounded font-mono whitespace-pre-wrap break-all border border-red-200">
                          {streamError}
                        </div>
                      </div>
                    </div>
                    <Button
                      variant="ghost"
                      size="sm"
                      onClick={onResetStream}
                      className="text-red-600 hover:text-red-800 hover:bg-red-100"
                    >
                      <X className="w-4 h-4" />
                    </Button>
                  </div>
                </div>
              )}
            </div>
          )}

          {nonStreamResponse && (
            <div className="mt-3 p-4 bg-white border-2 border-primary rounded-lg">
              <div className="flex items-start justify-between gap-3">
                <div className="flex items-start gap-3 flex-1">
                  <MessageSquare className="w-5 h-5 text-primary flex-shrink-0 mt-0.5" />
                  <div className="flex-1">
                    <div className="font-semibold text-primary mb-2">非流式测试结果</div>
                    <div className="text-sm text-primary bg-white/70 p-3 rounded font-mono whitespace-pre-wrap break-all border border-primary/20">
                      {nonStreamResponse}
                    </div>
                  </div>
                </div>
                <Button
                  variant="ghost"
                  size="sm"
                  onClick={onClearNonStreamResponse}
                  className="text-primary hover:text-primary-800 hover:bg-primary-100"
                >
                  <X className="w-4 h-4" />
                </Button>
              </div>
            </div>
          )}

          {nonStreamError && (
            <div className="mt-3 p-4 bg-red-50 border-2 border-red-300 rounded-lg">
              <div className="flex items-start justify-between gap-3">
                <div className="flex items-start gap-3 flex-1">
                  <X className="w-5 h-5 text-red-600 flex-shrink-0 mt-0.5" />
                  <div className="flex-1">
                    <div className="font-semibold text-red-900 mb-2">非流式测试失败</div>
                    <div className="text-sm text-red-800 bg-white/70 p-3 rounded font-mono whitespace-pre-wrap break-all border border-red-200">
                      {nonStreamError}
                    </div>
                  </div>
                </div>
                <Button
                  variant="ghost"
                  size="sm"
                  onClick={onClearNonStreamError}
                  className="text-red-600 hover:text-red-800 hover:bg-red-100"
                >
                  <X className="w-4 h-4" />
                </Button>
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  )
}
