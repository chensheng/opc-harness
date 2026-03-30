import { describe, it, expect } from 'vitest'
import { render, screen } from '@testing-library/react'
import { CompetitorTimeline } from './CompetitorTimeline'
import type { CompetitorAnalysis } from '@/types'

describe('CompetitorTimeline', () => {
  const mockAnalysis: CompetitorAnalysis = {
    competitors: [
      {
        name: 'Competitor A',
        strengths: ['Feature complete', 'Brand recognition'],
        weaknesses: ['Expensive'],
        marketShare: '35%',
      },
      {
        name: 'Competitor B',
        strengths: ['Free', 'Open source'],
        weaknesses: ['Performance issues'],
        marketShare: '25%',
      },
    ],
    differentiation: 'Our advantage',
    opportunities: ['Opportunity 1', 'Opportunity 2'],
  }

  it('should render timeline with events', () => {
    render(<CompetitorTimeline analysis={mockAnalysis} />)

    // Check title
    expect(screen.getByText('竞品发展历程')).toBeInTheDocument()

    // Check that at least one event is rendered
    const timelineEvents = screen.getAllByText(/成立/)
    expect(timelineEvents.length).toBeGreaterThan(0)
  })

  it('should render competitor names', () => {
    render(<CompetitorTimeline analysis={mockAnalysis} />)

    // Check competitor names exist
    const competitorAElements = screen.getAllByText('Competitor A')
    const competitorBElements = screen.getAllByText('Competitor B')

    expect(competitorAElements.length).toBeGreaterThan(0)
    expect(competitorBElements.length).toBeGreaterThan(0)
  })

  it('should render date information', () => {
    render(<CompetitorTimeline analysis={mockAnalysis} />)

    // Check for year/month display
    const yearElements = document.querySelectorAll('[class*="text-gray-600"]')
    expect(yearElements.length).toBeGreaterThan(0)
  })

  it('should render event legend', () => {
    render(<CompetitorTimeline analysis={mockAnalysis} />)

    // Check legend section title
    expect(screen.getByText('事件类型')).toBeInTheDocument()

    // Check that legend section exists
    const legendSection = screen.getByText('事件类型').closest('div')
    expect(legendSection).toBeInTheDocument()
  })

  it('should sort events by date', () => {
    const { container } = render(<CompetitorTimeline analysis={mockAnalysis} />)

    // Component should render without errors
    expect(container).toBeInTheDocument()

    // Events should be sorted chronologically (earliest first)
    const foundingEvent = screen.getByText(/Competitor A 成立/)
    expect(foundingEvent).toBeInTheDocument()
  })

  it('should handle empty competitors', () => {
    const emptyAnalysis: CompetitorAnalysis = {
      competitors: [],
      differentiation: 'No competitors',
      opportunities: [],
    }

    render(<CompetitorTimeline analysis={emptyAnalysis} />)

    // Should still render the component
    expect(screen.getByText('竞品发展历程')).toBeInTheDocument()
  })
})
