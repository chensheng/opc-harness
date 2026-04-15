import { useState } from 'react'
import { Bot, Loader2, FileText } from 'lucide-react'
import { invoke } from '@tauri-apps/api/core'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Label } from '@/components/ui/label'
import { Textarea } from '@/components/ui/textarea'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { Input } from '@/components/ui/input'

interface CreateAgentDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  onSuccess?: (agentId: string) => void
  projectId?: string
}

// CLI 工具类型定义
const CLI_TYPES = [
  { value: 'codefree', label: 'CodeFree CLI', description: 'AI 编码助手，支持流式输出' },
  { value: 'kimi', label: 'Kimi CLI', description: '月之暗面官方 CLI，中文支持优秀' },
  { value: 'claude', label: 'Claude Code', description: 'Anthropic 官方 CLI，英文场景强大' },
  { value: 'codex', label: 'OpenAI Codex', description: 'OpenAI 官方 CLI，基于 GPT-4o' },
] as const

export function CreateAgentDialog({
  open,
  onOpenChange,
  onSuccess,
  projectId,
}: CreateAgentDialogProps) {
  const [agentName, setAgentName] = useState<string>('')
  const [cliType, setCliType] = useState<string>('codefree')
  const [agentsContent, setAgentsContent] = useState<string>('')
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const handleCreate = async () => {
    if (!agentsContent.trim()) {
      setError('请填写 AGENTS.md 内容')
      return
    }

    setIsLoading(true)
    setError(null)

    console.log('[CreateAgentDialog] Starting agent creation...')
    console.log('[CreateAgentDialog] agentName:', agentName)
    console.log('[CreateAgentDialog] cliType:', cliType)
    console.log('[CreateAgentDialog] projectId:', projectId)
    console.log('[CreateAgentDialog] agentsContent length:', agentsContent.length)

    try {
      console.log('[CreateAgentDialog] Invoking create_agent_with_cli...')
      const agentId = await invoke<string>('create_agent_with_cli', {
        cliType,
        agentsContent,
        projectId: projectId || '',
        name: agentName.trim() || null,
      })

      console.log('[CreateAgentDialog] Agent created successfully:', agentId)
      onSuccess?.(agentId)
      onOpenChange(false)

      // 重置表单
      setAgentName('')
      setAgentsContent('')
      setError(null)
    } catch (err) {
      console.error('[CreateAgentDialog] Failed to create agent:', err)
      const errorMessage = err instanceof Error ? err.message : String(err)
      console.error('[CreateAgentDialog] Error message:', errorMessage)
      setError(errorMessage || '创建智能体失败')
    } finally {
      setIsLoading(false)
    }
  }

  const handleClose = () => {
    if (!isLoading) {
      onOpenChange(false)
      setError(null)
    }
  }

  return (
    <Dialog open={open} onOpenChange={handleClose}>
      <DialogContent className="sm:max-w-[600px]">
        <DialogHeader>
          <DialogTitle className="flex items-center gap-2">
            <Bot className="w-5 h-5" />
            创建智能体
          </DialogTitle>
          <DialogDescription>
            配置并创建一个新的 AI 智能体，只需选择 CLI 类型并提供 AGENTS.md 内容
          </DialogDescription>
        </DialogHeader>

        <div className="grid gap-4 py-4">
          {/* Agent Name */}
          <div className="grid gap-2">
            <Label htmlFor="agent-name">智能体名称（可选）</Label>
            <Input
              id="agent-name"
              placeholder="例如：代码生成助手、测试智能体等"
              value={agentName}
              onChange={e => setAgentName(e.target.value)}
              disabled={isLoading}
            />
            <p className="text-xs text-muted-foreground">
              为智能体设置一个易于识别的名称，方便后续管理
            </p>
          </div>

          {/* CLI Type */}
          <div className="grid gap-2">
            <Label htmlFor="cli-type">CLI 类型</Label>
            <Select value={cliType} onValueChange={setCliType} disabled={isLoading}>
              <SelectTrigger id="cli-type">
                <SelectValue placeholder="选择 CLI 类型" />
              </SelectTrigger>
              <SelectContent>
                {CLI_TYPES.map(type => (
                  <SelectItem key={type.value} value={type.value}>
                    <div className="flex items-center gap-2">
                      <span>
                        {type.value === 'codefree'
                          ? '🤖'
                          : type.value === 'kimi'
                            ? '🌙'
                            : type.value === 'claude'
                              ? '🎭'
                              : '🧠'}
                      </span>
                      <div>
                        <div className="font-medium">{type.label}</div>
                        <div className="text-xs text-muted-foreground">{type.description}</div>
                      </div>
                    </div>
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>

          {/* AGENTS.md Content */}
          <div className="grid gap-2">
            <Label htmlFor="agents-content" className="flex items-center gap-2">
              <FileText className="w-4 h-4" />
              AGENTS.md 内容 *
            </Label>
            <Textarea
              id="agents-content"
              placeholder="# Agent Instructions&#10;&#10;You are an AI coding assistant...&#10;&#10;## Guidelines&#10;- Follow best practices&#10;- Write clean code&#10;- Add comments when necessary"
              value={agentsContent}
              onChange={e => setAgentsContent(e.target.value)}
              disabled={isLoading}
              className="min-h-[200px] font-mono text-sm"
            />
            <p className="text-xs text-muted-foreground">
              此内容将保存为项目的 AGENTS.md 文件，用于指导 AI 智能体的行为
            </p>
          </div>

          {/* Error Message */}
          {error && (
            <div className="text-sm text-destructive bg-destructive/10 p-3 rounded-md">{error}</div>
          )}
        </div>

        <DialogFooter>
          <Button variant="outline" onClick={handleClose} disabled={isLoading}>
            取消
          </Button>
          <Button onClick={handleCreate} disabled={isLoading}>
            {isLoading ? (
              <>
                <Loader2 className="w-4 h-4 mr-2 animate-spin" />
                创建中...
              </>
            ) : (
              <>
                <Bot className="w-4 h-4 mr-2" />
                创建智能体
              </>
            )}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
