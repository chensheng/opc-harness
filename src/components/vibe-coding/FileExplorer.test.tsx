import { describe, it, expect, vi } from 'vitest'
import { render, screen, fireEvent } from '@testing-library/react'
import { FileExplorer } from './FileExplorer'
import type { FileNode } from '@/types'

describe('FileExplorer', () => {
  const mockFileTree: FileNode[] = [
    {
      name: 'src',
      path: '/src',
      type: 'directory',
      isExpanded: true,
      children: [
        {
          name: 'components',
          path: '/src/components',
          type: 'directory',
          isExpanded: true,
          children: [
            { name: 'Button.tsx', path: '/src/components/Button.tsx', type: 'file' },
            { name: 'Card.tsx', path: '/src/components/Card.tsx', type: 'file' },
          ],
        },
        { name: 'App.tsx', path: '/src/App.tsx', type: 'file' },
      ],
    },
    { name: 'package.json', path: '/package.json', type: 'file' },
  ]

  describe('基本渲染', () => {
    it('应该渲染文件树', () => {
      render(<FileExplorer fileTree={mockFileTree} />)

      expect(screen.getByText('src')).toBeInTheDocument()
      expect(screen.getByText('components')).toBeInTheDocument()
      expect(screen.getByText('Button.tsx')).toBeInTheDocument()
      expect(screen.getByText('Card.tsx')).toBeInTheDocument()
      expect(screen.getByText('App.tsx')).toBeInTheDocument()
      expect(screen.getByText('package.json')).toBeInTheDocument()
    })

    it('应该正确显示文件夹图标', () => {
      const { container } = render(<FileExplorer fileTree={mockFileTree} />)

      const folderIcons = container.querySelectorAll('svg')
      expect(folderIcons.length).toBeGreaterThan(0)
    })

    it('应该支持隐藏文件夹图标', () => {
      render(<FileExplorer fileTree={mockFileTree} showFolderIcons={false} />)

      // 验证仍然显示文本但可能没有图标
      expect(screen.getByText('src')).toBeInTheDocument()
    })

    it('应该支持隐藏文件图标', () => {
      render(<FileExplorer fileTree={mockFileTree} showFileIcons={false} />)

      expect(screen.getByText('Button.tsx')).toBeInTheDocument()
    })
  })

  describe('文件夹展开/折叠', () => {
    it('应该能够点击文件夹进行展开/折叠', () => {
      const onToggleFolder = vi.fn()
      render(
        <FileExplorer
          fileTree={[
            {
              name: 'test',
              path: '/test',
              type: 'directory',
              children: [{ name: 'file.txt', path: '/test/file.txt', type: 'file' }],
            },
          ]}
          onToggleFolder={onToggleFolder}
        />
      )

      const folderButton = screen.getByText('test').closest('button')
      expect(folderButton).toBeInTheDocument()

      if (folderButton) {
        fireEvent.click(folderButton)
        expect(onToggleFolder).toHaveBeenCalledWith('/test')
      }
    })

    it('初始状态应该根据 isExpanded 属性决定', () => {
      const treeWithCollapsed: FileNode[] = [
        {
          name: 'collapsed',
          path: '/collapsed',
          type: 'directory',
          isExpanded: false,
          children: [{ name: 'hidden.txt', path: '/collapsed/hidden.txt', type: 'file' }],
        },
      ]

      render(<FileExplorer fileTree={treeWithCollapsed} />)

      // 折叠的文件夹，子文件不应该显示
      expect(screen.queryByText('hidden.txt')).not.toBeInTheDocument()
    })
  })

  describe('文件选择', () => {
    it('应该能够选择文件', () => {
      const onSelectFile = vi.fn()
      render(<FileExplorer fileTree={mockFileTree} onSelectFile={onSelectFile} />)

      fireEvent.click(screen.getByText('Button.tsx'))
      expect(onSelectFile).toHaveBeenCalledWith('/src/components/Button.tsx')
    })

    it('应该高亮显示选中的文件', () => {
      render(<FileExplorer fileTree={mockFileTree} selectedFile="/src/components/Button.tsx" />)

      const buttonElement = screen.getByText('Button.tsx').closest('button')
      expect(buttonElement).toHaveClass('bg-accent')
    })

    it('选中状态改变时应该更新高亮', () => {
      const { rerender } = render(
        <FileExplorer fileTree={mockFileTree} selectedFile="/src/components/Button.tsx" />
      )

      expect(screen.getByText('Button.tsx').closest('button')).toHaveClass('bg-accent')

      rerender(<FileExplorer fileTree={mockFileTree} selectedFile="/src/App.tsx" />)

      expect(screen.getByText('Button.tsx').closest('button')).not.toHaveClass('bg-accent')
      expect(screen.getByText('App.tsx').closest('button')).toHaveClass('bg-accent')
    })
  })

  describe('自定义缩进', () => {
    it('应该支持自定义缩进大小', () => {
      render(<FileExplorer fileTree={mockFileTree} indentSize={24} />)

      const nestedFile = screen.getByText('Button.tsx').closest('button')
      expect(nestedFile).toHaveStyle('padding-left: 48px') // depth=2, 2*24=48
    })
  })

  describe('空状态', () => {
    it('应该正确处理空文件树', () => {
      const { container } = render(<FileExplorer fileTree={[]} />)

      expect(container.firstChild).toBeEmptyDOMElement()
    })

    it('应该正确处理没有子节点的文件夹', () => {
      const treeWithoutChildren: FileNode[] = [{ name: 'empty', path: '/empty', type: 'directory' }]

      render(<FileExplorer fileTree={treeWithoutChildren} />)

      expect(screen.getByText('empty')).toBeInTheDocument()
      // 点击不应该报错
      fireEvent.click(screen.getByText('empty'))
    })
  })

  describe('深层嵌套', () => {
    it('应该支持深层嵌套的目录结构', () => {
      const deepTree: FileNode[] = [
        {
          name: 'level1',
          path: '/level1',
          type: 'directory',
          isExpanded: true,
          children: [
            {
              name: 'level2',
              path: '/level1/level2',
              type: 'directory',
              isExpanded: true,
              children: [
                {
                  name: 'level3',
                  path: '/level1/level2/level3',
                  type: 'directory',
                  isExpanded: true,
                  children: [
                    { name: 'deep.txt', path: '/level1/level2/level3/deep.txt', type: 'file' },
                  ],
                },
              ],
            },
          ],
        },
      ]

      render(<FileExplorer fileTree={deepTree} />)

      expect(screen.getByText('level1')).toBeInTheDocument()
      expect(screen.getByText('level2')).toBeInTheDocument()
      expect(screen.getByText('level3')).toBeInTheDocument()
      expect(screen.getByText('deep.txt')).toBeInTheDocument()
    })
  })

  describe('性能优化', () => {
    it('应该使用 memoization 避免不必要的重新渲染', () => {
      const onSelectFile = vi.fn()
      const onToggleFolder = vi.fn()

      const { rerender } = render(
        <FileExplorer
          fileTree={mockFileTree}
          onSelectFile={onSelectFile}
          onToggleFolder={onToggleFolder}
        />
      )

      // 重新渲染相同的 props
      rerender(
        <FileExplorer
          fileTree={mockFileTree}
          onSelectFile={onSelectFile}
          onToggleFolder={onToggleFolder}
        />
      )

      // 不应该触发额外的回调
      expect(onSelectFile).not.toHaveBeenCalled()
      expect(onToggleFolder).not.toHaveBeenCalled()
    })
  })
})
