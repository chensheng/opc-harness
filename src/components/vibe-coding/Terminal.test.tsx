/**
 * Terminal Component Tests
 */

import { render, screen, fireEvent, waitFor } from '@testing-library/react'
import { Terminal, type TerminalProps } from './Terminal'

describe('Terminal Component', () => {
  const defaultProps: TerminalProps = {
    cwd: '/test/path',
    autoFocus: false,
  }

  /**
   * 基础渲染测试
   */
  describe('Rendering', () => {
    it('should render terminal component', () => {
      render(<Terminal {...defaultProps} />)

      expect(screen.getByText('CWD: /test/path')).toBeInTheDocument()
    })

    it('should render with custom cwd', () => {
      render(<Terminal cwd="/custom/path" />)

      expect(screen.getByText('CWD: /custom/path')).toBeInTheDocument()
    })

    it('should apply custom className', () => {
      const { container } = render(<Terminal className="custom-terminal" />)

      expect(container.querySelector('.custom-terminal')).toBeInTheDocument()
    })
  })

  /**
   * 命令输入测试
   */
  describe('Command Input', () => {
    it('should accept command input', async () => {
      render(<Terminal {...defaultProps} autoFocus={true} />)

      const input = screen.getByRole('textbox') as HTMLInputElement
      fireEvent.change(input, { target: { value: 'ls -la' } })

      expect(input.value).toBe('ls -la')
    })

    it('should not accept input in readOnly mode', () => {
      render(<Terminal {...defaultProps} readOnly={true} />)

      const input = screen.queryByRole('textbox')
      expect(input).not.toBeInTheDocument()
    })
  })

  /**
   * 命令执行测试
   */
  describe('Command Execution', () => {
    it('should execute command on submit', async () => {
      const onCommandExecuted = jest.fn()
      render(<Terminal {...defaultProps} autoFocus={true} onCommandExecuted={onCommandExecuted} />)

      const input = screen.getByRole('textbox') as HTMLInputElement
      fireEvent.change(input, { target: { value: 'ls -la' } })
      fireEvent.submit(input.form!)

      await waitFor(() => {
        expect(onCommandExecuted).toHaveBeenCalledWith(
          'ls -la',
          expect.stringContaining('Command executed successfully')
        )
      })
    })

    it('should not execute empty command', async () => {
      const onCommandExecuted = jest.fn()
      render(<Terminal {...defaultProps} autoFocus={true} onCommandExecuted={onCommandExecuted} />)

      const input = screen.getByRole('textbox') as HTMLInputElement
      fireEvent.change(input, { target: { value: '   ' } })
      fireEvent.submit(input.form!)

      await waitFor(() => {
        expect(onCommandExecuted).not.toHaveBeenCalled()
      })
    })
  })

  /**
   * 快捷键测试
   */
  describe('Keyboard Shortcuts', () => {
    it('should handle Ctrl+C to interrupt', () => {
      render(<Terminal {...defaultProps} autoFocus={true} />)

      const input = screen.getByRole('textbox') as HTMLInputElement
      fireEvent.change(input, { target: { value: 'running command' } })
      fireEvent.keyDown(input, { key: 'c', ctrlKey: true })

      expect(input.value).toBe('')
      expect(screen.getByText('^C')).toBeInTheDocument()
    })
  })

  /**
   * 输出显示测试
   */
  describe('Output Display', () => {
    it('should display command output', async () => {
      render(<Terminal {...defaultProps} autoFocus={true} />)

      const input = screen.getByRole('textbox') as HTMLInputElement
      fireEvent.change(input, { target: { value: 'echo hello' } })
      fireEvent.submit(input.form!)

      await waitFor(() => {
        expect(screen.getByText(/Command executed successfully/)).toBeInTheDocument()
      })
    })
  })

  /**
   * 主题配置测试
   */
  describe('Theme Configuration', () => {
    it('should use default theme when not provided', () => {
      const { container } = render(<Terminal />)

      const terminalContainer = container.querySelector('.terminal-container')
      expect(terminalContainer).toHaveStyle('background-color: #1e1e1e')
    })
  })

  /**
   * 状态栏测试
   */
  describe('Status Bar', () => {
    it('should display current working directory', () => {
      render(<Terminal cwd="/project/src" />)

      expect(screen.getByText('CWD: /project/src')).toBeInTheDocument()
    })

    it('should display shell path when provided', () => {
      render(<Terminal shell="/bin/zsh" />)

      expect(screen.getByText('Shell: /bin/zsh')).toBeInTheDocument()
    })
  })
})
