import { useState, useCallback, useRef, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import type { CompetitorAnalysis } from '@/types'

export interface CompetitorStreamRequest {
  idea: string
  provider: string
  model: string
  apiKey: string
}

export interface UseCompetitorStreamReturn {
  analysis: CompetitorAnalysis | null
  isStreaming: boolean
  isComplete: boolean
  error: string | null
  sessionId: string | null
  startStream: (request: CompetitorStreamRequest) => Promise<void>
  stopStream: () => void
  reset: () => void
}

/**
 * 解析竞品分析流式内容为结构化数据
 */
function parseCompetitorFromChunk(markdown: string): Partial<CompetitorAnalysis> {
  const result: Partial<CompetitorAnalysis> = {
    competitors: [],
    differentiation: '',
    opportunities: [],
  }

  // 提取竞品部分
  const competitorsMatch = markdown.match(
    /##\s+竞品分析\s*\n([\s\S]*?)(?=##\s+(?:差异化优势 | 市场机会)|$)/i
  )
  if (!competitorsMatch) return result

  const competitorsText = competitorsMatch[1]

  // 提取各个竞品卡片
  const competitorCards = competitorsText.split(/(?=\*\*[^*\n]+\*\*)/g).filter(Boolean)

  competitorCards.forEach(cardText => {
    const nameMatch = cardText.match(/\*\*([^*\n]+)\*\*/)

    if (nameMatch) {
      const lines = cardText.split('\n')
      let currentSection: 'strengths' | 'weaknesses' | null = null
      const strengths: string[] = []
      const weaknesses: string[] = []
      let marketShare: string | undefined

      for (const line of lines) {
        const trimmed = line.trim()

        // 检查是否是新的 section 开始 (支持中文冒号)
        if (/^\s*优势\s*[::]\s*$/.test(trimmed)) {
          currentSection = 'strengths'
          continue
        }
        if (/^\s*劣势\s*[::]\s*$/.test(trimmed)) {
          currentSection = 'weaknesses'
          continue
        }

        // 检查市场份额 (支持中文冒号和空格)
        const msMatch = trimmed.match(/市场份额\s*[:：]\s*(\d+%)/i)
        if (msMatch) {
          marketShare = msMatch[1]
          currentSection = null
          continue
        }

        // 如果是列表项，添加到当前 section
        if (trimmed.startsWith('-') && currentSection) {
          const content = trimmed.replace(/^-\s*/, '').trim()
          if (content.length > 0) {
            if (currentSection === 'strengths') {
              strengths.push(content)
            } else if (currentSection === 'weaknesses') {
              weaknesses.push(content)
            }
          }
        }
      }

      result.competitors?.push({
        name: nameMatch[1].trim(),
        strengths,
        weaknesses,
        marketShare,
      })
    }
  })

  // 提取差异化优势
  const diffMatch = markdown.match(/##\s+差异化优势\s*\n([\s\S]*?)(?=##\s+市场机会|$)/i)
  if (diffMatch) {
    result.differentiation = diffMatch[1].trim()
  }

  // 提取市场机会
  const oppMatch = markdown.match(/##\s+市场机会\s*\n([\s\S]*?)$/i)
  if (oppMatch) {
    const oppText = oppMatch[1]
    result.opportunities = oppText
      .split('\n')
      .filter(line => line.trim().startsWith('-'))
      .map(line => line.replace(/^-\s*/, '').trim())
      .filter(line => line.length > 0)
  }

  return result
}

/**
 * 竞品分析流式生成 Hook（打字机效果 + 渐进式渲染）
 */
export function useCompetitorStream(): UseCompetitorStreamReturn {
  const [analysis, setAnalysis] = useState<CompetitorAnalysis | null>(null)
  const [isStreaming, setIsStreaming] = useState(false)
  const [isComplete, setIsComplete] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [sessionId, setSessionId] = useState<string | null>(null)

  const unlistenRef = useRef<UnlistenFn[]>([])
  const isStreamingRef = useRef(false)
  // 使用 ref 存储累积的内容，避免闭包问题
  const accumulatedContentRef = useRef('')
  // 防抖定时器引用
  const parseTimerRef = useRef<NodeJS.Timeout | null>(null)

  // 清理所有订阅
  const cleanup = useCallback(() => {
    // 清除防抖定时器
    if (parseTimerRef.current) {
      clearTimeout(parseTimerRef.current)
      parseTimerRef.current = null
    }

    unlistenRef.current.forEach(unlisten => {
      try {
        unlisten()
      } catch (err) {
        console.error('[useCompetitorStream] Cleanup error:', err)
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
    accumulatedContentRef.current = ''
    setAnalysis(null)
    setIsComplete(false)
    setError(null)
    setSessionId(null)
  }, [stopStream])

  // 解析并更新竞品分析状态（带防抖）
  const parseAndUpdateAnalysis = useCallback((content: string, isFinal = false) => {
    // 如果是最终更新或距离上次解析超过 100ms，则立即解析
    const shouldParseImmediately = isFinal || !parseTimerRef.current

    if (shouldParseImmediately) {
      // 清除之前的定时器
      if (parseTimerRef.current) {
        clearTimeout(parseTimerRef.current)
        parseTimerRef.current = null
      }

      // 解析为结构化竞品分析
      const parsed = parseCompetitorFromChunk(content)
      setAnalysis(prevAnalysis => ({
        competitors:
          parsed.competitors && parsed.competitors.length > 0
            ? parsed.competitors
            : prevAnalysis?.competitors || [],
        differentiation: parsed.differentiation || prevAnalysis?.differentiation || '',
        opportunities:
          parsed.opportunities && parsed.opportunities.length > 0
            ? parsed.opportunities
            : prevAnalysis?.opportunities || [],
      }))
    } else {
      // 否则设置防抖，等待更多数据
      parseTimerRef.current = setTimeout(() => {
        const parsed = parseCompetitorFromChunk(content)
        setAnalysis(prevAnalysis => ({
          competitors:
            parsed.competitors && parsed.competitors.length > 0
              ? parsed.competitors
              : prevAnalysis?.competitors || [],
          differentiation: parsed.differentiation || prevAnalysis?.differentiation || '',
          opportunities:
            parsed.opportunities && parsed.opportunities.length > 0
              ? parsed.opportunities
              : prevAnalysis?.opportunities || [],
        }))
        parseTimerRef.current = null
      }, 100)
    }
  }, [])

  // 开始流式生成竞品分析
  const startStream = useCallback(
    async (request: CompetitorStreamRequest) => {
      // 清理之前的订阅
      cleanup()

      // 重置状态和累积内容
      accumulatedContentRef.current = ''
      setAnalysis(null)
      setIsComplete(false)
      setError(null)
      setIsStreaming(true)
      isStreamingRef.current = true

      try {
        // 监听竞品分析流式 chunk 事件
        const unlistenChunk = await listen<{
          session_id: string
          content: string
          is_complete: boolean
        }>('competitor-stream-chunk', event => {
          console.log(
            '[useCompetitorStream] Received chunk:',
            event.payload.content.length,
            'chars'
          )

          // 使用 ref 累积内容，确保原子性
          accumulatedContentRef.current += event.payload.content

          // 基于 ref 中的最新内容进行解析和更新
          parseAndUpdateAnalysis(accumulatedContentRef.current)
        })
        unlistenRef.current.push(unlistenChunk)

        // 监听完成事件
        const unlistenComplete = await listen<{ session_id: string; content: string }>(
          'competitor-stream-complete',
          event => {
            console.log('[useCompetitorStream] Stream complete:', event.payload.session_id)
            setIsComplete(true)
            setIsStreaming(false)
            isStreamingRef.current = false

            // 使用后端返回的最终完整内容
            accumulatedContentRef.current = event.payload.content
            parseAndUpdateAnalysis(event.payload.content, true)

            cleanup()
          }
        )
        unlistenRef.current.push(unlistenComplete)

        // 监听错误事件
        const unlistenError = await listen<{ session_id: string; error: string }>(
          'competitor-stream-error',
          event => {
            console.error('[useCompetitorStream] Stream error:', event.payload.error)
            setError(event.payload.error)
            setIsStreaming(false)
            isStreamingRef.current = false
            cleanup()
          }
        )
        unlistenRef.current.push(unlistenError)

        // 调用后端流式命令
        const response = await invoke<string>('stream_generate_competitor_analysis', {
          request: {
            idea: request.idea,
            provider: request.provider,
            model: request.model,
            api_key: request.apiKey,
          },
        })

        setSessionId(response)
      } catch (err) {
        console.error('[useCompetitorStream] Error starting stream:', err)
        setError(err instanceof Error ? err.message : '未知错误')
        setIsStreaming(false)
        isStreamingRef.current = false
        cleanup()
      }
    },
    [cleanup, parseAndUpdateAnalysis]
  )

  // 组件卸载时清理
  useEffect(() => {
    return () => {
      cleanup()
    }
  }, [cleanup])

  return {
    analysis,
    isStreaming,
    isComplete,
    error,
    sessionId,
    startStream,
    stopStream,
    reset,
  }
}
