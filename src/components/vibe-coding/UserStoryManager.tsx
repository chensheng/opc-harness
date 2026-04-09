import { useState } from 'react'
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
} from 'lucide-react'
import { useUserStoryDecomposition } from '@/hooks/useUserStoryDecomposition'
import type { UserStory } from '@/types'

interface UserStoryManagerProps {
  /** PRD 内容或功能描述 */
  prdContent?: string
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
  prdContent = '',
  apiKey,
  onStoriesGenerated,
}: UserStoryManagerProps) {
  const [inputContent, setInputContent] = useState(prdContent)
  const [activeTab, setActiveTab] = useState<'input' | 'stories'>('input')

  const { userStories, loading, error, decompose, reset } = useUserStoryDecomposition()

  const handleDecompose = async () => {
    if (!inputContent.trim()) {
      return
    }

    await decompose(inputContent, apiKey)

    if (userStories.length > 0 && onStoriesGenerated) {
      onStoriesGenerated(userStories)
    }

    setActiveTab('stories')
  }

  const handleReset = () => {
    reset()
    setActiveTab('input')
  }

  const getStoryStats = () => {
    const total = userStories.length
    const p0 = userStories.filter(s => s.priority === 'P0').length
    const p1 = userStories.filter(s => s.priority === 'P1').length
    const avgPoints = userStories.reduce((sum, s) => sum + (s.storyPoints || 0), 0) / total || 0

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
        {userStories.length > 0 && (
          <Button variant="outline" onClick={handleReset}>
            重新拆分
          </Button>
        )}
      </div>

      <Tabs value={activeTab} onValueChange={value => setActiveTab(value as 'input' | 'stories')}>
        <TabsList className="grid w-full grid-cols-2">
          <TabsTrigger value="input">
            <FileText className="w-4 h-4 mr-2" />
            输入 PRD
          </TabsTrigger>
          <TabsTrigger value="stories" disabled={userStories.length === 0}>
            <Target className="w-4 h-4 mr-2" />
            用户故事 ({userStories.length})
          </TabsTrigger>
        </TabsList>

        {/* Input Tab */}
        <TabsContent value="input" className="space-y-4">
          <Card>
            <CardHeader>
              <CardTitle>PRD 内容或功能描述</CardTitle>
              <CardDescription>
                粘贴您的产品需求文档或详细描述，AI 将自动识别并拆分为用户故事
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <Textarea
                placeholder="例如：&#10;我们需要一个任务管理系统，包含以下功能：&#10;1. 用户可以创建、编辑、删除任务&#10;2. 任务可以设置优先级和截止日期&#10;3. 支持任务分类和标签&#10;4. 提供任务统计报表..."
                value={inputContent}
                onChange={e => setInputContent(e.target.value)}
                className="min-h-[300px] font-mono text-sm"
              />

              {error && (
                <div className="flex items-start gap-2 p-3 bg-red-50 dark:bg-red-950/20 border border-red-200 dark:border-red-800 rounded-md">
                  <AlertCircle className="w-5 h-5 text-red-500 mt-0.5" />
                  <div className="text-sm text-red-700 dark:text-red-400">{error}</div>
                </div>
              )}

              <Button
                onClick={handleDecompose}
                disabled={!inputContent.trim() || loading}
                className="w-full"
                size="lg"
              >
                {loading ? (
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
                  <li>提供详细的功能描述可以获得更准确的拆分结果</li>
                  <li>AI 会遵循 INVEST 原则生成用户故事</li>
                  <li>每个故事都会包含验收标准和优先级评估</li>
                </ul>
              </div>
            </CardContent>
          </Card>
        </TabsContent>

        {/* Stories Tab */}
        <TabsContent value="stories">
          {userStories.length > 0 && (
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
                  {userStories.map(story => (
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
