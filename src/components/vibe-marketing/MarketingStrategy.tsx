import { useEffect, useState } from 'react'
import { useParams, useNavigate } from 'react-router-dom'
import {
  TrendingUp,
  Calendar,
  MessageSquare,
  Share2,
  Copy,
  Check,
  ArrowRight,
  ArrowLeft,
} from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { useProjectStore, useAppStore } from '@/stores'
import type { MarketingStrategy as MarketingStrategyType, MarketingCopy } from '@/types'

// Simulated AI-generated marketing strategy
function generateMockMarketingStrategy(): MarketingStrategyType {
  return {
    channels: [
      {
        name: 'Product Hunt',
        platform: 'producthunt',
        priority: 'high',
        description: '科技产品首发平台，适合获取早期用户',
      },
      {
        name: 'Twitter/X',
        platform: 'twitter',
        priority: 'high',
        description: '开发者社区活跃，适合技术产品推广',
      },
      {
        name: 'Reddit',
        platform: 'reddit',
        priority: 'medium',
        description: '相关subreddit可以精准触达目标用户',
      },
      {
        name: 'Hacker News',
        platform: 'hackernews',
        priority: 'medium',
        description: '技术社区，适合展示产品技术亮点',
      },
      {
        name: 'Indie Hackers',
        platform: 'indiehackers',
        priority: 'medium',
        description: '独立开发者社区，适合分享创业故事',
      },
    ],
    timeline: [
      {
        phase: '预热期',
        duration: '1周前',
        activities: ['创建预告页面', '准备宣传素材', '联系早期用户'],
      },
      {
        phase: '发布日',
        duration: 'Day 0',
        activities: ['Product Hunt发布', '社交媒体同步', '邮件通知订阅用户'],
      },
      {
        phase: '推广期',
        duration: '发布后1周',
        activities: ['回复用户反馈', '收集 testimonials', '持续社交媒体互动'],
      },
      {
        phase: '迭代期',
        duration: '发布后1月',
        activities: ['分析用户数据', '优化产品', '规划新功能'],
      },
    ],
    keyMessages: [
      '为独立创造者提供一站式解决方案',
      '从想法到产品，只需几天而非几周',
      'AI驱动，让每个人都能成为一人公司',
    ],
  }
}

// Simulated AI-generated marketing copy
function generateMockMarketingCopies(): MarketingCopy[] {
  return [
    {
      platform: 'twitter',
      content: `🚀 新产品发布！

OPC-HARNESS - AI驱动的一人公司操作系统

✨ 产品构思 → 快速构建 → 增长运营
✨ 从想法到产品，只需几天
✨ 让AI帮你完成从0到1

适合独立开发者、设计师、内容创作者

#BuildInPublic #IndieHackers #AI`,
      hashtags: ['BuildInPublic', 'IndieHackers', 'AI'],
    },
    {
      platform: 'producthunt',
      content: `OPC-HARNESS: 让每个人都能成为一人公司

我们整合了产品构思、快速构建、增长运营三大模块，为独立创造者提供一站式解决方案。

🎯 解决什么问题？
• 工具分散，流程割裂
• 技能短板，单打独斗困难
• 从想法到产品的路径不清晰

✨ 核心功能
• AI驱动产品构思
• 可视化代码生成
• 自动化增长运营

适合独立开发者、设计师、技术创业者`,
    },
    {
      platform: 'reddit',
      content: `Hi r/SideProject!

I've been working on OPC-HARNESS, an AI-powered operating system for one-person companies.

It helps indie creators go from idea to product in days instead of weeks by integrating:
- Vibe Design (AI product ideation)
- Vibe Coding (AI-assisted development)  
- Vibe Marketing (AI growth operations)

Would love your feedback!`,
    },
  ]
}

export function MarketingStrategy() {
  const { projectId } = useParams<{ projectId: string }>()
  const navigate = useNavigate()
  const { getProjectById, updateProjectStatus, updateProjectProgress } = useProjectStore()
  const { setLoading } = useAppStore()

  const [strategy, setStrategy] = useState<MarketingStrategyType | null>(null)
  const [copies, setCopies] = useState<MarketingCopy[]>([])
  const [copiedIndex, setCopiedIndex] = useState<number | null>(null)

  const project = projectId ? getProjectById(projectId) : undefined

  useEffect(() => {
    if (project) {
      generateMarketingContent()
      if (project.status === 'coding') {
        updateProjectStatus(projectId!, 'marketing')
        updateProjectProgress(projectId!, 75)
      }
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [project])

  const generateMarketingContent = async () => {
    setLoading(true, 'AI正在生成营销策略...')

    try {
      await new Promise(resolve => setTimeout(resolve, 2000))

      setStrategy(generateMockMarketingStrategy())
      setCopies(generateMockMarketingCopies())
    } finally {
      setLoading(false)
    }
  }

  const handleCopy = async (content: string, index: number) => {
    await navigator.clipboard.writeText(content)
    setCopiedIndex(index)
    setTimeout(() => setCopiedIndex(null), 2000)
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

  if (!strategy) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="text-center">
          <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary mx-auto" />
          <p className="mt-4 text-muted-foreground">正在生成营销策略...</p>
        </div>
      </div>
    )
  }

  return (
    <div className="max-w-4xl mx-auto space-y-6">
      <div>
        <h1 className="text-2xl font-bold">📈 Vibe Marketing</h1>
        <p className="text-muted-foreground">{project.name}</p>
      </div>

      <Tabs defaultValue="strategy" className="w-full">
        <TabsList className="grid w-full grid-cols-3">
          <TabsTrigger value="strategy">发布策略</TabsTrigger>
          <TabsTrigger value="timeline">时间线</TabsTrigger>
          <TabsTrigger value="copies">营销文案</TabsTrigger>
        </TabsList>

        <TabsContent value="strategy" className="space-y-4">
          <Card>
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <TrendingUp className="w-5 h-5" />
                推广渠道
              </CardTitle>
              <CardDescription>根据你的产品特点，推荐以下推广渠道</CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              {strategy.channels.map((channel, index) => (
                <div key={index} className="flex items-start justify-between p-4 border rounded-lg">
                  <div>
                    <div className="flex items-center gap-2">
                      <h3 className="font-medium">{channel.name}</h3>
                      <Badge variant={channel.priority === 'high' ? 'default' : 'secondary'}>
                        {channel.priority === 'high' ? '高优先级' : '中优先级'}
                      </Badge>
                    </div>
                    <p className="text-sm text-muted-foreground mt-1">{channel.description}</p>
                  </div>
                </div>
              ))}
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <MessageSquare className="w-5 h-5" />
                核心信息
              </CardTitle>
            </CardHeader>
            <CardContent>
              <ul className="space-y-2">
                {strategy.keyMessages.map((message, index) => (
                  <li key={index} className="flex items-start gap-2">
                    <span className="text-primary mt-1">•</span>
                    <span>{message}</span>
                  </li>
                ))}
              </ul>
            </CardContent>
          </Card>
        </TabsContent>

        <TabsContent value="timeline" className="space-y-4">
          <Card>
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Calendar className="w-5 h-5" />
                发布时间线
              </CardTitle>
            </CardHeader>
            <CardContent>
              <div className="space-y-4">
                {strategy.timeline.map((item, index) => (
                  <div key={index} className="flex gap-4">
                    <div className="flex flex-col items-center">
                      <div className="w-8 h-8 rounded-full bg-primary text-primary-foreground flex items-center justify-center text-sm font-medium">
                        {index + 1}
                      </div>
                      {index < strategy.timeline.length - 1 && (
                        <div className="w-0.5 h-full bg-border mt-2" />
                      )}
                    </div>
                    <div className="flex-1 pb-6">
                      <div className="flex items-center gap-2 mb-2">
                        <h3 className="font-medium">{item.phase}</h3>
                        <Badge variant="outline">{item.duration}</Badge>
                      </div>
                      <ul className="space-y-1">
                        {item.activities.map((activity, i) => (
                          <li key={i} className="text-sm text-muted-foreground">
                            • {activity}
                          </li>
                        ))}
                      </ul>
                    </div>
                  </div>
                ))}
              </div>
            </CardContent>
          </Card>
        </TabsContent>

        <TabsContent value="copies" className="space-y-4">
          {copies.map((copy, index) => (
            <Card key={index}>
              <CardHeader className="pb-3">
                <div className="flex items-center justify-between">
                  <CardTitle className="text-base capitalize flex items-center gap-2">
                    <Share2 className="w-4 h-4" />
                    {copy.platform}
                  </CardTitle>
                  <Button variant="ghost" size="sm" onClick={() => handleCopy(copy.content, index)}>
                    {copiedIndex === index ? (
                      <>
                        <Check className="w-4 h-4 mr-2" />
                        已复制
                      </>
                    ) : (
                      <>
                        <Copy className="w-4 h-4 mr-2" />
                        复制
                      </>
                    )}
                  </Button>
                </div>
              </CardHeader>
              <CardContent>
                <pre className="whitespace-pre-wrap text-sm text-muted-foreground bg-muted p-4 rounded-lg">
                  {copy.content}
                </pre>
                {copy.hashtags && (
                  <div className="flex flex-wrap gap-2 mt-3">
                    {copy.hashtags.map((tag, i) => (
                      <Badge key={i} variant="secondary">
                        #{tag}
                      </Badge>
                    ))}
                  </div>
                )}
              </CardContent>
            </Card>
          ))}
        </TabsContent>
      </Tabs>

      <div className="flex justify-between">
        <Button variant="outline" onClick={() => navigate(`/coding/${projectId}`)}>
          <ArrowLeft className="w-4 h-4 mr-2" />
          返回开发
        </Button>
        <Button onClick={() => navigate('/')}>
          完成
          <ArrowRight className="w-4 h-4 ml-2" />
        </Button>
      </div>
    </div>
  )
}
