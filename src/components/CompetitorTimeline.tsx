import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import {
  Building2,
  Rocket,
  DollarSign,
  Target,
  Handshake,
  Award,
  TrendingUp,
  Calendar,
} from 'lucide-react'
import type { CompetitorAnalysis as CompetitorAnalysisType } from '@/types'

/**
 * 时间线事件类型
 */
export type TimelineEventType =
  | 'founding' // 创立
  | 'product' // 产品发布
  | 'funding' // 融资
  | 'milestone' // 里程碑
  | 'acquisition' // 收购
  | 'award' // 获奖
  | 'growth' // 增长

/**
 * 时间线事件接口
 */
export interface TimelineEvent {
  id: string
  date: string // YYYY-MM-DD
  title: string
  description: string
  type: TimelineEventType
  competitor?: string // 关联的竞品名称
}

/**
 * 事件类型配置
 */
const EVENT_TYPE_CONFIG: Record<
  TimelineEventType,
  {
    icon: React.ComponentType<{ className?: string }>
    color: string
    bgColor: string
    label: string
  }
> = {
  founding: {
    icon: Building2,
    color: 'text-blue-600',
    bgColor: 'bg-blue-100',
    label: '创立',
  },
  product: {
    icon: Rocket,
    color: 'text-green-600',
    bgColor: 'bg-green-100',
    label: '产品发布',
  },
  funding: {
    icon: DollarSign,
    color: 'text-yellow-600',
    bgColor: 'bg-yellow-100',
    label: '融资',
  },
  milestone: {
    icon: Target,
    color: 'text-purple-600',
    bgColor: 'bg-purple-100',
    label: '里程碑',
  },
  acquisition: {
    icon: Handshake,
    color: 'text-red-600',
    bgColor: 'bg-red-100',
    label: '收购',
  },
  award: {
    icon: Award,
    color: 'text-orange-600',
    bgColor: 'bg-orange-100',
    label: '获奖',
  },
  growth: {
    icon: TrendingUp,
    color: 'text-cyan-600',
    bgColor: 'bg-cyan-100',
    label: '增长',
  },
}

/**
 * 从竞品分析数据生成时间线事件（简化实现）
 */
function generateTimelineEvents(analysis: CompetitorAnalysisType): TimelineEvent[] {
  const events: TimelineEvent[] = []
  const now = new Date()

  // 为每个竞品生成模拟的时间线事件
  analysis.competitors.forEach((competitor, index) => {
    const baseYear = now.getFullYear() - (5 - index) // 不同年份创立

    // 创立事件
    events.push({
      id: `${competitor.name}-founding`,
      date: `${baseYear}-01-15`,
      title: `${competitor.name} 成立`,
      description: `公司正式成立，开始${competitor.strengths[0] || '产品开发'}`,
      type: 'founding',
      competitor: competitor.name,
    })

    // 产品发布事件
    events.push({
      id: `${competitor.name}-product-v1`,
      date: `${baseYear}-06-01`,
      title: `发布 v1.0 产品`,
      description: '推出首个版本，核心功能上线',
      type: 'product',
      competitor: competitor.name,
    })

    // 融资事件
    if (index === 0) {
      // 只有第一个竞品有融资事件
      events.push({
        id: `${competitor.name}-funding-a`,
        date: `${baseYear + 1}-03-01`,
        title: '完成 A 轮融资',
        description: '获得知名投资机构投资，加速发展',
        type: 'funding',
        competitor: competitor.name,
      })
    }

    // 里程碑事件
    events.push({
      id: `${competitor.name}-milestone-users`,
      date: `${baseYear + 2}-09-01`,
      title: '用户突破 100 万',
      description: '注册用户数达到重要里程碑',
      type: 'milestone',
      competitor: competitor.name,
    })

    // 增长事件
    if (competitor.marketShare) {
      events.push({
        id: `${competitor.name}-growth-market`,
        date: `${baseYear + 3}-01-01`,
        title: `市场份额达到 ${competitor.marketShare}`,
        description: '在市场中占据重要地位',
        type: 'growth',
        competitor: competitor.name,
      })
    }
  })

  // 按日期排序
  return events.sort((a, b) => new Date(a.date).getTime() - new Date(b.date).getTime())
}

interface CompetitorTimelineProps {
  analysis: CompetitorAnalysisType
  /** 显示模式 */
  viewMode?: 'all' | 'by-competitor'
}

/**
 * 竞品时间线组件
 */
export function CompetitorTimeline({ analysis, viewMode = 'all' }: CompetitorTimelineProps) {
  const events = generateTimelineEvents(analysis)

  // 格式化日期显示
  const formatDate = (dateString: string) => {
    const date = new Date(dateString + 'T00:00:00') // 添加时间避免时区问题
    const year = date.getFullYear()
    const month = date.getMonth() + 1
    return `${year}年${month}月`
  }

  return (
    <Card className="w-full">
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          <Calendar className="w-5 h-5" />
          竞品发展历程
        </CardTitle>
      </CardHeader>
      <CardContent>
        <div className="relative">
          {/* 时间轴线 */}
          <div className="absolute left-8 top-0 bottom-0 w-0.5 bg-gradient-to-b from-blue-500 via-purple-500 to-green-500" />

          {/* 事件列表 */}
          <div className="space-y-6">
            {events.map((event, index) => {
              const config = EVENT_TYPE_CONFIG[event.type]
              const Icon = config.icon

              return (
                <div
                  key={event.id}
                  className="relative flex items-start gap-4 animate-in fade-in slide-in-from-bottom-4"
                >
                  {/* 时间点标记 */}
                  <div
                    className={`relative z-10 w-16 h-16 rounded-full ${config.bgColor} ${config.color} flex items-center justify-center shadow-lg ring-4 ring-white flex-shrink-0`}
                    style={{ animationDelay: `${index * 100}ms` }}
                  >
                    <Icon className="w-7 h-7" />
                  </div>

                  {/* 事件卡片 */}
                  <Card className="flex-1 border-0 shadow-md hover:shadow-lg transition-shadow duration-300">
                    <CardContent className="p-4">
                      <div className="flex items-start justify-between mb-2">
                        <div>
                          <h3 className="font-semibold text-lg text-gray-900">{event.title}</h3>
                          <div className="flex items-center gap-2 mt-1">
                            <Badge
                              variant="secondary"
                              className={`${config.bgColor} ${config.color}`}
                            >
                              {config.label}
                            </Badge>
                            {event.competitor && (
                              <Badge variant="outline">{event.competitor}</Badge>
                            )}
                          </div>
                        </div>
                        <div className="text-right">
                          <div className="text-sm font-medium text-gray-600">
                            {formatDate(event.date)}
                          </div>
                        </div>
                      </div>

                      <p className="text-sm text-gray-600 leading-relaxed">{event.description}</p>
                    </CardContent>
                  </Card>
                </div>
              )
            })}
          </div>
        </div>

        {/* 图例说明 */}
        <div className="mt-6 pt-4 border-t">
          <h4 className="text-sm font-medium text-gray-700 mb-3">事件类型</h4>
          <div className="flex flex-wrap gap-3">
            {Object.entries(EVENT_TYPE_CONFIG).map(([type, config]) => {
              const Icon = config.icon
              return (
                <div key={type} className="flex items-center gap-2 px-3 py-2 rounded-lg bg-gray-50">
                  <Icon className={`w-4 h-4 ${config.color}`} />
                  <span className="text-xs text-gray-600">{config.label}</span>
                </div>
              )
            })}
          </div>
        </div>
      </CardContent>
    </Card>
  )
}
