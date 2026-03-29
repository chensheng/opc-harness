import { useEffect, useState } from 'react'
import { useParams, useNavigate } from 'react-router-dom'
import { ArrowRight, Users, Target, Zap, Clock, Download, Edit, Sparkles } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { useProjectStore, useAppStore } from '@/stores'
import { downloadFile } from '@/lib/utils'
import type { PRD } from '@/types'
import { usePRDStream } from '@/hooks/usePRDStream'
import { useAIConfigStore } from '@/stores/aiConfigStore'

// Simulated AI-generated PRD (fallback)
function generateMockPRD(idea: string): PRD {
  return {
    title: idea.slice(0, 30) + (idea.length > 30 ? '...' : ''),
    overview: `这是一个基于用户想法「${idea.slice(0, 50)}...」的产品。该产品旨在解决目标用户的核心痛点，提供简洁高效的解决方案。`,
    targetUsers: ['独立开发者', '自由职业者', '技术型创业者', '小型团队负责人'],
    coreFeatures: [
      '直观的用户界面，降低学习成本',
      '核心功能模块化，按需使用',
      '数据同步和备份机制',
      '多平台支持（Web、移动端）',
      'API 接口开放，支持第三方集成',
    ],
    techStack: ['React', 'Node.js', 'PostgreSQL', 'Redis', 'Docker'],
    estimatedEffort: '2-4 周',
    businessModel: 'Freemium 模式，基础功能免费，高级功能订阅制',
    pricing: '免费版：基础功能；Pro 版：$9/月；Team 版：$29/月',
  }
}

export function PRDDisplay() {
  const { projectId } = useParams<{ projectId: string }>()
  const navigate = useNavigate()
  const { getProjectById, setProjectPRD, updateProjectStatus, updateProjectProgress } =
    useProjectStore()
  const { setLoading } = useAppStore()
  const aiConfigStore = useAIConfigStore()

  const [prd, setPrd] = useState<PRD | null>(null)
  const [isEditing, setIsEditing] = useState(false)

  // 使用 PRD 流式生成 Hook
  const {
    prd: streamingPRD,
    markdownContent,
    isStreaming,
    isComplete,
    error,
    sessionId: _sessionId,
    startStream,
    stopStream,
    reset,
  } = usePRDStream()

  const project = projectId ? getProjectById(projectId) : undefined

  useEffect(() => {
    if (project) {
      if (project.prd) {
        setPrd(project.prd)
      } else {
        // Generate PRD if not exists
        generatePRD()
      }
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [project])

  // 当流式 PRD 更新时，同步到本地状态
  useEffect(() => {
    if (streamingPRD && isStreaming) {
      setPrd(streamingPRD)
    }
  }, [streamingPRD, isStreaming])

  const generatePRD = async () => {
    if (!project) return

    const activeConfig = aiConfigStore.getActiveConfig()

    if (activeConfig?.apiKey) {
      setLoading(true, 'AI 正在生成产品需求文档...')

      // 使用流式生成
      reset()
      await startStream({
        idea: project.idea || project.description,
        provider: activeConfig.provider,
        model: activeConfig.model,
        apiKey: activeConfig.apiKey,
      })
    } else {
      // 降级到模拟生成
      setLoading(true, '正在生成产品需求文档...')
      try {
        await new Promise(resolve => setTimeout(resolve, 2000))
        const generatedPRD = generateMockPRD(project.idea || project.description)
        setPrd(generatedPRD)

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
    }
  }, [
    isComplete,
    streamingPRD,
    projectId,
    setProjectPRD,
    updateProjectStatus,
    updateProjectProgress,
    setLoading,
  ])

  const handleExport = () => {
    if (!prd && !markdownContent) return

    // 优先导出 Markdown 原文
    const content =
      markdownContent ||
      `# ${prd?.title}\n\n## 产品概述\n\n${prd?.overview}\n\n## 目标用户\n\n${prd?.targetUsers.map(u => `- ${u}`).join('\n')}\n\n## 核心功能\n\n${prd?.coreFeatures.map(f => `- ${f}`).join('\n')}\n\n## 技术栈\n\n${prd?.techStack.map(t => `- ${t}`).join('\n')}\n\n## 预估工作量\n\n${prd?.estimatedEffort}\n\n## 商业模式\n\n${prd?.businessModel || '待定'}\n\n## 定价策略\n\n${prd?.pricing || '待定'}\n`

    downloadFile(content, `${prd?.title || 'PRD'}-PRD.md`, 'text/markdown')
  }

  const handleStopGeneration = () => {
    stopStream()
    setLoading(false)
  }

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

  // 流式生成中的 UI
  if (isStreaming) {
    return (
      <div className="max-w-4xl mx-auto space-y-6">
        <div className="flex items-center justify-between">
          <div>
            <h1 className="text-2xl font-bold flex items-center gap-2">
              <Sparkles className="w-6 h-6 text-primary animate-pulse" />
              AI 正在创作 PRD...
            </h1>
            <p className="text-muted-foreground">{project.name}</p>
          </div>
          <Button variant="destructive" onClick={handleStopGeneration}>
            停止生成
          </Button>
        </div>

        {/* 实时内容预览 */}
        <Card>
          <CardHeader>
            <CardTitle>实时生成预览</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="prose prose-sm max-w-none">
              {markdownContent ? (
                <div className="whitespace-pre-wrap font-mono text-sm leading-relaxed">
                  {markdownContent}
                  <span className="inline-block w-2 h-4 bg-primary ml-1 animate-pulse" />
                </div>
              ) : (
                <div className="flex items-center justify-center py-12">
                  <div className="text-center">
                    <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary mx-auto" />
                    <p className="mt-4 text-muted-foreground">正在连接 AI...</p>
                  </div>
                </div>
              )}
            </div>
          </CardContent>
        </Card>

        {/* 进度提示 */}
        <Card>
          <CardContent className="py-6">
            <div className="space-y-2">
              <div className="flex items-center justify-between text-sm">
                <span className="text-muted-foreground">生成进度</span>
                <span className="text-muted-foreground">{markdownContent.length} 字符</span>
              </div>
              <div className="h-2 bg-accent rounded-full overflow-hidden">
                <div
                  className="h-full bg-primary transition-all duration-300"
                  style={{
                    width: `${Math.min((markdownContent.length / 2000) * 100, 100)}%`,
                  }}
                />
              </div>
            </div>
          </CardContent>
        </Card>

        {error && (
          <Card className="border-destructive">
            <CardContent className="py-6">
              <p className="text-destructive">{error}</p>
              <Button onClick={() => generatePRD()} className="mt-4" variant="outline">
                重试
              </Button>
            </CardContent>
          </Card>
        )}
      </div>
    )
  }

  // 没有 PRD 时的初始状态
  if (!prd) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="text-center">
          <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary mx-auto" />
          <p className="mt-4 text-muted-foreground">正在生成 PRD...</p>
        </div>
      </div>
    )
  }

  // PRD 已生成的正常显示
  return (
    <div className="max-w-4xl mx-auto space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold">📋 产品需求文档</h1>
          <p className="text-muted-foreground">{project.name}</p>
        </div>
        <div className="flex gap-2">
          <Button variant="outline" onClick={handleExport}>
            <Download className="w-4 h-4 mr-2" />
            导出
          </Button>
          <Button variant="outline" onClick={() => setIsEditing(!isEditing)}>
            <Edit className="w-4 h-4 mr-2" />
            编辑
          </Button>
        </div>
      </div>

      <Tabs defaultValue="overview" className="w-full">
        <TabsList className="grid w-full grid-cols-4">
          <TabsTrigger value="overview">概述</TabsTrigger>
          <TabsTrigger value="features">功能</TabsTrigger>
          <TabsTrigger value="tech">技术</TabsTrigger>
          <TabsTrigger value="business">商业</TabsTrigger>
        </TabsList>

        <TabsContent value="overview" className="space-y-4">
          <Card>
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Target className="w-5 h-5" />
                产品概述
              </CardTitle>
            </CardHeader>
            <CardContent>
              <p className="text-muted-foreground leading-relaxed">{prd.overview}</p>
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Users className="w-5 h-5" />
                目标用户
              </CardTitle>
            </CardHeader>
            <CardContent>
              <div className="flex flex-wrap gap-2">
                {prd.targetUsers.map((user, index) => (
                  <Badge key={index} variant="secondary">
                    {user}
                  </Badge>
                ))}
              </div>
            </CardContent>
          </Card>
        </TabsContent>

        <TabsContent value="features" className="space-y-4">
          <Card>
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Zap className="w-5 h-5" />
                核心功能
              </CardTitle>
            </CardHeader>
            <CardContent>
              <ul className="space-y-2">
                {prd.coreFeatures.map((feature, index) => (
                  <li key={index} className="flex items-start gap-2">
                    <span className="text-primary mt-1">•</span>
                    <span>{feature}</span>
                  </li>
                ))}
              </ul>
            </CardContent>
          </Card>
        </TabsContent>

        <TabsContent value="tech" className="space-y-4">
          <Card>
            <CardHeader>
              <CardTitle>技术栈</CardTitle>
            </CardHeader>
            <CardContent>
              <div className="flex flex-wrap gap-2">
                {prd.techStack.map((tech, index) => (
                  <Badge key={index} variant="outline">
                    {tech}
                  </Badge>
                ))}
              </div>
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Clock className="w-5 h-5" />
                预估工作量
              </CardTitle>
            </CardHeader>
            <CardContent>
              <p className="text-lg font-medium">{prd.estimatedEffort}</p>
            </CardContent>
          </Card>
        </TabsContent>

        <TabsContent value="business" className="space-y-4">
          <Card>
            <CardHeader>
              <CardTitle>商业模式</CardTitle>
            </CardHeader>
            <CardContent>
              <p className="text-muted-foreground">{prd.businessModel}</p>
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle>定价策略</CardTitle>
            </CardHeader>
            <CardContent>
              <p className="text-muted-foreground">{prd.pricing}</p>
            </CardContent>
          </Card>
        </TabsContent>
      </Tabs>

      <div className="flex justify-end gap-4">
        <Button variant="outline" onClick={() => navigate(`/personas/${projectId}`)}>
          查看用户画像
          <ArrowRight className="w-4 h-4 ml-2" />
        </Button>
        <Button onClick={() => navigate(`/coding/${projectId}`)}>
          开始开发
          <ArrowRight className="w-4 h-4 ml-2" />
        </Button>
      </div>
    </div>
  )
}
