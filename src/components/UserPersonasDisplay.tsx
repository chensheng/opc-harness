import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { User, Target, Frown, Activity, Quote } from 'lucide-react'
import type { UserPersona } from '@/types'

interface PersonaCardProps {
  persona: UserPersona
  index: number
}

/**
 * 单个用户画像卡片组件
 */
export function PersonaCard({ persona }: PersonaCardProps) {
  const initials = persona.name
    .split(' ')
    .map(word => word[0])
    .join('')
    .toUpperCase()
    .slice(0, 2)

  return (
    <Card className="animate-in fade-in slide-in-from-bottom-4 duration-700 fill-mode-backwards">
      <CardHeader>
        <div className="flex items-start gap-4">
          <div className="h-16 w-16 rounded-full bg-primary/10 flex items-center justify-center text-primary font-semibold text-lg">
            {initials}
          </div>
          <div className="flex-1">
            <CardTitle className="text-xl mb-2">{persona.name}</CardTitle>
            <div className="flex flex-wrap gap-2">
              {persona.age && (
                <Badge variant="secondary" className="flex items-center gap-1">
                  <User className="w-3 h-3" />
                  {persona.age}
                </Badge>
              )}
              {persona.occupation && (
                <Badge variant="outline" className="flex items-center gap-1">
                  <Target className="w-3 h-3" />
                  {persona.occupation}
                </Badge>
              )}
            </div>
          </div>
        </div>
      </CardHeader>
      <CardContent className="space-y-4">
        {/* 背景描述 */}
        {persona.background && (
          <div className="space-y-2">
            <h4 className="font-semibold text-sm text-muted-foreground">背景</h4>
            <p className="text-sm text-muted-foreground leading-relaxed">{persona.background}</p>
          </div>
        )}

        {/* 目标 */}
        {persona.goals.length > 0 && (
          <div className="space-y-2">
            <h4 className="font-semibold text-sm text-muted-foreground flex items-center gap-2">
              <Target className="w-4 h-4" />
              目标
            </h4>
            <ul className="space-y-1">
              {persona.goals.map((goal, idx) => (
                <li key={idx} className="text-sm flex items-start gap-2">
                  <span className="text-primary mt-1">•</span>
                  <span>{goal}</span>
                </li>
              ))}
            </ul>
          </div>
        )}

        {/* 痛点 */}
        {persona.painPoints.length > 0 && (
          <div className="space-y-2">
            <h4 className="font-semibold text-sm text-muted-foreground flex items-center gap-2">
              <Frown className="w-4 h-4" />
              痛点
            </h4>
            <ul className="space-y-1">
              {persona.painPoints.map((pain, idx) => (
                <li key={idx} className="text-sm flex items-start gap-2">
                  <span className="text-destructive mt-1">•</span>
                  <span>{pain}</span>
                </li>
              ))}
            </ul>
          </div>
        )}

        {/* 行为 */}
        {persona.behaviors.length > 0 && (
          <div className="space-y-2">
            <h4 className="font-semibold text-sm text-muted-foreground flex items-center gap-2">
              <Activity className="w-4 h-4" />
              行为特征
            </h4>
            <ul className="space-y-1">
              {persona.behaviors.map((behavior, idx) => (
                <li key={idx} className="text-sm flex items-start gap-2">
                  <span className="text-accent-foreground mt-1">•</span>
                  <span>{behavior}</span>
                </li>
              ))}
            </ul>
          </div>
        )}

        {/* 引用 */}
        {persona.quote && (
          <div className="border-l-4 border-primary pl-4 py-2 bg-accent/50 rounded-r">
            <Quote className="w-4 h-4 text-primary mb-1" />
            <blockquote className="text-sm italic text-muted-foreground">
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
}

/**
 * 用户画像渐进式渲染显示组件
 *
 * 支持流式生成和渐进式渲染
 */
export function UserPersonasDisplay({
  isGenerating = false,
  progress = 0,
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
        <Card>
          <CardContent className="py-6">
            <div className="space-y-2">
              <div className="flex items-center justify-between text-sm">
                <span className="text-muted-foreground">生成进度</span>
                <span className="text-muted-foreground">{progress}%</span>
              </div>
              <div className="h-2 bg-accent rounded-full overflow-hidden">
                <div
                  className="h-full bg-primary transition-all duration-300"
                  style={{ width: `${progress}%` }}
                />
              </div>
            </div>
          </CardContent>
        </Card>

        {/* 骨架屏加载动画 */}
        <div className="grid gap-6 md:grid-cols-2">
          {[1, 2].map(i => (
            <Card key={i} className="animate-pulse">
              <CardHeader>
                <div className="flex items-start gap-4">
                  <div className="rounded-full bg-muted h-16 w-16" />
                  <div className="flex-1 space-y-2">
                    <div className="h-4 bg-muted rounded w-3/4" />
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

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-2xl font-bold flex items-center gap-2">
          <User className="w-6 h-6" />
          用户画像
        </h2>
        <p className="text-muted-foreground mt-1">典型用户角色及其特征分析</p>
      </div>

      {/* 画像卡片网格 */}
      <div className="grid gap-6 md:grid-cols-2">
        {/* 示例画像 - 实际使用时由父组件传入 personas 数组 */}
        <Card>
          <CardHeader>
            <div className="flex items-start gap-4">
              <div className="h-16 w-16 rounded-full bg-primary/10 flex items-center justify-center text-primary font-semibold text-lg">
                张
              </div>
              <div className="flex-1">
                <CardTitle className="text-xl mb-2">张先生</CardTitle>
                <div className="flex flex-wrap gap-2">
                  <Badge variant="secondary">
                    <User className="w-3 h-3 mr-1" />
                    35 岁
                  </Badge>
                  <Badge variant="outline">
                    <Target className="w-3 h-3 mr-1" />
                    IT 经理
                  </Badge>
                </div>
              </div>
            </div>
          </CardHeader>
          <CardContent className="space-y-3">
            <div>
              <h4 className="font-semibold text-sm text-muted-foreground">背景</h4>
              <p className="text-sm text-muted-foreground">
                在一家中型企业担任 IT 部门负责人，管理 10 人团队...
              </p>
            </div>
            <div>
              <h4 className="font-semibold text-sm text-muted-foreground flex items-center gap-2">
                <Target className="w-4 h-4" />
                目标
              </h4>
              <ul className="space-y-1 text-sm">
                <li className="flex items-start gap-2">
                  <span className="text-primary mt-1">•</span>
                  <span>提高团队工作效率</span>
                </li>
                <li className="flex items-start gap-2">
                  <span className="text-primary mt-1">•</span>
                  <span>降低项目管理成本</span>
                </li>
              </ul>
            </div>
          </CardContent>
        </Card>
      </div>
    </div>
  )
}
