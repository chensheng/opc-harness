import React from 'react'
import { DecentralizedNodesPanel } from './vibe-coding/DecentralizedNodesPanel'

/**
 * 去中心化智能体测试页面
 */
export function DecentralizedTestPage() {
  return (
    <div className="min-h-screen bg-gray-50 dark:bg-gray-900 p-8">
      <div className="max-w-4xl mx-auto">
        <div className="mb-8">
          <h1 className="text-3xl font-bold text-gray-900 dark:text-white mb-2">
            🚀 去中心化 Agent Node 测试
          </h1>
          <p className="text-gray-600 dark:text-gray-400">
            测试单机多实例去中心化智能体系统 (纯内存实现,无需 Redis)
          </p>
        </div>

        <div className="grid gap-6">
          {/* 去中心化 Nodes 控制面板 */}
          <DecentralizedNodesPanel />

          {/* 架构说明 */}
          <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
            <h3 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">
              📐 架构说明
            </h3>
            
            <div className="space-y-3 text-sm text-gray-600 dark:text-gray-400">
              <div className="flex items-start gap-2">
                <span className="text-green-500">✅</span>
                <span><strong>SharedEventBus:</strong> 基于 tokio::sync::broadcast,所有 Node 共享</span>
              </div>
              <div className="flex items-start gap-2">
                <span className="text-green-500">✅</span>
                <span><strong>SharedLockManager:</strong> 基于 DashMap + Mutex,防止多 Node 处理同一 Story</span>
              </div>
              <div className="flex items-start gap-2">
                <span className="text-green-500">✅</span>
                <span><strong>自主决策:</strong> 每个 Node 独立决定是否接受任务 (基于负载、能力等)</span>
              </div>
              <div className="flex items-start gap-2">
                <span className="text-green-500">✅</span>
                <span><strong>零外部依赖:</strong> 纯内存实现,无需安装 Redis</span>
              </div>
            </div>

            <div className="mt-6 p-4 bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg">
              <p className="text-sm text-blue-800 dark:text-blue-300">
                💡 <strong>提示:</strong> 启动多个 Node 后,它们会竞争处理新创建的 Story。
                第一个获取到分布式锁的 Node 将执行任务,其他 Node 会自动放弃。
              </p>
            </div>
          </div>

          {/* 性能对比 */}
          <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
            <h3 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">
              ⚡ 性能优势
            </h3>
            
            <div className="overflow-x-auto">
              <table className="w-full text-sm">
                <thead>
                  <tr className="border-b border-gray-200 dark:border-gray-700">
                    <th className="text-left py-2 px-3 text-gray-700 dark:text-gray-300">指标</th>
                    <th className="text-left py-2 px-3 text-gray-700 dark:text-gray-300">Redis 方案</th>
                    <th className="text-left py-2 px-3 text-gray-700 dark:text-gray-300">纯内存方案</th>
                  </tr>
                </thead>
                <tbody className="text-gray-600 dark:text-gray-400">
                  <tr className="border-b border-gray-100 dark:border-gray-800">
                    <td className="py-2 px-3">事件发布延迟</td>
                    <td className="py-2 px-3">~1-5ms</td>
                    <td className="py-2 px-3 text-green-600 dark:text-green-400">&lt;1μs</td>
                  </tr>
                  <tr className="border-b border-gray-100 dark:border-gray-800">
                    <td className="py-2 px-3">锁获取延迟</td>
                    <td className="py-2 px-3">~1-3ms</td>
                    <td className="py-2 px-3 text-green-600 dark:text-green-400">&lt;1μs</td>
                  </tr>
                  <tr className="border-b border-gray-100 dark:border-gray-800">
                    <td className="py-2 px-3">吞吐量</td>
                    <td className="py-2 px-3">~10K ops/sec</td>
                    <td className="py-2 px-3 text-green-600 dark:text-green-400">~1M ops/sec</td>
                  </tr>
                  <tr>
                    <td className="py-2 px-3">外部依赖</td>
                    <td className="py-2 px-3 text-red-600 dark:text-red-400">需要 Redis</td>
                    <td className="py-2 px-3 text-green-600 dark:text-green-400">零依赖</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>

          {/* 使用示例 */}
          <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
            <h3 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">
              💻 代码示例
            </h3>
            
            <pre className="bg-gray-900 text-gray-100 p-4 rounded-lg overflow-x-auto text-sm">
{`// 在 React 组件中使用
import { useDecentralizedNodes } from './hooks/useDecentralizedNodes'

function MyComponent() {
  const { nodes, startNode, stopNode } = useDecentralizedNodes()

  // 启动新 Node
  const handleStart = async () => {
    const nodeId = await startNode({ 
      nodeId: 'my-node',
      maxConcurrent: 3 
    })
    console.log('Started node:', nodeId)
  }

  // 停止 Node
  const handleStop = async (nodeId) => {
    await stopNode(nodeId)
  }

  return (
    <div>
      <button onClick={handleStart}>启动 Node</button>
      {nodes.map(node => (
        <div key={node.node_id}>
          {node.node_id} - {node.is_running ? '运行中' : '已停止'}
        </div>
      ))}
    </div>
  )
}`}
            </pre>
          </div>
        </div>
      </div>
    </div>
  )
}
