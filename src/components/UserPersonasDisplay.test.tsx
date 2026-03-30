import { describe, it, expect } from 'vitest'
import { render, screen } from '@testing-library/react'
import { UserPersonasDisplay, PersonaCard } from './UserPersonasDisplay'
import type { UserPersona } from '@/types'

describe('UserPersonasDisplay', () => {
  it('should render skeleton when generating', () => {
    render(<UserPersonasDisplay isGenerating={true} progress={50} />)

    expect(screen.getByText(/AI 正在生成用户画像/i)).toBeInTheDocument()
    expect(screen.getByText('50%')).toBeInTheDocument()

    // Check for skeleton screens
    const skeletonCards = document.querySelectorAll('.animate-pulse')
    expect(skeletonCards.length).toBeGreaterThan(0)
  })

  it('should render empty state when no personas', () => {
    render(<UserPersonasDisplay isGenerating={false} progress={0} />)

    expect(screen.getByText(/用户画像/i)).toBeInTheDocument()
    expect(screen.getByText(/典型用户角色及其特征分析/i)).toBeInTheDocument()
  })

  it('should display persona information correctly', () => {
    render(<UserPersonasDisplay isGenerating={false} />)

    // Check for example persona
    expect(screen.getByText('张先生')).toBeInTheDocument()
    expect(screen.getByText('35 岁')).toBeInTheDocument()
    expect(screen.getByText('IT 经理')).toBeInTheDocument()
  })
})

describe('PersonaCard', () => {
  const mockPersona: UserPersona = {
    id: '1',
    name: '王工程师',
    age: '30 岁',
    occupation: '高级软件工程师',
    background: '5 年前端开发经验，精通 React 和 TypeScript',
    goals: ['学习新技术', '提升架构能力', '带领团队'],
    painPoints: ['技术更新太快', '文档不完善', '时间不够'],
    behaviors: ['阅读技术博客', '参加技术会议', '开源贡献'],
    quote: '代码改变世界',
  }

  it('should render persona card with all information', () => {
    const { container } = render(<PersonaCard persona={mockPersona} index={0} />)

    // Name
    expect(screen.getByText('王工程师')).toBeInTheDocument()

    // Age and occupation badges
    expect(screen.getByText('30 岁')).toBeInTheDocument()
    expect(screen.getByText('高级软件工程师')).toBeInTheDocument()

    // Background
    expect(screen.getByText(/背景/i)).toBeInTheDocument()
    expect(container.innerHTML).toContain('5 年前端开发经验')

    // Goals
    expect(screen.getByText(/目标/i)).toBeInTheDocument()
    expect(screen.getByText('学习新技术')).toBeInTheDocument()
    expect(screen.getByText('提升架构能力')).toBeInTheDocument()

    // Pain points
    expect(screen.getByText(/痛点/i)).toBeInTheDocument()
    expect(screen.getByText('技术更新太快')).toBeInTheDocument()

    // Behaviors
    expect(screen.getByText(/行为特征/i)).toBeInTheDocument()
    expect(screen.getByText('阅读技术博客')).toBeInTheDocument()

    // Quote
    expect(screen.getByText(/代码改变世界/i)).toBeInTheDocument()
  })

  it('should generate correct initials for avatar', () => {
    render(<PersonaCard persona={mockPersona} index={0} />)

    // Avatar shows first character of name (王工程师 -> 王)
    const avatarFallback = screen.getByText('王')
    expect(avatarFallback).toBeInTheDocument()
  })

  it('should handle persona without optional fields', () => {
    const minimalPersona: UserPersona = {
      id: '3',
      name: '赵先生',
      age: '45 岁',
      occupation: 'CEO',
      background: '',
      goals: [],
      painPoints: [],
      behaviors: [],
      quote: '',
    }

    const { container } = render(<PersonaCard persona={minimalPersona} index={2} />)

    expect(screen.getByText('赵先生')).toBeInTheDocument()
    expect(screen.getByText('赵')).toBeInTheDocument() // Avatar initials (first char)

    // Should not render empty sections
    const headings = container.querySelectorAll('h4')
    expect(headings.length).toBe(0)
  })

  it('should apply animation based on index', () => {
    const { container } = render(<PersonaCard persona={mockPersona} index={1} />)

    // Check for animation classes
    const card = container.firstChild as HTMLElement
    expect(card).toHaveClass('animate-in')
    expect(card).toHaveClass('fade-in')
    expect(card).toHaveClass('slide-in-from-bottom-4')
  })
})
