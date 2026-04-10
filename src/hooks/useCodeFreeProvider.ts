import { useState, useCallback } from 'react'

export interface Message {
  role: string
  content: string
}

export interface ChatRequest {
  model: string
  messages: Message[]
  temperature?: number
  maxTokens?: number
  stream?: boolean
}

export interface ChatResponse {
  content: string
  model: string
  usage?: {
    promptTokens: number
    completionTokens: number
    totalTokens: number
  }
}

export interface UseCodeFreeProviderReturn {
  isLoading: boolean
  error: string | null
  chat: (request: ChatRequest) => Promise<ChatResponse | null>
  streamChat: (request: ChatRequest, onChunk: (content: string) => void) => Promise<string | null>
  validateApiKey: (apiKey: string) => Promise<boolean>
}

/**
 * CodeFree Provider Hook - 提供与 CodeFree CLI 交互的能力
 * CodeFree 是一个本地 CLI 工具，不需要 API Key
 */
export function useCodeFreeProvider(): UseCodeFreeProviderReturn {
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  /**
   * 验证 CodeFree CLI 是否安装
   */
  const validateApiKey = useCallback(async (_apiKey: string): Promise<boolean> => {
    setIsLoading(true)
    setError(null)

    try {
      console.log('[useCodeFreeProvider] Validating CodeFree CLI installation...')

      // TODO: 实现 Tauri command 调用检测 CLI
      // const result = await invoke<boolean>('validate_codefree_cli')
      // return result

      // Mock 实现（暂时）- 假设 CLI 已安装
      await new Promise(resolve => setTimeout(resolve, 500))
      console.log('[useCodeFreeProvider] CodeFree CLI validation successful (mock)')
      return true
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'CodeFree CLI 验证失败'
      console.error('[useCodeFreeProvider] Validation failed:', errorMessage)
      setError(errorMessage)
      return false
    } finally {
      setIsLoading(false)
    }
  }, [])

  /**
   * 发送聊天请求（非流式）
   */
  const chat = useCallback(async (request: ChatRequest): Promise<ChatResponse | null> => {
    setIsLoading(true)
    setError(null)

    try {
      console.log('[useCodeFreeProvider] Sending chat request via CodeFree CLI:', request)

      // TODO: 实现 Tauri command 调用
      // const response = await invoke<ChatResponse>('codefree_chat', { request })
      // return response

      // Mock 实现（暂时）
      await new Promise(resolve => setTimeout(resolve, 1500))
      return {
        content:
          '这是一个模拟的 CodeFree CLI 响应。真实功能将在连接 Tauri 后端后实现。CodeFree 通过本地 CLI 工具提供 AI 能力，无需 API Key。',
        model: request.model,
        usage: undefined, // CLI 不提供 token 统计
      }
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : '聊天请求失败'
      console.error('[useCodeFreeProvider] Chat failed:', errorMessage)
      setError(errorMessage)
      return null
    } finally {
      setIsLoading(false)
    }
  }, [])

  /**
   * 发送流式聊天请求
   */
  const streamChat = useCallback(
    async (request: ChatRequest, onChunk: (content: string) => void): Promise<string | null> => {
      setIsLoading(true)
      setError(null)

      try {
        console.log('[useCodeFreeProvider] Sending stream chat request via CodeFree CLI:', request)

        // TODO: 实现 Tauri command 调用和流式处理
        // const fullContent = await invoke<string>('codefree_stream_chat', {
        //   request,
        //   onChunk
        // })
        // return fullContent

        // Mock 实现（暂时）
        const mockResponse =
          '这是一个通过 CodeFree CLI 的流式响应模拟内容。每个字都会逐步显示。CodeFree 是本地 CLI 工具，支持多种 AI 模型。'
        const chunks = mockResponse.split('')

        for (const chunk of chunks) {
          onChunk(chunk)
          await new Promise(resolve => setTimeout(resolve, 30))
        }

        return mockResponse
      } catch (err) {
        const errorMessage = err instanceof Error ? err.message : '流式聊天失败'
        console.error('[useCodeFreeProvider] Stream chat failed:', errorMessage)
        setError(errorMessage)
        return null
      } finally {
        setIsLoading(false)
      }
    },
    []
  )

  return {
    isLoading,
    error,
    chat,
    streamChat,
    validateApiKey,
  }
}
