import React, { useState } from 'react'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Label } from '@/components/ui/label'
import { Button } from '@/components/ui/button'
import { Switch } from '@/components/ui/switch'
import { Slider } from '@/components/ui/slider'
import { invoke } from '@tauri-apps/api/core'
import { RefreshCw, Settings2 } from 'lucide-react'

interface RetryConfigPanelProps {
  projectId: string
  initialConfig?: {
    maxRetries: number
    baseDelaySeconds: number
    maxDelaySeconds: number
    autoRetryEnabled: boolean
  }
  onConfigChange?: (config: RetryConfig) => void
}

interface RetryConfig {
  maxRetries: number
  baseDelaySeconds: number
  maxDelaySeconds: number
  autoRetryEnabled: boolean
}

export function RetryConfigPanel({
  projectId,
  initialConfig,
  onConfigChange,
}: RetryConfigPanelProps) {
  const [config, setConfig] = useState<RetryConfig>(
    initialConfig || {
      maxRetries: 3,
      baseDelaySeconds: 60,
      maxDelaySeconds: 3600,
      autoRetryEnabled: true,
    }
  )

  const [isSaving, setIsSaving] = useState(false)

  const handleSave = async () => {
    // 验证配置
    if (config.maxDelaySeconds < config.baseDelaySeconds) {
      alert('最大延迟时间不能小于基础延迟时间')
      return
    }

    setIsSaving(true)
    try {
      await invoke('update_user_story_retry_config', {
        request: {
          projectId,
          maxRetries: config.maxRetries,
          baseDelaySeconds: config.baseDelaySeconds,
          maxDelaySeconds: config.maxDelaySeconds,
        },
      })

      alert('重试配置已保存')
      onConfigChange?.(config)
    } catch (error) {
      console.error('Failed to save retry config:', error)
      alert('保存失败: ' + (error as Error).message)
    } finally {
      setIsSaving(false)
    }
  }

  const handleReset = () => {
    setConfig({
      maxRetries: 3,
      baseDelaySeconds: 60,
      maxDelaySeconds: 3600,
      autoRetryEnabled: true,
    })
    alert('已恢复默认配置')
  }

  return (
    <Card>
      <CardHeader>
        <div className="flex items-center gap-2">
          <Settings2 className="w-5 h-5" />
          <CardTitle>重试策略配置</CardTitle>
        </div>
        <CardDescription>配置用户故事失败后的自动重试行为</CardDescription>
      </CardHeader>
      <CardContent className="space-y-6">
        {/* 自动重试开关 */}
        <div className="flex items-center justify-between">
          <div className="space-y-0.5">
            <Label>启用自动重试</Label>
            <p className="text-sm text-muted-foreground">失败后自动根据错误类型决定是否重试</p>
          </div>
          <Switch
            checked={config.autoRetryEnabled}
            onCheckedChange={checked => setConfig({ ...config, autoRetryEnabled: checked })}
          />
        </div>

        {/* 最大重试次数 */}
        <div className="space-y-2">
          <div className="flex justify-between">
            <Label>最大重试次数</Label>
            <span className="text-sm text-muted-foreground">{config.maxRetries} 次</span>
          </div>
          <Slider
            value={[config.maxRetries]}
            min={1}
            max={10}
            step={1}
            onValueChange={([value]) => setConfig({ ...config, maxRetries: value })}
          />
          <p className="text-xs text-muted-foreground">超过此次数后将标记为永久失败</p>
        </div>

        {/* 基础延迟时间 */}
        <div className="space-y-2">
          <div className="flex justify-between">
            <Label>基础延迟时间</Label>
            <span className="text-sm text-muted-foreground">
              {config.baseDelaySeconds} 秒 ({Math.floor(config.baseDelaySeconds / 60)} 分钟)
            </span>
          </div>
          <Slider
            value={[config.baseDelaySeconds]}
            min={30}
            max={300}
            step={10}
            onValueChange={([value]) => setConfig({ ...config, baseDelaySeconds: value })}
          />
          <p className="text-xs text-muted-foreground">首次重试的等待时间，后续按指数增长</p>
        </div>

        {/* 最大延迟时间 */}
        <div className="space-y-2">
          <div className="flex justify-between">
            <Label>最大延迟时间</Label>
            <span className="text-sm text-muted-foreground">
              {config.maxDelaySeconds} 秒 ({Math.floor(config.maxDelaySeconds / 60)} 分钟)
            </span>
          </div>
          <Slider
            value={[config.maxDelaySeconds]}
            min={300}
            max={7200}
            step={300}
            onValueChange={([value]) => setConfig({ ...config, maxDelaySeconds: value })}
          />
          <p className="text-xs text-muted-foreground">重试延迟的上限，避免等待过久</p>
        </div>

        {/* 操作按钮 */}
        <div className="flex gap-2 pt-4 border-t">
          <Button onClick={handleSave} disabled={isSaving} className="flex-1">
            {isSaving ? (
              <>
                <RefreshCw className="w-4 h-4 mr-2 animate-spin" />
                保存中...
              </>
            ) : (
              '保存配置'
            )}
          </Button>
          <Button variant="outline" onClick={handleReset}>
            恢复默认
          </Button>
        </div>

        {/* 配置说明 */}
        <div className="bg-blue-50 dark:bg-blue-950/20 p-4 rounded-lg space-y-2">
          <p className="text-sm font-medium text-blue-900 dark:text-blue-100">💡 重试策略说明</p>
          <ul className="text-xs text-blue-800 dark:text-blue-200 space-y-1 list-disc list-inside">
            <li>临时错误（网络超时、API 限流）会自动重试</li>
            <li>永久错误（代码错误、依赖缺失）直接终止</li>
            <li>
              延迟时间按指数退避：{config.baseDelaySeconds}s → {config.baseDelaySeconds * 2}s →{' '}
              {config.baseDelaySeconds * 4}s...
            </li>
            <li>每次延迟添加 ±10% 随机抖动，避免并发冲突</li>
          </ul>
        </div>
      </CardContent>
    </Card>
  )
}
