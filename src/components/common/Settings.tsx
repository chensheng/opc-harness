import { Moon, Sun, Monitor, Globe, Save, Cpu } from 'lucide-react'
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '@/components/ui/card'
import { useAppStore } from '@/stores'
import { ToolDetector } from './ToolDetector'
import { GitDetector } from './GitDetector'

export function Settings() {
  const { settings, setSettings } = useAppStore()
  // 使用当前项目路径 (实际项目中应该从 store 获取)
  const currentProjectPath = '/tmp/opc-harness-project'

  return (
    <div className="max-w-2xl mx-auto space-y-6">
      <div>
        <h1 className="text-2xl font-bold">⚙️ 设置</h1>
        <p className="text-muted-foreground">自定义你的使用体验</p>
      </div>

      {/* 工具检测卡片 */}
      <ToolDetector />

      {/* Git 仓库状态检测 */}
      <GitDetector projectPath={currentProjectPath} />

      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Monitor className="w-5 h-5" />
            外观
          </CardTitle>
          <CardDescription>选择你喜欢的主题</CardDescription>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-3 gap-4">
            <button
              onClick={() => setSettings({ theme: 'light' })}
              className={`flex flex-col items-center gap-2 p-4 border rounded-lg transition-colors ${
                settings.theme === 'light' ? 'border-primary bg-primary/5' : 'hover:bg-accent'
              }`}
            >
              <Sun className="w-6 h-6" />
              <span className="text-sm">浅色</span>
            </button>
            <button
              onClick={() => setSettings({ theme: 'dark' })}
              className={`flex flex-col items-center gap-2 p-4 border rounded-lg transition-colors ${
                settings.theme === 'dark' ? 'border-primary bg-primary/5' : 'hover:bg-accent'
              }`}
            >
              <Moon className="w-6 h-6" />
              <span className="text-sm">深色</span>
            </button>
            <button
              onClick={() => setSettings({ theme: 'system' })}
              className={`flex flex-col items-center gap-2 p-4 border rounded-lg transition-colors ${
                settings.theme === 'system' ? 'border-primary bg-primary/5' : 'hover:bg-accent'
              }`}
            >
              <Monitor className="w-6 h-6" />
              <span className="text-sm">跟随系统</span>
            </button>
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Globe className="w-5 h-5" />
            语言
          </CardTitle>
          <CardDescription>选择界面语言</CardDescription>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-2 gap-4">
            <button
              onClick={() => setSettings({ language: 'zh' })}
              className={`flex items-center justify-center gap-2 p-4 border rounded-lg transition-colors ${
                settings.language === 'zh' ? 'border-primary bg-primary/5' : 'hover:bg-accent'
              }`}
            >
              <span className="text-lg">🇨🇳</span>
              <span>简体中文</span>
            </button>
            <button
              onClick={() => setSettings({ language: 'en' })}
              className={`flex items-center justify-center gap-2 p-4 border rounded-lg transition-colors ${
                settings.language === 'en' ? 'border-primary bg-primary/5' : 'hover:bg-accent'
              }`}
            >
              <span className="text-lg">🇺🇸</span>
              <span>English</span>
            </button>
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Save className="w-5 h-5" />
            自动保存
          </CardTitle>
        </CardHeader>
        <CardContent>
          <label className="flex items-center gap-3 cursor-pointer">
            <input
              type="checkbox"
              checked={settings.autoSave}
              onChange={e => setSettings({ autoSave: e.target.checked })}
              className="w-4 h-4 rounded border-gray-300 text-primary focus:ring-primary"
            />
            <span>启用自动保存</span>
          </label>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Cpu className="w-5 h-5" />
            Native Coding Agent
          </CardTitle>
          <CardDescription>选择编码智能体模式（需要重启应用生效）</CardDescription>
        </CardHeader>
        <CardContent>
          <div className="space-y-4">
            <label className="flex items-start gap-3 cursor-pointer p-3 border rounded-lg hover:bg-accent transition-colors">
              <input
                type="radio"
                name="agentMode"
                checked={!settings.useNativeAgent}
                onChange={() => setSettings({ useNativeAgent: false })}
                className="mt-1 w-4 h-4 text-primary focus:ring-primary"
              />
              <div className="flex-1">
                <div className="font-medium">CLI Agent（传统模式）</div>
                <div className="text-sm text-muted-foreground mt-1">
                  基于 Kimi CLI 的编码智能体，稳定性高，兼容性好
                </div>
              </div>
            </label>
            <label className="flex items-start gap-3 cursor-pointer p-3 border rounded-lg hover:bg-accent transition-colors">
              <input
                type="radio"
                name="agentMode"
                checked={settings.useNativeAgent}
                onChange={() => setSettings({ useNativeAgent: true })}
                className="mt-1 w-4 h-4 text-primary focus:ring-primary"
              />
              <div className="flex-1">
                <div className="font-medium">Native Agent（推荐）</div>
                <div className="text-sm text-muted-foreground mt-1">
                  纯 Rust 实现的编码智能体，性能提升 40-60%，支持详细日志和 Token 统计
                </div>
              </div>
            </label>
            <div className="text-xs text-muted-foreground bg-muted p-3 rounded">
              💡 提示：切换模式后需要重启应用才能生效。当前配置已保存到本地存储。
            </div>
          </div>
        </CardContent>
      </Card>

      <div className="text-center text-sm text-muted-foreground">
        <p>OPC-HARNESS v0.1.0</p>
        <p className="mt-1">AI驱动的一人公司操作系统</p>
      </div>
    </div>
  )
}
