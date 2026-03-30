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
    const { container } = render(
      <UserPersonasDisplay isGenerating={false} progress={0} personas={[]} />
    )

    // When personas is empty array, component returns null (no content rendered)
    expect(container.firstChild).toBeNull()
  })

  it('should display personas when provided', () => {
    const mockPersonas: UserPersona[] = [
      {
        id: '1',
        name: '张先生',
        age: '35 岁',
        occupation: 'IT 经理',
        background: '在一家中型企业担任 IT 部门负责人',
        goals: ['提高团队工作效率', '降低项目管理成本'],
        painPoints: ['技术更新太快', '人员流动大'],
        behaviors: ['阅读技术博客', '参加技术会议'],
        quote: '技术驱动业务',
      },
    ]

    render(<UserPersonasDisplay isGenerating={false} personas={mockPersonas} />)

    expect(screen.getByText('张先生')).toBeInTheDocument()
    expect(screen.getByText('35 岁')).toBeInTheDocument()
    expect(screen.getByText('IT 经理')).toBeInTheDocument()
    expect(screen.getByText(/背景/i)).toBeInTheDocument()
    expect(screen.getByText(/目标/i)).toBeInTheDocument()
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
    render(<PersonaCard persona={mockPersona} index={0} />)

    // Name
    expect(screen.getByText('王工程师')).toBeInTheDocument()

    // Age and occupation badges
    expect(screen.getByText('30 岁')).toBeInTheDocument()
    expect(screen.getByText('高级软件工程师')).toBeInTheDocument()

    // Background
    expect(screen.getByText(/背景/i)).toBeInTheDocument()
    expect(screen.getByText(/5 年前端开发经验/i)).toBeInTheDocument()

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
    const avatarElement = screen.getByText('王')
    expect(avatarElement).toBeInTheDocument()
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
    expect(screen.getByText('赵')).toBeInTheDocument() // Avatar initials

    // Should not render empty sections
    const headings = container.querySelectorAll('h4')
    expect(headings.length).toBe(0)
  })

  it('should apply gradient color based on index', () => {
    const { container: container1 } = render(<PersonaCard persona={mockPersona} index={0} />)
    const { container: container2 } = render(<PersonaCard persona={mockPersona} index={1} />)

    const card1 = container1.firstChild as HTMLElement
    const card2 = container2.firstChild as HTMLElement

    // Both should have animation classes
    expect(card1).toHaveClass('animate-in')
    expect(card2).toHaveClass('animate-in')

    // Different indices should have different gradient colors
    expect(card1.innerHTML).toContain('from-blue-500')
    expect(card2.innerHTML).toContain('from-purple-500')
  })

  it('should have hover effect classes', () => {
    render(<PersonaCard persona={mockPersona} index={0} />)

    const card = document.querySelector('.group')
    expect(card).toBeInTheDocument()
    expect(card?.className).toContain('hover:shadow-xl')
    expect(card?.className).toContain('transition-all')
  })
})
