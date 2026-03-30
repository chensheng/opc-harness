import { describe, it, expect, beforeEach, vi } from 'vitest'
import { renderHook, act } from '@testing-library/react'
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/core'
import { usePersonaStream } from './usePersonaStream'

// Mock Tauri APIs
vi.mock('@tauri-apps/api/event', () => ({
  listen: vi.fn(),
}))

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}))

describe('usePersonaStream', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should initialize with correct state', () => {
    const { result } = renderHook(() => usePersonaStream())

    expect(result.current.personas).toEqual([])
    expect(result.current.markdownContent).toBe('')
    expect(result.current.isStreaming).toBe(false)
    expect(result.current.isComplete).toBe(false)
    expect(result.current.error).toBe(null)
    expect(result.current.sessionId).toBe(null)
  })

  it('should handle streaming chunks', async () => {
    const mockUnlisten = vi.fn()
    vi.mocked(listen).mockImplementation(((event: string, handler: (payload: unknown) => void) => {
      if (event === 'persona-stream-chunk') {
        handler({
          event: 'persona-stream-chunk',
          id: 1,
          payload: {
            session_id: 'session-123',
            content: '## 用户画像 1\n姓名：张三\n年龄：30\n职业：工程师\n',
            is_complete: false,
          },
        })
      }
      return Promise.resolve(mockUnlisten)
    }) as typeof listen)

    // Mock invoke response
    vi.mocked(invoke).mockResolvedValue('session-123')

    const { result } = renderHook(() => usePersonaStream())

    // Start streaming
    await act(async () => {
      await result.current.startStream({
        idea: 'Test idea',
        provider: 'openai',
        model: 'gpt-4',
        apiKey: 'test-key',
      })
    })

    // Should be streaming
    expect(result.current.isStreaming).toBe(true)
    expect(result.current.sessionId).toBe('session-123')

    // Should have parsed the persona
    expect(result.current.markdownContent).toBe(
      '## 用户画像 1\n姓名：张三\n年龄：30\n职业：工程师\n'
    )
    expect(result.current.personas.length).toBeGreaterThan(0)
  })

  it('should handle stream complete', async () => {
    const mockUnlisten = vi.fn()
    vi.mocked(listen).mockResolvedValue(mockUnlisten)
    vi.mocked(invoke).mockResolvedValue('session-123')

    const { result } = renderHook(() => usePersonaStream())

    await act(async () => {
      await result.current.startStream({
        idea: 'Test idea',
        provider: 'openai',
        model: 'gpt-4',
        apiKey: 'test-key',
      })
    })

    // Simulate complete event
    const completeCallback = vi
      .mocked(listen)
      .mock.calls.find(call => call[0] === 'persona-stream-complete')?.[1]

    if (completeCallback) {
      await act(async () => {
        await completeCallback({
          event: 'persona-stream-complete',
          id: 2,
          payload: {
            session_id: 'session-123',
            content:
              '## 用户画像 1\n姓名：张三\n年龄：30\n职业：工程师\n目标:\n- 学习新技术\n- 提升技能\n',
          },
        })
      })

      expect(result.current.isComplete).toBe(true)
      expect(result.current.isStreaming).toBe(false)
      expect(result.current.personas.length).toBeGreaterThan(0)
    }
  })

  it('should handle stream error', async () => {
    const mockUnlisten = vi.fn()
    vi.mocked(listen).mockResolvedValue(mockUnlisten)
    vi.mocked(invoke).mockResolvedValue('session-123')

    const { result } = renderHook(() => usePersonaStream())

    await act(async () => {
      await result.current.startStream({
        idea: 'Test idea',
        provider: 'openai',
        model: 'gpt-4',
        apiKey: 'test-key',
      })
    })

    // Simulate error event
    const errorCallback = vi
      .mocked(listen)
      .mock.calls.find(call => call[0] === 'persona-stream-error')?.[1]

    if (errorCallback) {
      await act(async () => {
        await errorCallback({
          event: 'persona-stream-error',
          id: 3,
          payload: {
            session_id: 'session-123',
            error: 'API key invalid',
          },
        })
      })

      expect(result.current.error).toBe('API key invalid')
      expect(result.current.isStreaming).toBe(false)
    }
  })

  it('should stop streaming', async () => {
    const mockUnlisten = vi.fn()
    vi.mocked(listen).mockResolvedValue(mockUnlisten)
    vi.mocked(invoke).mockResolvedValue('session-123')

    const { result } = renderHook(() => usePersonaStream())

    await act(async () => {
      await result.current.startStream({
        idea: 'Test idea',
        provider: 'openai',
        model: 'gpt-4',
        apiKey: 'test-key',
      })
    })

    expect(result.current.isStreaming).toBe(true)

    // Stop streaming
    await act(async () => {
      result.current.stopStream()
    })

    expect(result.current.isStreaming).toBe(false)
  })

  it('should reset state', async () => {
    const mockUnlisten = vi.fn()
    vi.mocked(listen).mockResolvedValue(mockUnlisten)
    vi.mocked(invoke).mockResolvedValue('session-123')

    const { result } = renderHook(() => usePersonaStream())

    await act(async () => {
      await result.current.startStream({
        idea: 'Test idea',
        provider: 'openai',
        model: 'gpt-4',
        apiKey: 'test-key',
      })
    })

    // Reset
    await act(async () => {
      result.current.reset()
    })

    expect(result.current.personas).toEqual([])
    expect(result.current.markdownContent).toBe('')
    expect(result.current.isStreaming).toBe(false)
    expect(result.current.isComplete).toBe(false)
    expect(result.current.error).toBe(null)
    expect(result.current.sessionId).toBe(null)
  })

  it('should parse persona from markdown correctly', async () => {
    const mockUnlisten = vi.fn()
    vi.mocked(listen).mockResolvedValue(mockUnlisten)
    vi.mocked(invoke).mockResolvedValue('session-123')

    const { result } = renderHook(() => usePersonaStream())

    await act(async () => {
      await result.current.startStream({
        idea: 'Test idea',
        provider: 'openai',
        model: 'gpt-4',
        apiKey: 'test-key',
      })
    })

    // Simulate receiving complete persona data
    const chunkCallback = vi
      .mocked(listen)
      .mock.calls.find(call => call[0] === 'persona-stream-chunk')?.[1]

    const markdownContent = `## 用户画像 1
姓名：李明
年龄：28
职业：产品经理
背景：在一家互联网公司工作 5 年，负责多个成功产品。

目标:
- 打造用户体验优秀的产品
- 学习数据分析技能
- 提升团队管理能力

痛点:
- 需求变更频繁
- 资源不足
- 跨部门沟通困难

行为:
- 经常加班
- 喜欢研究竞品
- 定期参加行业会议

"做产品就像雕刻艺术品，需要耐心和细心"`

    if (chunkCallback) {
      await act(async () => {
        await chunkCallback({
          event: 'persona-stream-chunk',
          id: 4,
          payload: {
            session_id: 'session-123',
            content: markdownContent,
            is_complete: false,
          },
        })
      })

      const personas = result.current.personas
      expect(personas.length).toBe(1)
      expect(personas[0].name).toBe('李明')
      expect(personas[0].age).toBe('28')
      expect(personas[0].occupation).toBe('产品经理')
      expect(personas[0].goals.length).toBeGreaterThan(0)
      expect(personas[0].painPoints.length).toBeGreaterThan(0)
      expect(personas[0].behaviors.length).toBeGreaterThan(0)
      expect(personas[0].quote).toBeDefined()
    }
  })
})
