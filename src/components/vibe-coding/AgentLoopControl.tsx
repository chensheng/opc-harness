import { useState, useEffect } from 'react'
import { Play, Square, RefreshCw, Activity, FolderGit2, Trash2, HardDrive, Terminal, Eraser } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { useAgentLoop } from '@/hooks/useAgentLoop'
import { useWorktreeManager } from '@/hooks/useAgentLoop'

interface AgentLoopControlProps {
  projectId: string
}

export function AgentLoopControl({ projectId }: AgentLoopControlProps) {
  const { isRunning, isLoading, error, logs, logsEndRef, startAgentLoop, executeOnce, stopAgentLoop, checkStatus, clearLogs } = useAgentLoop()
  const { worktrees, diskUsage, formattedDiskUsage, isLoading: wtLoading, listWorktrees, cleanupOrphaned } = useWorktreeManager(projectId)
  const [lastExecutedAt, setLastExecutedAt] = useState<Date | null>(null)
  const [startedAgentsCount, setStartedAgentsCount] = useState<number>(0)

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
    try {
      await startAgentLoop(projectId, 60)
      setLastExecutedAt(new Date())
    } catch (err) {
      console.error('Failed to start Agent Loop:', err)
    }
  }

  // 执行一次
  const handleExecuteOnce = async () => {
    try {
      const count = await executeOnce(projectId)
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
              disabled={isRunning || isLoading}
              size="sm"
            >
              <Play className="w-4 h-4 mr-2" />
              启动
            </Button>
            <Button
              onClick={handleExecuteOnce}
              disabled={isLoading}
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
            <p>
              <span className="font-medium">当前项目:</span> {projectId}
            </p>
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
          </div>

          {/* 说明 */}
          <div className="text-xs text-muted-foreground border-t pt-3">
            <p>Agent Loop 会自动检测活跃的 Sprint 并执行待处理的用户故事。</p>
            <p>默认每 60 秒检测一次，可通过修改代码调整间隔时间。</p>
          </div>

          {/* 实时日志面板 */}
          <div className="border-t pt-4">
            <div className="flex items-center justify-between mb-2">
              <h4 className="text-sm font-medium flex items-center gap-2">
                <Terminal className="w-4 h-4" />
                运行日志
              </h4>
              <Button
                onClick={clearLogs}
                variant="ghost"
                size="sm"
                className="h-7 px-2"
              >
                <Eraser className="w-3 h-3 mr-1" />
                清空
              </Button>
            </div>
            <div className="bg-gray-50 dark:bg-gray-900 rounded-lg p-3 max-h-64 overflow-y-auto font-mono text-xs space-y-1">
              {logs.length === 0 ? (
                <p className="text-muted-foreground text-center py-4">
                  暂无日志，启动 Agent Loop 后将显示详细运行信息
                </p>
              ) : (
                logs.map((log, index) => (
                  <div
                    key={index}
                    className={`flex gap-2 ${
                      log.type === 'error' ? 'text-red-600 dark:text-red-400' :
                      log.type === 'success' ? 'text-green-600 dark:text-green-400' :
                      'text-gray-700 dark:text-gray-300'
                    }`}
                  >
                    <span className="text-muted-foreground shrink-0">
                      {log.timestamp.toLocaleTimeString()}
                    </span>
                    <span>{log.message}</span>
                  </div>
                ))
              )}
              <div ref={logsEndRef} />
            </div>
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
            <p>Worktree 为每个 Agent 提供独立的 Git 工作树环境，避免并发冲突。</p>
            <p>路径规范: .worktrees/agent-{`{agent_id}`}</p>
          </div>
        </CardContent>
      </Card>
    </div>
  )
}