import { useState, useCallback, useRef } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { useAIConfigStore } from '@/stores/aiConfigStore'

export interface ChatMessage {
  role: 'user' | 'assistant'
  content: string
}

export interface UsePRDAIChatReturn {
  messages: ChatMessage[]
  isStreaming: boolean
  error: string | null
  sendMessage: (userMessage: string, currentPRDContent: string) => Promise<void>
  stopStream: () => void
  reset: () => void
}

/**
 * PRD AI 对话优化 Hook
 * 支持通过自然语言对话方式优化 PRD 内容
 */
export function usePRDAIChat(): UsePRDAIChatReturn {
  const [messages, setMessages] = useState<ChatMessage[]>([])
  const [isStreaming, setIsStreaming] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const aiConfigStore = useAIConfigStore()
  const unlistenRef = useRef<UnlistenFn[]>([])
  const isStreamingRef = useRef(false)
  const accumulatedContentRef = useRef('')

  const cleanup = useCallback(() => {
    unlistenRef.current.forEach(unlisten => {
      try {
        unlisten()
      } catch (err) {
        console.error('[usePRDAIChat] Cleanup error:', err)
      }
    })
    unlistenRef.current = []
  }, [])

  const stopStream = useCallback(() => {
    cleanup()
    isStreamingRef.current = false
    setIsStreaming(false)
  }, [cleanup])

  const reset = useCallback(() => {
    stopStream()
    setMessages([])
    setError(null)
    accumulatedContentRef.current = ''
  }, [stopStream])

  const sendMessage = useCallback(
    async (userMessage: string, currentPRDContent: string) => {
      const activeConfig = aiConfigStore.getActiveConfig()

      // CodeFree CLI 不需要 API Key，其他 provider 需要检查
      if (activeConfig?.provider !== 'codefree' && !activeConfig?.apiKey) {
        setError('请先配置 AI 提供商')
        return
      }

      // 添加用户消息
      const userMsg: ChatMessage = { role: 'user', content: userMessage }
      setMessages(prev => [...prev, userMsg])

      // 重置状态
      cleanup()
      accumulatedContentRef.current = ''
      setError(null)
      setIsStreaming(true)
      isStreamingRef.current = true

      // 添加临时的助手消息占位
      setMessages(prev => [...prev, { role: 'assistant', content: '' }])

      try {
        // 监听流式数据块事件
        const unlistenChunk = await listen<{
          session_id: string
          content: string
          is_complete: boolean
        }>('ai-stream-chunk', event => {
          accumulatedContentRef.current += event.payload.content

          // 更新最后一条助手消息
          setMessages(prev => {
            const newMessages = [...prev]
            const lastIndex = newMessages.length - 1
            if (lastIndex >= 0 && newMessages[lastIndex].role === 'assistant') {
              newMessages[lastIndex] = {
                ...newMessages[lastIndex],
                content: accumulatedContentRef.current,
              }
            }
            return newMessages
          })
        })
        unlistenRef.current.push(unlistenChunk)

        // 监听完成事件
        const unlistenComplete = await listen<{ session_id: string; content: string }>(
          'ai-stream-complete',
          _event => {
            setIsStreaming(false)
            isStreamingRef.current = false
            cleanup()
          }
        )
        unlistenRef.current.push(unlistenComplete)

        // 监听错误事件
        const unlistenError = await listen<{ session_id: string; error: string }>(
          'ai-stream-error',
          event => {
            console.error('[usePRDAIChat] Stream error:', event.payload.error)
            setError(event.payload.error)
            setIsStreaming(false)
            isStreamingRef.current = false
            cleanup()
          }
        )
        unlistenRef.current.push(unlistenError)

        // 构建系统提示词，指导 AI 输出完整 PRD
        const systemPrompt = `你是一个专业的产品经理助手。你的任务是基于当前 PRD 内容和用户的优化需求，生成优化后的完整 PRD 文档。

重要规则：
1. 必须输出完整的 PRD Markdown 文档，保持原有结构和风格
2. 即使用户只要求修改某一部分，也要输出包含该部分优化的完整文档
3. 不要输出解释性文字、修改建议列表或非完整文档片段
4. 未修改的部分保持原样
5. 确保文档的专业性和可读性

当前 PRD 内容：
${currentPRDContent}

请基于以上 PRD 和用户需求，生成优化后的完整 PRD 文档。`

        // 调用后端流式聊天命令
        await invoke<string>('stream_chat', {
          request: {
            provider: aiConfigStore.defaultProvider,
            model: activeConfig.model,
            api_key: activeConfig.apiKey,
            messages: [
              { role: 'system', content: systemPrompt },
              { role: 'user', content: userMessage },
            ],
            temperature: 0.7,
            max_tokens: 8000,
          },
        })
      } catch (err) {
        console.error('[usePRDAIChat] Error sending message:', err)
        // 只有在还没有设置详细错误信息时，才设置通用错误
        // 因为详细的错误信息应该已经通过 ai-stream-error 事件设置了
        if (!error) {
          const errorMessage = err instanceof Error ? err.message : String(err)
          setError(`AI 调用失败：${errorMessage}`)
        }
        setIsStreaming(false)
        isStreamingRef.current = false
        cleanup()
      }
    },
    [aiConfigStore, cleanup]
  )

  return {
    messages,
    isStreaming,
    error,
    sendMessage,
    stopStream,
    reset,
  }
}
