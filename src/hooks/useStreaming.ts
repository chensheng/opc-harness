/**
 * 流式输出 Hook
 *
 * 用于处理 AI 响应的实时流式展示
 */
import { useState, useCallback, useRef, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { listen, UnlistenFn } from '@tauri-apps/api/event'

export interface UseStreamingReturn {
  /** 是否正在流式传输 */
  isStreaming: boolean
  /** 当前累积的内容 */
  content: string
  /** 进度 (0-100) */
  progress: number
  /** 错误信息 */
  error: string | null
  /** 开始流式传输 */
  startStream: (idea: string, provider?: string, apiKey?: string) => Promise<void>
  /** 停止流式传输 */
  stopStream: () => void
  /** 重置状态 */
  reset: () => void
}

/**
 * 流式输出 Hook
 */
export function useStreaming(): UseStreamingReturn {
  const [isStreaming, setIsStreaming] = useState(false)
  const [content, setContent] = useState('')
  const [progress, setProgress] = useState(0)
  const [error, setError] = useState<string | null>(null)

  const unlistenRef = useRef<UnlistenFn | null>(null)
  const contentLengthRef = useRef(0)

  // 清理事件监听器
  useEffect(() => {
    return () => {
      if (unlistenRef.current) {
        unlistenRef.current()
      }
    }
  }, [])

  /**
   * 开始流式传输
   */
  const startStream = useCallback(async (idea: string, provider = 'openai', apiKey = '') => {
    // 重置状态
    setIsStreaming(true)
    setContent('')
    setProgress(0)
    setError(null)
    contentLengthRef.current = 0

    try {
      // 监听 PRD 流式 chunk 事件
      const unlistenChunk = await listen<{ session_id: string; content: string }>(
        'prd-stream-chunk',
        event => {
          const chunkContent = event.payload.content

          // 更新内容
          setContent(prev => {
            const newContent = prev + chunkContent
            contentLengthRef.current = newContent.length
            return newContent
          })

          // 更新进度（基于内容长度估算）
          setProgress(prev => Math.min(prev + chunkContent.length / 10, 95))
        }
      )

      // 监听完成事件
      const unlistenComplete = await listen<{ session_id: string; content: string }>(
        'prd-stream-complete',
        event => {
          const finalContent = event.payload.content
          setContent(finalContent)
          setProgress(100)
          setIsStreaming(false)

          // 清理监听器
          if (unlistenRef.current) {
            unlistenRef.current()
          }
        }
      )

      // 监听错误事件
      const unlistenError = await listen<{ session_id: string; error: string }>(
        'prd-stream-error',
        event => {
          const errorMessage = event.payload.error
          setError(errorMessage)
          setIsStreaming(false)
          setProgress(0)

          // 清理监听器
          if (unlistenRef.current) {
            unlistenRef.current()
          }
        }
      )

      // 保存清理函数
      unlistenRef.current = () => {
        unlistenChunk()
        unlistenComplete()
        unlistenError()
      }

      // 调用 Rust 后端的流式命令
      await invoke<string>('stream_generate_prd', {
        request: {
          idea,
          provider,
          model: provider === 'openai' ? 'gpt-3.5-turbo' : 'gpt-3.5-turbo',
          api_key: apiKey,
        },
      })
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : '流式生成失败'
      setError(errorMessage)
      setIsStreaming(false)
      setProgress(0)

      // 清理监听器
      if (unlistenRef.current) {
        unlistenRef.current()
      }
    }
  }, [])

  /**
   * 停止流式传输
   */
  const stopStream = useCallback(() => {
    // 清理事件监听器
    if (unlistenRef.current) {
      unlistenRef.current()
      unlistenRef.current = null
    }
    setIsStreaming(false)
    setProgress(0)
  }, [])

  /**
   * 重置状态
   */
  const reset = useCallback(() => {
    stopStream()
    setContent('')
    setProgress(0)
    setError(null)
    contentLengthRef.current = 0
  }, [stopStream])

  return {
    isStreaming,
    content,
    progress,
    error,
    startStream,
    stopStream,
    reset,
  }
}
