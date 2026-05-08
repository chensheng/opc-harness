import { create } from 'zustand'
import { persist } from 'zustand/middleware'
import { immer } from 'zustand/middleware/immer'
import type { AppSettings } from '@/types'

interface AppState {
  settings: AppSettings
  isSidebarOpen: boolean
  activeTab: string
  isLoading: boolean
  loadingMessage: string
}

interface AppActions {
  setSettings: (settings: Partial<AppSettings>) => void
  toggleSidebar: () => void
  setActiveTab: (tab: string) => void
  setLoading: (isLoading: boolean, message?: string) => void
}

const initialSettings: AppSettings = {
  theme: 'system',
  language: 'zh',
  autoSave: true,
  useNativeAgent: false, // 默认使用 CLI Agent
}

export const useAppStore = create<AppState & AppActions>()(
  immer(
    persist(
      set => ({
        settings: initialSettings,
        isSidebarOpen: true,
        activeTab: 'dashboard',
        isLoading: false,
        loadingMessage: '',

        setSettings: settings =>
          set(state => {
            Object.assign(state.settings, settings)
          }),

        toggleSidebar: () =>
          set(state => {
            state.isSidebarOpen = !state.isSidebarOpen
          }),

        setActiveTab: tab =>
          set(state => {
            state.activeTab = tab
          }),

        setLoading: (isLoading, message = '') =>
          set(state => {
            state.isLoading = isLoading
            state.loadingMessage = message
          }),
      }),
      {
        name: 'opc-harness-app',
        partialize: state => ({ settings: state.settings }),
      }
    )
  )
)
