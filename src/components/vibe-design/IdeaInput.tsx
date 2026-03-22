import { useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { Sparkles, ArrowRight } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '@/components/ui/card'
import { Textarea } from '@/components/ui/textarea'
import { Input } from '@/components/ui/input'
import { useProjectStore, useAppStore } from '@/stores'

export function IdeaInput() {
  const navigate = useNavigate()
  const { createProject } = useProjectStore()
  const { setLoading } = useAppStore()
  
  const [projectName, setProjectName] = useState('')
  const [idea, setIdea] = useState('')

  const handleSubmit = async () => {
    if (!projectName.trim() || !idea.trim()) return

    setLoading(true, '正在创建项目...')
    
    try {
      const project = createProject(projectName, idea.slice(0, 100), idea)
      
      // Simulate AI processing
      await new Promise(resolve => setTimeout(resolve, 1500))
      
      navigate(`/prd/${project.id}`)
    } finally {
      setLoading(false)
    }
  }

  const exampleIdeas = [
    '我想做一个帮助独立开发者管理项目进度的工具，类似Trello但是更简单，专门为单人项目设计',
    '我想创建一个在线简历生成器，让用户可以通过拖拽组件快速制作专业简历',
    '我想开发一个浏览器插件，帮助用户屏蔽社交媒体上的负面内容',
  ]

  return (
    <div className="max-w-3xl mx-auto space-y-6">
      <div className="text-center space-y-2">
        <h1 className="text-3xl font-bold">💡 输入你的产品想法</h1>
        <p className="text-muted-foreground">
          用自然语言描述你的想法，AI将帮你完善产品构思
        </p>
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
        开始分析
        <ArrowRight className="w-4 h-4 ml-2" />
      </Button>
    </div>
  )
}
