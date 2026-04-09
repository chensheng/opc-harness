import { useState, useCallback, useRef } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import type { UserStory } from '@/types'

export interface UserStoryStreamRequest {
  prdContent: string
  provider: string
  model: string
  apiKey: string
}

export interface UseUserStoryStreamReturn {
  markdownContent: string
  userStories: UserStory[]
  isStreaming: boolean
  isComplete: boolean
  error: string | null
  sessionId: string | null
  startStream: (request: UserStoryStreamRequest) => Promise<void>
  stopStream: () => void
  reset: () => void
}

/**
 * 用户故事拆分流式 Hook
 */
export function useUserStoryStream(): UseUserStoryStreamReturn {
  const [markdownContent, setMarkdownContent] = useState('')
  const [userStories, setUserStories] = useState<UserStory[]>([])
  const [isStreaming, setIsStreaming] = useState(false)
  const [isComplete, setIsComplete] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [sessionId, setSessionId] = useState<string | null>(null)

  const unlistenRef = useRef<UnlistenFn[]>([])

  const cleanup = useCallback(() => {
    unlistenRef.current.forEach(unlisten => unlisten())
    unlistenRef.current = []
  }, [])

  const startStream = useCallback(
    async (request: UserStoryStreamRequest) => {
      // 清理之前的事件监听
      cleanup()

      // 重置状态
      setMarkdownContent('')
      setUserStories([])
      setIsStreaming(true)
      setIsComplete(false)
      setError(null)
      setSessionId(null)

      try {
        // 监听流式 chunk 事件
        const unlistenChunk = await listen<{
          session_id: string
          content: string
          is_complete: boolean
        }>('user-story-stream-chunk', event => {
          const { content } = event.payload

          // 累积内容
          setMarkdownContent(prev => prev + content)
        })
        unlistenRef.current.push(unlistenChunk)

        // 监听完成事件
        const unlistenComplete = await listen<{ session_id: string; content: string }>(
          'user-story-stream-complete',
          async event => {
            const { content } = event.payload

            setIsComplete(true)
            setIsStreaming(false)
            setMarkdownContent(content)

            // TODO: 解析 Markdown 表格为用户故事数组
            // 这里暂时设置空数组,后续需要实现解析逻辑
            setUserStories([])

            cleanup()
          }
        )
        unlistenRef.current.push(unlistenComplete)

        // 监听错误事件
        const unlistenError = await listen<{ session_id: string; error: string }>(
          'user-story-stream-error',
          event => {
            const { error: errorMsg } = event.payload

            setError(errorMsg)
            setIsStreaming(false)
            setIsComplete(false)

            cleanup()
          }
        )
        unlistenRef.current.push(unlistenError)

        // 调用后端流式 API
        const response = await invoke<string>('decompose_user_stories_streaming', {
          request: {
            prdContent: request.prdContent,
            provider: request.provider,
            model: request.model,
            apiKey: request.apiKey,
          },
        })

        // 响应已在事件中处理
        console.log('Stream completed with response:', response)
      } catch (err) {
        const errorMessage = err instanceof Error ? err.message : '流式请求失败'
        setError(errorMessage)
        setIsStreaming(false)
        setIsComplete(false)
        cleanup()
      }
    },
    [cleanup]
  )

  const stopStream = useCallback(() => {
    setIsStreaming(false)
    cleanup()
  }, [cleanup])

  const reset = useCallback(() => {
    cleanup()
    setMarkdownContent('')
    setUserStories([])
    setIsStreaming(false)
    setIsComplete(false)
    setError(null)
    setSessionId(null)
  }, [cleanup])

  return {
    markdownContent,
    userStories,
    isStreaming,
    isComplete,
    error,
    sessionId,
    startStream,
    stopStream,
    reset,
  }
}
