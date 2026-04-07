import React from 'react'
import ReactMarkdown, { type Components } from 'react-markdown'
import remarkGfm from 'remark-gfm'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Target, Users, Zap, Clock } from 'lucide-react'
import type { PRD } from '@/types'
import {
  TableComponent,
  ThComponent,
  TdComponent,
  TrComponent,
  FullDocComponents,
} from './PRDDisplayMarkdownComponents'

interface TabsContentProps {
  prd: PRD
}

// 完整文档视图
export function FullDocTab({ prd }: TabsContentProps) {
  return (
    <Card>
      <CardHeader>
        <CardTitle className="text-xl font-semibold">📄 产品需求文档</CardTitle>
      </CardHeader>
      <CardContent>
        <div className="prose prose-slate max-w-none">
          <ReactMarkdown
            remarkPlugins={[remarkGfm]}
            components={FullDocComponents as Partial<Components>}
          >
            {prd.markdownContent ||
              `# ${prd.title}\n\n## 产品概述\n\n${prd.overview}\n\n## 目标用户\n\n${prd.targetUsers.map(u => `- ${u}`).join('\n')}\n\n## 核心功能\n\n${prd.coreFeatures.map(f => `- ${f}`).join('\n')}\n\n## 技术栈\n\n${prd.techStack.map(t => `- ${t}`).join('\n')}\n\n## 预估工作量\n\n${prd.estimatedEffort}\n\n## 商业模式\n\n${prd.businessModel || '待定'}\n\n## 定价策略\n\n${prd.pricing || '待定'}`}
          </ReactMarkdown>
        </div>
      </CardContent>
    </Card>
  )
}

// 概述视图
export function OverviewTab({ prd }: TabsContentProps) {
  const tableComponents = {
    table: TableComponent,
    th: ThComponent,
    td: TdComponent,
    tr: TrComponent,
  }

  return (
    <>
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
              components={tableComponents as Partial<Components>}
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
    </>
  )
}

// 功能视图
export function FeaturesTab({ prd }: TabsContentProps) {
  return (
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
                    p: ({ ...props }) => <span {...props} />,
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
  )
}

// 技术视图
export function TechTab({ prd }: TabsContentProps) {
  const tableComponents = {
    table: TableComponent,
    th: ThComponent,
    td: TdComponent,
    tr: TrComponent,
  }

  return (
    <>
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
              components={tableComponents as Partial<Components>}
            >
              {prd.estimatedEffort}
            </ReactMarkdown>
          </div>
        </CardContent>
      </Card>
    </>
  )
}

// 商业视图
export function BusinessTab({ prd }: TabsContentProps) {
  const tableComponents = {
    table: TableComponent,
    th: ThComponent,
    td: TdComponent,
    tr: TrComponent,
  }

  return (
    <>
      <Card>
        <CardHeader>
          <CardTitle>商业模式</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="prose prose-sm max-w-none">
            <ReactMarkdown
              remarkPlugins={[remarkGfm]}
              components={tableComponents as Partial<Components>}
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
              components={tableComponents as Partial<Components>}
            >
              {prd.pricing}
            </ReactMarkdown>
          </div>
        </CardContent>
      </Card>
    </>
  )
}
