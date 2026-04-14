import { useState, useEffect } from 'react'
import React from 'react'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import { ScrollArea } from '@/components/ui/scroll-area'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Checkbox } from '@/components/ui/checkbox'
import { Input } from '@/components/ui/input'
import { Textarea } from '@/components/ui/textarea'
import {
  Users,
  Sparkles,
  Loader2,
  Search,
  X,
  CheckCircle2,
  AlertCircle,
  Calendar,
  Lightbulb,
} from 'lucide-react'
import type { Sprint, UserStory } from '@/types'
import { useAIConfigStore } from '@/stores/aiConfigStore'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import type { UnlistenFn } from '@tauri-apps/api/event'

interface AIAssignStoriesDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  sprint: Sprint
  unassignedStories: UserStory[]
  allStories?: UserStory[] // 所有故事，用于计算已分配故事的统计
  onAssign: (sprintId: string, storyIds: string[]) => Promise<void>
}

interface AIRecommendation {
  storyId: string // 故事的UUID
  storyNumber: string // 故事编号（如 US-001）
  reason: string
  confidence: number // 0-100
}

export function AIAssignStoriesDialog({
  open,
  onOpenChange,
  sprint,
  unassignedStories,
  allStories = [],
  onAssign,
}: AIAssignStoriesDialogProps) {
  const [isAnalyzing, setIsAnalyzing] = useState(false)
  const [recommendations, setRecommendations] = useState<AIRecommendation[]>([])
  const [selectedStoryIds, setSelectedStoryIds] = useState<string[]>([])
  const [error, setError] = useState<string | null>(null)
  const [filterKeyword, setFilterKeyword] = useState('')
  const [showOnlyRecommended, setShowOnlyRecommended] = useState(false)
  const [aiThinkingProcess, setAiThinkingProcess] = useState<string>('') // AI思考过程
  const [showThinkingProcess, setShowThinkingProcess] = useState<boolean>(false) // 控制AI分析过程区域显示
  const [userSuggestions, setUserSuggestions] = useState<string>('') // 用户分配建议

  const activeConfig = useAIConfigStore(state => state.getActiveConfig())

  // 重置状态
  useEffect(() => {
    if (open) {
      setRecommendations([])
      setSelectedStoryIds([])
      setError(null)
      setFilterKeyword('')
      setShowOnlyRecommended(false)
      setAiThinkingProcess('')
      setShowThinkingProcess(false)
      setUserSuggestions('')
    }
  }, [open])

  // 调试：监听error状态变化
  useEffect(() => {
    console.log('[AIAssignStoriesDialog] Error state changed:', error)
  }, [error])

  // 调用AI分析
  const handleAIAnalysis = async () => {
    if (!activeConfig) {
      setError('请先配置AI服务')
      return
    }

    if (unassignedStories.length === 0) {
      setError('没有未分配的用户故事')
      return
    }

    setIsAnalyzing(true)
    setError(null)
    setRecommendations([])
    setAiThinkingProcess('')
    setShowThinkingProcess(true) // 开始分析时显示思考过程区域

    try {
      // CodeFree 模式特殊处理
      if (activeConfig.provider === 'codefree') {
        await handleCodeFreeAnalysis()
      } else {
        // 其他 AI 提供商的标准处理流程
        await handleStandardAIAnalysis()
      }
    } catch (err) {
      console.error('[AIAssignStoriesDialog] AI analysis error:', err)
      // 直接显示原始错误信息
      setError(err instanceof Error ? err.message : String(err))
      setIsAnalyzing(false)
    }
  }

  // CodeFree 模式的AI分析处理
  const handleCodeFreeAnalysis = async () => {
    // 获取当前项目ID
    const { useProjectStore } = await import('@/stores/projectStore')
    const currentProjectId = useProjectStore.getState().currentProjectId

    if (!currentProjectId) {
      throw new Error('未找到项目ID,CodeFree模式需要项目上下文')
    }

    console.log('[AIAssignStoriesDialog] 🎯 Starting CodeFree analysis mode')

    let accumulatedContent = ''
    const unlistenFns: UnlistenFn[] = []

    // 监听 Sprint 分配流式数据 - 实时更新思考过程
    const unlistenChunk = await listen<{ content: string }>(
      'sprint-assignment-stream-chunk',
      event => {
        accumulatedContent += event.payload.content
        // 实时更新AI思考过程(去除Markdown代码块标记)
        const displayContent = accumulatedContent
          .replace(/```json\n?/g, '')
          .replace(/```\n?/g, '')
          .trim()
        setAiThinkingProcess(displayContent)
      }
    )
    unlistenFns.push(unlistenChunk)

    const unlistenComplete = await listen<{ session_id: string; content: string }>(
      'sprint-assignment-stream-complete',
      event => {
        console.log('[AIAssignStoriesDialog] ✅ CodeFree analysis complete')
        const { content } = event.payload

        // 解析AI返回的Markdown表格
        try {
          const recommendations = parseMarkdownTable(content)

          if (recommendations && recommendations.length > 0) {
            setRecommendations(recommendations)
            // 通过storyNumber匹配故事ID，然后自动选中所有推荐的故事
            const recommendedStoryIds = unassignedStories
              .filter(story => recommendations.some(rec => rec.storyNumber === story.storyNumber))
              .map(story => story.id)

            console.log('[AIAssignStoriesDialog] Auto-selecting stories:', recommendedStoryIds)
            setSelectedStoryIds(recommendedStoryIds)
          } else {
            setError('AI返回的数据格式不正确')
          }
        } catch (parseError) {
          console.error('[AIAssignStoriesDialog] Failed to parse AI response:', parseError)
          console.error('[AIAssignStoriesDialog] Raw content:', content)
          setError('解析AI推荐结果失败,请重试')
        }

        // 清理监听器
        unlistenFns.forEach(unlisten => unlisten())
        setIsAnalyzing(false)
      }
    )
    unlistenFns.push(unlistenComplete)

    const unlistenError = await listen<{ session_id: string; error: string }>(
      'sprint-assignment-stream-error',
      event => {
        console.error('[AIAssignStoriesDialog] AI stream error:', event.payload)
        // 直接显示原始错误信息
        setError(event.payload.error || 'AI分析失败')
        unlistenFns.forEach(unlisten => unlisten())
        setIsAnalyzing(false)
      }
    )
    unlistenFns.push(unlistenError)

    // 启动专门的Sprint分配流式请求（后端会自动写入AGENTS.md等文件）

    // 确保所有用户故事都有必需的字段，并进行字段名转换（驼峰 -> 蛇形）
    const validatedStories = unassignedStories.map(story => ({
      id: story.id,
      story_number: story.storyNumber || `US-${story.id.slice(-3)}`,
      title: story.title,
      role: story.role,
      feature: story.feature,
      benefit: story.benefit,
      description: story.description || '',
      acceptance_criteria: story.acceptanceCriteria || [],
      priority: story.priority,
      status: story.status,
      story_points: story.storyPoints,
      dependencies: story.dependencies || null,
      feature_module: story.featureModule || null,
      sprint_id: story.sprintId || null,
      labels: story.labels || [],
      created_at: story.createdAt,
      updated_at: story.updatedAt,
    }))

    await invoke('assign_stories_to_sprint_streaming', {
      request: {
        sprint: {
          id: sprint.id,
          name: sprint.name,
          goal: sprint.goal,
          start_date: sprint.startDate,
          end_date: sprint.endDate,
          total_story_points: sprint.totalStoryPoints,
          completed_story_points: sprint.completedStoryPoints,
        },
        stories: validatedStories,
        provider: activeConfig!.provider,
        model: activeConfig!.model,
        api_key: activeConfig!.apiKey || '',
        project_id: currentProjectId,
        user_suggestions: userSuggestions.trim() || undefined,
      },
    })
  }

  // 标准AI分析处理（原有逻辑）
  const handleStandardAIAnalysis = async () => {
    // 构建Prompt（包含用户建议）
    const prompt = buildAnalysisPrompt(sprint, unassignedStories, userSuggestions)

    // 调用AI流式接口
    const messages = [
      {
        role: 'system' as const,
        content:
          '你是一个专业的敏捷开发教练，擅长Sprint规划和用户故事优先级排序。请分析给定的Sprint信息和未分配的用户故事，推荐最适合分配到该Sprint的故事。',
      },
      {
        role: 'user' as const,
        content: prompt,
      },
    ]

    let accumulatedContent = ''
    const unlistenFns: UnlistenFn[] = []

    // 监听流式数据 - 实时更新思考过程
    const unlistenChunk = await listen<{ content: string }>('ai-stream-chunk', event => {
      accumulatedContent += event.payload.content
      // 实时更新AI思考过程（去除JSON代码块标记）
      const displayContent = accumulatedContent
        .replace(/```json\n?/g, '')
        .replace(/```\n?/g, '')
        .trim()
      setAiThinkingProcess(displayContent)
    })
    unlistenFns.push(unlistenChunk)

    const unlistenComplete = await listen('ai-stream-complete', () => {
      // 解析AI返回的Markdown表格
      try {
        const recommendations = parseMarkdownTable(accumulatedContent)

        if (recommendations && recommendations.length > 0) {
          setRecommendations(recommendations)
          // 通过storyNumber匹配故事ID，然后自动选中所有推荐的故事
          const recommendedStoryIds = unassignedStories
            .filter(story => recommendations.some(rec => rec.storyNumber === story.storyNumber))
            .map(story => story.id)

          console.log('[AIAssignStoriesDialog] Auto-selecting stories:', recommendedStoryIds)
          setSelectedStoryIds(recommendedStoryIds)
        } else {
          setError('AI返回的数据格式不正确')
        }
      } catch (parseError) {
        console.error('[AIAssignStoriesDialog] Failed to parse AI response:', parseError)
        console.error('[AIAssignStoriesDialog] Raw content:', accumulatedContent)
        setError('解析AI推荐结果失败，请重试')
      }

      // 清理监听器
      unlistenFns.forEach(unlisten => unlisten())
      setIsAnalyzing(false)
    })
    unlistenFns.push(unlistenComplete)

    const unlistenError = await listen<{ error: string }>('ai-stream-error', event => {
      console.error('[AIAssignStoriesDialog] AI stream error:', event.payload)
      // 直接显示原始错误信息
      setError(event.payload.error || 'AI分析失败')
      unlistenFns.forEach(unlisten => unlisten())
      setIsAnalyzing(false)
    })
    unlistenFns.push(unlistenError)

    // 启动流式请求
    await invoke('stream_chat', {
      request: {
        provider: activeConfig!.provider,
        model: activeConfig!.model,
        api_key: activeConfig!.apiKey,
        messages,
        temperature: 0.7,
        max_tokens: 4000,
      },
    })
  }

  // 格式化错误信息，使其更易读（已废弃，保留供参考）
  const _formatErrorMessage = (error: string): string => {
    // 如果已经是友好的错误信息，直接返回
    if (error.startsWith('请先配置') || error.startsWith('没有未分配')) {
      return error
    }

    // 尝试提取关键错误信息
    let formattedError = error

    // CodeFree CLI 认证错误特殊处理
    if (error.includes('CodeFree CLI 错误') || error.includes('.codefree-cli')) {
      if (error.includes('Auth method') || error.includes('environment variables')) {
        return `❌ CodeFree CLI 未配置认证信息

请在以下位置之一配置认证：
1. 配置文件：C:\\Users\\37844\\.codefree-cli\\settings.json
2. 环境变量：设置 CodeFree-oauth

详细错误：${error}`
      }
      return `❌ CodeFree CLI 执行失败\n\n${error}`
    }

    // 常见的API错误类型映射
    const errorMappings: Record<string, string> = {
      // 认证错误
      invalid_api_key: '❌ API密钥无效，请检查配置',
      unauthorized: '❌ 认证失败，请检查API密钥是否正确',
      authentication: '❌ 认证失败，请检查API密钥是否正确',

      // 配额错误
      quota_exceeded: '⚠️ API配额已用尽，请检查账户余额',
      rate_limit: '⚠️ API调用频率超限，请稍后重试',
      insufficient_quota: '⚠️ 账户余额不足，请充值',

      // 模型错误
      model_not_found: '❌ 模型不存在，请检查模型名称',
      invalid_model: '❌ 无效的模型名称',

      // 网络错误
      network_error: '🌐 网络连接失败，请检查网络设置',
      timeout: '⏱️ 请求超时，请检查网络连接后重试',
      connection_refused: '🌐 连接被拒绝，请检查网络或代理设置',

      // 内容错误
      content_filter: '⚠️ 内容被过滤，请调整输入内容',
      inappropriate_content: '⚠️ 内容不符合规范',

      // 服务器错误
      server_error: '🔧 服务器错误，请稍后重试',
      internal_error: '🔧 内部错误，请联系技术支持',
    }

    // 尝试匹配已知错误类型
    const lowerError = error.toLowerCase()
    for (const [key, message] of Object.entries(errorMappings)) {
      if (lowerError.includes(key)) {
        formattedError = `${message}\n\n原始错误：${error}`
        return formattedError
      }
    }

    // 如果是HTTP错误，提取状态码
    const httpStatusMatch = error.match(/HTTP (\d{3})/)
    if (httpStatusMatch) {
      const statusCode = httpStatusMatch[1]
      const statusMessages: Record<string, string> = {
        '400': '请求参数错误',
        '401': '认证失败，请检查API密钥',
        '403': '权限不足',
        '404': '资源不存在',
        '429': '请求频率超限',
        '500': '服务器内部错误',
        '502': '网关错误',
        '503': '服务暂时不可用',
        '504': '网关超时',
      }
      const statusMessage = statusMessages[statusCode] || '未知错误'
      formattedError = `❌ HTTP ${statusCode}: ${statusMessage}\n\n详细信息：${error}`
      return formattedError
    }

    // 默认返回原始错误，但添加前缀
    return `❌ AI分析失败\n\n${error}`
  }

  // 解析Markdown表格为AIRecommendation数组
  const parseMarkdownTable = (markdown: string): AIRecommendation[] | null => {
    try {
      console.log('[AIAssignStoriesDialog] Parsing markdown table:', markdown.substring(0, 200))

      // 提取表格内容（去除表头和分隔线）
      const lines = markdown.split('\n')
      const tableRows: string[] = []
      let inTable = false

      for (const line of lines) {
        const trimmedLine = line.trim()

        // 检测表格开始（以 | 开头的行）
        if (trimmedLine.startsWith('|') && !inTable) {
          inTable = true
          continue // 跳过表头
        }

        // 跳过分隔线（包含 --- 的行）
        if (inTable && trimmedLine.includes('---')) {
          continue
        }

        // 收集表格数据行
        if (inTable && trimmedLine.startsWith('|')) {
          tableRows.push(trimmedLine)
        }

        // 遇到空行或新段落，结束表格
        if (inTable && trimmedLine === '') {
          break
        }
      }

      if (tableRows.length === 0) {
        console.warn('[AIAssignStoriesDialog] No table rows found')
        return null
      }

      console.log(`[AIAssignStoriesDialog] Found ${tableRows.length} table rows`)

      // 解析每一行
      const recommendations: AIRecommendation[] = []

      for (const row of tableRows) {
        // 移除首尾的 | 并分割
        const cells = row
          .replace(/^\|/, '')
          .replace(/\|$/, '')
          .split('|')
          .map(cell => cell.trim())

        if (cells.length < 3) {
          console.warn('[AIAssignStoriesDialog] Row has insufficient cells:', cells)
          continue
        }

        // 提取故事编号（格式如 US-001）
        const storyNumberCell = cells[0] || ''
        const reasonCell = cells[1] || ''
        const confidenceCell = cells[2] || ''

        // 尝试从单元格中提取故事编号（格式如 US-001）
        const storyNumberMatch = storyNumberCell.match(/(US-\d+)/i)

        if (!storyNumberMatch) {
          console.warn('[AIAssignStoriesDialog] Could not extract story number from:', cells)
          continue
        }

        const storyNumber = storyNumberMatch[1].toUpperCase()

        // 提取置信度（数字）
        const confidenceMatch = confidenceCell.match(/(\d+)/)
        const confidence = confidenceMatch ? parseInt(confidenceMatch[1], 10) : 50

        // 使用推荐理由，如果没有则使用默认文本
        const reason = reasonCell || 'AI推荐'

        recommendations.push({
          storyId: '', // 暂时为空，后续通过storyNumber匹配
          storyNumber,
          reason,
          confidence: Math.min(100, Math.max(0, confidence)), // 确保在0-100范围内
        })
      }

      console.log(`[AIAssignStoriesDialog] Parsed ${recommendations.length} recommendations`)
      return recommendations.length > 0 ? recommendations : null
    } catch (error) {
      console.error('[AIAssignStoriesDialog] Failed to parse markdown table:', error)
      return null
    }
  }

  // 构建分析Prompt
  const buildAnalysisPrompt = (
    sprint: Sprint,
    stories: UserStory[],
    suggestions?: string
  ): string => {
    const sprintInfo = `
Sprint信息：
- 名称：${sprint.name}
- 目标：${sprint.goal || '未设置'}
- 时间范围：${sprint.startDate} 至 ${sprint.endDate}
- 当前容量：${sprint.totalStoryPoints || 0} 故事点
- 已完成：${sprint.completedStoryPoints || 0} 故事点
`

    const storiesInfo = stories
      .map(
        story => `
故事 ${story.storyNumber}：
- 标题：${story.title}
- 角色：${story.role}
- 功能：${story.feature}
- 价值：${story.benefit}
- 优先级：${story.priority}
- 故事点：${story.storyPoints || '未估算'}
- 标签：${story.labels.join(', ') || '无'}
- 依赖：${story.dependencies?.join(', ') || '无'}
`
      )
      .join('\n')

    const userSuggestionsSection =
      suggestions && suggestions.trim()
        ? `

用户的分配建议和特殊要求：
${suggestions.trim()}

请特别注意并优先考虑上述用户建议，在推荐时充分考虑这些约束条件和要求。
`
        : ''

    return `
${sprintInfo}

未分配的用户故事列表：
${storiesInfo}
${userSuggestionsSection}

请分析以上Sprint信息和未分配的用户故事，推荐最适合分配到该Sprint的故事。

考虑因素：
1. 优先级（P0 > P1 > P2 > P3）
3. 故事之间的依赖关系
4. 业务价值和Sprint目标的匹配度
5. 技术实现的可行性
6. 用户的特殊建议和约束（如果有）

请以Markdown表格格式返回推荐结果，格式如下：

| 故事ID | 推荐理由 | 置信度 |
|--------|----------|--------|
| US-001 | 这是P0优先级故事，与Sprint目标高度匹配 | 95 |
| US-003 | 虽然优先级为P1，但故事点较小，可以快速完成 | 85 |

注意：
- **故事ID**列必须使用故事编号格式（如 US-001、US-002）
- 只返回Markdown表格，不要有其他内容
- 推荐理由要具体、有说服力，如果用户提供了建议，需要说明如何遵循了这些建议
- 置信度反映你对推荐的确定程度（0-100的整数）
- 优先推荐高优先级、高价值、低依赖的故事
- 严格遵循用户提出的任何特殊要求或约束
`
  }

  // 切换故事选择
  const toggleStorySelection = (storyId: string) => {
    setSelectedStoryIds(prev =>
      prev.includes(storyId) ? prev.filter(id => id !== storyId) : [...prev, storyId]
    )
  }

  // 全选推荐
  const selectAllRecommended = () => {
    // 通过storyNumber匹配故事ID
    const recommendedStoryIds = unassignedStories
      .filter(story => recommendations.some(rec => rec.storyNumber === story.storyNumber))
      .map(story => story.id)

    setSelectedStoryIds(recommendedStoryIds)
  }

  // 清空选择
  const clearSelection = () => {
    setSelectedStoryIds([])
  }

  // 处理保存
  const handleSave = async () => {
    if (selectedStoryIds.length === 0) {
      setError('请至少选择一个用户故事')
      return
    }

    try {
      await onAssign(sprint.id, selectedStoryIds)
      onOpenChange(false)
    } catch (err) {
      console.error('[AIAssignStoriesDialog] Failed to assign stories:', err)
      setError('分配失败，请重试')
    }
  }

  // 筛选后的故事列表
  const filteredStories = unassignedStories.filter(story => {
    // 关键词筛选
    if (filterKeyword.trim()) {
      const keyword = filterKeyword.toLowerCase()
      const matchesKeyword =
        story.title.toLowerCase().includes(keyword) ||
        story.storyNumber.toLowerCase().includes(keyword) ||
        story.role.toLowerCase().includes(keyword)

      if (!matchesKeyword) return false
    }

    // 只显示推荐的故事
    if (showOnlyRecommended) {
      const recommendedIds = recommendations.map(r => r.storyId)
      if (!recommendedIds.includes(story.id)) return false
    }

    return true
  })

  // 获取故事的推荐信息
  const getRecommendation = (storyId: string) => {
    // 通过storyId找到对应的story,然后通过storyNumber匹配recommendation
    const story = unassignedStories.find(s => s.id === storyId)
    if (!story) return undefined

    return recommendations.find(r => r.storyNumber === story.storyNumber)
  }

  // 计算选中的故事点
  const selectedStoryPoints = unassignedStories
    .filter(story => selectedStoryIds.includes(story.id))
    .reduce((sum, story) => sum + (story.storyPoints || 0), 0)

  // 计算已分配的故事统计
  const assignedStoriesStats = React.useMemo(() => {
    // 优先使用传入的allStories，否则从unassignedStories推断
    const stories = allStories.length > 0 ? allStories : unassignedStories

    // 找出已分配到当前Sprint的故事
    const assignedStories = stories.filter(story => story.sprintId === sprint.id)

    const totalPoints = assignedStories.reduce((sum, story) => sum + (story.storyPoints || 0), 0)
    const completedPoints = assignedStories
      .filter(story => story.status === 'completed')
      .reduce((sum, story) => sum + (story.storyPoints || 0), 0)

    return {
      totalPoints,
      completedPoints,
      totalCount: assignedStories.length,
      completedCount: assignedStories.filter(story => story.status === 'completed').length,
    }
  }, [allStories, unassignedStories, sprint.id])

  // 格式化AI思考过程，增强可读性
  const formatThinkingProcess = (text: string) => {
    // 将Markdown标题转换为更醒目的格式
    const formatted = text
      // 突出显示关键分析步骤
      .replace(/^(分析|考虑|评估|推荐|总结).*/gm, match => `\n🔍 ${match}`)
      // 突出显示优先级相关内容
      .replace(/(P[0-3]|优先级)/g, '⚡ $1')
      // 突出显示故事点
      .replace(/(\d+)\s*点/g, '📊 $1 点')
      // 突出显示Sprint相关信息
      .replace(/(Sprint|容量|目标)/g, '🎯 $1')

    return formatted
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-6xl h-[85vh] flex flex-col p-0">
        {/* 固定头部 - 包含Sprint信息 */}
        <DialogHeader className="flex-shrink-0 px-6 pt-5 pb-3 border-b">
          <div className="flex items-start justify-between gap-4">
            <div className="flex items-center gap-3 flex-1">
              <div className="w-10 h-10 rounded-lg bg-primary/10 flex items-center justify-center flex-shrink-0">
                <Sparkles className="w-5 h-5 text-primary" />
              </div>
              <div className="flex-1 min-w-0">
                <DialogTitle className="text-xl mb-1">AI智能分配用户故事</DialogTitle>
                <DialogDescription className="text-xs">
                  让AI帮你分析并推荐最适合当前Sprint的用户故事
                </DialogDescription>
              </div>
            </div>

            {/* Sprint信息 - 详细统计展示 */}
            <div className="flex-shrink-0 bg-muted/30 rounded-lg px-4 py-2.5 border">
              <div className="flex items-center gap-5 text-xs">
                <div>
                  <div className="text-[9px] text-muted-foreground font-medium mb-0.5">Sprint</div>
                  <div className="font-semibold text-sm">{sprint.name}</div>
                </div>
                <div className="w-px h-8 bg-border" />
                <div>
                  <div className="text-[9px] text-muted-foreground font-medium mb-0.5 flex items-center gap-1">
                    <Calendar className="w-2.5 h-2.5" />
                    时间
                  </div>
                  <div className="text-[10px] whitespace-nowrap">
                    {new Date(sprint.startDate).toLocaleDateString('zh-CN', {
                      month: '2-digit',
                      day: '2-digit',
                    })}{' '}
                    -{' '}
                    {new Date(sprint.endDate).toLocaleDateString('zh-CN', {
                      month: '2-digit',
                      day: '2-digit',
                    })}
                  </div>
                </div>
                <div className="w-px h-8 bg-border" />
                <div>
                  <div className="text-[9px] text-muted-foreground font-medium mb-0.5">故事数</div>
                  <div className="flex items-baseline gap-1">
                    <span className="text-sm font-bold text-green-600 dark:text-green-400">
                      {assignedStoriesStats.completedCount}
                    </span>
                    <span className="text-[10px] text-muted-foreground">/</span>
                    <span className="text-sm font-bold text-blue-600 dark:text-blue-400">
                      {assignedStoriesStats.totalCount}
                    </span>
                  </div>
                </div>
                <div className="w-px h-8 bg-border" />
                <div>
                  <div className="text-[9px] text-muted-foreground font-medium mb-0.5">故事点</div>
                  <div className="flex items-baseline gap-1">
                    <span className="text-sm font-bold text-green-600 dark:text-green-400">
                      {assignedStoriesStats.completedPoints}
                    </span>
                    <span className="text-[10px] text-muted-foreground">/</span>
                    <span className="text-sm font-bold text-blue-600 dark:text-blue-400">
                      {assignedStoriesStats.totalPoints}
                    </span>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </DialogHeader>

        {/* 滚动主体区域 */}
        <div className="flex-1 overflow-hidden">
          <div className="h-full flex gap-4 p-6">
            {/* 左侧：操作区 - 移除Sprint信息 */}
            <div className="w-1/3 space-y-2 flex flex-col overflow-y-auto pr-2">
              {/* 用户分配建议输入框 */}
              <Card className="flex-shrink-0">
                <CardHeader className="pb-1 px-3 pt-3">
                  <CardTitle className="text-xs flex items-center gap-1.5">
                    <Lightbulb className="w-3.5 h-3.5 text-yellow-500" />
                    分配建议（可选）
                  </CardTitle>
                </CardHeader>
                <CardContent className="pt-0 px-3 pb-3">
                  <Textarea
                    placeholder="例如：优先处理支付功能相关的故事，本周专注于前端界面优化..."
                    value={userSuggestions}
                    onChange={e => setUserSuggestions(e.target.value)}
                    className="min-h-[60px] text-xs resize-none"
                    disabled={isAnalyzing}
                  />
                  <p className="text-[9px] text-muted-foreground mt-1.5">
                    💡 提供具体建议可帮助AI给出更精准的推荐
                  </p>
                </CardContent>
              </Card>

              {/* 操作按钮 - 紧凑版 */}
              <Card className="flex-shrink-0">
                <CardContent className="pt-4 space-y-2 px-4 pb-4">
                  {/* 错误提示 - 右上角关闭图标 */}
                  {error && (
                    <div className="relative rounded-md border border-destructive/40 bg-destructive/10 p-2.5 mb-2">
                      {/* 关闭按钮 - 右上角 */}
                      <button
                        onClick={() => setError(null)}
                        className="absolute top-1.5 right-1.5 text-destructive/60 hover:text-destructive transition-colors"
                        title="关闭错误提示"
                      >
                        <X className="w-3.5 h-3.5" />
                      </button>

                      <div className="flex items-start gap-2 pr-6">
                        <AlertCircle className="w-3.5 h-3.5 text-destructive mt-0.5 flex-shrink-0" />
                        <div className="flex-1 min-w-0">
                          <ScrollArea className="h-[60px] w-full">
                            <div className="text-[11px] leading-relaxed whitespace-pre-wrap text-destructive pr-1 break-all">
                              {error}
                            </div>
                          </ScrollArea>
                        </div>
                      </div>
                    </div>
                  )}

                  <Button
                    onClick={handleAIAnalysis}
                    disabled={isAnalyzing || unassignedStories.length === 0}
                    className="w-full"
                    size="lg"
                  >
                    {isAnalyzing ? (
                      <>
                        <Loader2 className="w-4 h-4 mr-2 animate-spin" />
                        AI分析中...
                      </>
                    ) : (
                      <>
                        <Sparkles className="w-4 h-4 mr-2" />
                        开始AI分析
                      </>
                    )}
                  </Button>

                  {recommendations.length > 0 && (
                    <div className="space-y-2">
                      <div className="flex items-center justify-between text-xs text-muted-foreground">
                        <span>已选择 {selectedStoryIds.length} 个故事</span>
                        <span className="font-medium text-primary">
                          共 {selectedStoryPoints} 点
                        </span>
                      </div>
                    </div>
                  )}
                </CardContent>
              </Card>

              {/* AI思考过程实时显示 */}
              {showThinkingProcess && aiThinkingProcess && (
                <Card className="flex-shrink-0 border-primary/30 bg-primary/5">
                  <CardHeader className="pb-2">
                    <div className="flex items-center justify-between">
                      <CardTitle className="text-sm flex items-center gap-2 text-primary">
                        <Sparkles className={`w-4 h-4 ${isAnalyzing ? 'animate-pulse' : ''}`} />
                        AI分析过程
                      </CardTitle>
                      <button
                        onClick={() => setShowThinkingProcess(false)}
                        className="text-muted-foreground hover:text-foreground transition-colors"
                        title="关闭分析过程"
                      >
                        <X className="w-4 h-4" />
                      </button>
                    </div>
                  </CardHeader>
                  <CardContent className="pt-0">
                    <ScrollArea className="h-[200px] w-full">
                      <div className="text-xs leading-relaxed whitespace-pre-wrap text-muted-foreground pr-2 font-mono">
                        {formatThinkingProcess(aiThinkingProcess)}
                        {isAnalyzing && (
                          <span className="inline-block w-2 h-4 ml-1 bg-primary animate-pulse" />
                        )}
                      </div>
                    </ScrollArea>
                  </CardContent>
                </Card>
              )}

              {/* 错误提示 - 已禁用，使用操作按钮Card内的紧凑版本 */}
              {false && error && (
                <>
                  {console.log('[AIAssignStoriesDialog] Rendering error card with message:', error)}
                  <Card
                    className="flex-shrink-0 border-destructive bg-destructive/5"
                    style={{ zIndex: 9999, position: 'relative' }}
                  >
                    <CardHeader className="pb-2">
                      <CardTitle className="text-sm flex items-center gap-2 text-destructive">
                        <AlertCircle className="w-4 h-4" />
                        AI分析出错
                      </CardTitle>
                    </CardHeader>
                    <CardContent className="pt-0">
                      <ScrollArea className="h-[150px] w-full">
                        <div className="text-xs leading-relaxed whitespace-pre-wrap text-destructive pr-2">
                          {error}
                        </div>
                      </ScrollArea>
                      <div className="mt-3 flex gap-2">
                        <Button
                          variant="outline"
                          size="sm"
                          onClick={() => setError(null)}
                          className="h-7 text-xs"
                        >
                          关闭
                        </Button>
                        <Button
                          variant="destructive"
                          size="sm"
                          onClick={handleAIAnalysis}
                          disabled={isAnalyzing}
                          className="h-7 text-xs"
                        >
                          {isAnalyzing ? (
                            <>
                              <Loader2 className="w-3 h-3 mr-1 animate-spin" />
                              重试中...
                            </>
                          ) : (
                            <>
                              <Sparkles className="w-3 h-3 mr-1" />
                              重新分析
                            </>
                          )}
                        </Button>
                      </div>
                    </CardContent>
                  </Card>
                </>
              )}
            </div>

            {/* 右侧：故事列表 */}
            <div className="flex-1 flex flex-col min-h-0">
              <Card className="flex-1 flex flex-col min-h-0">
                <CardHeader className="flex-shrink-0 pb-3">
                  <div className="flex items-center justify-between">
                    <CardTitle className="text-base flex items-center gap-2">
                      <Users className="w-4 h-4" />
                      待分配用户故事
                      {unassignedStories.length > 0 && (
                        <Badge variant="secondary" className="ml-2">
                          {unassignedStories.length}
                        </Badge>
                      )}
                    </CardTitle>

                    {/* 操作按钮 */}
                    {recommendations.length > 0 && (
                      <div className="flex gap-2">
                        <Button
                          variant="outline"
                          size="sm"
                          onClick={selectAllRecommended}
                          className="h-7 text-xs"
                        >
                          全选推荐
                        </Button>
                        <Button
                          variant="ghost"
                          size="sm"
                          onClick={clearSelection}
                          className="h-7 text-xs"
                        >
                          清空
                        </Button>
                      </div>
                    )}
                  </div>

                  {/* 搜索和筛选工具栏 */}
                  <div className="flex gap-2 mt-3">
                    <div className="relative flex-1">
                      <Search className="absolute left-2 top-1/2 -translate-y-1/2 w-3 h-3 text-muted-foreground" />
                      <Input
                        type="text"
                        placeholder="搜索故事编号、标题、角色..."
                        value={filterKeyword}
                        onChange={e => setFilterKeyword(e.target.value)}
                        className="pl-7 h-8 text-xs"
                      />
                      {filterKeyword && (
                        <button
                          onClick={() => setFilterKeyword('')}
                          className="absolute right-2 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground"
                        >
                          <X className="w-3 h-3" />
                        </button>
                      )}
                    </div>
                    {recommendations.length > 0 && (
                      <Button
                        variant={showOnlyRecommended ? 'default' : 'outline'}
                        size="sm"
                        onClick={() => setShowOnlyRecommended(!showOnlyRecommended)}
                        className="h-8 text-xs"
                      >
                        {showOnlyRecommended ? '显示全部' : '仅推荐'}
                      </Button>
                    )}
                  </div>
                </CardHeader>

                {/* 滚动内容区 */}
                <CardContent className="flex-1 overflow-hidden p-0">
                  <div className="h-full w-full pr-2">
                    <ScrollArea className="h-full w-full" type="always">
                      <div className="min-h-full">
                        {filteredStories.length > 0 ? (
                          <div className="space-y-2 p-4 pt-0">
                            {filteredStories.map(story => {
                              const recommendation = getRecommendation(story.id)
                              const isSelected = selectedStoryIds.includes(story.id)

                              return (
                                <div
                                  key={story.id}
                                  className={`p-3 rounded-lg border transition-all ${
                                    isSelected
                                      ? 'bg-primary/5 border-primary/30'
                                      : recommendation
                                        ? 'bg-green-50/50 dark:bg-green-950/20 border-green-200 dark:border-green-800'
                                        : 'hover:bg-muted/50 border-border'
                                  }`}
                                >
                                  <div className="flex items-start gap-3">
                                    <Checkbox
                                      id={`story-${story.id}`}
                                      checked={isSelected}
                                      onChange={() => toggleStorySelection(story.id)}
                                      className="mt-1"
                                    />
                                    <div className="flex-1 space-y-2">
                                      {/* 标题和编号 */}
                                      <div
                                        className="cursor-pointer"
                                        onClick={() => toggleStorySelection(story.id)}
                                      >
                                        <div className="text-sm font-medium leading-none mb-1">
                                          {story.storyNumber} - {story.title}
                                        </div>
                                        <div className="flex items-center gap-2 text-xs text-muted-foreground">
                                          <span>{story.role}</span>
                                          {story.storyPoints && (
                                            <Badge
                                              variant="outline"
                                              className="text-[10px] h-4 px-1"
                                            >
                                              {story.storyPoints} 点
                                            </Badge>
                                          )}
                                          <Badge
                                            className={`${
                                              story.priority === 'P0'
                                                ? 'bg-red-100 text-red-700 dark:bg-red-900/30 dark:text-red-400'
                                                : story.priority === 'P1'
                                                  ? 'bg-orange-100 text-orange-700 dark:bg-orange-900/30 dark:text-orange-400'
                                                  : story.priority === 'P2'
                                                    ? 'bg-yellow-100 text-yellow-700 dark:bg-yellow-900/30 dark:text-yellow-400'
                                                    : 'bg-gray-100 text-gray-700 dark:bg-gray-800 dark:text-gray-400'
                                            } text-[10px] h-4 px-1`}
                                          >
                                            {story.priority}
                                          </Badge>
                                        </div>
                                      </div>

                                      {/* AI推荐理由 */}
                                      {recommendation && (
                                        <div className="text-xs space-y-1">
                                          <div className="flex items-center gap-1.5 text-green-700 dark:text-green-400 font-medium">
                                            <CheckCircle2 className="w-3 h-3" />
                                            AI推荐 (置信度: {recommendation.confidence}%)
                                          </div>
                                          <div className="text-muted-foreground pl-5">
                                            {recommendation.reason}
                                          </div>
                                        </div>
                                      )}
                                    </div>
                                  </div>
                                </div>
                              )
                            })}
                          </div>
                        ) : (
                          <div className="flex flex-col items-center justify-center h-full text-muted-foreground py-12">
                            <Users className="w-12 h-12 mb-3 opacity-30" />
                            <p className="text-sm">
                              {filterKeyword || showOnlyRecommended
                                ? '没有符合条件的用户故事'
                                : isAnalyzing
                                  ? 'AI正在分析中...'
                                  : '暂无未分配的用户故事'}
                            </p>
                            {!isAnalyzing && unassignedStories.length === 0 && (
                              <p className="text-xs mt-2 text-center max-w-xs">
                                所有用户故事都已分配到Sprint
                              </p>
                            )}
                          </div>
                        )}
                      </div>
                    </ScrollArea>
                  </div>
                </CardContent>
              </Card>
            </div>
          </div>
        </div>

        {/* 固定底部操作按钮 */}
        <DialogFooter className="flex-shrink-0 px-6 py-4 border-t bg-background">
          <Button variant="outline" onClick={() => onOpenChange(false)}>
            取消
          </Button>
          <Button
            onClick={handleSave}
            disabled={selectedStoryIds.length === 0 || isAnalyzing}
            className="gap-2"
          >
            <CheckCircle2 className="w-4 h-4" />
            确认分配 ({selectedStoryIds.length})
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
