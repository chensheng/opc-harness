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
 * 解析 Markdown 格式的 PRD 内容为结构化数据
 */
function parsePRDFromMarkdown(markdown: string): Partial<PRD> {
  const result: Partial<PRD> = {}

  // 提取标题
  const titleMatch = markdown.match(/^#\s+(.+)$/m)
  if (titleMatch) {
    result.title = titleMatch[1].trim()
  }

  // 提取产品概述
  const overviewMatch = markdown.match(/##\s+产品概述\s*\n([\s\S]*?)(?=##|$)/)
  if (overviewMatch) {
    result.overview = overviewMatch[1].trim().replace(/\n/g, ' ')
  }

  // 提取目标用户
  const targetUsersMatch = markdown.match(/##\s+目标用户\s*\n([\s\S]*?)(?=##|$)/)
  if (targetUsersMatch) {
    const usersText = targetUsersMatch[1]
    const users = usersText
      .split('\n')
      .filter(line => line.trim().startsWith('- ') || line.trim().startsWith('* '))
      .map(line => line.replace(/^[-*]\s+/, '').trim())
    if (users.length > 0) {
      result.targetUsers = users
    }
  }

  // 提取核心功能
  const featuresMatch = markdown.match(/##\s+核心功能\s*\n([\s\S]*?)(?=##|$)/)
  if (featuresMatch) {
    const featuresText = featuresMatch[1]
    const features = featuresText
      .split('\n')
      .filter(line => line.trim().startsWith('- ') || line.trim().startsWith('* '))
      .map(line => line.replace(/^[-*]\s+/, '').trim())
    if (features.length > 0) {
      result.coreFeatures = features
    }
  }

  // 提取技术栈
  const techStackMatch = markdown.match(/##\s+技术栈\s*\n([\s\S]*?)(?=##|$)/)
  if (techStackMatch) {
    const techText = techStackMatch[1]
    const techs = techText
      .split('\n')
      .filter(line => line.trim().startsWith('- ') || line.trim().startsWith('* '))
      .map(line => line.replace(/^[-*]\s+/, '').trim())
    if (techs.length > 0) {
      result.techStack = techs
    }
  }

  // 提取预估工作量
  const effortMatch = markdown.match(/##\s+(?:预估工作量 | 时间估算)\s*\n([\s\S]*?)(?=##|$)/)
  if (effortMatch) {
    result.estimatedEffort = effortMatch[1].trim()
  }

  // 提取商业模式
  const businessMatch = markdown.match(/##\s+商业模式\s*\n([\s\S]*?)(?=##|$)/)
  if (businessMatch) {
    result.businessModel = businessMatch[1].trim()
  }

  // 提取定价策略
  const pricingMatch = markdown.match(/##\s+定价策略\s*\n([\s\S]*?)(?=##|$)/)
  if (pricingMatch) {
    result.pricing = pricingMatch[1].trim()
  }

  return result
}

/**
 * PRD 流式生成 Hook（打字机效果）
 */
export function usePRDStream(): UsePRDStreamReturn {
  const [prd, setPrd] = useState<PRD | null>(null)
  const [markdownContent, setMarkdownContent] = useState('')
  const [isStreaming, setIsStreaming] = useState(false)
  const [isComplete, setIsComplete] = useState(false)
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
        console.error('[usePRDStream] Cleanup error:', err)
      }
    })
    unlistenRef.current = []
  }, [])

  // 停止流式
  const stopStream = useCallback(() => {
    cleanup()
    isStreamingRef.current = false
    setIsStreaming(false)
  }, [cleanup])

  // 重置状态
  const reset = useCallback(() => {
    stopStream()
    setPrd(null)
    setMarkdownContent('')
    setIsComplete(false)
    setError(null)
    setSessionId(null)
  }, [stopStream])

  // 开始流式生成 PRD
  const startStream = useCallback(
    async (request: PRDStreamRequest) => {
      // 清理之前的订阅
      cleanup()

      // 重置状态
      setMarkdownContent('')
      setPrd(null)
      setIsComplete(false)
      setError(null)
      setIsStreaming(true)
      isStreamingRef.current = true

      try {
        // 监听 PRD 流式 chunk 事件
        const unlistenChunk = await listen<{
          session_id: string
          content: string
          is_complete: boolean
        }>('prd-stream-chunk', event => {
          console.log('[usePRDStream] Received PRD chunk:', event.payload.content.length, 'chars')

          // 追加内容
          setMarkdownContent(prev => {
            const newContent = prev + event.payload.content

            // 实时解析 Markdown 为结构化 PRD
            const parsed = parsePRDFromMarkdown(newContent)
            setPrd(prevPRD => ({
              title: parsed.title || prevPRD?.title || '生成中...',
              overview: parsed.overview || prevPRD?.overview || '',
              targetUsers: parsed.targetUsers || prevPRD?.targetUsers || [],
              coreFeatures: parsed.coreFeatures || prevPRD?.coreFeatures || [],
              techStack: parsed.techStack || prevPRD?.techStack || [],
              estimatedEffort: parsed.estimatedEffort || prevPRD?.estimatedEffort || '',
              businessModel: parsed.businessModel || prevPRD?.businessModel,
              pricing: parsed.pricing || prevPRD?.pricing,
            }))

            return newContent
          })
        })
        unlistenRef.current.push(unlistenChunk)

        // 监听完成事件
        const unlistenComplete = await listen<{ session_id: string; content: string }>(
          'prd-stream-complete',
          event => {
            console.log('[usePRDStream] PRD stream complete:', event.payload.session_id)
            setIsComplete(true)
            setIsStreaming(false)
            isStreamingRef.current = false
            cleanup()

            // 确保最终内容完整
            setMarkdownContent(event.payload.content)
            const parsed = parsePRDFromMarkdown(event.payload.content)
            setPrd(prevPRD => ({
              title: parsed.title || prevPRD?.title || 'PRD',
              overview: parsed.overview || prevPRD?.overview || '',
              targetUsers: parsed.targetUsers || prevPRD?.targetUsers || [],
              coreFeatures: parsed.coreFeatures || prevPRD?.coreFeatures || [],
              techStack: parsed.techStack || prevPRD?.techStack || [],
              estimatedEffort: parsed.estimatedEffort || prevPRD?.estimatedEffort || '',
              businessModel: parsed.businessModel || prevPRD?.businessModel,
              pricing: parsed.pricing || prevPRD?.pricing,
            }))
          }
        )
        unlistenRef.current.push(unlistenComplete)

        // 监听错误事件
        const unlistenError = await listen<{ session_id: string; error: string }>(
          'prd-stream-error',
          event => {
            console.error('[usePRDStream] PRD stream error:', event.payload.error)
            setError(event.payload.error)
            setIsStreaming(false)
            isStreamingRef.current = false
            cleanup()
          }
        )
        unlistenRef.current.push(unlistenError)

        // 调用后端流式命令
        const response = await invoke<string>('stream_generate_prd', {
          request: {
            idea: request.idea,
            provider: request.provider,
            model: request.model,
            api_key: request.apiKey,
          },
        })

        setSessionId(response)
      } catch (err) {
        console.error('[usePRDStream] Error starting stream:', err)
        setError(err instanceof Error ? err.message : '未知错误')
        setIsStreaming(false)
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
