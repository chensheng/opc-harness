import { useState, useEffect, useCallback, useRef } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { listen, UnlistenFn } from '@tauri-apps/api/event'
import type { StreamChunk, StreamComplete, StreamError, Message } from '@/types'

export interface StreamChatRequest {
  provider: string
  model: string
  apiKey: string
  messages: Message[]
  temperature?: number
  maxTokens?: number
}

export interface UseAIStreamReturn {
  content: string
  isComplete: boolean
  isLoading: boolean
  error: string | null
  sessionId: string | null
  startStream: (request: StreamChatRequest) => Promise<void>
  stopStream: () => void
  reset: () => void
}

export function useAIStream(): UseAIStreamReturn {
  const [content, setContent] = useState('')
  const [isComplete, setIsComplete] = useState(false)
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [sessionId, setSessionId] = useState<string | null>(null)

  const unlistenRef = useRef<UnlistenFn[]>([])
  const isStreamingRef = useRef(false)

  // 清理所有订阅
  const cleanup = useCallback(() => {
    unlistenRef.current.forEach(unlisten => {
      try {
        unlisten()
      } catch (err) {
        console.error('[useAIStream] Cleanup error:', err)
      }
    })
    unlistenRef.current = []
  }, [])

  // 停止流式
  const stopStream = useCallback(() => {
    cleanup()
    isStreamingRef.current = false
    setIsLoading(false)
  }, [cleanup])

  // 重置状态
  const reset = useCallback(() => {
    stopStream()
    setContent('')
    setIsComplete(false)
    setError(null)
    setSessionId(null)
  }, [stopStream])

  // 开始流式
  const startStream = useCallback(
    async (request: StreamChatRequest) => {
      // 清理之前的订阅
      cleanup()

      // 重置状态
      setContent('')
      setIsComplete(false)
      setError(null)
      setIsLoading(true)
      isStreamingRef.current = true

      try {
        // 监听流式数据块事件
        const unlistenChunk = await listen<StreamChunk>('ai-stream-chunk', event => {
          console.log('[useAIStream] Received chunk:', event.payload.content.length, 'chars')
          setContent(prev => prev + event.payload.content)
        })
        unlistenRef.current.push(unlistenChunk)

        // 监听完成事件
        const unlistenComplete = await listen<StreamComplete>('ai-stream-complete', event => {
          console.log('[useAIStream] Stream complete:', event.payload.session_id)
          setIsComplete(true)
          setIsLoading(false)
          isStreamingRef.current = false
          cleanup()
        })
        unlistenRef.current.push(unlistenComplete)

        // 监听错误事件
        const unlistenError = await listen<StreamError>('ai-stream-error', event => {
          console.error('[useAIStream] Stream error:', event.payload.error)
          setError(event.payload.error)
          setIsLoading(false)
          isStreamingRef.current = false
          cleanup()
        })
        unlistenRef.current.push(unlistenError)

        // 调用后端流式命令
        const response = await invoke<string>('stream_chat', {
          request: {
            provider: request.provider,
            model: request.model,
            api_key: request.apiKey,
            messages: request.messages,
            temperature: request.temperature || 0.7,
            max_tokens: request.maxTokens,
          },
        })

        setSessionId(response)
      } catch (err) {
        console.error('[useAIStream] Error starting stream:', err)
        setError(err instanceof Error ? err.message : '未知错误')
        setIsLoading(false)
        isStreamingRef.current = false
        cleanup()
      }
    },
    [cleanup]
  )

  // 组件卸载时清理
  useEffect(() => {
    return () => {
      cleanup()
    }
  }, [cleanup])

  return {
    content,
    isComplete,
    isLoading,
    error,
    sessionId,
    startStream,
    stopStream,
    reset,
  }
}
