import { useState } from 'react'
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
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'

interface CreateAgentDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  onSuccess?: (agentId: string) => void
}

export function CreateAgentDialog({ open, onOpenChange, onSuccess }: CreateAgentDialogProps) {
  const [agentType, setAgentType] = useState<string>('coding')
  const [sessionId, setSessionId] = useState<string>('')
  const [projectPath, setProjectPath] = useState<string>('')
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const handleCreate = async () => {
    if (!sessionId || !projectPath) {
      setError('请填写所有必填字段')
      return
    }

    setIsLoading(true)
    setError(null)

    try {
      const agentId = await invoke<string>('create_agent', {
        agentType,
        sessionId,
        projectPath,
      })

      onSuccess?.(agentId)
      onOpenChange(false)

      // 重置表单
      setSessionId('')
      setProjectPath('')
      setError(null)
    } catch (err) {
      setError(err instanceof Error ? err.message : '创建智能体失败')
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
      <DialogContent className="sm:max-w-[425px]">
        <DialogHeader>
          <DialogTitle className="flex items-center gap-2">
            <Bot className="w-5 h-5" />
            创建智能体
          </DialogTitle>
          <DialogDescription>配置并创建一个新的 AI 智能体来执行特定任务</DialogDescription>
        </DialogHeader>

        <div className="grid gap-4 py-4">
          {/* Agent Type */}
          <div className="grid gap-2">
            <Label htmlFor="agent-type">智能体类型</Label>
            <Select value={agentType} onValueChange={setAgentType} disabled={isLoading}>
              <SelectTrigger id="agent-type">
                <SelectValue placeholder="选择智能体类型" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="initializer">
                  <div className="flex items-center gap-2">
                    <span>🚀</span>
                    <div>
                      <div className="font-medium">初始化智能体</div>
                      <div className="text-xs text-muted-foreground">项目初始化和任务分解</div>
                    </div>
                  </div>
                </SelectItem>
                <SelectItem value="coding">
                  <div className="flex items-center gap-2">
                    <span>💻</span>
                    <div>
                      <div className="font-medium">编码智能体</div>
                      <div className="text-xs text-muted-foreground">代码生成和修改</div>
                    </div>
                  </div>
                </SelectItem>
                <SelectItem value="mr_creation">
                  <div className="flex items-center gap-2">
                    <span>🔀</span>
                    <div>
                      <div className="font-medium">MR 创建智能体</div>
                      <div className="text-xs text-muted-foreground">合并请求创建和管理</div>
                    </div>
                  </div>
                </SelectItem>
              </SelectContent>
            </Select>
          </div>

          {/* Session ID */}
          <div className="grid gap-2">
            <Label htmlFor="session-id">会话 ID *</Label>
            <Input
              id="session-id"
              placeholder="例如: session-001"
              value={sessionId}
              onChange={e => setSessionId(e.target.value)}
              disabled={isLoading}
            />
            <p className="text-xs text-muted-foreground">用于关联同一会话的多个智能体</p>
          </div>

          {/* Project Path */}
          <div className="grid gap-2">
            <Label htmlFor="project-path">项目路径 *</Label>
            <Input
              id="project-path"
              placeholder="例如: /home/user/my-project"
              value={projectPath}
              onChange={e => setProjectPath(e.target.value)}
              disabled={isLoading}
            />
            <p className="text-xs text-muted-foreground">智能体将在此项目目录下工作</p>
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
