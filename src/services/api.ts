/**
 * API service layer
 * Handles communication with Tauri backend
 */

import { invoke } from '@tauri-apps/api/core';
import type { Project, AIConfig, CLISession, ToolInfo } from '@/types';

// Project API
export const projectApi = {
  createProject: (data: Omit<Project, 'id' | 'createdAt' | 'updatedAt'>) =>
    invoke<Project>('create_project', { data }),

  getProjects: () => invoke<Project[]>('get_projects'),

  getProject: (id: string) => invoke<Project>('get_project', { id }),

  updateProject: (id: string, data: Partial<Project>) =>
    invoke<Project>('update_project', { id, data }),

  deleteProject: (id: string) => invoke<void>('delete_project', { id }),
};

// AI Config API
export const aiConfigApi = {
  getConfigs: () => invoke<Record<string, AIConfig>>('get_ai_configs'),

  setConfig: (config: AIConfig) => invoke<void>('set_ai_config', { config }),

  removeConfig: (provider: string) => invoke<void>('remove_ai_config', { provider }),

  validateApiKey: (provider: string, apiKey: string) =>
    invoke<boolean>('validate_api_key', { provider, apiKey }),
};

// CLI API
export const cliApi = {
  startSession: (tool: string, projectPath: string) =>
    invoke<CLISession>('start_cli_session', { tool, projectPath }),

  sendCommand: (sessionId: string, command: string) =>
    invoke<void>('send_cli_command', { sessionId, command }),

  killSession: (sessionId: string) => invoke<void>('kill_cli_session', { sessionId }),
};

// System API
export const systemApi = {
  detectTools: () => invoke<ToolInfo[]>('detect_tools'),

  openInVSCode: (path: string) => invoke<void>('open_in_vscode', { path }),

  selectDirectory: () => invoke<string | null>('select_directory'),
};
