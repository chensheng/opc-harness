import { Eye, EyeOff, ShieldCheck, Save, Check } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { formatTokens } from '../utils'

interface ProviderConfigFormProps {
  provider: {
    id: string
    name: string
    models: Array<{ id: string; name: string; maxTokens: number }>
  }
  selectedModel: string
  apiKey: string
  showKey: boolean
  isValidating: boolean
  validationStatus: 'success' | 'error' | null
  validationError: string
  onModelSelect: (modelId: string) => void
  onKeyChange: (value: string) => void
  onToggleShowKey: () => void
  onValidate: () => void
  onSave: () => void
}

/**
 * AI 提供商配置表单组件（未配置状态）
 */
export function ProviderConfigForm({
  provider,
  selectedModel,
  apiKey,
  showKey,
  isValidating,
  validationStatus,
  validationError,
  onModelSelect,
  onKeyChange,
  onToggleShowKey,
  onValidate,
  onSave,
}: ProviderConfigFormProps) {
  return (
    <div className="space-y-3">
      {/* 模型选择 */}
      <div className="space-y-2">
        <label className="text-sm font-medium">选择模型</label>
        <select
          value={selectedModel || provider.models[0].id}
          onChange={e => onModelSelect(e.target.value)}
          className="w-full border rounded-md px-3 py-2 bg-background hover:bg-accent cursor-pointer focus:outline-none focus:ring-2 focus:ring-primary focus:border-transparent"
        >
          {provider.models.map(model => (
            <option key={model.id} value={model.id}>
              {model.name} ({formatTokens(model.maxTokens)})
            </option>
          ))}
        </select>
      </div>

      <label className="text-sm font-medium">API Key</label>
      <div className="relative flex-1">
        <Input
          type={showKey ? 'text' : 'password'}
          value={apiKey}
          onChange={e => onKeyChange(e.target.value)}
          placeholder={`输入 ${provider.name} API Key`}
        />
        <button
          onClick={onToggleShowKey}
          className="absolute right-3 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground"
        >
          {showKey ? <EyeOff className="w-4 h-4" /> : <Eye className="w-4 h-4" />}
        </button>
      </div>

      <div className="flex gap-2">
        <Button variant="outline" onClick={onValidate} disabled={!apiKey || isValidating}>
          <ShieldCheck className="w-4 h-4 mr-2" />
          {isValidating ? '验证中...' : '验证'}
        </Button>

        <Button onClick={onSave} disabled={validationStatus !== 'success'}>
          <Save className="w-4 h-4 mr-2" />
          保存
        </Button>
      </div>

      {/* 验证状态提示 */}
      {validationStatus === 'success' && (
        <div className="flex items-center gap-2 text-green-600 text-sm">
          <Check className="w-4 h-4" />
          <span>API Key 有效</span>
        </div>
      )}
      {validationStatus === 'error' && (
        <div className="space-y-2">
          <div className="flex items-center gap-2 text-red-600 text-sm font-medium">
            <span>验证失败</span>
          </div>
          <div className="text-sm text-red-500 bg-red-50 p-3 rounded border border-red-100 whitespace-pre-line">
            {validationError}
          </div>
        </div>
      )}
    </div>
  )
}
