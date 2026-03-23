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

export interface UseOpenAIProviderReturn {
  isLoading: boolean
  error: string | null
  chat: (request: ChatRequest) => Promise<ChatResponse | null>
  streamChat: (request: ChatRequest, onChunk: (content: string) => void) => Promise<string | null>
  validateApiKey: (apiKey: string) => Promise<boolean>
}

/**
 * OpenAI Provider Hook - 提供与 OpenAI API 交互的能力
 */
export function useOpenAIProvider(): UseOpenAIProviderReturn {
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  /**
   * 验证 API Key
   */
  const validateApiKey = useCallback(async (apiKey: string): Promise<boolean> => {
    setIsLoading(true)
    setError(null)

    try {
      // TODO: 实现 Tauri command 调用
      // const result = await invoke<boolean>('validate_openai_key', { apiKey })
      // return result

      // Mock 实现（暂时）
      console.log('[useOpenAIProvider] Validating API key:', apiKey.substring(0, 8) + '...')
      await new Promise(resolve => setTimeout(resolve, 500))
      return true
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'API Key 验证失败'
      console.error('[useOpenAIProvider] Validation failed:', errorMessage)
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
      console.log('[useOpenAIProvider] Sending chat request:', request)

      // TODO: 实现 Tauri command 调用
      // const response = await invoke<ChatResponse>('openai_chat', { request })
      // return response

      // Mock 实现（暂时）
      await new Promise(resolve => setTimeout(resolve, 1000))
      return {
        content: '这是一个模拟的 OpenAI 响应。真实功能将在连接 Tauri 后端后实现。',
        model: request.model,
        usage: {
          promptTokens: 10,
          completionTokens: 20,
          totalTokens: 30,
        },
      }
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : '聊天请求失败'
      console.error('[useOpenAIProvider] Chat failed:', errorMessage)
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
        console.log('[useOpenAIProvider] Sending stream chat request:', request)

        // TODO: 实现 Tauri command 调用和流式处理
        // const fullContent = await invoke<string>('openai_stream_chat', {
        //   request,
        //   onChunk
        // })
        // return fullContent

        // Mock 实现（暂时）
        const mockResponse = '这是一个流式响应的模拟内容。每个字都会逐步显示。'
        const chunks = mockResponse.split('')

        for (const chunk of chunks) {
          onChunk(chunk)
          await new Promise(resolve => setTimeout(resolve, 50))
        }

        return mockResponse
      } catch (err) {
        const errorMessage = err instanceof Error ? err.message : '流式聊天失败'
        console.error('[useOpenAIProvider] Stream chat failed:', errorMessage)
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
