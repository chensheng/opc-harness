import { useState, useCallback, useRef, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import type { PRD } from '@/types'

export interface PRDStreamRequest {
  idea: string
  provider: string
  model: string
  apiKey: string
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
  const [accumulatedContent, setAccumulatedContent] = useState('')
  const [markdownContent, setMarkdownContent] = useState('')

  const isStreamingRef = useRef(false)
  const accumulatedContentRef = useRef('')
  const unlistenRef = useRef<UnlistenFn[]>([])

  const cleanup = useCallback(() => {
    unlistenRef.current.forEach(unlisten => unlisten())
    unlistenRef.current = []
  }, [])

  // 更新 PRD 状态（只保存 markdown 内容）
  const updatePRDState = useCallback((content: string) => {
    setMarkdownContent(content)
    
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
          console.log('[usePRDStream] Received PRD chunk:', event.payload.content.length, 'chars')

          accumulatedContentRef.current += event.payload.content
          
          updatePRDState(accumulatedContentRef.current)
        })
        unlistenRef.current.push(unlistenChunk)

        const unlistenComplete = await listen<{ session_id: string; content: string }>(
          'prd-stream-complete',
          event => {
            console.log('[usePRDStream] PRD stream complete:', event.payload.session_id)
            setIsComplete(true)
            setIsStreaming(false)
            isStreamingRef.current = false
            
            accumulatedContentRef.current = event.payload.content
            updatePRDState(event.payload.content)
            
            cleanup()
          }
        )
        unlistenRef.current.push(unlistenComplete)

        const { session_id } = await invoke<{ session_id: string }>('start_prd_stream', {
          idea: request.idea,
          provider: request.provider,
          model: request.model,
          api_key: request.apiKey,
        })

        setSessionId(session_id)
      } catch (err) {
        console.error('[usePRDStream] Error starting PRD stream:', err)
        setError(err instanceof Error ? err.message : String(err))
        setIsStreaming(false)
        isStreamingRef.current = false
      }
    },
    [cleanup, updatePRDState]
  )

  // 停止流式生成 PRD
  const stopStream = useCallback(() => {
    if (isStreamingRef.current) {
      invoke('stop_prd_stream', { session_id: sessionId }).catch(err => {
        console.error('[usePRDStream] Error stopping PRD stream:', err)
      })
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
