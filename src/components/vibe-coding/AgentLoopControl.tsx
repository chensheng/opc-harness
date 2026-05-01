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
  const { worktrees, diskUsage, formattedDiskUsage, isLoading: wtLoading, listWorktrees, cleanupOrphaned } = useWorktreeManager()
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