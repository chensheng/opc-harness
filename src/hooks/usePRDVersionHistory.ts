/**
 * PRD 版本历史 Hook
 */

import { useState, useCallback } from 'react'
import { invoke } from '@tauri-apps/api/core'
import type { PRDVersion, PRDDiff, IterationHistory } from '@/types/prd-iteration'

interface UsePRDVersionHistoryReturn {
  /** 版本历史 */
  history: IterationHistory | null
  /** 是否正在加载 */
  isLoading: boolean
  /** 错误信息 */
  error: string | null
  /** 获取版本历史 */
  loadHistory: () => Promise<IterationHistory>
  /** 对比两个版本 */
  compareVersions: (versionId1: string, versionId2: string) => Promise<PRDDiff>
  /** 回滚到指定版本 */
  rollbackToVersion: (versionId: string) => Promise<PRDVersion>
  /** 刷新历史 */
  refresh: () => Promise<void>
  /** 重置 */
  reset: () => void
}

/**
 * PRD 版本历史 Hook
 */
export function usePRDVersionHistory(): UsePRDVersionHistoryReturn {
  const [history, setHistory] = useState<IterationHistory | null>(null)
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  /**
   * 获取版本历史
   */
  const loadHistory = useCallback(async (): Promise<IterationHistory> => {
    setIsLoading(true)
    setError(null)

    try {
      const response = await invoke<IterationHistory>('get_iteration_history', {
        request: {},
      })
      setHistory(response)
      return response
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : '获取版本历史失败'
      setError(errorMessage)
      console.error('Failed to load version history:', err)
      throw err
    } finally {
      setIsLoading(false)
    }
  }, [])

  /**
   * 对比两个版本
   */
  const compareVersions = useCallback(
    async (versionId1: string, versionId2: string): Promise<PRDDiff> => {
      setIsLoading(true)
      setError(null)

      try {
        // 简化实现：从历史记录中计算差异
        if (!history) {
          throw new Error('版本历史未加载')
        }

        const version1 = history.versions.find(v => v.versionId === versionId1)
        const version2 = history.versions.find(v => v.versionId === versionId2)

        if (!version1 || !version2) {
          throw new Error('一个或两个版本不存在')
        }

        // 调用后端计算差异（简化实现）
        const response = await invoke<PRDDiff>('compare_versions', {
          request: {
            version_id_1: versionId1,
            version_id_2: versionId2,
          },
        })

        return response
      } catch (err) {
        const errorMessage = err instanceof Error ? err.message : '版本对比失败'
        setError(errorMessage)
        console.error('Failed to compare versions:', err)
        throw err
      } finally {
        setIsLoading(false)
      }
    },
    [history]
  )

  /**
   * 回滚到指定版本
   */
  const rollbackToVersion = useCallback(
    async (versionId: string): Promise<PRDVersion> => {
      setIsLoading(true)
      setError(null)

      try {
        const response = await invoke<{ version: PRDVersion }>('rollback_to_version', {
          request: {
            version_id: versionId,
          },
        })

        // 回滚后刷新历史
        await loadHistory()

        return response.version
      } catch (err) {
        const errorMessage = err instanceof Error ? err.message : '版本回滚失败'
        setError(errorMessage)
        console.error('Failed to rollback version:', err)
        throw err
      } finally {
        setIsLoading(false)
      }
    },
    [loadHistory]
  )

  /**
   * 刷新历史
   */
  const refresh = useCallback(async () => {
    await loadHistory()
  }, [loadHistory])

  /**
   * 重置
   */
  const reset = useCallback(() => {
    setHistory(null)
    setIsLoading(false)
    setError(null)
  }, [])

  return {
    history,
    isLoading,
    error,
    loadHistory,
    compareVersions,
    rollbackToVersion,
    refresh,
    reset,
  }
}
