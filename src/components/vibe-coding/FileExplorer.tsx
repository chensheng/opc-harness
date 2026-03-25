import { useState } from 'react'
import {
  FolderTree,
  Folder,
  File,
  FileCode,
  FileText,
  Plus,
  RefreshCw,
  ChevronRight,
  Edit3,
  Trash2,
} from 'lucide-react'
import { Button } from '@/components/ui/button'

interface FileTreeNode {
  id: string
  name: string
  type: 'file' | 'folder'
  path: string
  children?: FileTreeNode[]
  expanded?: boolean
  selected?: boolean
}

interface FileTreeProps {
  files: FileTreeNode[]
  onFileSelect?: (node: FileTreeNode) => void
  onFolderExpand?: (node: FileTreeNode) => void
  onContextMenu?: (node: FileTreeNode, event: React.MouseEvent) => void
}

function FileTreeNodeItem({
  node,
  depth = 0,
  onFileSelect,
  onFolderExpand,
  onContextMenu,
}: {
  node: FileTreeNode
  depth?: number
  onFileSelect?: (node: FileTreeNode) => void
  onFolderExpand?: (node: FileTreeNode) => void
  onContextMenu?: (node: FileTreeNode, event: React.MouseEvent) => void
}) {
  const [isExpanded, setIsExpanded] = useState(node.expanded ?? false)

  const handleToggle = (e: React.MouseEvent) => {
    e.stopPropagation()
    if (node.type === 'folder') {
      const newExpanded = !isExpanded
      setIsExpanded(newExpanded)
      onFolderExpand?.({ ...node, expanded: newExpanded })
    } else {
      onFileSelect?.(node)
    }
  }

  const handleContextMenu = (e: React.MouseEvent) => {
    e.preventDefault()
    e.stopPropagation()
    onContextMenu?.(node, e)
  }

  const getIcon = () => {
    if (node.type === 'folder') {
      return isExpanded ? (
        <Folder className="w-4 h-4 text-blue-500" />
      ) : (
        <Folder className="w-4 h-4 text-yellow-500" />
      )
    }

    // File icons based on extension
    const ext = node.name.split('.').pop()?.toLowerCase()
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

  return (
    <div>
      <div
        className={`flex items-center gap-1 px-2 py-1 cursor-pointer hover:bg-accent rounded transition-colors ${
          node.selected ? 'bg-accent' : ''
        }`}
        style={{ paddingLeft: `${depth * 12 + 8}px` }}
        onClick={handleToggle}
        onContextMenu={handleContextMenu}
      >
        {node.type === 'folder' && (
          <ChevronRight
            className={`w-3 h-3 text-muted-foreground transition-transform ${
              isExpanded ? 'rotate-90' : ''
            }`}
          />
        )}
        {getIcon()}
        <span className="text-sm">{node.name}</span>
      </div>
      {node.type === 'folder' && isExpanded && node.children && (
        <div>
          {node.children.map(child => (
            <FileTreeNodeItem
              key={child.id}
              node={child}
              depth={depth + 1}
              onFileSelect={onFileSelect}
              onFolderExpand={onFolderExpand}
              onContextMenu={onContextMenu}
            />
          ))}
        </div>
      )}
    </div>
  )
}

function FileTree({ files, onFileSelect, onFolderExpand, onContextMenu }: FileTreeProps) {
  return (
    <div className="h-full overflow-auto">
      <div className="p-2 space-y-1">
        {files.map(file => (
          <FileTreeNodeItem
            key={file.id}
            node={file}
            onFileSelect={onFileSelect}
            onFolderExpand={onFolderExpand}
            onContextMenu={onContextMenu}
          />
        ))}
      </div>
    </div>
  )
}

export function FileExplorer() {
  // Mock file tree data - will be replaced with real data from Backend
  const [fileTree] = useState<FileTreeNode[]>([
    {
      id: '1',
      name: 'opc-harness',
      type: 'folder',
      path: '/opc-harness',
      expanded: true,
      children: [
        {
          id: '2',
          name: 'src',
          type: 'folder',
          path: '/opc-harness/src',
          expanded: true,
          children: [
            {
              id: '3',
              name: 'components',
              type: 'folder',
              path: '/opc-harness/src/components',
              children: [
                {
                  id: '4',
                  name: 'vibe-coding',
                  type: 'folder',
                  path: '/opc-harness/src/components/vibe-coding',
                  children: [
                    {
                      id: '5',
                      name: 'CodingWorkspace.tsx',
                      type: 'file',
                      path: '/opc-harness/src/components/vibe-coding/CodingWorkspace.tsx',
                    },
                    {
                      id: '6',
                      name: 'CheckpointReview.tsx',
                      type: 'file',
                      path: '/opc-harness/src/components/vibe-coding/CheckpointReview.tsx',
                    },
                    {
                      id: '7',
                      name: 'FileExplorer.tsx',
                      type: 'file',
                      path: '/opc-harness/src/components/vibe-coding/FileExplorer.tsx',
                    },
                  ],
                },
              ],
            },
            {
              id: '8',
              name: 'App.tsx',
              type: 'file',
              path: '/opc-harness/src/App.tsx',
            },
            {
              id: '9',
              name: 'main.tsx',
              type: 'file',
              path: '/opc-harness/src/main.tsx',
            },
          ],
        },
        {
          id: '10',
          name: 'src-tauri',
          type: 'folder',
          path: '/opc-harness/src-tauri',
          expanded: false,
          children: [
            {
              id: '11',
              name: 'src',
              type: 'folder',
              path: '/opc-harness/src-tauri/src',
              children: [
                {
                  id: '12',
                  name: 'main.rs',
                  type: 'file',
                  path: '/opc-harness/src-tauri/src/main.rs',
                },
                {
                  id: '13',
                  name: 'lib.rs',
                  type: 'file',
                  path: '/opc-harness/src-tauri/src/lib.rs',
                },
              ],
            },
            {
              id: '14',
              name: 'Cargo.toml',
              type: 'file',
              path: '/opc-harness/src-tauri/Cargo.toml',
            },
          ],
        },
        {
          id: '15',
          name: 'package.json',
          type: 'file',
          path: '/opc-harness/package.json',
        },
        {
          id: '16',
          name: 'tsconfig.json',
          type: 'file',
          path: '/opc-harness/tsconfig.json',
        },
        {
          id: '17',
          name: 'README.md',
          type: 'file',
          path: '/opc-harness/README.md',
        },
      ],
    },
  ])

  const [selectedFile, setSelectedFile] = useState<FileTreeNode | null>(null)
  const [contextMenuNode, setContextMenuNode] = useState<FileTreeNode | null>(null)
  const [contextMenuPosition, setContextMenuPosition] = useState({ x: 0, y: 0 })

  const handleFileSelect = (node: FileTreeNode) => {
    setSelectedFile(node)
    console.log('Selected file:', node.path)
    // TODO: Open file in editor
  }

  const handleFolderExpand = (node: FileTreeNode) => {
    console.log('Folder expanded/collapsed:', node.path)
  }

  const handleContextMenu = (node: FileTreeNode, event: React.MouseEvent) => {
    event.preventDefault()
    setContextMenuNode(node)
    setContextMenuPosition({ x: event.clientX, y: event.clientY })
  }

  const closeContextMenu = () => {
    setContextMenuNode(null)
  }

  const handleNewFile = () => {
    console.log('Create new file')
    closeContextMenu()
  }

  const handleNewFolder = () => {
    console.log('Create new folder')
    closeContextMenu()
  }

  const handleRename = () => {
    console.log('Rename:', contextMenuNode?.name)
    closeContextMenu()
  }

  const handleDelete = () => {
    console.log('Delete:', contextMenuNode?.name)
    closeContextMenu()
  }

  return (
    <div className="h-full flex flex-col">
      {/* Header */}
      <div className="flex items-center justify-between p-3 border-b">
        <div className="flex items-center gap-2">
          <FolderTree className="w-4 h-4 text-blue-500" />
          <span className="font-semibold text-sm">项目文件</span>
        </div>
        <div className="flex gap-1">
          <Button variant="ghost" size="sm" onClick={handleNewFile}>
            <Plus className="w-3 h-3" />
          </Button>
          <Button variant="ghost" size="sm" onClick={handleNewFolder}>
            <Plus className="w-3 h-3" />
          </Button>
          <Button variant="ghost" size="sm">
            <RefreshCw className="w-3 h-3" />
          </Button>
        </div>
      </div>

      {/* File Tree */}
      <div className="flex-1 overflow-auto">
        <FileTree
          files={fileTree}
          onFileSelect={handleFileSelect}
          onFolderExpand={handleFolderExpand}
          onContextMenu={handleContextMenu}
        />
      </div>

      {/* Context Menu */}
      {contextMenuNode && (
        <div
          className="fixed z-50 bg-popover border rounded-md shadow-lg py-1 min-w-[160px]"
          style={{
            left: Math.min(contextMenuPosition.x, window.innerWidth - 200),
            top: Math.min(contextMenuPosition.y, window.innerHeight - 200),
          }}
        >
          <div className="px-3 py-2 text-xs font-semibold border-b">{contextMenuNode.name}</div>
          <button
            className="w-full text-left px-3 py-2 text-sm hover:bg-accent flex items-center gap-2"
            onClick={handleNewFile}
          >
            <Plus className="w-3 h-3" />
            新建文件
          </button>
          <button
            className="w-full text-left px-3 py-2 text-sm hover:bg-accent flex items-center gap-2"
            onClick={handleNewFolder}
          >
            <Folder className="w-3 h-3" />
            新建文件夹
          </button>
          <div className="border-t my-1" />
          <button
            className="w-full text-left px-3 py-2 text-sm hover:bg-accent flex items-center gap-2"
            onClick={handleRename}
          >
            <Edit3 className="w-3 h-3" />
            重命名
          </button>
          <button
            className="w-full text-left px-3 py-2 text-sm hover:bg-accent flex items-center gap-2 text-destructive"
            onClick={handleDelete}
          >
            <Trash2 className="w-3 h-3" />
            删除
          </button>
        </div>
      )}

      {/* Selected File Info */}
      {selectedFile && (
        <div className="border-t p-3 text-xs text-muted-foreground">
          <div className="flex items-center gap-2">
            {selectedFile.type === 'folder' ? (
              <Folder className="w-3 h-3" />
            ) : (
              <File className="w-3 h-3" />
            )}
            <span>{selectedFile.path}</span>
          </div>
        </div>
      )}
    </div>
  )
}
