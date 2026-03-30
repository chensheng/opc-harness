import { useState, useMemo } from 'react'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import {
  BarChart,
  Bar,
  LineChart,
  Line,
  PieChart,
  Pie,
  Cell,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  Legend,
  ResponsiveContainer,
} from 'recharts'
import { BarChart3, TrendingUp, Filter, SortAsc, SortDesc, Download, RefreshCw } from 'lucide-react'
import type { Competitor, ExplorerConfig, MetricKey, ViewMode, DataFilters } from '@/types'
import { MetricSelector } from './metrics/MetricSelector'
import { DataFilter } from './metrics/DataFilter'
import { ViewSwitcher } from './metrics/ViewSwitcher'
import { MetricCard } from './metrics/MetricCard'

interface InteractiveDataExplorerProps {
  competitors: Competitor[]
}

const COLORS = ['#0088FE', '#00C49F', '#FFBB28', '#FF8042', '#8884D8', '#82CA9D']

const METRIC_LABELS: Record<MetricKey, string> = {
  marketShare: '市场份额',
  revenue: '收入',
  userGrowth: '用户增长',
  employeeCount: '员工数量',
  funding: '融资',
  customerSatisfaction: '客户满意度',
  innovationScore: '创新能力',
}

export function InteractiveDataExplorer({ competitors }: InteractiveDataExplorerProps) {
  const [config, setConfig] = useState<ExplorerConfig>({
    selectedMetrics: ['marketShare'],
    viewMode: 'bar',
    filters: {},
    sortBy: 'marketShare',
    sortOrder: 'desc',
  })

  // 过滤后的数据
  const filteredCompetitors = useMemo(() => {
    return competitors.filter(c => {
      if (config.filters.minMarketShare && c.marketShare) {
        const share = parseFloat(c.marketShare.replace('%', ''))
        if (share < config.filters.minMarketShare) return false
      }
      if (config.filters.maxEmployeeCount && c.employeeCount) {
        if (c.employeeCount > config.filters.maxEmployeeCount) return false
      }
      if (config.filters.foundedAfter && c.foundedYear) {
        if (c.foundedYear < config.filters.foundedAfter) return false
      }
      if (config.filters.hasFunding && !c.funding) {
        return false
      }
      return true
    })
  }, [competitors, config.filters])

  // 排序后的数据
  const sortedData = useMemo(() => {
    if (!config.sortBy) return filteredCompetitors

    return [...filteredCompetitors].sort((a, b) => {
      let aVal: number | undefined
      let bVal: number | undefined

      switch (config.sortBy) {
        case 'marketShare':
          aVal = a.marketShare ? parseFloat(a.marketShare.replace('%', '')) : 0
          bVal = b.marketShare ? parseFloat(b.marketShare.replace('%', '')) : 0
          break
        case 'employeeCount':
          aVal = a.employeeCount || 0
          bVal = b.employeeCount || 0
          break
        case 'customerSatisfaction':
          aVal = a.customerSatisfaction || 0
          bVal = b.customerSatisfaction || 0
          break
        case 'innovationScore':
          aVal = a.innovationScore || 0
          bVal = b.innovationScore || 0
          break
        default:
          aVal = 0
          bVal = 0
      }

      return config.sortOrder === 'asc' ? aVal - bVal : bVal - aVal
    })
  }, [filteredCompetitors, config.sortBy, config.sortOrder])

  // 图表数据准备
  const chartData = useMemo(() => {
    return sortedData.map(c => {
      const dataPoint: Record<string, string | number> = { name: c.name }

      config.selectedMetrics.forEach(metric => {
        const value = c[metric]
        let numericValue: number

        if (value === undefined || value === null) {
          numericValue = 0
        } else if (typeof value === 'string') {
          if (value.includes('%')) {
            numericValue = parseFloat(value.replace('%', ''))
          } else if (value.includes('B')) {
            numericValue = parseFloat(value.replace('B', '')) * 10
          } else if (value.includes('M')) {
            numericValue = parseFloat(value.replace('M', ''))
          } else {
            numericValue = parseFloat(value) || 0
          }
        } else {
          numericValue = value
        }

        dataPoint[metric] = numericValue
      })

      return dataPoint
    })
  }, [sortedData, config.selectedMetrics])

  const handleMetricToggle = (metric: MetricKey) => {
    setConfig(prev => ({
      ...prev,
      selectedMetrics: prev.selectedMetrics.includes(metric)
        ? prev.selectedMetrics.filter(m => m !== metric)
        : [...prev.selectedMetrics, metric],
    }))
  }

  const handleViewModeChange = (mode: ViewMode) => {
    setConfig(prev => ({ ...prev, viewMode: mode }))
  }

  const handleFilterChange = (filters: DataFilters) => {
    setConfig(prev => ({ ...prev, filters }))
  }

  const handleSortChange = (metric: MetricKey) => {
    setConfig(prev => ({
      ...prev,
      sortBy: metric,
      sortOrder: prev.sortBy === metric && prev.sortOrder === 'desc' ? 'asc' : 'desc',
    }))
  }

  const renderChart = () => {
    switch (config.viewMode) {
      case 'bar':
        return (
          <ResponsiveContainer width="100%" height={400}>
            <BarChart data={chartData}>
              <CartesianGrid strokeDasharray="3 3" />
              <XAxis dataKey="name" />
              <YAxis />
              <Tooltip />
              <Legend />
              {config.selectedMetrics.map((metric, index) => (
                <Bar key={metric} dataKey={metric} fill={COLORS[index % COLORS.length]} />
              ))}
            </BarChart>
          </ResponsiveContainer>
        )

      case 'line':
        return (
          <ResponsiveContainer width="100%" height={400}>
            <LineChart data={chartData}>
              <CartesianGrid strokeDasharray="3 3" />
              <XAxis dataKey="name" />
              <YAxis />
              <Tooltip />
              <Legend />
              {config.selectedMetrics.map((metric, index) => (
                <Line
                  key={metric}
                  type="monotone"
                  dataKey={metric}
                  stroke={COLORS[index % COLORS.length]}
                />
              ))}
            </LineChart>
          </ResponsiveContainer>
        )

      case 'pie':
        if (config.selectedMetrics.length !== 1) {
          return (
            <div className="flex items-center justify-center h-64 text-muted-foreground">
              饼图模式只支持单个指标，请选择一个指标
            </div>
          )
        }
        return (
          <ResponsiveContainer width="100%" height={400}>
            <PieChart>
              <Pie
                data={chartData}
                dataKey={config.selectedMetrics[0]}
                nameKey="name"
                cx="50%"
                cy="50%"
                outerRadius={150}
                label
              >
                {chartData.map((_, index) => (
                  <Cell key={`cell-${index}`} fill={COLORS[index % COLORS.length]} />
                ))}
              </Pie>
              <Tooltip />
              <Legend />
            </PieChart>
          </ResponsiveContainer>
        )

      case 'radar':
        return (
          <div className="flex items-center justify-center h-64 text-muted-foreground">
            雷达图模式请使用竞品对比功能
          </div>
        )

      case 'cards':
        return (
          <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
            {sortedData.map(competitor => (
              <MetricCard
                key={competitor.name}
                competitor={competitor}
                metrics={config.selectedMetrics}
              />
            ))}
          </div>
        )

      default:
        return null
    }
  }

  return (
    <Card className="animate-in fade-in slide-in-from-bottom-4 duration-500">
      <CardHeader>
        <CardTitle className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            <BarChart3 className="h-6 w-6" />
            <span>交互式数据探索</span>
          </div>
          <div className="flex items-center gap-2">
            <Button
              variant="outline"
              size="sm"
              onClick={() =>
                setConfig({
                  selectedMetrics: ['marketShare'],
                  viewMode: 'bar',
                  filters: {},
                  sortBy: 'marketShare',
                  sortOrder: 'desc',
                })
              }
            >
              <RefreshCw className="h-4 w-4 mr-1" />
              重置
            </Button>
            <Button variant="outline" size="sm">
              <Download className="h-4 w-4 mr-1" />
              导出
            </Button>
          </div>
        </CardTitle>
      </CardHeader>
      <CardContent className="space-y-6">
        {/* 工具栏 */}
        <div className="flex flex-wrap gap-4">
          <MetricSelector selectedMetrics={config.selectedMetrics} onToggle={handleMetricToggle} />
          <ViewSwitcher currentMode={config.viewMode} onChange={handleViewModeChange} />
        </div>

        {/* 过滤器 */}
        <DataFilter filters={config.filters} onChange={handleFilterChange} />

        {/* 排序按钮 */}
        <div className="flex items-center gap-2">
          <span className="text-sm text-muted-foreground">排序:</span>
          {(
            [
              'marketShare',
              'employeeCount',
              'customerSatisfaction',
              'innovationScore',
            ] as MetricKey[]
          ).map(metric => (
            <Button
              key={metric}
              variant={config.sortBy === metric ? 'default' : 'outline'}
              size="sm"
              onClick={() => handleSortChange(metric)}
            >
              {METRIC_LABELS[metric]}
              {config.sortBy === metric &&
                (config.sortOrder === 'desc' ? (
                  <SortDesc className="h-3 w-3 ml-1" />
                ) : (
                  <SortAsc className="h-3 w-3 ml-1" />
                ))}
            </Button>
          ))}
        </div>

        {/* 数据统计 */}
        <div className="flex items-center gap-4 text-sm text-muted-foreground">
          <Badge variant="secondary">
            <Filter className="h-3 w-3 mr-1" />
            显示 {filteredCompetitors.length} / {competitors.length} 个公司
          </Badge>
          <Badge variant="secondary">
            <TrendingUp className="h-3 w-3 mr-1" />
            {config.selectedMetrics.length} 个指标
          </Badge>
        </div>

        {/* 图表区域 */}
        <div className="border rounded-lg p-4 bg-muted/20">{renderChart()}</div>
      </CardContent>
    </Card>
  )
}
