import React, { createContext, useContext, useState, useEffect, useCallback } from 'react'
import type { ThemeConfig, ThemeContextValue } from '@/types'
import { DEFAULT_THEME, THEME_STORAGE_KEY } from '@/types'

interface ThemeProviderProps {
  children: React.ReactNode
}

/**
 * 主题上下文
 */
const ThemeContext = createContext<ThemeContextValue | undefined>(undefined)

/**
 * 主题提供者组件
 */
export const ThemeProvider: React.FC<ThemeProviderProps> = ({ children }) => {
  const [theme, setThemeState] = useState<ThemeConfig>(() => {
    // 从 localStorage 加载用户偏好
    if (typeof window !== 'undefined') {
      const saved = localStorage.getItem(THEME_STORAGE_KEY)
      if (saved) {
        try {
          return JSON.parse(saved) as ThemeConfig
        } catch (e) {
          console.error('Failed to parse theme preferences:', e)
        }
      }
    }
    return DEFAULT_THEME
  })

  // 保存到 localStorage
  useEffect(() => {
    if (typeof window !== 'undefined') {
      localStorage.setItem(THEME_STORAGE_KEY, JSON.stringify(theme))

      // 更新 HTML 类名以支持暗色模式
      document.documentElement.classList.toggle('dark', theme.mode === 'dark')
    }
  }, [theme])

  /**
   * 设置主题（部分更新）
   */
  const setTheme = useCallback((newTheme: Partial<ThemeConfig>) => {
    setThemeState(prev => ({ ...prev, ...newTheme }))
  }, [])

  /**
   * 重置主题
   */
  const resetTheme = useCallback(() => {
    setThemeState(DEFAULT_THEME)
  }, [])

  return (
    <ThemeContext.Provider value={{ theme, setTheme, resetTheme }}>
      {children}
    </ThemeContext.Provider>
  )
}

/**
 * 使用主题上下文
 */
export const useTheme = (): ThemeContextValue => {
  const context = useContext(ThemeContext)
  if (!context) {
    throw new Error('useTheme must be used within a ThemeProvider')
  }
  return context
}
