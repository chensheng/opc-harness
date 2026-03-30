import { Checkbox } from '@/components/ui/checkbox'
import { Label } from '@/components/ui/label'
import { Badge } from '@/components/ui/badge'
import type { MetricKey } from '@/types'

interface MetricSelectorProps {
  selectedMetrics: MetricKey[]
  onToggle: (metric: MetricKey) => void
}

const METRICS: Array<{ key: MetricKey; label: string; description: string }> = [
  { key: 'marketShare', label: '市场份额', description: '市场占有率百分比' },
  { key: 'revenue', label: '收入', description: '年度营业收入' },
  { key: 'userGrowth', label: '用户增长', description: '用户增长率' },
  { key: 'employeeCount', label: '员工数量', description: '公司员工总数' },
  { key: 'funding', label: '融资', description: '总融资金额' },
  { key: 'customerSatisfaction', label: '客户满意度', description: '客户满意度评分' },
  { key: 'innovationScore', label: '创新能力', description: '创新能力评分' },
]

export function MetricSelector({ selectedMetrics, onToggle }: MetricSelectorProps) {
  return (
    <div className="space-y-2">
      <div className="flex items-center justify-between">
        <Label className="text-sm font-medium">选择指标</Label>
        <Badge variant="outline">
          {selectedMetrics.length} / {METRICS.length}
        </Badge>
      </div>
      <div className="grid grid-cols-2 gap-3 max-h-48 overflow-y-auto p-3 border rounded-md bg-card">
        {METRICS.map(metric => (
          <div
            key={metric.key}
            className="flex items-start space-x-2 p-2 rounded hover:bg-muted/50 transition-colors"
          >
            <Checkbox
              id={metric.key}
              checked={selectedMetrics.includes(metric.key)}
              onChange={() => onToggle(metric.key)}
            />
            <div className="grid gap-1.5 leading-none">
              <Label htmlFor={metric.key} className="text-sm font-medium cursor-pointer">
                {metric.label}
              </Label>
              <p className="text-xs text-muted-foreground">{metric.description}</p>
            </div>
          </div>
        ))}
      </div>
    </div>
  )
}
