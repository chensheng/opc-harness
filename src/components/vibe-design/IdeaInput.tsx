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
import { ProjectListFloatingButton } from './ProjectListFloatingButton'

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

    // CodeFree 不需要 API Key，其他提供商需要
    const needsApiKey = activeConfig?.provider !== 'codefree'

    if (!activeConfig || (needsApiKey && !activeConfig.apiKey)) {
      // 没有配置或需要 API Key 但没有设置，显示提示对话框
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
    <div className="max-w-3xl mx-auto h-[calc(100vh-8rem)] flex flex-col">
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

      {/* Main Content - Compact Layout */}
      <div className="flex-1 flex flex-col gap-4 overflow-hidden">
        {/* Header - Compact */}
        <div className="text-center shrink-0">
          <h1 className="text-2xl font-bold mb-1">💡 输入你的产品想法</h1>
          <p className="text-sm text-muted-foreground">用自然语言描述你的想法，AI 将帮你完善产品构思</p>
        </div>

        {/* Combined Form Card */}
        <Card className="flex-1 flex flex-col overflow-hidden">
          <CardHeader className="pb-2 pt-4 px-4">
            <CardTitle className="text-lg">项目信息</CardTitle>
          </CardHeader>
          <CardContent className="flex-1 flex flex-col gap-3 px-4 pb-4 overflow-y-auto">
            {/* Project Name */}
            <div>
              <label className="text-xs font-medium mb-1 block text-muted-foreground">项目名称</label>
              <Input
                placeholder="例如：SoloFlow - 一人项目管理系统"
                value={projectName}
                onChange={e => setProjectName(e.target.value)}
                className="h-9"
              />
            </div>

            {/* Product Idea */}
            <div className="flex-1 flex flex-col">
              <label className="text-xs font-medium mb-1 block text-muted-foreground">产品想法</label>
              <Textarea
                placeholder="我想做一个..."
                value={idea}
                onChange={e => setIdea(e.target.value)}
                rows={4}
                className="resize-none flex-1 min-h-[100px]"
              />
            </div>

            {/* Example Ideas - Compact */}
            <div className="shrink-0">
              <p className="text-xs text-muted-foreground mb-2">或者选择一个示例：</p>
              <div className="grid grid-cols-1 gap-1.5 max-h-[120px] overflow-y-auto">
                {exampleIdeas.map((example, index) => (
                  <button
                    key={index}
                    onClick={() => setIdea(example)}
                    className="w-full text-left p-2 text-xs border rounded hover:bg-accent transition-colors line-clamp-2"
                  >
                    {example}
                  </button>
                ))}
              </div>
            </div>
          </CardContent>
        </Card>

        {/* Submit Button - Fixed at bottom */}
        <Button
          onClick={handleSubmit}
          disabled={!projectName.trim() || !idea.trim()}
          className="w-full shrink-0"
          size="lg"
        >
          <Sparkles className="w-4 h-4 mr-2" />
          开始分析（AI 驱动）
          <ArrowRight className="w-4 h-4 ml-2" />
        </Button>
      </div>

      {/* 项目列表悬浮按钮 */}
      <ProjectListFloatingButton />
    </div>
  )
}
