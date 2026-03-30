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
  // 新增指标字段
  revenue?: string // 收入
  userGrowth?: string // 用户增长率
  employeeCount?: number // 员工数量
  foundedYear?: number // 成立年份
  funding?: string // 融资金额
  customerSatisfaction?: number // 客户满意度 (0-100)
  innovationScore?: number // 创新能力评分 (0-100)
  priceRange?: string // 价格区间
  geographicPresence?: string[] // 地理覆盖
  keyProducts?: string[] // 核心产品
}

// 数据探索器配置
export interface ExplorerConfig {
  selectedMetrics: MetricKey[]
  viewMode: ViewMode
  filters: DataFilters
  sortBy?: MetricKey
  sortOrder?: 'asc' | 'desc'
}

export type MetricKey =
  | 'marketShare'
  | 'revenue'
  | 'userGrowth'
  | 'employeeCount'
  | 'funding'
  | 'customerSatisfaction'
  | 'innovationScore'

export type ViewMode = 'bar' | 'line' | 'pie' | 'radar' | 'cards'

export interface DataFilters {
  minMarketShare?: number
  maxEmployeeCount?: number
  foundedAfter?: number
  hasFunding?: boolean
}

// 指标元数据（用于显示）
export interface MetricDefinition {
  key: MetricKey
  label: string
  description: string
  unit: string
  icon: string
  color: string
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
