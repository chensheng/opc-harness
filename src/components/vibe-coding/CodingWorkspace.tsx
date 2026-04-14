import { useState, useEffect, useRef, useCallback } from 'react'
import { useParams, useNavigate } from 'react-router-dom'
import {
  Play,
  Square,
  Send,
  FolderTree,
  FileCode,
  ExternalLink,
  RefreshCw,
  Users,
  Calendar,
  Bot,
} from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card } from '@/components/ui/card'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { Badge } from '@/components/ui/badge'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { useProjectStore } from '@/stores'
import { useUserStoryStore } from '@/stores/userStoryStore'
import type { FileNode, CLIOutputLine, UserStory } from '@/types'
import { FileExplorer } from './FileExplorer'
import { UserStoryManager } from './UserStoryManager'
import { SprintManager } from './SprintManager'
import { ProgressVisualization } from './ProgressVisualization'
import { AgentMonitor } from './AgentMonitor'
import { InitializerWorkflow } from './InitializerWorkflow'
import { LogTerminal } from './LogTerminal'
import { mockFileTree, mockCLIOutput, WorkspaceMode } from './CodingWorkspaceTypes'
import { prdToMarkdown } from './CodingWorkspaceUtils'

export function CodingWorkspace() {
  const { projectId } = useParams<{ projectId: string }>()
  const navigate = useNavigate()
  const {
    getProjectById,
    updateProjectStatus,
    updateProjectProgress,
    projects,
    setCurrentProject,
  } = useProjectStore()

  // 当 projectId 变化时，设置当前项目ID
  useEffect(() => {
    if (projectId) {
      setCurrentProject(projectId)
    }
  }, [projectId, setCurrentProject])

  const [fileTree, setFileTree] = useState<FileNode[]>(mockFileTree)
  const [selectedFile, setSelectedFile] = useState<string | null>(null)
  const [cliOutput, setCliOutput] = useState<CLIOutputLine[]>(mockCLIOutput)
  const [cliInput, setCliInput] = useState('')
  const [isRunning, setIsRunning] = useState(false)
  const [activeTab, setActiveTab] = useState('code')
  const [workspaceMode, setWorkspaceMode] = useState<WorkspaceMode>('sprints')
  const outputEndRef = useRef<HTMLDivElement>(null)

  // 如果没有 projectId，重定向到最近的项目
  useEffect(() => {
    if (!projectId && projects.length > 0) {
      // 获取最近的项目（按 updatedAt 排序）
      const sortedProjects = [...projects].sort(
        (a, b) => new Date(b.updatedAt).getTime() - new Date(a.updatedAt).getTime()
      )
      const mostRecentProject = sortedProjects[0]
      navigate(`/coding/${mostRecentProject.id}`, { replace: true })
    } else if (!projectId && projects.length === 0) {
      // 没有项目，重定向到首页
      navigate('/', { replace: true })
    }
  }, [projectId, projects, navigate])

  const project = projectId ? getProjectById(projectId) : undefined

  useEffect(() => {
    if (project && project.status === 'design') {
      updateProjectStatus(projectId!, 'coding')
      updateProjectProgress(projectId!, 50)
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [project])

  useEffect(() => {
    outputEndRef.current?.scrollIntoView({ behavior: 'smooth' })
  }, [cliOutput])

  const handleFileSelect = useCallback((path: string) => {
    setSelectedFile(path)
  }, [])

  const handleFolderToggle = useCallback(
    (path: string) => {
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
    },
    [fileTree]
  )

  const handleSendCommand = () => {
    if (!cliInput.trim()) return

    setCliOutput(prev => [
      ...prev,
      { type: 'input' as const, content: cliInput, timestamp: new Date().toLocaleTimeString() },
    ])

    // Simulate response
    setTimeout(() => {
      setCliOutput(prev => [
        ...prev,
        {
          type: 'stdout',
          content: `Executing: ${cliInput}`,
          timestamp: new Date().toLocaleTimeString(),
        },
        { type: 'stdout', content: 'Done!', timestamp: new Date().toLocaleTimeString() },
      ])
    }, 500)

    setCliInput('')
  }

  const handleStoriesGenerated = useCallback(
    (stories: UserStory[]) => {
      // 保存用户故事到数据库
      if (projectId && stories.length > 0) {
        // 使用 useUserStoryStore 的 setProjectStories 方法
        const userStoryStore = useUserStoryStore.getState()

        userStoryStore
          .setProjectStories(projectId, stories)
          .then(() => {
            // 重新加载以确认保存成功
            return userStoryStore.loadProjectStories(projectId)
          })
          .catch(() => {
            // 静默处理错误
          })
      }
    },
    [projectId]
  )

  const handleStartServer = () => {
    setIsRunning(!isRunning)
    if (!isRunning) {
      setCliOutput(prev => [
        ...prev,
        {
          type: 'stdout',
          content: '> Starting server...',
          timestamp: new Date().toLocaleTimeString(),
        },
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
      <div className="flex items-center justify-between mb-4 gap-4">
        <div className="flex items-center gap-4 flex-1">
          {/* Project Selector */}
          {projects.length > 1 && (
            <Select value={projectId} onValueChange={value => navigate(`/coding/${value}`)}>
              <SelectTrigger className="w-[250px]">
                <SelectValue placeholder="选择项目" />
              </SelectTrigger>
              <SelectContent>
                {projects.map(p => (
                  <SelectItem key={p.id} value={p.id}>
                    <div className="flex items-center gap-2">
                      <span>{p.name}</span>
                      <Badge variant="secondary" className="text-xs">
                        {p.status === 'idea'
                          ? '构思中'
                          : p.status === 'design'
                            ? '设计中'
                            : p.status === 'coding'
                              ? '开发中'
                              : p.status === 'marketing'
                                ? '运营中'
                                : '已完成'}
                      </Badge>
                    </div>
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          )}

          {/* Workspace Mode Switcher */}
          <Tabs
            value={workspaceMode}
            onValueChange={value => setWorkspaceMode(value as WorkspaceMode)}
            className="w-auto"
          >
            <TabsList className="h-9">
              <TabsTrigger value="sprints" className="flex items-center gap-2 text-sm">
                <Calendar className="w-4 h-4" />
                Sprint计划
              </TabsTrigger>
              <TabsTrigger value="stories" className="flex items-center gap-2 text-sm">
                <Users className="w-4 h-4" />
                用户故事
              </TabsTrigger>
              {/* <TabsTrigger value="coding" className="flex items-center gap-2 text-sm">
                <FileCode className="w-4 h-4" />
                代码工作区
              </TabsTrigger> */}
              <TabsTrigger value="agents" className="flex items-center gap-2 text-sm">
                <Bot className="w-4 h-4" />
                智能体
              </TabsTrigger>
            </TabsList>
          </Tabs>
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

      {/* Conditional Content Based on Workspace Mode */}
      {workspaceMode === 'coding' ? (
        <div className="flex-1 grid grid-cols-12 gap-4 min-h-0">
          {/* File Tree */}
          <Card className="col-span-3 overflow-hidden flex flex-col">
            <div className="p-3 border-b flex items-center gap-2">
              <FolderTree className="w-4 h-4" />
              <span className="text-sm font-medium">文件</span>
            </div>
            <div className="flex-1 overflow-auto p-2">
              <FileExplorer
                fileTree={fileTree}
                selectedFile={selectedFile}
                onSelectFile={handleFileSelect}
                onToggleFolder={handleFolderToggle}
              />
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
                <iframe src="about:blank" className="w-full h-full border-0" title="Preview" />
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
      ) : workspaceMode === 'stories' ? (
        /* User Story Management */
        <div className="flex-1 overflow-auto">
          <UserStoryManager
            prdContent={project?.prdMarkdown || (project?.prd ? prdToMarkdown(project.prd) : '')}
            onStoriesGenerated={handleStoriesGenerated}
          />
        </div>
      ) : workspaceMode === 'agents' ? (
        /* Agent Management */
        <div className="flex-1 overflow-auto">
          <AgentMonitor />
        </div>
      ) : (
        /* Sprint Plan Management */
        <div className="flex-1 overflow-auto">
          <SprintManager />
        </div>
      )}
    </div>
  )
}

// 导出其他组件供路由使用
export { ProgressVisualization, AgentMonitor, InitializerWorkflow, LogTerminal }
