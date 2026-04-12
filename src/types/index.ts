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
  cliPath?: string // CodeFree CLI 等本地工具的完整路径（可选）
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
  prd?: PRD // 内存中的结构化 PRD 对象（用于展示）
  prdMarkdown?: string // 数据库中的原始 Markdown 内容（用于持久化）
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
  markdownContent?: string // 完整的 Markdown 文档内容
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

// User Story Types (用户故事管理)
export interface UserStory {
  /** 用户故事 ID */
  id: string
  /** 故事编号（如 US-001） */
  storyNumber: string
  /** 故事标题 */
  title: string
  /** 用户故事描述（As a... I want... So that...） */
  description: string
  /** 角色（As a ...） */
  role: string
  /** 功能（I want ...） */
  feature: string
  /** 价值（So that ...） */
  benefit: string
  /** 验收标准 */
  acceptanceCriteria: string[]
  /** 优先级 */
  priority: 'P0' | 'P1' | 'P2' | 'P3'
  /** 状态 */
  status: 'draft' | 'refined' | 'approved' | 'in_development' | 'completed'
  /** 估算的故事点 */
  storyPoints?: number
  /** 依赖的故事 ID */
  dependencies?: string[]
  /** 关联的功能模块 */
  featureModule?: string
  /** 标签 */
  labels: string[]
  /** 创建时间 */
  createdAt: string
  /** 更新时间 */
  updatedAt: string
}

/**
 * AI 拆分用户故事的请求
 */
export interface DecomposeUserStoriesRequest {
  /** PRD 内容或功能描述 */
  prdContent: string
  /** AI 提供商 (openai, anthropic, kimi, glm, minimax, codefree) */
  provider?: string
  /** AI 模型名称 */
  model?: string
  /** 可选：AI API Key */
  apiKey?: string
  /** 可选：项目 ID（用于 CodeFree 写入文件） */
  projectId?: string
  /** 可选：已有的用户故事列表（用于避免重复生成） */
  existingStories?: Array<{
    title: string
    role: string
    feature: string
  }>
}

/**
 * AI 拆分用户故事的响应
 */
export interface DecomposeUserStoriesResponse {
  /** 是否成功 */
  success: boolean
  /** 拆分出的用户故事列表 */
  userStories: UserStory[]
  /** 错误消息 */
  errorMessage?: string
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

/**
 * US-060: 自定义可视化样式类型定义
 */

/**
 * 主题模式
 */
export type ThemeMode = 'light' | 'dark'

/**
 * 配色方案
 */
export type ColorScheme = 'blue' | 'green' | 'purple' | 'orange'

/**
 * 字体大小
 */
export type FontSize = 'small' | 'medium' | 'large'

/**
 * 卡片圆角
 */
export type CardRadius = 'none' | 'small' | 'medium' | 'large'

/**
 * 卡片阴影
 */
export type CardShadow = 'none' | 'small' | 'medium' | 'large'

/**
 * 主题配置接口
 */
export interface ThemeConfig {
  /** 主题模式 */
  mode: ThemeMode
  /** 配色方案 */
  colorScheme: ColorScheme
  /** 字体大小 */
  fontSize: FontSize
  /** 卡片圆角 */
  cardRadius: CardRadius
  /** 卡片阴影 */
  cardShadow: CardShadow
}

/**
 * 主题上下文值
 */
export interface ThemeContextValue {
  /** 当前主题配置 */
  theme: ThemeConfig
  /** 设置主题 */
  setTheme: (theme: Partial<ThemeConfig>) => void
  /** 重置主题 */
  resetTheme: () => void
}

/**
 * 默认主题配置
 */
export const DEFAULT_THEME: ThemeConfig = {
  mode: 'light',
  colorScheme: 'blue',
  fontSize: 'medium',
  cardRadius: 'medium',
  cardShadow: 'medium',
}

/**
 * 主题存储键名
 */
export const THEME_STORAGE_KEY = 'harness-theme-preferences'

/**
 * US-032: 任务分解相关类型
 */
export enum TaskType {
  FRONTEND = 'frontend',
  BACKEND = 'backend',
  DATABASE = 'database',
  TESTING = 'testing',
  DOCUMENTATION = 'documentation',
  DEPLOYMENT = 'deployment',
}

export interface TechnicalTask {
  id: string
  title: string
  description: string
  taskType: TaskType
  estimatedHours: number
  dependencies: string[]
  priority: number
  featureId: string
  complexity: number
  skills: string[]
}

export interface DependencyEdge {
  fromTask: string
  toTask: string
  dependencyType: string
  strength: string
}

export interface TaskStatistics {
  totalTasks: number
  frontendTasks: number
  backendTasks: number
  databaseTasks: number
  testingTasks: number
  averageHours: number
  averageComplexity: number
}

export interface TaskDependencyGraph {
  tasks: TechnicalTask[]
  edges: DependencyEdge[]
  criticalPath: string[]
  totalEstimatedHours: number
  statistics: TaskStatistics
}

export interface DecomposeTasksRequest {
  analysis: PrdAnalysis
}

export interface DecomposeTasksResponse {
  success: boolean
  taskGraph: TaskDependencyGraph
  errorMessage?: string
}

/**
 * PRD 深度分析相关类型
 */
export enum FeatureType {
  CORE = 'core',
  AUXILIARY = 'auxiliary',
  ENHANCED = 'enhanced',
}

export enum RiskLevel {
  LOW = 'low',
  MEDIUM = 'medium',
  HIGH = 'high',
  CRITICAL = 'critical',
}

export interface Feature {
  id: string
  name: string
  description: string
  featureType: FeatureType
  complexity: number
  estimatedHours: number
  priority: number
  dependencies: string[]
}

export interface Dependency {
  fromFeature: string
  toFeature: string
  dependencyType: string
  strength: string
}

export interface Risk {
  id: string
  description: string
  level: RiskLevel
  impact: string
  mitigation?: string
  relatedFeatures: string[]
}

export interface Estimates {
  totalFeatures: number
  coreFeatures: number
  auxiliaryFeatures: number
  enhancedFeatures: number
  averageComplexity: number
  totalEstimatedHours: number
  highRisksCount: number
}

export interface PrdAnalysis {
  features: Feature[]
  dependencies: Dependency[]
  risks: Risk[]
  estimates: Estimates
}

export interface AnalyzePRDDepthRequest {
  prdContent: string
  apiKey?: string
}

export interface AnalyzePRDDepthResponse {
  success: boolean
  analysis: PrdAnalysis
  errorMessage?: string
}
