import { invoke } from '@tauri-apps/api/core'
import { useState } from 'react'
import {
  Key,
  Check,
  X,
  Eye,
  EyeOff,
  Cpu,
  ExternalLink,
  MessageSquare,
  Play,
  Square,
  Send,
  ShieldCheck,
  Save,
} from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { Badge } from '@/components/ui/badge'
import { Tabs, TabsList, TabsTrigger, TabsContent } from '@/components/ui/tabs'
import { useAIConfigStore } from '@/stores'
import { useAIStream } from '@/hooks/useAIStream'
import { Textarea } from '@/components/ui/textarea'

interface AIResponse {
  content?: string
  [key: string]: unknown
}

export function AIConfig() {
  const { providers, setConfig, removeConfig, getConfig } = useAIConfigStore()

  const [showKey, setShowKey] = useState<Record<string, boolean>>({})
  const [tempKeys, setTempKeys] = useState<Record<string, string>>({})
  const [selectedModels, setSelectedModels] = useState<Record<string, string>>({})
  const [validating, setValidating] = useState<Record<string, boolean>>({})
  const [validationStatus, setValidationStatus] = useState<
    Record<string, 'success' | 'error' | null>
  >({})
  const [validationError, setValidationError] = useState<Record<string, string>>({})

  // 流式测试状态
  const [testProvider, setTestProvider] = useState<string | null>(null)
  const [testMessage, setTestMessage] = useState('你好，请介绍一下你自己')
  const {
    content: streamContent,
    isComplete,
    isLoading: isStreaming,
    error: streamError,
    startStream,
    stopStream,
    reset: resetStream,
  } = useAIStream()

  // 非流式测试状态
  const [nonStreamTesting, setNonStreamTesting] = useState<string | null>(null)
  const [nonStreamResponse, setNonStreamResponse] = useState<Record<string, string>>({})
  const [nonStreamLoading, setNonStreamLoading] = useState(false)
  const [nonStreamError, setNonStreamError] = useState<string | null>(null)

  // 对厂商进行排序：已配置的排在前面，按最后修改时间倒序
  const sortedProviders = [...providers].sort((a, b) => {
    const configA = getConfig(a.id)
    const configB = getConfig(b.id)

    // 如果都未配置，保持原始顺序
    if (!configA && !configB) return 0

    // 已配置的排在未配置的前面
    if (configA && !configB) return -1
    if (!configA && configB) return 1

    // 都已配置，按最后修改时间倒序
    if (configA && configB) {
      const timeA = configA.lastModified || 0
      const timeB = configB.lastModified || 0
      return timeB - timeA
    }

    return 0
  })

  const handleKeyChange = (providerId: string, value: string) => {
    setTempKeys(prev => ({ ...prev, [providerId]: value }))
    setValidationStatus(prev => ({ ...prev, [providerId]: null }))
    setValidationError(prev => ({ ...prev, [providerId]: '' }))
  }

  const handleModelSelect = (providerId: string, modelId: string) => {
    setSelectedModels(prev => ({ ...prev, [providerId]: modelId }))
  }

  const handleValidate = async (providerId: string) => {
    const key = tempKeys[providerId]
    if (!key) return

    // 获取当前 provider 的配置
    const provider = providers.find(p => p.id === providerId)
    if (!provider) return

    const selectedModel = selectedModels[providerId] || provider.models[0]?.id

    setValidating(prev => ({ ...prev, [providerId]: true }))
    setValidationError(prev => ({ ...prev, [providerId]: '' }))

    try {
      // 调用后端的 validate_ai_key 命令进行真实验证
      const result = await invoke<boolean>('validate_ai_key', {
        request: {
          provider: providerId,
          api_key: key,
          model: selectedModel || null,
        },
      })

      if (result) {
        setValidationStatus(prev => ({ ...prev, [providerId]: 'success' }))
      } else {
        setValidationStatus(prev => ({ ...prev, [providerId]: 'error' }))
        setValidationError(prev => ({
          ...prev,
          [providerId]: `API Key 验证失败：${providerId} 服务返回验证未通过。请检查 API Key 是否正确或是否已过期。`,
        }))
      }
    } catch (err) {
      // 后端验证失败或网络错误
      setValidationStatus(prev => ({ ...prev, [providerId]: 'error' }))

      // 提取详细的错误信息
      let errorMessage = '验证请求失败'
      if (err instanceof Error) {
        errorMessage = err.message
      } else if (typeof err === 'string') {
        errorMessage = err
      }

      // 存储详细错误信息，并提供可能原因的提示
      setValidationError(prev => ({
        ...prev,
        [providerId]: `${errorMessage}\n\n可能原因:\n1. API Key 格式不正确\n2. API Key 已过期或无效\n3. 网络连接问题\n4. ${providerId} API 服务不可用`,
      }))

      console.error('API Key validation failed:', err)
    } finally {
      setValidating(prev => ({ ...prev, [providerId]: false }))
    }
  }

  const handleSave = (providerId: string) => {
    const key = tempKeys[providerId]
    if (!key) return

    const provider = providers.find(p => p.id === providerId)
    if (!provider) return

    setConfig(providerId, {
      provider: providerId,
      model: selectedModels[providerId] || provider.models[0].id,
      apiKey: key,
    })

    setValidationStatus(prev => ({ ...prev, [providerId]: null }))
    setTempKeys(prev => ({ ...prev, [providerId]: '' }))
    setSelectedModels(prev => ({ ...prev, [providerId]: '' }))
  }

  const handleRemove = (providerId: string) => {
    removeConfig(providerId)
  }

  const toggleShowKey = (providerId: string) => {
    setShowKey(prev => ({ ...prev, [providerId]: !prev[providerId] }))
  }

  const handleTestStream = async (providerId: string) => {
    if (testProvider === providerId) {
      // 停止当前测试
      stopStream()
      setTestProvider(null)
    } else {
      // 开始新测试
      resetStream()
      setTestProvider(providerId)

      const config = getConfig(providerId)
      if (!config) return

      try {
        await startStream({
          provider: providerId,
          model: config.model,
          apiKey: config.apiKey,
          messages: [{ role: 'user', content: testMessage }],
        })
      } catch (err) {
        const errorMessage = err instanceof Error ? err.message : '未知错误'
        alert(
          `流式测试失败：${errorMessage}\n\n请检查:\n1. API Key 格式是否正确\n2. API Key 是否有访问权限\n3. 网络连接是否正常`
        )
      }
    }
  }

  const handleTestNonStream = async (providerId: string) => {
    setNonStreamTesting(providerId)
    setNonStreamLoading(true)
    setNonStreamError(null)
    setNonStreamResponse(prev => ({ ...prev, [providerId]: '' }))

    const config = getConfig(providerId)
    if (!config) return

    try {
      // 动态导入 Tauri API
      const core = await import('@tauri-apps/api/core')

      // 根据 provider 类型调用不同的命令
      let command = ''
      switch (providerId) {
        case 'openai':
          command = 'chat_openai'
          break
        case 'anthropic':
          command = 'chat_anthropic'
          break
        case 'kimi':
          command = 'chat_kimi'
          break
        case 'glm':
          command = 'chat_glm'
          break
        case 'minimax':
          command = 'chat_minimax'
          break
        default:
          throw new Error(`不支持的 provider: ${providerId}`)
      }

      const response = await core.invoke<AIResponse>(command, {
        request: {
          provider: providerId,
          model: config.model,
          api_key: config.apiKey,
          messages: [{ role: 'user', content: testMessage }],
          temperature: 0.7,
          max_tokens: 2048,
        },
      })

      setNonStreamResponse(prev => ({
        ...prev,
        [providerId]: response.content || JSON.stringify(response),
      }))
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : '请求失败'
      setNonStreamError(errorMessage)
    } finally {
      setNonStreamLoading(false)
      setNonStreamTesting(null)
    }
  }

  return (
    <div className="max-w-3xl mx-auto space-y-6">
      <div>
        <h1 className="text-2xl font-bold flex items-center gap-2">
          <Cpu className="w-6 h-6" />
          AI 厂商配置
        </h1>
        <p className="text-muted-foreground">配置你的AI服务提供商，支持多家厂商切换使用</p>
      </div>

      <Tabs defaultValue={sortedProviders[0]?.id} className="w-full">
        <TabsList className="grid w-full grid-cols-5 mb-4">
          {sortedProviders.map(provider => (
            <TabsTrigger key={provider.id} value={provider.id}>
              {provider.name}
            </TabsTrigger>
          ))}
        </TabsList>

        {sortedProviders.map(provider => {
          const existingConfig = getConfig(provider.id)
          const isConfigured = !!existingConfig
          const isTesting = testProvider === provider.id

          return (
            <TabsContent key={provider.id} value={provider.id}>
              <Card>
                <CardHeader>
                  <div className="flex items-center justify-between">
                    <div className="flex items-center gap-3">
                      <div className="w-10 h-10 rounded-lg bg-primary/10 flex items-center justify-center">
                        <Key className="w-5 h-5 text-primary" />
                      </div>
                      <div>
                        <CardTitle className="text-lg">{provider.name}</CardTitle>
                        <CardDescription>{provider.models.length} 个模型可用</CardDescription>
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
                  {/* API Key Input */}
                  {!isConfigured ? (
                    <div className="space-y-3">
                      {/* 模型选择 */}
                      <div className="space-y-2">
                        <label className="text-sm font-medium">选择模型</label>
                        <select
                          value={selectedModels[provider.id] || provider.models[0].id}
                          onChange={e => handleModelSelect(provider.id, e.target.value)}
                          className="w-full border rounded-md px-3 py-2 bg-background hover:bg-accent cursor-pointer focus:outline-none focus:ring-2 focus:ring-primary focus:border-transparent"
                        >
                          {provider.models.map(model => (
                            <option key={model.id} value={model.id}>
                              {model.name} ({model.maxTokens.toLocaleString()} tokens)
                            </option>
                          ))}
                        </select>
                      </div>

                      <label className="text-sm font-medium">API Key</label>
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

                      <div className="flex gap-2">
                        <Button
                          variant="outline"
                          onClick={() => handleValidate(provider.id)}
                          disabled={!tempKeys[provider.id] || validating[provider.id]}
                        >
                          <ShieldCheck className="w-4 h-4 mr-2" />
                          {validating[provider.id] ? '验证中...' : '验证'}
                        </Button>

                        <Button
                          onClick={() => handleSave(provider.id)}
                          disabled={validationStatus[provider.id] !== 'success'}
                        >
                          <Save className="w-4 h-4 mr-2" />
                          保存
                        </Button>
                      </div>

                      {/* 验证状态提示 */}
                      {validationStatus[provider.id] === 'success' && (
                        <div className="flex items-center gap-2 text-green-600 text-sm">
                          <Check className="w-4 h-4" />
                          <span>API Key 有效</span>
                        </div>
                      )}
                      {validationStatus[provider.id] === 'error' && (
                        <div className="space-y-2">
                          <div className="flex items-center gap-2 text-red-600 text-sm font-medium">
                            <X className="w-4 h-4" />
                            <span>验证失败</span>
                          </div>
                          <div className="text-sm text-red-500 bg-red-50 p-3 rounded border border-red-100 whitespace-pre-line">
                            {validationError[provider.id]}
                          </div>
                        </div>
                      )}
                    </div>
                  ) : (
                    <div className="space-y-4">
                      <div className="flex items-center justify-between p-3 bg-muted rounded-lg">
                        <div className="space-y-2 w-full">
                          <div className="flex items-center gap-2">
                            <Key className="w-4 h-4 text-muted-foreground" />
                            <span className="text-sm font-mono">
                              {existingConfig.apiKey.slice(0, 8)}...
                              {existingConfig.apiKey.slice(-4)}
                            </span>
                          </div>

                          {/* 模型选择 */}
                          <div className="flex items-center gap-2">
                            <span className="text-xs text-muted-foreground">当前模型:</span>
                            <select
                              value={existingConfig.model}
                              onChange={e => {
                                setConfig(provider.id, {
                                  ...existingConfig,
                                  model: e.target.value,
                                })
                              }}
                              className="text-xs border rounded px-2 py-1 bg-background hover:bg-accent cursor-pointer"
                            >
                              {provider.models.map(model => (
                                <option key={model.id} value={model.id}>
                                  {model.name} ({model.maxTokens.toLocaleString()} tokens)
                                </option>
                              ))}
                            </select>
                          </div>
                        </div>
                        <Button variant="ghost" size="sm" onClick={() => handleRemove(provider.id)}>
                          <X className="w-4 h-4 mr-2" />
                          删除
                        </Button>
                      </div>

                      {/* 流式测试区域 */}
                      <div className="border-t pt-4">
                        <div className="flex items-center gap-2 mb-3">
                          <MessageSquare className="w-4 h-4 text-primary" />
                          <span className="text-sm font-medium">流式测试</span>
                        </div>

                        <div className="space-y-2">
                          <Textarea
                            value={testMessage}
                            onChange={e => setTestMessage(e.target.value)}
                            placeholder="输入测试消息..."
                            disabled={isStreaming}
                            rows={2}
                          />

                          <div className="flex gap-2">
                            <Button
                              onClick={() => handleTestStream(provider.id)}
                              disabled={isStreaming && !isTesting}
                              variant={isTesting ? 'destructive' : 'default'}
                              size="sm"
                            >
                              {isTesting ? (
                                <>
                                  <Square className="w-4 h-4 mr-2" />
                                  停止
                                </>
                              ) : (
                                <>
                                  <Play className="w-4 h-4 mr-2" />
                                  测试流式
                                </>
                              )}
                            </Button>

                            <Button
                              onClick={() => handleTestNonStream(provider.id)}
                              disabled={nonStreamLoading || nonStreamTesting === provider.id}
                              variant="outline"
                              size="sm"
                            >
                              {nonStreamTesting === provider.id ? (
                                <>
                                  <Send className="w-4 h-4 mr-2 animate-spin" />
                                  请求中...
                                </>
                              ) : (
                                <>
                                  <Send className="w-4 h-4 mr-2" />
                                  测试非流式
                                </>
                              )}
                            </Button>
                          </div>
                        </div>

                        {/* 流式输出显示 */}
                        {(isTesting || streamContent) && (
                          <div className="mt-3 p-3 bg-muted rounded-lg min-h-[100px]">
                            {isStreaming && (
                              <div className="mb-2 text-xs text-muted-foreground animate-pulse">
                                AI 正在思考中...
                              </div>
                            )}
                            <div className="text-sm whitespace-pre-wrap">
                              {streamContent || '等待响应...'}
                            </div>
                            {isComplete && (
                              <Badge className="mt-2 bg-green-500">
                                <Check className="w-3 h-3 mr-1" />
                                完成
                              </Badge>
                            )}
                            {streamError && (
                              <div className="mt-2 text-sm text-red-600">
                                <div className="font-semibold mb-1 flex items-center gap-1">
                                  <X className="w-4 h-4" />
                                  错误
                                </div>
                                <div className="font-mono text-xs">{streamError}</div>
                              </div>
                            )}
                          </div>
                        )}

                        {/* 非流式输出显示 */}
                        {(nonStreamTesting === provider.id || nonStreamResponse[provider.id]) && (
                          <div className="mt-3 p-3 bg-muted rounded-lg min-h-[100px]">
                            {nonStreamLoading && nonStreamTesting === provider.id && (
                              <div className="mb-2 text-xs text-muted-foreground animate-pulse">
                                AI 正在思考中...
                              </div>
                            )}
                            <div className="text-sm whitespace-pre-wrap">
                              {nonStreamResponse[provider.id] || '等待响应...'}
                            </div>
                            {!nonStreamLoading && nonStreamResponse[provider.id] && (
                              <Badge className="mt-2 bg-green-500">
                                <Check className="w-3 h-3 mr-1" />
                                完成
                              </Badge>
                            )}
                            {nonStreamError && nonStreamTesting === provider.id && (
                              <div className="mt-2 text-sm text-red-600">
                                <div className="font-semibold mb-1 flex items-center gap-1">
                                  <X className="w-4 h-4" />
                                  错误
                                </div>
                                <div className="font-mono text-xs">{nonStreamError}</div>
                              </div>
                            )}
                          </div>
                        )}
                      </div>
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
            </TabsContent>
          )
        })}
      </Tabs>
    </div>
  )
}
