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
