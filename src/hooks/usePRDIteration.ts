/**
 * PRD 迭代优化 Hook
 *
 * 支持多轮迭代优化 PRD，保留版本历史记录
 *
 * @example
 * ```ts
 * const { submitFeedback, getVersionHistory, restoreToVersion } = usePRDIteration()
 * await submitFeedback('希望增加更多技术细节')
 * ```
 */

import { useState, useCallback, useRef, useEffect } from 'react'
import type { PRD, PRDVersion, PRDIterationHistory, UserFeedback } from '@/types'

/**
 * usePRDIteration 返回值
 */
export interface UsePRDIterationReturn {
  /** 当前 PRD */
  currentPRD: PRD | null
  /** 当前版本号 */
  currentVersion: number
  /** 版本历史 */
  history: PRDIterationHistory
  /** 是否正在迭代中 */
  isIterating: boolean
  /** 错误信息 */
  error: Error | null
  /** 提交反馈并重新生成 PRD */
  submitFeedback: (feedback: string) => Promise<void>
  /** 获取所有版本历史 */
  getVersionHistory: () => PRDVersion[]
  /** 恢复到指定版本 */
  restoreToVersion: (version: number) => void
  /** 获取指定版本的 PRD */
  getVersionPRD: (version: number) => PRD | undefined
  /** 清除所有历史 */
  clearHistory: () => void
  /** 重置状态 */
  reset: () => void
}

/**
 * Hook 配置选项
 */
export interface UsePRDIterationOptions {
  /** 最大保留版本数，默认 10 */
  maxVersions?: number
  /** AI Provider，默认使用 OpenAI */
  provider?: string
  /** AI 模型，默认 gpt-4o-mini */
  model?: string
  /** API Key */
  apiKey?: string
}

/**
 * 从 Tauri 命令导入
 */
import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'

export function usePRDIteration(options: UsePRDIterationOptions = {}): UsePRDIterationReturn {
  const { maxVersions = 10, provider = 'openai', model = 'gpt-4o-mini', apiKey = '' } = options

  // 状态管理
  const [currentPRD, setCurrentPRD] = useState<PRD | null>(null)
  const [currentVersion, setCurrentVersion] = useState<number>(0)
  const [history, setHistory] = useState<PRDIterationHistory>({
    currentVersion: 0,
    versions: [],
    maxVersions,
  })
  const [isIterating, setIsIterating] = useState(false)
  const [error, setError] = useState<Error | null>(null)

  // 引用管理
  const unlistenRef = useRef<UnlistenFn[]>([])
  const isIteratingRef = useRef(false)

  // 清理所有订阅
  const cleanup = useCallback(() => {
    unlistenRef.current.forEach(unlisten => {
      try {
        unlisten()
      } catch (err) {
        console.error('[usePRDIteration] Cleanup error:', err)
      }
    })
    unlistenRef.current = []
  }, [])

  // 重置状态
  const reset = useCallback(() => {
    cleanup()
    isIteratingRef.current = false
    setIsIterating(false)
    setCurrentPRD(null)
    setCurrentVersion(0)
    setHistory({
      currentVersion: 0,
      versions: [],
      maxVersions,
    })
    setError(null)
  }, [cleanup, maxVersions])

  /**
   * 提交反馈并重新生成 PRD
   */
  const submitFeedback = useCallback(
    async (feedback: string) => {
      if (!currentPRD) {
        const err = new Error('当前没有可迭代的 PRD')
        setError(err)
        throw err
      }

      // 保存当前版本到历史
      const feedbackData: UserFeedback = {
        version: currentVersion,
        feedback,
        timestamp: new Date(),
      }

      // 将当前 PRD 保存到历史
      setHistory(prev => {
        const currentVersionData = prev.versions.find(v => v.version === currentVersion)

        // 如果当前版本还没有被保存，先保存
        let versions = [...prev.versions]
        if (!currentVersionData) {
          versions.push({
            version: currentVersion,
            prd: currentPRD,
            createdAt: new Date(),
          })
        } else {
          // 更新当前版本，添加反馈信息
          versions = versions.map(v =>
            v.version === currentVersion ? { ...v, feedback: feedbackData } : v
          )
        }

        // 限制版本数量
        if (versions.length > maxVersions) {
          versions = versions.slice(-maxVersions)
        }

        return {
          ...prev,
          versions,
        }
      })

      // 开始迭代
      setIsIterating(true)
      isIteratingRef.current = true
      setError(null)

      try {
        // 调用后端流式命令进行迭代优化
        const response = await invoke<string>('stream_generate_prd_iteration', {
          request: {
            original_prd: currentPRD,
            feedback,
            provider,
            model,
            api_key: apiKey,
          },
        })

        console.log('[usePRDIteration] Iteration started, session:', response)

        // 监听迭代流式响应（复用现有的事件监听机制）
        // 注意：这里假设后端返回新的 PRD 内容，我们通过事件监听接收
        const unlistenChunk = await listen<{
          session_id: string
          content: string
          is_complete: boolean
        }>('prd-stream-chunk', event => {
          console.log(
            '[usePRDIteration] Received iteration chunk:',
            event.payload.content.length,
            'chars'
          )

          // 实时更新 PRD（简化处理，实际应该解析 Markdown）
          // 这里需要根据实际的返回格式进行调整
        })
        unlistenRef.current.push(unlistenChunk)

        // 监听完成事件
        const unlistenComplete = await listen<{ session_id: string; content: string }>(
          'prd-stream-complete',
          event => {
            console.log('[usePRDIteration] Iteration complete:', event.payload.session_id)

            // 解析新的 PRD 内容
            // 注意：这里需要实际的 Markdown 解析逻辑
            // 暂时使用占位符，实际使用时需要从 Markdown 解析
            const newPRD: PRD = {
              ...currentPRD, // 保留原有数据
              title: currentPRD.title + ' (已优化)', // 标记已优化
            }

            const newVersion = currentVersion + 1

            // 更新状态
            setCurrentPRD(newPRD)
            setCurrentVersion(newVersion)
            setHistory(prev => ({
              ...prev,
              currentVersion: newVersion,
              versions: [
                ...prev.versions,
                {
                  version: newVersion,
                  prd: newPRD,
                  feedback: feedbackData,
                  createdAt: new Date(),
                  changes: ['根据用户反馈优化'],
                },
              ].slice(-maxVersions),
            }))

            setIsIterating(false)
            isIteratingRef.current = false
            cleanup()
          }
        )
        unlistenRef.current.push(unlistenComplete)

        // 监听错误事件
        const unlistenError = await listen<{ session_id: string; error: string }>(
          'prd-stream-error',
          event => {
            console.error('[usePRDIteration] Iteration error:', event.payload.error)
            const err = new Error(event.payload.error)
            setError(err)
            setIsIterating(false)
            isIteratingRef.current = false
            cleanup()
          }
        )
        unlistenRef.current.push(unlistenError)
      } catch (err) {
        console.error('[usePRDIteration] Error submitting feedback:', err)
        const errorObj = err instanceof Error ? err : new Error(String(err))
        setError(errorObj)
        setIsIterating(false)
        isIteratingRef.current = false
        cleanup()
        throw errorObj
      }
    },
    [currentPRD, currentVersion, maxVersions, provider, model, apiKey, cleanup]
  )

  /**
   * 获取所有版本历史
   */
  const getVersionHistory = useCallback((): PRDVersion[] => {
    return [...history.versions].sort((a, b) => a.version - b.version)
  }, [history.versions])

  /**
   * 恢复到指定版本
   */
  const restoreToVersion = useCallback(
    (version: number) => {
      const versionData = history.versions.find(v => v.version === version)
      if (!versionData) {
        const err = new Error(`版本 ${version} 不存在`)
        setError(err)
        throw err
      }

      setCurrentPRD(versionData.prd)
      setCurrentVersion(version)
    },
    [history.versions]
  )

  /**
   * 获取指定版本的 PRD
   */
  const getVersionPRD = useCallback(
    (version: number): PRD | undefined => {
      return history.versions.find(v => v.version === version)?.prd
    },
    [history.versions]
  )

  /**
   * 清除所有历史
   */
  const clearHistory = useCallback(() => {
    setHistory({
      currentVersion: currentVersion,
      versions: [],
      maxVersions,
    })
  }, [currentVersion, maxVersions])

  // 组件卸载时清理
  useEffect(() => {
    return () => {
      cleanup()
    }
  }, [cleanup])

  return {
    currentPRD,
    currentVersion,
    history,
    isIterating,
    error,
    submitFeedback,
    getVersionHistory,
    restoreToVersion,
    getVersionPRD,
    clearHistory,
    reset,
  }
}
