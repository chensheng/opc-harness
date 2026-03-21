/**
 * React Hooks for Tauri API
 */

import { useState, useCallback } from 'react';
import * as api from '@/api';
import type { Project, AIConfig, CLISession, ToolInfo } from '@/types';

// ============================================================================
// System Hooks
// ============================================================================

export function useAppVersion() {
  const [version, setVersion] = useState<string>('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fetchVersion = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const v = await api.getAppVersion();
      setVersion(v);
      return v;
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Unknown error');
      throw e;
    } finally {
      setLoading(false);
    }
  }, []);

  return { version, loading, error, fetchVersion };
}

export function useToolDetection() {
  const [tools, setTools] = useState<ToolInfo[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const detectTools = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const t = await api.detectTools();
      setTools(t);
      return t;
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Unknown error');
      throw e;
    } finally {
      setLoading(false);
    }
  }, []);

  return { tools, loading, error, detectTools };
}

// ============================================================================
// Project Hooks
// ============================================================================

export function useProjects() {
  const [projects, setProjects] = useState<Project[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fetchProjects = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const p = await api.getProjects();
      setProjects(p);
      return p;
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Unknown error');
      throw e;
    } finally {
      setLoading(false);
    }
  }, []);

  const createProject = useCallback(async (input: api.CreateProjectInput) => {
    setLoading(true);
    setError(null);
    try {
      const p = await api.createProject(input);
      setProjects(prev => [...prev, p]);
      return p;
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Unknown error');
      throw e;
    } finally {
      setLoading(false);
    }
  }, []);

  const updateProject = useCallback(async (id: string, input: api.UpdateProjectInput) => {
    setLoading(true);
    setError(null);
    try {
      const p = await api.updateProject(id, input);
      setProjects(prev => prev.map(proj => (proj.id === id ? p : proj)));
      return p;
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Unknown error');
      throw e;
    } finally {
      setLoading(false);
    }
  }, []);

  const deleteProject = useCallback(async (id: string) => {
    setLoading(true);
    setError(null);
    try {
      await api.deleteProject(id);
      setProjects(prev => prev.filter(p => p.id !== id));
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Unknown error');
      throw e;
    } finally {
      setLoading(false);
    }
  }, []);

  return {
    projects,
    loading,
    error,
    fetchProjects,
    createProject,
    updateProject,
    deleteProject,
  };
}

// ============================================================================
// AI Config Hooks
// ============================================================================

export function useAIConfigs() {
  const [configs, setConfigs] = useState<AIConfig[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fetchConfigs = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const c = await api.getAIConfigs();
      setConfigs(c);
      return c;
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Unknown error');
      throw e;
    } finally {
      setLoading(false);
    }
  }, []);

  const saveConfig = useCallback(
    async (config: AIConfig) => {
      setLoading(true);
      setError(null);
      try {
        await api.saveAIConfig(config);
        await fetchConfigs();
      } catch (e) {
        setError(e instanceof Error ? e.message : 'Unknown error');
        throw e;
      } finally {
        setLoading(false);
      }
    },
    [fetchConfigs]
  );

  const removeConfig = useCallback(
    async (provider: string) => {
      setLoading(true);
      setError(null);
      try {
        await api.removeAIConfig(provider);
        await fetchConfigs();
      } catch (e) {
        setError(e instanceof Error ? e.message : 'Unknown error');
        throw e;
      } finally {
        setLoading(false);
      }
    },
    [fetchConfigs]
  );

  const validateKey = useCallback(async (provider: string, apiKey: string) => {
    setLoading(true);
    setError(null);
    try {
      const valid = await api.validateAIKey(provider, apiKey);
      return valid;
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Unknown error');
      throw e;
    } finally {
      setLoading(false);
    }
  }, []);

  return {
    configs,
    loading,
    error,
    fetchConfigs,
    saveConfig,
    removeConfig,
    validateKey,
  };
}

// ============================================================================
// CLI Hooks
// ============================================================================

export function useCLISession() {
  const [session, setSession] = useState<CLISession | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const startSession = useCallback(async (tool: api.CLITool, projectPath: string) => {
    setLoading(true);
    setError(null);
    try {
      const s = await api.startCLISession(tool, projectPath);
      setSession(s);
      return s;
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Unknown error');
      throw e;
    } finally {
      setLoading(false);
    }
  }, []);

  const sendCommand = useCallback(
    async (command: string) => {
      if (!session) throw new Error('No active session');
      setLoading(true);
      setError(null);
      try {
        await api.sendCLICommand(session.id, command);
      } catch (e) {
        setError(e instanceof Error ? e.message : 'Unknown error');
        throw e;
      } finally {
        setLoading(false);
      }
    },
    [session]
  );

  const killSession = useCallback(async () => {
    if (!session) return;
    setLoading(true);
    setError(null);
    try {
      await api.killCLISession(session.id);
      setSession(null);
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Unknown error');
      throw e;
    } finally {
      setLoading(false);
    }
  }, [session]);

  return {
    session,
    loading,
    error,
    startSession,
    sendCommand,
    killSession,
  };
}
