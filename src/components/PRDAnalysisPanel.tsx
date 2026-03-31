import React from 'react'
import { usePRDAnalysis } from '../hooks/usePRDAnalysis'
import { Card, CardContent, CardHeader, CardTitle } from './ui/card'
import { Badge } from './ui/badge'
import { Progress } from './ui/progress'
import { Button } from './ui/button'
import type { Feature, Risk, Estimates } from '../types'

interface PRDAnalysisPanelProps {
  /** PRD 内容 */
  prdContent: string
  /** API Key（可选） */
  apiKey?: string
}

/**
 * PRD 深度分析面板组件
 */
export function PRDAnalysisPanel({ prdContent, apiKey }: PRDAnalysisPanelProps) {
  const { analysis, loading, error, analyze, reset } = usePRDAnalysis()

  // 自动执行分析
  React.useEffect(() => {
    if (prdContent && !analysis) {
      analyze(prdContent, apiKey)
    }
  }, [prdContent, apiKey])

  if (loading) {
    return (
      <div className="w-full h-64 flex items-center justify-center">
        <div className="text-center">
          <div className="animate-spin mb-4">🔄</div>
          <p className="text-muted-foreground">正在深度分析 PRD...</p>
        </div>
      </div>
    )
  }

  if (error) {
    return (
      <Card className="border-red-500">
        <CardHeader>
          <CardTitle className="text-red-500">❌ 分析失败</CardTitle>
        </CardHeader>
        <CardContent>
          <p className="text-destructive">{error}</p>
          <Button onClick={() => analyze(prdContent, apiKey)} className="mt-4">
            重试
          </Button>
        </CardContent>
      </Card>
    )
  }

  if (!analysis) {
    return (
      <div className="w-full h-64 flex items-center justify-center">
        <Button onClick={() => analyze(prdContent, apiKey)}>开始分析</Button>
      </div>
    )
  }

  return (
    <div className="space-y-6">
      {/* 统计概览 */}
      <Card>
        <CardHeader>
          <CardTitle>📊 分析概览</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
            <StatCard title="总功能数" value={analysis.estimates.totalFeatures} icon="🎯" />
            <StatCard title="核心功能" value={analysis.estimates.coreFeatures} icon="⭐" />
            <StatCard
              title="平均复杂度"
              value={analysis.estimates.averageComplexity.toFixed(1)}
              icon="📈"
            />
            <StatCard
              title="预估工时"
              value={`${analysis.estimates.totalEstimatedHours.toFixed(1)}h`}
              icon="⏱️"
            />
          </div>
        </CardContent>
      </Card>

      {/* 功能列表 */}
      <Card>
        <CardHeader>
          <CardTitle>🔧 功能清单 ({analysis.features.length})</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="space-y-2 max-h-96 overflow-y-auto">
            {analysis.features.map(feature => (
              <FeatureItem key={feature.id} feature={feature} />
            ))}
          </div>
        </CardContent>
      </Card>

      {/* 风险评估 */}
      {analysis.risks.length > 0 && (
        <Card>
          <CardHeader>
            <CardTitle>⚠️ 风险评估 ({analysis.risks.length})</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="space-y-2">
              {analysis.risks.map(risk => (
                <RiskItem key={risk.id} risk={risk} />
              ))}
            </div>
          </CardContent>
        </Card>
      )}

      {/* 依赖关系 */}
      {analysis.dependencies.length > 0 && (
        <Card>
          <CardHeader>
            <CardTitle>🔗 依赖关系 ({analysis.dependencies.length})</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="space-y-2">
              {analysis.dependencies.map((dep, index) => (
                <DependencyItem key={index} dependency={dep} />
              ))}
            </div>
          </CardContent>
        </Card>
      )}

      {/* 操作按钮 */}
      <div className="flex justify-end gap-2">
        <Button variant="outline" onClick={reset}>
          重置
        </Button>
        <Button onClick={() => analyze(prdContent, apiKey)}>重新分析</Button>
      </div>
    </div>
  )
}

/**
 * 统计卡片组件
 */
function StatCard({ title, value, icon }: { title: string; value: number | string; icon: string }) {
  return (
    <div className="bg-muted/50 rounded-lg p-4 text-center">
      <div className="text-2xl mb-2">{icon}</div>
      <div className="text-2xl font-bold">{value}</div>
      <div className="text-sm text-muted-foreground">{title}</div>
    </div>
  )
}

/**
 * 功能项组件
 */
function FeatureItem({ feature }: { feature: Feature }) {
  const getTypeColor = (type: string) => {
    switch (type) {
      case 'core':
        return 'bg-red-500'
      case 'auxiliary':
        return 'bg-blue-500'
      case 'enhanced':
        return 'bg-green-500'
      default:
        return 'bg-gray-500'
    }
  }

  const getTypeLabel = (type: string) => {
    switch (type) {
      case 'core':
        return '核心'
      case 'auxiliary':
        return '辅助'
      case 'enhanced':
        return '增强'
      default:
        return '未知'
    }
  }

  return (
    <div className="border rounded-lg p-3 hover:bg-muted/50">
      <div className="flex items-start justify-between mb-2">
        <div>
          <div className="font-semibold">{feature.name}</div>
          <div className="text-sm text-muted-foreground">{feature.description}</div>
        </div>
        <div className="flex gap-2">
          <Badge variant="secondary" className={getTypeColor(feature.featureType)}>
            {getTypeLabel(feature.featureType)}
          </Badge>
          <Badge variant="outline">复杂度：{feature.complexity}</Badge>
        </div>
      </div>
      <div className="flex gap-4 text-sm">
        <span>优先级：{feature.priority}</span>
        <span>预估：{feature.estimatedHours}h</span>
        {feature.dependencies.length > 0 && <span>依赖：{feature.dependencies.join(', ')}</span>}
      </div>
    </div>
  )
}

/**
 * 风险项组件
 */
function RiskItem({ risk }: { risk: Risk }) {
  const getLevelColor = (level: string) => {
    switch (level) {
      case 'low':
        return 'bg-green-500'
      case 'medium':
        return 'bg-yellow-500'
      case 'high':
        return 'bg-orange-500'
      case 'critical':
        return 'bg-red-500'
      default:
        return 'bg-gray-500'
    }
  }

  const getLevelLabel = (level: string) => {
    switch (level) {
      case 'low':
        return '低'
      case 'medium':
        return '中'
      case 'high':
        return '高'
      case 'critical':
        return '严重'
      default:
        return '未知'
    }
  }

  return (
    <div className="border rounded-lg p-3">
      <div className="flex items-start justify-between mb-2">
        <div className="font-semibold">{risk.description}</div>
        <Badge variant="secondary" className={getLevelColor(risk.level)}>
          {getLevelLabel(risk.level)}风险
        </Badge>
      </div>
      <div className="text-sm text-muted-foreground">影响：{risk.impact}</div>
      {risk.mitigation && (
        <div className="text-sm text-muted-foreground mt-1">缓解措施：{risk.mitigation}</div>
      )}
    </div>
  )
}

/**
 * 依赖项组件
 */
function DependencyItem({ dependency }: { dependency: any }) {
  return (
    <div className="border rounded-lg p-3">
      <div className="flex items-center gap-2">
        <Badge variant="outline">{dependency.fromFeature}</Badge>
        <span>→</span>
        <Badge variant="outline">{dependency.toFeature}</Badge>
        <span className="text-sm text-muted-foreground">({dependency.dependencyType})</span>
      </div>
    </div>
  )
}
