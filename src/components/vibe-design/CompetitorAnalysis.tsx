import { useEffect, useState, useRef } from 'react'
import { useParams, useNavigate } from 'react-router-dom'
import {
  ArrowRight,
  ArrowLeft,
  TrendingUp,
  Check,
  X,
  Lightbulb,
  Sparkles,
  BarChart3,
  Calendar,
} from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { useProjectStore, useAppStore } from '@/stores'
import { useCompetitorStream } from '@/hooks/useCompetitorStream'
import { CompetitorRadarChart } from '@/components/CompetitorRadarChart'
import { CompetitorTimeline } from '@/components/CompetitorTimeline'
import { InteractiveDataExplorer } from '@/components/InteractiveDataExplorer'
import type { CompetitorAnalysis as CompetitorAnalysisType } from '@/types'

// Simulated AI-generated competitor analysis (fallback)
function generateMockCompetitorAnalysis(): CompetitorAnalysisType {
  return {
    competitors: [
      {
        name: 'Competitor A',
        strengths: ['品牌知名度高', '功能完善', '用户基础大'],
        weaknesses: ['价格较高', '学习曲线陡峭', '定制化能力有限'],
        marketShare: '35%',
      },
      {
        name: 'Competitor B',
        strengths: ['免费版本功能丰富', '社区活跃', '开源'],
        weaknesses: ['界面不够美观', '技术支持有限', '性能问题'],
        marketShare: '25%',
      },
      {
        name: 'Competitor C',
        strengths: ['专注垂直领域', '客户支持优秀', '集成能力强'],
        weaknesses: ['适用范围窄', '价格不透明', '更新频率低'],
        marketShare: '15%',
      },
    ],
    differentiation:
      '我们的产品专注于为独立创造者提供一站式解决方案，整合产品构思、快速构建和增长运营三大模块，这是现有竞品所不具备的核心优势。',
    opportunities: [
      '一人公司/独立创造者市场快速增长',
      'AI 工具普及降低了技术门槛',
      '远程工作趋势推动副业经济发展',
      '用户对 All-in-one 解决方案的需求增加',
    ],
  }
}

export function CompetitorAnalysis() {
  const { projectId } = useParams<{ projectId: string }>()
  const navigate = useNavigate()
  const { getProjectById, setProjectCompetitorAnalysis, syncProjectToDatabase } = useProjectStore()
  const { setLoading } = useAppStore()

  const [_useFallback, setUseFallback] = useState(false)
  const [showRadar, setShowRadar] = useState(false)
  const [showTimeline, setShowTimeline] = useState(false)
  const [showExplorer, setShowExplorer] = useState(false)

  // 防止重复启动流式生成的标志
  const hasStartedGenerationRef = useRef(false)

  // 使用流式 Hook
  const {
    analysis,
    isStreaming,
    isComplete,
    error,
    sessionId: _sessionId,
    startStream,
    reset,
  } = useCompetitorStream()

  const project = projectId ? getProjectById(projectId) : undefined

  useEffect(() => {
    if (project && !hasStartedGenerationRef.current) {
      if (project.competitorAnalysis && !isStreaming) {
        // 已有缓存数据，直接使用
        reset()
      } else if (!isStreaming && !error) {
        // 启动流式生成
        hasStartedGenerationRef.current = true
        startStreamWithIdea(project)
      }
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [project])

  const startStreamWithIdea = async (proj: {
    prd?: { overview?: string }
    description?: string
  }) => {
    try {
      // 从项目 idea 生成竞品分析
      const idea = proj.prd?.overview || proj.description || '创建一个创新的产品'

      await startStream({
        idea,
        provider: 'openai',
        model: 'gpt-4o-mini',
        apiKey: '', // TODO: 从配置中获取
      })
    } catch (err) {
      console.error('[CompetitorAnalysis] Error starting stream:', err)
      // 降级到 mock 数据
      setUseFallback(true)
      generateAnalysis()
    }
  }

  const generateAnalysis = async () => {
    setLoading(true, '正在分析竞品...')

    try {
      await new Promise(resolve => setTimeout(resolve, 2000))

      const generatedAnalysis = generateMockCompetitorAnalysis()

      if (projectId) {
        setProjectCompetitorAnalysis(projectId, generatedAnalysis)
      }
    } finally {
      setLoading(false)
    }
  }

  // 保存到 store 当分析完成时
  useEffect(() => {
    if (isComplete && analysis && projectId) {
      setProjectCompetitorAnalysis(projectId, analysis)

      // 同步到数据库
      syncProjectToDatabase(projectId).catch(err => {
        console.error('[CompetitorAnalysis] Failed to sync competitor analysis to database:', err)
      })
    }
  }, [isComplete, analysis, projectId, setProjectCompetitorAnalysis, syncProjectToDatabase])

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
  if (!analysis && isStreaming) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="text-center">
          <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary mx-auto" />
          <p className="mt-4 text-muted-foreground flex items-center gap-2">
            <Sparkles className="w-4 h-4 animate-pulse" />
            AI 正在实时分析竞品...
          </p>
        </div>
      </div>
    )
  }

  // 错误状态
  if (error && !analysis) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="text-center">
          <p className="text-destructive mb-4">生成失败：{error}</p>
          <Button onClick={() => setUseFallback(true)} variant="outline">
            使用示例数据
          </Button>
        </div>
      </div>
    )
  }

  // 没有数据
  if (!analysis) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="text-center">
          <p className="text-muted-foreground">暂无竞品分析数据</p>
          <Button onClick={generateAnalysis} className="mt-4">
            生成示例数据
          </Button>
        </div>
      </div>
    )
  }

  return (
    <div className="max-w-6xl mx-auto space-y-6">
      <div>
        <h1 className="text-2xl font-bold flex items-center gap-2">
          🔍 竞品分析
          {isStreaming && (
            <Badge variant="secondary" className="animate-pulse">
              <Sparkles className="w-3 h-3 mr-1" />
              实时更新中...
            </Badge>
          )}
          {isComplete && (
            <Badge variant="default" className="bg-green-600">
              <Check className="w-3 h-3 mr-1" />
              完成
            </Badge>
          )}
        </h1>
        <p className="text-muted-foreground">{project.name}</p>
      </div>

      {/* 可视化切换按钮组 */}
      <div className="flex justify-end gap-2 flex-wrap">
        <Button variant="outline" onClick={() => setShowRadar(!showRadar)} className="gap-2">
          <BarChart3 className="w-4 h-4" />
          {showRadar ? '隐藏对比图' : '显示对比图'}
        </Button>
        <Button variant="outline" onClick={() => setShowTimeline(!showTimeline)} className="gap-2">
          <Calendar className="w-4 h-4" />
          {showTimeline ? '隐藏时间线' : '显示时间线'}
        </Button>
        <Button variant="outline" onClick={() => setShowExplorer(!showExplorer)} className="gap-2">
          <BarChart3 className="w-4 h-4" />
          {showExplorer ? '隐藏数据探索' : '数据探索'}
        </Button>
      </div>

      {/* 雷达图 */}
      {showRadar && (
        <div className="animate-in fade-in slide-in-from-bottom-4">
          <CompetitorRadarChart analysis={analysis} productName={project.name || '本产品'} />
        </div>
      )}

      {/* 时间线 */}
      {showTimeline && (
        <div className="animate-in fade-in slide-in-from-bottom-4">
          <CompetitorTimeline analysis={analysis} />
        </div>
      )}

      {/* 交互式数据探索器 */}
      {showExplorer && (
        <div className="animate-in fade-in slide-in-from-bottom-4">
          <InteractiveDataExplorer competitors={analysis.competitors} />
        </div>
      )}

      {/* Competitors - 渐进式渲染 */}
      <div className="space-y-4">
        <h2 className="text-lg font-medium">主要竞品</h2>
        {analysis.competitors.length > 0 ? (
          analysis.competitors.map((competitor, index) => (
            <Card
              key={index}
              className={isStreaming ? 'animate-in fade-in slide-in-from-bottom-4' : ''}
              style={{ animationDelay: `${index * 100}ms` }}
            >
              <CardHeader className="pb-3">
                <div className="flex items-center justify-between">
                  <CardTitle>{competitor.name}</CardTitle>
                  {competitor.marketShare && (
                    <Badge variant="secondary">
                      <TrendingUp className="w-3 h-3 mr-1" />
                      市场份额：{competitor.marketShare}
                    </Badge>
                  )}
                </div>
              </CardHeader>
              <CardContent>
                <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                  <div>
                    <h4 className="flex items-center gap-2 text-sm font-medium text-green-600 mb-2">
                      <Check className="w-4 h-4" />
                      优势
                    </h4>
                    <ul className="space-y-1">
                      {competitor.strengths.map((strength, i) => (
                        <li
                          key={i}
                          className="text-sm text-muted-foreground animate-in fade-in"
                          style={{ animationDelay: `${i * 50}ms` }}
                        >
                          • {strength}
                        </li>
                      ))}
                    </ul>
                  </div>
                  <div>
                    <h4 className="flex items-center gap-2 text-sm font-medium text-red-600 mb-2">
                      <X className="w-4 h-4" />
                      劣势
                    </h4>
                    <ul className="space-y-1">
                      {competitor.weaknesses.map((weakness, i) => (
                        <li
                          key={i}
                          className="text-sm text-muted-foreground animate-in fade-in"
                          style={{ animationDelay: `${i * 50}ms` }}
                        >
                          • {weakness}
                        </li>
                      ))}
                    </ul>
                  </div>
                </div>
              </CardContent>
            </Card>
          ))
        ) : (
          <p className="text-muted-foreground text-center py-8">正在分析竞品...</p>
        )}
      </div>

      {/* Differentiation - 打字机效果 */}
      <Card className={isStreaming ? 'animate-in fade-in' : ''}>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Lightbulb className="w-5 h-5" />
            差异化优势
          </CardTitle>
        </CardHeader>
        <CardContent>
          {analysis.differentiation ? (
            <p className="text-muted-foreground leading-relaxed whitespace-pre-wrap animate-in fade-in">
              {analysis.differentiation}
            </p>
          ) : (
            <p className="text-muted-foreground">正在生成差异化分析...</p>
          )}
        </CardContent>
      </Card>

      {/* Opportunities - 渐进式列表 */}
      <Card className={isStreaming ? 'animate-in fade-in' : ''}>
        <CardHeader>
          <CardTitle>市场机会</CardTitle>
        </CardHeader>
        <CardContent>
          {analysis.opportunities.length > 0 ? (
            <ul className="space-y-2">
              {analysis.opportunities.map((opportunity, index) => (
                <li
                  key={index}
                  className="flex items-start gap-2 animate-in fade-in slide-in-from-left-2"
                  style={{ animationDelay: `${index * 100}ms` }}
                >
                  <span className="text-primary mt-1">•</span>
                  <span className="text-muted-foreground">{opportunity}</span>
                </li>
              ))}
            </ul>
          ) : (
            <p className="text-muted-foreground">正在识别市场机会...</p>
          )}
        </CardContent>
      </Card>

      <div className="flex justify-between">
        <Button
          variant="outline"
          onClick={() => navigate(`/personas/${projectId}`)}
          disabled={isStreaming}
        >
          <ArrowLeft className="w-4 h-4 mr-2" />
          返回用户画像
        </Button>
        <Button onClick={() => navigate(`/coding/${projectId}`)} disabled={isStreaming}>
          开始开发
          <ArrowRight className="w-4 h-4 ml-2" />
        </Button>
      </div>
    </div>
  )
}
