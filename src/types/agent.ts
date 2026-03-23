/**
 * Agent 通信协议类型定义
 * 
 * 用于 Vibe Coding 模块的 Agent 与守护进程、前端之间的通信
 */

/** Agent 生命周期阶段 */
export type AgentPhase = 'initializer' | 'coding' | 'mr_creation'

/** Agent 运行状态 */
export type AgentStatusType = 'idle' | 'running' | 'paused' | 'completed' | 'failed'

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
  messageId: string
  timestamp: number
  source: 'agent' | 'daemon' | 'frontend'
  type: MessageType
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
  disconnectWebSocket: () => void
  sendAgentRequest: (agentId: string, action: string, payload: unknown) => Promise<AgentResponse>
  subscribeAgent: (agentId: string) => void
  unsubscribeAgent: (agentId: string) => void
}
