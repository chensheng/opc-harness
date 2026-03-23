import { useState, useCallback } from 'react'
import { invoke } from '@tauri-apps/api/core'

export interface ToolStatus {
  name: string
  is_installed: boolean
  version: string | null
  install_url: string | null
}

export interface DetectToolsResponse {
  tools: ToolStatus[]
}

export interface UseToolDetectorReturn {
  tools: ToolStatus[]
  isLoading: boolean
  error: string | null
  detectTools: () => Promise<void>
  installedCount: number
  totalCount: number
}

export function useToolDetector(): UseToolDetectorReturn {
  const [tools, setTools] = useState<ToolStatus[]>([])
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const detectTools = useCallback(async () => {
    setIsLoading(true)
    setError(null)

    try {
      const response = await invoke<DetectToolsResponse>('detect_tools')
      setTools(response.tools)
      console.log('[useToolDetector] Detected tools:', response.tools)
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : '检测工具失败'
      console.error('[useToolDetector] Error detecting tools:', errorMessage)
      setError(errorMessage)
      throw err
    } finally {
      setIsLoading(false)
    }
  }, [])

  const installedCount = tools.filter(t => t.is_installed).length
  const totalCount = tools.length

  return {
    tools,
    isLoading,
    error,
    detectTools,
    installedCount,
    totalCount,
  }
}
