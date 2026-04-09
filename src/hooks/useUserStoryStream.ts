import { useState, useCallback, useRef } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import type { UserStory } from '@/types'

/**
 * 从 Markdown 表格解析用户故事数组
 * @param markdownContent AI返回的Markdown表格内容
 * @returns 解析后的UserStory数组
 */
function parseUserStoriesFromMarkdown(markdownContent: string): UserStory[] {
  const userStories: UserStory[] = []

  try {
    // 提取表格行
    const lines = markdownContent.split('\n')
    const tableRows: string[] = []
    let inTable = false

    for (const line of lines) {
      const trimmed = line.trim()

      // 检测表格开始(以|开头)
      if (trimmed.startsWith('|') && !inTable) {
        inTable = true
      }

      if (inTable && trimmed.startsWith('|')) {
        // 跳过分隔线(如 |---|---|)
        if (trimmed.match(/^\|[\s\-:|]+\|$/)) {
          continue
        }
        tableRows.push(trimmed)
      } else if (inTable && !trimmed.startsWith('|')) {
        // 表格结束
        break
      }
    }

    // 解析每一行(跳过表头)
    for (let i = 1; i < tableRows.length; i++) {
      const row = tableRows[i]
      const cells = row
        .split('|')
        .map(cell => cell.trim())
        .filter((_, index) => index > 0 && index < row.split('|').length)

      if (cells.length < 10) continue // 至少需要10列

      // 映射表格列到UserStory字段
      // 表格格式: 序号 | 标题 | 角色 | 功能 | 价值 | 优先级 | 故事点 | 验收标准 | 模块 | 标签 | 依赖
      const [
        storyNumber,
        title,
        role,
        feature,
        benefit,
        priority,
        storyPointsStr,
        acceptanceCriteriaStr,
        featureModule,
        labelsStr,
        dependenciesStr,
      ] = cells

      // 解析验收标准(分号分隔)
      const acceptanceCriteria = acceptanceCriteriaStr
        ? acceptanceCriteriaStr
            .split(/[;；]/)
            .map(s => s.trim())
            .filter(s => s.length > 0)
        : []

      // 解析标签(逗号分隔)
      const labels = labelsStr
        ? labelsStr
            .split(/[,，]/)
            .map(s => s.trim())
            .filter(s => s.length > 0)
        : []

      // 解析依赖
      const dependencies =
        dependenciesStr && dependenciesStr !== '无'
          ? dependenciesStr
              .split(/[,，]/)
              .map(s => s.trim())
              .filter(s => s.length > 0)
          : []

      // 解析故事点
      const storyPoints = storyPointsStr ? parseInt(storyPointsStr, 10) : undefined

      // 构建UserStory对象
      const userStory: UserStory = {
        id: `us-${Date.now()}-${i}`,
        storyNumber: storyNumber || `US-${String(i).padStart(3, '0')}`,
        title: title || '未命名故事',
        description: `As a ${role}, I want ${feature}, so that ${benefit}`,
        role: role || '',
        feature: feature || '',
        benefit: benefit || '',
        acceptanceCriteria,
        priority: (priority as 'P0' | 'P1' | 'P2' | 'P3') || 'P2',
        status: 'draft',
        storyPoints: isNaN(storyPoints || 0) ? undefined : storyPoints,
        dependencies,
        featureModule: featureModule || '',
        labels,
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
      }

      userStories.push(userStory)
    }

    console.log(
      `[parseUserStoriesFromMarkdown] Parsed ${userStories.length} user stories from markdown`
    )
  } catch (error) {
    console.error('[parseUserStoriesFromMarkdown] Error parsing markdown:', error)
  }

  return userStories
}

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
  startStream: (
    request: UserStoryStreamRequest,
    onComplete?: (stories: UserStory[]) => void
  ) => Promise<void>
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
    async (request: UserStoryStreamRequest, onComplete?: (stories: UserStory[]) => void) => {
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

            // 解析 Markdown 表格为用户故事数组
            const parsedStories = parseUserStoriesFromMarkdown(content)
            setUserStories(parsedStories)

            console.log(
              `[useUserStoryStream] Stream completed with ${parsedStories.length} user stories`
            )

            // 调用完成回调（如果提供）
            if (onComplete && parsedStories.length > 0) {
              console.log('[useUserStoryStream] Calling onComplete callback')
              onComplete(parsedStories)
            }

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
        await invoke<string>('decompose_user_stories_streaming', {
          request: {
            prdContent: request.prdContent,
            provider: request.provider,
            model: request.model,
            apiKey: request.apiKey,
          },
        })

        // 响应已在事件中处理
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
