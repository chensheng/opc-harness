import { useEffect, useState } from 'react'
import { useParams, useNavigate } from 'react-router-dom'
import { User, Briefcase, Target, Quote, ArrowRight, ArrowLeft } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card, CardContent } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { useProjectStore, useAppStore } from '@/stores'
import type { UserPersona } from '@/types'

// Simulated AI-generated personas
function generateMockPersonas(): UserPersona[] {
  return [
    {
      id: '1',
      name: 'Alex',
      age: '28岁',
      occupation: '全栈开发者',
      background: '有5年开发经验，正在开发一个SaaS产品作为副业。喜欢尝试新技术，追求效率。',
      goals: ['快速验证产品想法', '减少重复性工作', '实现被动收入'],
      painPoints: ['时间有限', '不懂设计和营销', '难以坚持长期项目'],
      behaviors: ['订阅技术博客', '活跃在Twitter/X', '周末投入side project'],
      quote: '我想把更多时间花在创造性工作上，而不是重复性任务。',
    },
    {
      id: '2',
      name: 'Sarah',
      age: '32岁',
      occupation: 'UI/UX设计师',
      background: '在设计行业工作8年，正在转型独立创业。有丰富设计经验但技术能力有限。',
      goals: ['将设计能力变现', '建立个人品牌', '实现工作自由'],
      painPoints: ['不懂技术实现', '缺乏商业思维', '担心收入不稳定'],
      behaviors: ['在Dribbble分享作品', '运营设计公众号', '参加设计社区活动'],
      quote: '我有好的设计想法，但不知道怎么把它们变成实际产品。',
    },
    {
      id: '3',
      name: 'Mike',
      age: '35岁',
      occupation: '产品经理',
      background: '在科技公司工作10年，有丰富的产品经验。正在考虑辞职创业。',
      goals: ['验证创业想法', '建立MVP', '找到早期用户'],
      painPoints: ['缺乏技术合伙人', '资源有限', '需要快速迭代'],
      behaviors: ['阅读创业书籍', '参加创业活动', '关注Product Hunt'],
      quote: '我需要快速验证我的想法，而不是花几个月开发一个可能没人要的产品。',
    },
  ]
}

export function UserPersonas() {
  const { projectId } = useParams<{ projectId: string }>()
  const navigate = useNavigate()
  const { getProjectById, setProjectPersonas } = useProjectStore()
  const { setLoading } = useAppStore()

  const [personas, setPersonas] = useState<UserPersona[]>([])
  const [activeIndex, setActiveIndex] = useState(0)

  const project = projectId ? getProjectById(projectId) : undefined

  useEffect(() => {
    if (project) {
      if (project.userPersonas && project.userPersonas.length > 0) {
        setPersonas(project.userPersonas)
      } else {
        generatePersonas()
      }
    }
  }, [project])

  const generatePersonas = async () => {
    setLoading(true, 'AI正在生成用户画像...')

    try {
      await new Promise(resolve => setTimeout(resolve, 2000))

      const generatedPersonas = generateMockPersonas()
      setPersonas(generatedPersonas)

      if (projectId) {
        setProjectPersonas(projectId, generatedPersonas)
      }
    } finally {
      setLoading(false)
    }
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

  if (personas.length === 0) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="text-center">
          <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary mx-auto" />
          <p className="mt-4 text-muted-foreground">正在生成用户画像...</p>
        </div>
      </div>
    )
  }

  const activePersona = personas[activeIndex]

  return (
    <div className="max-w-4xl mx-auto space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold">👥 用户画像</h1>
          <p className="text-muted-foreground">{project.name}</p>
        </div>
        <div className="flex gap-2">
          {personas.map((_, index) => (
            <button
              key={index}
              onClick={() => setActiveIndex(index)}
              className={`w-3 h-3 rounded-full transition-colors ${
                index === activeIndex ? 'bg-primary' : 'bg-muted'
              }`}
            />
          ))}
        </div>
      </div>

      <Card className="overflow-hidden">
        <div className="bg-gradient-to-r from-primary/10 to-primary/5 p-6">
          <div className="flex items-center gap-4">
            <div className="w-16 h-16 rounded-full bg-primary/20 flex items-center justify-center">
              <User className="w-8 h-8 text-primary" />
            </div>
            <div>
              <h2 className="text-2xl font-bold">{activePersona.name}</h2>
              <p className="text-muted-foreground">
                {activePersona.age} · {activePersona.occupation}
              </p>
            </div>
          </div>
        </div>

        <CardContent className="p-6 space-y-6">
          <div>
            <h3 className="flex items-center gap-2 font-medium mb-2">
              <Briefcase className="w-4 h-4" />
              背景
            </h3>
            <p className="text-muted-foreground">{activePersona.background}</p>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div>
              <h3 className="flex items-center gap-2 font-medium mb-2">
                <Target className="w-4 h-4" />
                目标
              </h3>
              <ul className="space-y-1">
                {activePersona.goals.map((goal, index) => (
                  <li key={index} className="text-sm text-muted-foreground">
                    • {goal}
                  </li>
                ))}
              </ul>
            </div>

            <div>
              <h3 className="flex items-center gap-2 font-medium mb-2">
                <Target className="w-4 h-4" />
                痛点
              </h3>
              <ul className="space-y-1">
                {activePersona.painPoints.map((point, index) => (
                  <li key={index} className="text-sm text-muted-foreground">
                    • {point}
                  </li>
                ))}
              </ul>
            </div>
          </div>

          <div>
            <h3 className="font-medium mb-2">行为特征</h3>
            <div className="flex flex-wrap gap-2">
              {activePersona.behaviors.map((behavior, index) => (
                <Badge key={index} variant="secondary">
                  {behavior}
                </Badge>
              ))}
            </div>
          </div>

          {activePersona.quote && (
            <div className="bg-muted p-4 rounded-lg">
              <Quote className="w-5 h-5 text-muted-foreground mb-2" />
              <p className="italic text-muted-foreground">"{activePersona.quote}"</p>
            </div>
          )}
        </CardContent>
      </Card>

      <div className="flex justify-between">
        <Button variant="outline" onClick={() => navigate(`/prd/${projectId}`)}>
          <ArrowLeft className="w-4 h-4 mr-2" />
          返回PRD
        </Button>
        <Button onClick={() => navigate(`/competitors/${projectId}`)}>
          查看竞品分析
          <ArrowRight className="w-4 h-4 ml-2" />
        </Button>
      </div>
    </div>
  )
}
