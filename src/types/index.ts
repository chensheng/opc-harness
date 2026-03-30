// AI Provider Types
export interface AIProvider {
  id: string
  name: string
  baseUrl: string
  models: AIModel[]
}

export interface AIModel {
  id: string
  name: string
  maxTokens: number
  description?: string
}

export interface AIConfig {
  provider: string
  model: string
  apiKey: string
  temperature?: number
  maxTokens?: number
  streaming?: boolean
}

export interface Message {
  role: 'system' | 'user' | 'assistant'
  content: string
}

// Streaming Types
export interface StreamChunk {
  session_id: string
  content: string
  is_complete: boolean
}

export interface StreamComplete {
  session_id: string
  content: string
}

export interface StreamError {
  session_id: string
  error: string
}

// Project Types
export interface Project {
  id: string
  name: string
  description: string
  status: ProjectStatus
  progress: number
  createdAt: string
  updatedAt: string
  idea?: string
  prd?: PRD
  userPersonas?: UserPersona[]
  competitorAnalysis?: CompetitorAnalysis
  techStack?: string[]
  estimatedEffort?: string
}

export type ProjectStatus = 'idea' | 'design' | 'coding' | 'marketing' | 'completed'

export interface PRD {
  title: string
  overview: string
  targetUsers: string[]
  coreFeatures: string[]
  techStack: string[]
  estimatedEffort: string
  businessModel?: string
  pricing?: string
}

// ========== PRD 迭代优化相关类型 ==========

/**
 * 用户反馈
 */
export interface UserFeedback {
  version: number // 反馈对应的版本号
  feedback: string // 反馈内容
  timestamp: Date // 反馈时间
}

/**
 * PRD 版本
 */
export interface PRDVersion {
  version: number // 版本号（从 1 开始）
  prd: PRD // 该版本的 PRD 内容
  feedback?: UserFeedback // 产生该版本的反馈意见（如果有）
  createdAt: Date // 创建时间
  changes?: string[] // 相对于上一版本的变更说明
}

/**
 * PRD 迭代历史
 */
export interface PRDIterationHistory {
  currentVersion: number // 当前版本号
  versions: PRDVersion[] // 所有版本列表
  maxVersions: number // 最大保留版本数（默认 10）
}

export interface UserPersona {
  id: string
  name: string
  age: string
  occupation: string
  background: string
  goals: string[]
  painPoints: string[]
  behaviors: string[]
  quote?: string
}

export interface CompetitorAnalysis {
  competitors: Competitor[]
  differentiation: string
  opportunities: string[]
}

export interface Competitor {
  name: string
  strengths: string[]
  weaknesses: string[]
  marketShare?: string
}

// CLI Tool Types
export interface CLITool {
  id: string
  name: string
  command: string
  installUrl: string
  description: string
  features: string[]
  protocol: 'stdio' | 'mcp'
  isInstalled?: boolean
}

export interface CLISession {
  id: string
  toolType: string
  projectPath: string
  isRunning: boolean
  output: CLIOutputLine[]
  createdAt: string
}

export interface CLIOutputLine {
  type: 'stdout' | 'stderr' | 'input' | 'error'
  content: string
  timestamp: string
}

// File System Types
export interface FileNode {
  name: string
  path: string
  type: 'file' | 'directory'
  children?: FileNode[]
  isExpanded?: boolean
}

// Marketing Types
export interface MarketingStrategy {
  channels: MarketingChannel[]
  timeline: MarketingTimelineItem[]
  keyMessages: string[]
}

export interface MarketingChannel {
  name: string
  platform: string
  priority: 'high' | 'medium' | 'low'
  description: string
}

export interface MarketingTimelineItem {
  phase: string
  duration: string
  activities: string[]
}

export interface MarketingCopy {
  platform: string
  content: string
  hashtags?: string[]
}

// Tool Detection Types
export interface ToolStatus {
  name: string
  isInstalled: boolean
  version?: string
  installUrl?: string
  isRequired: boolean
  category: 'runtime' | 'vcs' | 'cli' | 'editor'
}

// App State Types
export interface AppSettings {
  theme: 'light' | 'dark' | 'system'
  language: 'zh' | 'en'
  autoSave: boolean
  defaultAIProvider?: string
}

// HITL Checkpoint Types (Vibe Coding)
export interface Issue {
  id: string
  iid: number
  title: string
  description: string
  acceptanceCriteria: string[]
  priority: 'P0' | 'P1' | 'P2' | 'P3'
  status: 'todo' | 'in_progress' | 'done'
  estimatedHours?: number
  dependencies?: number[] // issue iids
  labels: string[]
  filePath?: string
}

export interface Milestone {
  id: string
  iid: number
  title: string
  description: string
  issues: number[] // issue iids
  dueDate?: string
}

export type CheckpointStatus = 'pending' | 'reviewed' | 'approved' | 'rejected'

export type CheckpointId =
  | 'CP-001' // 项目验证
  | 'CP-002' // 任务分解审查
  | 'CP-003' // 上下文丰富化
  | 'CP-004' // 回归测试审查
  | 'CP-005' // Issue 选择确认
  | 'CP-006' // Issue 完成审查
  | 'CP-007' // MR 创建审查
  | 'CP-008' // 最终 MR 审查

export interface Checkpoint {
  id: CheckpointId
  name: string
  description: string
  triggeredAt: string
  agentId: string
  status: CheckpointStatus
  reviewItems: ReviewItem[]
  userAction?: UserAction
  feedback?: string
  autoAcceptEnabled: boolean
  trustThreshold: number
}

export interface ReviewItem {
  itemType: 'issue_list' | 'milestone_list' | 'code_diff' | 'test_report' | 'project_info'
  title: string
  content: string // Markdown or JSON string
  severity: 'high' | 'medium' | 'low'
  metadata?: Record<string, unknown>
}

export interface UserAction {
  action: 'approve' | 'reject' | 'modify'
  timestamp: string
  modifications?: Modification[]
}

export interface Modification {
  issueIid?: number
  field: string
  oldValue: unknown
  newValue: unknown
}

export interface CheckpointResponse {
  checkpointId: CheckpointId
  approved: boolean
  modifications?: Modification[]
  feedback?: string
}
