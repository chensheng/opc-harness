/* eslint-disable @typescript-eslint/no-explicit-any */
import { renderHook, act, waitFor } from '@testing-library/react'
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { usePRDStream } from './usePRDStream'

// Mock Tauri APIs
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}))

vi.mock('@tauri-apps/api/event', () => ({
  listen: vi.fn(),
}))

const mockInvoke = vi.mocked(await import('@tauri-apps/api/core')).invoke
const mockListen = vi.mocked(await import('@tauri-apps/api/event')).listen

describe('usePRDStream', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  afterEach(() => {
    vi.resetAllMocks()
  })

  it('should initialize with default values', () => {
    const { result } = renderHook(() => usePRDStream())

    expect(result.current.prd).toBeNull()
    expect(result.current.markdownContent).toBe('')
    expect(result.current.isStreaming).toBe(false)
    expect(result.current.isComplete).toBe(false)
    expect(result.current.error).toBeNull()
    expect(result.current.sessionId).toBeNull()
  })

  it('should start streaming and receive chunks', async () => {
    const mockSessionId = 'test-session-id'
    const mockChunks = ['# 产品需求文档\n\n', '## 产品概述\n\n', '这是一个测试产品']

    // Mock invoke to return session ID
    mockInvoke.mockResolvedValueOnce(mockSessionId)

    // Mock listen for chunk events
    let chunkCallback:
      | ((event: {
          payload: { session_id: string; content: string; is_complete: boolean }
        }) => void)
      | null = null
    mockListen.mockImplementation(((
      event: string,
      callback: (event: {
        payload: { session_id: string; content: string; is_complete: boolean }
      }) => void
    ) => {
      if (event === 'prd-stream-chunk') {
        chunkCallback = callback
      }
      return Promise.resolve(vi.fn())
    }) as any)

    const { result } = renderHook(() => usePRDStream())

    await act(async () => {
      await result.current.startStream({
        idea: '测试产品想法',
        provider: 'openai',
        model: 'gpt-4o',
        apiKey: 'test-key',
      })
    })

    expect(result.current.isStreaming).toBe(true)
    expect(result.current.sessionId).toBe(mockSessionId)

    // Simulate receiving chunks
    await act(async () => {
      mockChunks.forEach(chunk => {
        if (chunkCallback) {
          chunkCallback({
            payload: {
              session_id: mockSessionId,
              content: chunk,
              is_complete: false,
            },
          })
        }
      })
    })

    // Wait for state updates
    await waitFor(() => {
      expect(result.current.markdownContent).toContain('测试产品')
    })

    expect(result.current.prd).not.toBeNull()
  })

  it('should handle stream completion', async () => {
    const mockSessionId = 'test-session-id'
    const mockFinalContent = '# 完整 PRD 内容'

    mockInvoke.mockResolvedValueOnce(mockSessionId)

    let completeCallback:
      | ((event: { payload: { session_id: string; content: string } }) => void)
      | null = null
    mockListen.mockImplementation(((
      event: string,
      callback: (event: { payload: { session_id: string; content: string } }) => void
    ) => {
      if (event === 'prd-stream-complete') {
        completeCallback = callback
      }
      return Promise.resolve(vi.fn())
    }) as any)

    const { result } = renderHook(() => usePRDStream())

    await act(async () => {
      await result.current.startStream({
        idea: '测试产品',
        provider: 'openai',
        model: 'gpt-4o',
        apiKey: 'test-key',
      })
    })

    // Simulate completion
    await act(async () => {
      if (completeCallback) {
        completeCallback({
          payload: {
            session_id: mockSessionId,
            content: mockFinalContent,
          },
        })
      }
    })

    await waitFor(() => {
      expect(result.current.isStreaming).toBe(false)
      expect(result.current.isComplete).toBe(true)
    })

    expect(result.current.markdownContent).toBe(mockFinalContent)
  })

  it('should handle stream errors', async () => {
    const mockSessionId = 'test-session-id'
    const mockError = 'AI 调用失败'

    mockInvoke.mockResolvedValueOnce(mockSessionId)

    let errorCallback:
      | ((event: { payload: { session_id: string; error: string } }) => void)
      | null = null
    mockListen.mockImplementation(((
      event: string,
      callback: (event: { payload: { session_id: string; error: string } }) => void
    ) => {
      if (event === 'prd-stream-error') {
        errorCallback = callback
      }
      return Promise.resolve(vi.fn())
    }) as any)

    const { result } = renderHook(() => usePRDStream())

    await act(async () => {
      await result.current.startStream({
        idea: '测试产品',
        provider: 'openai',
        model: 'gpt-4o',
        apiKey: 'test-key',
      })
    })

    // Simulate error
    await act(async () => {
      if (errorCallback) {
        errorCallback({
          payload: {
            session_id: mockSessionId,
            error: mockError,
          },
        })
      }
    })

    await waitFor(() => {
      expect(result.current.error).toBe(mockError)
      expect(result.current.isStreaming).toBe(false)
    })
  })

  it('should stop streaming when stopStream is called', async () => {
    const mockSessionId = 'test-session-id'
    const mockUnlisten = vi.fn()

    mockInvoke.mockResolvedValueOnce(mockSessionId)
    mockListen.mockResolvedValueOnce(mockUnlisten as any)

    const { result } = renderHook(() => usePRDStream())

    await act(async () => {
      await result.current.startStream({
        idea: '测试产品',
        provider: 'openai',
        model: 'gpt-4o',
        apiKey: 'test-key',
      })
    })

    expect(result.current.isStreaming).toBe(true)

    await act(async () => {
      result.current.stopStream()
    })

    expect(result.current.isStreaming).toBe(false)
    expect(mockUnlisten).toHaveBeenCalled()
  })

  it('should reset all state when reset is called', async () => {
    const { result } = renderHook(() => usePRDStream())

    // Set some initial state by simulating a stream
    await act(async () => {
      result.current.reset()
    })

    expect(result.current.prd).toBeNull()
    expect(result.current.markdownContent).toBe('')
    expect(result.current.isStreaming).toBe(false)
    expect(result.current.isComplete).toBe(false)
    expect(result.current.error).toBeNull()
    expect(result.current.sessionId).toBeNull()
  })

  it('should parse markdown content to structured PRD', async () => {
    const mockSessionId = 'test-session-id'
    const mockMarkdown = `# 测试产品

## 产品概述

这是一个测试产品的概述。

## 目标用户

- 用户类型 1
- 用户类型 2

## 核心功能

- 功能 1
- 功能 2

## 技术栈

- React
- TypeScript
`

    mockInvoke.mockResolvedValueOnce(mockSessionId)

    let completeCallback: ((event: any) => void) | null = null
    mockListen.mockImplementation(((event: string, callback: (event: any) => void) => {
      if (event === 'prd-stream-complete') {
        completeCallback = callback
      }
      return Promise.resolve(vi.fn())
    }) as any)

    const { result } = renderHook(() => usePRDStream())

    await act(async () => {
      await result.current.startStream({
        idea: '测试产品',
        provider: 'openai',
        model: 'gpt-4o',
        apiKey: 'test-key',
      })
    })

    // Simulate completion with full markdown
    await act(async () => {
      if (completeCallback) {
        completeCallback({
          payload: {
            session_id: mockSessionId,
            content: mockMarkdown,
          },
        })
      }
    })

    await waitFor(() => {
      expect(result.current.prd).not.toBeNull()
    })

    expect(result.current.prd?.title).toBe('测试产品')
    expect(result.current.prd?.overview).toContain('测试产品的概述')
    expect(result.current.prd?.targetUsers).toHaveLength(2)
    expect(result.current.prd?.coreFeatures).toHaveLength(2)
    expect(result.current.prd?.techStack).toHaveLength(2)
  })
})
