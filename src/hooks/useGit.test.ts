import { describe, it, expect, vi, beforeEach } from 'vitest'
import { renderHook, act } from '@testing-library/react'
import { useGit } from './useGit'
import * as core from '@tauri-apps/api/core'

// Mock invoke
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}))

describe('useGit', () => {
  const mockGitStatus = {
    isGitRepo: true,
    gitVersion: '2.42.0',
    branch: 'main',
    commitCount: 10,
    isDirty: false,
  }

  const mockGitConfig = {
    userName: 'Test User',
    userEmail: 'test@example.com',
  }

  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should initialize with null values', () => {
    const { result } = renderHook(() => useGit())

    expect(result.current.gitStatus).toBeNull()
    expect(result.current.gitConfig).toBeNull()
    expect(result.current.isLoading).toBe(false)
    expect(result.current.error).toBeNull()
  })

  it('should check git status successfully', async () => {
    vi.spyOn(core, 'invoke').mockResolvedValue(mockGitStatus)

    const { result } = renderHook(() => useGit())

    await act(async () => {
      await result.current.checkGitStatus('/tmp/test')
    })

    expect(result.current.gitStatus).toEqual(mockGitStatus)
    expect(result.current.isLoading).toBe(false)
    expect(result.current.error).toBeNull()
  })

  it('should handle git status check error', async () => {
    const errorMessage = 'Failed to check git status'
    vi.spyOn(core, 'invoke').mockRejectedValue(new Error(errorMessage))

    const { result } = renderHook(() => useGit())

    await act(async () => {
      await result.current.checkGitStatus('/tmp/test').catch(() => {
        // Expected error
      })
    })

    expect(result.current.gitStatus).toBeNull()
    expect(result.current.error).toBe(errorMessage)
  })

  it('should initialize git repo successfully', async () => {
    vi.spyOn(core, 'invoke').mockResolvedValue(true)

    const { result } = renderHook(() => useGit())

    const initResult = await act(async () => {
      return await result.current.initGitRepo('/tmp/test', 'main')
    })

    expect(initResult).toBe(true)
    expect(result.current.isLoading).toBe(false)
    expect(result.current.error).toBeNull()
  })

  it('should set git config successfully', async () => {
    vi.spyOn(core, 'invoke').mockResolvedValue(true)

    const { result } = renderHook(() => useGit())

    const setResult = await act(async () => {
      return await result.current.setGitConfig('/tmp/test', 'user.name', 'Test User')
    })

    expect(setResult).toBe(true)
  })

  it('should get git config successfully', async () => {
    vi.spyOn(core, 'invoke').mockResolvedValue('Test User')

    const { result } = renderHook(() => useGit())

    const configValue = await act(async () => {
      return await result.current.getGitConfig('/tmp/test', 'user.name')
    })

    expect(configValue).toBe('Test User')
  })

  it('should get all git config successfully', async () => {
    vi.spyOn(core, 'invoke').mockResolvedValue(mockGitConfig)

    const { result } = renderHook(() => useGit())

    const config = await act(async () => {
      return await result.current.getAllGitConfig('/tmp/test')
    })

    expect(config).toEqual(mockGitConfig)
    expect(result.current.gitConfig).toEqual(mockGitConfig)
  })

  it('should handle get all git config error', async () => {
    const errorMessage = 'Failed to get all git config'
    vi.spyOn(core, 'invoke').mockRejectedValue(new Error(errorMessage))

    const { result } = renderHook(() => useGit())

    const config = await act(async () => {
      return await result.current.getAllGitConfig('/tmp/test')
    })

    expect(config).toEqual({ userName: null, userEmail: null })
    expect(result.current.error).toBe(errorMessage)
  })
})
