import { describe, it, expect, vi, beforeEach } from 'vitest'
import { renderHook, act } from '@testing-library/react'
import { useOpenAIProvider } from './useOpenAIProvider'

describe('useOpenAIProvider', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should initialize with correct state', () => {
    const { result } = renderHook(() => useOpenAIProvider())

    expect(result.current.isLoading).toBe(false)
    expect(result.current.error).toBe(null)
    expect(typeof result.current.chat).toBe('function')
    expect(typeof result.current.streamChat).toBe('function')
    expect(typeof result.current.validateApiKey).toBe('function')
  })

  it('should validate API key successfully', async () => {
    const { result } = renderHook(() => useOpenAIProvider())

    let isValid: boolean | undefined
    await act(async () => {
      isValid = await result.current.validateApiKey('sk-test123')
    })

    expect(isValid).toBe(true)
    expect(result.current.isLoading).toBe(false)
    expect(result.current.error).toBe(null)
  })

  it('should handle chat request', async () => {
    const { result } = renderHook(() => useOpenAIProvider())

    const request = {
      model: 'gpt-4',
      messages: [{ role: 'user', content: 'Hello!' }],
      temperature: 0.7,
    }

    let response: any
    await act(async () => {
      response = await result.current.chat(request)
    })

    expect(response).not.toBeNull()
    expect(response?.content).toContain('模拟的 OpenAI 响应')
    expect(response?.usage).toBeDefined()
    expect(result.current.isLoading).toBe(false)
  })

  it('should handle stream chat with chunks', async () => {
    const { result } = renderHook(() => useOpenAIProvider())

    const chunks: string[] = []
    const request = {
      model: 'gpt-4',
      messages: [{ role: 'user', content: 'Tell me a story' }],
      stream: true,
    }

    let fullContent: string | null = null
    await act(async () => {
      fullContent = await result.current.streamChat(request, chunk => chunks.push(chunk))
    })

    expect(fullContent).not.toBeNull()
    expect(chunks.length).toBeGreaterThan(0)
    expect(fullContent).toContain('流式响应')
  })

  it('should clear error on successful operation', async () => {
    const { result } = renderHook(() => useOpenAIProvider())

    await act(async () => {
      await result.current.validateApiKey('sk-valid-key')
    })

    expect(result.current.error).toBe(null)
  })
})
