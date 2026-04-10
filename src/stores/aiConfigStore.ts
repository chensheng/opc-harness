import { create } from 'zustand'
import { persist } from 'zustand/middleware'
import { immer } from 'zustand/middleware/immer'
import type { AIConfig, AIProvider } from '@/types'

interface AIConfigWithTimestamp extends AIConfig {
  lastModified?: number
  validated?: boolean // 标记是否已通过验证
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
  markAsValidated: (provider: string) => void
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
      // Kimi K2.5 - 最新全能旗舰模型
      { id: 'kimi-k2.5', name: 'Kimi K2.5（全能旗舰）', maxTokens: 256000 },

      // Kimi K2 - 高性能模型
      { id: 'kimi-k2', name: 'Kimi K2（高性能）', maxTokens: 256000 },
      { id: 'kimi-k2-thinking', name: 'Kimi K2 Thinking（深度思考）', maxTokens: 256000 },

      // Kimi Code - 专用编程模型
      { id: 'kimi-code', name: 'Kimi Code（编程专用）', maxTokens: 256000 },
    ],
  },
  {
    id: 'glm',
    name: '智谱 AI (GLM)',
    baseUrl: 'https://open.bigmodel.cn/api/paas/v4',
    models: [
      // GLM-5 系列 - 最新旗舰基座
      { id: 'glm-5', name: 'GLM-5（最新旗舰）', maxTokens: 256000 },
      { id: 'glm-5-code', name: 'GLM-5 Code（编程增强）', maxTokens: 256000 },

      // GLM-4.7/4.6 系列 - 旗舰文本模型
      { id: 'glm-4.7', name: 'GLM-4.7（旗舰文本）', maxTokens: 256000 },
      { id: 'glm-4.6', name: 'GLM-4.6（代码生成）', maxTokens: 128000 },
    ],
  },
  {
    id: 'minimax',
    name: 'MiniMax',
    baseUrl: 'https://api.minimaxi.com/v1',
    models: [
      // M2.7 系列 - 最新旗舰
      { id: 'MiniMax-M2.7', name: 'MiniMax-M2.7（最新旗舰）', maxTokens: 256000 },
      { id: 'MiniMax-M2.7-highspeed', name: 'MiniMax-M2.7 HighSpeed（高速版）', maxTokens: 256000 },

      // M2.5 系列 - 性能与性价比
      { id: 'MiniMax-M2.5', name: 'MiniMax-M2.5（性能旗舰）', maxTokens: 256000 },
      { id: 'MiniMax-M2.5-highspeed', name: 'MiniMax-M2.5 HighSpeed（高速版）', maxTokens: 256000 },
    ],
  },
  {
    id: 'codefree',
    name: 'CodeFree CLI',
    baseUrl: 'cli://codefree',
    models: [
      { id: 'default', name: '默认模型', maxTokens: 128000 },
      { id: 'gpt-4o', name: 'GPT-4o (通过 CodeFree)', maxTokens: 128000 },
      { id: 'claude-3.5-sonnet', name: 'Claude 3.5 Sonnet (通过 CodeFree)', maxTokens: 200000 },
      { id: 'kimi-k2.5', name: 'Kimi K2.5 (通过 CodeFree)', maxTokens: 256000 },
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
            // 如果删除的是默认提供商，重置为 openai
            if (state.defaultProvider === provider) {
              state.defaultProvider = 'openai'
            }
          }),

        setDefaultProvider: provider =>
          set(state => {
            const config = state.configs[provider]
            // 只有已配置且验证通过的才能设为默认
            if (config && config.validated) {
              state.defaultProvider = provider
            } else {
              console.warn(
                `[AIConfig] Cannot set ${provider} as default: not configured or not validated`
              )
            }
          }),

        markAsValidated: provider =>
          set(state => {
            if (state.configs[provider]) {
              state.configs[provider].validated = true
            }
          }),

        getConfig: provider => get().configs[provider],

        getActiveConfig: () => {
          const { defaultProvider, configs } = get()

          // 1. 优先返回 defaultProvider 的配置
          if (configs[defaultProvider]?.apiKey) {
            return configs[defaultProvider]
          }

          // 2. 如果 defaultProvider 没有配置，查找所有已配置的 provider
          const configuredProviders = Object.entries(configs)
            .filter(([_, config]) => config?.apiKey)
            .sort(([_, configA], [__, configB]) => {
              // 按 lastModified 降序排序，最新的在前
              const timeA = configA?.lastModified || 0
              const timeB = configB?.lastModified || 0
              return timeB - timeA
            })

          if (configuredProviders.length > 0) {
            // 返回最新配置的那个 provider
            const [latestProvider, latestConfig] = configuredProviders[0]
            console.log(
              '[AIConfig] Using provider:',
              latestProvider,
              'configured at:',
              new Date(latestConfig.lastModified || 0).toLocaleString()
            )
            return latestConfig
          }

          // 3. 没有任何配置，返回 undefined
          return undefined
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
