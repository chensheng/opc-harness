import { create } from 'zustand';
import { devtools, persist } from 'zustand/middleware';
import { immer } from 'zustand/middleware/immer';

export type AIProvider = 'openai' | 'anthropic' | 'kimi' | 'glm';

export interface AIConfig {
  provider: AIProvider;
  apiKey: string;
  baseUrl?: string;
  model: string;
  enabled: boolean;
}

interface AIConfigState {
  // AI 配置列表
  configs: Record<AIProvider, AIConfig | null>;

  // 当前使用的提供商
  activeProvider: AIProvider | null;

  // 配置操作
  setConfig: (provider: AIProvider, config: AIConfig) => void;
  removeConfig: (provider: AIProvider) => void;
  setActiveProvider: (provider: AIProvider | null) => void;

  // 模型列表（各提供商支持的模型）
  availableModels: Record<AIProvider, string[]>;
}

const defaultModels: Record<AIProvider, string[]> = {
  openai: ['gpt-4o', 'gpt-4o-mini', 'gpt-4-turbo', 'gpt-3.5-turbo'],
  anthropic: ['claude-3-5-sonnet-20241022', 'claude-3-opus-20240229', 'claude-3-haiku-20240307'],
  kimi: ['kimi-k2', 'kimi-k2-0711', 'moonshot-v1-8k', 'moonshot-v1-32k', 'moonshot-v1-128k'],
  glm: ['glm-4-plus', 'glm-4-0520', 'glm-4-air', 'glm-4-flash'],
};

export const useAIConfigStore = create<AIConfigState>()(
  devtools(
    immer(
      persist(
        set => ({
          // 初始状态
          configs: {
            openai: null,
            anthropic: null,
            kimi: null,
            glm: null,
          },
          activeProvider: null,
          availableModels: defaultModels,

          setConfig: (provider, config) =>
            set(state => {
              state.configs[provider] = config;
              if (config.enabled && !state.activeProvider) {
                state.activeProvider = provider;
              }
            }),

          removeConfig: provider =>
            set(state => {
              state.configs[provider] = null;
              if (state.activeProvider === provider) {
                // 寻找下一个启用的提供商
                const nextEnabled = (Object.keys(state.configs) as AIProvider[]).find(
                  p => p !== provider && state.configs[p]?.enabled
                );
                state.activeProvider = nextEnabled || null;
              }
            }),

          setActiveProvider: provider =>
            set(state => {
              state.activeProvider = provider;
            }),
        }),
        {
          name: 'ai-config-storage',
          partialize: state => ({
            configs: state.configs,
            activeProvider: state.activeProvider,
          }),
        }
      )
    ),
    { name: 'AIConfigStore' }
  )
);
