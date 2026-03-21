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

export interface AIConfig {
  provider: AIProvider;
  apiKey: string;
  baseUrl?: string;
  model: string;
  enabled: boolean;
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
