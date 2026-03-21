import { create } from 'zustand';
import { devtools, persist } from 'zustand/middleware';
import { immer } from 'zustand/middleware/immer';

export type Theme = 'light' | 'dark' | 'system';

interface AppState {
  // 主题设置
  theme: Theme;
  setTheme: (theme: Theme) => void;

  // 侧边栏状态
  sidebarOpen: boolean;
  toggleSidebar: () => void;
  setSidebarOpen: (open: boolean) => void;

  // 全局加载状态
  isLoading: boolean;
  setLoading: (loading: boolean) => void;

  // 全局错误信息
  error: string | null;
  setError: (error: string | null) => void;
  clearError: () => void;

  // 当前激活的模块
  activeModule: 'design' | 'coding' | 'marketing' | 'settings';
  setActiveModule: (module: AppState['activeModule']) => void;
}

export const useAppStore = create<AppState>()(
  devtools(
    immer(
      persist(
        set => ({
          // 初始状态
          theme: 'system',
          setTheme: theme =>
            set(state => {
              state.theme = theme;
            }),

          sidebarOpen: true,
          toggleSidebar: () =>
            set(state => {
              state.sidebarOpen = !state.sidebarOpen;
            }),
          setSidebarOpen: open =>
            set(state => {
              state.sidebarOpen = open;
            }),

          isLoading: false,
          setLoading: loading =>
            set(state => {
              state.isLoading = loading;
            }),

          error: null,
          setError: error =>
            set(state => {
              state.error = error;
            }),
          clearError: () =>
            set(state => {
              state.error = null;
            }),

          activeModule: 'design',
          setActiveModule: module =>
            set(state => {
              state.activeModule = module;
            }),
        }),
        {
          name: 'app-storage',
          partialize: state => ({
            theme: state.theme,
            sidebarOpen: state.sidebarOpen,
          }),
        }
      )
    ),
    { name: 'AppStore' }
  )
);
