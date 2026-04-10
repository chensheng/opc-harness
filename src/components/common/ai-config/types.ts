/**
 * AIConfig 组件的类型定义
 */

export interface AIResponse {
  content?: string
  [key: string]: unknown
}

export interface ValidationState {
  status: 'success' | 'error' | null
  error: string
}

export interface ProviderValidationStates {
  [providerId: string]: ValidationState
}
