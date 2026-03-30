import { Card, CardContent, CardHeader } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { TrendingUp, TrendingDown, Minus } from 'lucide-react'
import type { Competitor, MetricKey } from '@/types'

interface MetricCardProps {
  competitor: Competitor
  metrics: MetricKey[]
}

const METRIC_LABELS: Record<MetricKey, string> = {
  marketShare: '市场份额',
  revenue: '收入',
  userGrowth: '用户增长',
  employeeCount: '员工数量',
  funding: '融资',
  customerSatisfaction: '客户满意度',
  innovationScore: '创新能力',
}

const formatValue = (value: number | string | null | undefined, metric: MetricKey): string => {
  if (value === undefined || value === null) return 'N/A'

  switch (metric) {
    case 'marketShare':
      return typeof value === 'string' ? value : `${value}%`
    case 'employeeCount':
      return value.toString()
    case 'customerSatisfaction':
    case 'innovationScore':
      return typeof value === 'number' ? `${value}/100` : 'N/A'
    default:
      return String(value)
  }
}

const getTrendIcon = (value: number | string, metric: MetricKey) => {
  if (typeof value !== 'number') return <Minus className="h-4 w-4 text-muted-foreground" />

  const thresholds: Record<MetricKey, number> = {
    marketShare: 30,
    revenue: 50,
    userGrowth: 20,
    employeeCount: 500,
    funding: 100,
    customerSatisfaction: 80,
    innovationScore: 70,
  }

  const threshold = thresholds[metric]
  if (value >= threshold * 1.2) {
    return <TrendingUp className="h-4 w-4 text-green-500" />
  } else if (value <= threshold * 0.8) {
    return <TrendingDown className="h-4 w-4 text-red-500" />
  }
  return <Minus className="h-4 w-4 text-muted-foreground" />
}

export function MetricCard({ competitor, metrics }: MetricCardProps) {
  return (
    <Card className="hover:shadow-lg transition-shadow">
      <CardHeader className="pb-3">
        <div className="flex items-center justify-between">
          <h3 className="font-semibold text-lg">{competitor.name}</h3>
          {competitor.marketShare && <Badge variant="secondary">{competitor.marketShare}</Badge>}
        </div>
      </CardHeader>
      <CardContent className="space-y-3">
        {metrics.map(metric => {
          const value = competitor[metric] as number | string | null | undefined
          const displayValue = formatValue(value, metric)

          return (
            <div key={metric} className="flex items-center justify-between">
              <span className="text-sm text-muted-foreground">{METRIC_LABELS[metric]}</span>
              <div className="flex items-center gap-2">
                {typeof value === 'number' && getTrendIcon(value, metric)}
                <span className="font-medium text-sm">{displayValue}</span>
              </div>
            </div>
          )
        })}

        {/* 强弱项 */}
        {(competitor.strengths.length > 0 || competitor.weaknesses.length > 0) && (
          <div className="pt-3 border-t space-y-2">
            {competitor.strengths.length > 0 && (
              <div className="text-xs">
                <span className="text-green-600 font-medium">优势:</span>
                <span className="text-muted-foreground ml-1">
                  {competitor.strengths.slice(0, 2).join(', ')}
                </span>
              </div>
            )}
            {competitor.weaknesses.length > 0 && (
              <div className="text-xs">
                <span className="text-red-600 font-medium">劣势:</span>
                <span className="text-muted-foreground ml-1">
                  {competitor.weaknesses.slice(0, 2).join(', ')}
                </span>
              </div>
            )}
          </div>
        )}
      </CardContent>
    </Card>
  )
}
