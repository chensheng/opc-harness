import { describe, it, expect, vi, beforeEach } from 'vitest'
import { renderHook, act } from '@testing-library/react'
import { useDaemon } from './useDaemon'

// Mock Tauri invoke
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}))

describe('useDaemon', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should initialize with empty state', () => {
    const { result } = renderHook(() => useDaemon())

    expect(result.current.snapshot).toBeNull()
    expect(result.current.status).toBeNull()
    expect(result.current.isLoading).toBe(false)
    expect(result.current.error).toBeNull()
  })

  it('should start daemon successfully', async () => {
    const mockSnapshot = {
      daemonId: 'daemon-001',
      status: 'running' as const,
      config: {
        sessionId: 'session-001',
        projectPath: '/tmp/test',
        logLevel: 'debug',
        maxConcurrentAgents: 3,
        workspaceDir: '/tmp',
      },
      activeAgents: [],
      completedTasks: [],
      pendingTasks: [],
      startTime: Date.now(),
      lastUpdate: Date.now(),
      systemInfo: {
        os: 'Windows',
        arch: 'x86_64',
        totalMemory: 16384,
        availableMemory: 8192,
        cpuCores: 8,
        rustVersion: '0.1.0',
      },
    }

    const { invoke } = await import('@tauri-apps/api/core')
    vi.mocked(invoke).mockResolvedValue(mockSnapshot)

    const { result } = renderHook(() => useDaemon())

    await act(async () => {
      await result.current.startDaemon({
        sessionId: 'session-001',
        projectPath: '/tmp/test',
        logLevel: 'debug',
        maxConcurrentAgents: 3,
      })
    })

    expect(result.current.isLoading).toBe(false)
    expect(result.current.error).toBeNull()
  })

  it('should stop daemon successfully', async () => {
    const { invoke } = await import('@tauri-apps/api/core')
    vi.mocked(invoke).mockResolvedValue(undefined)

    const { result } = renderHook(() => useDaemon())

    await act(async () => {
      await result.current.stopDaemon(true)
    })

    expect(result.current.isLoading).toBe(false)
    expect(result.current.status).toBe('stopped')
  })

  it('should pause and resume daemon', async () => {
    const { invoke } = await import('@tauri-apps/api/core')
    vi.mocked(invoke).mockResolvedValue(undefined)

    const { result } = renderHook(() => useDaemon())

    // 暂停
    await act(async () => {
      await result.current.pauseDaemon()
    })

    expect(result.current.status).toBe('paused')

    // 恢复
    await act(async () => {
      await result.current.resumeDaemon()
    })

    expect(result.current.status).toBe('running')
  })

  it('should spawn agent successfully', async () => {
    const { invoke } = await import('@tauri-apps/api/core')
    vi.mocked(invoke).mockResolvedValue('agent-001')

    const { result } = renderHook(() => useDaemon())

    let agentId: string | undefined
    await act(async () => {
      agentId = await result.current.spawnAgent('initializer')
    })

    expect(agentId).toBe('agent-001')
  })

  it('should kill agent successfully', async () => {
    const { invoke } = await import('@tauri-apps/api/core')
    vi.mocked(invoke).mockResolvedValue(undefined)

    const { result } = renderHook(() => useDaemon())

    await act(async () => {
      await result.current.killAgent('agent-001')
    })

    expect(result.current.isLoading).toBe(false)
  })

  it('should refresh snapshot', async () => {
    const mockSnapshot = {
      daemonId: 'daemon-001',
      status: 'running' as const,
      config: {
        sessionId: 'session-001',
        projectPath: '/tmp/test',
        logLevel: 'info',
        maxConcurrentAgents: 5,
        workspaceDir: '/tmp',
      },
      activeAgents: [],
      completedTasks: [],
      pendingTasks: [],
      startTime: Date.now(),
      lastUpdate: Date.now(),
      systemInfo: {
        os: 'Windows',
        arch: 'x86_64',
        totalMemory: 16384,
        availableMemory: 8192,
        cpuCores: 8,
        rustVersion: '0.1.0',
      },
    }

    const { invoke } = await import('@tauri-apps/api/core')
    vi.mocked(invoke).mockResolvedValue(mockSnapshot)

    const { result } = renderHook(() => useDaemon())

    await act(async () => {
      await result.current.refreshSnapshot()
    })

    expect(result.current.snapshot).not.toBeNull()
    expect(result.current.error).toBeNull()
  })

  it('should handle start error', async () => {
    const { invoke } = await import('@tauri-apps/api/core')
    vi.mocked(invoke).mockRejectedValue(new Error('Failed to start'))

    const { result } = renderHook(() => useDaemon())

    try {
      await act(async () => {
        await result.current.startDaemon({})
      })
    } catch {
      // Expected to fail
    }

    expect(result.current.isLoading).toBe(false)
  })

  it('should manage multiple agents', async () => {
    const { invoke } = await import('@tauri-apps/api/core')
    vi.mocked(invoke).mockResolvedValueOnce('agent-001').mockResolvedValueOnce('agent-002')

    const { result } = renderHook(() => useDaemon())

    const agentIds: string[] = []

    await act(async () => {
      const id1 = await result.current.spawnAgent('coding')
      const id2 = await result.current.spawnAgent('mr_creation')
      agentIds.push(id1, id2)
    })

    expect(agentIds).toEqual(['agent-001', 'agent-002'])
  })

  it('should track loading state correctly', async () => {
    const { invoke } = await import('@tauri-apps/api/core')
    vi.mocked(invoke).mockResolvedValue({
      daemonId: 'daemon-001',
      status: 'running' as const,
      config: {
        sessionId: 'test',
        projectPath: '',
        logLevel: 'info',
        maxConcurrentAgents: 5,
        workspaceDir: '',
      },
      activeAgents: [],
      completedTasks: [],
      pendingTasks: [],
      startTime: Date.now(),
      lastUpdate: Date.now(),
      systemInfo: {
        os: 'Windows',
        arch: 'x86_64',
        totalMemory: 16384,
        availableMemory: 8192,
        cpuCores: 8,
        rustVersion: '0.1.0',
      },
    })

    const { result } = renderHook(() => useDaemon())

    const startPromise = act(async () => {
      await result.current.startDaemon({ sessionId: 'test' })
    })

    await startPromise
    expect(result.current.isLoading).toBe(false)
  })
})
