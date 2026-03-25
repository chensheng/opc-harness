import { useState, useCallback } from 'react'
import {
  ChevronRight,
  ChevronDown,
  Folder,
  FolderOpen,
  File,
  FileCode,
  FileText,
} from 'lucide-react'
import type { FileNode } from '@/types'

export interface FileExplorerProps {
  /** 文件树数据 */
  fileTree: FileNode[]
  /** 当前选中的文件路径 */
  selectedFile?: string | null
  /** 文件选择回调 */
  onSelectFile?: (path: string) => void
  /** 文件夹展开/折叠状态变化回调 */
  onToggleFolder?: (path: string) => void
  /** 自定义类名 */
  className?: string
  /** 是否显示文件夹图标 */
  showFolderIcons?: boolean
  /** 是否显示文件图标 */
  showFileIcons?: boolean
  /** 缩进层级（像素） */
  indentSize?: number
}

/**
 * 根据文件扩展名获取对应的图标
 */
function getFileIcon(fileName: string) {
  const ext = fileName.split('.').pop()?.toLowerCase()
  switch (ext) {
    case 'ts':
    case 'tsx':
      return <FileCode className="w-4 h-4 text-blue-400" />
    case 'js':
    case 'jsx':
      return <FileCode className="w-4 h-4 text-yellow-400" />
    case 'rs':
      return <FileCode className="w-4 h-4 text-orange-400" />
    case 'json':
      return <FileCode className="w-4 h-4 text-green-400" />
    case 'md':
      return <FileText className="w-4 h-4 text-gray-400" />
    default:
      return <File className="w-4 h-4 text-gray-400" />
  }
}

/**
 * FileExplorer - 可复用的文件树组件
 *
 * 功能特性：
 * - 递归渲染无限层级目录
 * - 文件夹展开/折叠
 * - 文件选择高亮
 * - 自定义图标和样式
 *
 * @example
 * ```tsx
 * <FileExplorer
 *   fileTree={fileTree}
 *   selectedFile={selectedFile}
 *   onSelectFile={(path) => console.log('Selected:', path)}
 * />
 * ```
 */
export function FileExplorer({
  fileTree,
  selectedFile = null,
  onSelectFile,
  onToggleFolder,
  className = '',
  showFolderIcons = true,
  showFileIcons = true,
  indentSize = 16,
}: FileExplorerProps) {
  const [expandedFolders, setExpandedFolders] = useState<Set<string>>(() => {
    // 初始化时展开所有包含已展开标记的文件夹
    const initial = new Set<string>()
    fileTree.forEach(node => {
      if (node.type === 'directory' && node.isExpanded) {
        initial.add(node.path)
      }
    })
    return initial
  })

  const toggleFolder = useCallback(
    (path: string) => {
      setExpandedFolders(prev => {
        const next = new Set(prev)
        if (next.has(path)) {
          next.delete(path)
        } else {
          next.add(path)
        }
        return next
      })
      onToggleFolder?.(path)
    },
    [onToggleFolder]
  )

  const handleFileClick = useCallback(
    (path: string) => {
      onSelectFile?.(path)
    },
    [onSelectFile]
  )

  const renderNode = (node: FileNode, depth = 0) => {
    const isExpanded = expandedFolders.has(node.path) || node.isExpanded
    const isSelected = selectedFile === node.path

    if (node.type === 'directory') {
      return (
        <div key={node.path}>
          <button
            onClick={() => toggleFolder(node.path)}
            className={`flex items-center gap-1 w-full px-2 py-1 text-sm rounded hover:bg-accent transition-colors ${
              isSelected ? 'bg-accent' : ''
            }`}
            style={{ paddingLeft: depth * indentSize }}
            title={node.path}
          >
            {isExpanded ? (
              <ChevronDown className="w-4 h-4 flex-shrink-0" />
            ) : (
              <ChevronRight className="w-4 h-4 flex-shrink-0" />
            )}
            {showFolderIcons &&
              (isExpanded ? (
                <FolderOpen className="w-4 h-4 text-yellow-500 flex-shrink-0" />
              ) : (
                <Folder className="w-4 h-4 text-yellow-500 flex-shrink-0" />
              ))}
            <span className="truncate text-left">{node.name}</span>
          </button>
          {isExpanded && node.children && (
            <div>{node.children.map(child => renderNode(child, depth + 1))}</div>
          )}
        </div>
      )
    }

    // File node
    return (
      <button
        key={node.path}
        onClick={() => handleFileClick(node.path)}
        className={`flex items-center gap-1 w-full px-2 py-1 text-sm rounded hover:bg-accent transition-colors ${
          isSelected ? 'bg-accent' : ''
        }`}
        style={{ paddingLeft: depth * indentSize }}
        title={node.path}
      >
        {showFileIcons && <span className="flex-shrink-0">{getFileIcon(node.name)}</span>}
        {!showFileIcons && <span className="w-4 flex-shrink-0" />}
        <span className="truncate text-left">{node.name}</span>
      </button>
    )
  }

  return <div className={`space-y-1 ${className}`}>{fileTree.map(node => renderNode(node))}</div>
}
