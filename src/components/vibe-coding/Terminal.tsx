/**
 * Terminal Emulator Component (终端模拟器组件)
 *
 * 为 Vibe Coding 工作区提供命令执行和输出查看能力
 *
 * @module components/vibe-coding/Terminal
 */

import React, { useState, useRef, useEffect, useCallback } from 'react'

// ANSI 颜色代码映射
const ANSI_COLORS: Record<string, string> = {
  '0': 'var(--color-text)', // Reset
  '1': 'var(--color-primary)', // Bold/Bright
  '30': '#000000', // Black
  '31': '#ff5555', // Red
  '32': '#50fa7b', // Green
  '33': '#f1fa8c', // Yellow
  '34': '#bd93f9', // Blue
  '35': '#ff79c6', // Magenta
  '36': '#8be9fd', // Cyan
  '37': '#bbbbbb', // White
}

/**
 * 解析 ANSI 转义码并转换为 HTML
 */
function parseAnsiToHtml(text: string): string {
  let html = text.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;')

  // 处理 ANSI 颜色代码 (使用 Unicode 转义替代 \x1b)
  // eslint-disable-next-line no-control-regex
  html = html.replace(/\u001b\[([0-9;]*)m/g, (_match, codes) => {
    const codeList = codes.split(';')
    const colorCode = codeList.find((c: string) => c.startsWith('3'))

    if (colorCode && ANSI_COLORS[colorCode]) {
      return `<span style="color: ${ANSI_COLORS[colorCode]}">`
    }

    if (codes.includes('0')) {
      return '</span>'
    }

    return ''
  })

  // 处理换行
  html = html.replace(/\n/g, '<br/>')

  return html
}

/**
 * 终端输出类型
 */
export interface TerminalOutput {
  content: string
  type: 'stdout' | 'stderr' | 'info'
  timestamp: number
}

/**
 * 终端主题配置
 */
export interface TerminalTheme {
  background: string
  foreground: string
  cursor: string
  selection: string
  black: string
  red: string
  green: string
  yellow: string
  blue: string
  magenta: string
  cyan: string
  white: string
}

/**
 * Terminal 组件 Props
 */
export interface TerminalProps {
  /** 主题配置 */
  theme?: TerminalTheme
  /** 字体大小（像素） */
  fontSize?: number
  /** 最大历史记录数 */
  maxHistory?: number
  /** 命令执行后的回调 */
  onCommandExecuted?: (command: string, output: string) => void
  /** 自定义类名 */
  className?: string
  /** 是否自动聚焦 */
  autoFocus?: boolean
  /** 是否只读模式 */
  readOnly?: boolean
  /** 显示行号 */
  showLineNumbers?: boolean
}

/**
 * Terminal Handle 接口
 */
export interface TerminalHandle {
  /** 执行命令 */
  executeCommand(command: string): Promise<string>
  /** 清空终端 */
  clear(): void
  /** 聚焦终端 */
  focus(): void
  /** 获取命令历史 */
  getHistory(): string[]
}

/**
 * Terminal Emulator Component
 *
 * 一个功能完整的终端模拟器组件，支持：
 * - 命令输入和执行
 * - ANSI 颜色输出
 * - 命令历史记录
 * - 自动滚动
 * - 复制/粘贴
 *
 * @example
 * ```tsx
 * <Terminal
 *   cwd="/path/to/project"
 *   onCommandExecuted={(cmd, output) => console.log(output)}
 * />
 * ```
 */
export const Terminal: React.FC<TerminalProps> = ({
  theme,
  fontSize = 14,
  maxHistory = 100,
  onCommandExecuted,
  className = '',
  autoFocus = true,
  readOnly = false,
  showLineNumbers = false,
}) => {
  // 状态管理
  const [outputs, setOutputs] = useState<TerminalOutput[]>([])
  const [currentCommand, setCurrentCommand] = useState('')
  const [commandHistory, setCommandHistory] = useState<string[]>([])
  const [historyIndex, setHistoryIndex] = useState(-1)
  const [isExecuting, setIsExecuting] = useState(false)

  // Refs
  const inputRef = useRef<HTMLInputElement>(null)
  const outputRef = useRef<HTMLDivElement>(null)
  const historyRef = useRef<string[]>([])

  // 自动聚焦
  useEffect(() => {
    if (autoFocus && inputRef.current) {
      inputRef.current.focus()
    }
  }, [autoFocus])

  // 自动滚动到最新输出
  useEffect(() => {
    if (outputRef.current) {
      outputRef.current.scrollTop = outputRef.current.scrollHeight
    }
  }, [outputs])

  /**
   * 执行命令
   */
  const executeCommand = useCallback(
    async (command: string): Promise<string> => {
      if (!command.trim()) {
        return ''
      }

      setIsExecuting(true)

      try {
        // TODO: 集成真实的后端命令执行

        // 模拟命令执行（占位符）
        await new Promise(resolve => setTimeout(resolve, 100))

        const mockOutput = `$ ${command}\nCommand executed successfully\n`

        // 添加到输出列表
        const output: TerminalOutput = {
          content: mockOutput,
          type: 'stdout',
          timestamp: Date.now(),
        }

        setOutputs(prev => [...prev.slice(-(maxHistory - 1)), output])

        // 添加到命令历史
        setCommandHistory(prev => {
          const newHistory = [...prev, command].slice(-maxHistory)
          historyRef.current = newHistory
          return newHistory
        })
        setHistoryIndex(-1)

        // 回调
        onCommandExecuted?.(command, mockOutput)

        return mockOutput
      } catch (error) {
        const errorMessage = error instanceof Error ? error.message : 'Unknown error'
        const errorOutput: TerminalOutput = {
          content: `Error: ${errorMessage}`,
          type: 'stderr',
          timestamp: Date.now(),
        }

        setOutputs(prev => [...prev, errorOutput])
        throw error
      } finally {
        setIsExecuting(false)
      }
    },
    [maxHistory, onCommandExecuted]
  )

  /**
   * 处理命令提交
   */
  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()

    if (isExecuting || readOnly) {
      return
    }

    const command = currentCommand.trim()
    if (command) {
      await executeCommand(command)
      setCurrentCommand('')
    }
  }

  /**
   * 处理键盘事件
   */
  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    // 上箭头 - 上一条命令
    if (e.key === 'ArrowUp' && commandHistory.length > 0) {
      e.preventDefault()
      const newIndex =
        historyIndex === -1 ? commandHistory.length - 1 : Math.max(0, historyIndex - 1)
      setHistoryIndex(newIndex)
      setCurrentCommand(commandHistory[newIndex])
    }

    // 下箭头 - 下一条命令
    if (e.key === 'ArrowDown' && commandHistory.length > 0) {
      e.preventDefault()
      if (historyIndex === -1) return

      const newIndex = historyIndex + 1
      if (newIndex >= commandHistory.length) {
        setHistoryIndex(-1)
        setCurrentCommand('')
      } else {
        setHistoryIndex(newIndex)
        setCurrentCommand(commandHistory[newIndex])
      }
    }

    // Ctrl+C - 中断当前命令
    if (e.key === 'c' && e.ctrlKey) {
      e.preventDefault()
      setCurrentCommand('')
      const interruptOutput: TerminalOutput = {
        content: '^C',
        type: 'info',
        timestamp: Date.now(),
      }
      setOutputs(prev => [...prev, interruptOutput])
    }

    // Ctrl+L - 清屏
    if (e.key === 'l' && e.ctrlKey) {
      e.preventDefault()
      setOutputs([])
    }
  }

  /**
   * 清空终端
   */
  const clear = useCallback(() => {
    setOutputs([])
  }, [])

  /**
   * 聚焦终端
   */
  const focus = useCallback(() => {
    inputRef.current?.focus()
  }, [])

  /**
   * 获取命令历史
   */
  const getHistory = useCallback((): string[] => {
    return [...historyRef.current]
  }, [])

  // 暴露方法给父组件
  useEffect(() => {
    const _handle: TerminalHandle = {
      executeCommand,
      clear,
      focus,
      getHistory,
    }

    // 可以通过 ref 传递给父组件
    // terminalRef.current = _handle;
  }, [executeCommand, clear, focus, getHistory])

  // 默认主题
  const defaultTheme: TerminalTheme = {
    background: '#1e1e1e',
    foreground: '#cccccc',
    cursor: '#ffffff',
    selection: '#264f78',
    black: '#000000',
    red: '#cd3131',
    green: '#0dbc79',
    yellow: '#e5e510',
    blue: '#2472c8',
    magenta: '#bc3fbc',
    cyan: '#11a8cd',
    white: '#e5e5e5',
  }

  const activeTheme = { ...defaultTheme, ...theme }

  return (
    <div
      className={`terminal-container ${className}`}
      style={{
        backgroundColor: activeTheme.background,
        color: activeTheme.foreground,
        fontSize: `${fontSize}px`,
        fontFamily: 'Consolas, "Courier New", monospace',
        height: '100%',
        display: 'flex',
        flexDirection: 'column',
        overflow: 'hidden',
      }}
    >
      {/* 输出区域 */}
      <div
        ref={outputRef}
        style={{
          flex: 1,
          overflowY: 'auto',
          padding: '8px',
          whiteSpace: 'pre-wrap',
          wordBreak: 'break-all',
        }}
        onClick={() => inputRef.current?.focus()}
      >
        {outputs.map((output, index) => (
          <div
            key={index}
            style={{
              color: output.type === 'stderr' ? activeTheme.red : activeTheme.foreground,
              marginBottom: '4px',
            }}
            dangerouslySetInnerHTML={{ __html: parseAnsiToHtml(output.content) }}
          />
        ))}

        {/* 当前命令提示符 */}
        {!readOnly && (
          <form onSubmit={handleSubmit} style={{ display: 'flex', alignItems: 'center' }}>
            <span style={{ marginRight: '8px', color: activeTheme.green }}>
              {showLineNumbers ? `[${outputs.length + 1}]` : '$'}
            </span>
            <input
              ref={inputRef}
              type="text"
              value={currentCommand}
              onChange={e => setCurrentCommand(e.target.value)}
              onKeyDown={handleKeyDown}
              disabled={isExecuting || readOnly}
              autoComplete="off"
              autoCorrect="off"
              autoCapitalize="off"
              spellCheck={false}
              style={{
                flex: 1,
                background: 'transparent',
                border: 'none',
                color: activeTheme.foreground,
                fontSize: 'inherit',
                fontFamily: 'inherit',
                outline: 'none',
                caretColor: activeTheme.cursor,
              }}
            />
          </form>
        )}

        {/* 执行中指示器 */}
        {isExecuting && (
          <div style={{ color: activeTheme.yellow, marginTop: '4px' }}>Executing...</div>
        )}
      </div>

      {/* 状态栏 */}
      <div
        style={{
          borderTop: `1px solid ${activeTheme.selection}`,
          padding: '4px 8px',
          fontSize: '12px',
          color: activeTheme.white,
          display: 'flex',
          justifyContent: 'space-between',
        }}
      >
        <div>
          <span>Ready</span>
        </div>
        <div>
          <span>History: {commandHistory.length}</span>
          {isExecuting && (
            <span style={{ marginLeft: '16px', color: activeTheme.yellow }}>● Running</span>
          )}
        </div>
      </div>
    </div>
  )
}

export default Terminal
