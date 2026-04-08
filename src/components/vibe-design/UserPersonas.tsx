import { useEffect, useRef } from 'react'
import { useParams, useNavigate } from 'react-router-dom'
import { ArrowRight, ArrowLeft, RefreshCw } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { UserPersonasDisplay } from '@/components/UserPersonasDisplay'
import { useProjectStore, useAppStore, useAIConfigStore } from '@/stores'
import { usePersonaStream } from '@/hooks/usePersonaStream'
import { ProjectListFloatingButton } from './ProjectListFloatingButton'

export function UserPersonas() {
  const { projectId } = useParams<{ projectId: string }>()
  const navigate = useNavigate()
  const { getProjectById, setProjectPersonas, syncProjectToDatabase } = useProjectStore()
  const { setLoading } = useAppStore()
  const aiConfigStore = useAIConfigStore()

  // 防止重复启动流式生成的标志
  const hasStartedGenerationRef = useRef(false)

  const project = projectId ? getProjectById(projectId) : undefined

  // 使用流式生成 Hook
  const { personas, isStreaming, isComplete, error, startStream } = usePersonaStream()

  const hasExistingPersonas = project?.userPersonas && project.userPersonas.length > 0

  // 初始化时加载已有的用户画像或触发生成
  useEffect(() => {
    if (project && !hasStartedGenerationRef.current) {
      if (hasExistingPersonas && project.userPersonas) {
        // 已有数据，直接使用
        console.log('[UserPersonas] Using existing personas:', project.userPersonas.length)
      } else {
        // 无数据，触发流式生成
        hasStartedGenerationRef.current = true
        triggerStreamGeneration()
      }
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [project])

  // 触发生成
  const triggerStreamGeneration = async () => {
    if (!projectId) return

    setLoading(true, 'AI 正在生成用户画像...')

    try {
      const activeConfig = aiConfigStore.getActiveConfig()

      if (!activeConfig?.apiKey) {
        console.error('[UserPersonas] No API key configured')
        setLoading(false)
        return
      }

      await startStream({
        idea: project?.idea || project?.description || '',
        provider: aiConfigStore.defaultProvider,
        model: activeConfig.model,
        apiKey: activeConfig.apiKey,
      })
    } catch (err) {
      console.error('[UserPersonas] Generation failed:', err)
    } finally {
      setLoading(false)
    }
  }

  // 保存生成的用户画像到项目
  useEffect(() => {
    if (isComplete && personas.length > 0 && projectId) {
      setProjectPersonas(projectId, personas)
      console.log('[UserPersonas] Saved personas to project:', personas.length)

      // 同步到数据库
      syncProjectToDatabase(projectId).catch(err => {
        console.error('[UserPersonas] Failed to sync personas to database:', err)
      })
    }
  }, [isComplete, personas, projectId, setProjectPersonas, syncProjectToDatabase])

  if (!project) {
    return (
      <div className="text-center py-12">
        <p className="text-muted-foreground">项目不存在</p>
        <Button onClick={() => navigate('/')} className="mt-4">
          返回首页
        </Button>
      </div>
    )
  }

  return (
    <div className="max-w-7xl mx-auto space-y-6 px-4 py-8">
      {/* 顶部导航栏 */}
      <div className="flex items-center justify-between flex-wrap gap-4">
        <div>
          <h1 className="text-3xl font-bold text-gray-900 flex items-center gap-3">
            <span className="text-4xl">👥</span>
            用户画像
          </h1>
          <p className="text-gray-600 mt-1 font-medium">{project.name}</p>
          {isStreaming && (
            <p className="text-sm text-blue-600 flex items-center gap-2 mt-2">
              <span className="animate-pulse inline-block">✨</span>
              <span className="font-medium">AI 正在生成中...</span>
            </p>
          )}
          {isComplete && (
            <p className="text-sm text-green-600 font-medium mt-1">
              ✓ 已生成 {personas.length} 个用户画像
            </p>
          )}
        </div>
        <div className="flex items-center gap-3">
          {/* 重新生成按钮 */}
          <Button
            variant="outline"
            size="sm"
            onClick={triggerStreamGeneration}
            disabled={isStreaming}
            className="gap-2"
          >
            <RefreshCw className={`w-4 h-4 ${isStreaming ? 'animate-spin' : ''}`} />
            {isStreaming ? '生成中...' : '重新生成'}
          </Button>
        </div>
      </div>

      {/* 显示错误状态 */}
      {error && personas.length === 0 && (
        <div className="bg-red-50 border border-red-200 rounded-lg p-6 text-center">
          <p className="text-red-800 mb-4 font-medium">{error}</p>
          <Button onClick={triggerStreamGeneration} variant="destructive" className="gap-2">
            <RefreshCw className="w-4 h-4" />
            重新生成
          </Button>
        </div>
      )}

      {/* 用户画像展示组件 */}
      <UserPersonasDisplay
        isGenerating={isStreaming}
        progress={isStreaming ? 50 : 100}
        personas={personas}
      />

      {/* 底部导航 */}
      <div className="flex justify-between items-center pt-6 border-t">
        <Button
          variant="outline"
          onClick={() => navigate(`/prd/${projectId}`)}
          className="gap-2 px-6"
        >
          <ArrowLeft className="w-4 h-4" />
          返回 PRD
        </Button>
        <Button
          onClick={() => navigate(`/competitors/${projectId}`)}
          className="gap-2 px-6 bg-gradient-to-r from-primary to-cyan-600 hover:from-primary/90 hover:to-cyan-600/90"
        >
          查看竞品分析
          <ArrowRight className="w-4 h-4" />
        </Button>
      </div>

      {/* 项目列表悬浮按钮 */}
      <ProjectListFloatingButton />
    </div>
  )
}
