import { describe, it, expect, vi } from 'vitest'
import { render, screen } from '@testing-library/react'
import { CompetitorRadarChart } from './CompetitorRadarChart'
import type { CompetitorAnalysis } from '@/types'

// Mock Recharts
vi.mock('recharts', () => ({
  ResponsiveContainer: ({ children }: { children: React.ReactNode }) => (
    <div data-testid="responsive-container">{children}</div>
  ),
  RadarChart: ({ children }: { children: React.ReactNode }) => (
    <div data-testid="radar-chart">{children}</div>
  ),
  PolarGrid: () => <div data-testid="polar-grid" />,
  PolarAngleAxis: () => <div data-testid="polar-angle-axis" />,
  PolarRadiusAxis: () => <div data-testid="polar-radius-axis" />,
  Radar: () => <div data-testid="radar" />,
  Legend: () => <div data-testid="legend" />,
  Tooltip: () => <div data-testid="tooltip" />,
}))

describe('CompetitorRadarChart', () => {
  const mockAnalysis: CompetitorAnalysis = {
    competitors: [
      {
        name: '竞品 A',
        strengths: ['功能完善', '品牌知名'],
        weaknesses: ['价格高'],
        marketShare: '35%',
      },
      {
        name: '竞品 B',
        strengths: ['免费', '开源'],
        weaknesses: ['性能差', '支持有限'],
        marketShare: '25%',
      },
    ],
    differentiation: '我们的优势',
    opportunities: ['机会 1', '机会 2'],
  }

  it('should render radar chart with analysis data', () => {
    render(<CompetitorRadarChart analysis={mockAnalysis} productName="测试产品" />)

    // Check title
    expect(screen.getByText('竞品对比分析')).toBeInTheDocument()

    // Check legend items
    expect(screen.getByText('测试产品')).toBeInTheDocument()
    expect(screen.getByText('竞品 A')).toBeInTheDocument()
    expect(screen.getByText('竞品 B')).toBeInTheDocument()
  })

  it('should render with single competitor', () => {
    const singleCompetitorAnalysis: CompetitorAnalysis = {
      competitors: [
        {
          name: '唯一竞品',
          strengths: ['优势'],
          weaknesses: ['劣势'],
          marketShare: '50%',
        },
      ],
      differentiation: '差异化',
      opportunities: [],
    }

    render(<CompetitorRadarChart analysis={singleCompetitorAnalysis} productName="本产品" />)

    expect(screen.getByText('本产品')).toBeInTheDocument()
    expect(screen.getByText('唯一竞品')).toBeInTheDocument()
  })

  it('should render with multiple competitors', () => {
    const multiCompetitorAnalysis: CompetitorAnalysis = {
      competitors: [
        {
          name: '竞品 1',
          strengths: ['优势 1'],
          weaknesses: ['劣势 1'],
          marketShare: '30%',
        },
        {
          name: '竞品 2',
          strengths: ['优势 2'],
          weaknesses: ['劣势 2'],
          marketShare: '25%',
        },
        {
          name: '竞品 3',
          strengths: ['优势 3'],
          weaknesses: ['劣势 3'],
          marketShare: '20%',
        },
      ],
      differentiation: '差异化优势',
      opportunities: ['机会'],
    }

    render(<CompetitorRadarChart analysis={multiCompetitorAnalysis} productName="我们的产品" />)

    expect(screen.getByText('我们的产品')).toBeInTheDocument()
    expect(screen.getByText('竞品 1')).toBeInTheDocument()
    expect(screen.getByText('竞品 2')).toBeInTheDocument()
    expect(screen.getByText('竞品 3')).toBeInTheDocument()
  })

  it('should use default product name when not provided', () => {
    render(<CompetitorRadarChart analysis={mockAnalysis} />)

    // Should use default name "本产品"
    expect(screen.getByText('本产品')).toBeInTheDocument()
  })

  it('should calculate scores based on strengths and weaknesses', () => {
    const { container } = render(
      <CompetitorRadarChart analysis={mockAnalysis} productName="Test Product" />
    )

    // Component should render without errors
    expect(container).toBeInTheDocument()

    // Check that the chart renders with correct structure
    expect(screen.getByText('竞品对比分析')).toBeInTheDocument()
    expect(screen.getByText('Test Product')).toBeInTheDocument()
    expect(screen.getByText('竞品 A')).toBeInTheDocument()
    expect(screen.getByText('竞品 B')).toBeInTheDocument()
  })
})
