import { render, screen } from '@testing-library/react'
import { describe, it, expect } from 'vitest'
import { InteractiveDataExplorer } from './InteractiveDataExplorer'
import type { Competitor } from '@/types'

const mockCompetitors: Competitor[] = [
  {
    name: 'Competitor A',
    strengths: ['品牌知名度高', '功能完善'],
    weaknesses: ['价格较高'],
    marketShare: '35%',
    revenue: '100M',
    userGrowth: '25%',
    employeeCount: 500,
    funding: '50M',
    customerSatisfaction: 85,
    innovationScore: 75,
  },
  {
    name: 'Competitor B',
    strengths: ['免费版本'],
    weaknesses: ['技术支持有限'],
    marketShare: '25%',
    revenue: '50M',
    userGrowth: '15%',
    employeeCount: 200,
    funding: '20M',
    customerSatisfaction: 70,
    innovationScore: 60,
  },
]

describe('InteractiveDataExplorer', () => {
  it('renders correctly with competitors data', () => {
    render(<InteractiveDataExplorer competitors={mockCompetitors} />)

    expect(screen.getByText('交互式数据探索')).toBeInTheDocument()
    expect(screen.getByText('选择指标')).toBeInTheDocument()
  })

  it('displays initial statistics', () => {
    render(<InteractiveDataExplorer competitors={mockCompetitors} />)

    // 检查是否显示统计信息
    expect(screen.getByText(/显示 2 \/ 2 个公司/)).toBeInTheDocument()
  })

  it('has view switcher buttons', () => {
    render(<InteractiveDataExplorer competitors={mockCompetitors} />)

    // 检查视图切换按钮存在
    expect(screen.getByText('柱状图')).toBeInTheDocument()
    expect(screen.getByText('折线图')).toBeInTheDocument()
    expect(screen.getByText('饼图')).toBeInTheDocument()
  })

  it('has metric selector checkboxes', () => {
    render(<InteractiveDataExplorer competitors={mockCompetitors} />)

    // 检查复选框存在
    const checkboxes = screen.getAllByRole('checkbox')
    expect(checkboxes.length).toBeGreaterThan(0)
  })

  it('has filter button', () => {
    render(<InteractiveDataExplorer competitors={mockCompetitors} />)

    expect(screen.getByText('数据过滤')).toBeInTheDocument()
  })

  it('has sort buttons', () => {
    render(<InteractiveDataExplorer competitors={mockCompetitors} />)

    // 检查排序按钮存在（可能有多个相同文本的按钮）
    const sortButtons = screen.getAllByText('员工数量')
    expect(sortButtons.length).toBeGreaterThan(0)
  })
})
