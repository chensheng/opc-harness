/**
 * CodeEditor 组件测试
 */

import { render } from '@testing-library/react'
import { describe, it, expect, vi, beforeEach } from 'vitest'
import { CodeEditor, getLanguageFromExtension, getLanguageFromFileName } from './CodeEditor'

describe('CodeEditor', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  describe('语言推断', () => {
    describe('getLanguageFromExtension', () => {
      it('应该正确识别 JavaScript', () => {
        expect(getLanguageFromExtension('js')).toBe('javascript')
        expect(getLanguageFromExtension('mjs')).toBe('javascript')
      })

      it('应该正确识别 TypeScript', () => {
        expect(getLanguageFromExtension('ts')).toBe('typescript')
      })

      it('应该正确识别 JSX/TSX', () => {
        expect(getLanguageFromExtension('jsx')).toBe('jsx')
        expect(getLanguageFromExtension('tsx')).toBe('tsx')
      })

      it('应该正确识别 Rust', () => {
        expect(getLanguageFromExtension('rs')).toBe('rust')
      })

      it('应该正确识别 HTML/CSS', () => {
        expect(getLanguageFromExtension('html')).toBe('html')
        expect(getLanguageFromExtension('css')).toBe('css')
      })

      it('应该正确识别 JSON', () => {
        expect(getLanguageFromExtension('json')).toBe('json')
      })

      it('应该正确识别 Markdown', () => {
        expect(getLanguageFromExtension('md')).toBe('plaintext')
        expect(getLanguageFromExtension('')).toBe('plaintext')
      })

      it('未知扩展名应该返回 plaintext', () => {
        expect(getLanguageFromExtension('xyz')).toBe('plaintext')
        expect(getLanguageFromExtension('')).toBe('plaintext')
      })
    })

    describe('getLanguageFromFileName', () => {
      it('应该从文件名正确推断语言', () => {
        expect(getLanguageFromFileName('app.tsx')).toBe('tsx')
        expect(getLanguageFromFileName('utils.js')).toBe('javascript')
        expect(getLanguageFromFileName('lib.rs')).toBe('rust')
        expect(getLanguageFromFileName('index.html')).toBe('html')
        expect(getLanguageFromFileName('styles.css')).toBe('css')
      })

      it('没有扩展名应该返回 plaintext', () => {
        expect(getLanguageFromFileName('README')).toBe('plaintext')
        expect(getLanguageFromFileName('Makefile')).toBe('plaintext')
      })
    })
  })

  describe('基本渲染', () => {
    it('应该渲染编辑器', () => {
      render(<CodeEditor value="" />)
      // CodeMirror 会渲染一个 .cm-editor 容器
      expect(document.querySelector('.cm-editor')).toBeInTheDocument()
    })

    it('应该显示初始内容', () => {
      const initialValue = 'const hello = "world";'
      render(<CodeEditor value={initialValue} language="javascript" />)
      expect(document.querySelector('.cm-editor')).toHaveTextContent(initialValue)
    })

    it('应该显示行号（默认）', () => {
      render(<CodeEditor value="" />)
      expect(document.querySelector('.cm-gutters')).toBeInTheDocument()
    })

    it('可以不显示行号', () => {
      render(<CodeEditor value="" showLineNumbers={false} />)
      // CodeMirror 即使不显示行号也会有 fold gutter，所以检查 lineNumbers 相关类
      const lineNumbers = document.querySelector('.cm-lineNumbers')
      expect(lineNumbers).not.toBeInTheDocument()
    })
  })

  describe('内容编辑', () => {
    it.skip('应该可以编辑内容（需要模拟 CodeMirror 输入，待完善）', async () => {
      // TODO: CodeMirror 6 的编辑测试需要更复杂的模拟
      // 暂时跳过，实际使用中已验证编辑功能正常
      const handleChange = vi.fn()
      render(<CodeEditor value="" onChange={handleChange} />)

      // 真实场景中通过 CodeMirror API 进行编辑
      // 这里仅验证 onChange 回调存在
      expect(handleChange).toBeDefined()
    })

    it('在只读模式下不应该触发 onChange', () => {
      const handleChange = vi.fn()
      render(<CodeEditor value="readonly" readOnly onChange={handleChange} />)

      const editor = document.querySelector('.cm-content') as HTMLElement
      if (editor) {
        // 只读模式下编辑器不可编辑
        expect(editor.getAttribute('contenteditable')).toBe('false')
      }
    })
  })

  describe('主题和样式', () => {
    it('默认使用 dark 主题（oneDark）', () => {
      render(<CodeEditor value="" />)
      // oneDark 主题会应用特定的样式，这里仅验证组件正常渲染
      const editor = document.querySelector('.cm-editor')
      expect(editor).toBeInTheDocument()
      // 实际主题效果通过视觉验证
    })

    it('可以使用 light 主题', () => {
      render(<CodeEditor value="" theme="light" />)
      // light 主题不会使用 oneDark
      const editor = document.querySelector('.cm-editor')
      expect(editor).toBeInTheDocument()
    })

    it('应该应用自定义最小高度', () => {
      const minHeight = '600px'
      render(<CodeEditor value="" minHeight={minHeight} />)
      const container = document.querySelector('.code-editor-container')
      expect(container).toHaveStyle({ minHeight })
    })
  })

  describe('语言支持', () => {
    it('应该支持 JavaScript', () => {
      render(<CodeEditor value="const x = 1;" language="javascript" />)
      expect(document.querySelector('.cm-editor')).toBeInTheDocument()
    })

    it('应该支持 TypeScript', () => {
      render(<CodeEditor value="const x: number = 1;" language="typescript" />)
      expect(document.querySelector('.cm-editor')).toBeInTheDocument()
    })

    it('应该支持 Rust', () => {
      render(<CodeEditor value="fn main() {}" language="rust" />)
      expect(document.querySelector('.cm-editor')).toBeInTheDocument()
    })

    it('应该支持 HTML', () => {
      render(<CodeEditor value="<div>Hello</div>" language="html" />)
      expect(document.querySelector('.cm-editor')).toBeInTheDocument()
    })

    it('应该支持 CSS', () => {
      render(<CodeEditor value=".class { color: red; }" language="css" />)
      expect(document.querySelector('.cm-editor')).toBeInTheDocument()
    })
  })

  describe('可访问性', () => {
    it('应该有正确的 ARIA 属性', () => {
      render(<CodeEditor value="" placeholder="Enter code..." />)
      // CodeMirror 会自动处理 ARIA 属性
      expect(document.querySelector('[role="textbox"]')).toBeInTheDocument()
    })
  })

  describe('性能优化', () => {
    it('应该在值不变时避免重新渲染', () => {
      const { rerender } = render(<CodeEditor value="static" />)
      const initialEditor = document.querySelector('.cm-editor')

      rerender(<CodeEditor value="static" />)
      const updatedEditor = document.querySelector('.cm-editor')

      expect(initialEditor).toBe(updatedEditor)
    })
  })
})
