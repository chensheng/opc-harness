import { useState } from 'react'
import type { AIResponse } from '../types'
import { extractErrorMessage } from '../utils'

interface UseAITestingProps {
  getConfig: (providerId: string) => { model: string; apiKey: string } | undefined
}

/**
 * AI 测试 Hook（流式和非流式）
 */
export function useAITesting({ getConfig }: UseAITestingProps) {
  const [testProvider, setTestProvider] = useState<string | null>(null)
  const [testMessage, setTestMessage] = useState('你好，请介绍一下你自己')
  const [nonStreamResponse, setNonStreamResponse] = useState<Record<string, string>>({})
  const [nonStreamLoading, setNonStreamLoading] = useState(false)
  const [nonStreamError, setNonStreamError] = useState<string | null>(null)

  const handleTestNonStream = async (providerId: string) => {
    setNonStreamLoading(true)
    setNonStreamError(null)
    setNonStreamResponse(prev => ({ ...prev, [providerId]: '' }))

    const config = getConfig(providerId)
    if (!config) return

    try {
      const core = await import('@tauri-apps/api/core')

      // 使用通用的 chat 命令，支持所有 provider（包括 codefree）
      const response = await core.invoke<any>('chat', {
        request: {
          provider: providerId,
          model: config.model,
          api_key: config.apiKey,
          messages: [{ role: 'user', content: testMessage }],
          temperature: 0.7,
          max_tokens: 2048,
        },
      })

      setNonStreamResponse(prev => ({
        ...prev,
        [providerId]: response.content || JSON.stringify(response),
      }))
    } catch (err) {
      console.error('[handleTestNonStream] 非流式测试失败:', err)
      const errorMessage = extractErrorMessage(err)
      setNonStreamError(errorMessage)
    } finally {
      setNonStreamLoading(false)
    }
  }

  const clearNonStreamResponse = (providerId: string) => {
    setNonStreamResponse(prev => ({ ...prev, [providerId]: '' }))
  }

  return {
    testProvider,
    setTestProvider,
    testMessage,
    setTestMessage,
    nonStreamResponse,
    nonStreamLoading,
    nonStreamError,
    setNonStreamError,
    handleTestNonStream,
    clearNonStreamResponse,
  }
}
