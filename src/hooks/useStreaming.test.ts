/**
 * 流式输出 Hook 测试
 */
import { renderHook, act } from '@testing-library/react'
import { useStreaming } from './useStreaming'

// Mock Tauri APIs
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}))

vi.mock('@tauri-apps/api/event', () => ({
  listen: vi.fn(() => Promise.resolve(vi.fn())),
}))

import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

type EventHandler = (event: { payload: { content: string } }) => void

describe('useStreaming', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should initialize with idle state', () => {
    const { result } = renderHook(() => useStreaming())

    expect(result.current.isStreaming).toBe(false)
    expect(result.current.content).toBe('')
    expect(result.current.progress).toBe(0)
    expect(result.current.error).toBe(null)
  })

  it('should stream content successfully', async () => {
    // Mock event listeners
    const mockListeners: Record<string, EventHandler> = {}
    vi.mocked(listen).mockImplementation(async (_event: string, handler: EventHandler) => {
      mockListeners[_event] = handler
      return vi.fn()
    })

    // Mock invoke to simulate completion
    vi.mocked(invoke).mockImplementation(async () => {
      // Simulate chunks arriving
      await act(async () => {
        if (mockListeners['prd-stream-chunk']) {
          mockListeners['prd-stream-chunk']({ payload: { content: 'Hello' } })
          await new Promise(resolve => setTimeout(resolve, 10))
          mockListeners['prd-stream-chunk']({ payload: { content: ' World' } })
          await new Promise(resolve => setTimeout(resolve, 10))
        }
      })

      // Simulate completion
      await act(async () => {
        if (mockListeners['prd-stream-complete']) {
          mockListeners['prd-stream-complete']({ payload: { content: 'Hello World' } })
        }
      })

      return 'Hello World'
    })

    const { result } = renderHook(() => useStreaming())

    await act(async () => {
      await result.current.startStream('Test idea')
    })

    expect(result.current.isStreaming).toBe(false)
    expect(result.current.content).toBe('Hello World')
    expect(result.current.error).toBe(null)
  })

  it('should handle streaming error', async () => {
    vi.mocked(invoke).mockRejectedValue(new Error('Network error'))

    const { result } = renderHook(() => useStreaming())

    await act(async () => {
      await result.current.startStream('Test idea')
    })

    expect(result.current.isStreaming).toBe(false)
    expect(result.current.content).toBe('')
    expect(result.current.error).toBe('Network error')
  })

  it('should reset state on new stream start', async () => {
    const mockListeners: Record<string, EventHandler> = {}
    vi.mocked(listen).mockImplementation(async (_event: string, handler: EventHandler) => {
      mockListeners[_event] = handler
      return vi.fn()
    })

    vi.mocked(invoke).mockResolvedValueOnce('First content').mockResolvedValueOnce('Second content')

    const { result } = renderHook(() => useStreaming())

    await act(async () => {
      await result.current.startStream('First idea')
      if (mockListeners['prd-stream-complete']) {
        mockListeners['prd-stream-complete']({ payload: { content: 'First content' } })
      }
    })

    expect(result.current.content).toBe('First content')

    await act(async () => {
      await result.current.startStream('Second idea')
      if (mockListeners['prd-stream-complete']) {
        mockListeners['prd-stream-complete']({ payload: { content: 'Second content' } })
      }
    })

    expect(result.current.content).toBe('Second content')
  })
})
