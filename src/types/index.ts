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
  /** 各厂商的配置（使用 Record 类型支持动态添加） */
  configs: Partial<Record<AIProvider, AIConfig>>;

  /** 当前激活的厂商 ID */
  activeProvider: AIProvider | null;

  /** 可用的模型列表（按厂商分组） */
  availableModels: Record<AIProvider, string[]>;

  /** 设置某个厂商的配置 */
  setConfig: (provider: AIProvider, config: AIConfig) => void;

  /** 移除某个厂商的配置 */
  removeConfig: (provider: AIProvider) => void;

  /** 设置当前使用的厂商 */
  setActiveProvider: (provider: AIProvider | null) => void;

  /** 验证 API 密钥有效性 */
  validateApiKey?: (provider: AIProvider) => Promise<boolean>;
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
