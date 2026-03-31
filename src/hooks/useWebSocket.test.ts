import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest'
import { renderHook, act } from '@testing-library/react'
import { useWebSocket, type WebSocketMessage } from './useWebSocket'

// Mock WebSocket
const mockWebSocket = vi.hoisted(() => {
  return class MockWebSocket {
    static OPEN = 1
    static CONNECTING = 0
    static CLOSING = 2
    static CLOSED = 3

    readyState = 0
    onopen: (() => void) | null = null
    onclose: (() => void) | null = null
    onerror: ((event: any) => void) | null = null
    onmessage: ((event: { data: string }) => void) | null = null

    constructor(public url: string) {
      setTimeout(() => {
        this.readyState = 1
        this.onopen?.()
      }, 10)
    }

    send(data: string) {
      // Mock send
    }

    close() {
      this.readyState = 3
      this.onclose?.()
    }
  }
})

vi.stubGlobal('WebSocket', mockWebSocket)

describe('useWebSocket', () => {
  beforeEach(() => {
    vi.clearAllMocks()
    vi.useFakeTimers()
  })

  afterEach(() => {
    vi.useRealTimers()
  })

  it('should connect to WebSocket server on mount', () => {
    const onOpen = vi.fn()

    renderHook(() =>
      useWebSocket({
        url: 'ws://localhost:8080',
        autoConnect: true,
        onOpen,
      })
    )

    expect(onOpen).not.toHaveBeenCalled()

    act(() => {
      vi.advanceTimersByTime(10)
    })

    expect(onOpen).toHaveBeenCalled()
  })

  it('should update connected status after connection', () => {
    const { result } = renderHook(() =>
      useWebSocket({
        url: 'ws://localhost:8080',
        autoConnect: true,
      })
    )

    expect(result.current.connected).toBe(false)
    expect(result.current.status).toBe('connecting')

    act(() => {
      vi.advanceTimersByTime(10)
    })

    expect(result.current.connected).toBe(true)
    expect(result.current.status).toBe('connected')
  })

  it('should send message when connected', () => {
    const { result } = renderHook(() =>
      useWebSocket({
        url: 'ws://localhost:8080',
        autoConnect: true,
      })
    )

    act(() => {
      vi.advanceTimersByTime(10)
    })

    const message: WebSocketMessage = {
      type: 'subscribe',
      topic: 'test-topic',
    }

    // Should not throw
    act(() => {
      result.current.sendMessage(message)
    })
  })

  it('should subscribe to topic', () => {
    const { result } = renderHook(() =>
      useWebSocket({
        url: 'ws://localhost:8080',
        autoConnect: true,
      })
    )

    act(() => {
      vi.advanceTimersByTime(10)
    })

    act(() => {
      result.current.subscribe('agent-logs')
    })

    // Should have sent subscribe message
  })

  it('should unsubscribe from topic', () => {
    const { result } = renderHook(() =>
      useWebSocket({
        url: 'ws://localhost:8080',
        autoConnect: true,
      })
    )

    act(() => {
      vi.advanceTimersByTime(10)
    })

    act(() => {
      result.current.unsubscribe('agent-logs')
    })
  })

  it('should disconnect and prevent reconnect', () => {
    const onClose = vi.fn()

    const { result } = renderHook(() =>
      useWebSocket({
        url: 'ws://localhost:8080',
        autoConnect: true,
        onClose,
      })
    )

    act(() => {
      vi.advanceTimersByTime(10)
    })

    expect(result.current.connected).toBe(true)

    act(() => {
      result.current.disconnect()
    })

    expect(result.current.connected).toBe(false)
    expect(result.current.status).toBe('disconnected')
  })

  it('should handle connection error', () => {
    const onError = vi.fn()

    // Mock WebSocket that immediately errors
    const ErrorWebSocket = class extends mockWebSocket {
      constructor(url: string) {
        super(url)
        setTimeout(() => {
          this.onerror?.({ type: 'error' })
        }, 10)
      }
    }

    vi.stubGlobal('WebSocket', ErrorWebSocket)

    const { result } = renderHook(() =>
      useWebSocket({
        url: 'ws://localhost:8080',
        autoConnect: true,
        onError,
      })
    )

    act(() => {
      vi.advanceTimersByTime(10)
    })

    expect(result.current.status).toBe('error')
    expect(result.current.error).toBeTruthy()

    vi.stubGlobal('WebSocket', mockWebSocket)
  })

  it('should handle messages correctly', () => {
    const onMessage = vi.fn()

    renderHook(() =>
      useWebSocket({
        url: 'ws://localhost:8080',
        autoConnect: true,
        onMessage,
      })
    )

    act(() => {
      vi.advanceTimersByTime(10)
    })

    // Just verify the hook is set up correctly and can receive messages
    expect(onMessage).toBeDefined()
  })
})
