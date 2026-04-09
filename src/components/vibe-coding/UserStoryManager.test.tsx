import { describe, it, expect } from 'vitest'
import { render, screen } from '@testing-library/react'
import { UserStoryManager } from './UserStoryManager'

describe('UserStoryManager', () => {
  it('renders input tab by default', () => {
    render(<UserStoryManager />)

    expect(screen.getByText(/PRD 内容或功能描述/i)).toBeInTheDocument()
    expect(screen.getByText(/开始拆分用户故事/i)).toBeInTheDocument()
  })

  it('shows disabled button when input is empty', () => {
    render(<UserStoryManager />)

    const button = screen.getByRole('button', { name: /开始拆分用户故事/i })
    expect(button).toBeDisabled()
  })

  it('enables button when input has content', () => {
    render(<UserStoryManager prdContent="Test PRD content" />)

    const textarea = screen.getByPlaceholderText(/例如：/i)
    expect(textarea).toHaveValue('Test PRD content')
  })
})
