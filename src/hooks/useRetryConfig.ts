import { useState, useCallback } from 'react'
import { invoke } from '@tauri-apps/api/core'

/**
 * 重试配置类型
 */
export interface RetryConfig {
  maxRetries: number
  baseDelaySeconds: number
  maxDelaySeconds: number
  autoRetryEnabled: boolean
}

/**
 * 重试配置 Hook
 * 提供项目级别重试配置的读取和更新功能
 */
export function useRetryConfig(projectId: string) {
  const [config, setConfig] = useState<RetryConfig>({
    maxRetries: 3,
    baseDelaySeconds: 60,
    maxDelaySeconds: 3600,
    autoRetryEnabled: true,
  })
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  /**
   * 加载重试配置（目前使用默认值，后续可从后端加载）
   */
  const loadConfig = useCallback(async () => {
    setLoading(true)
    setError(null)

    try {
      // TODO: 实现从后端加载配置的 Command
      // const response = await invoke<RetryConfig>('get_retry_config', { projectId })
      // setConfig(response)

      // 目前使用默认配置
      console.log(`[useRetryConfig] Using default config for project ${projectId}`)
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : 'Failed to load retry config'
      setError(errorMsg)
      console.error('[useRetryConfig] Failed to load config:', err)
    } finally {
      setLoading(false)
    }
  }, [projectId])

  /**
   * 更新重试配置
   */
  const updateConfig = useCallback(
    async (newConfig: Partial<RetryConfig>) => {
      setLoading(true)
      setError(null)

      const updatedConfig = { ...config, ...newConfig }

      // 验证配置
      if (updatedConfig.maxRetries < 1 || updatedConfig.maxRetries > 10) {
        setError('最大重试次数必须在 1-10 之间')
        setLoading(false)
        return false
      }

      if (updatedConfig.baseDelaySeconds < 30 || updatedConfig.baseDelaySeconds > 300) {
        setError('基础延迟时间必须在 30-300 秒之间')
        setLoading(false)
        return false
      }

      if (updatedConfig.maxDelaySeconds < 300 || updatedConfig.maxDelaySeconds > 7200) {
        setError('最大延迟时间必须在 300-7200 秒之间')
        setLoading(false)
        return false
      }

      if (updatedConfig.maxDelaySeconds < updatedConfig.baseDelaySeconds) {
        setError('最大延迟时间不能小于基础延迟时间')
        setLoading(false)
        return false
      }

      try {
        await invoke('update_user_story_retry_config', {
          request: {
            projectId,
            maxRetries: updatedConfig.maxRetries,
            baseDelaySeconds: updatedConfig.baseDelaySeconds,
            maxDelaySeconds: updatedConfig.maxDelaySeconds,
          },
        })

        setConfig(updatedConfig)
        console.log('[useRetryConfig] Config updated successfully')
        return true
      } catch (err) {
        const errorMsg = err instanceof Error ? err.message : 'Failed to update retry config'
        setError(errorMsg)
        console.error('[useRetryConfig] Failed to update config:', err)
        return false
      } finally {
        setLoading(false)
      }
    },
    [config, projectId]
  )

  /**
   * 重置为默认配置
   */
  const resetConfig = useCallback(() => {
    setConfig({
      maxRetries: 3,
      baseDelaySeconds: 60,
      maxDelaySeconds: 3600,
      autoRetryEnabled: true,
    })
    setError(null)
  }, [])

  /**
   * 获取当前配置
   */
  const getConfig = useCallback((): RetryConfig => {
    return config
  }, [config])

  return {
    config,
    loading,
    error,
    loadConfig,
    updateConfig,
    resetConfig,
    getConfig,
  }
}
