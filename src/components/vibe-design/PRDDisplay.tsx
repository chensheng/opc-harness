import { useEffect, useState } from 'react'
import { useParams, useNavigate } from 'react-router-dom'
import { ArrowRight, Users, Target, Zap, Clock, Download, Edit, Sparkles, Save, X, Eye, Pencil, PanelLeftClose, Columns } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { Textarea } from '@/components/ui/textarea'
import { Input } from '@/components/ui/input'
import { useProjectStore, useAppStore } from '@/stores'
import type { PRD } from '@/types'
import { usePRDStream } from '@/hooks/usePRDStream'
import { useAIConfigStore } from '@/stores/aiConfigStore'
import ReactMarkdown from 'react-markdown'
import remarkGfm from 'remark-gfm'
import { save } from '@tauri-apps/plugin-dialog'
import { writeTextFile } from '@tauri-apps/plugin-fs'
import { Progress } from '@/components/ui/progress'
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogFooter,
} from '@/components/ui/dialog'

// Markdown 表格自定义组件，确保边框显示并增加上下间距
const TableComponent = ({ node, ...props }: any) => (
  <div className="overflow-x-auto my-6 first:mt-4 last:mb-4">
    <table className="w-full border-collapse border border-border" {...props} />
  </div>
)

const ThComponent = ({ node, ...props }: any) => (
  <th className="border border-border px-4 py-3 bg-muted/80 text-left font-semibold text-sm" {...props} />
)

const TdComponent = ({ node, ...props }: any) => (
  <td className="border border-border px-4 py-3 text-left text-sm" {...props} />
)

const TrComponent = ({ node, ...props }: any) => (
  <tr className="even:bg-muted/30 hover:bg-muted/50 transition-colors" {...props} />
)

// 段落组件，确保与表格有适当间距
const ParagraphComponent = ({ node, ...props }: any) => (
  <p className="text-base leading-relaxed mb-4 last:mb-0 text-foreground/90" {...props} />
)

// 完整文档视图的自定义组件 - 更美观的排版
const FullDocComponents = {
  // 标题层级
  h1: ({ node, ...props }: any) => (
    <h1 className="text-3xl font-bold mb-6 mt-8 pb-2 border-b border-border text-primary" {...props} />
  ),
  h2: ({ node, ...props }: any) => (
    <h2 className="text-2xl font-semibold mb-4 mt-7 pb-1.5 border-b border-border/50 text-foreground" {...props} />
  ),
  h3: ({ node, ...props }: any) => (
    <h3 className="text-xl font-medium mb-3 mt-5 text-foreground/90" {...props} />
  ),
  h4: ({ node, ...props }: any) => (
    <h4 className="text-lg font-medium mb-2 mt-4 text-foreground/80" {...props} />
  ),
  
  // 段落和文本
  p: ParagraphComponent,
  
  // 列表
  ul: ({ node, ...props }: any) => (
    <ul className="list-disc list-outside pl-6 mb-4 space-y-2" {...props} />
  ),
  ol: ({ node, ...props }: any) => (
    <ol className="list-decimal list-outside pl-6 mb-4 space-y-2" {...props} />
  ),
  li: ({ node, ...props }: any) => (
    <li className="text-base leading-relaxed text-foreground/90" {...props} />
  ),
  
  // 强调
  strong: ({ node, ...props }: any) => (
    <strong className="font-bold text-foreground" {...props} />
  ),
  em: ({ node, ...props }: any) => (
    <em className="italic text-foreground/80" {...props} />
  ),
  
  // 代码
  code: ({ node, inline, className, children, ...props }: any) => {
    return inline ? (
      <code className="bg-muted/80 px-2 py-0.5 rounded-md text-sm font-mono text-primary border border-border/30" {...props}>
        {children}
      </code>
    ) : (
      <code className="block bg-muted p-4 rounded-lg my-4 overflow-x-auto border border-border/50" {...props}>
        {children}
      </code>
    )
  },
  pre: ({ node, ...props }: any) => (
    <pre className="bg-gradient-to-br from-muted to-muted/80 p-0 rounded-lg my-4 overflow-hidden border border-border/50 shadow-sm" {...props} />
  ),
  
  // 引用块
  blockquote: ({ node, ...props }: any) => (
    <blockquote className="border-l-4 border-primary pl-4 py-2 my-4 bg-muted/20 rounded-r-lg italic text-foreground/80" {...props} />
  ),
  
  // 链接
  a: ({ node, ...props }: any) => (
    <a className="text-primary hover:text-primary/80 underline decoration-primary/50 hover:decoration-primary transition-all font-medium" {...props} />
  ),
  
  // 分隔线
  hr: ({ node, ...props }: any) => (
    <hr className="border-border my-8" {...props} />
  ),
  
  // 表格
  table: TableComponent,
  th: ThComponent,
  td: TdComponent,
  tr: TrComponent,
}

// Simulated AI-generated PRD (fallback)
function generateMockPRD(idea: string): PRD {
  return {
    title: idea.slice(0, 30) + (idea.length > 30 ? '...' : ''),
    overview: `这是一个基于用户想法「${idea.slice(0, 50)}...」的产品。该产品旨在解决目标用户的核心痛点，提供简洁高效的解决方案。`,
    targetUsers: ['独立开发者', '自由职业者', '技术型创业者', '小型团队负责人'],
    coreFeatures: [
      '直观的用户界面，降低学习成本',
      '核心功能模块化，按需使用',
      '数据同步和备份机制',
      '多平台支持（Web、移动端）',
      'API 接口开放，支持第三方集成',
    ],
    techStack: ['React', 'Node.js', 'PostgreSQL', 'Redis', 'Docker'],
    estimatedEffort: '2-4 周',
    businessModel: 'Freemium 模式，基础功能免费，高级功能订阅制',
    pricing: '免费版：基础功能；Pro 版：$9/月；Team 版：$29/月',
  }
}

export function PRDDisplay() {
  const { projectId } = useParams<{ projectId: string }>()
  const navigate = useNavigate()
  const { getProjectById, setProjectPRD, updateProjectStatus, updateProjectProgress } =
    useProjectStore()
  const { setLoading } = useAppStore()
  const aiConfigStore = useAIConfigStore()

  const [prd, setPrd] = useState<PRD | null>(null)
  const [isEditing, setIsEditing] = useState(false)
  const [editedMarkdown, setEditedMarkdown] = useState<string>('')
  const [previewMode, setPreviewMode] = useState<'edit' | 'preview' | 'split'>('split')
  
  // 导出状态
  const [isExporting, setIsExporting] = useState(false)
  const [exportProgress, setExportProgress] = useState(0)
  const [showExportDialog, setShowExportDialog] = useState(false)
  const [exportStatus, setExportStatus] = useState<'success' | 'error'>('success')
  const [exportMessage, setExportMessage] = useState('')

  // 使用 PRD 流式生成 Hook
  const {
    prd: streamingPRD,
    markdownContent,
    isStreaming,
    isComplete,
    error,
    sessionId: _sessionId,
    startStream,
    stopStream,
    reset,
  } = usePRDStream()

  const project = projectId ? getProjectById(projectId) : undefined

  useEffect(() => {
    if (project) {
      if (project.prd) {
        setPrd(project.prd)
      } else {
        // Generate PRD if not exists
        generatePRD()
      }
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [project])

  // 当流式 PRD 更新时，同步到本地状态
  useEffect(() => {
    if (streamingPRD && isStreaming) {
      setPrd(streamingPRD)
    }
  }, [streamingPRD, isStreaming])

  const generatePRD = async () => {
    if (!project) return

    const activeConfig = aiConfigStore.getActiveConfig()

    if (activeConfig?.apiKey) {
      setLoading(true, 'AI 正在生成产品需求文档...')

      // 使用流式生成
      reset()
      await startStream({
        idea: project.idea || project.description,
        provider: activeConfig.provider,
        model: activeConfig.model,
        apiKey: activeConfig.apiKey,
      })
    } else {
      // 降级到模拟生成
      setLoading(true, '正在生成产品需求文档...')
      try {
        await new Promise(resolve => setTimeout(resolve, 2000))
        const generatedPRD = generateMockPRD(project.idea || project.description)
        setPrd(generatedPRD)

        if (projectId) {
          setProjectPRD(projectId, generatedPRD)
          updateProjectStatus(projectId, 'design')
          updateProjectProgress(projectId, 25)
        }
      } finally {
        setLoading(false)
      }
    }
  }

  // 监听流式完成，保存 PRD
  useEffect(() => {
    if (isComplete && streamingPRD && projectId) {
      setProjectPRD(projectId, streamingPRD)
      updateProjectStatus(projectId, 'design')
      updateProjectProgress(projectId, 25)
      setLoading(false)
    }
  }, [
    isComplete,
    streamingPRD,
    projectId,
    setProjectPRD,
    updateProjectStatus,
    updateProjectProgress,
    setLoading,
  ])

  const handleExport = async () => {
    if (!prd && !editedMarkdown) {
      alert('暂无可导出的内容，请先生成产品需求文档')
      return
    }

    setIsExporting(true)
    setExportProgress(0)
    setShowExportDialog(true)
    setExportStatus('success')

    try {
      // 步骤 1: 准备内容 (进度 10%)
      setExportProgress(10)
      await new Promise(resolve => setTimeout(resolve, 300))
      
      const content =
        editedMarkdown ||
        `# ${prd?.title}\n\n## 产品概述\n\n${prd?.overview}\n\n## 目标用户\n\n${prd?.targetUsers.map(u => `- ${u}`).join('\n')}\n\n## 核心功能\n\n${prd?.coreFeatures.map(f => `- ${f}`).join('\n')}\n\n## 技术栈\n\n${prd?.techStack.map(t => `- ${t}`).join('\n')}\n\n## 预估工作量\n\n${prd?.estimatedEffort}\n\n## 商业模式\n\n${prd?.businessModel || '待定'}\n\n## 定价策略\n\n${prd?.pricing || '待定'}`

      // 生成默认文件名
      const defaultFilename = `${prd?.title || editedMarkdown ? 'PRD' : '产品需求文档'}-PRD.md`
      
      console.log('[PRD Export] Starting export...')
      console.log('[PRD Export] Default filename:', defaultFilename)
      console.log('[PRD Export] Content length:', content.length)
      
      // 步骤 2: 打开保存对话框 (进度 30%)
      setExportProgress(30)
      await new Promise(resolve => setTimeout(resolve, 400))
      
      const filePath = await save({
        defaultPath: defaultFilename,
        filters: [{
          name: 'Markdown Files',
          extensions: ['md']
        }]
      })
      
      console.log('[PRD Export] Selected path:', filePath)
      
      if (filePath) {
        // 步骤 3: 写入文件 (进度 50% -> 90%)
        setExportProgress(50)
        await new Promise(resolve => setTimeout(resolve, 300))
        
        await writeTextFile(filePath, content)
        
        setExportProgress(90)
        await new Promise(resolve => setTimeout(resolve, 300))
        
        setExportProgress(100)
        setExportMessage(`文件已成功保存到：${filePath}`)
        console.log('[PRD Export] File saved successfully to:', filePath)
        
        // 延迟关闭对话框
        setTimeout(() => {
          setShowExportDialog(false)
          setIsExporting(false)
        }, 1500)
      } else {
        // 用户取消
        setExportMessage('已取消导出')
        setExportProgress(0)
        setTimeout(() => {
          setShowExportDialog(false)
          setIsExporting(false)
        }, 800)
      }
    } catch (error) {
      console.error('[PRD Export] Error during export:', error)
      setExportStatus('error')
      setExportMessage(`导出失败：${error instanceof Error ? error.message : error}`)
      setExportProgress(0)
      // 错误时不自动关闭对话框，让用户手动关闭
      setIsExporting(false)
    }
  }

  const handleStopGeneration = () => {
    stopStream()
    setLoading(false)
  }

  const handleSaveEdit = () => {
    if (!editedMarkdown || !projectId) return
    
    // 从 markdown 内容解析回 PRD 对象
    const updatedPrd = parseMarkdownToPRD(editedMarkdown)
    
    setProjectPRD(projectId, updatedPrd)
    setPrd(updatedPrd)
    setIsEditing(false)
    setEditedMarkdown('')
    setLoading(false)
  }

  const handleCancelEdit = () => {
    setIsEditing(false)
    setEditedMarkdown('')
  }

  const startEditing = () => {
    if (prd) {
      // 将 PRD 对象转换为 markdown 格式
      const markdownContent = convertPRDToMarkdown(prd)
      setEditedMarkdown(markdownContent)
      setIsEditing(true)
      setPreviewMode('split') // 默认分屏模式
    }
  }

  // 将 PRD 对象转换为 Markdown 格式
  const convertPRDToMarkdown = (prdData: PRD): string => {
    let markdown = `# ${prdData.title}\n\n`
    
    markdown += `## 产品概述\n\n${prdData.overview}\n\n`
    
    markdown += `## 目标用户\n\n`
    prdData.targetUsers.forEach(user => {
      markdown += `- ${user}\n`
    })
    markdown += `\n`
    
    markdown += `## 核心功能\n\n`
    prdData.coreFeatures.forEach(feature => {
      markdown += `- ${feature}\n`
    })
    markdown += `\n`
    
    markdown += `## 技术栈\n\n`
    prdData.techStack.forEach(tech => {
      markdown += `- ${tech}\n`
    })
    markdown += `\n`
    
    markdown += `## 预估工作量\n\n${prdData.estimatedEffort}\n\n`
    
    if (prdData.businessModel) {
      markdown += `## 商业模式\n\n${prdData.businessModel}\n\n`
    }
    
    if (prdData.pricing) {
      markdown += `## 定价策略\n\n${prdData.pricing}\n\n`
    }
    
    return markdown.trim()
  }

  // 从 Markdown 内容解析回 PRD 对象
  const parseMarkdownToPRD = (markdown: string): PRD => {
    const lines = markdown.split('\n')
    const prd: any = {
      title: '',
      overview: '',
      targetUsers: [],
      coreFeatures: [],
      techStack: [],
      estimatedEffort: '',
      businessModel: '',
      pricing: '',
    }
    
    let currentSection = ''
    let currentContent: string[] = []
    
    const saveCurrentSection = () => {
      const content = currentContent.join('\n').trim()
      if (!content) return
      
      switch (currentSection.toLowerCase()) {
        case '产品概述':
          prd.overview = content
          break
        case '目标用户':
          prd.targetUsers = content
            .split('\n')
            .filter(line => line.startsWith('-'))
            .map(line => line.replace(/^- /, '').trim())
          break
        case '核心功能':
          prd.coreFeatures = content
            .split('\n')
            .filter(line => line.startsWith('-'))
            .map(line => line.replace(/^- /, '').trim())
          break
        case '技术栈':
          prd.techStack = content
            .split('\n')
            .filter(line => line.startsWith('-'))
            .map(line => line.replace(/^- /, '').trim())
          break
        case '预估工作量':
          prd.estimatedEffort = content
          break
        case '商业模式':
          prd.businessModel = content
          break
        case '定价策略':
          prd.pricing = content
          break
      }
    }
    
    for (const line of lines) {
      const trimmedLine = line.trim()
      
      // 检测标题 (# 开头)
      if (trimmedLine.startsWith('# ') && !currentSection) {
        // 第一个 # 标题是产品标题
        prd.title = trimmedLine.replace(/^# /, '').trim()
        continue
      }
      
      // 检测章节 (## 开头)
      if (trimmedLine.startsWith('## ')) {
        // 保存之前的章节内容
        saveCurrentSection()
        
        // 开始新章节
        currentSection = trimmedLine.replace(/^## /, '').trim()
        currentContent = []
        continue
      }
      
      // 收集内容（跳过空行和标题）
      if (trimmedLine && !trimmedLine.startsWith('#')) {
        currentContent.push(trimmedLine)
      }
    }
    
    // 保存最后一个章节
    if (currentSection) {
      saveCurrentSection()
    }
    
    // 如果标题为空，使用默认值
    if (!prd.title) {
      prd.title = '产品需求文档'
    }
    
    return prd as PRD
  }

  if (!project) {
    return (
      <div className="text-center py-12">
        <p className="text-muted-foreground">项目不存在</p>
        <Button onClick={() => navigate('/')} className="mt-4">
          返回首页
        </Button>
      </div>
    )
  }

  // 流式生成中的 UI
  if (isStreaming) {
    return (
      <div className="max-w-4xl mx-auto space-y-6">
        <div className="flex items-center justify-between">
          <div>
            <h1 className="text-2xl font-bold flex items-center gap-2">
              <Sparkles className="w-6 h-6 text-primary animate-pulse" />
              AI 正在创作 PRD...
            </h1>
            <p className="text-muted-foreground">{project.name}</p>
          </div>
          <Button variant="destructive" onClick={handleStopGeneration}>
            停止生成
          </Button>
        </div>

        {/* 实时内容预览 */}
        <Card>
          <CardHeader>
            <CardTitle>实时生成预览</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="prose prose-sm max-w-none">
              {markdownContent ? (
                <div className="text-sm leading-relaxed">
                  <ReactMarkdown
                    remarkPlugins={[remarkGfm]}
                    components={{
                      table: TableComponent,
                      th: ThComponent,
                      td: TdComponent,
                      tr: TrComponent,
                      h1: ({ node, ...props }: any) => (
                        <h1 className="text-2xl font-bold mb-4 mt-6 pb-2 border-b border-border text-primary" {...props} />
                      ),
                      h2: ({ node, ...props }: any) => (
                        <h2 className="text-xl font-semibold mb-3 mt-5 pb-1.5 border-b border-border/50 text-foreground" {...props} />
                      ),
                      h3: ({ node, ...props }: any) => (
                        <h3 className="text-lg font-medium mb-2 mt-4 text-foreground/90" {...props} />
                      ),
                      p: ({ node, ...props }: any) => (
                        <p className="text-base leading-relaxed mb-3 last:mb-0 text-foreground/90" {...props} />
                      ),
                      ul: ({ node, ...props }: any) => (
                        <ul className="list-disc list-outside pl-6 mb-3 space-y-1.5" {...props} />
                      ),
                      ol: ({ node, ...props }: any) => (
                        <ol className="list-decimal list-outside pl-6 mb-3 space-y-1.5" {...props} />
                      ),
                      li: ({ node, ...props }: any) => (
                        <li className="text-sm leading-relaxed text-foreground/90" {...props} />
                      ),
                      strong: ({ node, ...props }: any) => (
                        <strong className="font-bold text-foreground" {...props} />
                      ),
                      code: ({ node, inline, className, children, ...props }: any) => {
                        return inline ? (
                          <code className="bg-muted/80 px-2 py-0.5 rounded-md text-sm font-mono text-primary border border-border/30" {...props}>
                            {children}
                          </code>
                        ) : (
                          <code className="block bg-muted p-3 rounded-lg my-3 overflow-x-auto border border-border/50" {...props}>
                            {children}
                          </code>
                        )
                      },
                      pre: ({ node, ...props }: any) => (
                        <pre className="bg-gradient-to-br from-muted to-muted/80 p-0 rounded-lg my-3 overflow-hidden border border-border/50 shadow-sm" {...props} />
                      ),
                      blockquote: ({ node, ...props }: any) => (
                        <blockquote className="border-l-4 border-primary pl-4 py-2 my-3 bg-muted/20 rounded-r-lg italic text-foreground/80" {...props} />
                      ),
                      a: ({ node, ...props }: any) => (
                        <a className="text-primary hover:text-primary/80 underline decoration-primary/50 hover:decoration-primary transition-all font-medium" {...props} />
                      ),
                    }}
                  >
                    {markdownContent}
                  </ReactMarkdown>
                  <span className="inline-block w-2 h-4 bg-primary ml-1 animate-pulse" />
                </div>
              ) : (
                <div className="flex items-center justify-center py-12">
                  <div className="text-center">
                    <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary mx-auto" />
                    <p className="mt-4 text-muted-foreground">正在连接 AI...</p>
                  </div>
                </div>
              )}
            </div>
          </CardContent>
        </Card>

        {/* 进度提示 */}
        <Card>
          <CardContent className="py-6">
            <div className="space-y-2">
              <div className="flex items-center justify-between text-sm">
                <span className="text-muted-foreground">生成进度</span>
                <span className="text-muted-foreground">{Math.min((markdownContent.length / 2000) * 100, 100).toFixed(0)}%</span>
              </div>
              <div className="h-2 bg-accent rounded-full overflow-hidden">
                <div
                  className="h-full bg-primary transition-all duration-300"
                  style={{
                    width: `${Math.min((markdownContent.length / 2000) * 100, 100)}%`,
                  }}
                />
              </div>
            </div>
          </CardContent>
        </Card>

        {error && (
          <Card className="border-destructive">
            <CardContent className="py-6">
              <p className="text-destructive">{error}</p>
              <Button onClick={() => generatePRD()} className="mt-4" variant="outline">
                重试
              </Button>
            </CardContent>
          </Card>
        )}
      </div>
    )
  }

  // 没有 PRD 时的初始状态
  if (!prd) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="text-center">
          <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary mx-auto" />
          <p className="mt-4 text-muted-foreground">正在生成 PRD...</p>
        </div>
      </div>
    )
  }

  // PRD 已生成的正常显示
  return (
    <div className="max-w-4xl mx-auto space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold">📋 产品需求文档</h1>
          <p className="text-muted-foreground">{project.name}</p>
        </div>
        <div className="flex gap-2">
          {!isEditing && (
            <>
              <Button variant="outline" onClick={handleExport}>
                <Download className="w-4 h-4 mr-2" />
                导出
              </Button>
              <Button variant="outline" onClick={startEditing}>
                <Edit className="w-4 h-4 mr-2" />
                编辑
              </Button>
            </>
          )}
          {isEditing && (
            <>
              <Button variant="outline" onClick={handleCancelEdit}>
                <X className="w-4 h-4 mr-2" />
                取消
              </Button>
              <Button onClick={handleSaveEdit}>
                <Save className="w-4 h-4 mr-2" />
                保存
              </Button>
            </>
          )}
        </div>
      </div>

      {/* 编辑模式 - Markdown 编辑器 */}
      {isEditing && editedMarkdown ? (
        <div className="space-y-4">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <Pencil className="w-5 h-5 text-primary" />
              <h2 className="text-xl font-bold">编辑产品需求文档</h2>
            </div>
            <div className="flex gap-2">
              <Button
                variant={previewMode === 'edit' ? 'secondary' : 'ghost'}
                size="sm"
                onClick={() => setPreviewMode('edit')}
                title="仅编辑"
                className={previewMode === 'edit' ? 'bg-secondary text-secondary-foreground' : ''}
              >
                <Pencil className="w-4 h-4 mr-1" />
                编辑
              </Button>
              <Button
                variant={previewMode === 'split' ? 'secondary' : 'ghost'}
                size="sm"
                onClick={() => setPreviewMode('split')}
                title="分屏预览"
                className={previewMode === 'split' ? 'bg-secondary text-secondary-foreground' : ''}
              >
                <Columns className="w-4 h-4 mr-1" />
                分屏
              </Button>
              <Button
                variant={previewMode === 'preview' ? 'secondary' : 'ghost'}
                size="sm"
                onClick={() => setPreviewMode('preview')}
                title="仅预览"
                className={previewMode === 'preview' ? 'bg-secondary text-secondary-foreground' : ''}
              >
                <Eye className="w-4 h-4 mr-1" />
                预览
              </Button>
            </div>
          </div>

          <div className={`grid gap-4 ${
            previewMode === 'split' 
              ? 'grid-cols-2' 
              : 'grid-cols-1'
          }`}>
            {/* 编辑器面板 */}
            {(previewMode === 'edit' || previewMode === 'split') && (
              <Card>
                <CardHeader className="py-3">
                  <CardTitle className="text-sm font-medium text-muted-foreground">
                    Markdown 编辑器
                  </CardTitle>
                </CardHeader>
                <CardContent>
                  <Textarea
                    value={editedMarkdown}
                    onChange={(e) => setEditedMarkdown(e.target.value)}
                    className="w-full min-h-[600px] font-mono text-sm leading-relaxed resize-y"
                    placeholder="在此编辑 Markdown 格式的 PRD 内容..."
                  />
                  <p className="mt-2 text-xs text-muted-foreground">
                    💡 提示：支持标准 Markdown 语法，包括表格、列表、代码块等
                  </p>
                </CardContent>
              </Card>
            )}

            {/* 预览面板 */}
            {(previewMode === 'preview' || previewMode === 'split') && (
              <Card>
                <CardHeader className="py-3">
                  <CardTitle className="text-sm font-medium text-muted-foreground">
                    实时预览
                  </CardTitle>
                </CardHeader>
                <CardContent>
                  <div className="prose prose-slate max-w-none bg-muted/30 p-4 rounded-lg border border-border min-h-[600px] overflow-auto">
                    <ReactMarkdown
                      remarkPlugins={[remarkGfm]}
                      components={FullDocComponents as any}
                    >
                      {editedMarkdown}
                    </ReactMarkdown>
                  </div>
                </CardContent>
              </Card>
            )}
          </div>

          <div className="flex justify-end gap-2">
            <Button variant="outline" onClick={handleCancelEdit}>
              <X className="w-4 h-4 mr-2" />
              取消
            </Button>
            <Button onClick={handleSaveEdit}>
              <Save className="w-4 h-4 mr-2" />
              保存更改
            </Button>
          </div>
        </div>
      ) : (
        <Tabs defaultValue="full" className="w-full">
          <TabsList className="grid w-full grid-cols-5">
            <TabsTrigger value="full">完整文档</TabsTrigger>
            <TabsTrigger value="overview">概述</TabsTrigger>
            <TabsTrigger value="features">功能</TabsTrigger>
            <TabsTrigger value="tech">技术</TabsTrigger>
            <TabsTrigger value="business">商业</TabsTrigger>
          </TabsList>

          {/* 完整文档视图 - 使用 Markdown 渲染 */}
          <TabsContent value="full" className="space-y-4">
            <Card>
              <CardHeader>
                <CardTitle className="text-xl font-semibold">📄 产品需求文档</CardTitle>
              </CardHeader>
              <CardContent>
                <div className="prose prose-slate max-w-none">
                  <ReactMarkdown
                    remarkPlugins={[remarkGfm]}
                    components={FullDocComponents as any}
                  >
                    {`# ${prd.title}\n\n## 产品概述\n\n${prd.overview}\n\n## 目标用户\n\n${prd.targetUsers.map(u => `- ${u}`).join('\n')}\n\n## 核心功能\n\n${prd.coreFeatures.map(f => `- ${f}`).join('\n')}\n\n## 技术栈\n\n${prd.techStack.map(t => `- ${t}`).join('\n')}\n\n## 预估工作量\n\n${prd.estimatedEffort}\n\n## 商业模式\n\n${prd.businessModel || '待定'}\n\n## 定价策略\n\n${prd.pricing || '待定'}`}
                  </ReactMarkdown>
                </div>
              </CardContent>
            </Card>
          </TabsContent>

          {/* 概述视图 */}
          <TabsContent value="overview" className="space-y-4">
            <Card>
              <CardHeader>
                <CardTitle className="flex items-center gap-2">
                  <Target className="w-5 h-5" />
                  产品概述
                </CardTitle>
              </CardHeader>
              <CardContent>
                <div className="prose prose-sm max-w-none">
                  <ReactMarkdown
                    remarkPlugins={[remarkGfm]}
                    components={{
                      table: TableComponent,
                      th: ThComponent,
                      td: TdComponent,
                      tr: TrComponent,
                    }}
                  >
                    {prd.overview}
                  </ReactMarkdown>
                </div>
              </CardContent>
            </Card>

            <Card>
              <CardHeader>
                <CardTitle className="flex items-center gap-2">
                  <Users className="w-5 h-5" />
                  目标用户
                </CardTitle>
              </CardHeader>
              <CardContent>
                <div className="flex flex-wrap gap-2">
                  {prd.targetUsers.map((user, index) => (
                    <Badge key={index} variant="secondary">
                      {user}
                    </Badge>
                  ))}
                </div>
              </CardContent>
            </Card>
          </TabsContent>

          <TabsContent value="features" className="space-y-4">
            <Card>
              <CardHeader>
                <CardTitle className="flex items-center gap-2">
                  <Zap className="w-5 h-5" />
                  核心功能
                </CardTitle>
              </CardHeader>
              <CardContent>
                <ul className="space-y-2">
                  {prd.coreFeatures.map((feature, index) => (
                    <li key={index} className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span className="flex-1">
                        <ReactMarkdown
                          remarkPlugins={[remarkGfm]}
                          components={{
                            p: ({node, ...props}) => <span {...props} />,
                            br: () => null,
                            table: TableComponent,
                            th: ThComponent,
                            td: TdComponent,
                            tr: TrComponent,
                          }}
                        >
                          {feature}
                        </ReactMarkdown>
                      </span>
                    </li>
                  ))}
                </ul>
              </CardContent>
            </Card>
          </TabsContent>

          <TabsContent value="tech" className="space-y-4">
            <Card>
              <CardHeader>
                <CardTitle>技术栈</CardTitle>
              </CardHeader>
              <CardContent>
                <div className="flex flex-wrap gap-2">
                  {prd.techStack.map((tech, index) => (
                    <Badge key={index} variant="outline">
                      {tech}
                    </Badge>
                  ))}
                </div>
              </CardContent>
            </Card>

            <Card>
              <CardHeader>
                <CardTitle className="flex items-center gap-2">
                  <Clock className="w-5 h-5" />
                  预估工作量
                </CardTitle>
              </CardHeader>
              <CardContent>
                <div className="prose prose-sm max-w-none">
                  <ReactMarkdown
                    remarkPlugins={[remarkGfm]}
                    components={{
                      table: TableComponent,
                      th: ThComponent,
                      td: TdComponent,
                      tr: TrComponent,
                    }}
                  >
                    {prd.estimatedEffort}
                  </ReactMarkdown>
                </div>
              </CardContent>
            </Card>
          </TabsContent>

          <TabsContent value="business" className="space-y-4">
            <Card>
              <CardHeader>
                <CardTitle>商业模式</CardTitle>
              </CardHeader>
              <CardContent>
                <div className="prose prose-sm max-w-none">
                  <ReactMarkdown
                    remarkPlugins={[remarkGfm]}
                    components={{
                      table: TableComponent,
                      th: ThComponent,
                      td: TdComponent,
                      tr: TrComponent,
                    }}
                  >
                    {prd.businessModel}
                  </ReactMarkdown>
                </div>
              </CardContent>
            </Card>

            <Card>
              <CardHeader>
                <CardTitle>定价策略</CardTitle>
              </CardHeader>
              <CardContent>
                <div className="prose prose-sm max-w-none">
                  <ReactMarkdown
                    remarkPlugins={[remarkGfm]}
                    components={{
                      table: TableComponent,
                      th: ThComponent,
                      td: TdComponent,
                      tr: TrComponent,
                    }}
                  >
                    {prd.pricing}
                  </ReactMarkdown>
                </div>
              </CardContent>
            </Card>
          </TabsContent>
        </Tabs>
      )}

      {/* 导出进度对话框 */}
      <Dialog open={showExportDialog} onOpenChange={setShowExportDialog}>
        <DialogContent className="sm:max-w-md">
          <DialogHeader>
            <DialogTitle>
              {exportStatus === 'success' ? '📦 导出产品需求文档' : '❌ 导出失败'}
            </DialogTitle>
            <DialogDescription>
              {exportStatus === 'success' 
                ? (isExporting ? '正在导出您的 PRD 文档...' : exportMessage)
                : exportMessage
              }
            </DialogDescription>
          </DialogHeader>
          
          {isExporting && (
            <div className="py-4">
              <div className="flex items-center justify-between mb-2">
                <span className="text-sm text-muted-foreground">导出进度</span>
                <span className="text-sm text-muted-foreground">{exportProgress}%</span>
              </div>
              <Progress value={exportProgress} className="w-full" />
              
              <div className="mt-4 flex items-center justify-center">
                <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary" />
              </div>
            </div>
          )}
          
          {!isExporting && exportStatus === 'error' && (
            <DialogFooter>
              <Button onClick={() => setShowExportDialog(false)}>关闭</Button>
              <Button onClick={handleExport} variant="outline">重试</Button>
            </DialogFooter>
          )}
          
          {!isExporting && exportStatus === 'success' && (
            <DialogFooter>
              <Button onClick={() => setShowExportDialog(false)}>确定</Button>
            </DialogFooter>
          )}
        </DialogContent>
      </Dialog>
    </div>
  )
}
