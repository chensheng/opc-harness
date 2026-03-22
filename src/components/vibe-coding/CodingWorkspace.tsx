import { useState, useEffect, useRef } from 'react'
import { useParams, useNavigate } from 'react-router-dom'
import { 
  Play, 
  Square, 
  Send, 
  FolderTree, 
  FileCode, 
  ExternalLink,
  RefreshCw,
  ChevronRight,
  ChevronDown,
  File,
  Folder
} from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card, CardContent } from '@/components/ui/card'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { Badge } from '@/components/ui/badge'
import { useProjectStore, useAppStore } from '@/stores'
import type { FileNode, CLISession } from '@/types'

// Mock file tree
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
        children: [
          { name: 'Button.tsx', path: '/src/components/Button.tsx', type: 'file' },
          { name: 'Card.tsx', path: '/src/components/Card.tsx', type: 'file' },
        ],
      },
      { name: 'App.tsx', path: '/src/App.tsx', type: 'file' },
      { name: 'main.tsx', path: '/src/main.tsx', type: 'file' },
    ],
  },
  { name: 'package.json', path: '/package.json', type: 'file' },
  { name: 'README.md', path: '/README.md', type: 'file' },
]

// Mock CLI output
const mockCLIOutput = [
  { type: 'stdout' as const, content: '> Starting development server...', timestamp: '10:00:01' },
  { type: 'stdout' as const, content: '> Ready on http://localhost:3000', timestamp: '10:00:03' },
  { type: 'stdout' as const, content: '> Compiling...', timestamp: '10:00:05' },
  { type: 'stdout' as const, content: '> Compiled successfully', timestamp: '10:00:08' },
]

export function CodingWorkspace() {
  const { projectId } = useParams<{ projectId: string }>()
  const navigate = useNavigate()
  const { getProjectById, updateProjectStatus, updateProjectProgress } = useProjectStore()
  const { setLoading } = useAppStore()
  
  const [fileTree, setFileTree] = useState<FileNode[]>(mockFileTree)
  const [selectedFile, setSelectedFile] = useState<string | null>(null)
  const [cliOutput, setCliOutput] = useState<typeof mockCLIOutput>(mockCLIOutput)
  const [cliInput, setCliInput] = useState('')
  const [isRunning, setIsRunning] = useState(false)
  const [activeTab, setActiveTab] = useState('code')
  const outputEndRef = useRef<HTMLDivElement>(null)

  const project = projectId ? getProjectById(projectId) : undefined

  useEffect(() => {
    if (project && project.status === 'design') {
      updateProjectStatus(projectId!, 'coding')
      updateProjectProgress(projectId!, 50)
    }
  }, [project])

  useEffect(() => {
    outputEndRef.current?.scrollIntoView({ behavior: 'smooth' })
  }, [cliOutput])

  const toggleFolder = (path: string) => {
    const toggleNode = (nodes: FileNode[]): FileNode[] => {
      return nodes.map(node => {
        if (node.path === path) {
          return { ...node, isExpanded: !node.isExpanded }
        }
        if (node.children) {
          return { ...node, children: toggleNode(node.children) }
        }
        return node
      })
    }
    setFileTree(toggleNode(fileTree))
  }

  const renderFileTree = (nodes: FileNode[], depth = 0) => {
    return nodes.map(node => (
      <div key={node.path} style={{ paddingLeft: depth * 16 }}>
        <button
          onClick={() =>
            node.type === 'directory'
              ? toggleFolder(node.path)
              : setSelectedFile(node.path)
          }
          className={`flex items-center gap-1 w-full px-2 py-1 text-sm rounded hover:bg-accent ${
            selectedFile === node.path ? 'bg-accent' : ''
          }`}
        >
          {node.type === 'directory' ? (
            <>
              {node.isExpanded ? (
                <ChevronDown className="w-4 h-4" />
              ) : (
                <ChevronRight className="w-4 h-4" />
              )}
              <Folder className="w-4 h-4 text-yellow-500" />
            </>
          ) : (
            <>
              <span className="w-4" />
              <File className="w-4 h-4 text-blue-500" />
            </>
          )}
          <span className="truncate">{node.name}</span>
        </button>
        {node.type === 'directory' &&
          node.isExpanded &&
          node.children &&
          renderFileTree(node.children, depth + 1)}
      </div>
    ))
  }

  const handleSendCommand = () => {
    if (!cliInput.trim()) return
    
    setCliOutput(prev => [
      ...prev,
      { type: 'input', content: cliInput, timestamp: new Date().toLocaleTimeString() },
    ])
    
    // Simulate response
    setTimeout(() => {
      setCliOutput(prev => [
        ...prev,
        { type: 'stdout', content: `Executing: ${cliInput}`, timestamp: new Date().toLocaleTimeString() },
        { type: 'stdout', content: 'Done!', timestamp: new Date().toLocaleTimeString() },
      ])
    }, 500)
    
    setCliInput('')
  }

  const handleStartServer = () => {
    setIsRunning(!isRunning)
    if (!isRunning) {
      setCliOutput(prev => [
        ...prev,
        { type: 'stdout', content: '> Starting server...', timestamp: new Date().toLocaleTimeString() },
      ])
    }
  }

  if (!project) {
    return (
      <div className="text-center py-12">
        <p className="text-muted-foreground">项目不存在</p>
        <Button onClick={() => navigate('/')} className="mt-4">
          返回首页
        </Button>
      </div>
    )
  }

  return (
    <div className="h-[calc(100vh-8rem)] flex flex-col">
      <div className="flex items-center justify-between mb-4">
        <div>
          <h1 className="text-xl font-bold">💻 Vibe Coding</h1>
          <p className="text-sm text-muted-foreground">{project.name}</p>
        </div>
        <div className="flex gap-2">
          <Button variant="outline" size="sm" onClick={handleStartServer}>
            {isRunning ? (
              <>
                <Square className="w-4 h-4 mr-2" />
                停止
              </>
            ) : (
              <>
                <Play className="w-4 h-4 mr-2" />
                运行
              </>
            )}
          </Button>
          <Button variant="outline" size="sm">
            <ExternalLink className="w-4 h-4 mr-2" />
            预览
          </Button>
        </div>
      </div>

      <div className="flex-1 grid grid-cols-12 gap-4 min-h-0">
        {/* File Tree */}
        <Card className="col-span-3 overflow-hidden flex flex-col">
          <div className="p-3 border-b flex items-center gap-2">
            <FolderTree className="w-4 h-4" />
            <span className="text-sm font-medium">文件</span>
          </div>
          <div className="flex-1 overflow-auto p-2">
            {renderFileTree(fileTree)}
          </div>
        </Card>

        {/* Editor / Preview */}
        <Card className="col-span-6 overflow-hidden flex flex-col">
          <Tabs value={activeTab} onValueChange={setActiveTab} className="flex flex-col h-full">
            <div className="border-b px-3">
              <TabsList className="h-10">
                <TabsTrigger value="code" className="flex items-center gap-2">
                  <FileCode className="w-4 h-4" />
                  代码
                </TabsTrigger>
                <TabsTrigger value="preview" className="flex items-center gap-2">
                  <ExternalLink className="w-4 h-4" />
                  预览
                </TabsTrigger>
              </TabsList>
            </div>
            
            <TabsContent value="code" className="flex-1 m-0 p-4 overflow-auto">
              {selectedFile ? (
                <div className="font-mono text-sm">
                  <div className="text-muted-foreground mb-4">{selectedFile}</div>
                  <pre className="text-muted-foreground">
{`// Example code for ${selectedFile}
import React from 'react'

export function Component() {
  return (
    <div className="p-4">
      <h1>Hello World</h1>
    </div>
  )
}`}
                  </pre>
                </div>
              ) : (
                <div className="flex items-center justify-center h-full text-muted-foreground">
                  选择一个文件开始编辑
                </div>
              )}
            </TabsContent>
            
            <TabsContent value="preview" className="flex-1 m-0 p-0 overflow-hidden">
              <iframe
                src="about:blank"
                className="w-full h-full border-0"
                title="Preview"
              />
            </TabsContent>
          </Tabs>
        </Card>

        {/* CLI Console */}
        <Card className="col-span-3 overflow-hidden flex flex-col bg-slate-950">
          <div className="p-3 border-b border-slate-800 flex items-center justify-between">
            <div className="flex items-center gap-2">
              <Badge variant={isRunning ? 'default' : 'secondary'} className="text-xs">
                {isRunning ? '运行中' : '已停止'}
              </Badge>
            </div>
            <Button variant="ghost" size="icon" className="h-6 w-6">
              <RefreshCw className="w-3 h-3" />
            </Button>
          </div>
          
          <div className="flex-1 overflow-auto p-3 font-mono text-xs space-y-1">
            {cliOutput.map((line, index) => (
              <div
                key={index}
                className={`${
                  line.type === 'stderr'
                    ? 'text-red-400'
                    : line.type === 'input'
                    ? 'text-blue-400'
                    : 'text-slate-300'
                }`}
              >
                <span className="text-slate-600 mr-2">[{line.timestamp}]</span>
                {line.content}
              </div>
            ))}
            <div ref={outputEndRef} />
          </div>
          
          <div className="p-3 border-t border-slate-800 flex gap-2">
            <input
              type="text"
              value={cliInput}
              onChange={e => setCliInput(e.target.value)}
              onKeyDown={e => e.key === 'Enter' && handleSendCommand()}
              placeholder="输入命令..."
              className="flex-1 bg-slate-900 border-slate-700 rounded px-3 py-2 text-sm text-slate-200 outline-none focus:ring-1 focus:ring-primary"
            />
            <Button size="sm" onClick={handleSendCommand}>
              <Send className="w-4 h-4" />
            </Button>
          </div>
        </Card>
      </div>
    </div>
  )
}
