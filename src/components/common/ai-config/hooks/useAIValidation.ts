import { useState } from 'react'
import { invoke } from '@tauri-apps/api/core'
import type { ProviderValidationStates } from '../types'
import { generateValidationError } from '../utils'
import { useAIConfigStore } from '@/stores'

interface UseAIValidationProps {
  providers: Array<{
    id: string
    name: string
    models: Array<{ id: string; name: string; maxTokens: number }>
  }>
}

/**
 * AI Key 验证 Hook
 */
export function useAIValidation({ providers }: UseAIValidationProps) {
  const [validating, setValidating] = useState<Record<string, boolean>>({})
  const [validationStatus, setValidationStatus] = useState<ProviderValidationStates>({})
  const { markAsValidated } = useAIConfigStore()

  const handleValidate = async (providerId: string, apiKey: string, selectedModel?: string) => {
    // CodeFree CLI 不需要 API Key，其他 provider 需要
    const isCodeFree = providerId === 'codefree'
    if (!isCodeFree && !apiKey) return

    const provider = providers.find(p => p.id === providerId)
    if (!provider) return

    const model = selectedModel || provider.models[0]?.id

    setValidating(prev => ({ ...prev, [providerId]: true }))
    setValidationStatus(prev => ({
      ...prev,
      [providerId]: { status: null, error: '' },
    }))

    try {
      const result = await invoke<boolean>('validate_ai_key', {
        request: {
          provider: providerId,
          api_key: apiKey,
          model: model || null,
        },
      })

      if (result) {
        setValidationStatus(prev => ({
          ...prev,
          [providerId]: { status: 'success', error: '' },
        }))
        // 验证成功后标记为已验证
        markAsValidated(providerId)
      } else {
        setValidationStatus(prev => ({
          ...prev,
          [providerId]: {
            status: 'error',
            error: `API Key 验证失败：${providerId} 服务返回验证未通过。请检查 API Key 是否正确或是否已过期。`,
          },
        }))
      }
    } catch (err) {
      let errorMessage = '验证请求失败'
      if (err instanceof Error) {
        errorMessage = err.message
      } else if (typeof err === 'string') {
        errorMessage = err
      }

      setValidationStatus(prev => ({
        ...prev,
        [providerId]: {
          status: 'error',
          error: generateValidationError(errorMessage, provider.name),
        },
      }))

      console.error('API Key validation failed:', err)
    } finally {
      setValidating(prev => ({ ...prev, [providerId]: false }))
    }
  }

  const clearValidation = (providerId: string) => {
    setValidationStatus(prev => ({
      ...prev,
      [providerId]: { status: null, error: '' },
    }))
  }

  return {
    validating,
    validationStatus,
    handleValidate,
    clearValidation,
  }
}
