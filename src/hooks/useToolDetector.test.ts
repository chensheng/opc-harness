import { describe, it, expect, vi, beforeEach } from 'vitest'
import { renderHook, act } from '@testing-library/react'
import { useToolDetector } from './useToolDetector'
import * as core from '@tauri-apps/api/core'

// Mock invoke
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}))

describe('useToolDetector', () => {
  const mockTools = [
    {
      name: 'Node.js',
      is_installed: true,
      version: 'v18.17.0',
      install_url: 'https://nodejs.org',
    },
    {
      name: 'Git',
      is_installed: true,
      version: '2.42.0',
      install_url: 'https://git-scm.com',
    },
    {
      name: 'Kimi CLI',
      is_installed: false,
      version: null,
      install_url: 'https://www.moonshot.cn/docs/cli',
    },
  ]

  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should initialize with empty tools', () => {
    const { result } = renderHook(() => useToolDetector())

    expect(result.current.tools).toEqual([])
    expect(result.current.isLoading).toBe(false)
    expect(result.current.error).toBe(null)
    expect(result.current.installedCount).toBe(0)
    expect(result.current.totalCount).toBe(0)
  })

  it('should detect tools successfully', async () => {
    vi.spyOn(core, 'invoke').mockResolvedValue({ tools: mockTools })

    const { result } = renderHook(() => useToolDetector())

    await act(async () => {
      await result.current.detectTools()
    })

    expect(result.current.tools).toEqual(mockTools)
    expect(result.current.isLoading).toBe(false)
    expect(result.current.error).toBe(null)
    expect(result.current.installedCount).toBe(2)
    expect(result.current.totalCount).toBe(3)
  })

  it('should handle detection error', async () => {
    const errorMessage = 'Failed to detect tools'
    vi.spyOn(core, 'invoke').mockRejectedValue(new Error(errorMessage))

    const { result } = renderHook(() => useToolDetector())

    await act(async () => {
      try {
        await result.current.detectTools()
      } catch (error) {
        // Expected error
      }
    })

    expect(result.current.tools).toEqual([])
    expect(result.current.error).toBe(errorMessage)
  })

  it('should calculate correct installation progress', async () => {
    const partialInstalledTools = [
      {
        name: 'Node.js',
        is_installed: true,
        version: 'v18.17.0',
        install_url: 'https://nodejs.org',
      },
      { name: 'npm', is_installed: true, version: '9.6.7', install_url: 'https://www.npmjs.com' },
      { name: 'Git', is_installed: false, version: null, install_url: 'https://git-scm.com' },
      {
        name: 'Cargo',
        is_installed: false,
        version: null,
        install_url: 'https://www.rust-lang.org',
      },
    ]

    vi.spyOn(core, 'invoke').mockResolvedValue({ tools: partialInstalledTools })

    const { result } = renderHook(() => useToolDetector())

    await act(async () => {
      await result.current.detectTools()
    })

    expect(result.current.installedCount).toBe(2)
    expect(result.current.totalCount).toBe(4)
  })
})
