import { useEffect, useState } from 'react'
import { useParams, useNavigate } from 'react-router-dom'
import { ArrowRight, ArrowLeft, TrendingUp, Check, X, Lightbulb } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { useProjectStore, useAppStore } from '@/stores'
import type { CompetitorAnalysis } from '@/types'

// Simulated AI-generated competitor analysis
function generateMockCompetitorAnalysis(): CompetitorAnalysis {
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
    differentiation: '我们的产品专注于为独立创造者提供一站式解决方案，整合产品构思、快速构建和增长运营三大模块，这是现有竞品所不具备的核心优势。',
    opportunities: [
      '一人公司/独立创造者市场快速增长',
      'AI工具普及降低了技术门槛',
      '远程工作趋势推动副业经济发展',
      '用户对All-in-one解决方案的需求增加',
    ],
  }
}

export function CompetitorAnalysis() {
  const { projectId } = useParams<{ projectId: string }>()
  const navigate = useNavigate()
  const { getProjectById, setProjectCompetitorAnalysis } = useProjectStore()
  const { setLoading } = useAppStore()
  
  const [analysis, setAnalysis] = useState<CompetitorAnalysis | null>(null)

  const project = projectId ? getProjectById(projectId) : undefined

  useEffect(() => {
    if (project) {
      if (project.competitorAnalysis) {
        setAnalysis(project.competitorAnalysis)
      } else {
        generateAnalysis()
      }
    }
  }, [project])

  const generateAnalysis = async () => {
    setLoading(true, 'AI正在分析竞品...')
    
    try {
      await new Promise(resolve => setTimeout(resolve, 2000))
      
      const generatedAnalysis = generateMockCompetitorAnalysis()
      setAnalysis(generatedAnalysis)
      
      if (projectId) {
        setProjectCompetitorAnalysis(projectId, generatedAnalysis)
      }
    } finally {
      setLoading(false)
    }
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

  if (!analysis) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="text-center">
          <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary mx-auto" />
          <p className="mt-4 text-muted-foreground">正在分析竞品...</p>
        </div>
      </div>
    )
  }

  return (
    <div className="max-w-4xl mx-auto space-y-6">
      <div>
        <h1 className="text-2xl font-bold">🔍 竞品分析</h1>
        <p className="text-muted-foreground">{project.name}</p>
      </div>

      {/* Competitors */}
      <div className="space-y-4">
        <h2 className="text-lg font-medium">主要竞品</h2>
        {analysis.competitors.map((competitor, index) => (
          <Card key={index}>
            <CardHeader className="pb-3">
              <div className="flex items-center justify-between">
                <CardTitle>{competitor.name}</CardTitle>
                {competitor.marketShare && (
                  <Badge variant="secondary">
                    <TrendingUp className="w-3 h-3 mr-1" />
                    市场份额: {competitor.marketShare}
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
                      <li key={i} className="text-sm text-muted-foreground">
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
                      <li key={i} className="text-sm text-muted-foreground">
                        • {weakness}
                      </li>
                    ))}
                  </ul>
                </div>
              </div>
            </CardContent>
          </Card>
        ))}
      </div>

      {/* Differentiation */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Lightbulb className="w-5 h-5" />
            差异化优势
          </CardTitle>
        </CardHeader>
        <CardContent>
          <p className="text-muted-foreground leading-relaxed">
            {analysis.differentiation}
          </p>
        </CardContent>
      </Card>

      {/* Opportunities */}
      <Card>
        <CardHeader>
          <CardTitle>市场机会</CardTitle>
        </CardHeader>
        <CardContent>
          <ul className="space-y-2">
            {analysis.opportunities.map((opportunity, index) => (
              <li key={index} className="flex items-start gap-2">
                <span className="text-primary mt-1">•</span>
                <span className="text-muted-foreground">{opportunity}</span>
              </li>
            ))}
          </ul>
        </CardContent>
      </Card>

      <div className="flex justify-between">
        <Button variant="outline" onClick={() => navigate(`/personas/${projectId}`)}>
          <ArrowLeft className="w-4 h-4 mr-2" />
          返回用户画像
        </Button>
        <Button onClick={() => navigate(`/coding/${projectId}`)}>
          开始开发
          <ArrowRight className="w-4 h-4 ml-2" />
        </Button>
      </div>
    </div>
  )
}
