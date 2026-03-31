/**
 * 流式输出展示组件测试
 */
import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { describe, it, expect, vi, beforeEach } from 'vitest'
import { StreamingDisplay } from './StreamingDisplay'

// Mock useStreaming hook
const mockUseStreaming = vi.fn()
vi.mock('../hooks/useStreaming', () => ({
  useStreaming: () => mockUseStreaming(),
}))

describe('StreamingDisplay', () => {
  const mockProps = {
    prompt: 'Test prompt for streaming',
    autoStart: false,
    onComplete: vi.fn(),
    onError: vi.fn(),
  }

  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should render initial state', () => {
    mockUseStreaming.mockReturnValue({
      isStreaming: false,
      content: '',
      progress: 0,
      error: null,
      startStream: vi.fn(),
      stopStream: vi.fn(),
    })

    render(<StreamingDisplay {...mockProps} />)

    expect(screen.getByText('AI 流式输出')).toBeInTheDocument()
    expect(screen.getByText('等待生成')).toBeInTheDocument()
    expect(screen.getByText('0 字符')).toBeInTheDocument()
  })

  it('should show loading state when streaming', async () => {
    // Mock streaming state
    mockUseStreaming.mockReturnValue({
      isStreaming: true,
      content: '',
      progress: 50,
      error: null,
      startStream: vi.fn(),
      stopStream: vi.fn(),
    })

    render(<StreamingDisplay {...mockProps} />)

    expect(screen.getByText('生成中...')).toBeInTheDocument()
    expect(screen.getByText('生成进度')).toBeInTheDocument()
    expect(screen.getByText('50%')).toBeInTheDocument()
  })

  it('should display streamed content', () => {
    // Mock content state
    mockUseStreaming.mockReturnValue({
      isStreaming: false,
      content: 'This is the generated content',
      progress: 100,
      error: null,
      startStream: vi.fn(),
      stopStream: vi.fn(),
    })

    render(<StreamingDisplay {...mockProps} />)

    expect(screen.getByText(/This is the generated content/)).toBeInTheDocument()
    expect(screen.getByText('生成完成')).toBeInTheDocument()
    expect(screen.getByText('29 字符')).toBeInTheDocument()
  })

  it('should show error state', () => {
    // Mock error state
    mockUseStreaming.mockReturnValue({
      isStreaming: false,
      content: '',
      progress: 0,
      error: 'Network error occurred',
      startStream: vi.fn(),
      stopStream: vi.fn(),
    })

    render(<StreamingDisplay {...mockProps} />)

    expect(screen.getByText('生成失败')).toBeInTheDocument()
    expect(screen.getByText('Network error occurred')).toBeInTheDocument()
  })

  it('should call startStream when regenerate button clicked', async () => {
    const mockStartStream = vi.fn()

    mockUseStreaming.mockReturnValue({
      isStreaming: false,
      content: '',
      progress: 0,
      error: null,
      startStream: mockStartStream,
      stopStream: vi.fn(),
    })

    render(<StreamingDisplay {...mockProps} />)

    const regenerateButton = screen.getByText('重新生成')
    await userEvent.click(regenerateButton)

    expect(mockStartStream).toHaveBeenCalledWith(mockProps.prompt)
  })

  it('should call stopStream when stop button clicked', async () => {
    const mockStopStream = vi.fn()

    mockUseStreaming.mockReturnValue({
      isStreaming: true,
      content: 'Generating...',
      progress: 50,
      error: null,
      startStream: vi.fn(),
      stopStream: mockStopStream,
    })

    render(<StreamingDisplay {...mockProps} />)

    const stopButton = screen.getByText('停止')
    await userEvent.click(stopButton)

    expect(mockStopStream).toHaveBeenCalled()
  })

  it('should call onComplete when streaming finishes', () => {
    const mockOnComplete = vi.fn()

    mockUseStreaming.mockReturnValue({
      isStreaming: false,
      content: 'Completed content',
      progress: 100,
      error: null,
      startStream: vi.fn(),
      stopStream: vi.fn(),
    })

    render(<StreamingDisplay {...mockProps} onComplete={mockOnComplete} />)

    expect(mockOnComplete).toHaveBeenCalledWith('Completed content')
  })

  it('should call onError when error occurs', () => {
    const mockOnError = vi.fn()

    mockUseStreaming.mockReturnValue({
      isStreaming: false,
      content: '',
      progress: 0,
      error: 'Test error',
      startStream: vi.fn(),
      stopStream: vi.fn(),
    })

    render(<StreamingDisplay {...mockProps} onError={mockOnError} />)

    expect(mockOnError).toHaveBeenCalledWith('Test error')
  })
})
