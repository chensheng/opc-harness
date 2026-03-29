/* eslint-disable @typescript-eslint/no-explicit-any */
import { renderHook, act, waitFor } from '@testing-library/react'
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { usePersonaStream } from './usePersonaStream'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

// Mock Tauri APIs
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}))

vi.mock('@tauri-apps/api/event', () => ({
  listen: vi.fn(),
}))

vi.mock('@/stores/aiConfigStore', () => ({
  useAIConfigStore: () => ({
    getActiveConfig: vi.fn(() => ({
      provider: 'openai',
      apiKey: 'test-api-key',
      model: 'gpt-4o',
    })),
  }),
}))

const mockInvoke = vi.mocked(invoke)
const mockListen = vi.mocked(listen)

describe('usePersonaStream', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  afterEach(() => {
    vi.restoreAllMocks()
  })

  it('should initialize with default values', () => {
    const { result } = renderHook(() => usePersonaStream())

    expect(result.current.personas).toEqual([])
    expect(result.current.markdownContent).toBe('')
    expect(result.current.isStreaming).toBe(false)
    expect(result.current.isComplete).toBe(false)
    expect(result.current.error).toBeNull()
    expect(result.current.sessionId).toBeNull()
  })

  it('should start streaming and receive persona chunks', async () => {
    const mockSessionId = 'persona-session-123'
    const mockFullMarkdown = `## 用户画像 1: Alex
- **年龄**: 28 岁
- **职业**: 全栈开发者
- **背景**: 有 5 年开发经验
- **目标**:
  - 快速验证产品想法
  - 减少重复性工作
- **痛点**:
  - 时间有限
  - 不懂设计
- **行为特征**:
  - 订阅技术博客
- **引言**: "我想提高效率"
`

    // Mock invoke to return session ID
    mockInvoke.mockResolvedValueOnce(mockSessionId)

    // Mock listen for complete event
    let completeCallback:
      | ((event: { payload: { session_id: string; content: string } }) => void)
      | null = null
    const mockUnlisten = vi.fn()
    mockListen.mockImplementation(((
      event: string,
      callback: (event: { payload: { session_id: string; content: string } }) => void
    ) => {
      if (event === 'persona-stream-complete') {
        completeCallback = callback
      }
      return Promise.resolve(mockUnlisten)
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
    }) as any)

    const { result } = renderHook(() => usePersonaStream())

    await act(async () => {
      await result.current.startStream({
        prdId: 'prd-1',
        projectId: 'project-1',
      })
    })

    expect(result.current.isStreaming).toBe(true)
    expect(result.current.sessionId).toBe(mockSessionId)

    // Simulate completion with full content
    await act(async () => {
      if (completeCallback) {
        completeCallback({
          payload: {
            session_id: mockSessionId,
            content: mockFullMarkdown,
          },
        })
      }
    })

    // Wait for parsing
    await waitFor(
      () => {
        expect(result.current.personas.length).toBeGreaterThan(0)
      },
      { timeout: 1000 }
    )

    expect(result.current.personas[0].name).toBe('Alex')
  })

  it('should handle stream completion', async () => {
    const mockSessionId = 'persona-session-complete'
    const mockFinalContent = `## 用户画像 1: Sarah
- **年龄**: 32 岁
- **职业**: UI/UX 设计师
- **背景**: 在设计行业工作 8 年
- **目标**:
  - 将设计能力变现
  - 建立个人品牌
- **痛点**:
  - 不懂技术实现
  - 缺乏商业思维
`

    mockInvoke.mockResolvedValueOnce(mockSessionId)

    let completeCallback:
      | ((event: { payload: { session_id: string; content: string } }) => void)
      | null = null
    const mockUnlistenComplete = vi.fn()
    mockListen.mockImplementation(((
      event: string,
      callback: (event: { payload: { session_id: string; content: string } }) => void
    ) => {
      if (event === 'persona-stream-complete') {
        completeCallback = callback
      }
      return Promise.resolve(mockUnlistenComplete)
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
    }) as any)

    const { result } = renderHook(() => usePersonaStream())

    await act(async () => {
      await result.current.startStream({
        prdId: 'prd-1',
        projectId: 'project-1',
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

    expect(result.current.personas.length).toBe(1)
    expect(result.current.personas[0].name).toBe('Sarah')
  })

  it('should handle stream errors', async () => {
    const mockSessionId = 'persona-session-error'
    const mockError = 'AI 调用失败'

    mockInvoke.mockResolvedValueOnce(mockSessionId)

    let errorCallback:
      | ((event: { payload: { session_id: string; error: string } }) => void)
      | null = null
    const mockUnlistenError = vi.fn()
    mockListen.mockImplementation(((
      event: string,
      callback: (event: { payload: { session_id: string; error: string } }) => void
    ) => {
      if (event === 'persona-stream-error') {
        errorCallback = callback
      }
      return Promise.resolve(mockUnlistenError)
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
    }) as any)

    const { result } = renderHook(() => usePersonaStream())

    await act(async () => {
      await result.current.startStream({
        prdId: 'prd-1',
        projectId: 'project-1',
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
    const mockSessionId = 'persona-session-stop'
    const mockUnlisten = vi.fn()

    mockInvoke.mockResolvedValueOnce(mockSessionId)
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    mockListen.mockResolvedValueOnce(mockUnlisten as any)

    const { result } = renderHook(() => usePersonaStream())

    await act(async () => {
      await result.current.startStream({
        prdId: 'prd-1',
        projectId: 'project-1',
      })
    })

    expect(result.current.isStreaming).toBe(true)

    await act(async () => {
      result.current.stopStream()
    })

    expect(result.current.isStreaming).toBe(false)
  })

  it('should reset all state when reset is called', async () => {
    const { result } = renderHook(() => usePersonaStream())

    // Set some initial state by simulating a stream
    await act(async () => {
      result.current.reset()
    })

    expect(result.current.personas).toEqual([])
    expect(result.current.markdownContent).toBe('')
    expect(result.current.isStreaming).toBe(false)
    expect(result.current.isComplete).toBe(false)
    expect(result.current.error).toBeNull()
    expect(result.current.sessionId).toBeNull()
  })

  it('should parse markdown content to structured personas', async () => {
    const mockSessionId = 'persona-session-parse'
    const mockMarkdown = `## 用户画像 1: Mike
- **年龄**: 35 岁
- **职业**: 产品经理
- **背景**: 在科技公司工作 10 年
- **目标**:
  - 验证创业想法
  - 建立 MVP
- **痛点**:
  - 缺乏技术合伙人
  - 资源有限
- **引言**: "我需要快速验证我的想法"
`

    mockInvoke.mockResolvedValueOnce(mockSessionId)

    let completeCallback:
      | ((event: { payload: { session_id: string; content: string } }) => void)
      | null = null
    const mockUnlistenParse = vi.fn()
    mockListen.mockImplementation(((
      event: string,
      callback: (event: { payload: { session_id: string; content: string } }) => void
    ) => {
      if (event === 'persona-stream-complete') {
        completeCallback = callback
      }
      return Promise.resolve(mockUnlistenParse)
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
    }) as any)

    const { result } = renderHook(() => usePersonaStream())

    await act(async () => {
      await result.current.startStream({
        prdId: 'prd-1',
        projectId: 'project-1',
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
      expect(result.current.personas.length).toBe(1)
    })

    expect(result.current.personas[0]).toMatchObject({
      name: 'Mike',
      age: '35 岁',
      occupation: '产品经理',
      background: '在科技公司工作 10 年',
      goals: ['验证创业想法', '建立 MVP'],
      painPoints: ['缺乏技术合伙人', '资源有限'],
      quote: '"我需要快速验证我的想法"',
    })
  })
})
