import React, { useState } from 'react'
import { useAgentWorkers } from '../hooks/useAgentWorkers'

/**
 * 完全去中心化 Agent Worker 测试页面
 */
export function DecentralizedWorkerTestPage() {
  const {
    workers,
    isLoading,
    error,
    startWorker,
    stopWorker,
    refreshWorkers,
    getRunningCount,
    getBusyCount,
  } = useAgentWorkers()

  const [newWorkerId, setNewWorkerId] = useState('')
  const [projectId, setProjectId] = useState('')
  const [checkInterval, setCheckInterval] = useState(30)

  const handleStartWorker = async () => {
    if (!projectId) {
      alert('请输入 Project ID')
      return
    }

    try {
      await startWorker({
        worker_id: newWorkerId || undefined,
        project_id: projectId,
        check_interval: checkInterval,
      })
      setNewWorkerId('')
    } catch (err) {
      console.error('Failed to start worker:', err)
    }
  }

  const handleStopWorker = async (workerId: string) => {
    try {
      await stopWorker(workerId)
    } catch (err) {
      console.error('Failed to stop worker:', err)
    }
  }

  return (
    <div className="min-h-screen bg-gray-50 dark:bg-gray-900 p-8">
      <div className="max-w-6xl mx-auto">
        {/* 页面标题 */}
        <div className="mb-8">
          <h1 className="text-3xl font-bold text-gray-900 dark:text-white mb-2">
            🚀 完全去中心化 Agent Worker 测试
          </h1>
          <p className="text-gray-600 dark:text-gray-400">
            每个 Worker 拥有独立的 Agent Loop，通过数据库查询和乐观锁竞争领取 Story
          </p>
        </div>

        {/* 统计信息 */}
        <div className="grid grid-cols-3 gap-4 mb-6">
          <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-4">
            <div className="text-sm text-gray-500 dark:text-gray-400">总 Worker 数</div>
            <div className="text-2xl font-bold text-gray-900 dark:text-white">{workers.length}</div>
          </div>
          <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-4">
            <div className="text-sm text-gray-500 dark:text-gray-400">运行中</div>
            <div className="text-2xl font-bold text-green-600">{getRunningCount()}</div>
          </div>
          <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-4">
            <div className="text-sm text-gray-500 dark:text-gray-400">处理 Story 中</div>
            <div className="text-2xl font-bold text-blue-600">{getBusyCount()}</div>
          </div>
        </div>

        {/* 启动新 Worker */}
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6 mb-6">
          <h3 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">
            ➕ 启动新 Worker
          </h3>

          <div className="grid grid-cols-4 gap-4">
            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                Worker ID (可选)
              </label>
              <input
                type="text"
                value={newWorkerId}
                onChange={e => setNewWorkerId(e.target.value)}
                placeholder="留空自动生成"
                className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
              />
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                Project ID *
              </label>
              <input
                type="text"
                value={projectId}
                onChange={e => setProjectId(e.target.value)}
                placeholder="输入项目 ID"
                className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
              />
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                检查间隔 (秒)
              </label>
              <input
                type="number"
                value={checkInterval}
                onChange={e => setCheckInterval(Number(e.target.value))}
                min="10"
                max="300"
                className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
              />
            </div>

            <div className="flex items-end">
              <button
                onClick={handleStartWorker}
                disabled={isLoading || !projectId}
                className="w-full px-4 py-2 bg-blue-600 hover:bg-blue-700 disabled:bg-gray-400 text-white rounded-md transition-colors"
              >
                {isLoading ? '启动中...' : '启动 Worker'}
              </button>
            </div>
          </div>
        </div>

        {/* Worker 列表 */}
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
          <div className="flex justify-between items-center mb-4">
            <h3 className="text-lg font-semibold text-gray-900 dark:text-white">📋 Worker 列表</h3>
            <button
              onClick={refreshWorkers}
              className="px-3 py-1 text-sm bg-gray-200 dark:bg-gray-700 hover:bg-gray-300 dark:hover:bg-gray-600 rounded-md transition-colors"
            >
              刷新
            </button>
          </div>

          {error && (
            <div className="mb-4 p-3 bg-red-100 dark:bg-red-900/20 border border-red-300 dark:border-red-800 rounded-md text-red-800 dark:text-red-300">
              {error}
            </div>
          )}

          {workers.length === 0 ? (
            <div className="text-center py-8 text-gray-500 dark:text-gray-400">
              暂无 Worker，请启动新的 Worker
            </div>
          ) : (
            <div className="space-y-3">
              {workers.map(worker => (
                <div
                  key={worker.worker_id}
                  className="border border-gray-200 dark:border-gray-700 rounded-lg p-4 hover:shadow-md transition-shadow"
                >
                  <div className="flex justify-between items-start">
                    <div className="flex-1">
                      <div className="flex items-center gap-2 mb-2">
                        <span className="font-mono text-sm text-gray-900 dark:text-white">
                          {worker.worker_id}
                        </span>
                        <span
                          className={`px-2 py-1 text-xs rounded-full ${
                            worker.is_running
                              ? 'bg-green-100 dark:bg-green-900/30 text-green-800 dark:text-green-300'
                              : 'bg-gray-100 dark:bg-gray-700 text-gray-600 dark:text-gray-400'
                          }`}
                        >
                          {worker.is_running ? '运行中' : '已停止'}
                        </span>
                        {worker.current_story_id && (
                          <span className="px-2 py-1 text-xs rounded-full bg-blue-100 dark:bg-blue-900/30 text-blue-800 dark:text-blue-300">
                            处理中: {worker.current_story_id}
                          </span>
                        )}
                      </div>

                      <div className="text-sm text-gray-600 dark:text-gray-400">
                        {worker.current_story_id ? (
                          <span>
                            正在处理 Story: <strong>{worker.current_story_id}</strong>
                          </span>
                        ) : (
                          <span>空闲，等待新任务...</span>
                        )}
                      </div>
                    </div>

                    <button
                      onClick={() => handleStopWorker(worker.worker_id)}
                      disabled={!worker.is_running}
                      className="ml-4 px-3 py-1 text-sm bg-red-600 hover:bg-red-700 disabled:bg-gray-400 text-white rounded-md transition-colors"
                    >
                      停止
                    </button>
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>

        {/* 架构说明 */}
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6 mt-6">
          <h3 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">📐 架构说明</h3>

          <div className="space-y-3 text-sm text-gray-600 dark:text-gray-400">
            <div className="flex items-start gap-2">
              <span className="text-green-500">✅</span>
              <span>
                <strong>完全独立:</strong> 每个 Worker 拥有完整的 Agent Loop，不依赖中心调度器
              </span>
            </div>
            <div className="flex items-start gap-2">
              <span className="text-green-500">✅</span>
              <span>
                <strong>数据库查询:</strong> Worker 定时查询数据库获取活跃 Sprint 和待处理的 User
                Stories
              </span>
            </div>
            <div className="flex items-start gap-2">
              <span className="text-green-500">✅</span>
              <span>
                <strong>乐观锁:</strong> 多个 Worker 通过数据库乐观锁竞争领取同一个 Story
              </span>
            </div>
            <div className="flex items-start gap-2">
              <span className="text-green-500">✅</span>
              <span>
                <strong>零外部依赖:</strong> 纯内存实现，无需 Redis 或 EventBus
              </span>
            </div>
            <div className="flex items-start gap-2">
              <span className="text-green-500">✅</span>
              <span>
                <strong>自动状态更新:</strong> 任务完成后自动更新 Story 状态为 completed 或 failed
              </span>
            </div>
          </div>

          <div className="mt-6 p-4 bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg">
            <p className="text-sm text-blue-800 dark:text-blue-300">
              💡 <strong>提示:</strong> 启动多个 Worker 后，它们会独立查询数据库并竞争处理新创建的
              Story。 第一个成功获取乐观锁的 Worker 将执行任务，其他 Worker 会自动跳过该 Story
              并继续等待下一个。
            </p>
          </div>
        </div>

        {/* 工作流程图 */}
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6 mt-6">
          <h3 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">🔄 工作流程</h3>

          <div className="space-y-2 text-sm font-mono text-gray-700 dark:text-gray-300">
            <div>1. Worker 定时查询数据库 (每 {checkInterval} 秒)</div>
            <div>
              {' '}
              └─ SELECT * FROM user_stories WHERE status IN ('draft', 'refined', 'approved')
            </div>
            <div>2. 尝试乐观锁锁定 Story</div>
            <div>
              {' '}
              └─ UPDATE user_stories SET locked_by='worker-id' WHERE id=? AND (locked_by IS NULL OR
              locked_at &lt; NOW()-30min)
            </div>
            <div>3. 锁定成功 → 创建 Worktree → 启动 AI CLI → 执行任务</div>
            <div>4. 任务完成 → Git Commit & Push → 更新 Story 状态</div>
            <div>5. 回到步骤 1，继续等待新任务</div>
          </div>
        </div>
      </div>
    </div>
  )
}
