import { useState, useEffect } from 'react'
import { Play, Square, RefreshCw, Activity, FolderGit2, Trash2, HardDrive } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { useAgentLoop } from '@/hooks/useAgentLoop'
import { useWorktreeManager } from '@/hooks/useAgentLoop'
import { useProjectStore } from '@/stores'

export function AgentLoopControl() {
  const { isRunning, isLoading, error, startAgentLoop, executeOnce, stopAgentLoop, checkStatus } = useAgentLoop()
  const { worktrees, diskUsage, formattedDiskUsage, isLoading: wtLoading, listWorktrees, cleanupOrphaned } = useWorktreeManager()
  const { projects } = useProjectStore()
  const [lastExecutedAt, setLastExecutedAt] = useState<Date | null>(null)
  const [startedAgentsCount, setStartedAgentsCount] = useState<number>(0)

  // 获取当前项目 ID
  const currentProjectId = projects.length > 0 ? projects[0].id : null

  // 定期检查状态
  useEffect(() => {
    const interval = setInterval(() => {
      checkStatus()
    }, 5000) // 每 5 秒检查一次

    return () => clearInterval(interval)
  }, [checkStatus])

  // 加载 Worktree 列表
  useEffect(() => {
    listWorktrees()
  }, [listWorktrees])

  // 启动 Agent Loop
  const handleStart = async () => {
    if (!currentProjectId) {
      alert('请先选择一个项目')
      return
    }

    try {
      await startAgentLoop(currentProjectId, 60)
      setLastExecutedAt(new Date())
    } catch (err) {
      console.error('Failed to start Agent Loop:', err)
    }
  }

  // 执行一次
  const handleExecuteOnce = async () => {
    if (!currentProjectId) {
      alert('请先选择一个项目')
      return
    }

    try {
      const count = await executeOnce(currentProjectId)
      setStartedAgentsCount(count)
      setLastExecutedAt(new Date())
    } catch (err) {
      console.error('Failed to execute Agent Loop:', err)
    }
  }

  // 停止 Agent Loop
  const handleStop = async () => {
    try {
      await stopAgentLoop()
    } catch (err) {
      console.error('Failed to stop Agent Loop:', err)
    }
  }

  // 清理孤立 Worktrees
  const handleCleanup = async () => {
    try {
      const count = await cleanupOrphaned()
      alert(`已清理 ${count} 个孤立的 Worktrees`)
    } catch (err) {
      console.error('Failed to cleanup worktrees:', err)
    }
  }

  return (
    <div className="space-y-4">
      {/* Agent Loop 控制面板 */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Activity className="w-5 h-5" />
            Agent Loop 自动化执行引擎
            {isRunning && (
              <Badge variant="default" className="bg-green-500">
                运行中
              </Badge>
            )}
            {!isRunning && (
              <Badge variant="secondary">已停止</Badge>
            )}
          </CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          {/* 控制按钮 */}
          <div className="flex gap-2">
            <Button
              onClick={handleStart}
              disabled={isRunning || isLoading || !currentProjectId}
              size="sm"
            >
              <Play className="w-4 h-4 mr-2" />
              启动
            </Button>
            <Button
              onClick={handleExecuteOnce}
              disabled={isLoading || !currentProjectId}
              variant="outline"
              size="sm"
            >
              <RefreshCw className="w-4 h-4 mr-2" />
              执行一次
            </Button>
            <Button
              onClick={handleStop}
              disabled={!isRunning || isLoading}
              variant="destructive"
              size="sm"
            >
              <Square className="w-4 h-4 mr-2" />
              停止
            </Button>
          </div>

          {/* 状态信息 */}
          <div className="text-sm space-y-2">
            {currentProjectId && (
              <p>
                <span className="font-medium">当前项目:</span> {currentProjectId}
              </p>
            )}
            {lastExecutedAt && (
              <p>
                <span className="font-medium">最后执行:</span>{' '}
                {lastExecutedAt.toLocaleTimeString()}
              </p>
            )}
            {startedAgentsCount > 0 && (
              <p>
                <span className="font-medium">上次启动 Agent 数:</span> {startedAgentsCount}
              </p>
            )}
            {error && (
              <p className="text-red-500">
                <span className="font-medium">错误:</span> {error}
              </p>
            )}
            {!currentProjectId && (
              <p className="text-yellow-600">
                提示: 请先创建或选择一个项目以启用 Agent Loop
              </p>
            )}
          </div>

          {/* 说明 */}
          <div className="text-xs text-muted-foreground border-t pt-3">
            <p>Agent Loop 会自动检测活跃的 Sprint 并执行待处理的用户故事。</p>
            <p>默认每 60 秒检测一次,可通过修改代码调整间隔时间。</p>
          </div>
        </CardContent>
      </Card>

      {/* Worktree 管理面板 */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <FolderGit2 className="w-5 h-5" />
              Worktree 管理器
            </div>
            <Badge variant="outline">
              {worktrees.length} 个 Worktrees
            </Badge>
          </CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          {/* 磁盘使用量 */}
          <div className="flex items-center gap-2 text-sm">
            <HardDrive className="w-4 h-4 text-muted-foreground" />
            <span>磁盘使用量:</span>
            <span className="font-medium">{formattedDiskUsage}</span>
          </div>

          {/* 操作按钮 */}
          <div className="flex gap-2">
            <Button
              onClick={() => listWorktrees()}
              disabled={wtLoading}
              variant="outline"
              size="sm"
            >
              <RefreshCw className="w-4 h-4 mr-2" />
              刷新
            </Button>
            <Button
              onClick={handleCleanup}
              disabled={wtLoading || worktrees.length === 0}
              variant="outline"
              size="sm"
            >
              <Trash2 className="w-4 h-4 mr-2" />
              清理孤立 Worktrees
            </Button>
          </div>

          {/* Worktree 列表 */}
          {worktrees.length > 0 ? (
            <div className="space-y-2 max-h-60 overflow-y-auto">
              {worktrees.map(wt => (
                <div
                  key={wt.id}
                  className="flex items-center justify-between p-3 border rounded-lg text-sm"
                >
                  <div className="flex-1">
                    <p className="font-medium">{wt.id}</p>
                    <p className="text-xs text-muted-foreground truncate">
                      {wt.path}
                    </p>
                    <p className="text-xs text-muted-foreground">
                      分支: {wt.branch || 'N/A'}
                    </p>
                  </div>
                  {wt.is_orphaned && (
                    <Badge variant="destructive" className="ml-2">
                      孤立
                    </Badge>
                  )}
                </div>
              ))}
            </div>
          ) : (
            <div className="text-center py-8 text-muted-foreground text-sm">
              <p>暂无 Worktrees</p>
              <p className="text-xs mt-1">
                Worktrees 会在 Agent Loop 执行时自动创建
              </p>
            </div>
          )}

          {/* 说明 */}
          <div className="text-xs text-muted-foreground border-t pt-3">
            <p>Worktree 为每个 Agent 提供独立的 Git 工作树环境,避免并发冲突。</p>
            <p>路径规范: .worktrees/agent-{`{agent_id}`}</p>
          </div>
        </CardContent>
      </Card>
    </div>
  )
}
