import { render, screen, fireEvent } from '@testing-library/react'
import { describe, it, expect, vi } from 'vitest'
import { PRDDisplayAIChat } from './PRDDisplayAIChat'
import type { ChatMessage } from '@/hooks/usePRDAIChat'

describe('PRDDisplayAIChat', () => {
  const mockProps = {
    messages: [] as ChatMessage[],
    isStreaming: false,
    error: null,
    onSendMessage: vi.fn(),
    onStopStream: vi.fn(),
    onApplyOptimization: vi.fn(),
    onClose: vi.fn(),
  }

  it('应该正确渲染组件', () => {
    render(<PRDDisplayAIChat {...mockProps} />)
    expect(screen.getByText('AI 优化助手')).toBeInTheDocument()
  })

  it('应该显示空状态提示', () => {
    render(<PRDDisplayAIChat {...mockProps} />)
    expect(screen.getByText('输入优化需求，AI 将帮您完善 PRD')).toBeInTheDocument()
  })

  it('应该显示快捷建议按钮', () => {
    render(<PRDDisplayAIChat {...mockProps} />)
    expect(screen.getByText('完善目标用户画像')).toBeInTheDocument()
    expect(screen.getByText('补充技术实现细节')).toBeInTheDocument()
  })

  it('点击快捷建议应该调用onSendMessage', () => {
    render(<PRDDisplayAIChat {...mockProps} />)
    fireEvent.click(screen.getByText('完善目标用户画像'))
    expect(mockProps.onSendMessage).toHaveBeenCalledWith('完善目标用户画像')
  })

  it('应该能够发送消息', () => {
    render(<PRDDisplayAIChat {...mockProps} />)
    const textarea = screen.getByPlaceholderText(/输入优化需求/)
    fireEvent.change(textarea, { target: { value: '测试消息' } })
    
    // 找到包含Send图标的按钮
    const allButtons = screen.getAllByRole('button')
    // 发送按钮在输入区域，是最后一个有SVG子元素的按钮
    const sendButton = allButtons.find(btn => {
      const svg = btn.querySelector('svg')
      return svg && btn.parentElement?.classList.contains('flex-col')
    })
    
    expect(sendButton).toBeDefined()
    if (sendButton) {
      fireEvent.click(sendButton)
      expect(mockProps.onSendMessage).toHaveBeenCalledWith('测试消息')
    }
  })

  it('应该显示消息历史', () => {
    const messages: ChatMessage[] = [
      { role: 'user', content: '你好' },
      { role: 'assistant', content: '你好！有什么可以帮助你的吗？' },
    ]
    render(<PRDDisplayAIChat {...mockProps} messages={messages} />)
    expect(screen.getByText('你好')).toBeInTheDocument()
    expect(screen.getByText('你好！有什么可以帮助你的吗？')).toBeInTheDocument()
  })

  it('流式状态下应该显示加载指示器', () => {
    render(<PRDDisplayAIChat {...mockProps} isStreaming={true} />)
    expect(screen.getByText('AI 正在生成...')).toBeInTheDocument()
  })

  it('应该显示错误信息', () => {
    render(<PRDDisplayAIChat {...mockProps} error="测试错误" />)
    expect(screen.getByText('测试错误')).toBeInTheDocument()
  })

  it('点击关闭按钮应该调用onClose', () => {
    render(<PRDDisplayAIChat {...mockProps} />)
    // 获取所有按钮，第一个按钮是关闭按钮
    const buttons = screen.getAllByRole('button')
    const closeButton = buttons[0]
    fireEvent.click(closeButton)
    expect(mockProps.onClose).toHaveBeenCalled()
  })

  it('助手消息完成后应该显示应用按钮', () => {
    const messages: ChatMessage[] = [
      { role: 'user', content: '优化PRD' },
      { role: 'assistant', content: '这是优化后的内容' },
    ]
    render(<PRDDisplayAIChat {...mockProps} messages={messages} />)
    const applyButton = screen.getByText('应用优化')
    expect(applyButton).toBeInTheDocument()
    fireEvent.click(applyButton)
    expect(mockProps.onApplyOptimization).toHaveBeenCalledWith('这是优化后的内容')
  })
})
