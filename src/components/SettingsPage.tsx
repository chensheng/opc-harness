import { useState } from 'react';
import { Settings, Key, Database, Bell, Palette, Globe, Shield, CheckCircle2 } from 'lucide-react';
import { AIProviderConfigPanel } from './AIProviderConfigPanel';
import { Button } from './ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from './ui/card';
import { Tabs, TabsContent, TabsList, TabsTrigger } from './ui/tabs';
import { Switch } from './ui/switch';
import { Label } from './ui/label';

/**
 * 设置页面组件 (VD-003)
 *
 * 提供完整的系统设置界面：
 * - AI 厂商配置管理
 * - 通用设置（主题、语言等）
 * - 通知设置
 * - 数据存储管理
 */
export function SettingsPage() {
  const [activeTab, setActiveTab] = useState<'ai' | 'general' | 'notification' | 'storage'>('ai');

  return (
    <div className="space-y-6">
      {/* 页面标题 */}
      <div className="flex items-center gap-3">
        <div className="p-2 bg-blue-100 rounded-lg">
          <Settings className="h-6 w-6 text-blue-600" />
        </div>
        <div>
          <h1 className="text-2xl font-bold">系统设置</h1>
          <p className="text-sm text-muted-foreground">配置和管理您的 OPC-HARNESS 系统</p>
        </div>
      </div>

      {/* 设置导航 */}
      <Tabs
        value={activeTab}
        onValueChange={value => setActiveTab(value as typeof activeTab)}
        className="w-full"
      >
        <TabsList className="grid w-full grid-cols-4">
          <TabsTrigger value="ai" className="flex items-center gap-2">
            <Key className="h-4 w-4" />
            <span>AI 配置</span>
          </TabsTrigger>
          <TabsTrigger value="general" className="flex items-center gap-2">
            <Palette className="h-4 w-4" />
            <span>通用</span>
          </TabsTrigger>
          <TabsTrigger value="notification" className="flex items-center gap-2">
            <Bell className="h-4 w-4" />
            <span>通知</span>
          </TabsTrigger>
          <TabsTrigger value="storage" className="flex items-center gap-2">
            <Database className="h-4 w-4" />
            <span>存储</span>
          </TabsTrigger>
        </TabsList>

        {/* AI 配置标签页 */}
        <TabsContent value="ai" className="space-y-4">
          <Card>
            <CardHeader>
              <div className="flex items-center justify-between">
                <div className="space-y-1">
                  <CardTitle className="flex items-center gap-2">
                    <Key className="h-5 w-5" />
                    AI 厂商配置
                  </CardTitle>
                  <CardDescription>
                    管理 AI 厂商的 API 密钥和连接配置，支持多个主流 AI 服务提供商
                  </CardDescription>
                </div>
              </div>
            </CardHeader>
            <CardContent>
              <AIProviderConfigPanel />
            </CardContent>
          </Card>

          {/* AI 使用提示 */}
          <Card>
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Shield className="h-5 w-5" />
                安全提示
              </CardTitle>
            </CardHeader>
            <CardContent className="space-y-3 text-sm text-muted-foreground">
              <div className="flex items-start gap-2">
                <CheckCircle2 className="h-4 w-4 text-green-600 mt-0.5" />
                <p>API 密钥将安全存储在操作系统的密钥管理中，不会保存到数据库或发送到服务器</p>
              </div>
              <div className="flex items-start gap-2">
                <CheckCircle2 className="h-4 w-4 text-green-600 mt-0.5" />
                <p>建议为不同的 AI 厂商申请专用的 API 密钥，以便管理和控制权限</p>
              </div>
              <div className="flex items-start gap-2">
                <CheckCircle2 className="h-4 w-4 text-green-600 mt-0.5" />
                <p>定期检查 API 密钥的使用情况，避免密钥泄露或超额使用</p>
              </div>
            </CardContent>
          </Card>
        </TabsContent>

        {/* 通用设置标签页 */}
        <TabsContent value="general" className="space-y-4">
          <Card>
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Palette className="h-5 w-5" />
                外观设置
              </CardTitle>
              <CardDescription>自定义应用的显示风格和主题</CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="flex items-center justify-between">
                <div className="space-y-0.5">
                  <Label>深色模式</Label>
                  <p className="text-sm text-muted-foreground">切换应用的明暗主题</p>
                </div>
                <Switch />
              </div>
              <div className="flex items-center justify-between">
                <div className="space-y-0.5">
                  <Label>紧凑模式</Label>
                  <p className="text-sm text-muted-foreground">减小元素间距，显示更多内容</p>
                </div>
                <Switch />
              </div>
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Globe className="h-5 w-5" />
                语言和区域
              </CardTitle>
              <CardDescription>设置应用的语言和地区偏好</CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="space-y-2">
                <Label htmlFor="language">界面语言</Label>
                <select id="language" className="w-full p-2 border rounded-md" defaultValue="zh-CN">
                  <option value="zh-CN">简体中文</option>
                  <option value="zh-TW">繁體中文</option>
                  <option value="en-US">English</option>
                </select>
              </div>
              <div className="space-y-2">
                <Label htmlFor="timezone">时区</Label>
                <select
                  id="timezone"
                  className="w-full p-2 border rounded-md"
                  defaultValue="Asia/Shanghai"
                >
                  <option value="Asia/Shanghai">中国标准时间 (UTC+8)</option>
                  <option value="Asia/Tokyo">日本标准时间 (UTC+9)</option>
                  <option value="America/New_York">美国东部时间 (UTC-5)</option>
                </select>
              </div>
            </CardContent>
          </Card>
        </TabsContent>

        {/* 通知设置标签页 */}
        <TabsContent value="notification" className="space-y-4">
          <Card>
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Bell className="h-5 w-5" />
                通知偏好
              </CardTitle>
              <CardDescription>管理应用的通知和提醒设置</CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="flex items-center justify-between">
                <div className="space-y-0.5">
                  <Label>桌面通知</Label>
                  <p className="text-sm text-muted-foreground">在系统通知中心显示提醒</p>
                </div>
                <Switch defaultChecked />
              </div>
              <div className="flex items-center justify-between">
                <div className="space-y-0.5">
                  <Label>声音提醒</Label>
                  <p className="text-sm text-muted-foreground">播放提示音</p>
                </div>
                <Switch defaultChecked />
              </div>
              <div className="flex items-center justify-between">
                <div className="space-y-0.5">
                  <Label>任务完成通知</Label>
                  <p className="text-sm text-muted-foreground">当 AI 完成任务时通知</p>
                </div>
                <Switch defaultChecked />
              </div>
            </CardContent>
          </Card>
        </TabsContent>

        {/* 存储设置标签页 */}
        <TabsContent value="storage" className="space-y-4">
          <Card>
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Database className="h-5 w-5" />
                数据存储
              </CardTitle>
              <CardDescription>管理本地数据和缓存</CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="p-4 bg-slate-50 rounded-lg">
                <div className="flex justify-between items-center mb-2">
                  <span className="text-sm font-medium">已用存储空间</span>
                  <span className="text-sm text-muted-foreground">2.5 MB / 100 MB</span>
                </div>
                <div className="w-full bg-slate-200 rounded-full h-2">
                  <div className="bg-blue-600 h-2 rounded-full" style={{ width: '2.5%' }}></div>
                </div>
              </div>

              <div className="space-y-2">
                <Button variant="outline" className="w-full justify-start">
                  <Database className="h-4 w-4 mr-2" />
                  清理缓存
                  <span className="ml-auto text-xs text-muted-foreground">约 1.2 MB</span>
                </Button>
                <Button variant="outline" className="w-full justify-start">
                  <Database className="h-4 w-4 mr-2" />
                  导出数据
                  <span className="ml-auto text-xs text-muted-foreground">JSON 格式</span>
                </Button>
                <Button variant="outline" className="w-full justify-start">
                  <Database className="h-4 w-4 mr-2" />
                  导入数据
                  <span className="ml-auto text-xs text-muted-foreground">从备份恢复</span>
                </Button>
              </div>

              <div className="p-4 bg-red-50 border border-red-200 rounded-lg">
                <h4 className="text-sm font-semibold text-red-800 mb-2">危险区域</h4>
                <p className="text-xs text-red-600 mb-3">
                  此操作将永久删除所有本地数据，包括项目配置、AI 对话历史等。
                </p>
                <Button variant="destructive" size="sm">
                  重置所有数据
                </Button>
              </div>
            </CardContent>
          </Card>
        </TabsContent>
      </Tabs>
    </div>
  );
}
