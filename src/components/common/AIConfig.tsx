import { useState } from 'react'
import { Cpu, Check, Star } from 'lucide-react'
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Tabs, TabsList, TabsTrigger, TabsContent } from '@/components/ui/tabs'
import { useAIConfigStore } from '@/stores'
import { useAIStream } from '@/hooks/useAIStream'
import { useAIValidation } from './ai-config/hooks/useAIValidation'
import { useAITesting } from './ai-config/hooks/useAITesting'
import { ProviderConfigForm } from './ai-config/components/ProviderConfigForm'
import { ProviderConfigured } from './ai-config/components/ProviderConfigured'
import { DeleteConfirmDialog } from './ai-config/components/DeleteConfirmDialog'

/**
 * AI 厂商配置主组件
 * 负责协调各个子组件和状态管理
 */
export function AIConfig() {
  const {
    providers,
    setConfig,
    removeConfig,
    getConfig,
    defaultProvider,
    setDefaultProvider,
    markAsValidated,
  } = useAIConfigStore()

  // UI 状态
  const [showKey, setShowKey] = useState<Record<string, boolean>>({})
  const [tempKeys, setTempKeys] = useState<Record<string, string>>({})
  const [selectedModels, setSelectedModels] = useState<Record<string, string>>({})
  const [deleteDialogOpen, setDeleteDialogOpen] = useState(false)
  const [providerToDelete, setProviderToDelete] = useState<string | null>(null)

  // 使用自定义 Hooks
  const { validating, validationStatus, handleValidate, clearValidation } = useAIValidation({
    providers,
  })

  const {
    testProvider,
    setTestProvider,
    testMessage,
    setTestMessage,
    nonStreamResponse,
    nonStreamLoading,
    nonStreamError,
    setNonStreamError,
    handleTestNonStream,
    clearNonStreamResponse,
  } = useAITesting({ getConfig })

  // 流式测试
  const {
    content: streamContent,
    isComplete,
    isLoading: isStreaming,
    error: streamError,
    startStream,
    stopStream,
    reset: resetStream,
  } = useAIStream()

  // 对厂商进行排序：已配置的排在前面，按最后修改时间倒序
  const sortedProviders = [...providers].sort((a, b) => {
    const configA = getConfig(a.id)
    const configB = getConfig(b.id)

    if (!configA && !configB) return 0
    if (configA && !configB) return -1
    if (!configA && configB) return 1

    if (configA && configB) {
      const timeA = configA.lastModified || 0
      const timeB = configB.lastModified || 0
      return timeB - timeA
    }

    return 0
  })

  // 事件处理函数
  const handleKeyChange = (providerId: string, value: string) => {
    setTempKeys(prev => ({ ...prev, [providerId]: value }))
    clearValidation(providerId)
  }

  const handleModelSelect = (providerId: string, modelId: string) => {
    setSelectedModels(prev => ({ ...prev, [providerId]: modelId }))
  }

  const handleSave = (providerId: string) => {
    // CodeFree CLI 不需要 API Key，其他 provider 需要
    const isCodeFree = providerId === 'codefree'
    const key = tempKeys[providerId]

    if (!isCodeFree && !key) return

    const provider = providers.find(p => p.id === providerId)
    if (!provider) return

    setConfig(providerId, {
      provider: providerId,
      model: selectedModels[providerId] || provider.models[0].id,
      apiKey: key || '', // CodeFree 使用空字符串作为 apiKey
    })

    // 保存后自动标记为已验证，允许设为默认
    markAsValidated(providerId)

    clearValidation(providerId)
    setTempKeys(prev => ({ ...prev, [providerId]: '' }))
    setSelectedModels(prev => ({ ...prev, [providerId]: '' }))
  }

  const handleRemove = (providerId: string) => {
    setProviderToDelete(providerId)
    setDeleteDialogOpen(true)
  }

  const handleConfirmDelete = () => {
    if (providerToDelete) {
      removeConfig(providerToDelete)
      setDeleteDialogOpen(false)
      setProviderToDelete(null)
    }
  }

  const handleCancelDelete = () => {
    setDeleteDialogOpen(false)
    setProviderToDelete(null)
  }

  const toggleShowKey = (providerId: string) => {
    setShowKey(prev => ({ ...prev, [providerId]: !prev[providerId] }))
  }

  const handleTestStream = async (providerId: string) => {
    if (testProvider === providerId) {
      stopStream()
      setTestProvider(null)
    } else {
      resetStream()
      setTestProvider(providerId)

      const config = getConfig(providerId)
      if (!config) return

      try {
        // 直接调用 startStream，它会内部调用 stream_chat 命令
        await startStream({
          provider: providerId,
          model: config.model,
          apiKey: config.apiKey,
          messages: [{ role: 'user', content: testMessage }],
        })
      } catch (err) {
        console.error('[handleTestStream] 启动流式测试失败:', err)
      }
    }
  }

  const handleSetDefaultProvider = (providerId: string) => {
    const config = getConfig(providerId)
    if (config && config.validated) {
      setDefaultProvider(providerId)
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
        <TabsList className="grid w-full grid-cols-6 mb-4">
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
          const validationState = validationStatus[provider.id]
          const isDefault = defaultProvider === provider.id

          return (
            <TabsContent key={provider.id} value={provider.id}>
              <Card>
                <CardHeader>
                  <div className="flex items-center justify-between">
                    <div className="flex items-center gap-3">
                      <div className="w-10 h-10 rounded-lg bg-primary/10 flex items-center justify-center">
                        <Cpu className="w-5 h-5 text-primary" />
                      </div>
                      <div>
                        <CardTitle className="text-lg">{provider.name}</CardTitle>
                        {/* CodeFree CLI 不显示模型数量 */}
                        {provider.id !== 'codefree' && (
                          <CardDescription>{provider.models.length} 个模型可用</CardDescription>
                        )}
                      </div>
                    </div>
                    <div className="flex items-center gap-2">
                      {isConfigured && (
                        <Badge className="bg-green-500">
                          <Check className="w-3 h-3 mr-1" />
                          已配置
                        </Badge>
                      )}
                      {isDefault && isConfigured && (
                        <Badge className="bg-yellow-500">
                          <Star className="w-3 h-3 mr-1 fill-current" />
                          默认
                        </Badge>
                      )}
                    </div>
                  </div>
                </CardHeader>

                <CardContent className="space-y-4">
                  {!isConfigured ? (
                    <ProviderConfigForm
                      provider={provider}
                      selectedModel={selectedModels[provider.id]}
                      apiKey={tempKeys[provider.id] || ''}
                      showKey={showKey[provider.id] || false}
                      isValidating={validating[provider.id] || false}
                      validationStatus={validationState?.status || null}
                      validationError={validationState?.error || ''}
                      onModelSelect={modelId => handleModelSelect(provider.id, modelId)}
                      onKeyChange={value => handleKeyChange(provider.id, value)}
                      onToggleShowKey={() => toggleShowKey(provider.id)}
                      onValidate={() =>
                        handleValidate(
                          provider.id,
                          tempKeys[provider.id] || '',
                          selectedModels[provider.id]
                        )
                      }
                      onSave={() => handleSave(provider.id)}
                    />
                  ) : (
                    <ProviderConfigured
                      provider={provider}
                      existingConfig={existingConfig}
                      isDefault={isDefault}
                      isTesting={isTesting}
                      testMessage={testMessage}
                      streamContent={streamContent}
                      isStreaming={isStreaming}
                      isComplete={isComplete}
                      streamError={streamError}
                      nonStreamResponse={nonStreamResponse[provider.id] || ''}
                      nonStreamLoading={nonStreamLoading}
                      nonStreamError={nonStreamError}
                      onModelChange={model => setConfig(provider.id, { ...existingConfig, model })}
                      onRemove={() => handleRemove(provider.id)}
                      onSetDefault={() => handleSetDefaultProvider(provider.id)}
                      onTestStream={() => handleTestStream(provider.id)}
                      _onStopStream={() => {
                        stopStream()
                        setTestProvider(null)
                      }}
                      onResetStream={() => {
                        resetStream()
                        setTestProvider(null)
                      }}
                      onTestMessageChange={setTestMessage}
                      onTestNonStream={() => handleTestNonStream(provider.id)}
                      onClearNonStreamResponse={() => clearNonStreamResponse(provider.id)}
                      onClearNonStreamError={() => setNonStreamError(null)}
                    />
                  )}
                </CardContent>
              </Card>
            </TabsContent>
          )
        })}
      </Tabs>

      <DeleteConfirmDialog
        open={deleteDialogOpen}
        onOpenChange={setDeleteDialogOpen}
        onConfirm={handleConfirmDelete}
        onCancel={handleCancelDelete}
      />
    </div>
  )
}
