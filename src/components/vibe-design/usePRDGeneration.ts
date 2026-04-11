import { useEffect } from 'react'
import type { PRD, ProjectStatus } from '@/types'
import { usePRDStream } from '@/hooks/usePRDStream'
import { useAIConfigStore } from '@/stores/aiConfigStore'
import { generateMockPRD } from './PRDDisplayUtils'

interface UsePRDGenerationParams {
  projectId: string | undefined
  projectIdea: string
  setProjectPRD: (projectId: string, prd: PRD) => void
  updateProjectStatus: (projectId: string, status: ProjectStatus) => void
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
      const idea = decodeURIComponent(urlParams?.get('idea') || projectIdea)
      const provider = urlParams?.get('provider') || ''
      const model = urlParams?.get('model') || ''
      const apiKey = urlParams?.get('apiKey') || ''

      // CodeFree 不需要 API Key，其他提供商需要
      const needsApiKey = provider !== 'codefree'

      if (!provider || !model || (needsApiKey && !apiKey)) {
        console.error('[usePRDGeneration] Missing AI config from URL params', {
          provider,
          model,
          apiKey: needsApiKey ? '(required)' : '(not required for codefree)',
        })
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
        projectId: projectId || null,
      })
      return
    }

    // 原有的逻辑：从 store 获取 AI 配置
    const activeConfig = aiConfigStore.getActiveConfig()

    // CodeFree CLI 不需要 API Key，其他 provider 需要检查
    const hasValidConfig =
      activeConfig && (activeConfig.provider === 'codefree' || activeConfig.apiKey)

    if (hasValidConfig) {
      // 使用流式生成（不显示加载遮罩）
      reset()
      await startStream({
        idea: projectIdea,
        provider: aiConfigStore.defaultProvider,
        model: activeConfig.model,
        apiKey: activeConfig.apiKey || '', // codefree 传空字符串
        projectId: projectId || null,
      })
    } else {
      // 降级到模拟生成
      try {
        await new Promise(resolve => setTimeout(resolve, 2000))
        const generatedPRD = generateMockPRD(projectIdea)

        if (projectId) {
          setProjectPRD(projectId, generatedPRD)
          updateProjectStatus(projectId, 'design')
          updateProjectProgress(projectId, 25)
        }
      } catch (err) {
        console.error('[usePRDGeneration] Mock generation failed:', err)
      }
    }
  }

  // 监听流式完成，保存 PRD
  useEffect(() => {
    if (isComplete && streamingPRD && projectId) {
      setProjectPRD(projectId, streamingPRD)
      updateProjectStatus(projectId, 'design')
      updateProjectProgress(projectId, 25)

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
