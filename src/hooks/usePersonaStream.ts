import { useState, useCallback, useRef, useEffect } from 'react'
import { listen, UnlistenFn } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/core'
import type { UserPersona } from '@/types'

/**
 * 用户画像流式生成请求参数
 */
export interface PersonaStreamRequest {
  /** 项目创意或描述 */
  idea: string
  /** AI 提供商 */
  provider: string
  /** AI 模型 */
  model: string
  /** API 密钥 */
  apiKey: string
}

/**
 * 用户画像流式生成返回值
 */
export interface UsePersonaStreamReturn {
  /** 已生成的用户画像列表 */
  personas: UserPersona[]
  /** Markdown 原始内容 */
  markdownContent: string
  /** 是否正在流式生成中 */
  isStreaming: boolean
  /** 流式生成是否完成 */
  isComplete: boolean
  /** 错误信息 */
  error: string | null
  /** 会话 ID */
  sessionId: string | null
  /** 开始流式生成 */
  startStream: (request: PersonaStreamRequest) => Promise<void>
  /** 停止流式生成 */
  stopStream: () => void
  /** 重置状态 */
  reset: () => void
}

/**
 * 从 Markdown 解析用户画像
 * @param markdown Markdown 格式的文本
 * @returns 解析后的用户画像数组
 */
function parsePersonasFromMarkdown(markdown: string): UserPersona[] {
  const personas: UserPersona[] = []

  // 按画像分隔符分割（假设每个画像以 "##" 开头）
  const sections = markdown.split(/(?=##\s*(?:用户？画像？|Persona))/i)

  for (const section of sections) {
    if (!section.trim()) continue

    // 提取基本信息
    const nameMatch = section.match(/(?:姓名|Name)[:：]\s*(.+)/i)
    const ageMatch = section.match(/(?:年龄|Age)[:：]\s*(.+)/i)
    const occupationMatch = section.match(/(?:职业|Occupation)[:：]\s*(.+)/i)
    const backgroundMatch = section.match(/(?:背景|Background)[:：]\s*([\s\S]*?)(?=\n\n|\n#|$)/i)

    // 提取列表项（目标、痛点、行为）
    const extractList = (pattern: RegExp): string[] => {
      const match = section.match(pattern)
      if (!match) return []

      const content = match[1]
      const items = content
        .split('\n')
        .filter(line => line.trim().startsWith('-') || line.trim().startsWith('•'))
        .map(line => line.replace(/^[-•]\s*/, '').trim())
        .filter(item => item.length > 0)

      return items
    }

    const goals = extractList(/(?:目标|Goals)[:：]\s*\n([\s\S]*?)(?=\n\n[A-Z]|\n#|$)/i)
    const painPoints = extractList(/(?:痛点|Pain\s*Points)[:：]\s*\n([\s\S]*?)(?=\n\n[A-Z]|\n#|$)/i)
    const behaviors = extractList(/(?:行为|Behaviors)[:：]\s*\n([\s\S]*?)(?=\n\n[A-Z]|\n#|$)/i)

    // 提取引用
    const quoteMatch = section.match(/(?:"([^"]+)"|'([^']+)'|「([^」]+)」)/)

    // 只有找到姓名才创建画像
    if (nameMatch) {
      personas.push({
        id: `persona-${personas.length + 1}`,
        name: nameMatch[1].trim(),
        age: ageMatch?.[1].trim() || '',
        occupation: occupationMatch?.[1].trim() || '',
        background: backgroundMatch?.[1].trim() || '',
        goals,
        painPoints,
        behaviors,
        quote: quoteMatch?.[1] || quoteMatch?.[2] || quoteMatch?.[3],
      })
    }
  }

  return personas
}

/**
 * 用户画像流式生成 Hook
 *
 * 支持渐进式渲染的 AI 用户画像生成
 *
 * @example
 * ```typescript
 * const {
 *   personas,
 *   markdownContent,
 *   isStreaming,
 *   isComplete,
 *   error,
 *   startStream,
 *   stopStream,
 *   reset
 * } = usePersonaStream()
 *
 * await startStream({
 *   idea: '一个在线学习平台',
 *   provider: 'openai',
 *   model: 'gpt-4',
 *   apiKey: 'sk-...'
 * })
 * ```
 */
export function usePersonaStream(): UsePersonaStreamReturn {
  const [personas, setPersonas] = useState<UserPersona[]>([])
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
        console.error('[usePersonaStream] Cleanup error:', err)
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
    setPersonas([])
    setMarkdownContent('')
    setIsComplete(false)
    setError(null)
    setSessionId(null)
  }, [stopStream])

  // 开始流式生成用户画像
  const startStream = useCallback(
    async (request: PersonaStreamRequest) => {
      // 清理之前的订阅
      cleanup()

      // 重置状态
      setMarkdownContent('')
      setPersonas([])
      setIsComplete(false)
      setError(null)
      setIsStreaming(true)
      isStreamingRef.current = true

      try {
        // 监听用户画像流式 chunk 事件
        const unlistenChunk = await listen<{
          session_id: string
          content: string
          is_complete: boolean
        }>('persona-stream-chunk', event => {
          console.log(
            '[usePersonaStream] Received persona chunk:',
            event.payload.content.length,
            'chars'
          )

          // 追加内容
          setMarkdownContent(prev => {
            const newContent = prev + event.payload.content

            // 实时解析 Markdown 为结构化用户画像
            const parsed = parsePersonasFromMarkdown(newContent)
            setPersonas(parsed)

            return newContent
          })
        })
        unlistenRef.current.push(unlistenChunk)

        // 监听完成事件
        const unlistenComplete = await listen<{ session_id: string; content: string }>(
          'persona-stream-complete',
          event => {
            console.log('[usePersonaStream] Persona stream complete:', event.payload.session_id)
            setIsComplete(true)
            setIsStreaming(false)
            isStreamingRef.current = false
            cleanup()

            // 确保最终内容完整
            setMarkdownContent(event.payload.content)
            const parsed = parsePersonasFromMarkdown(event.payload.content)
            setPersonas(parsed)
          }
        )
        unlistenRef.current.push(unlistenComplete)

        // 监听错误事件
        const unlistenError = await listen<{ session_id: string; error: string }>(
          'persona-stream-error',
          event => {
            console.error('[usePersonaStream] Persona stream error:', event.payload.error)
            setError(event.payload.error)
            setIsStreaming(false)
            isStreamingRef.current = false
            cleanup()
          }
        )
        unlistenRef.current.push(unlistenError)

        // 调用后端流式命令
        const response = await invoke<string>('stream_generate_personas', {
          request: {
            idea: request.idea,
            provider: request.provider,
            model: request.model,
            api_key: request.apiKey,
          },
        })

        setSessionId(response)
      } catch (err) {
        console.error('[usePersonaStream] Error starting stream:', err)
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
    personas,
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
