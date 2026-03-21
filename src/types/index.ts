/**
 * Global TypeScript type definitions
 */

// Project types
export interface Project {
  id: string;
  name: string;
  description?: string;
  status: 'draft' | 'designing' | 'coding' | 'marketing' | 'completed';
  createdAt: number;
  updatedAt: number;
  path?: string;
}

// AI Provider types
export type AIProvider = 'openai' | 'anthropic' | 'kimi' | 'glm';

/**
 * AI 厂商配置数据结构 (VD-001)
 *
 * 用于存储和管理不同 AI 厂商的 API 配置信息
 */
export interface AIConfig {
  /** AI 厂商标识 */
  provider: AIProvider;

  /** API 密钥（可选，支持从系统 keychain 读取） */
  apiKey?: string;

  /** API 基础 URL */
  baseUrl: string;

  /** 使用的模型名称 */
  model: string;

  /** 是否启用该配置 */
  enabled: boolean;

  /** 配置名称（用户自定义） */
  name?: string;

  /** 创建时间戳 */
  createdAt?: number;

  /** 最后更新时间戳 */
  updatedAt?: number;

  /** API 密钥最后验证时间戳 (VD-004) */
  lastVerifiedAt?: number;

  /** API 密钥验证状态 (VD-004) */
  isValid?: boolean;
}

/**
 * AI 厂商元数据信息
 * 用于在 UI 中显示厂商信息和默认配置
 */
export interface AIProviderMeta {
  /** 厂商标识 */
  id: AIProvider;

  /** 厂商显示名称 */
  name: string;

  /** 官方文档链接 */
  docUrl?: string;

  /** API 密钥申请链接 */
  apiKeyUrl?: string;

  /** 默认 API 基础 URL */
  defaultBaseUrl: string;

  /** 支持的模型列表 */
  supportedModels: string[];

  /** 默认模型 */
  defaultModel: string;

  /** 是否支持流式输出 */
  supportsStreaming: boolean;

  /** 是否支持视觉功能 */
  supportsVision?: boolean;
}

/**
 * AI 厂商配置状态管理接口
 */
export interface AIConfigState {
  /** 各厂商的配置（使用 Record 类型支持动态键） */
  configs: Partial<Record<AIProvider, AIConfig>>;

  /** 当前激活的厂商 ID */
  activeProvider: AIProvider | null;

  /** 可用的模型列表（按厂商分组） */
  availableModels: Record<AIProvider, string[]>;

  /** 流式输出回调函数类型 (VD-015) */
  StreamCallback?: (chunk: string) => void;
}

// VD-015 流式输出相关类型

/**
 * 流式聊天请求参数
 */
export interface StreamChatRequest {
  /** AI 厂商标识 */
  provider: AIProvider;

  /** 消息数组 */
  messages: ChatMessage[];

  /** 流式输出回调函数 */
  onChunk: (chunk: string) => void;

  /** 完成回调函数 */
  onComplete?: () => void;

  /** 错误回调函数 */
  onError?: (error: string) => void;
}

/**
 * 聊天消息类型
 */
export interface ChatMessage {
  /** 角色：'user', 'assistant', 'system' */
  role: string;

  /** 消息内容 */
  content: string;
}

// Message types for AI chat
export type MessageRole = 'system' | 'user' | 'assistant';

export interface Message {
  role: MessageRole;
  content: string;
}

// CLI Tool types
export type CLIToolType = 'kimi' | 'claude' | 'codex';

export interface CLISession {
  id: string;
  tool: CLIToolType;
  projectPath: string;
  status: 'running' | 'stopped' | 'error';
  startedAt: number;
}

export interface CLIOutput {
  sessionId: string;
  outputType: 'stdout' | 'stderr' | 'system';
  content: string;
  timestamp: number;
}

// User Persona types (VD-025)
/**
 * 用户画像数据结构
 */
export interface UserPersona {
  /** 基本信息 - 姓名 */
  name?: string;

  /** 基本信息 - 年龄 */
  age?: string;

  /** 基本信息 - 职业 */
  occupation?: string;

  /** 基本信息 - 城市 */
  city?: string;

  /** 基本信息 - 收入水平 */
  incomeLevel?: string;

  /** 个人背景 - 教育背景 */
  education?: string;

  /** 个人背景 - 工作经历 */
  workExperience?: string;

  /** 个人背景 - 技术能力 */
  technicalSkills?: string;

  /** 个人背景 - 生活方式 */
  lifestyle?: string;

  /** 与产品相关的特征 - 核心需求 */
  coreNeeds?: string;

  /** 与产品相关的特征 - 使用场景 */
  usageScenarios?: string;

  /** 与产品相关的特征 - 期望功能 */
  expectedFeatures?: string;

  /** 与产品相关的特征 - 付费意愿 */
  willingnessToPay?: string;

  /** 行为特征 - 获取信息渠道 */
  informationChannels?: string;

  /** 行为特征 - 决策因素 */
  decisionFactors?: string;

  /** 行为特征 - 使用频率预期 */
  expectedUsageFrequency?: string;

  /** 行为特征 - 推荐意愿 */
  recommendationWillingness?: string;

  /** 用户引言 */
  quote?: string;

  /** 其他描述信息（用于兼容非结构化数据） */
  description?: string;

  /** 原始 Markdown 内容（如果解析失败） */
  markdown?: string;

  /** 备注信息 */
  note?: string;
}

// Tool detection types
export interface ToolInfo {
  name: string;
  installed: boolean;
  version?: string;
  path?: string;
}

// UI types
export type Theme = 'light' | 'dark' | 'system';

export interface Toast {
  id: string;
  type: 'success' | 'error' | 'warning' | 'info';
  title: string;
  message?: string;
  duration?: number;
}
