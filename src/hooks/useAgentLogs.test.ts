import { describe, it, expect, beforeEach, vi } from 'vitest'
import { renderHook, act } from '@testing-library/react'
import { useAgentLogs, LogLevel } from './useAgentLogs'

describe('useAgentLogs', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should initialize with empty logs', () => {
    const { result } = renderHook(() =>
      useAgentLogs({ wsUrl: 'ws://localhost:8080', autoConnect: false })
    )

    expect(result.current.logs).toEqual([])
    expect(result.current.connected).toBe(false)
    expect(result.current.paused).toBe(false)
    expect(result.current.filterLevel).toBeNull()
  })

  it('should provide pause and resume functions', () => {
    const { result } = renderHook(() =>
      useAgentLogs({ wsUrl: 'ws://localhost:8080', autoConnect: false })
    )

    expect(result.current.paused).toBe(false)

    act(() => {
      result.current.pause()
    })

    expect(result.current.paused).toBe(true)

    act(() => {
      result.current.resume()
    })

    expect(result.current.paused).toBe(false)
  })

  it('should provide clear function', () => {
    const { result } = renderHook(() =>
      useAgentLogs({ wsUrl: 'ws://localhost:8080', autoConnect: false })
    )

    // Should not throw
    act(() => {
      result.current.clear()
    })

    expect(result.current.logs).toEqual([])
  })

  it('should provide filter level state and setter', () => {
    const { result } = renderHook(() =>
      useAgentLogs({ wsUrl: 'ws://localhost:8080', autoConnect: false })
    )

    expect(result.current.filterLevel).toBeNull()

    act(() => {
      result.current.setFilterLevel(LogLevel.ERROR)
    })

    expect(result.current.filterLevel).toBe(LogLevel.ERROR)
  })

  it('should support maxLogs configuration', () => {
    const { result } = renderHook(() =>
      useAgentLogs({ wsUrl: 'ws://localhost:8080', maxLogs: 100, autoConnect: false })
    )

    expect(result.current).toBeDefined()
  })
})
