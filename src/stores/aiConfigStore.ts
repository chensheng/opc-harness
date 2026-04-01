import { create } from 'zustand'
import { persist } from 'zustand/middleware'
import { immer } from 'zustand/middleware/immer'
import type { AIConfig, AIProvider } from '@/types'

interface AIConfigWithTimestamp extends AIConfig {
  lastModified?: number
}

interface AIConfigState {
  providers: AIProvider[]
  configs: Record<string, AIConfigWithTimestamp>
  defaultProvider: string
}

interface AIConfigActions {
  setConfig: (provider: string, config: AIConfigWithTimestamp) => void
  removeConfig: (provider: string) => void
  setDefaultProvider: (provider: string) => void
  getConfig: (provider: string) => AIConfigWithTimestamp | undefined
  getActiveConfig: () => AIConfigWithTimestamp | undefined
}

export const AI_PROVIDERS: AIProvider[] = [
  {
    id: 'openai',
    name: 'OpenAI',
    baseUrl: 'https://api.openai.com/v1',
    models: [
      { id: 'gpt-4o', name: 'GPT-4o', maxTokens: 128000 },
      { id: 'gpt-4o-mini', name: 'GPT-4o Mini', maxTokens: 128000 },
      { id: 'o1-preview', name: 'o1 Preview', maxTokens: 128000 },
      { id: 'o1-mini', name: 'o1 Mini', maxTokens: 128000 },
    ],
  },
  {
    id: 'anthropic',
    name: 'Anthropic Claude',
    baseUrl: 'https://api.anthropic.com/v1',
    models: [
      { id: 'claude-3-5-sonnet-20241022', name: 'Claude 3.5 Sonnet', maxTokens: 200000 },
      { id: 'claude-3-opus-20240229', name: 'Claude 3 Opus', maxTokens: 200000 },
      { id: 'claude-3-sonnet-20240229', name: 'Claude 3 Sonnet', maxTokens: 200000 },
      { id: 'claude-3-haiku-20240307', name: 'Claude 3 Haiku', maxTokens: 200000 },
    ],
  },
  {
    id: 'kimi',
    name: '月之暗面 (Kimi)',
    baseUrl: 'https://api.moonshot.cn/v1',
    models: [
      { id: 'kimi-code', name: 'Kimi Code', maxTokens: 256000 },
      { id: 'kimi-k1.5', name: 'Kimi K1.5', maxTokens: 256000 },
      { id: 'kimi-k1', name: 'Kimi K1', maxTokens: 128000 },
    ],
  },
  {
    id: 'glm',
    name: '智谱 AI (GLM)',
    baseUrl: 'https://open.bigmodel.cn/api/paas/v4',
    models: [
      { id: 'glm-4-plus', name: 'GLM-4 Plus', maxTokens: 128000 },
      { id: 'glm-4', name: 'GLM-4', maxTokens: 128000 },
      { id: 'glm-4-air', name: 'GLM-4 Air', maxTokens: 128000 },
      { id: 'codegeex-4', name: 'CodeGeeX-4', maxTokens: 128000 },
    ],
  },
  {
    id: 'minimax',
    name: 'MiniMax',
    baseUrl: 'https://api.minimaxi.com/v1',
    models: [
      { id: 'speech-2.5-turbo', name: 'Speech 2.5 Turbo', maxTokens: 128000 },
      { id: 'speech-2-turbo', name: 'Speech 2 Turbo', maxTokens: 128000 },
      { id: 'speech-v1', name: 'Speech V1', maxTokens: 128000 },
    ],
  },
]

export const useAIConfigStore = create<AIConfigState & AIConfigActions>()(
  immer(
    persist(
      (set, get) => ({
        providers: AI_PROVIDERS,
        configs: {},
        defaultProvider: 'openai',

        setConfig: (provider, config) =>
          set(state => {
            config.lastModified = Date.now()
            state.configs[provider] = config
          }),

        removeConfig: provider =>
          set(state => {
            delete state.configs[provider]
          }),

        setDefaultProvider: provider =>
          set(state => {
            state.defaultProvider = provider
          }),

        getConfig: provider => get().configs[provider],

        getActiveConfig: () => {
          const { defaultProvider, configs } = get()
          return configs[defaultProvider]
        },
      }),
      {
        name: 'opc-harness-ai-config',
        // 合并策略：确保 providers 始终包含所有最新的厂商
        merge: (persistedState, currentState) => {
          // 始终使用最新的 AI_PROVIDERS
          const mergedState = {
            ...currentState,
            ...(persistedState as object),
            providers: AI_PROVIDERS,
          }
          
          return mergedState as typeof currentState
        },
      }
    )
  )
)
