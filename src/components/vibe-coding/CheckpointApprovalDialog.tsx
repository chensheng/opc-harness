import { useState, useEffect } from 'react'
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogFooter,
} from '@/components/ui/dialog'
import { Button } from '@/components/ui/button'
import { Textarea } from '@/components/ui/textarea'
import { Badge } from '@/components/ui/badge'
import { Card } from '@/components/ui/card'
import { ScrollArea } from '@/components/ui/scroll-area'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import {
  CheckCircle,
  XCircle,
  Clock,
  AlertTriangle,
  FileCode,
  Package,
  TestTube,
  GitCommit,
  MessageSquare,
} from 'lucide-react'

// Checkpoint 类型定义
export interface CheckpointData {
  id: string
  agent_id: string
  story_id: string
  checkpoint_type:
    | 'code_generation'
    | 'dependency_installation'
    | 'test_execution'
    | 'commit_review'
  status: 'pending' | 'approved' | 'rejected' | 'timed_out'
  data: {
    title: string
    description: string
    payload?: Record<string, unknown>
    timeout_secs: number
  }
  created_at: string
  expires_at?: string
  user_decision?: 'approve' | 'reject'
  user_feedback?: string
  resolved_at?: string
}

interface CheckpointApprovalDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  checkpoint: CheckpointData | null
  onApprove: (checkpointId: string, feedback?: string) => Promise<void>
  onReject: (checkpointId: string, feedback?: string) => Promise<void>
  onApproveAll?: () => Promise<void>
  pendingCount?: number
}

export function CheckpointApprovalDialog({
  open,
  onOpenChange,
  checkpoint,
  onApprove,
  onReject,
  onApproveAll,
  pendingCount = 0,
}: CheckpointApprovalDialogProps) {
  const [feedback, setFeedback] = useState('')
  const [isSubmitting, setIsSubmitting] = useState(false)
  const [timeRemaining, setTimeRemaining] = useState<number | null>(null)

  // 计算剩余时间
  useEffect(() => {
    if (!checkpoint?.expires_at) {
      setTimeRemaining(null)
      return
    }

    const updateTimer = () => {
      const expiresAt = new Date(checkpoint.expires_at!).getTime()
      const now = Date.now()
      const remaining = Math.max(0, Math.floor((expiresAt - now) / 1000))
      setTimeRemaining(remaining)
    }

    updateTimer()
    const interval = setInterval(updateTimer, 1000)
    return () => clearInterval(interval)
  }, [checkpoint?.expires_at])

  // 格式化剩余时间
  const formatTimeRemaining = (seconds: number) => {
    const mins = Math.floor(seconds / 60)
    const secs = seconds % 60
    return `${mins}:${secs.toString().padStart(2, '0')}`
  }

  // 判断是否超时
  const isTimedOut = timeRemaining === 0

  // 获取 checkpoint 类型图标和标签
  const getCheckpointTypeInfo = (type: string) => {
    switch (type) {
      case 'code_generation':
        return { icon: FileCode, label: '代码生成', color: 'text-blue-500' }
      case 'dependency_installation':
        return { icon: Package, label: '依赖安装', color: 'text-purple-500' }
      case 'test_execution':
        return { icon: TestTube, label: '测试执行', color: 'text-green-500' }
      case 'commit_review':
        return { icon: GitCommit, label: '提交审核', color: 'text-orange-500' }
      default:
        return { icon: AlertTriangle, label: type, color: 'text-gray-500' }
    }
  }

  const handleApprove = async () => {
    if (!checkpoint) return
    setIsSubmitting(true)
    try {
      await onApprove(checkpoint.id, feedback || undefined)
      setFeedback('')
      onOpenChange(false)
    } catch (error) {
      console.error('Failed to approve checkpoint:', error)
    } finally {
      setIsSubmitting(false)
    }
  }

  const handleReject = async () => {
    if (!checkpoint) return
    setIsSubmitting(true)
    try {
      await onReject(checkpoint.id, feedback || undefined)
      setFeedback('')
      onOpenChange(false)
    } catch (error) {
      console.error('Failed to reject checkpoint:', error)
    } finally {
      setIsSubmitting(false)
    }
  }

  if (!checkpoint) return null

  const typeInfo = getCheckpointTypeInfo(checkpoint.checkpoint_type)
  const TypeIcon = typeInfo.icon

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-4xl max-h-[80vh] overflow-hidden flex flex-col">
        <DialogHeader>
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-3">
              <TypeIcon className={`w-6 h-6 ${typeInfo.color}`} />
              <div>
                <DialogTitle>{checkpoint.data.title}</DialogTitle>
                <DialogDescription className="mt-1">
                  {checkpoint.data.description}
                </DialogDescription>
              </div>
            </div>
            <div className="flex items-center gap-2">
              <Badge variant="outline" className={typeInfo.color}>
                {typeInfo.label}
              </Badge>
              <Badge variant={checkpoint.status === 'pending' ? 'default' : 'secondary'}>
                {checkpoint.status === 'pending' ? '待审批' : checkpoint.status}
              </Badge>
            </div>
          </div>
        </DialogHeader>

        {/* 超时警告 */}
        {isTimedOut && (
          <Card className="p-3 bg-red-50 border-red-200 mb-4">
            <div className="flex items-center gap-2 text-red-700">
              <AlertTriangle className="w-5 h-5" />
              <span className="font-medium">已超时</span>
              <span className="text-sm">此检查点已超时，请重新触发或联系管理员</span>
            </div>
          </Card>
        )}

        {/* 计时器 */}
        {!isTimedOut && timeRemaining !== null && (
          <Card className="p-3 bg-yellow-50 border-yellow-200 mb-4">
            <div className="flex items-center gap-2 text-yellow-700">
              <Clock className="w-5 h-5" />
              <span className="font-medium">剩余时间：</span>
              <span className="font-mono text-lg">{formatTimeRemaining(timeRemaining)}</span>
            </div>
          </Card>
        )}

        {/* 主要内容区域 */}
        <ScrollArea className="flex-1 pr-4">
          <Tabs defaultValue="details" className="w-full">
            <TabsList className="mb-4">
              <TabsTrigger value="details">详细信息</TabsTrigger>
              <TabsTrigger value="payload">数据预览</TabsTrigger>
              <TabsTrigger value="history">操作历史</TabsTrigger>
            </TabsList>

            <TabsContent value="details" className="space-y-4">
              <Card className="p-4">
                <h3 className="font-semibold mb-2">检查点详情</h3>
                <div className="space-y-2 text-sm">
                  <div className="flex justify-between">
                    <span className="text-muted-foreground">Agent ID:</span>
                    <span className="font-mono">{checkpoint.agent_id}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-muted-foreground">Story ID:</span>
                    <span className="font-mono">{checkpoint.story_id}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-muted-foreground">创建时间:</span>
                    <span>{new Date(checkpoint.created_at).toLocaleString('zh-CN')}</span>
                  </div>
                  {checkpoint.expires_at && (
                    <div className="flex justify-between">
                      <span className="text-muted-foreground">过期时间:</span>
                      <span>{new Date(checkpoint.expires_at).toLocaleString('zh-CN')}</span>
                    </div>
                  )}
                  {checkpoint.resolved_at && (
                    <div className="flex justify-between">
                      <span className="text-muted-foreground">解决时间:</span>
                      <span>{new Date(checkpoint.resolved_at).toLocaleString('zh-CN')}</span>
                    </div>
                  )}
                </div>
              </Card>

              <Card className="p-4">
                <h3 className="font-semibold mb-2">操作说明</h3>
                <p className="text-sm text-muted-foreground">
                  {checkpoint.checkpoint_type === 'code_generation' &&
                    'AI 已完成代码生成，请审查生成的代码是否符合需求。批准后将继续执行后续步骤，拒绝将撤销更改并重新生成。'}
                  {checkpoint.checkpoint_type === 'dependency_installation' &&
                    '即将安装新的依赖包，请确认依赖的必要性和版本兼容性。批准后将继续安装，拒绝将跳过此依赖。'}
                  {checkpoint.checkpoint_type === 'test_execution' &&
                    '即将执行测试套件，请确认测试配置正确。批准后将继续执行测试，拒绝将跳过测试。'}
                  {checkpoint.checkpoint_type === 'commit_review' &&
                    '即将提交代码更改，请审查提交信息和更改内容。批准后将继续提交，拒绝将撤销暂存的更改。'}
                </p>
              </Card>
            </TabsContent>

            <TabsContent value="payload" className="space-y-4">
              <Card className="p-4">
                <h3 className="font-semibold mb-2">Payload 数据</h3>
                <pre className="bg-muted p-3 rounded text-xs overflow-auto max-h-64">
                  {JSON.stringify(checkpoint.data.payload, null, 2)}
                </pre>
              </Card>
            </TabsContent>

            <TabsContent value="history" className="space-y-4">
              <Card className="p-4">
                <h3 className="font-semibold mb-2">操作历史</h3>
                {checkpoint.user_decision ? (
                  <div className="space-y-2">
                    <div className="flex items-center gap-2">
                      {checkpoint.user_decision === 'approve' ? (
                        <CheckCircle className="w-5 h-5 text-green-500" />
                      ) : (
                        <XCircle className="w-5 h-5 text-red-500" />
                      )}
                      <span className="font-medium">
                        {checkpoint.user_decision === 'approve' ? '已批准' : '已拒绝'}
                      </span>
                    </div>
                    {checkpoint.user_feedback && (
                      <div className="ml-7">
                        <span className="text-sm text-muted-foreground">反馈：</span>
                        <p className="text-sm mt-1">{checkpoint.user_feedback}</p>
                      </div>
                    )}
                  </div>
                ) : (
                  <p className="text-sm text-muted-foreground">暂无操作记录</p>
                )}
              </Card>
            </TabsContent>
          </Tabs>
        </ScrollArea>

        {/* 反馈输入框 */}
        <div className="mt-4 space-y-2">
          <label className="text-sm font-medium flex items-center gap-2">
            <MessageSquare className="w-4 h-4" />
            反馈意见（可选）
          </label>
          <Textarea
            placeholder="请输入您的反馈意见..."
            value={feedback}
            onChange={e => setFeedback(e.target.value)}
            rows={3}
            disabled={isSubmitting || isTimedOut}
          />
        </div>

        <DialogFooter className="gap-2 sm:gap-0">
          {onApproveAll && pendingCount > 1 && (
            <Button
              variant="outline"
              onClick={() => onApproveAll()}
              disabled={isSubmitting || isTimedOut}
            >
              全部批准 ({pendingCount})
            </Button>
          )}
          <Button
            variant="destructive"
            onClick={handleReject}
            disabled={isSubmitting || isTimedOut}
          >
            <XCircle className="w-4 h-4 mr-2" />
            拒绝
          </Button>
          <Button onClick={handleApprove} disabled={isSubmitting || isTimedOut}>
            <CheckCircle className="w-4 h-4 mr-2" />
            批准
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
