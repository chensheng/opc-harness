import { describe, it, expect, beforeEach } from 'vitest'
import { act, renderHook } from '@testing-library/react'
import { useAppStore } from './appStore'

describe('useAppStore', () => {
  beforeEach(() => {
    // 重置 store 到初始状态
    const { result } = renderHook(() => useAppStore())
    act(() => {
      result.current.setSettings({
        theme: 'system',
        language: 'zh',
        autoSave: true,
        useNativeAgent: false,
      })
      result.current.setActiveTab('dashboard')
      // 确保侧边栏打开（初始状态就是 true）
      if (!result.current.isSidebarOpen) {
        result.current.toggleSidebar()
      }
    })
  })

  it('should initialize with default settings', () => {
    const { result } = renderHook(() => useAppStore())

    expect(result.current.settings).toEqual({
      theme: 'system',
      language: 'zh',
      autoSave: true,
      useNativeAgent: false,
    })
    expect(result.current.isSidebarOpen).toBe(true)
    expect(result.current.activeTab).toBe('dashboard')
  })

  it('should update settings', () => {
    const { result } = renderHook(() => useAppStore())

    act(() => {
      result.current.setSettings({ theme: 'dark' })
    })

    expect(result.current.settings.theme).toBe('dark')
    expect(result.current.settings.language).toBe('zh') // 保持不变
  })

  it('should toggle sidebar', () => {
    const { result } = renderHook(() => useAppStore())

    expect(result.current.isSidebarOpen).toBe(true)

    act(() => {
      result.current.toggleSidebar()
    })

    expect(result.current.isSidebarOpen).toBe(false)

    act(() => {
      result.current.toggleSidebar()
    })

    expect(result.current.isSidebarOpen).toBe(true)
  })

  it('should set active tab', () => {
    const { result } = renderHook(() => useAppStore())

    act(() => {
      result.current.setActiveTab('coding')
    })

    expect(result.current.activeTab).toBe('coding')

    act(() => {
      result.current.setActiveTab('marketing')
    })

    expect(result.current.activeTab).toBe('marketing')
  })

  it('should set loading state', () => {
    const { result } = renderHook(() => useAppStore())

    expect(result.current.isLoading).toBe(false)

    act(() => {
      result.current.setLoading(true, '加载中...')
    })

    expect(result.current.isLoading).toBe(true)
    expect(result.current.loadingMessage).toBe('加载中...')

    act(() => {
      result.current.setLoading(false)
    })

    expect(result.current.isLoading).toBe(false)
  })

  it('should persist settings to localStorage', () => {
    const { result } = renderHook(() => useAppStore())

    act(() => {
      result.current.setSettings({ theme: 'light', language: 'en' })
    })

    // 验证 localStorage 中是否有持久化的数据
    const stored = localStorage.getItem('opc-harness-app')
    expect(stored).toBeTruthy()

    const parsed = JSON.parse(stored!)
    expect(parsed.state.settings).toEqual({
      theme: 'light',
      language: 'en',
      autoSave: true,
      useNativeAgent: false,
    })
  })
})
