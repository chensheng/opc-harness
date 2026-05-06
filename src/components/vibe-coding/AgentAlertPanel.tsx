/**
 * Agent 告警面板组件
 *
 * 展示智能体的告警信息，支持告警横幅、详情展开、历史查询和配置界面
 */

import { useState } from 'react'
import { useParams } from 'react-router-dom'
import { invoke } from '@tauri-apps/api/core'
import {
  AlertTriangle,
  AlertCircle,
  CheckCircle,
  Settings,
  Search,
  ChevronDown,
  ChevronUp,
} from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Input } from '@/components/ui/input'
import { Switch } from '@/components/ui/switch'
import { Slider } from '@/components/ui/slider'
import type { AlertLevel } from '@/types/agentObservability'
import { useObservabilityStore } from '@/stores/observabilityStore'

interface AgentAlertPanelProps {
  agentId?: string
}

export function AgentAlertPanel({ agentId: _agentId }: AgentAlertPanelProps) {
  const { projectId: _projectId } = useParams<{ projectId: string }>()

  const [showConfig, setShowConfig] = useState(false)
  const [searchText, setSearchText] = useState('')
  const [expandedAlerts, setExpandedAlerts] = useState<Set<string>>(new Set())

  const activeAlerts = useObservabilityStore(state => state.getActiveAlerts())
  const alertHistory = useObservabilityStore(state => state.getAlertHistory())
  const alertConfig = useObservabilityStore(state => state.alertConfig)
  // const addAlert = useObservabilityStore(state => state.addAlert)
  const resolveAlert = useObservabilityStore(state => state.resolveAlert)
  const updateAlertConfig = useObservabilityStore(state => state.updateAlertConfig)

  // 过滤告警
  const filteredAlerts = activeAlerts.filter(
    alert =>
      alert.message.toLowerCase().includes(searchText.toLowerCase()) ||
      alert.alertType.toLowerCase().includes(searchText.toLowerCase())
  )

  const filteredHistory = alertHistory.filter(
    alert =>
      alert.message.toLowerCase().includes(searchText.toLowerCase()) ||
      alert.alertType.toLowerCase().includes(searchText.toLowerCase())
  )

  // 获取告警级别图标
  const getAlertLevelIcon = (level: AlertLevel) => {
    switch (level) {
      case 'critical':
        return <AlertCircle className="w-5 h-5 text-red-500" />
      case 'warning':
        return <AlertTriangle className="w-5 h-5 text-yellow-500" />
      default:
        return <AlertCircle className="w-5 h-5 text-blue-500" />
    }
  }

  // 获取告警级别标签
  const getAlertLevelLabel = (level: AlertLevel) => {
    switch (level) {
      case 'critical':
        return '严重'
      case 'warning':
        return '警告'
      default:
        return level
    }
  }

  // 获取告警类型标签
  const getAlertTypeLabel = (alertType: string) => {
    switch (alertType) {
      case 'no_response':
        return '无响应'
      case 'error_rate':
        return '错误率'
      case 'cpu_high':
        return 'CPU 使用率高'
      case 'memory_high':
        return '内存使用率高'
      default:
        return alertType
    }
  }

  // 解决告警
  const handleResolveAlert = async (alertId: string) => {
    try {
      await invoke('resolve_agent_alert', { alertId })
      resolveAlert(alertId)
    } catch (error) {
      console.error('Failed to resolve alert:', error)
    }
  }

  // 展开/收起告警详情
  const toggleAlertExpand = (alertId: string) => {
    setExpandedAlerts(prev => {
      const newSet = new Set(prev)
      if (newSet.has(alertId)) {
        newSet.delete(alertId)
      } else {
        newSet.add(alertId)
      }
      return newSet
    })
  }

  // 更新告警配置
  const handleConfigChange = (key: string, value: number | boolean) => {
    updateAlertConfig({
      ...alertConfig,
      thresholds: {
        ...alertConfig.thresholds,
        [key]: value,
      },
    })
  }

  return (
    <Card className="p-4">
      <div className="flex items-center justify-between mb-4">
        <h2 className="text-lg font-semibold flex items-center gap-2">
          <AlertCircle className="w-5 h-5" />
          告警面板
        </h2>

        <div className="flex items-center gap-2">
          <Button variant="outline" size="sm" onClick={() => setShowConfig(!showConfig)}>
            <Settings className="w-4 h-4 mr-2" />
            配置
          </Button>
        </div>
      </div>

      {/* 告警统计 */}
      <div className="grid grid-cols-3 gap-4 mb-4">
        <Card className="p-3 bg-red-50 border-red-200">
          <div className="flex items-center gap-2">
            <AlertCircle className="w-5 h-5 text-red-500" />
            <div>
              <p className="text-2xl font-bold text-red-600">
                {activeAlerts.filter(a => a.level === 'critical').length}
              </p>
              <p className="text-xs text-red-600">严重告警</p>
            </div>
          </div>
        </Card>

        <Card className="p-3 bg-yellow-50 border-yellow-200">
          <div className="flex items-center gap-2">
            <AlertTriangle className="w-5 h-5 text-yellow-500" />
            <div>
              <p className="text-2xl font-bold text-yellow-600">
                {activeAlerts.filter(a => a.level === 'warning').length}
              </p>
              <p className="text-xs text-yellow-600">警告</p>
            </div>
          </div>
        </Card>

        <Card className="p-3 bg-blue-50 border-blue-200">
          <div className="flex items-center gap-2">
            <CheckCircle className="w-5 h-5 text-blue-500" />
            <div>
              <p className="text-2xl font-bold text-blue-600">
                {alertHistory.filter(a => a.status === 'resolved').length}
              </p>
              <p className="text-xs text-blue-600">已解决</p>
            </div>
          </div>
        </Card>
      </div>

      {/* 搜索框 */}
      <div className="relative mb-4">
        <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 w-4 h-4 text-muted-foreground" />
        <Input
          placeholder="搜索告警..."
          value={searchText}
          onChange={e => setSearchText(e.target.value)}
          className="pl-10"
        />
      </div>

      {/* 告警配置面板 */}
      {showConfig && (
        <Card className="p-4 mb-4 bg-muted">
          <h3 className="font-medium mb-4">告警配置</h3>

          <div className="space-y-4">
            <div className="flex items-center justify-between">
              <span className="text-sm">启用告警</span>
              <Switch
                checked={alertConfig.enabled}
                onCheckedChange={checked => updateAlertConfig({ ...alertConfig, enabled: checked })}
              />
            </div>

            <div className="space-y-2">
              <label className="text-sm">无响应警告 (分钟)</label>
              <Slider
                value={[alertConfig.thresholds.noResponseWarningMinutes]}
                onValueChange={([value]) => handleConfigChange('noResponseWarningMinutes', value)}
                min={1}
                max={30}
                step={1}
              />
              <p className="text-xs text-muted-foreground">
                {alertConfig.thresholds.noResponseWarningMinutes} 分钟
              </p>
            </div>

            <div className="space-y-2">
              <label className="text-sm">无响应严重 (分钟)</label>
              <Slider
                value={[alertConfig.thresholds.noResponseCriticalMinutes]}
                onValueChange={([value]) => handleConfigChange('noResponseCriticalMinutes', value)}
                min={5}
                max={60}
                step={5}
              />
              <p className="text-xs text-muted-foreground">
                {alertConfig.thresholds.noResponseCriticalMinutes} 分钟
              </p>
            </div>

            <div className="space-y-2">
              <label className="text-sm">错误率警告 (个/分钟)</label>
              <Slider
                value={[alertConfig.thresholds.errorRateWarningPerMinute]}
                onValueChange={([value]) => handleConfigChange('errorRateWarningPerMinute', value)}
                min={5}
                max={50}
                step={5}
              />
              <p className="text-xs text-muted-foreground">
                {alertConfig.thresholds.errorRateWarningPerMinute} 个/分钟
              </p>
            </div>

            <div className="space-y-2">
              <label className="text-sm">CPU 使用率警告 (%)</label>
              <Slider
                value={[alertConfig.thresholds.cpuHighPercent]}
                onValueChange={([value]) => handleConfigChange('cpuHighPercent', value)}
                min={70}
                max={99}
                step={5}
              />
              <p className="text-xs text-muted-foreground">
                {alertConfig.thresholds.cpuHighPercent}%
              </p>
            </div>

            <div className="space-y-2">
              <label className="text-sm">内存使用率警告 (%)</label>
              <Slider
                value={[alertConfig.thresholds.memoryHighPercent]}
                onValueChange={([value]) => handleConfigChange('memoryHighPercent', value)}
                min={70}
                max={99}
                step={5}
              />
              <p className="text-xs text-muted-foreground">
                {alertConfig.thresholds.memoryHighPercent}%
              </p>
            </div>
          </div>
        </Card>
      )}

      {/* 活跃告警列表 */}
      <div className="mb-4">
        <h3 className="font-medium mb-2">活跃告警 ({filteredAlerts.length})</h3>

        {filteredAlerts.length === 0 ? (
          <div className="text-center py-8 text-muted-foreground">暂无活跃告警</div>
        ) : (
          <div className="space-y-2">
            {filteredAlerts.map(alert => {
              const isExpanded = expandedAlerts.has(alert.id)

              return (
                <Card
                  key={alert.id}
                  className={`p-3 border-l-4 ${
                    alert.level === 'critical'
                      ? 'border-l-red-500 bg-red-50'
                      : 'border-l-yellow-500 bg-yellow-50'
                  }`}
                >
                  <div className="flex items-start gap-3">
                    {getAlertLevelIcon(alert.level)}
                    <div className="flex-1">
                      <div className="flex items-center gap-2 mb-1">
                        <Badge variant={alert.level === 'critical' ? 'destructive' : 'secondary'}>
                          {getAlertLevelLabel(alert.level)}
                        </Badge>
                        <Badge variant="outline">{getAlertTypeLabel(alert.alertType)}</Badge>
                        <span className="text-xs text-muted-foreground">
                          {alert.createdAt.toLocaleString()}
                        </span>
                      </div>

                      <p className="text-sm font-medium">{alert.message}</p>

                      <div className="flex items-center gap-2 mt-2">
                        <Button
                          variant="outline"
                          size="sm"
                          onClick={() => toggleAlertExpand(alert.id)}
                        >
                          {isExpanded ? (
                            <ChevronUp className="w-4 h-4" />
                          ) : (
                            <ChevronDown className="w-4 h-4" />
                          )}
                          详情
                        </Button>

                        <Button
                          variant="outline"
                          size="sm"
                          onClick={() => handleResolveAlert(alert.id)}
                        >
                          <CheckCircle className="w-4 h-4 mr-1" />
                          解决
                        </Button>
                      </div>

                      {isExpanded && (
                        <div className="mt-3 p-3 bg-white rounded border">
                          <div className="space-y-2 text-sm">
                            <div>
                              <span className="text-muted-foreground">告警 ID:</span> {alert.id}
                            </div>
                            <div>
                              <span className="text-muted-foreground">智能体 ID:</span>{' '}
                              {alert.agentId}
                            </div>
                            <div>
                              <span className="text-muted-foreground">类型:</span>{' '}
                              {getAlertTypeLabel(alert.alertType)}
                            </div>
                            <div>
                              <span className="text-muted-foreground">状态:</span>{' '}
                              {alert.status === 'active' ? '活跃' : '已解决'}
                            </div>
                          </div>
                        </div>
                      )}
                    </div>
                  </div>
                </Card>
              )
            })}
          </div>
        )}
      </div>

      {/* 告警历史 */}
      <div>
        <h3 className="font-medium mb-2">告警历史 ({filteredHistory.length})</h3>

        {filteredHistory.length === 0 ? (
          <div className="text-center py-4 text-muted-foreground text-sm">暂无告警历史</div>
        ) : (
          <div className="space-y-2 max-h-64 overflow-auto">
            {filteredHistory.slice(0, 10).map(alert => (
              <Card key={alert.id} className="p-2 opacity-60">
                <div className="flex items-center gap-2">
                  <CheckCircle className="w-4 h-4 text-green-500" />
                  <span className="text-sm flex-1">{alert.message}</span>
                  <span className="text-xs text-muted-foreground">
                    {alert.resolvedAt?.toLocaleString()}
                  </span>
                </div>
              </Card>
            ))}
          </div>
        )}
      </div>
    </Card>
  )
}
