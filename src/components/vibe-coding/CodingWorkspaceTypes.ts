import type { FileNode, CLIOutputLine } from '@/types'

/**
 * 里程碑接口
 */
export interface Milestone {
  id: string
  title: string
  progress: number
  totalTasks: number
  completedTasks: number
  status: 'pending' | 'in_progress' | 'completed'
  dueDate?: string
}

/**
 * 任务统计接口
 */
export interface TaskStats {
  total: number
  todo: number
  inProgress: number
  review: number
  done: number
}

/**
 * Agent 信息接口
 */
export interface AgentInfo {
  agentId: string
  type: 'initializer' | 'coding' | 'mr_creation'
  status: 'idle' | 'running' | 'paused' | 'completed' | 'failed' | 'stopped'
  currentTask?: string
  progress: number
  cpuUsage: number
  memoryUsage: number
  logs: string[]
  sessionId: string
}

/**
 * 初始化步骤接口
 */
export interface InitializerStep {
  id: string
  name: string
  description: string
  icon: React.ReactNode
  status: 'pending' | 'running' | 'completed' | 'failed'
  logs: string[]
  error?: string
}

/**
 * 日志条目接口
 */
export interface LogEntry {
  id: string
  timestamp: Date
  level: 'info' | 'warn' | 'error' | 'debug' | 'success'
  source: string
  message: string
  data?: unknown
}

/**
 * 日志统计接口
 */
export interface LogStats {
  total: number
  info: number
  warn: number
  error: number
  debug: number
  success: number
}

/**
 * 工作区模式
 */
export type WorkspaceMode = 'sprints' | 'stories' | 'coding'

/**
 * Mock 文件树数据
 */
export const mockFileTree: FileNode[] = [
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

/**
 * Mock CLI 输出数据
 */
export const mockCLIOutput: CLIOutputLine[] = [
  { type: 'stdout' as const, content: '> Starting development server...', timestamp: '10:00:01' },
  { type: 'stdout' as const, content: '> Ready on http://localhost:3000', timestamp: '10:00:03' },
  { type: 'stdout' as const, content: '> Compiling...', timestamp: '10:00:05' },
  { type: 'stdout' as const, content: '> Compiled successfully', timestamp: '10:00:08' },
]
