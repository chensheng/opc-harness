import { useState, useCallback, useRef } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { useAIConfigStore } from '@/stores/aiConfigStore'
import { useProjectStore } from '@/stores'

export interface ChatMessage {
  role: 'user' | 'assistant'
  content: string
}

export interface UsePRDAIChatReturn {
  messages: ChatMessage[]
  isStreaming: boolean
  error: string | null
  sendMessage: (userMessage: string, currentPRDContent: string, projectId?: string) => Promise<void>
  stopStream: () => void
  reset: () => void
}

/**
 * PRD AI 对话优化 Hook
 * 支持通过自然语言对话方式优化 PRD 内容
 */
export function usePRDAIChat(): UsePRDAIChatReturn {
  const [messages, setMessages] = useState<ChatMessage[]>([])
  const [isStreaming, setIsStreaming] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const aiConfigStore = useAIConfigStore()
  const projectStore = useProjectStore()
  const unlistenRef = useRef<UnlistenFn[]>([])
  const isStreamingRef = useRef(false)
  const accumulatedContentRef = useRef('')

  const cleanup = useCallback(() => {
    unlistenRef.current.forEach(unlisten => {
      try {
        unlisten()
      } catch (err) {
        console.error('[usePRDAIChat] Cleanup error:', err)
      }
    })
    unlistenRef.current = []
  }, [])

  const stopStream = useCallback(() => {
    cleanup()
    isStreamingRef.current = false
    setIsStreaming(false)
  }, [cleanup])

  const reset = useCallback(() => {
    stopStream()
    setMessages([])
    setError(null)
    accumulatedContentRef.current = ''
  }, [stopStream])

  const sendMessage = useCallback(
    async (userMessage: string, currentPRDContent: string, projectId?: string) => {
      const activeConfig = aiConfigStore.getActiveConfig()

      // CodeFree CLI 不需要 API Key，其他 provider 需要检查
      if (activeConfig?.provider !== 'codefree' && !activeConfig?.apiKey) {
        setError('请先配置 AI 提供商')
        return
      }

      // 添加用户消息
      const userMsg: ChatMessage = { role: 'user', content: userMessage }
      setMessages(prev => [...prev, userMsg])

      // 重置状态
      cleanup()
      accumulatedContentRef.current = ''
      setError(null)
      setIsStreaming(true)
      isStreamingRef.current = true

      // 添加临时的助手消息占位
      setMessages(prev => [...prev, { role: 'assistant', content: '' }])

      try {
        let prdContentForAI = currentPRDContent
        let workspacePath: string | null = null

        // 如果使用 CodeFree，需要将 PRD 和系统提示词写入文件并通过 @ 引用
        if (activeConfig?.provider === 'codefree' && projectId) {
          const project = projectStore.getProjectById(projectId)
          if (project) {
            try {
              console.log('[usePRDAIChat] Writing PRD and AGENTS files for CodeFree...')

              // 获取项目的实际工作区路径
              workspacePath = await invoke<string>('get_project_workspace_path', {
                projectId,
              })

              console.log('[usePRDAIChat] Project workspace path:', workspacePath)

              // 1. 写入 PRD.md
              const filePath = await invoke<string>('write_prd_to_file', {
                projectPath: workspacePath,
                prdContent: currentPRDContent,
              })

              console.log('[usePRDAIChat] PRD written to:', filePath)

              // 在提示词中使用 @ 引用（文件在 .opc-harness 子目录中）
              prdContentForAI = '@.opc-harness/PRD.md'
            } catch (err) {
              console.error('[usePRDAIChat] Error writing PRD and AGENTS files:', err)
              setError('无法写入 PRD 和 AGENTS 文件')
              return
            }
          }
        }

        // 构建系统提示词，指导 AI 输出完整 PRD
        const isCodeFree = activeConfig?.provider === 'codefree'

        const systemPrompt = isCodeFree
          ? `你是一个专业的产品经理助手。你的任务是基于当前 PRD 内容和用户的优化需求，生成优化后的完整 PRD 文档.

重要规则：
1. 必须输出完整的 PRD Markdown 文档，保持原有结构和风格
2. 即使用户只要求修改某一部分，也要输出包含该部分优化的完整文档
3. 不要输出解释性文字、修改建议列表或非完整文档片段
4. 未修改的部分保持原样
5. 确保文档的专业性和可读性

请读取 @.opc-harness/PRD.md 获取当前 PRD 内容，并基于用户需求和该 PRD 内容，生成优化后的完整 PRD 文档。`
          : `你是一个专业的产品经理助手。你的任务是基于当前 PRD 内容和用户的优化需求，生成优化后的完整 PRD 文档.

重要规则：
1. 必须输出完整的 PRD Markdown 文档，保持原有结构和风格
2. 即使用户只要求修改某一部分，也要输出包含该部分优化的完整文档
3. 不要输出解释性文字、修改建议列表或非完整文档片段
4. 未修改的部分保持原样
5. 确保文档的专业性和可读性

当前 PRD 内容：
${prdContentForAI}

请基于以上 PRD 和用户需求，生成优化后的完整 PRD 文档。`

        // 发送请求 - 使用 stream_chat 命令
        // ✅ 监听流式数据事件（修正事件名称为 ai-stream-chunk）
        const unlisten = await listen<{ session_id: string; content: string }>(
          'ai-stream-chunk',
          ({ payload }) => {
            console.log('[usePRDAIChat] Received chunk:', payload.content.substring(0, 50))
            accumulatedContentRef.current += payload.content
            setMessages(prev => {
              const lastMsg = prev[prev.length - 1]
              return [
                ...prev.slice(0, -1),
                {
                  role: lastMsg.role,
                  content: accumulatedContentRef.current,
                },
              ]
            })
          }
        )

        unlistenRef.current.push(unlisten)

        // 监听错误事件
        const errorUnlisten = await listen<{ error: string }>('ai-stream-error', ({ payload }) => {
          console.error('[usePRDAIChat] Stream error event received:', payload)
          setError(payload.error || '生成 PRD 时发生错误')
          isStreamingRef.current = false
          setIsStreaming(false)
          cleanup() // 清理事件监听器
        })

        unlistenRef.current.push(errorUnlisten)

        // ✅ 监听完成事件（关键修复）
        const completeUnlisten = await listen<{ session_id: string; content: string }>(
          'ai-stream-complete',
          () => {
            console.log('[usePRDAIChat] Stream completed event received')
            // 注意：不立即设置 isStreaming = false，因为 CodeFree 还需要从文件读取
            // isStreaming 会在后续的文件读取完成后或 invoke 返回后设置
          }
        )

        unlistenRef.current.push(completeUnlisten)

        // 构建消息数组
        const messages = [
          { role: 'system' as const, content: systemPrompt },
          { role: 'user' as const, content: userMessage },
        ]

        await invoke('stream_chat', {
          request: {
            provider: activeConfig?.provider || 'openai',
            model: activeConfig?.model || 'gpt-4',
            api_key: activeConfig?.apiKey || '',
            messages,
            temperature: 0.7,
            max_tokens: 8000,
            project_id: projectId || null,
          },
        })

        // ✅ stream_chat 调用完成，说明流式输出已结束
        console.log('[usePRDAIChat] stream_chat invoke completed')

        // 如果使用 CodeFree，优化完成后从文件读取最终的 PRD 内容
        if (activeConfig?.provider === 'codefree' && workspacePath) {
          console.log('[usePRDAIChat] Reading optimized PRD from file...')
          try {
            const optimizedContent = await invoke<string>('read_prd_from_file', {
              projectPath: workspacePath,
            })

            console.log('[usePRDAIChat] Successfully read optimized PRD from file')

            // 更新最后一条助手消息为文件中的内容
            setMessages(prev => {
              const newMessages = [...prev]
              const lastIndex = newMessages.length - 1
              if (lastIndex >= 0 && newMessages[lastIndex].role === 'assistant') {
                newMessages[lastIndex] = {
                  ...newMessages[lastIndex],
                  content: optimizedContent,
                }
              }
              return newMessages
            })

            // 同时更新累积内容
            accumulatedContentRef.current = optimizedContent
          } catch (err) {
            console.error('[usePRDAIChat] Failed to read optimized PRD from file:', err)
            // 即使读取失败，也不影响已有的流式内容
          }
        }

        // ✅ 所有处理完成后，设置流式状态为 false
        console.log('[usePRDAIChat] Setting isStreaming to false')
        isStreamingRef.current = false
        setIsStreaming(false)
        cleanup() // 清理所有事件监听器
      } catch (err) {
        console.error('[usePRDAIChat] Error:', err)
        // 使用后端返回的详细错误信息，而不是通用消息
        const errorMessage = err instanceof Error ? err.message : String(err)
        setError(`AI 调用失败：${errorMessage}`)
        isStreamingRef.current = false
        setIsStreaming(false)
        cleanup()
      }
    },
    [aiConfigStore, projectStore, cleanup]
  )

  return {
    messages,
    isStreaming,
    error,
    sendMessage,
    stopStream,
    reset,
  }
}
