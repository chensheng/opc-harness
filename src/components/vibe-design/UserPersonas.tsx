import { useEffect, useState } from 'react'
import { useParams, useNavigate } from 'react-router-dom'
import { User, Briefcase, Target, Quote, ArrowRight, ArrowLeft, RefreshCw } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card, CardContent } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { useProjectStore, useAppStore } from '@/stores'
import { usePersonaStream } from '@/hooks/usePersonaStream'

export function UserPersonas() {
  const { projectId } = useParams<{ projectId: string }>()
  const navigate = useNavigate()
  const { getProjectById, setProjectPersonas } = useProjectStore()
  const { setLoading } = useAppStore()

  const [activeIndex, setActiveIndex] = useState(0)

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
      await startStream({
        prdId: projectId, // 使用 projectId 作为 prdId
        projectId: projectId,
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

  // 显示加载状态
  if (personas.length === 0 && isStreaming) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="text-center">
          <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary mx-auto mb-4" />
          <p className="text-muted-foreground">AI 正在生成用户画像...</p>
          {error && <p className="text-red-500 mt-2">{error}</p>}
        </div>
      </div>
    )
  }

  // 显示错误状态
  if (error && personas.length === 0) {
    return (
      <div className="max-w-2xl mx-auto space-y-6">
        <div className="text-center py-12">
          <p className="text-red-500 mb-4">{error}</p>
          <Button onClick={triggerStreamGeneration} className="gap-2">
            <RefreshCw className="w-4 h-4" />
            重新生成
          </Button>
        </div>
      </div>
    )
  }

  const activePersona = personas[activeIndex]

  return (
    <div className="max-w-4xl mx-auto space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold">👥 用户画像</h1>
          <p className="text-muted-foreground">{project.name}</p>
          {isStreaming && (
            <p className="text-sm text-blue-500 flex items-center gap-2 mt-1">
              <span className="animate-pulse">✨</span>
              AI 正在生成中...
            </p>
          )}
          {isComplete && (
            <p className="text-sm text-green-600 mt-1">✓ 已生成 {personas.length} 个用户画像</p>
          )}
        </div>
        <div className="flex gap-2">
          {/* 画像选择器 */}
          {personas.map((_, index) => (
            <button
              key={index}
              onClick={() => setActiveIndex(index)}
              className={`w-3 h-3 rounded-full transition-colors ${
                index === activeIndex ? 'bg-primary' : 'bg-muted'
              }`}
            />
          ))}
          {/* 重新生成按钮 */}
          <Button
            variant="outline"
            size="sm"
            onClick={triggerStreamGeneration}
            disabled={isStreaming}
            className="ml-2"
          >
            <RefreshCw className="w-4 h-4" />
          </Button>
        </div>
      </div>

      <Card className="overflow-hidden">
        <div className="bg-gradient-to-r from-primary/10 to-primary/5 p-6">
          <div className="flex items-center gap-4">
            <div className="w-16 h-16 rounded-full bg-primary/20 flex items-center justify-center">
              <User className="w-8 h-8 text-primary" />
            </div>
            <div>
              <h2 className="text-2xl font-bold">{activePersona.name}</h2>
              <p className="text-muted-foreground">
                {activePersona.age} · {activePersona.occupation}
              </p>
            </div>
          </div>
        </div>

        <CardContent className="p-6 space-y-6">
          <div>
            <h3 className="flex items-center gap-2 font-medium mb-2">
              <Briefcase className="w-4 h-4" />
              背景
            </h3>
            <p className="text-muted-foreground">{activePersona.background}</p>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div>
              <h3 className="flex items-center gap-2 font-medium mb-2">
                <Target className="w-4 h-4" />
                目标
              </h3>
              <ul className="space-y-1">
                {activePersona.goals?.map((goal, index) => (
                  <li key={index} className="text-sm text-muted-foreground">
                    • {goal}
                  </li>
                )) || <li className="text-sm text-muted-foreground">暂无数据</li>}
              </ul>
            </div>

            <div>
              <h3 className="flex items-center gap-2 font-medium mb-2">
                <Target className="w-4 h-4" />
                痛点
              </h3>
              <ul className="space-y-1">
                {activePersona.painPoints?.map((point, index) => (
                  <li key={index} className="text-sm text-muted-foreground">
                    • {point}
                  </li>
                )) || <li className="text-sm text-muted-foreground">暂无数据</li>}
              </ul>
            </div>
          </div>

          <div>
            <h3 className="font-medium mb-2">行为特征</h3>
            <div className="flex flex-wrap gap-2">
              {activePersona.behaviors?.map((behavior, index) => (
                <Badge key={index} variant="secondary">
                  {behavior}
                </Badge>
              )) || <span className="text-sm text-muted-foreground">暂无数据</span>}
            </div>
          </div>

          {activePersona.quote && (
            <div className="bg-muted p-4 rounded-lg">
              <Quote className="w-5 h-5 text-muted-foreground mb-2" />
              <p className="italic text-muted-foreground">"{activePersona.quote}"</p>
            </div>
          )}
        </CardContent>
      </Card>

      <div className="flex justify-between">
        <Button variant="outline" onClick={() => navigate(`/prd/${projectId}`)}>
          <ArrowLeft className="w-4 h-4 mr-2" />
          返回 PRD
        </Button>
        <Button onClick={() => navigate(`/competitors/${projectId}`)}>
          查看竞品分析
          <ArrowRight className="w-4 h-4 ml-2" />
        </Button>
      </div>
    </div>
  )
}
