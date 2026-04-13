import { describe, it, expect } from 'vitest'
import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
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
  it('renders user story list view by default', () => {
    render(<UserStoryManager prdContent={mockPRD} />)

    // 检查标题区域的用户故事文本
    const heading = screen.getByRole('heading', { level: 2 })
    expect(heading).toHaveTextContent(/用户故事/i)

    // 检查是否存在拆分按钮（可能有多个）
    const buttons = screen.getAllByRole('button')
    const hasDecomposeButton = buttons.some(btn => btn.textContent?.includes('拆分'))
    expect(hasDecomposeButton).toBe(true)
  })

  it('shows empty state when no stories exist', () => {
    render(<UserStoryManager prdContent={mockPRD} />)

    expect(screen.getByText(/暂无用户故事/i)).toBeInTheDocument()
    // 即使有PRD内容，空状态下的按钮也应该可用（会打开对话框）
    const decomposeButton = screen.getByRole('button', { name: /开始拆分/i })
    expect(decomposeButton).not.toBeDisabled()
  })

  it('shows disabled start button in empty state when PRD is empty', () => {
    render(<UserStoryManager prdContent="" />)

    expect(screen.getByText(/暂无用户故事/i)).toBeInTheDocument()
    // 空状态下，如果没有PRD，开始拆分按钮应该被禁用
    const decomposeButton = screen.getByRole('button', { name: /开始拆分/i })
    expect(decomposeButton).toBeDisabled()
  })

  it('shows disabled decompose button when PRD is empty', () => {
    render(<UserStoryManager prdContent="" />)

    const button = screen.getByRole('button', { name: /拆分用户故事/i })
    expect(button).toBeDisabled()
  })

  it('enables decompose button when PRD has content', () => {
    render(<UserStoryManager prdContent={mockPRD} />)

    const button = screen.getByRole('button', { name: /拆分用户故事/i })
    expect(button).not.toBeDisabled()
  })

  it('displays PRD preview in decompose dialog', async () => {
    const user = userEvent.setup()
    render(<UserStoryManager prdContent={mockPRD} />)

    // 点击拆分按钮打开对话框
    const decomposeButton = screen.getByRole('button', { name: /拆分用户故事/i })
    await user.click(decomposeButton)

    // 检查PRD预览是否显示
    expect(await screen.findByText(/帮助用户高效管理日常工作/i)).toBeInTheDocument()
  })
})
