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
  ChevronLeft,
  ChevronRight,
  ArrowUpDown,
  ArrowUp,
  ArrowDown,
} from 'lucide-react'
import type { UserStory } from '@/types'
import { useUserStoryDecomposition } from '@/hooks/useUserStoryDecomposition'
import { useUserStoryStream } from '@/hooks/useUserStoryStream'
import { useProjectStore } from '@/stores/projectStore'
import { useUserStoryStore } from '@/stores/userStoryStore'

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

const statusColors: Record<string, string> = {
  draft: 'bg-gray-100 text-gray-700 border border-gray-300',
  refined: 'bg-blue-50 text-blue-700 border border-blue-200',
  approved: 'bg-green-50 text-green-700 border border-green-200',
  in_development: 'bg-purple-50 text-purple-700 border border-purple-200',
  completed: 'bg-emerald-50 text-emerald-700 border border-emerald-200',
}

const statusLabels: Record<string, string> = {
  draft: '草稿',
  refined: '已细化',
  approved: '已批准',
  in_development: '开发中',
  completed: '已完成',
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

  // 分页状态
  const [currentPage, setCurrentPage] = useState(1)
  const [pageSize, setPageSize] = useState(10) // 每页显示10个故事
  
  // 排序状态（支持多条件排序）
  type SortField = 'priority' | 'storyPoints'
  const [sortConfigs, setSortConfigs] = useState<Array<{ field: SortField; order: 'asc' | 'desc' }>>([])

  // 使用流式 Hook
  const {
    markdownContent,
    userStories: streamUserStories,
    isStreaming,
    isComplete: _isComplete,
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

  // 获取当前项目ID和用户故事Store
  const currentProjectId = useProjectStore(state => state.currentProjectId)
  const loadProjectStories = useUserStoryStore(state => state.loadProjectStories)
  const isLoadingFromDB = useUserStoryStore(state => state.isLoading)

  // 直接从 Store 订阅用户故事(响应式)
  const savedStories = useUserStoryStore(state =>
    currentProjectId ? state.storiesByProject[currentProjectId] || [] : []
  )

  // 组件挂载时加载用户故事
  React.useEffect(() => {
    if (currentProjectId) {
      loadProjectStories(currentProjectId)
    }
  }, [currentProjectId, loadProjectStories])

  // 优先使用流式的用户故事，否则使用保存的故事
  const displayLoading = isStreaming || _loading
  const displayError = streamError || _error
  const displayStories =
    streamUserStories.length > 0
      ? streamUserStories
      : _userStories.length > 0
        ? _userStories
        : savedStories

  const handleDecompose = async () => {
    // 组合 PRD 内容和用户提示词
    const fullContent = prompt.trim() ? `${prdContent}\n\n---\n\n用户要求：${prompt}` : prdContent

    // 从 AI 配置 Store 获取配置
    const { useAIConfigStore } = await import('@/stores/aiConfigStore')
    const store = useAIConfigStore.getState()
    const activeConfig = store.getActiveConfig()

    // CodeFree CLI 不需要 API Key，其他 provider 需要检查
    if (activeConfig?.provider !== 'codefree' && !activeConfig?.apiKey) {
      alert('未配置 AI API Key，请先在设置中配置')
      return
    }

    // 显示流式视图
    setShowStreamingView(true)

    // 开始流式拆分，并在完成后自动保存
    await startStream(
      {
        prdContent: fullContent,
        provider: store.defaultProvider, // ✅ 使用默认厂商而非activeConfig.provider
        model: activeConfig.model,
        apiKey: activeConfig.apiKey || '', // codefree 传空字符串
        projectId: currentProjectId || undefined, // 传递项目 ID 用于 CodeFree 文件写入
      },
      stories => {
        // 流式完成后自动调用保存回调
        if (onStoriesGenerated) {
          onStoriesGenerated(stories)
        }
        setActiveTab('stories')
      }
    )
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

  // 计算分页数据（先排序再分页）
  const sortedStories = React.useMemo(() => {
    if (sortConfigs.length === 0) return displayStories

    return [...displayStories].sort((a: UserStory, b: UserStory) => {
      let comparison = 0

      for (const config of sortConfigs) {
        if (config.field === 'priority') {
          // 优先级排序：P0 < P1 < P2 < P3
          const priorityOrder: Record<string, number> = { P0: 0, P1: 1, P2: 2, P3: 3 }
          const aPriority = priorityOrder[a.priority] ?? 999
          const bPriority = priorityOrder[b.priority] ?? 999
          comparison = aPriority - bPriority
        } else if (config.field === 'storyPoints') {
          // 故事点排序
          const aPoints = a.storyPoints || 0
          const bPoints = b.storyPoints || 0
          comparison = aPoints - bPoints
        }

        if (comparison !== 0) {
          return config.order === 'asc' ? comparison : -comparison
        }
      }

      return 0
    })
  }, [displayStories, sortConfigs])

  const totalPages = Math.ceil(sortedStories.length / pageSize)
  const startIndex = (currentPage - 1) * pageSize
  const endIndex = startIndex + pageSize
  const paginatedStories = sortedStories.slice(startIndex, endIndex)

  // 处理排序切换（支持多条件排序）
  const handleSort = (field: SortField, event?: React.MouseEvent) => {
    const existingConfig = sortConfigs.find(config => config.field === field)
    
    if (existingConfig) {
      // 如果字段已存在，切换排序方向
      const newOrder = existingConfig.order === 'asc' ? 'desc' : 'asc'
      setSortConfigs(prevConfigs =>
        prevConfigs.map(config =>
          config.field === field ? { ...config, order: newOrder } : config
        )
      )
    } else {
      // 如果字段不存在，添加到排序条件列表
      setSortConfigs(prevConfigs => [...prevConfigs, { field, order: 'asc' }])
    }
    
    // 重置到第一页
    setCurrentPage(1)
  }
  
  // 清除所有排序条件
  const clearSort = () => {
    setSortConfigs([])
    setCurrentPage(1)
  }
  
  // 获取字段的排序配置
  const getSortConfig = (field: SortField) => {
    return sortConfigs.find(config => config.field === field)
  }
  
  // 获取字段在排序列表中的序号（用于多条件排序显示）
  const getSortIndex = (field: SortField) => {
    const index = sortConfigs.findIndex(config => config.field === field)
    return index >= 0 ? index + 1 : null
  }
  
  // 重置分页（当故事列表变化时）
  React.useEffect(() => {
    setCurrentPage(1)
  }, [displayStories.length])

  const handlePageChange = (page: number) => {
    setCurrentPage(page)
  }

  const handlePageSizeChange = (size: number) => {
    setPageSize(size)
    setCurrentPage(1) // 重置到第一页
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
          <TabsTrigger
            value="stories"
            disabled={(!displayStories || displayStories.length === 0) && !isLoadingFromDB}
          >
            {isLoadingFromDB ? (
              <>
                <Loader2 className="w-4 h-4 mr-2 animate-spin" />
                加载中...
              </>
            ) : (
              <>
                <FileText className="w-4 h-4 mr-2" />
                用户故事 ({displayStories?.length || 0})
              </>
            )}
          </TabsTrigger>
        </TabsList>

        {/* Input Tab */}
        <TabsContent value="input" className="space-y-3">
          {/* 紧凑布局：PRD 预览和输入框并排 */}
          <div className="grid grid-cols-2 gap-3">
            {/* PRD Preview Card - 左侧 */}
            <Card className="h-[calc(100vh-280px)] flex flex-col">
              <CardHeader className="pb-2 pt-3 px-4">
                <CardTitle className="flex items-center gap-2 text-base">
                  <FileText className="w-4 h-4" />
                  项目 PRD
                </CardTitle>
                <CardDescription className="text-xs">AI 将基于此内容拆分</CardDescription>
              </CardHeader>
              <CardContent className="flex-1 overflow-hidden px-4 pb-3">
                <ScrollArea className="h-full w-full rounded-md border p-3 bg-card">
                  {prdContent ? (
                    <div className="prose prose-xs dark:prose-invert max-w-none">
                      <ReactMarkdown remarkPlugins={[remarkGfm]} components={PRDPreviewComponents}>
                        {prdContent}
                      </ReactMarkdown>
                    </div>
                  ) : (
                    <div className="flex items-center justify-center h-full text-muted-foreground">
                      <div className="text-center">
                        <FileText className="w-8 h-8 mx-auto mb-2 opacity-50" />
                        <p className="text-xs">暂无 PRD 内容</p>
                      </div>
                    </div>
                  )}
                </ScrollArea>
              </CardContent>
            </Card>

            {/* Prompt Input Card - 右侧 */}
            <Card className="h-[calc(100vh-280px)] flex flex-col">
              <CardHeader className="pb-2 pt-3 px-4">
                <CardTitle className="flex items-center gap-2 text-base">
                  <MessageSquare className="w-4 h-4" />
                  拆分要求
                </CardTitle>
                <CardDescription className="text-xs">可选的额外要求</CardDescription>
              </CardHeader>
              <CardContent className="flex-1 overflow-hidden px-4 pb-3 flex flex-col space-y-3">
                <Textarea
                  placeholder="例如：&#10;- 重点关注用户认证&#10;- 优先核心业务流程&#10;- 考虑技术债务..."
                  value={prompt}
                  onChange={e => setPrompt(e.target.value)}
                  className="min-h-[120px] flex-1 resize-none text-sm"
                />

                {displayError && (
                  <div className="flex items-start gap-2 p-2 bg-red-50 dark:bg-red-950/20 border border-red-200 dark:border-red-800 rounded-md">
                    <AlertCircle className="w-4 h-4 text-red-500 mt-0.5 flex-shrink-0" />
                    <div className="text-xs text-red-700 dark:text-red-400">{displayError}</div>
                  </div>
                )}

                {/* 流式响应实时显示 - 紧凑版 */}
                {showStreamingView && isStreaming && (
                  <Card className="border-primary/50 bg-gradient-to-br from-primary/5 to-purple-500/5 flex-1">
                    <CardHeader className="pb-2 pt-2 px-3">
                      <CardTitle className="flex items-center gap-2 text-sm">
                        <Loader2 className="w-4 h-4 animate-spin text-primary" />
                        AI 生成中...
                      </CardTitle>
                    </CardHeader>
                    <CardContent className="px-3 pb-3">
                      <ScrollArea className="h-[calc(100%-40px)] w-full rounded-md border p-2 bg-background/50">
                        {markdownContent ? (
                          <div className="prose prose-xs dark:prose-invert max-w-none">
                            <ReactMarkdown
                              remarkPlugins={[remarkGfm]}
                              components={PRDPreviewComponents}
                            >
                              {markdownContent}
                            </ReactMarkdown>
                          </div>
                        ) : (
                          <div className="flex items-center justify-center h-20 text-muted-foreground">
                            <div className="text-center space-y-1">
                              <Loader2 className="w-6 h-6 animate-spin mx-auto" />
                              <p className="text-xs">正在连接 AI...</p>
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
                  size="sm"
                >
                  {displayLoading ? (
                    <>
                      <Sparkles className="w-4 h-4 mr-2 animate-spin" />
                      AI 拆分中...
                    </>
                  ) : (
                    <>
                      <Sparkles className="w-4 h-4 mr-2" />
                      开始拆分用户故事
                    </>
                  )}
                </Button>

                <div className="text-xs text-muted-foreground space-y-0.5">
                  <p>💡 提示：</p>
                  <ul className="list-disc list-inside space-y-0.5 ml-1 text-[10px]">
                    <li>AI 自动基于 PRD 拆分</li>
                    <li>可输入额外要求指导拆分</li>
                    <li>遵循 INVEST 原则</li>
                    <li>包含验收标准和优先级</li>
                  </ul>
                </div>
              </CardContent>
            </Card>
          </div>
        </TabsContent>

        {/* Stories Tab */}
        <TabsContent value="stories">
          {displayStories && displayStories.length > 0 && (
            <div className="space-y-2">
              {/* Story List - 表格形式（超紧凑版） */}
              <Card>
                <CardContent className="p-0">
                  <ScrollArea className="h-[calc(100vh-380px)]">
                    <table className="w-full border-collapse text-xs">
                      <thead className="sticky top-0 bg-muted/90 backdrop-blur-sm z-10">
                        <tr className="border-b border-border">
                          <th className="text-left py-1.5 px-2 font-semibold text-[10px] w-16">序号</th>
                          <th 
                            className="text-left py-1.5 px-2 font-semibold text-[10px] w-16 cursor-pointer hover:bg-muted/50 transition-colors select-none"
                            onClick={(e) => handleSort('priority', e)}
                            title="点击切换排序，多次点击可添加多条件排序"
                          >
                            <div className="flex items-center gap-0.5">
                              <span>优先</span>
                              {(() => {
                                const config = getSortConfig('priority')
                                const index = getSortIndex('priority')
                                if (!config) {
                                  return <ArrowUpDown className="w-2.5 h-2.5 text-muted-foreground" />
                                }
                                return (
                                  <div className="flex items-center gap-0.5">
                                    {config.order === 'asc' ? (
                                      <ArrowUp className="w-2.5 h-2.5" />
                                    ) : (
                                      <ArrowDown className="w-2.5 h-2.5" />
                                    )}
                                    {sortConfigs.length > 1 && index && (
                                      <span className="text-[8px] bg-primary text-primary-foreground rounded-full w-3 h-3 flex items-center justify-center">
                                        {index}
                                      </span>
                                    )}
                                  </div>
                                )
                              })()}
                            </div>
                          </th>
                          <th className="text-left py-1.5 px-2 font-semibold text-[10px]">标题</th>
                          <th className="text-left py-1.5 px-2 font-semibold text-[10px] w-24">角色</th>
                          <th 
                            className="text-left py-1.5 px-2 font-semibold text-[10px] w-16 cursor-pointer hover:bg-muted/50 transition-colors select-none"
                            onClick={(e) => handleSort('storyPoints', e)}
                            title="点击切换排序，多次点击可添加多条件排序"
                          >
                            <div className="flex items-center gap-0.5">
                              <span>点数</span>
                              {(() => {
                                const config = getSortConfig('storyPoints')
                                const index = getSortIndex('storyPoints')
                                if (!config) {
                                  return <ArrowUpDown className="w-2.5 h-2.5 text-muted-foreground" />
                                }
                                return (
                                  <div className="flex items-center gap-0.5">
                                    {config.order === 'asc' ? (
                                      <ArrowUp className="w-2.5 h-2.5" />
                                    ) : (
                                      <ArrowDown className="w-2.5 h-2.5" />
                                    )}
                                    {sortConfigs.length > 1 && index && (
                                      <span className="text-[8px] bg-primary text-primary-foreground rounded-full w-3 h-3 flex items-center justify-center">
                                        {index}
                                      </span>
                                    )}
                                  </div>
                                )
                              })()}
                            </div>
                          </th>
                          <th className="text-left py-1.5 px-2 font-semibold text-[10px] w-16">状态</th>
                          <th className="text-left py-1.5 px-2 font-semibold text-[10px] w-16">操作</th>
                        </tr>
                      </thead>
                      <tbody>
                        {paginatedStories.map((story: UserStory, index: number) => (
                          <tr
                            key={story.id}
                            className={`border-b border-border/30 hover:bg-muted/20 transition-colors ${
                              index % 2 === 0 ? 'bg-background' : 'bg-muted/5'
                            }`}
                          >
                            <td className="py-1.5 px-2 align-middle">
                              <span className="font-mono text-[10px]">{story.storyNumber}</span>
                            </td>
                            <td className="py-1.5 px-2 align-middle">
                              <Badge className={`${priorityColors[story.priority]} text-[9px] px-1 py-0 h-4`}>
                                {story.priority}
                              </Badge>
                            </td>
                            <td className="py-1.5 px-2 align-middle">
                              <div className="space-y-0.5">
                                <div className="font-medium text-xs leading-tight">{story.title}</div>
                                <div className="text-[10px] text-muted-foreground line-clamp-1">
                                  {story.feature}
                                </div>
                              </div>
                            </td>
                            <td className="py-1.5 px-2 align-middle">
                              <div className="text-[10px] truncate" title={story.role}>{story.role}</div>
                            </td>
                            <td className="py-1.5 px-2 align-middle text-center">
                              {story.storyPoints && (
                                <span className="text-[10px]">{story.storyPoints}</span>
                              )}
                            </td>
                            <td className="py-1.5 px-2 align-middle">
                              <Badge 
                                className={`${statusColors[story.status] || 'bg-gray-100 text-gray-700'} text-[9px] px-1.5 py-0 h-4 font-medium`}
                              >
                                {statusLabels[story.status] || story.status}
                              </Badge>
                            </td>
                            <td className="py-1.5 px-2 align-middle">
                              <div className="flex gap-0.5">
                                <Button variant="ghost" size="sm" className="h-5 w-5 p-0">
                                  <Edit2 className="w-2.5 h-2.5" />
                                </Button>
                                <Button variant="ghost" size="sm" className="h-5 w-5 p-0">
                                  <Trash2 className="w-2.5 h-2.5" />
                                </Button>
                              </div>
                            </td>
                          </tr>
                        ))}
                      </tbody>
                    </table>
                  </ScrollArea>
                </CardContent>
              </Card>

              {/* Pagination Controls - 超紧凑版 */}
              <div className="flex items-center justify-between py-1 px-1.5 border-t">
                <div className="flex items-center gap-1 text-[10px]">
                  <span className="text-muted-foreground">每页:</span>
                  <div className="flex gap-0.5">
                    {[10, 20, 50].map(size => (
                      <Button
                        key={size}
                        variant={pageSize === size ? 'default' : 'outline'}
                        size="sm"
                        onClick={() => handlePageSizeChange(size)}
                        className="h-5 px-1.5 text-[9px]"
                      >
                        {size}
                      </Button>
                    ))}
                  </div>
                  
                  {/* 排序状态显示和清除按钮 */}
                  {sortConfigs.length > 0 && (
                    <>
                      <span className="text-muted-foreground ml-2">|</span>
                      <div className="flex items-center gap-1">
                        <span className="text-muted-foreground">排序:</span>
                        <div className="flex gap-0.5">
                          {sortConfigs.map((config, idx) => (
                            <Badge 
                              key={config.field} 
                              variant="secondary" 
                              className="text-[9px] h-4 px-1 flex items-center gap-0.5"
                            >
                              <span>{config.field === 'priority' ? '优先' : '点数'}</span>
                              {config.order === 'asc' ? '↑' : '↓'}
                              {sortConfigs.length > 1 && (
                                <span className="text-[8px] opacity-70">{idx + 1}</span>
                              )}
                            </Badge>
                          ))}
                        </div>
                        <Button
                          variant="ghost"
                          size="sm"
                          onClick={clearSort}
                          className="h-4 px-1 text-[9px] hover:bg-destructive/10 hover:text-destructive"
                          title="清除所有排序"
                        >
                          ✕
                        </Button>
                      </div>
                    </>
                  )}
                </div>

                {totalPages > 1 && (
                  <div className="flex items-center gap-0.5">
                    <Button
                      variant="outline"
                      size="sm"
                      onClick={() => handlePageChange(currentPage - 1)}
                      disabled={currentPage === 1}
                      className="gap-0.5 h-5 px-1.5 text-[9px]"
                    >
                      <ChevronLeft className="w-2.5 h-2.5" />
                      上页
                    </Button>

                    <div className="flex items-center gap-0.5">
                      {Array.from({ length: Math.min(5, totalPages) }, (_, i) => {
                        let pageNum
                        if (totalPages <= 5) {
                          pageNum = i + 1
                        } else if (currentPage <= 3) {
                          pageNum = i + 1
                        } else if (currentPage >= totalPages - 2) {
                          pageNum = totalPages - 4 + i
                        } else {
                          pageNum = currentPage - 2 + i
                        }

                        return (
                          <Button
                            key={pageNum}
                            variant={currentPage === pageNum ? 'default' : 'outline'}
                            size="sm"
                            onClick={() => handlePageChange(pageNum)}
                            className="w-5 h-5 p-0 text-[9px]"
                          >
                            {pageNum}
                          </Button>
                        )
                      })}
                    </div>

                    <Button
                      variant="outline"
                      size="sm"
                      onClick={() => handlePageChange(currentPage + 1)}
                      disabled={currentPage === totalPages}
                      className="gap-0.5 h-5 px-1.5 text-[9px]"
                    >
                      下页
                      <ChevronRight className="w-2.5 h-2.5" />
                    </Button>
                  </div>
                )}
              </div>
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
  compact?: boolean // 紧凑模式
}

function UserStoryCard({ story, compact = false }: UserStoryCardProps) {
  if (compact) {
    return <CompactUserStoryCard story={story} />
  }

  return (
    <Card className={`hover:shadow-md transition-shadow ${compact ? 'py-1' : ''}`}>
      <CardHeader className={`${compact ? 'py-2 px-4' : 'pb-3'}`}>
        <div className="flex items-start justify-between gap-4">
          <div className="flex-1 space-y-1">
            <div className="flex items-center gap-2 flex-wrap">
              <Badge variant="outline" className="font-mono text-xs">
                {story.storyNumber}
              </Badge>
              <Badge className={`${priorityColors[story.priority]} text-xs`}>{story.priority}</Badge>
              <div className="flex items-center gap-1 text-xs text-muted-foreground">
                {statusIcons[story.status]}
                <span className="capitalize">{story.status.replace('_', ' ')}</span>
              </div>
            </div>

            <CardTitle className={`${compact ? 'text-base' : 'text-lg'}`}>{story.title}</CardTitle>
          </div>

          {!compact && (
            <div className="flex gap-2">
              <Button variant="ghost" size="sm">
                <Edit2 className="w-4 h-4" />
              </Button>
              <Button variant="ghost" size="sm">
                <Trash2 className="w-4 h-4" />
              </Button>
            </div>
          )}
        </div>
      </CardHeader>

      <CardContent className={`${compact ? 'pt-0 px-4 pb-3' : 'space-y-4'}`}>
        {!compact ? (
          <>
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
          </>
        ) : (
          /* Compact View Summary */
          <div className="space-y-2">
             <p className="text-sm text-muted-foreground line-clamp-2">
               <span className="font-medium text-foreground">As a</span> {story.role},{' '}
               <span className="font-medium text-foreground">I want</span> {story.feature},{' '}
               <span className="font-medium text-foreground">so that</span> {story.benefit}.
             </p>
             
             {story.acceptanceCriteria.length > 0 && (
                <div className="flex items-center gap-2 text-xs text-muted-foreground">
                   <CheckCircle2 className="w-3 h-3 text-green-500" />
                   <span>{story.acceptanceCriteria.length} 个验收标准</span>
                </div>
             )}
          </div>
        )}

        {/* Meta Info */}
        <div className={`flex items-center gap-4 text-xs text-muted-foreground ${!compact ? 'pt-2 border-t' : ''}`}>
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
                <Badge key={idx} variant="secondary" className="text-[10px] px-1 py-0">
                  {label}
                </Badge>
              ))}
              {story.labels.length > 3 && (
                <Badge variant="secondary" className="text-[10px] px-1 py-0">
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

/**
 * 紧凑型用户故事卡片组件 - 用于分页列表显示
 */
function CompactUserStoryCard({ story }: { story: UserStory }) {
  return (
    <Card className="hover:shadow-md transition-shadow">
      <CardHeader className="pb-2 pt-3 px-4">
        <div className="flex items-start justify-between gap-3">
          <div className="flex-1 space-y-1.5">
            <div className="flex items-center gap-2 flex-wrap">
              <Badge variant="outline" className="font-mono text-xs">
                {story.storyNumber}
              </Badge>
              <Badge className={`${priorityColors[story.priority]} text-xs`}>{story.priority}</Badge>
              <div className="flex items-center gap-1 text-xs text-muted-foreground">
                {statusIcons[story.status]}
                <span className="capitalize">{story.status.replace('_', ' ')}</span>
              </div>
            </div>

            <CardTitle className="text-base leading-tight">{story.title}</CardTitle>
          </div>

          <div className="flex gap-1">
            <Button variant="ghost" size="sm" className="h-7 w-7 p-0">
              <Edit2 className="w-3.5 h-3.5" />
            </Button>
            <Button variant="ghost" size="sm" className="h-7 w-7 p-0">
              <Trash2 className="w-3.5 h-3.5" />
            </Button>
          </div>
        </div>
      </CardHeader>

      <CardContent className="space-y-2 px-4 pb-3">
        {/* User Story Format - 紧凑版 */}
        <div className="space-y-1 p-2 bg-blue-50 dark:bg-blue-950/20 rounded-md text-xs">
          <div className="flex items-start gap-1.5">
            <Users className="w-3 h-3 text-blue-500 mt-0.5 flex-shrink-0" />
            <div>
              <span className="font-medium text-blue-700 dark:text-blue-400">As a </span>
              <span className="text-blue-600 dark:text-blue-300">{story.role}</span>
            </div>
          </div>
          <div className="flex items-start gap-1.5">
            <Target className="w-3 h-3 text-blue-500 mt-0.5 flex-shrink-0" />
            <div>
              <span className="font-medium text-blue-700 dark:text-blue-400">I want </span>
              <span className="text-blue-600 dark:text-blue-300">{story.feature}</span>
            </div>
          </div>
          <div className="flex items-start gap-1.5">
            <Lightbulb className="w-3 h-3 text-blue-500 mt-0.5 flex-shrink-0" />
            <div>
              <span className="font-medium text-blue-700 dark:text-blue-400">So that </span>
              <span className="text-blue-600 dark:text-blue-300">{story.benefit}</span>
            </div>
          </div>
        </div>

        {/* Acceptance Criteria - 紧凑版 */}
        {story.acceptanceCriteria.length > 0 && (
          <div>
            <p className="text-xs font-medium mb-1">验收标准</p>
            <ul className="space-y-0.5">
              {story.acceptanceCriteria.slice(0, 3).map((criteria, idx) => (
                <li key={idx} className="flex items-start gap-1.5 text-xs">
                  <CheckCircle2 className="w-3 h-3 text-green-500 mt-0.5 flex-shrink-0" />
                  <span className="text-muted-foreground line-clamp-1">{criteria}</span>
                </li>
              ))}
              {story.acceptanceCriteria.length > 3 && (
                <li className="text-xs text-muted-foreground pl-5">
                  +{story.acceptanceCriteria.length - 3} 更多...
                </li>
              )}
            </ul>
          </div>
        )}

        {/* Meta Info - 紧凑版 */}
        <div className="flex items-center gap-3 text-xs text-muted-foreground pt-1.5 border-t">
          {story.storyPoints && (
            <div className="flex items-center gap-1">
              <Target className="w-3 h-3" />
              <span>{story.storyPoints} 点</span>
            </div>
          )}
          {story.featureModule && <div>模块: {story.featureModule}</div>}
          {story.labels.length > 0 && (
            <div className="flex gap-1 flex-wrap">
              {story.labels.slice(0, 2).map((label, idx) => (
                <Badge key={idx} variant="secondary" className="text-[10px] px-1.5 py-0">
                  {label}
                </Badge>
              ))}
              {story.labels.length > 2 && (
                <Badge variant="secondary" className="text-[10px] px-1.5 py-0">
                  +{story.labels.length - 2}
                </Badge>
              )}
            </div>
          )}
        </div>
      </CardContent>
    </Card>
  )
}
