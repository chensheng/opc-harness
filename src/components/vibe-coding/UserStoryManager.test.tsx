import { describe, it, expect } from 'vitest'
import { render, screen } from '@testing-library/react'
import { UserStoryManager } from './UserStoryManager'

const mockPRD = `# 任务管理系统

## 产品概述
我们需要一个任务管理系统，帮助用户高效管理日常工作。

## 核心功能
1. 用户可以创建、编辑、删除任务
2. 任务可以设置优先级和截止日期
3. 支持任务分类和标签
4. 提供任务统计报表`

describe('UserStoryManager', () => {
  it('renders input tab by default', () => {
    render(<UserStoryManager prdContent={mockPRD} />)

    expect(screen.getByText(/拆分配置/i)).toBeInTheDocument()
    expect(screen.getByText(/开始拆分用户故事/i)).toBeInTheDocument()
  })

  it('shows disabled button when PRD is empty', () => {
    render(<UserStoryManager prdContent="" />)

    const button = screen.getByRole('button', { name: /开始拆分用户故事/i })
    expect(button).toBeDisabled()
  })

  it('enables button when PRD has content', () => {
    render(<UserStoryManager prdContent={mockPRD} />)

    const button = screen.getByRole('button', { name: /开始拆分用户故事/i })
    expect(button).not.toBeDisabled()
  })

  it('displays PRD preview', () => {
    render(<UserStoryManager prdContent={mockPRD} />)

    // 使用更具体的文本匹配，避免在多个元素中找到
    expect(screen.getByText(/帮助用户高效管理日常工作/i)).toBeInTheDocument()
  })

  it('has prompt input field', () => {
    render(<UserStoryManager prdContent={mockPRD} />)

    const textarea = screen.getByPlaceholderText(/例如：/i)
    expect(textarea).toBeInTheDocument()
  })
})
