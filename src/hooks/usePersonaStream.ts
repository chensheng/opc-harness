import { useState, useCallback, useRef, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import type { UserPersona } from '@/types'
import { useAIConfigStore } from '@/stores/aiConfigStore'

/**
 * 用户画像流式生成 Hook
 *
 * 功能特性:
 * - 调用后端 stream_generate_personas 接口
 * - 监听 persona-stream-chunk/complete/error 事件
 * - 渐进式渲染用户画像（先基础信息，后详细特征）
 * - 打字机效果逐字展示
 * - 自动解析 Markdown 为 UserPersona 对象
 */

interface PersonaStreamConfig {
  prdId: string
  projectId: string
}

interface UsePersonaStreamReturn {
  personas: UserPersona[]
  markdownContent: string
  isStreaming: boolean
  isComplete: boolean
  error: string | null
  sessionId: string | null
  startStream: (config: PersonaStreamConfig & { apiKey?: string; model?: string }) => Promise<void>
  stopStream: () => void
  reset: () => void
}

/**
 * 解析 Markdown 格式的用户画像为结构化数据
 *
 * Markdown 格式示例:
 * ````
 * ## 用户画像 1: Alex
 * - **年龄**: 28 岁
 * - **职业**: 全栈开发者
 * - **背景**: 有 5 年开发经验...
 * - **目标**:
 *   - 快速验证产品想法
 *   - 减少重复性工作
 * - **痛点**:
 *   - 时间有限
 *   - 不懂设计和营销
 * - **引言**: "我想把更多时间..."
 * ```
 */
function parsePersonasFromMarkdown(markdown: string): UserPersona[] {
  const personas: UserPersona[] = []

  // 按画像分割（每个 ## 标题开始一个新的画像）
  const personaBlocks = markdown.split(/(?=##\s+用户画像\s*\d*:)/)

  for (const block of personaBlocks) {
    if (!block.trim()) continue

    // 提取姓名
    const nameMatch = block.match(/##\s+用户画像\s*\d*:\s*(.+)/)
    const name = nameMatch ? nameMatch[1].trim() : `用户画像 ${personas.length + 1}`

    // 提取各个字段
    const extractField = (fieldName: string): string | null => {
      const regex = new RegExp(`-\\s*\\*\\*${fieldName}\\*\\*:\\s*(.+?)(?=\\n-|$)`, 's')
      const match = block.match(regex)
      return match ? match[1].trim().replace(/\n\s*/g, '') : null
    }

    // 提取列表字段（如目标、痛点等）
    const extractList = (fieldName: string): string[] => {
      const regex = new RegExp(
        `-\\s*\\*\\*${fieldName}\\*\\*:\\s*\\n([\\s\\S]*?)(?=\\n-\\s*\\*\\*|$)`,
        's'
      )
      const match = block.match(regex)
      if (!match) return []

      const listContent = match[1]
      const items = listContent
        .split('\n')
        .filter(line => line.trim().startsWith('-'))
        .map(line => line.replace(/^\s*-\s*/, '').trim())
      return items
    }

    const age = extractField('年龄')
    const occupation = extractField('职业')
    const background = extractField('背景')
    const goals = extractList('目标')
    const painPoints = extractList('痛点')
    const behaviors = extractList('行为特征')
    const quote = extractField('引言')

    // 只有当有基本信息时才添加
    if (name || occupation || background) {
      personas.push({
        id: `persona-${personas.length}`,
        name: name || `用户画像 ${personas.length + 1}`,
        age: age || '未指定',
        occupation: occupation || '未指定',
        background: background || '暂无背景信息',
        goals: goals.length > 0 ? goals : ['暂无明确目标'],
        painPoints: painPoints.length > 0 ? painPoints : ['暂无明确痛点'],
        behaviors: behaviors.length > 0 ? behaviors : ['暂无明确行为特征'],
        quote: quote || undefined,
      })
    }
  }

  return personas
}

export function usePersonaStream(): UsePersonaStreamReturn {
  const [personas, setPersonas] = useState<UserPersona[]>([])
  const [markdownContent, setMarkdownContent] = useState('')
  const [isStreaming, setIsStreaming] = useState(false)
  const [isComplete, setIsComplete] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [sessionId, setSessionId] = useState<string | null>(null)

  // 流式内容缓冲
  const [streamingContent, setStreamingContent] = useState('')
  const typingTimerRef = useRef<NodeJS.Timeout | null>(null)
  const unlistenFns = useRef<Function[]>([])

  // AI 配置
  const { getActiveConfig } = useAIConfigStore()

  // 打字机效果：逐字展示 streamingContent
  useEffect(() => {
    if (!streamingContent) return

    let charIndex = 0
    const contentToAdd = streamingContent
    const totalChars = contentToAdd.length

    if (totalChars === 0) return

    const interval = setInterval(() => {
      if (charIndex < totalChars) {
        setMarkdownContent(prev => prev + contentToAdd[charIndex])
        charIndex++
      } else {
        clearInterval(interval)

        // 解析当前完整的 Markdown 内容
        const parsedPersonas = parsePersonasFromMarkdown(markdownContent + streamingContent)
        if (parsedPersonas.length > 0) {
          setPersonas(parsedPersonas)
        }
      }
    }, 30) // 30ms/字符，比 PRD 更快

    return () => clearInterval(interval)
  }, [streamingContent])

  // 开始流式生成
  const startStream = useCallback(
    async (config: PersonaStreamConfig & { apiKey?: string; model?: string }) => {
      try {
        setError(null)
        setIsComplete(false)
        setPersonas([])
        setMarkdownContent('')
        setStreamingContent('')

        // 获取 AI 配置
        const activeConfig = getActiveConfig()
        const provider = activeConfig?.provider || 'openai'
        const apiKey = config.apiKey || activeConfig?.apiKey || ''
        const model = config.model || activeConfig?.model || 'gpt-4o'

        if (!apiKey) {
          throw new Error('请先配置 API Key')
        }

        // 调用后端流式生成接口
        const session_id = await invoke<string>('stream_generate_personas', {
          prdId: config.prdId,
          projectId: config.projectId,
          provider,
          apiKey,
          model,
        })

        setSessionId(session_id)
        setIsStreaming(true)

        // 监听 chunk 事件
        const unlistenChunk = await listen<{
          session_id: string
          content: string
          is_complete: boolean
        }>('persona-stream-chunk', event => {
          if (event.payload.session_id === session_id) {
            console.log(
              '[usePersonaStream] Received persona chunk:',
              event.payload.content.length,
              'chars'
            )
            setStreamingContent(prev => prev + event.payload.content)
          }
        })
        unlistenFns.current.push(unlistenChunk)

        // 监听 complete 事件
        const unlistenComplete = await listen<{ session_id: string; content: string }>(
          'persona-stream-complete',
          event => {
            if (event.payload.session_id === session_id) {
              console.log('[usePersonaStream] Persona stream complete:', session_id)
              setStreamingContent('')
              setIsStreaming(false)
              setIsComplete(true)

              // 确保最终内容被解析
              const finalPersonas = parsePersonasFromMarkdown(event.payload.content)
              if (finalPersonas.length > 0) {
                setPersonas(finalPersonas)
              }
            }
          }
        )
        unlistenFns.current.push(unlistenComplete)

        // 监听 error 事件
        const unlistenError = await listen<{ session_id: string; error: string }>(
          'persona-stream-error',
          event => {
            if (event.payload.session_id === session_id) {
              console.error('[usePersonaStream] Persona stream error:', event.payload.error)
              setError(event.payload.error)
              setIsStreaming(false)
            }
          }
        )
        unlistenFns.current.push(unlistenError)
      } catch (err) {
        console.error('[usePersonaStream] Start stream failed:', err)
        setError(err instanceof Error ? err.message : '生成失败')
        setIsStreaming(false)
      }
    },
    [getActiveConfig]
  )

  // 停止流式生成
  const stopStream = useCallback(() => {
    if (typingTimerRef.current) {
      clearTimeout(typingTimerRef.current)
      typingTimerRef.current = null
    }

    setIsStreaming(false)
    setStreamingContent('')

    // 清理所有监听器
    unlistenFns.current.forEach(unlistenFn => {
      if (typeof unlistenFn === 'function') {
        unlistenFn()
      }
    })
    unlistenFns.current = []
  }, [])

  // 重置状态
  const reset = useCallback(() => {
    stopStream()
    setPersonas([])
    setMarkdownContent('')
    setError(null)
    setIsComplete(false)
    setSessionId(null)
  }, [stopStream])

  // Cleanup on unmount
  useEffect(() => {
    return () => {
      if (typingTimerRef.current) {
        clearTimeout(typingTimerRef.current)
      }
      unlistenFns.current.forEach(unlistenFn => {
        if (typeof unlistenFn === 'function') {
          unlistenFn()
        }
      })
    }
  }, [])

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
