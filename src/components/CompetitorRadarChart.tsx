import {
  ResponsiveContainer,
  RadarChart,
  PolarGrid,
  PolarAngleAxis,
  PolarRadiusAxis,
  Radar,
  Legend,
  Tooltip,
} from 'recharts'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import type { CompetitorAnalysis as CompetitorAnalysisType, Competitor } from '@/types'

interface RadarChartData {
  dimension: string
  ourProduct: number
  [key: string]: string | number
}

interface CompetitorRadarChartProps {
  analysis: CompetitorAnalysisType
  /** 本产品名称 */
  productName?: string
}

/**
 * 评估维度配置
 */
const DIMENSIONS = [
  { key: 'functionality', label: '功能性', weight: 1.0 },
  { key: 'usability', label: '易用性', weight: 0.9 },
  { key: 'performance', label: '性能', weight: 0.8 },
  { key: 'reliability', label: '可靠性', weight: 0.85 },
  { key: 'innovation', label: '创新性', weight: 0.75 },
  { key: 'value', label: '性价比', weight: 0.8 },
]

/**
 * 配色方案
 */
const COLORS = ['#2563eb', '#ef4444', '#22c55e', '#f97316', '#8b5cf6']

/**
 * 从竞品数据中生成雷达图评分
 */
function generateRadarData(analysis: CompetitorAnalysisType): RadarChartData[] {
  const data: RadarChartData[] = []

  DIMENSIONS.forEach(dim => {
    const dataPoint: RadarChartData = {
      dimension: dim.label,
      ourProduct: calculateScoreForDimension('ourProduct', dim),
    }

    // 为每个竞品生成分数
    analysis.competitors.forEach((competitor, index) => {
      const key = `competitor${index + 1}`
      dataPoint[key] = calculateScoreForCompetitor(competitor, dim)
    })

    data.push(dataPoint)
  })

  return data
}

/**
 * 计算某个维度的分数（简化实现）
 */
function calculateScoreForDimension(product: string, dimension: (typeof DIMENSIONS)[0]): number {
  // 简化实现：返回基准分数
  if (product === 'ourProduct') {
    // 本产品根据维度权重返回分数
    return Math.round(dimension.weight * 85 + Math.random() * 10)
  }
  return 75
}

/**
 * 计算竞品的维度分数
 */
function calculateScoreForCompetitor(
  competitor: Competitor,
  dimension: (typeof DIMENSIONS)[0]
): number {
  // 基于优劣势数量计算分数
  const baseScore = 70
  const strengthBonus = competitor.strengths.length * 5
  const weaknessPenalty = competitor.weaknesses.length * 3
  const score = baseScore + strengthBonus - weaknessPenalty

  // 根据维度调整
  const adjustedScore = score * dimension.weight

  return Math.min(100, Math.max(0, Math.round(adjustedScore)))
}

/**
 * 竞品对比雷达图组件
 */
export function CompetitorRadarChart({
  analysis,
  productName = '本产品',
}: CompetitorRadarChartProps) {
  const data = generateRadarData(analysis)

  // 构建图例数据
  const legendData = [
    { name: productName, color: COLORS[0] },
    ...analysis.competitors.map((c, i) => ({
      name: c.name,
      color: COLORS[i + 1] || COLORS[(i + 1) % COLORS.length],
    })),
  ]

  return (
    <Card className="w-full">
      <CardHeader>
        <CardTitle className="text-lg">竞品对比分析</CardTitle>
      </CardHeader>
      <CardContent>
        <div className="w-full h-[400px]">
          <ResponsiveContainer width="100%" height="100%">
            <RadarChart cx="50%" cy="50%" outerRadius="80%" data={data}>
              <PolarGrid stroke="#e5e7eb" />
              <PolarAngleAxis dataKey="dimension" tick={{ fill: '#374151', fontSize: 12 }} />
              <PolarRadiusAxis
                angle={30}
                domain={[0, 100]}
                tick={{ fill: '#6b7280', fontSize: 10 }}
              />

              {/* 本产品 */}
              <Radar
                name={productName}
                dataKey="ourProduct"
                stroke={COLORS[0]}
                fill={COLORS[0]}
                fillOpacity={0.6}
                strokeWidth={2}
              />

              {/* 竞品 */}
              {analysis.competitors.map((_, index) => (
                <Radar
                  key={index}
                  name={analysis.competitors[index].name}
                  dataKey={`competitor${index + 1}`}
                  stroke={COLORS[index + 1] || COLORS[(index + 1) % COLORS.length]}
                  fill={COLORS[index + 1] || COLORS[(index + 1) % COLORS.length]}
                  fillOpacity={0.4}
                  strokeWidth={2}
                />
              ))}

              <Legend />
              <Tooltip
                contentStyle={{
                  backgroundColor: 'rgba(255, 255, 255, 0.95)',
                  border: '1px solid #e5e7eb',
                  borderRadius: '8px',
                  boxShadow: '0 4px 6px -1px rgba(0, 0, 0, 0.1)',
                }}
              />
            </RadarChart>
          </ResponsiveContainer>
        </div>

        {/* 图例说明 */}
        <div className="mt-4 flex flex-wrap gap-4 justify-center">
          {legendData.map((item, index) => (
            <div key={index} className="flex items-center gap-2">
              <div className="w-3 h-3 rounded" style={{ backgroundColor: item.color }} />
              <span className="text-sm text-gray-600">{item.name}</span>
            </div>
          ))}
        </div>
      </CardContent>
    </Card>
  )
}
