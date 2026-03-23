import { useState, useCallback } from 'react'
import { invoke } from '@tauri-apps/api/core'

export interface GitStatus {
  isGitRepo: boolean
  gitVersion: string | null
  branch: string | null
  commitCount: number | null
  isDirty: boolean | null
}

export interface GitConfig {
  userName: string | null
  userEmail: string | null
}

interface UseGitReturn {
  gitStatus: GitStatus | null
  gitConfig: GitConfig | null
  isLoading: boolean
  error: string | null
  checkGitStatus: (path: string) => Promise<void>
  initGitRepo: (path: string, initialBranch?: string) => Promise<boolean>
  setGitConfig: (path: string, key: string, value: string) => Promise<boolean>
  getGitConfig: (path: string, key: string) => Promise<string | null>
  getAllGitConfig: (path: string) => Promise<GitConfig>
}

export function useGit(): UseGitReturn {
  const [gitStatus, setGitStatus] = useState<GitStatus | null>(null)
  const [gitConfig, setGitConfigState] = useState<GitConfig | null>(null)
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const checkGitStatus = useCallback(async (path: string) => {
    setIsLoading(true)
    setError(null)
    try {
      const status = await invoke<GitStatus>('check_git_status', { path })
      setGitStatus(status)
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : 'Failed to check git status'
      setError(errorMsg)
      setGitStatus(null)
    } finally {
      setIsLoading(false)
    }
  }, [])

  const initGitRepo = useCallback(
    async (path: string, initialBranch?: string): Promise<boolean> => {
      setIsLoading(true)
      setError(null)
      try {
        const result = await invoke<boolean>('init_git_repo', {
          request: {
            path,
            initialBranch: initialBranch || 'main',
          },
        })
        return result
      } catch (err) {
        const errorMsg = err instanceof Error ? err.message : 'Failed to initialize git repository'
        setError(errorMsg)
        return false
      } finally {
        setIsLoading(false)
      }
    },
    []
  )

  const setGitConfig = useCallback(
    async (path: string, key: string, value: string): Promise<boolean> => {
      setIsLoading(true)
      setError(null)
      try {
        const result = await invoke<boolean>('set_git_config', {
          request: {
            path,
            key,
            value,
          },
        })
        return result
      } catch (err) {
        const errorMsg = err instanceof Error ? err.message : 'Failed to set git config'
        setError(errorMsg)
        return false
      } finally {
        setIsLoading(false)
      }
    },
    []
  )

  const getGitConfig = useCallback(async (path: string, key: string): Promise<string | null> => {
    setIsLoading(true)
    setError(null)
    try {
      const result = await invoke<string | null>('get_git_config', {
        request: {
          path,
          key,
        },
      })
      return result
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : 'Failed to get git config'
      setError(errorMsg)
      return null
    } finally {
      setIsLoading(false)
    }
  }, [])

  const getAllGitConfig = useCallback(async (path: string): Promise<GitConfig> => {
    setIsLoading(true)
    setError(null)
    try {
      const config = await invoke<GitConfig>('get_all_git_config', { path })
      setGitConfigState(config)
      return config
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : 'Failed to get all git config'
      setError(errorMsg)
      return { userName: null, userEmail: null }
    } finally {
      setIsLoading(false)
    }
  }, [])

  return {
    gitStatus,
    gitConfig,
    isLoading,
    error,
    checkGitStatus,
    initGitRepo,
    setGitConfig,
    getGitConfig,
    getAllGitConfig,
  }
}
