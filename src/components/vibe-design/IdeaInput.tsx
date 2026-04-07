import { useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { Sparkles, ArrowRight } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '@/components/ui/card'
import { Textarea } from '@/components/ui/textarea'
import { Input } from '@/components/ui/input'
import {
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogFooter,
  AlertDialog,
  AlertDialogContent,
} from '@/components/ui/dialog'
import { useProjectStore } from '@/stores'
import { useAIConfigStore } from '@/stores/aiConfigStore'

export function IdeaInput() {
  const navigate = useNavigate()
  const { createProject } = useProjectStore()
  const aiConfigStore = useAIConfigStore()

  const [projectName, setProjectName] = useState('')
  const [idea, setIdea] = useState('')

  // Dialog states
  const [showNoApiKeyDialog, setShowNoApiKeyDialog] = useState(false)
  const [errorDialog, setErrorDialog] = useState<{
    show: boolean
    message: string
    detail: string
  }>({ show: false, message: '', detail: '' })

  const handleSubmit = async () => {
    if (!projectName.trim() || !idea.trim()) return

    // 检查 AI 配置
    const activeConfig = aiConfigStore.getActiveConfig()

    if (!activeConfig?.apiKey) {
      // 没有 API Key，显示提示对话框
      setShowNoApiKeyDialog(true)
      return
    }

    try {
      // 1. 创建项目（现在是异步的）
      const project = await createProject(projectName, idea.slice(0, 100), idea)

      // 2. 直接跳转到 PRD 页面，由 PRDDisplay 组件处理流式生成
      // 通过 URL 参数传递想法和 AI 配置信息
      const params = new URLSearchParams({
        mode: 'streaming',
        idea: encodeURIComponent(idea),
        provider: activeConfig.provider,
        model: activeConfig.model,
        apiKey: activeConfig.apiKey,
      })

      navigate(`/prd/${project.id}?${params.toString()}`)
    } catch (err) {
      console.error('创建项目失败:', err)
      const errorMessage = err instanceof Error ? err.message : '创建项目失败'

      setErrorDialog({
        show: true,
        message: errorMessage,
        detail: '请重试或检查网络连接',
      })
    }
  }

  const handleGoToSettings = () => {
    setShowNoApiKeyDialog(false)
    navigate('/ai-config')
  }

  const handleErrorDialogClose = () => {
    setErrorDialog(prev => ({ ...prev, show: false }))
    navigate('/ai-config')
  }

  const exampleIdeas = [
    '我想做一个帮助独立开发者管理项目进度的工具，类似 Trello 但是更简单，专门为单人项目设计',
    '我想创建一个在线简历生成器，让用户可以通过拖拽组件快速制作专业简历',
    '我想开发一个浏览器插件，帮助用户屏蔽社交媒体上的负面内容',
  ]

  return (
    <div className="max-w-3xl mx-auto space-y-6">
      {/* No API Key Dialog */}
      <AlertDialog open={showNoApiKeyDialog} onOpenChange={setShowNoApiKeyDialog} type="warning">
        <AlertDialogContent aria-label="未配置 API Key">
          <DialogHeader>
            <DialogTitle>⚠️ 未检测到 AI API Key 配置</DialogTitle>
            <DialogDescription className="text-sm mt-2">
              请先前往 AI 配置页面设置 API Key，然后才能开始分析。
              <br />
              <br />
              <strong>支持的服务商：</strong>
              <br />
              • OpenAI
              <br />
              • Anthropic Claude
              <br />
              • Kimi
              <br />
              • GLM
              <br />• MiniMax
            </DialogDescription>
          </DialogHeader>
          <DialogFooter className="gap-2 sm:gap-0">
            <Button variant="outline" onClick={() => setShowNoApiKeyDialog(false)}>
              取消
            </Button>
            <Button onClick={handleGoToSettings}>前往配置</Button>
          </DialogFooter>
        </AlertDialogContent>
      </AlertDialog>

      {/* Error Dialog */}
      <AlertDialog
        open={errorDialog.show}
        onOpenChange={open => setErrorDialog(prev => ({ ...prev, show: open }))}
        type="error"
      >
        <AlertDialogContent aria-label="创建项目失败">
          <DialogHeader>
            <DialogTitle>❌ 创建项目失败</DialogTitle>
            <DialogDescription className="text-sm mt-2 space-y-2">
              <p className="font-medium text-red-600">{errorDialog.message}</p>
              {errorDialog.detail && <p className="text-muted-foreground">{errorDialog.detail}</p>}
            </DialogDescription>
          </DialogHeader>
          <DialogFooter>
            <Button onClick={handleErrorDialogClose}>关闭</Button>
          </DialogFooter>
        </AlertDialogContent>
      </AlertDialog>

      {/* Main Content */}
      <div className="text-center space-y-2">
        <h1 className="text-3xl font-bold">💡 输入你的产品想法</h1>
        <p className="text-muted-foreground">用自然语言描述你的想法，AI 将帮你完善产品构思</p>
      </div>

      <Card>
        <CardHeader>
          <CardTitle>项目信息</CardTitle>
          <CardDescription>给你的项目起个名字</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div>
            <label className="text-sm font-medium mb-2 block">项目名称</label>
            <Input
              placeholder="例如：SoloFlow - 一人项目管理系统"
              value={projectName}
              onChange={e => setProjectName(e.target.value)}
            />
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>产品想法</CardTitle>
          <CardDescription>详细描述你的产品想法</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <Textarea
            placeholder="我想做一个..."
            value={idea}
            onChange={e => setIdea(e.target.value)}
            rows={6}
            className="resize-none"
          />

          <div className="space-y-2">
            <p className="text-sm text-muted-foreground">或者选择一个示例：</p>
            <div className="space-y-2">
              {exampleIdeas.map((example, index) => (
                <button
                  key={index}
                  onClick={() => setIdea(example)}
                  className="w-full text-left p-3 text-sm border rounded-lg hover:bg-accent transition-colors"
                >
                  {example}
                </button>
              ))}
            </div>
          </div>
        </CardContent>
      </Card>

      <Button
        onClick={handleSubmit}
        disabled={!projectName.trim() || !idea.trim()}
        className="w-full"
        size="lg"
      >
        <Sparkles className="w-4 h-4 mr-2" />
        开始分析（AI 驱动）
        <ArrowRight className="w-4 h-4 ml-2" />
      </Button>
    </div>
  )
}
