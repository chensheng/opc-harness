import { useState } from 'react'
import React from 'react'
import ReactMarkdown from 'react-markdown'
import remarkGfm from 'remark-gfm'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import { Textarea } from '@/components/ui/textarea'
import { ScrollArea } from '@/components/ui/scroll-area'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import {
  Sparkles,
  FileText,
  CheckCircle2,
  Clock,
  AlertCircle,
  Users,
  Target,
  Lightbulb,
  Edit2,
  Trash2,
  MessageSquare,
  Loader2,
} from 'lucide-react'
import { useUserStoryDecomposition } from '@/hooks/useUserStoryDecomposition'
import { useUserStoryStream } from '@/hooks/useUserStoryStream'
import type { UserStory } from '@/types'

// PRD 预览的自定义 Markdown 组件
const PRDPreviewComponents = {
  h1: ({ ...props }) => (
    <h1
      className="text-2xl font-bold mb-4 mt-6 pb-2 border-b border-border text-primary"
      {...props}
    />
  ),
  h2: ({ ...props }) => (
    <h2
      className="text-xl font-semibold mb-3 mt-5 pb-1 border-b border-border/50 text-foreground"
      {...props}
    />
  ),
  h3: ({ ...props }) => (
    <h3 className="text-lg font-medium mb-2 mt-4 text-foreground/90" {...props} />
  ),
  p: ({ ...props }) => (
    <p className="text-base leading-relaxed mb-3 text-foreground/90" {...props} />
  ),
  ul: ({ ...props }) => <ul className="list-disc list-outside pl-6 mb-3 space-y-1.5" {...props} />,
  ol: ({ ...props }) => (
    <ol className="list-decimal list-outside pl-6 mb-3 space-y-1.5" {...props} />
  ),
  li: ({ ...props }) => <li className="text-sm leading-relaxed text-foreground/85" {...props} />,
  strong: ({ ...props }) => <strong className="font-semibold text-foreground" {...props} />,
  em: ({ ...props }) => <em className="italic text-foreground/80" {...props} />,
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  code: ({ inline, ...props }: any) =>
    inline ? (
      <code
        className="bg-muted/80 px-1.5 py-0.5 rounded text-xs font-mono text-primary"
        {...props}
      />
    ) : (
      <code
        className="block bg-muted p-2.5 rounded-md my-2 overflow-x-auto text-xs font-mono"
        {...props}
      />
    ),
  pre: ({ ...props }) => (
    <pre
      className="bg-muted/50 p-3 rounded-md my-3 overflow-x-auto border border-border/30"
      {...props}
    />
  ),
  blockquote: ({ ...props }) => (
    <blockquote
      className="border-l-4 border-primary/50 pl-4 py-2 my-3 bg-muted/20 italic text-foreground/75"
      {...props}
    />
  ),
  table: ({ ...props }) => (
    <div className="overflow-x-auto my-6 first:mt-4 last:mb-4">
      <table className="w-full border-collapse border border-border" {...props} />
    </div>
  ),
  th: ({ ...props }) => (
    <th
      className="border border-border px-4 py-3 bg-muted/80 text-left font-semibold text-sm"
      {...props}
    />
  ),
  td: ({ ...props }) => (
    <td className="border border-border px-4 py-3 text-left text-sm" {...props} />
  ),
  tr: ({ ...props }) => (
    <tr className="even:bg-muted/30 hover:bg-muted/50 transition-colors" {...props} />
  ),
}

interface UserStoryManagerProps {
  /** 项目 PRD 内容（必需） */
  prdContent: string
  /** AI API Key（可选） */
  apiKey?: string
  /** 拆分完成后的回调 */
  onStoriesGenerated?: (stories: UserStory[]) => void
}

const priorityColors: Record<string, string> = {
  P0: 'bg-red-500 text-white',
  P1: 'bg-orange-500 text-white',
  P2: 'bg-yellow-500 text-white',
  P3: 'bg-gray-500 text-white',
}

const statusIcons: Record<string, React.ReactNode> = {
  draft: <Edit2 className="w-4 h-4" />,
  refined: <Lightbulb className="w-4 h-4" />,
  approved: <CheckCircle2 className="w-4 h-4" />,
  in_development: <Clock className="w-4 h-4" />,
  completed: <CheckCircle2 className="w-4 h-4 text-green-500" />,
}

/**
 * 用户故事管理组件
 *
 * 提供通过 AI 拆分 PRD 为用户故事的功能，并支持故事的查看、编辑和管理
 */
export function UserStoryManager({
  prdContent,
  apiKey: _apiKey, // 保留参数用于未来扩展,当前从 AI 配置 Store 自动获取
  onStoriesGenerated,
}: UserStoryManagerProps) {
  const [prompt, setPrompt] = useState('')
  const [activeTab, setActiveTab] = useState<'input' | 'stories'>('input')
  const [showStreamingView, setShowStreamingView] = useState(false)

  // 使用流式 Hook
  const {
    markdownContent,
    userStories: streamUserStories,
    isStreaming,
    isComplete,
    error: streamError,
    startStream,
    reset: resetStream,
  } = useUserStoryStream()

  // 使用非流式 Hook (作为后备)
  const {
    userStories: _userStories,
    loading: _loading,
    error: _error,
    decompose: _decompose,
    reset: _reset,
  } = useUserStoryDecomposition()

  // 优先使用流式的用户故事
  const displayLoading = isStreaming || _loading
  const displayError = streamError || _error
  const displayStories = streamUserStories.length > 0 ? streamUserStories : _userStories

  // 调试日志：检查接收到的 PRD 内容
  React.useEffect(() => {
    if (prdContent) {
      console.log('[UserStoryManager] Received PRD content:', {
        length: prdContent.length,
        hasTableSyntax: prdContent.includes('|'),
        preview: prdContent.substring(0, 300),
      })

      // 如果包含表格，输出表格部分
      if (prdContent.includes('|')) {
        const tableMatch = prdContent.match(/\|.*\|[\s\S]*?\|[-|\s]+\|[\s\S]*?\|.*\|/m)
        if (tableMatch) {
          console.log('[UserStoryManager] Table content preview:', tableMatch[0].substring(0, 400))
        }
      }
    }
  }, [prdContent])

  const handleDecompose = async () => {
    // 组合 PRD 内容和用户提示词
    const fullContent = prompt.trim() ? `${prdContent}\n\n---\n\n用户要求：${prompt}` : prdContent

    // 从 AI 配置 Store 获取配置
    const { useAIConfigStore } = await import('@/stores/aiConfigStore')
    const store = useAIConfigStore.getState()
    const activeConfig = store.getActiveConfig()

    if (!activeConfig?.apiKey) {
      alert('未配置 AI API Key，请先在设置中配置')
      return
    }

    // 显示流式视图
    setShowStreamingView(true)

    // 开始流式拆分
    await startStream({
      prdContent: fullContent,
      provider: activeConfig.provider,
      model: activeConfig.model,
      apiKey: activeConfig.apiKey,
    })

    // 完成后切换到故事 Tab
    if (isComplete && streamUserStories.length > 0 && onStoriesGenerated) {
      onStoriesGenerated(streamUserStories)
      setActiveTab('stories')
    }
  }

  const handleReset = () => {
    _reset()
    resetStream()
    setShowStreamingView(false)
    setActiveTab('input')
  }

  const getStoryStats = () => {
    const stories = displayStories || []
    const total = stories.length
    const p0 = stories.filter((s: UserStory) => s.priority === 'P0').length
    const p1 = stories.filter((s: UserStory) => s.priority === 'P1').length
    const avgPoints = stories.reduce((sum, s) => sum + (s.storyPoints || 0), 0) / total || 0

    return { total, p0, p1, avgPoints }
  }

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-2xl font-bold flex items-center gap-2">
            <Users className="w-6 h-6" />
            用户故事管理
          </h2>
          <p className="text-muted-foreground mt-1">
            通过 AI 将 PRD 拆分为符合 INVEST 原则的用户故事
          </p>
        </div>
        {displayStories.length > 0 && (
          <Button variant="outline" onClick={handleReset}>
            重新拆分
          </Button>
        )}
      </div>

      <Tabs value={activeTab} onValueChange={value => setActiveTab(value as 'input' | 'stories')}>
        <TabsList className="grid w-full grid-cols-2">
          <TabsTrigger value="input">
            <FileText className="w-4 h-4 mr-2" />
            拆分配置
          </TabsTrigger>
          <TabsTrigger value="stories" disabled={!displayStories || displayStories.length === 0}>
            <FileText className="w-4 h-4" />
            用户故事 ({displayStories?.length || 0})
          </TabsTrigger>
        </TabsList>

        {/* Input Tab */}
        <TabsContent value="input" className="space-y-4">
          {/* PRD Preview Card */}
          <Card>
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <FileText className="w-5 h-5" />
                项目 PRD
              </CardTitle>
              <CardDescription>AI 将基于以下 PRD 内容拆分用户故事</CardDescription>
            </CardHeader>
            <CardContent>
              <ScrollArea className="h-[400px] w-full rounded-md border p-6 bg-card">
                {prdContent ? (
                  <div className="prose prose-sm dark:prose-invert max-w-none">
                    <ReactMarkdown remarkPlugins={[remarkGfm]} components={PRDPreviewComponents}>
                      {prdContent}
                    </ReactMarkdown>
                  </div>
                ) : (
                  <div className="flex items-center justify-center h-full text-muted-foreground">
                    <div className="text-center">
                      <FileText className="w-12 h-12 mx-auto mb-3 opacity-50" />
                      <p className="text-sm">暂无 PRD 内容</p>
                    </div>
                  </div>
                )}
              </ScrollArea>
            </CardContent>
          </Card>

          {/* Prompt Input Card */}
          <Card>
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <MessageSquare className="w-5 h-5" />
                拆分要求（可选）
              </CardTitle>
              <CardDescription>
                输入额外的拆分要求或关注点，AI 会根据您的要求优化拆分结果
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <Textarea
                placeholder="例如：&#10;- 重点关注用户认证和权限管理相关的故事&#10;- 优先拆分核心业务流程&#10;- 考虑技术债务和重构需求&#10;- 需要包含单元测试和文档编写任务..."
                value={prompt}
                onChange={e => setPrompt(e.target.value)}
                className="min-h-[150px]"
              />

              {displayError && (
                <div className="flex items-start gap-2 p-3 bg-red-50 dark:bg-red-950/20 border border-red-200 dark:border-red-800 rounded-md">
                  <AlertCircle className="w-5 h-5 text-red-500 mt-0.5" />
                  <div className="text-sm text-red-700 dark:text-red-400">{displayError}</div>
                </div>
              )}

              {/* 流式响应实时显示 */}
              {showStreamingView && isStreaming && (
                <Card className="border-primary/50 bg-gradient-to-br from-primary/5 to-purple-500/5">
                  <CardHeader className="pb-3">
                    <CardTitle className="flex items-center gap-2 text-lg">
                      <Loader2 className="w-5 h-5 animate-spin text-primary" />
                      AI 正在生成用户故事...
                    </CardTitle>
                    <CardDescription>实时查看 AI 拆分的用户故事内容</CardDescription>
                  </CardHeader>
                  <CardContent>
                    <ScrollArea className="h-[400px] w-full rounded-md border p-4 bg-background/50">
                      {markdownContent ? (
                        <div className="prose prose-sm dark:prose-invert max-w-none">
                          <ReactMarkdown
                            remarkPlugins={[remarkGfm]}
                            components={PRDPreviewComponents}
                          >
                            {markdownContent}
                          </ReactMarkdown>
                        </div>
                      ) : (
                        <div className="flex items-center justify-center h-full text-muted-foreground">
                          <div className="text-center space-y-2">
                            <Loader2 className="w-8 h-8 animate-spin mx-auto" />
                            <p className="text-sm">正在连接 AI...</p>
                          </div>
                        </div>
                      )}
                    </ScrollArea>
                  </CardContent>
                </Card>
              )}

              <Button
                onClick={handleDecompose}
                disabled={!prdContent || displayLoading}
                className="w-full"
                size="lg"
              >
                {displayLoading ? (
                  <>
                    <Sparkles className="w-5 h-5 mr-2 animate-spin" />
                    AI 正在拆分中...
                  </>
                ) : (
                  <>
                    <Sparkles className="w-5 h-5 mr-2" />
                    开始拆分用户故事
                  </>
                )}
              </Button>

              <div className="text-xs text-muted-foreground space-y-1">
                <p>💡 提示：</p>
                <ul className="list-disc list-inside space-y-1 ml-2">
                  <li>AI 会自动基于项目 PRD 进行拆分</li>
                  <li>可以输入额外要求来指导拆分方向</li>
                  <li>遵循 INVEST 原则生成高质量用户故事</li>
                  <li>每个故事包含验收标准和优先级评估</li>
                </ul>
              </div>
            </CardContent>
          </Card>
        </TabsContent>

        {/* Stories Tab */}
        <TabsContent value="stories">
          {displayStories && displayStories.length > 0 && (
            <div className="space-y-4">
              {/* Statistics */}
              <div className="grid grid-cols-4 gap-4">
                <Card>
                  <CardContent className="pt-6">
                    <div className="text-2xl font-bold">{getStoryStats().total}</div>
                    <p className="text-xs text-muted-foreground">总故事数</p>
                  </CardContent>
                </Card>
                <Card>
                  <CardContent className="pt-6">
                    <div className="text-2xl font-bold text-red-500">{getStoryStats().p0}</div>
                    <p className="text-xs text-muted-foreground">P0 优先级</p>
                  </CardContent>
                </Card>
                <Card>
                  <CardContent className="pt-6">
                    <div className="text-2xl font-bold text-orange-500">{getStoryStats().p1}</div>
                    <p className="text-xs text-muted-foreground">P1 优先级</p>
                  </CardContent>
                </Card>
                <Card>
                  <CardContent className="pt-6">
                    <div className="text-2xl font-bold">{getStoryStats().avgPoints.toFixed(1)}</div>
                    <p className="text-xs text-muted-foreground">平均故事点</p>
                  </CardContent>
                </Card>
              </div>

              {/* Story List */}
              <ScrollArea className="h-[600px]">
                <div className="space-y-3">
                  {_userStories?.map((story: UserStory) => (
                    <UserStoryCard key={story.id} story={story} />
                  ))}
                </div>
              </ScrollArea>
            </div>
          )}
        </TabsContent>
      </Tabs>
    </div>
  )
}

/**
 * 用户故事卡片组件
 */
interface UserStoryCardProps {
  story: UserStory
}

function UserStoryCard({ story }: UserStoryCardProps) {
  return (
    <Card className="hover:shadow-md transition-shadow">
      <CardHeader className="pb-3">
        <div className="flex items-start justify-between gap-4">
          <div className="flex-1 space-y-2">
            <div className="flex items-center gap-2">
              <Badge variant="outline" className="font-mono">
                {story.storyNumber}
              </Badge>
              <Badge className={priorityColors[story.priority]}>{story.priority}</Badge>
              <div className="flex items-center gap-1 text-sm text-muted-foreground">
                {statusIcons[story.status]}
                <span className="capitalize">{story.status.replace('_', ' ')}</span>
              </div>
            </div>

            <CardTitle className="text-lg">{story.title}</CardTitle>
          </div>

          <div className="flex gap-2">
            <Button variant="ghost" size="sm">
              <Edit2 className="w-4 h-4" />
            </Button>
            <Button variant="ghost" size="sm">
              <Trash2 className="w-4 h-4" />
            </Button>
          </div>
        </div>
      </CardHeader>

      <CardContent className="space-y-4">
        {/* User Story Format */}
        <div className="space-y-2 p-3 bg-blue-50 dark:bg-blue-950/20 rounded-md">
          <div className="flex items-start gap-2">
            <Users className="w-4 h-4 text-blue-500 mt-0.5" />
            <div>
              <span className="font-medium text-blue-700 dark:text-blue-400">As a </span>
              <span className="text-blue-600 dark:text-blue-300">{story.role}</span>
            </div>
          </div>
          <div className="flex items-start gap-2">
            <Target className="w-4 h-4 text-blue-500 mt-0.5" />
            <div>
              <span className="font-medium text-blue-700 dark:text-blue-400">I want </span>
              <span className="text-blue-600 dark:text-blue-300">{story.feature}</span>
            </div>
          </div>
          <div className="flex items-start gap-2">
            <Lightbulb className="w-4 h-4 text-blue-500 mt-0.5" />
            <div>
              <span className="font-medium text-blue-700 dark:text-blue-400">So that </span>
              <span className="text-blue-600 dark:text-blue-300">{story.benefit}</span>
            </div>
          </div>
        </div>

        {/* Description */}
        {story.description && (
          <div>
            <p className="text-sm font-medium mb-1">详细描述</p>
            <p className="text-sm text-muted-foreground">{story.description}</p>
          </div>
        )}

        {/* Acceptance Criteria */}
        {story.acceptanceCriteria.length > 0 && (
          <div>
            <p className="text-sm font-medium mb-2">验收标准</p>
            <ul className="space-y-1">
              {story.acceptanceCriteria.map((criteria, idx) => (
                <li key={idx} className="flex items-start gap-2 text-sm">
                  <CheckCircle2 className="w-4 h-4 text-green-500 mt-0.5 flex-shrink-0" />
                  <span className="text-muted-foreground">{criteria}</span>
                </li>
              ))}
            </ul>
          </div>
        )}

        {/* Meta Info */}
        <div className="flex items-center gap-4 text-xs text-muted-foreground pt-2 border-t">
          {story.storyPoints && (
            <div className="flex items-center gap-1">
              <Target className="w-3 h-3" />
              <span>{story.storyPoints} 故事点</span>
            </div>
          )}
          {story.featureModule && <div>模块: {story.featureModule}</div>}
          {story.labels.length > 0 && (
            <div className="flex gap-1">
              {story.labels.slice(0, 3).map((label, idx) => (
                <Badge key={idx} variant="secondary" className="text-xs">
                  {label}
                </Badge>
              ))}
              {story.labels.length > 3 && (
                <Badge variant="secondary" className="text-xs">
                  +{story.labels.length - 3}
                </Badge>
              )}
            </div>
          )}
        </div>
      </CardContent>
    </Card>
  )
}
