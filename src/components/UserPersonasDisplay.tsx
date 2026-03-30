import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { User, Target, Frown, Activity, Quote, Briefcase, TrendingUp, Heart } from 'lucide-react'
import type { UserPersona } from '@/types'

interface PersonaCardProps {
  persona: UserPersona
  index: number
}

/**
 * 单个用户画像卡片组件（优化版）
 */
export function PersonaCard({ persona, index }: PersonaCardProps) {
  const initials = persona.name
    .split(' ')
    .map(word => word[0])
    .join('')
    .toUpperCase()
    .slice(0, 2)

  // 生成渐变色头像背景
  const gradientColors = [
    'from-blue-500 to-cyan-500',
    'from-purple-500 to-pink-500',
    'from-green-500 to-emerald-500',
    'from-orange-500 to-red-500',
    'from-indigo-500 to-blue-500',
    'from-teal-500 to-green-500',
  ]
  const colorIndex = index % gradientColors.length

  return (
    <Card className="group hover:shadow-xl transition-all duration-300 animate-in fade-in slide-in-from-bottom-4 fill-mode-backwards border-0 shadow-md">
      <CardHeader className="bg-gradient-to-br from-muted/50 to-muted/10 rounded-t-lg pb-4">
        <div className="flex items-start gap-4">
          {/* 头像 */}
          <div
            className={`h-20 w-20 rounded-full bg-gradient-to-br ${gradientColors[colorIndex]} flex items-center justify-center text-white font-bold text-xl shadow-lg ring-4 ring-white/50 group-hover:scale-110 transition-transform duration-300`}
          >
            {initials}
          </div>
          <div className="flex-1 pt-1">
            <CardTitle className="text-2xl font-bold mb-2 text-gray-900 group-hover:text-primary transition-colors duration-300">
              {persona.name}
            </CardTitle>
            <div className="flex flex-wrap gap-2">
              {persona.age && (
                <Badge
                  variant="secondary"
                  className="flex items-center gap-1 bg-blue-100 text-blue-700 hover:bg-blue-200"
                >
                  <User className="w-3 h-3" />
                  {persona.age}
                </Badge>
              )}
              {persona.occupation && (
                <Badge
                  variant="outline"
                  className="flex items-center gap-1 border-purple-200 text-purple-700 bg-purple-50 hover:bg-purple-100"
                >
                  <Briefcase className="w-3 h-3" />
                  {persona.occupation}
                </Badge>
              )}
            </div>
          </div>
        </div>
      </CardHeader>
      <CardContent className="space-y-4 pt-4">
        {/* 背景描述 */}
        {persona.background && (
          <div className="space-y-2 p-3 bg-blue-50/50 rounded-lg border border-blue-100">
            <h4 className="font-semibold text-sm text-blue-800 flex items-center gap-2">
              <User className="w-4 h-4" />
              背景
            </h4>
            <p className="text-sm text-gray-700 leading-relaxed">{persona.background}</p>
          </div>
        )}

        {/* 目标 */}
        {persona.goals.length > 0 && (
          <div className="space-y-2 p-3 bg-green-50/50 rounded-lg border border-green-100">
            <h4 className="font-semibold text-sm text-green-800 flex items-center gap-2">
              <TrendingUp className="w-4 h-4" />
              目标
            </h4>
            <ul className="space-y-1.5">
              {persona.goals.map((goal, idx) => (
                <li key={idx} className="text-sm flex items-start gap-2">
                  <span className="text-green-600 mt-1 text-lg leading-none">›</span>
                  <span className="text-gray-700">{goal}</span>
                </li>
              ))}
            </ul>
          </div>
        )}

        {/* 痛点 */}
        {persona.painPoints.length > 0 && (
          <div className="space-y-2 p-3 bg-red-50/50 rounded-lg border border-red-100">
            <h4 className="font-semibold text-sm text-red-800 flex items-center gap-2">
              <Heart className="w-4 h-4 rotate-180" />
              痛点
            </h4>
            <ul className="space-y-1.5">
              {persona.painPoints.map((pain, idx) => (
                <li key={idx} className="text-sm flex items-start gap-2">
                  <span className="text-red-600 mt-1 text-lg leading-none">!</span>
                  <span className="text-gray-700">{pain}</span>
                </li>
              ))}
            </ul>
          </div>
        )}

        {/* 行为 */}
        {persona.behaviors.length > 0 && (
          <div className="space-y-2 p-3 bg-purple-50/50 rounded-lg border border-purple-100">
            <h4 className="font-semibold text-sm text-purple-800 flex items-center gap-2">
              <Activity className="w-4 h-4" />
              行为特征
            </h4>
            <ul className="space-y-1.5">
              {persona.behaviors.map((behavior, idx) => (
                <li key={idx} className="text-sm flex items-start gap-2">
                  <span className="text-purple-600 mt-1 text-lg leading-none">•</span>
                  <span className="text-gray-700">{behavior}</span>
                </li>
              ))}
            </ul>
          </div>
        )}

        {/* 引用 */}
        {persona.quote && (
          <div className="border-l-4 border-primary pl-4 py-3 bg-gradient-to-r from-primary/10 to-transparent rounded-r-lg">
            <Quote className="w-5 h-5 text-primary/60 mb-1" />
            <blockquote className="text-sm italic text-gray-700 font-medium">
              "{persona.quote}"
            </blockquote>
          </div>
        )}
      </CardContent>
    </Card>
  )
}

interface UserPersonasDisplayProps {
  /** 是否正在生成中 */
  isGenerating?: boolean
  /** 生成进度 */
  progress?: number
  /** 用户画像数组 */
  personas?: UserPersona[]
}

/**
 * 用户画像渐进式渲染显示组件（优化版）
 *
 * 支持流式生成和渐进式渲染
 */
export function UserPersonasDisplay({
  isGenerating = false,
  progress = 0,
  personas = [],
}: UserPersonasDisplayProps) {
  // 这个组件将由父组件配合 usePersonaStream Hook 使用
  // 这里提供静态展示和加载状态

  if (isGenerating) {
    return (
      <div className="space-y-6">
        <div className="flex items-center justify-between">
          <div>
            <h2 className="text-2xl font-bold flex items-center gap-2">
              <User className="w-6 h-6 text-primary animate-pulse" />
              AI 正在生成用户画像...
            </h2>
            <p className="text-muted-foreground mt-1">基于产品特性和市场分析，创建典型用户角色</p>
          </div>
        </div>

        {/* 进度条 */}
        <Card className="border-0 shadow-md">
          <CardContent className="py-6">
            <div className="space-y-2">
              <div className="flex items-center justify-between text-sm">
                <span className="text-muted-foreground">生成进度</span>
                <span className="text-muted-foreground font-medium">{progress}%</span>
              </div>
              <div className="h-3 bg-accent/50 rounded-full overflow-hidden">
                <div
                  className="h-full bg-gradient-to-r from-primary to-cyan-500 transition-all duration-300 ease-out"
                  style={{ width: `${progress}%` }}
                />
              </div>
            </div>
          </CardContent>
        </Card>

        {/* 骨架屏加载动画 */}
        <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
          {[1, 2, 3].map(i => (
            <Card key={i} className="animate-pulse border-0 shadow-md">
              <CardHeader>
                <div className="flex items-start gap-4">
                  <div className="rounded-full bg-muted h-20 w-20" />
                  <div className="flex-1 space-y-2">
                    <div className="h-5 bg-muted rounded w-3/4" />
                    <div className="flex gap-2">
                      <div className="h-6 bg-muted rounded w-20" />
                      <div className="h-6 bg-muted rounded w-24" />
                    </div>
                  </div>
                </div>
              </CardHeader>
              <CardContent className="space-y-3">
                <div className="h-3 bg-muted rounded w-full" />
                <div className="h-3 bg-muted rounded w-5/6" />
                <div className="h-3 bg-muted rounded w-4/6" />
              </CardContent>
            </Card>
          ))}
        </div>
      </div>
    )
  }

  if (personas.length === 0) {
    return null
  }

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-2xl font-bold flex items-center gap-2">
          <User className="w-6 h-6" />
          用户画像
        </h2>
        <p className="text-muted-foreground mt-1">典型用户角色及其特征分析</p>
      </div>

      {/* 画像卡片网格 - 响应式布局 */}
      <div className="grid gap-6 sm:grid-cols-1 md:grid-cols-2 lg:grid-cols-3 auto-rows-fr">
        {personas.map((persona, index) => (
          <PersonaCard key={index} persona={persona} index={index} />
        ))}
      </div>
    </div>
  )
}
