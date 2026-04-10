import { describe, it, expect, beforeEach } from 'vitest'
import { act, renderHook } from '@testing-library/react'
import { useAIConfigStore } from './aiConfigStore'

describe('useAIConfigStore - Default Provider', () => {
  beforeEach(() => {
    // 重置 store 到初始状态
    const { result } = renderHook(() => useAIConfigStore())
    act(() => {
      // 清除所有配置
      Object.keys(result.current.configs).forEach(provider => {
        result.current.removeConfig(provider)
      })
      // 配置并验证 openai，然后设为默认
      result.current.setConfig('openai', {
        provider: 'openai',
        model: 'gpt-4o',
        apiKey: 'test-key',
        validated: true,
      })
      result.current.setDefaultProvider('openai')
    })
  })

  it('should initialize with openai as default provider', () => {
    const { result } = renderHook(() => useAIConfigStore())
    expect(result.current.defaultProvider).toBe('openai')
  })

  it('should only allow setting validated provider as default', () => {
    const { result } = renderHook(() => useAIConfigStore())

    // 配置但未验证的厂商不能设为默认
    act(() => {
      result.current.setConfig('anthropic', {
        provider: 'anthropic',
        model: 'claude-3-5-sonnet-20241022',
        apiKey: 'test-key',
      })
      result.current.setDefaultProvider('anthropic')
    })

    // 应该仍然是openai，因为anthropic未验证
    expect(result.current.defaultProvider).toBe('openai')

    // 标记为已验证后可以设为默认
    act(() => {
      result.current.markAsValidated('anthropic')
      result.current.setDefaultProvider('anthropic')
    })

    expect(result.current.defaultProvider).toBe('anthropic')
  })

  it('should reset to openai when deleting default provider', () => {
    const { result } = renderHook(() => useAIConfigStore())

    act(() => {
      result.current.setConfig('kimi', {
        provider: 'kimi',
        model: 'kimi-k2.5',
        apiKey: 'test-key',
        validated: true,
      })
      result.current.setDefaultProvider('kimi')
    })

    expect(result.current.defaultProvider).toBe('kimi')

    act(() => {
      result.current.removeConfig('kimi')
    })

    expect(result.current.defaultProvider).toBe('openai')
  })

  it('should mark provider as validated', () => {
    const { result } = renderHook(() => useAIConfigStore())

    act(() => {
      result.current.setConfig('glm', {
        provider: 'glm',
        model: 'glm-5',
        apiKey: 'test-key',
      })
    })

    let config = result.current.getConfig('glm')
    expect(config?.validated).toBeFalsy()

    act(() => {
      result.current.markAsValidated('glm')
    })

    config = result.current.getConfig('glm')
    expect(config?.validated).toBe(true)
  })

  it('should persist default provider to localStorage', () => {
    const { result } = renderHook(() => useAIConfigStore())

    act(() => {
      result.current.setConfig('minimax', {
        provider: 'minimax',
        model: 'MiniMax-M2.7',
        apiKey: 'test-key',
        validated: true,
      })
      result.current.setDefaultProvider('minimax')
    })

    expect(result.current.defaultProvider).toBe('minimax')

    // 验证 localStorage 中是否有持久化的数据
    const stored = localStorage.getItem('opc-harness-ai-config')
    expect(stored).toBeTruthy()

    const parsed = JSON.parse(stored!)
    expect(parsed.state.defaultProvider).toBe('minimax')
  })

  it('should get active config using default provider', () => {
    const { result } = renderHook(() => useAIConfigStore())

    const activeConfig = result.current.getActiveConfig()
    expect(activeConfig?.provider).toBe('openai')
    expect(activeConfig?.model).toBe('gpt-4o')
  })
})
