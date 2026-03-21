/**
 * Tauri API - Frontend to Backend IPC
 *
 * This module provides typed wrappers for all Tauri commands.
 */

import { invoke } from '@tauri-apps/api/core';
import type { Project, AIConfig, CLISession, CLIOutput, ToolInfo } from '@/types';

// ============================================================================
// System Commands
// ============================================================================

/**
 * Greet command (example/test)
 */
export async function greet(name: string): Promise<string> {
  return invoke('greet', { name });
}

/**
 * Get application version
 */
export async function getAppVersion(): Promise<string> {
  return invoke('get_app_version');
}

/**
 * Detect all development tools
 */
export async function detectTools(): Promise<ToolInfo[]> {
  return invoke('detect_tools');
}

/**
 * Check if a specific tool is installed
 */
export async function getToolStatus(tool: string): Promise<boolean> {
  return invoke('get_tool_status', { tool });
}

/**
 * Open a directory in VS Code
 */
export async function openInVSCode(path: string): Promise<void> {
  return invoke('open_in_vscode', { path });
}

/**
 * Open directory picker dialog
 */
export async function selectDirectory(): Promise<string | null> {
  return invoke('select_directory');
}

// ============================================================================
// Project Commands
// ============================================================================

export interface CreateProjectInput {
  name: string;
  description?: string;
  path?: string;
}

export interface UpdateProjectInput {
  name?: string;
  description?: string | null;
  status?: 'draft' | 'designing' | 'coding' | 'marketing' | 'completed';
  path?: string | null;
}

/**
 * Create a new project
 */
export async function createProject(input: CreateProjectInput): Promise<Project> {
  return invoke('create_project', input);
}

/**
 * Get all projects
 */
export async function getProjects(): Promise<Project[]> {
  return invoke('get_projects');
}

/**
 * Get a project by ID
 */
export async function getProject(id: string): Promise<Project | null> {
  return invoke('get_project', { id });
}

/**
 * Update a project
 */
export async function updateProject(id: string, input: UpdateProjectInput): Promise<Project> {
  return invoke('update_project', {
    id,
    name: input.name,
    description: input.description,
    status: input.status,
    path: input.path,
  });
}

/**
 * Delete a project
 */
export async function deleteProject(id: string): Promise<void> {
  return invoke('delete_project', { id });
}

// ============================================================================
// AI Configuration Commands
// ============================================================================

/**
 * Get all AI provider configurations
 */
export async function getAIConfigs(): Promise<AIConfig[]> {
  return invoke('get_ai_configs');
}

/**
 * Save AI provider configuration
 */
export async function saveAIConfig(config: AIConfig): Promise<void> {
  return invoke('save_ai_config', { config });
}

/**
 * Remove AI provider configuration
 */
export async function removeAIConfig(provider: string): Promise<void> {
  return invoke('remove_ai_config', { provider });
}

/**
 * Validate AI API key
 */
export async function validateAIKey(provider: string, apiKey: string): Promise<boolean> {
  return invoke('validate_ai_key', { provider, apiKey });
}

/**
 * Generate PRD from idea
 */
export async function generatePRD(idea: string): Promise<string> {
  return invoke('generate_prd', { idea });
}

// ============================================================================
// CLI Commands
// ============================================================================

export type CLITool = 'kimi' | 'claude' | 'codex';

/**
 * Start a CLI session
 */
export async function startCLISession(tool: CLITool, projectPath: string): Promise<CLISession> {
  return invoke('start_cli_session', { tool, projectPath });
}

/**
 * Send command to CLI session
 */
export async function sendCLICommand(sessionId: string, command: string): Promise<void> {
  return invoke('send_cli_command', { sessionId, command });
}

/**
 * Kill CLI session
 */
export async function killCLISession(sessionId: string): Promise<void> {
  return invoke('kill_cli_session', { sessionId });
}

/**
 * Get CLI output
 */
export async function getCLIOutput(sessionId: string): Promise<CLIOutput[]> {
  return invoke('get_cli_output', { sessionId });
}

// ============================================================================
// API Error Handling
// ============================================================================

export class APIError extends Error {
  constructor(
    message: string,
    public readonly code?: string,
    public readonly cause?: unknown
  ) {
    super(message);
    this.name = 'APIError';
  }
}

/**
 * Wrap an API call with error handling
 */
export async function withErrorHandling<T>(fn: () => Promise<T>, errorMessage: string): Promise<T> {
  try {
    return await fn();
  } catch (error) {
    console.error(errorMessage, error);
    throw new APIError(errorMessage, 'API_ERROR', error);
  }
}
