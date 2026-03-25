/**
 * CodeEditor 组件
 *
 * 基于 CodeMirror 6 的代码编辑器
 *
 * @author AI Agent
 * @date 2026-03-25
 */

import { useRef, useCallback, useMemo } from 'react'
import CodeMirror, { ReactCodeMirrorRef, EditorView, Extension } from '@uiw/react-codemirror'
import { javascript } from '@codemirror/lang-javascript'
import { rust } from '@codemirror/lang-rust'
import { html } from '@codemirror/lang-html'
import { css } from '@codemirror/lang-css'
import { oneDark } from '@codemirror/theme-one-dark'

/**
 * 支持的语言
 */
export type Language =
  | 'javascript'
  | 'typescript'
  | 'jsx'
  | 'tsx'
  | 'rust'
  | 'html'
  | 'css'
  | 'json'
  | 'markdown'
  | 'plaintext'

/**
 * CodeEditor 组件 Props
 */
export interface CodeEditorProps {
  /** 编辑器内容 */
  value: string
  /** 内容变化回调 */
  onChange?: (value: string) => void
  /** 编程语言 */
  language?: Language
  /** 是否只读 */
  readOnly?: boolean
  /** 是否显示行号 */
  showLineNumbers?: boolean
  /** 主题 */
  theme?: 'light' | 'dark'
  /** 最小高度 */
  minHeight?: string
  /** 最大高度 */
  maxHeight?: string
  /** 自定义扩展 */
  extensions?: Extension[]
  /** 编辑器类名 */
  className?: string
  /** 占位符 */
  placeholder?: string
  /** Tab 键行为 */
  indentWithTab?: boolean
}

/**
 * 根据文件扩展名推断语言
 */
export function getLanguageFromExtension(extension: string): Language {
  const ext = extension.toLowerCase()
  switch (ext) {
    case 'js':
    case 'mjs':
      return 'javascript'
    case 'ts':
      return 'typescript'
    case 'jsx':
      return 'jsx'
    case 'tsx':
      return 'tsx'
    case 'rs':
      return 'rust'
    case 'html':
    case 'htm':
      return 'html'
    case 'css':
    case 'scss':
    case 'sass':
    case 'less':
      return 'css'
    case 'json':
      return 'json'
    case 'md':
    case 'markdown':
      return 'plaintext'
    default:
      return 'plaintext'
  }
}

/**
 * 根据文件名推断语言
 */
export function getLanguageFromFileName(fileName: string): Language {
  const parts = fileName.split('.')
  if (parts.length > 1) {
    return getLanguageFromExtension(parts[parts.length - 1])
  }
  return 'plaintext'
}

/**
 * CodeEditor 组件
 */
export function CodeEditor({
  value,
  onChange,
  language = 'plaintext',
  readOnly = false,
  showLineNumbers = true,
  theme = 'dark',
  minHeight = '400px',
  maxHeight,
  extensions = [],
  className,
  placeholder,
  indentWithTab = true,
}: CodeEditorProps) {
  const editorRef = useRef<ReactCodeMirrorRef>(null)

  /**
   * 获取语言扩展
   */
  const languageExtensions = useMemo(() => {
    switch (language) {
      case 'javascript':
      case 'jsx':
        return [javascript({ jsx: language === 'jsx' })]
      case 'typescript':
      case 'tsx':
        return [javascript({ jsx: language === 'tsx', typescript: true })]
      case 'rust':
        return [rust()]
      case 'html':
        return [html()]
      case 'css':
        return [css()]
      case 'json':
        // JSON 使用 javascript 语法高亮
        return [javascript()]
      default:
        return []
    }
  }, [language])

  /**
   * 处理内容变化
   */
  const handleChange = useCallback(
    (val: string) => {
      if (!readOnly && onChange) {
        onChange(val)
      }
    },
    [readOnly, onChange]
  )

  /**
   * 构建扩展列表
   */
  const allExtensions = useMemo(() => {
    const exts: Extension[] = [...languageExtensions, ...extensions]

    // 添加只读模式
    if (readOnly) {
      exts.push(EditorView.editable.of(false))
    }

    // 添加主题
    if (theme === 'dark') {
      exts.push(oneDark)
    }

    return exts
  }, [languageExtensions, extensions, readOnly, theme])

  return (
    <div className={`code-editor-container ${className || ''}`} style={{ minHeight, maxHeight }}>
      <CodeMirror
        ref={editorRef}
        value={value}
        height="100%"
        theme={theme === 'dark' ? oneDark : undefined}
        extensions={allExtensions}
        onChange={handleChange}
        editable={!readOnly}
        basicSetup={{
          lineNumbers: showLineNumbers,
          foldGutter: true,
          dropCursor: true,
          allowMultipleSelections: true,
          indentOnInput: true,
          syntaxHighlighting: true,
          bracketMatching: true,
          closeBrackets: true,
          autocompletion: true,
          rectangularSelection: true,
          crosshairCursor: true,
          highlightActiveLineGutter: true,
          highlightSpecialChars: true,
          highlightSelectionMatches: true,
        }}
        placeholder={placeholder}
        indentWithTab={indentWithTab}
        className="w-full h-full"
      />
    </div>
  )
}

/**
 * 默认导出
 */
export default CodeEditor
