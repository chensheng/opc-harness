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
import { useProjectStore, useAppStore } from '@/stores'
import { useAIConfigStore } from '@/stores/aiConfigStore'
import { invoke } from '@tauri-apps/api/core'

export function IdeaInput() {
  const navigate = useNavigate()
  const { createProject, setProjectPRD, updateProjectStatus, updateProjectProgress } =
    useProjectStore()
  const { setLoading } = useAppStore()
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

    setLoading(true, 'AI 正在分析你的想法并生成 PRD...')

    try {
      // 1. 创建项目（现在是异步的）
      const project = await createProject(projectName, idea.slice(0, 100), idea)

      // 2. 调用真实 AI 生成 PRD
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      const prdResponse = await invoke<Record<string, any>>('generate_prd', {
        request: {
          idea: idea,
          provider: activeConfig.provider,
          model: activeConfig.model,
          api_key: activeConfig.apiKey,
        },
      })

      // 3. 转换后端返回的 PRD 格式为前端格式
      const generatedPRD = {
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        title: (prdResponse as any).title || projectName,
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        overview: (prdResponse as any).overview || '',
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        targetUsers: (prdResponse as any).target_users || [],
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        coreFeatures: (prdResponse as any).core_features || [],
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        techStack: (prdResponse as any).tech_stack || [],
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        estimatedEffort: (prdResponse as any).estimated_effort || '待评估',
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        businessModel: (prdResponse as any).business_model || undefined,
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        pricing: (prdResponse as any).pricing || undefined,
      }

      // 4. 保存 PRD 到项目
      setProjectPRD(project.id, generatedPRD)
      updateProjectStatus(project.id, 'design')
      updateProjectProgress(project.id, 25)

      setLoading(false)
      // 5. 跳转到 PRD 编辑页面
      navigate(`/prd/${project.id}`)
    } catch (err) {
      console.error('AI 生成 PRD 失败:', err)
      const errorMessage = err instanceof Error ? err.message : 'AI 调用失败'

      let errorDetail = ''
      if (errorMessage.includes('API key') || errorMessage.includes('invalid')) {
        errorDetail = '可能原因：API Key 无效或已过期'
      } else if (errorMessage.includes('network') || errorMessage.includes('fetch')) {
        errorDetail = '可能原因：网络连接问题，请检查网络后重试'
      } else if (errorMessage.includes('quota') || errorMessage.includes('balance')) {
        errorDetail = '可能原因：API 额度不足或余额不足'
      }

      setErrorDialog({
        show: true,
        message: errorMessage,
        detail: errorDetail,
      })
    } finally {
      setLoading(false)
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
        <AlertDialogContent aria-label="AI 调用失败">
          <DialogHeader>
            <DialogTitle>❌ AI 生成 PRD 失败</DialogTitle>
            <DialogDescription className="text-sm mt-2 space-y-2">
              <p className="font-medium text-red-600">{errorDialog.message}</p>
              {errorDialog.detail && <p className="text-muted-foreground">{errorDialog.detail}</p>}
              <div className="mt-3 p-3 bg-red-50 rounded-md">
                <p className="text-sm font-semibold mb-2">请检查：</p>
                <ol className="text-sm list-decimal list-inside space-y-1">
                  <li>API Key 是否正确配置</li>
                  <li>网络连接是否正常</li>
                  <li>API 账户余额是否充足</li>
                </ol>
              </div>
            </DialogDescription>
          </DialogHeader>
          <DialogFooter>
            <Button onClick={handleErrorDialogClose}>前往 AI 配置重新配置</Button>
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
