import { useEffect, useState } from 'react'
import type { PRD } from '@/types'
import { usePRDStream } from '@/hooks/usePRDStream'
import { useAIConfigStore } from '@/stores/aiConfigStore'
import { generateMockPRD } from './PRDDisplayUtils'

interface UsePRDGenerationParams {
  projectId: string | undefined
  projectIdea: string
  setProjectPRD: (projectId: string, prd: PRD) => void
  updateProjectStatus: (projectId: string, status: any) => void
  updateProjectProgress: (projectId: string, progress: number) => void
  syncProjectToDatabase: (projectId: string) => Promise<void>
  setLoading: (loading: boolean, message?: string) => void
  urlParams?: URLSearchParams // 从 URL 读取的参数
}

export function usePRDGeneration({
  projectId,
  projectIdea,
  setProjectPRD,
  updateProjectStatus,
  updateProjectProgress,
  syncProjectToDatabase,
  setLoading,
  urlParams,
}: UsePRDGenerationParams) {
  const aiConfigStore = useAIConfigStore()

  // 使用 PRD 流式生成 Hook
  const {
    prd: streamingPRD,
    markdownContent,
    isStreaming,
    isComplete,
    error,
    startStream,
    stopStream,
    reset,
  } = usePRDStream()

  const generatePRD = async () => {
    // 检查是否有 URL 参数（来自 IdeaInput 的流式生成请求）
    const mode = urlParams?.get('mode')
    
    if (mode === 'streaming') {
      // 从 URL 参数中获取 AI 配置
      const idea = decodeURIComponent(urlParams.get('idea') || projectIdea)
      const provider = urlParams.get('provider') || ''
      const model = urlParams.get('model') || ''
      const apiKey = urlParams.get('apiKey') || ''
      
      if (!apiKey || !provider || !model) {
        console.error('[usePRDGeneration] Missing AI config from URL params')
        return
      }
      
      console.log('[usePRDGeneration] Starting streaming generation from URL params')
      
      // 直接使用 URL 中的配置进行流式生成
      reset()
      await startStream({
        idea,
        provider,
        model,
        apiKey,
      })
      return
    }
    
    // 原有的逻辑：从 store 获取 AI 配置
    const activeConfig = aiConfigStore.getActiveConfig()

    if (activeConfig?.apiKey) {
      setLoading(true, 'AI 正在生成产品需求文档...')

      // 使用流式生成
      reset()
      await startStream({
        idea: projectIdea,
        provider: activeConfig.provider,
        model: activeConfig.model,
        apiKey: activeConfig.apiKey,
      })
    } else {
      // 降级到模拟生成
      setLoading(true, '正在生成产品需求文档...')
      try {
        await new Promise(resolve => setTimeout(resolve, 2000))
        const generatedPRD = generateMockPRD(projectIdea)

        if (projectId) {
          setProjectPRD(projectId, generatedPRD)
          updateProjectStatus(projectId, 'design')
          updateProjectProgress(projectId, 25)
        }
      } finally {
        setLoading(false)
      }
    }
  }

  // 监听流式完成，保存 PRD
  useEffect(() => {
    if (isComplete && streamingPRD && projectId) {
      setProjectPRD(projectId, streamingPRD)
      updateProjectStatus(projectId, 'design')
      updateProjectProgress(projectId, 25)
      setLoading(false)
      
      // 同步到数据库
      syncProjectToDatabase(projectId).catch(err => {
        console.error('[PRDDisplay] Failed to sync PRD to database:', err)
      })
    }
  }, [
    isComplete,
    streamingPRD,
    projectId,
    setProjectPRD,
    updateProjectStatus,
    updateProjectProgress,
    setLoading,
    syncProjectToDatabase,
  ])

  const handleStopGeneration = () => {
    stopStream()
    setLoading(false)
  }

  return {
    streamingPRD,
    markdownContent,
    isStreaming,
    isComplete,
    error,
    generatePRD,
    handleStopGeneration,
  }
}
