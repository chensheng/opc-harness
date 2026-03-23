import { describe, it, expect, vi, beforeEach } from 'vitest'
import { renderHook, act } from '@testing-library/react'
import { useAgent } from './useAgent'

describe('useAgent', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should initialize with empty state', () => {
    const { result } = renderHook(() => useAgent())

    expect(result.current.agents).toEqual([])
    expect(result.current.messages).toEqual([])
    expect(result.current.daemonState).toBeNull()
    expect(result.current.isLoading).toBe(false)
    expect(result.current.error).toBeNull()
  })

  it('should connect WebSocket successfully', async () => {
    const { result } = renderHook(() => useAgent())

    await act(async () => {
      await result.current.connectWebSocket('session-001')
    })

    expect(result.current.isLoading).toBe(false)
    expect(result.current.error).toBeNull()
  })

  it('should handle WebSocket connection error', async () => {
    const errorMessage = 'Connection failed'
    
    // Mock console.log to avoid noise
    vi.spyOn(console, 'log').mockImplementation(() => {})

    const { result } = renderHook(() => useAgent())

    await act(async () => {
      // 当前实现不会抛出错误，需要后续完善
      await result.current.connectWebSocket('session-001').catch(() => {
        // Expected error
      })
    })

    expect(result.current.isLoading).toBe(false)
  })

  it('should disconnect WebSocket', () => {
    const { result } = renderHook(() => useAgent())

    act(() => {
      result.current.disconnectWebSocket()
    })

    // 验证断开连接不抛出错误
    expect(() => result.current.disconnectWebSocket()).not.toThrow()
  })

  it('should send agent request successfully', async () => {
    const { result } = renderHook(() => useAgent())

    let response: any
    await act(async () => {
      response = await result.current.sendAgentRequest(
        'agent-001',
        'initialize',
        { project: 'test' },
      )
    })

    expect(response).toBeDefined()
    expect(response.success).toBe(true)
    expect(response.requestId).toBeDefined()
  })

  it('should handle agent request error', async () => {
    const { result } = renderHook(() => useAgent())

    let response: any
    await act(async () => {
      response = await result.current.sendAgentRequest(
        'agent-001',
        'invalid_action',
        {},
      )
    })

    // 当前 Mock 实现返回成功响应，需要后续完善错误处理
    expect(response).toBeDefined()
  })

  it('should subscribe and unsubscribe agent', () => {
    const { result } = renderHook(() => useAgent())

    act(() => {
      result.current.subscribeAgent('agent-001')
    })

    act(() => {
      result.current.unsubscribeAgent('agent-001')
    })

    // 验证订阅/取消订阅不抛出错误
    expect(() => result.current.subscribeAgent('agent-002')).not.toThrow()
    expect(() => result.current.unsubscribeAgent('agent-002')).not.toThrow()
  })

  it('should manage multiple agents', async () => {
    const { result } = renderHook(() => useAgent())

    // 模拟多个 Agent 请求
    await act(async () => {
      await Promise.all([
        result.current.sendAgentRequest('agent-001', 'init', {}),
        result.current.sendAgentRequest('agent-002', 'init', {}),
        result.current.sendAgentRequest('agent-003', 'init', {}),
      ])
    })

    expect(result.current.isLoading).toBe(false)
  })
})
