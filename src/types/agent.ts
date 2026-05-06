/**
 * Agent 通信协议类型定义
 *
 * 用于 Vibe Coding 模块的 Agent 与守护进程、前端之间的通信
 */

/** Agent 生命周期阶段 */
export type AgentPhase = 'initializer' | 'coding' | 'mr_creation'

/** Agent 运行状态 */
export type AgentStatusType = 'idle' | 'running' | 'completed' | 'failed'

/** Agent 配置信息 */
export interface AgentConfig {
  agentId: string
  type: 'initializer' | 'coding' | 'mr_creation'
  phase: AgentPhase
  status: AgentStatusType
  projectPath: string
  sessionId: string
}

/** Agent 请求消息 */
export interface AgentRequest {
  requestId: string
  agentId: string
  action: string
  payload: unknown
}

/** Agent 响应消息 */
export interface AgentResponse {
  responseId: string
  requestId: string
  success: boolean
  data?: unknown
  error?: string
}

/** 消息类型 */
export type MessageType = 'log' | 'status' | 'progress' | 'error' | 'heartbeat'

/** Agent 消息 (用于实时推送) */
export interface AgentMessage {
  id?: string // 消息唯一标识（来自 WebSocket）
  messageId: string
  sessionId?: string // Session ID（来自 WebSocket）
  timestamp: number | string // 支持数字时间戳和 ISO 字符串
  source?: 'agent' | 'daemon' | 'frontend' // 可选，WebSocket 消息可能没有
  type: MessageType | 'response' | 'status' | 'progress' | 'log' | 'error' // 扩展类型以支持 WebSocket
  content: string
  metadata?: Record<string, unknown>
}

/** 守护进程状态快照 */
export interface DaemonState {
  sessionId: string
  projectId: string
  currentPhase: AgentPhase
  activeAgents: AgentStatusType[]
  completedIssues: string[]
  pendingIssues: string[]
  logFile?: string
  lastSnapshot: number
  cpuUsage: number
  memoryUsage: number
}

/** WebSocket 消息基础接口 */
export interface WebSocketMessage {
  type: 'connect' | 'disconnect' | 'message' | 'heartbeat' | 'subscribe' | 'unsubscribe'
  data?: unknown
}

/** Stdio 管道命令 */
export interface StdioCommand {
  commandId: string
  command: string
  args: string[]
  cwd?: string
  env?: Record<string, string>
}

/** Stdio 输出行 */
export interface StdioOutput {
  stdout?: string
  stderr?: string
  exitCode?: number
  timestamp: number
}

/** Agent Hook 返回值 */
export interface UseAgentReturn {
  agents: AgentConfig[]
  messages: AgentMessage[]
  daemonState: DaemonState | null
  isLoading: boolean
  error: string | null
  connectWebSocket: (sessionId: string) => Promise<void>
  disconnectWebSocket: () => void | Promise<void>
  sendAgentRequest: (agentId: string, action: string, payload: unknown) => Promise<AgentResponse>
  subscribeAgent: (agentId: string) => void
  unsubscribeAgent: (agentId: string) => void
  clearMessages?: () => void // 新增：清空消息列表
}

/**
 * 守护进程运行状态
 */
export type DaemonStatusType = 'starting' | 'running' | 'paused' | 'stopping' | 'stopped' | 'failed'

/**
 * 资源使用情况
 */
export interface ResourceUsage {
  cpuPercent: number // CPU 使用率 (%)
  memoryMb: number // 内存使用量 (MB)
  diskIoRead: number // 磁盘读取 (bytes)
  diskIoWrite: number // 磁盘写入 (bytes)
  networkRx: number // 网络接收 (bytes)
  networkTx: number // 网络发送 (bytes)
}

/**
 * Agent 进程信息
 */
export interface AgentProcessInfo {
  agentId: string // Agent 唯一标识
  agentType: string // Agent 类型
  pid?: number // 进程 ID
  status: AgentStatusType // 运行状态
  startedAt: number // 启动时间戳
  resourceUsage: ResourceUsage // 资源使用情况
}

/**
 * 系统信息
 */
export interface SystemInfo {
  os: string // 操作系统
  arch: string // 架构
  totalMemory: number // 总内存 (MB)
  availableMemory: number // 可用内存 (MB)
  cpuCores: number // CPU 核心数
  rustVersion: string // Rust 版本
}

/**
 * 守护进程配置
 */
export interface DaemonConfig {
  sessionId: string // 会话 ID
  projectPath: string // 项目路径
  logLevel: string // 日志级别
  maxConcurrentAgents: number // 最大并发 Agent 数
  workspaceDir: string // 工作目录
}

/**
 * 守护进程快照
 */
export interface DaemonSnapshot {
  daemonId: string // 守护进程 ID
  status: DaemonStatusType // 运行状态
  config: DaemonConfig // 配置信息
  activeAgents: AgentProcessInfo[] // 活跃的 Agent 列表
  completedTasks: string[] // 已完成的任务列表
  pendingTasks: string[] // 待处理的任务列表
  startTime: number // 启动时间戳
  lastUpdate: number // 最后更新时间戳
  systemInfo: SystemInfo // 系统信息
}

/**
 * 守护进程管理 Hook 返回值
 */
export interface UseDaemonReturn {
  snapshot: DaemonSnapshot | null
  status: DaemonStatusType | null
  isLoading: boolean
  error: string | null
  startDaemon: (config: Partial<DaemonConfig>) => Promise<void>
  stopDaemon: (graceful?: boolean) => Promise<void>
  pauseDaemon: () => Promise<void>
  resumeDaemon: () => Promise<void>
  spawnAgent: (agentType: string) => Promise<string>
  killAgent: (agentId: string) => Promise<void>
  refreshSnapshot: () => Promise<void>
}
