import { describe, it, expect, beforeEach } from 'vitest'
import { renderHook, act } from '@testing-library/react'
import { ThemeProvider, useTheme } from '@/contexts/ThemeContext'
import type { ThemeConfig } from '@/types'

const wrapper = ({ children }: { children: React.ReactNode }) => (
  <ThemeProvider>{children}</ThemeProvider>
)

describe('useTheme', () => {
  beforeEach(() => {
    // 清除 localStorage
    localStorage.clear()
  })

  it('should provide default theme on initial load', () => {
    const { result } = renderHook(() => useTheme(), { wrapper })

    expect(result.current.theme).toEqual({
      mode: 'light',
      colorScheme: 'blue',
      fontSize: 'medium',
      cardRadius: 'medium',
      cardShadow: 'medium',
    })
  })

  it('should update theme with setTheme', () => {
    const { result } = renderHook(() => useTheme(), { wrapper })

    act(() => {
      result.current.setTheme({ mode: 'dark' })
    })

    expect(result.current.theme.mode).toBe('dark')
  })

  it('should support partial theme updates', () => {
    const { result } = renderHook(() => useTheme(), { wrapper })

    act(() => {
      result.current.setTheme({
        colorScheme: 'green',
        fontSize: 'large',
      })
    })

    expect(result.current.theme.colorScheme).toBe('green')
    expect(result.current.theme.fontSize).toBe('large')
    expect(result.current.theme.mode).toBe('light') // unchanged
  })

  it('should reset theme to defaults', () => {
    const { result } = renderHook(() => useTheme(), { wrapper })

    act(() => {
      result.current.setTheme({ mode: 'dark', colorScheme: 'purple' })
    })

    expect(result.current.theme.mode).toBe('dark')

    act(() => {
      result.current.resetTheme()
    })

    expect(result.current.theme.mode).toBe('light')
    expect(result.current.theme.colorScheme).toBe('blue')
  })

  it('should persist theme to localStorage', () => {
    const { result } = renderHook(() => useTheme(), { wrapper })

    act(() => {
      result.current.setTheme({ mode: 'dark' })
    })

    const savedTheme = localStorage.getItem('harness-theme-preferences')
    expect(savedTheme).toBeTruthy()

    const parsed = JSON.parse(savedTheme!) as ThemeConfig
    expect(parsed.mode).toBe('dark')
  })

  it('should load theme from localStorage on mount', () => {
    // 预先设置 localStorage
    localStorage.setItem(
      'harness-theme-preferences',
      JSON.stringify({
        mode: 'dark',
        colorScheme: 'green',
        fontSize: 'small',
        cardRadius: 'none',
        cardShadow: 'small',
      })
    )

    const { result } = renderHook(() => useTheme(), { wrapper })

    expect(result.current.theme.mode).toBe('dark')
    expect(result.current.theme.colorScheme).toBe('green')
    expect(result.current.theme.fontSize).toBe('small')
  })

  it('should throw error when used outside ThemeProvider', () => {
    expect(() => renderHook(() => useTheme())).toThrow(
      'useTheme must be used within a ThemeProvider'
    )
  })
})
