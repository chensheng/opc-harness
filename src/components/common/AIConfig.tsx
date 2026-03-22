import { useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { 
  Key, 
  Check, 
  X, 
  Eye, 
  EyeOff, 
  Cpu,
  ExternalLink
} from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { Badge } from '@/components/ui/badge'
import { useAIConfigStore } from '@/stores'

export function AIConfig() {
  const navigate = useNavigate()
  const { providers, configs, setConfig, removeConfig, getConfig } = useAIConfigStore()
  
  const [showKey, setShowKey] = useState<Record<string, boolean>>({})
  const [tempKeys, setTempKeys] = useState<Record<string, string>>({})
  const [validating, setValidating] = useState<Record<string, boolean>>({})
  const [validationStatus, setValidationStatus] = useState<Record<string, 'success' | 'error' | null>>({})

  const handleKeyChange = (providerId: string, value: string) => {
    setTempKeys(prev => ({ ...prev, [providerId]: value }))
    setValidationStatus(prev => ({ ...prev, [providerId]: null }))
  }

  const handleValidate = async (providerId: string) => {
    const key = tempKeys[providerId]
    if (!key) return

    setValidating(prev => ({ ...prev, [providerId]: true }))
    
    // Simulate validation
    await new Promise(resolve => setTimeout(resolve, 1000))
    
    // Mock validation - in real app, this would call the backend
    const isValid = key.length > 10
    
    setValidationStatus(prev => ({ ...prev, [providerId]: isValid ? 'success' : 'error' }))
    setValidating(prev => ({ ...prev, [providerId]: false }))
  }

  const handleSave = (providerId: string) => {
    const key = tempKeys[providerId]
    if (!key) return

    const provider = providers.find(p => p.id === providerId)
    if (!provider) return

    setConfig(providerId, {
      provider: providerId,
      model: provider.models[0].id,
      apiKey: key,
    })

    setValidationStatus(prev => ({ ...prev, [providerId]: null }))
    setTempKeys(prev => ({ ...prev, [providerId]: '' }))
  }

  const handleRemove = (providerId: string) => {
    removeConfig(providerId)
  }

  const toggleShowKey = (providerId: string) => {
    setShowKey(prev => ({ ...prev, [providerId]: !prev[providerId] }))
  }

  return (
    <div className="max-w-3xl mx-auto space-y-6">
      <div>
        <h1 className="text-2xl font-bold flex items-center gap-2">
          <Cpu className="w-6 h-6" />
          AI 厂商配置
        </h1>
        <p className="text-muted-foreground">
          配置你的AI服务提供商，支持多家厂商切换使用
        </p>
      </div>

      <div className="space-y-4">
        {providers.map(provider => {
          const existingConfig = getConfig(provider.id)
          const isConfigured = !!existingConfig

          return (
            <Card key={provider.id}>
              <CardHeader>
                <div className="flex items-center justify-between">
                  <div className="flex items-center gap-3">
                    <div className="w-10 h-10 rounded-lg bg-primary/10 flex items-center justify-center">
                      <Key className="w-5 h-5 text-primary" />
                    </div>
                    <div>
                      <CardTitle className="text-lg">{provider.name}</CardTitle>
                      <CardDescription>
                        {provider.models.length} 个模型可用
                      </CardDescription>
                    </div>
                  </div>
                  {isConfigured && (
                    <Badge className="bg-green-500">
                      <Check className="w-3 h-3 mr-1" />
                      已配置
                    </Badge>
                  )}
                </div>
              </CardHeader>
              
              <CardContent className="space-y-4">
                {/* Available Models */}
                <div>
                  <p className="text-sm font-medium mb-2">可用模型</p>
                  <div className="flex flex-wrap gap-2">
                    {provider.models.map(model => (
                      <Badge key={model.id} variant="outline">
                        {model.name}
                      </Badge>
                    ))}
                  </div>
                </div>

                {/* API Key Input */}
                {!isConfigured ? (
                  <div className="space-y-3">
                    <label className="text-sm font-medium">API Key</label>
                    <div className="flex gap-2">
                      <div className="relative flex-1">
                        <Input
                          type={showKey[provider.id] ? 'text' : 'password'}
                          value={tempKeys[provider.id] || ''}
                          onChange={e => handleKeyChange(provider.id, e.target.value)}
                          placeholder={`输入 ${provider.name} API Key`}
                        />
                        <button
                          onClick={() => toggleShowKey(provider.id)}
                          className="absolute right-3 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground"
                        >
                          {showKey[provider.id] ? (
                            <EyeOff className="w-4 h-4" />
                          ) : (
                            <Eye className="w-4 h-4" />
                          )}
                        </button>
                      </div>
                      <Button
                        variant="outline"
                        onClick={() => handleValidate(provider.id)}
                        disabled={!tempKeys[provider.id] || validating[provider.id]}
                      >
                        {validating[provider.id] ? '验证中...' : '验证'}
                      </Button>
                      <Button
                        onClick={() => handleSave(provider.id)}
                        disabled={validationStatus[provider.id] !== 'success'}
                      >
                        保存
                      </Button>
                    </div>
                    
                    {validationStatus[provider.id] === 'success' && (
                      <p className="text-sm text-green-600 flex items-center gap-1">
                        <Check className="w-4 h-4" />
                        API Key 验证成功
                      </p>
                    )}
                    {validationStatus[provider.id] === 'error' && (
                      <p className="text-sm text-red-600 flex items-center gap-1">
                        <X className="w-4 h-4" />
                        API Key 验证失败，请检查后重试
                      </p>
                    )}
                    
                    <p className="text-xs text-muted-foreground">
                      你的API Key将被安全存储在系统钥匙串中，不会上传到任何服务器
                    </p>
                  </div>
                ) : (
                  <div className="flex items-center justify-between p-3 bg-muted rounded-lg">
                    <div className="flex items-center gap-2">
                      <Key className="w-4 h-4 text-muted-foreground" />
                      <span className="text-sm font-mono">
                        {existingConfig.apiKey.slice(0, 8)}...{existingConfig.apiKey.slice(-4)}
                      </span>
                    </div>
                    <Button
                      variant="ghost"
                      size="sm"
                      onClick={() => handleRemove(provider.id)}
                    >
                      <X className="w-4 h-4 mr-2" />
                      删除
                    </Button>
                  </div>
                )}

                {/* Documentation Link */}
                <a
                  href={provider.baseUrl}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="inline-flex items-center text-sm text-primary hover:underline"
                >
                  查看文档
                  <ExternalLink className="w-3 h-3 ml-1" />
                </a>
              </CardContent>
            </Card>
          )
        })}
      </div>
    </div>
  )
}
