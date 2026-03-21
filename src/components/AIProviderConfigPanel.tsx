import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import {
  Key,
  Globe,
  Cpu,
  CheckCircle2,
  XCircle,
  Plus,
  Trash2,
  Edit,
  ExternalLink,
  Eye,
  EyeOff,
  Shield,
} from 'lucide-react';
import { Button } from './ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from './ui/card';
import { Input } from './ui/input';
import { Label } from './ui/label';
import { Switch } from './ui/switch';
import { Badge } from './ui/badge';
import { Tabs, TabsContent, TabsList, TabsTrigger } from './ui/tabs';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from './ui/select';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from './ui/dialog';
import { AIConfig, AIProviderMeta, AIProvider } from '@/types';

/**
 * AI 厂商配置面板组件 (VD-001)
 *
 * 提供 AI 厂商配置的完整管理界面：
 * - 显示所有支持的 AI 厂商信息
 * - 添加/编辑/删除厂商配置
 * - 验证 API 密钥有效性
 * - 切换启用的厂商
 */
export function AIProviderConfigPanel() {
  // 所有可用的 AI 厂商元数据
  const [providers, setProviders] = useState<AIProviderMeta[]>([]);

  // 当前已保存的配置列表
  const [configs, setConfigs] = useState<AIConfig[]>([]);

  // 加载状态
  const [loading, setLoading] = useState(true);

  // 编辑/新增对话框状态
  const [editingConfig, setEditingConfig] = useState<AIConfig | null>(null);
  const [isEditDialogOpen, setIsEditDialogOpen] = useState(false);

  // 验证状态
  const [validating, setValidating] = useState<string | null>(null);
  const [validationResult, setValidationResult] = useState<Record<string, boolean>>({});
  const [validationMessage, setValidationMessage] = useState<Record<string, string>>({});

  // 显示 API 密钥
  const [showApiKey, setShowApiKey] = useState(false);

  /**
   * 加载 AI 厂商元数据和配置
   */
  useEffect(() => {
    loadProviders();
    loadConfigs();
  }, []);

  const loadProviders = async () => {
    try {
      const providerList = await invoke<AIProviderMeta[]>('get_ai_providers');
      setProviders(providerList);
    } catch (error) {
      console.error('Failed to load AI providers:', error);
    }
  };

  const loadConfigs = async () => {
    try {
      const configList = await invoke<AIConfig[]>('get_ai_configs');
      setConfigs(configList);

      // 检查每个厂商是否有 API 密钥
      const validationPromises = configList.map(async config => {
        const hasKey = await invoke<boolean>('has_ai_api_key', { provider: config.provider });
        return { provider: config.provider, hasKey };
      });

      const results = await Promise.all(validationPromises);
      const validationMap: Record<string, boolean> = {};
      results.forEach(({ provider, hasKey }) => {
        validationMap[provider] = hasKey;
      });
      setValidationResult(validationMap);
    } catch (error) {
      console.error('Failed to load AI configs:', error);
    } finally {
      setLoading(false);
    }
  };

  /**
   * 打开编辑对话框
   */
  const handleEdit = (providerId: string) => {
    const provider = providers.find(p => p.id === providerId);
    if (!provider) return;

    const existingConfig = configs.find(c => c.provider === providerId);

    // 重置显示状态
    setShowApiKey(false);

    if (existingConfig) {
      setEditingConfig(existingConfig);
    } else {
      // 创建新配置
      setEditingConfig({
        provider: providerId as AIProvider,
        baseUrl: provider.defaultBaseUrl,
        model: provider.defaultModel,
        enabled: false,
      });
    }

    setIsEditDialogOpen(true);
  };

  /**
   * 保存配置
   */
  const handleSaveConfig = async () => {
    if (!editingConfig) return;

    try {
      await invoke('save_ai_config', { config: editingConfig });
      await loadConfigs();
      setIsEditDialogOpen(false);
      setEditingConfig(null);
    } catch (error) {
      console.error('Failed to save config:', error);
      alert(`保存失败：${error}`);
    }
  };

  /**
   * 删除配置
   */
  const handleDelete = async (providerId: string) => {
    if (!confirm(`确定要删除 ${providerId} 的配置吗？`)) return;

    try {
      await invoke('remove_ai_config', { provider: providerId });
      await loadConfigs();
    } catch (error) {
      console.error('Failed to delete config:', error);
      alert(`删除失败：${error}`);
    }
  };

  /**
   * 验证 API 密钥 (VD-004)
   */
  const handleValidate = async (providerId: string, apiKey?: string) => {
    if (!apiKey) {
      setValidationMessage(prev => ({ ...prev, [providerId]: '请输入 API 密钥' }));
      return;
    }

    setValidating(providerId);
    try {
      const result = await invoke<boolean>('validate_ai_key', {
        provider: providerId,
        api_key: apiKey,
      });

      setValidationResult(prev => ({ ...prev, [providerId]: result }));

      if (result) {
        setValidationMessage(prev => ({ ...prev, [providerId]: '✓ 密钥有效' }));

        // 如果验证成功，可以选择保存配置
        if (editingConfig) {
          // 更新验证时间
          setEditingConfig({
            ...editingConfig,
            lastVerifiedAt: Date.now(),
            isValid: true,
          });
        }
      } else {
        setValidationMessage(prev => ({
          ...prev,
          [providerId]: '✗ 密钥无效，请检查后重试',
        }));

        if (editingConfig) {
          setEditingConfig({
            ...editingConfig,
            isValid: false,
          });
        }
      }
    } catch (error) {
      console.error('Failed to validate API key:', error);
      const errorMessage = error instanceof Error ? error.message : String(error);
      setValidationMessage(prev => ({
        ...prev,
        [providerId]: `✗ 验证失败：${errorMessage}`,
      }));

      if (editingConfig) {
        setEditingConfig({
          ...editingConfig,
          isValid: false,
        });
      }
    } finally {
      setValidating(null);
    }
  };

  /**
   * 获取厂商的图标
   */
  const getProviderIcon = (providerId: string) => {
    switch (providerId) {
      case 'openai':
        return '🤖';
      case 'anthropic':
        return '🧠';
      case 'kimi':
        return '🌙';
      case 'glm':
        return '🧠';
      default:
        return '⚙️';
    }
  };

  if (loading) {
    return <div className="flex items-center justify-center p-8">加载中...</div>;
  }

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-2xl font-bold">AI 厂商配置</h2>
          <p className="text-muted-foreground">管理和配置不同的 AI 服务提供商</p>
        </div>
      </div>

      <Tabs defaultValue="all" className="w-full">
        <TabsList>
          <TabsTrigger value="all">全部厂商</TabsTrigger>
          <TabsTrigger value="configured">已配置</TabsTrigger>
          <TabsTrigger value="enabled">已启用</TabsTrigger>
        </TabsList>

        <TabsContent value="all" className="space-y-4">
          <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
            {providers.map(provider => {
              const config = configs.find(c => c.provider === provider.id);
              const hasApiKey = validationResult[provider.id];

              return (
                <Card key={provider.id}>
                  <CardHeader>
                    <div className="flex items-start justify-between">
                      <div className="flex items-center gap-2">
                        <span className="text-2xl">{getProviderIcon(provider.id)}</span>
                        <div>
                          <CardTitle className="text-lg">{provider.name}</CardTitle>
                          <CardDescription>{provider.id}</CardDescription>
                        </div>
                      </div>
                      <Badge variant={config ? 'default' : 'secondary'}>
                        {config ? (config.enabled ? '已启用' : '已配置') : '未配置'}
                      </Badge>
                    </div>
                  </CardHeader>

                  <CardContent className="space-y-4">
                    {/* 配置信息 */}
                    {config && (
                      <>
                        <div className="space-y-2">
                          <div className="flex items-center gap-2 text-sm">
                            <Globe className="h-4 w-4" />
                            <span className="truncate">
                              {config.baseUrl || provider.defaultBaseUrl}
                            </span>
                          </div>
                          <div className="flex items-center gap-2 text-sm">
                            <Cpu className="h-4 w-4" />
                            <span>{config.model || provider.defaultModel}</span>
                          </div>
                        </div>

                        {/* API 密钥状态 */}
                        <div className="flex items-center gap-2">
                          <Key className="h-4 w-4" />
                          {hasApiKey ? (
                            <span className="text-green-600 flex items-center gap-1">
                              <CheckCircle2 className="h-3 w-3" />
                              已设置
                            </span>
                          ) : (
                            <span className="text-red-600 flex items-center gap-1">
                              <XCircle className="h-3 w-3" />
                              未设置
                            </span>
                          )}
                        </div>
                      </>
                    )}

                    {/* 操作按钮 */}
                    <div className="flex gap-2">
                      <Button
                        size="sm"
                        variant="outline"
                        onClick={() => handleEdit(provider.id)}
                        className="flex-1"
                      >
                        {config ? <Edit className="h-4 w-4" /> : <Plus className="h-4 w-4" />}
                        {config ? '编辑' : '添加'}
                      </Button>

                      {config && (
                        <Button
                          size="sm"
                          variant="destructive"
                          onClick={() => handleDelete(provider.id)}
                        >
                          <Trash2 className="h-4 w-4" />
                        </Button>
                      )}
                    </div>

                    {/* 外部链接 */}
                    <div className="flex gap-2 pt-2 border-t">
                      {provider.docUrl && (
                        <Button
                          size="sm"
                          variant="ghost"
                          onClick={() => window.open(provider.docUrl, '_blank')}
                          className="flex-1"
                        >
                          <ExternalLink className="h-3 w-3 mr-1" />
                          文档
                        </Button>
                      )}
                      {provider.apiKeyUrl && (
                        <Button
                          size="sm"
                          variant="ghost"
                          onClick={() => window.open(provider.apiKeyUrl, '_blank')}
                          className="flex-1"
                        >
                          <ExternalLink className="h-3 w-3 mr-1" />
                          获取密钥
                        </Button>
                      )}
                    </div>
                  </CardContent>
                </Card>
              );
            })}
          </div>
        </TabsContent>

        <TabsContent value="configured" className="space-y-4">
          <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
            {providers
              .filter(p => configs.some(c => c.provider === p.id))
              .map(provider => {
                const config = configs.find(c => c.provider === provider.id)!;
                const hasApiKey = validationResult[provider.id];

                return (
                  <Card key={provider.id}>
                    <CardHeader>
                      <div className="flex items-center gap-2">
                        <span className="text-2xl">{getProviderIcon(provider.id)}</span>
                        <CardTitle className="text-lg">{provider.name}</CardTitle>
                      </div>
                    </CardHeader>
                    <CardContent className="space-y-2">
                      <div className="text-sm text-muted-foreground">
                        <p>模型：{config.model}</p>
                        <p>API 密钥：{hasApiKey ? '✓' : '✗'}</p>
                        <p>状态：{config.enabled ? '✅ 启用' : '❌ 禁用'}</p>
                      </div>
                      <Button
                        size="sm"
                        variant="outline"
                        onClick={() => handleEdit(provider.id)}
                        className="w-full"
                      >
                        编辑配置
                      </Button>
                    </CardContent>
                  </Card>
                );
              })}
          </div>
        </TabsContent>

        <TabsContent value="enabled" className="space-y-4">
          <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
            {providers
              .filter(p => configs.some(c => c.provider === p.id && c.enabled))
              .map(provider => {
                const config = configs.find(c => c.provider === provider.id && c.enabled)!;

                return (
                  <Card key={provider.id}>
                    <CardHeader>
                      <div className="flex items-center gap-2">
                        <span className="text-2xl">{getProviderIcon(provider.id)}</span>
                        <CardTitle className="text-lg">{provider.name}</CardTitle>
                      </div>
                    </CardHeader>
                    <CardContent>
                      <div className="text-sm">
                        <p>
                          <strong>模型:</strong> {config.model}
                        </p>
                        <p>
                          <strong>Base URL:</strong> {config.baseUrl}
                        </p>
                      </div>
                    </CardContent>
                  </Card>
                );
              })}
            {configs.filter(c => c.enabled).length === 0 && (
              <div className="col-span-full text-center py-8 text-muted-foreground">
                暂无启用的 AI 厂商
              </div>
            )}
          </div>
        </TabsContent>
      </Tabs>

      {/* 编辑/新增对话框 */}
      <Dialog
        open={isEditDialogOpen}
        onOpenChange={open => {
          setIsEditDialogOpen(open);
          if (!open) {
            // 关闭对话框时重置状态
            setShowApiKey(false);
            setEditingConfig(null);
          }
        }}
      >
        <DialogContent className="max-w-2xl">
          <DialogHeader>
            <DialogTitle>
              {configs.some(c => c.provider === editingConfig?.provider) ? '编辑' : '添加'}{' '}
              {editingConfig && getProviderIcon(editingConfig.provider)}{' '}
              {editingConfig && providers.find(p => p.id === editingConfig.provider)?.name}
            </DialogTitle>
            <DialogDescription>配置 AI 厂商的 API 连接信息</DialogDescription>
          </DialogHeader>

          {editingConfig && (
            <div className="space-y-4">
              <div className="grid gap-2">
                <Label htmlFor="api-key">
                  <div className="flex items-center gap-2">
                    <Key className="h-4 w-4" />
                    API 密钥
                  </div>
                </Label>

                <div className="relative">
                  <Input
                    id="api-key"
                    type={showApiKey ? 'text' : 'password'}
                    placeholder="sk-..."
                    value={editingConfig.apiKey || ''}
                    onChange={e => setEditingConfig({ ...editingConfig, apiKey: e.target.value })}
                    className="pr-10"
                  />
                  <Button
                    type="button"
                    variant="ghost"
                    size="sm"
                    onClick={() => setShowApiKey(!showApiKey)}
                    className="absolute right-0 top-0 h-full px-3 hover:bg-transparent"
                  >
                    {showApiKey ? (
                      <EyeOff className="h-4 w-4 text-muted-foreground" />
                    ) : (
                      <Eye className="h-4 w-4 text-muted-foreground" />
                    )}
                  </Button>
                </div>

                {/* 安全提示 */}
                <div className="flex items-start gap-2 text-xs text-muted-foreground">
                  <Shield className="h-3 w-3 mt-0.5" />
                  <p>API 密钥将安全存储在操作系统的密钥管理中，不会保存到数据库或发送到服务器</p>
                </div>

                {/* 密钥状态指示 */}
                {existingConfig && !editingConfig.apiKey && (
                  <div className="flex items-center gap-2 text-xs">
                    {validationResult[editingConfig.provider] ? (
                      <>
                        <CheckCircle2 className="h-3 w-3 text-green-600" />
                        <span className="text-green-600">已存储有效密钥</span>
                      </>
                    ) : (
                      <>
                        <XCircle className="h-3 w-3 text-yellow-600" />
                        <span className="text-yellow-600">已存储密钥（未验证）</span>
                      </>
                    )}
                  </div>
                )}

                {editingConfig.apiKey && (
                  <div className="space-y-2">
                    <Button
                      size="sm"
                      variant="outline"
                      onClick={() => handleValidate(editingConfig.provider, editingConfig.apiKey)}
                      disabled={validating === editingConfig.provider}
                      className="w-full"
                    >
                      {validating === editingConfig.provider ? (
                        <>
                          <div className="animate-spin mr-2 h-4 w-4 border-2 border-primary border-t-transparent rounded-full" />
                          验证中...
                        </>
                      ) : validationResult[editingConfig.provider] ? (
                        <>
                          <CheckCircle2 className="h-4 w-4 text-green-600 mr-2" />
                          <span className="text-green-600">✓ 密钥有效</span>
                        </>
                      ) : validationMessage[editingConfig.provider] ? (
                        <>
                          <XCircle className="h-4 w-4 text-red-600 mr-2" />
                          <span className="text-red-600">
                            {validationMessage[editingConfig.provider]}
                          </span>
                        </>
                      ) : (
                        <>
                          <Key className="h-4 w-4 mr-2" />
                          验证密钥
                        </>
                      )}
                    </Button>

                    {/* 显示最后验证时间 */}
                    {editingConfig.lastVerifiedAt && (
                      <p className="text-xs text-muted-foreground text-center">
                        最后验证时间：
                        {new Date(editingConfig.lastVerifiedAt).toLocaleString('zh-CN')}
                      </p>
                    )}
                  </div>
                )}
              </div>

              <div className="grid gap-2">
                <Label htmlFor="base-url">API 基础 URL</Label>
                <Input
                  id="base-url"
                  placeholder="https://api.example.com/v1"
                  value={editingConfig.baseUrl}
                  onChange={e => setEditingConfig({ ...editingConfig, baseUrl: e.target.value })}
                />
              </div>

              <div className="grid gap-2">
                <Label htmlFor="model">模型</Label>
                <Select
                  value={editingConfig.model}
                  onValueChange={value => setEditingConfig({ ...editingConfig, model: value })}
                >
                  <SelectTrigger>
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    {providers
                      .find(p => p.id === editingConfig.provider)
                      ?.supportedModels.map(model => (
                        <SelectItem key={model} value={model}>
                          {model}
                        </SelectItem>
                      ))}
                  </SelectContent>
                </Select>
              </div>

              <div className="flex items-center space-x-2">
                <Switch
                  id="enabled"
                  checked={editingConfig.enabled}
                  onCheckedChange={checked =>
                    setEditingConfig({ ...editingConfig, enabled: checked })
                  }
                />
                <Label htmlFor="enabled">启用此配置</Label>
              </div>
            </div>
          )}

          <DialogFooter>
            <Button variant="outline" onClick={() => setIsEditDialogOpen(false)}>
              取消
            </Button>
            <Button onClick={handleSaveConfig}>保存</Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  );
}
