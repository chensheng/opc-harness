import React from 'react'
import ReactMarkdown, { type Components } from 'react-markdown'
import remarkGfm from 'remark-gfm'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Textarea } from '@/components/ui/textarea'
import { Pencil, Columns, Eye, X, Save } from 'lucide-react'
import { FullDocComponents } from './PRDDisplayMarkdownComponents'

interface EditorProps {
  editedMarkdown: string
  onMarkdownChange: (value: string) => void
  previewMode: 'edit' | 'preview' | 'split'
  onPreviewModeChange: (mode: 'edit' | 'preview' | 'split') => void
  onCancel: () => void
  onSave: () => void
}

export function PRDDEditor({
  editedMarkdown,
  onMarkdownChange,
  previewMode,
  onPreviewModeChange,
  onCancel,
  onSave,
}: EditorProps) {
  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-2">
          <Pencil className="w-5 h-5 text-primary" />
          <h2 className="text-xl font-bold">编辑产品需求文档</h2>
        </div>
        <div className="flex gap-2">
          <Button
            variant={previewMode === 'edit' ? 'secondary' : 'ghost'}
            size="sm"
            onClick={() => onPreviewModeChange('edit')}
            title="仅编辑"
            className={previewMode === 'edit' ? 'bg-secondary text-secondary-foreground' : ''}
          >
            <Pencil className="w-4 h-4 mr-1" />
            编辑
          </Button>
          <Button
            variant={previewMode === 'split' ? 'secondary' : 'ghost'}
            size="sm"
            onClick={() => onPreviewModeChange('split')}
            title="分屏预览"
            className={previewMode === 'split' ? 'bg-secondary text-secondary-foreground' : ''}
          >
            <Columns className="w-4 h-4 mr-1" />
            分屏
          </Button>
          <Button
            variant={previewMode === 'preview' ? 'secondary' : 'ghost'}
            size="sm"
            onClick={() => onPreviewModeChange('preview')}
            title="仅预览"
            className={previewMode === 'preview' ? 'bg-secondary text-secondary-foreground' : ''}
          >
            <Eye className="w-4 h-4 mr-1" />
            预览
          </Button>
        </div>
      </div>

      <div className={`grid gap-4 ${previewMode === 'split' ? 'grid-cols-2' : 'grid-cols-1'}`}>
        {/* 编辑器面板 */}
        {(previewMode === 'edit' || previewMode === 'split') && (
          <Card>
            <CardHeader className="py-3">
              <CardTitle className="text-sm font-medium text-muted-foreground">
                Markdown 编辑器
              </CardTitle>
            </CardHeader>
            <CardContent>
              <Textarea
                value={editedMarkdown}
                onChange={e => onMarkdownChange(e.target.value)}
                className="w-full min-h-[600px] font-mono text-sm leading-relaxed resize-y"
                placeholder="在此编辑 Markdown 格式的 PRD 内容..."
              />
              <p className="mt-2 text-xs text-muted-foreground">
                💡 提示：支持标准 Markdown 语法，包括表格、列表、代码块等
              </p>
            </CardContent>
          </Card>
        )}

        {/* 预览面板 */}
        {(previewMode === 'preview' || previewMode === 'split') && (
          <Card>
            <CardHeader className="py-3">
              <CardTitle className="text-sm font-medium text-muted-foreground">
                实时预览
              </CardTitle>
            </CardHeader>
            <CardContent>
              <div className="prose prose-slate max-w-none bg-muted/30 p-4 rounded-lg border border-border min-h-[600px] overflow-auto">
                <ReactMarkdown
                  remarkPlugins={[remarkGfm]}
                  components={FullDocComponents as Partial<Components>}
                >
                  {editedMarkdown}
                </ReactMarkdown>
              </div>
            </CardContent>
          </Card>
        )}
      </div>

      <div className="flex justify-end gap-2">
        <Button variant="outline" onClick={onCancel}>
          <X className="w-4 h-4 mr-2" />
          取消
        </Button>
        <Button onClick={onSave}>
          <Save className="w-4 h-4 mr-2" />
          保存更改
        </Button>
      </div>
    </div>
  )
}
