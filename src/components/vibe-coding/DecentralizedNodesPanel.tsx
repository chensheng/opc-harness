import React from 'react'
import { useDecentralizedNodes } from '../../hooks/useDecentralizedNodes'
import { Play, Square, RefreshCw, Server } from 'lucide-react'

/**
 * 去中心化 Agent Nodes 控制面板
 */
export function DecentralizedNodesPanel() {
  const {
    nodes,
    isLoading,
    error,
    startNode,
    stopNode,
    refreshNodes,
    getRunningCount,
  } = useDecentralizedNodes()

  const handleStartNode = async () => {
    try {
      await startNode()
    } catch (err) {
      console.error('Failed to start node:', err)
    }
  }

  const handleStopNode = async (nodeId: string) => {
    try {
      await stopNode(nodeId)
    } catch (err) {
      console.error('Failed to stop node:', err)
    }
  }

  const handleRefresh = async () => {
    await refreshNodes()
  }

  return (
    <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
      <div className="flex items-center justify-between mb-4">
        <div className="flex items-center gap-2">
          <Server className="w-5 h-5 text-blue-500" />
          <h3 className="text-lg font-semibold text-gray-900 dark:text-white">
            去中心化 Agent Nodes
          </h3>
        </div>
        
        <div className="flex items-center gap-2">
          <span className="text-sm text-gray-500 dark:text-gray-400">
            运行中: {getRunningCount()} / {nodes.length}
          </span>
          <button
            onClick={handleRefresh}
            disabled={isLoading}
            className="p-2 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg transition-colors"
            title="刷新节点列表"
          >
            <RefreshCw className={`w-4 h-4 ${isLoading ? 'animate-spin' : ''}`} />
          </button>
        </div>
      </div>

      {error && (
        <div className="mb-4 p-3 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg">
          <p className="text-sm text-red-600 dark:text-red-400">{error}</p>
        </div>
      )}

      <div className="space-y-3">
        {/* 启动新节点按钮 */}
        <button
          onClick={handleStartNode}
          disabled={isLoading}
          className="w-full flex items-center justify-center gap-2 px-4 py-2 bg-green-500 hover:bg-green-600 disabled:bg-gray-300 text-white rounded-lg transition-colors"
        >
          <Play className="w-4 h-4" />
          启动新 Node
        </button>

        {/* 节点列表 */}
        {nodes.length === 0 ? (
          <div className="text-center py-8 text-gray-500 dark:text-gray-400">
            <p>暂无运行中的 Node</p>
            <p className="text-sm mt-1">点击"启动新 Node"开始</p>
          </div>
        ) : (
          <div className="space-y-2 max-h-96 overflow-y-auto">
            {nodes.map((node) => (
              <div
                key={node.node_id}
                className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-700 rounded-lg"
              >
                <div className="flex items-center gap-3">
                  <div
                    className={`w-2 h-2 rounded-full ${
                      node.is_running ? 'bg-green-500' : 'bg-gray-400'
                    }`}
                  />
                  <div>
                    <p className="text-sm font-mono text-gray-900 dark:text-white">
                      {node.node_id}
                    </p>
                    <p className="text-xs text-gray-500 dark:text-gray-400">
                      {node.is_running ? '运行中' : '已停止'}
                    </p>
                  </div>
                </div>

                {node.is_running && (
                  <button
                    onClick={() => handleStopNode(node.node_id)}
                    disabled={isLoading}
                    className="p-2 hover:bg-red-100 dark:hover:bg-red-900/30 text-red-500 rounded-lg transition-colors"
                    title="停止节点"
                  >
                    <Square className="w-4 h-4" />
                  </button>
                )}
              </div>
            ))}
          </div>
        )}
      </div>

      <div className="mt-4 pt-4 border-t border-gray-200 dark:border-gray-700">
        <p className="text-xs text-gray-500 dark:text-gray-400">
          💡 提示: 每个 Node 独立运行,自主决策是否处理任务
        </p>
      </div>
    </div>
  )
}
