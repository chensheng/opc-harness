import { useState } from 'react'
import { User, Briefcase, Target, Quote } from 'lucide-react'
import { Card, CardContent } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'

interface Persona {
  name: string
  age: string
  occupation: string
  background: string
  goals?: string[]
  painPoints?: string[]
  behaviors?: string[]
  quote?: string
}

interface UserPersonasDisplayProps {
  isGenerating: boolean
  progress: number
  personas: Persona[]
}

export function UserPersonasDisplay({
  isGenerating,
  progress,
  personas,
}: UserPersonasDisplayProps) {
  const [activeIndex, setActiveIndex] = useState(0)

  if (isGenerating && personas.length === 0) {
    return (
      <div className="flex items-center justify-center h-80 bg-gradient-to-br from-gray-50 to-gray-100 rounded-xl border-2 border-dashed border-gray-200">
        <div className="text-center space-y-4">
          <div className="relative">
            <div className="animate-spin rounded-full h-16 w-16 border-4 border-primary/20 border-t-primary mx-auto" />
            <div className="absolute inset-0 flex items-center justify-center">
              <span className="text-lg font-bold text-primary">{progress}%</span>
            </div>
          </div>
          <div>
            <p className="text-lg font-medium text-gray-700">AI 正在创作用户画像</p>
            <p className="text-sm text-gray-500 mt-1">这需要几秒钟时间...</p>
          </div>
        </div>
      </div>
    )
  }

  if (personas.length === 0) {
    return (
      <div className="flex items-center justify-center h-80 bg-gradient-to-br from-gray-50 to-gray-100 rounded-xl border-2 border-dashed border-gray-200">
        <div className="text-center space-y-4">
          <div className="w-16 h-16 rounded-full bg-gray-200 flex items-center justify-center mx-auto">
            <User className="w-8 h-8 text-gray-400" />
          </div>
          <div>
            <p className="text-lg font-medium text-gray-700">暂无用户画像</p>
            <p className="text-sm text-gray-500 mt-1">点击"重新生成"开始创建</p>
          </div>
        </div>
      </div>
    )
  }

  const activePersona = personas[activeIndex]

  return (
    <div className="space-y-6">
      {/* 画像选择器 */}
      <div className="flex items-center justify-center gap-3 flex-wrap">
        {personas.map((persona, index) => (
          <button
            key={index}
            onClick={() => setActiveIndex(index)}
            className={`px-4 py-2 rounded-full text-sm font-medium transition-all duration-200 ${
              index === activeIndex
                ? 'bg-primary text-white shadow-lg scale-105'
                : 'bg-white text-gray-600 hover:bg-gray-50 border border-gray-200'
            }`}
          >
            {persona.name}
          </button>
        ))}
      </div>

      {/* 画像卡片 */}
      <Card className="overflow-hidden shadow-xl border-0 bg-gradient-to-br from-white to-gray-50">
        <div className="bg-gradient-to-r from-primary/15 via-primary/10 to-primary/5 p-8">
          <div className="flex items-center gap-6">
            <div className="w-20 h-20 rounded-full bg-gradient-to-br from-primary/30 to-primary/10 flex items-center justify-center shadow-lg">
              <User className="w-10 h-10 text-primary" />
            </div>
            <div>
              <h2 className="text-3xl font-bold text-gray-900">{activePersona.name}</h2>
              <p className="text-gray-600 text-lg mt-1">
                {activePersona.age} · {activePersona.occupation}
              </p>
            </div>
          </div>
        </div>

        <CardContent className="p-8 space-y-8">
          <div>
            <h3 className="flex items-center gap-2 font-semibold text-gray-900 mb-3 text-lg">
              <Briefcase className="w-5 h-5 text-primary" />
              背景
            </h3>
            <p className="text-gray-600 leading-relaxed">{activePersona.background}</p>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
            <div>
              <h3 className="flex items-center gap-2 font-semibold text-gray-900 mb-3 text-lg">
                <Target className="w-5 h-5 text-primary" />
                目标
              </h3>
              <ul className="space-y-2">
                {activePersona.goals?.map((goal, index) => (
                  <li key={index} className="flex items-start gap-2 text-gray-600">
                    <span className="text-primary mt-1.5">•</span>
                    <span>{goal}</span>
                  </li>
                )) || <li className="text-gray-400">暂无数据</li>}
              </ul>
            </div>

            <div>
              <h3 className="flex items-center gap-2 font-semibold text-gray-900 mb-3 text-lg">
                <Target className="w-5 h-5 text-primary" />
                痛点
              </h3>
              <ul className="space-y-2">
                {activePersona.painPoints?.map((point, index) => (
                  <li key={index} className="flex items-start gap-2 text-gray-600">
                    <span className="text-primary mt-1.5">•</span>
                    <span>{point}</span>
                  </li>
                )) || <li className="text-gray-400">暂无数据</li>}
              </ul>
            </div>
          </div>

          <div>
            <h3 className="font-semibold text-gray-900 mb-3 text-lg">行为特征</h3>
            <div className="flex flex-wrap gap-2">
              {activePersona.behaviors?.map((behavior, index) => (
                <Badge
                  key={index}
                  variant="secondary"
                  className="px-3 py-1 text-sm bg-gradient-to-r from-gray-100 to-gray-50 hover:from-gray-200 hover:to-gray-100"
                >
                  {behavior}
                </Badge>
              )) || <span className="text-gray-400">暂无数据</span>}
            </div>
          </div>

          {activePersona.quote && (
            <div className="bg-gradient-to-r from-primary/5 to-blue-50 p-6 rounded-xl border-l-4 border-primary">
              <Quote className="w-6 h-6 text-primary/60 mb-3" />
              <p className="italic text-gray-700 text-lg leading-relaxed">
                "{activePersona.quote}"
              </p>
            </div>
          )}
        </CardContent>
      </Card>
    </div>
  )
}
import { useEffect } from 'react'
import { useParams, useNavigate } from 'react-router-dom'
import { ArrowRight, ArrowLeft, RefreshCw } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { UserPersonasDisplay } from './UserPersonasDisplay'
import { useProjectStore, useAppStore, useAIConfigStore } from '@/stores'
import { usePersonaStream } from '@/hooks/usePersonaStream'

export function UserPersonas() {
  const { projectId } = useParams<{ projectId: string }>()
  const navigate = useNavigate()
  const { getProjectById, setProjectPersonas } = useProjectStore()
  const { setLoading } = useAppStore()
  const aiConfigStore = useAIConfigStore()

  const project = projectId ? getProjectById(projectId) : undefined

  // 使用流式生成 Hook
  const { personas, isStreaming, isComplete, error, startStream } = usePersonaStream()

  const hasExistingPersonas = project?.userPersonas && project.userPersonas.length > 0

  // 初始化时加载已有的用户画像或触发生成
  useEffect(() => {
    if (project) {
      if (hasExistingPersonas && project.userPersonas) {
        // 已有数据，直接使用
        console.log('[UserPersonas] Using existing personas:', project.userPersonas.length)
      } else {
        // 无数据，触发流式生成
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
    }
  }, [isComplete, personas, projectId, setProjectPersonas])

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
    </div>
  )
}
