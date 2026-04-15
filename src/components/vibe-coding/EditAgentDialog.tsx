import { useState, useEffect } from 'react'
import { Bot, Loader2 } from 'lucide-react'
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
import { Input } from '@/components/ui/input'
import { Textarea } from '@/components/ui/textarea'
import type { AgentInfo } from './CodingWorkspaceTypes'

interface EditAgentDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  agent: AgentInfo | null
  onSuccess?: () => void
  projectId?: string
}

export function EditAgentDialog({
  open,
  onOpenChange,
  agent,
  onSuccess,
  projectId: _projectId,
}: EditAgentDialogProps) {
  const [agentName, setAgentName] = useState<string>('')
  const [agentsContent, setAgentsContent] = useState<string>('')
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  // 当agent变化时,初始化表单并加载AGENTS.md内容
  useEffect(() => {
    if (agent && open) {
      setAgentName(agent.name || '')
      setError(null)
      loadAgentsContent()
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [agent, open])

  // 加载AGENTS.md文件内容
  const loadAgentsContent = async () => {
    if (!agent) {
      return
    }

    try {
      // 从数据库获取完整的session信息(包含agents_md_content)
      const session = (await invoke('get_agent_session_by_id', {
        agentId: agent.agentId,
      })) as { agentsMdContent?: string }

      if (session && session.agentsMdContent) {
        setAgentsContent(session.agentsMdContent)
      } else {
        // 如果没有内容,设置为空字符串
        setAgentsContent('')
      }
    } catch (err) {
      console.error('[EditAgentDialog] Failed to load AGENTS.md content:', err)
      setAgentsContent('')
    }
  }

  const handleSave = async () => {
    if (!agent) {
      setError('未找到智能体信息')
      return
    }

    setIsLoading(true)
    setError(null)

    try {
      // 1. 获取完整的session信息
      const session = await invoke('get_agent_session_by_id', {
        agentId: agent.agentId,
      })

      if (!session) {
        throw new Error('无法获取智能体信息')
      }

      // 2. 更新session信息(包括name和agents_md_content)
      await invoke('update_agent_session', {
        session: {
          ...session,
          name: agentName.trim() || null,
          agents_md_content: agentsContent || null,
          updated_at: new Date().toISOString(),
        },
      })

      console.log('[EditAgentDialog] Agent and AGENTS.md content saved to database')

      onSuccess?.()
      onOpenChange(false)

      // 重置表单
      setAgentName('')
      setAgentsContent('')
      setError(null)
    } catch (err) {
      console.error('[EditAgentDialog] Failed to update agent:', err)
      const errorMessage = err instanceof Error ? err.message : String(err)
      setError(errorMessage || '更新智能体失败')
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

  if (!agent) {
    return null
  }

  return (
    <Dialog open={open} onOpenChange={handleClose}>
      <DialogContent className="sm:max-w-[500px]">
        <DialogHeader>
          <DialogTitle className="flex items-center gap-2">
            <Bot className="w-5 h-5" />
            编辑智能体
          </DialogTitle>
          <DialogDescription>修改智能体的名称和其他配置信息</DialogDescription>
        </DialogHeader>

        <div className="grid gap-4 py-4">
          {/* Agent Name */}
          <div className="grid gap-2">
            <Label htmlFor="agent-name">智能体名称</Label>
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

          {/* Agent ID (只读显示) */}
          <div className="grid gap-2">
            <Label>智能体 ID</Label>
            <div className="text-sm font-mono text-muted-foreground bg-muted p-2 rounded">
              {agent.agentId}
            </div>
            <p className="text-xs text-muted-foreground">智能体的唯一标识符，不可修改</p>
          </div>

          {/* Agent Type (只读显示) */}
          <div className="grid gap-2">
            <Label>智能体类型</Label>
            <div className="text-sm text-muted-foreground">
              {agent.type === 'initializer'
                ? '初始化'
                : agent.type === 'coding'
                  ? '编码'
                  : '创建 MR'}
            </div>
          </div>

          {/* AGENTS.md Content */}
          <div className="grid gap-2">
            <Label htmlFor="agents-content">AGENTS.md 内容</Label>
            <Textarea
              id="agents-content"
              placeholder="# Agent Instructions&#10;&#10;You are an AI coding assistant...&#10;&#10;## Guidelines&#10;- Follow best practices&#10;- Write clean code&#10;- Add comments when necessary"
              value={agentsContent}
              onChange={e => setAgentsContent(e.target.value)}
              disabled={isLoading}
              className="min-h-[200px] font-mono text-sm"
            />
            <p className="text-xs text-muted-foreground">
              编辑此文件以自定义智能体的行为准则和指令。此内容将保存为项目的 AGENTS.md 文件。
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
          <Button onClick={handleSave} disabled={isLoading}>
            {isLoading ? (
              <>
                <Loader2 className="w-4 h-4 mr-2 animate-spin" />
                保存中...
              </>
            ) : (
              '保存'
            )}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
