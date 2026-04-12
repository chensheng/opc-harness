import { useState, useCallback, useRef, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import type { PRD } from '@/types'
import { parseMarkdownToPRD } from '@/lib/prd-parser'

export interface PRDStreamRequest {
  idea: string
  provider: string
  model: string
  apiKey: string
  projectId?: string | null
}

export interface UsePRDStreamReturn {
  prd: PRD | null
  markdownContent: string
  isStreaming: boolean
  isComplete: boolean
  error: string | null
  sessionId: string | null
  startStream: (request: PRDStreamRequest) => Promise<void>
  stopStream: () => void
  reset: () => void
}

/**
 * PRD 流式生成 Hook（打字机效果）
 */
export function usePRDStream(): UsePRDStreamReturn {
  const [prd, setPrd] = useState<PRD | null>(null)
  const [isStreaming, setIsStreaming] = useState(false)
  const [isComplete, setIsComplete] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [sessionId, setSessionId] = useState<string | null>(null)
  const [_accumulatedContent, _setAccumulatedContent] = useState('')
  const [markdownContent, setMarkdownContent] = useState('')

  const isStreamingRef = useRef(false)
  const accumulatedContentRef = useRef('')
  const unlistenRef = useRef<UnlistenFn[]>([])

  const cleanup = useCallback(() => {
    unlistenRef.current.forEach(unlisten => unlisten())
    unlistenRef.current = []
  }, [])

  // 更新 PRD 状态（只保存 markdown 内容）
  const updatePRDState = useCallback((content: string, isComplete = false) => {
    setMarkdownContent(content)

    // 如果内容完整，尝试解析为 PRD 对象
    if (isComplete && content) {
      try {
        const parsedPRD = parseMarkdownToPRD(content)
        // 重要：添加完整的 Markdown 内容到 PRD 对象
        parsedPRD.markdownContent = content
        setPrd(parsedPRD)
      } catch (error) {
        console.error('[usePRDStream] Error parsing markdown to PRD:', error)
        // 如果解析失败，仍然设置基本的 prd 对象
        setPrd(prevPRD => ({
          title: prevPRD?.title || '生成中...',
          overview: prevPRD?.overview || '',
          targetUsers: prevPRD?.targetUsers || [],
          coreFeatures: prevPRD?.coreFeatures || [],
          techStack: prevPRD?.techStack || [],
          estimatedEffort: prevPRD?.estimatedEffort || '',
          businessModel: prevPRD?.businessModel,
          pricing: prevPRD?.pricing,
          markdownContent: content,
        }))
      }
    } else {
      // 流式过程中，只更新 markdown 内容
      setPrd(prevPRD => ({
        title: prevPRD?.title || '生成中...',
        overview: prevPRD?.overview || '',
        targetUsers: prevPRD?.targetUsers || [],
        coreFeatures: prevPRD?.coreFeatures || [],
        techStack: prevPRD?.techStack || [],
        estimatedEffort: prevPRD?.estimatedEffort || '',
        businessModel: prevPRD?.businessModel,
        pricing: prevPRD?.pricing,
        markdownContent: content,
      }))
    }
  }, [])

  // 开始流式生成 PRD
  const startStream = useCallback(
    async (request: PRDStreamRequest) => {
      cleanup()

      accumulatedContentRef.current = ''
      setMarkdownContent('')
      setPrd(null)
      setIsComplete(false)
      setError(null)
      setIsStreaming(true)
      isStreamingRef.current = true

      try {
        const unlistenChunk = await listen<{
          session_id: string
          content: string
          is_complete: boolean
        }>('prd-stream-chunk', event => {
          accumulatedContentRef.current += event.payload.content

          updatePRDState(accumulatedContentRef.current)
        })
        unlistenRef.current.push(unlistenChunk)

        const unlistenComplete = await listen<{ session_id: string; content: string }>(
          'prd-stream-complete',
          event => {
            console.log('[usePRDStream] ========================================')
            console.log('[usePRDStream] PRD stream complete event received')
            console.log('[usePRDStream] Session ID:', event.payload.session_id)
            console.log('[usePRDStream] Content length:', event.payload.content.length)
            console.log('[usePRDStream] Content preview (first 200 chars):', event.payload.content.substring(0, 200))
            console.log('[usePRDStream] ========================================')
            
            setIsComplete(true)
            setIsStreaming(false)
            isStreamingRef.current = false

            accumulatedContentRef.current = event.payload.content
            updatePRDState(event.payload.content, true)

            cleanup()
          }
        )
        unlistenRef.current.push(unlistenComplete)

        // 监听错误事件 - 直接显示后端返回的完整错误信息
        const unlistenError = await listen<{ session_id: string; error: string }>(
          'prd-stream-error',
          event => {
            console.error('[usePRDStream] ========================================')
            console.error('[usePRDStream] PRD stream error event received:')
            console.error('[usePRDStream] Session ID:', event.payload.session_id)
            console.error('[usePRDStream] Error type:', typeof event.payload.error)
            console.error('[usePRDStream] Error length:', event.payload.error?.length || 0)
            console.error('[usePRDStream] Full error content:')
            console.error(event.payload.error)
            console.error('[usePRDStream] ========================================')

            setError(event.payload.error)
            setIsStreaming(false)
            isStreamingRef.current = false
            cleanup()
          }
        )
        unlistenRef.current.push(unlistenError)

        // 等待一小段时间，确保所有监听器都已完全注册
        // 这样可以避免错过在 invoke 返回后立即发送的错误事件
        await new Promise(resolve => setTimeout(resolve, 50))

        console.log('[usePRDStream] Calling start_prd_stream with:', {
          idea: request.idea.slice(0, 50) + '...',
          provider: request.provider,
          model: request.model,
          projectId: request.projectId,
          project_id_for_backend: request.projectId || null,
        })

        const invokeParams = {
          request: {
            idea: request.idea,
            provider: request.provider,
            model: request.model,
            api_key: request.apiKey, // 使用snake_case与后端结构体匹配
            project_id: request.projectId || null,
          },
        }

        console.log('[usePRDStream] ====== 准备调用 start_prd_stream ======')
        console.log('[usePRDStream] request.projectId:', request.projectId)
        console.log(
          '[usePRDStream] invokeParams.request.project_id:',
          invokeParams.request.project_id
        )
        console.log('[usePRDStream] Full invoke params JSON:', JSON.stringify(invokeParams))
        console.log('[usePRDStream] ==========================================')

        const { session_id } = await invoke<{ session_id: string }>(
          'start_prd_stream',
          invokeParams
        )

        setSessionId(session_id)
      } catch (err) {
        console.error('[usePRDStream] Error starting PRD stream:', err)
        setError(err instanceof Error ? err.message : String(err))
        setIsStreaming(false)
        isStreamingRef.current = false
        cleanup()
      }
    },
    [cleanup, updatePRDState]
  )

  // 停止流式生成 PRD
  const stopStream = useCallback(() => {
    if (isStreamingRef.current && sessionId) {
      invoke('stop_prd_stream', { session_id: sessionId })
        .catch((err: unknown) => {
          console.error('[usePRDStream] Error stopping PRD stream:', err)
        })
        .finally(() => {
          setIsStreaming(false)
          isStreamingRef.current = false
          cleanup()
        })
    } else {
      // 即使没有 sessionId，也要清理状态
      setIsStreaming(false)
      isStreamingRef.current = false
      cleanup()
    }
  }, [sessionId, cleanup])

  // 重置状态
  const reset = useCallback(() => {
    cleanup()
    accumulatedContentRef.current = ''
    setPrd(null)
    setIsComplete(false)
    setError(null)
    setIsStreaming(false)
    isStreamingRef.current = false
  }, [cleanup])

  useEffect(() => {
    return () => {
      cleanup()
    }
  }, [cleanup])

  return {
    prd,
    markdownContent,
    isStreaming,
    isComplete,
    error,
    sessionId,
    startStream,
    stopStream,
    reset,
  }
}
