import React, { useState } from 'react'
import { usePRDIteration } from '@/hooks/usePRDIteration'
import type { PRD } from '@/types'

interface PRDIterationPanelProps {
  prd: PRD
  onPrdUpdate?: (newPrd: PRD) => void
}

/**
 * PRD 迭代优化面板组件（简化版）
 */
export const PRDIterationPanel: React.FC<PRDIterationPanelProps> = ({ prd, onPrdUpdate }) => {
  const { currentVersionId, isIterating, error, createInitialVersion, iterateWithFeedback, reset } =
    usePRDIteration()

  const [feedback, setFeedback] = useState('')
  const [initialized, setInitialized] = useState(false)

  // 初始化版本
  const handleInitialize = async () => {
    try {
      await createInitialVersion(prd)
      setInitialized(true)
    } catch (err) {
      console.error('Failed to initialize:', err)
    }
  }

  // 执行迭代
  const handleIterate = async () => {
    if (!feedback.trim()) return

    try {
      const response = await iterateWithFeedback(prd, feedback)

      // 通知父组件 PRD 已更新
      if (onPrdUpdate) {
        onPrdUpdate(response.optimizedPrd)
      }

      // 清空反馈
      setFeedback('')
    } catch (err) {
      console.error('Failed to iterate:', err)
    }
  }

  return (
    <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
      {/* 标题 */}
      <div className="flex items-center justify-between mb-6">
        <h3 className="text-lg font-semibold text-gray-900">PRD 迭代优化</h3>
        {initialized && (
          <button onClick={reset} className="text-sm text-blue-600 hover:text-blue-700">
            重置
          </button>
        )}
      </div>

      {/* 未初始化状态 */}
      {!initialized ? (
        <div className="text-center py-8">
          <p className="text-gray-600 mb-4">开始迭代优化前，需要先创建初始版本</p>
          <button
            onClick={handleInitialize}
            className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700"
          >
            创建初始版本
          </button>
        </div>
      ) : (
        <div className="space-y-4">
          {/* 当前版本信息 */}
          <div className="bg-blue-50 border border-blue-200 rounded-lg p-3">
            <p className="text-sm text-blue-800">
              <span className="font-medium">当前版本：</span>
              {currentVersionId?.substring(0, 8)}...
            </p>
          </div>

          {/* 反馈输入框 */}
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">反馈意见</label>
            <textarea
              value={feedback}
              onChange={e => setFeedback(e.target.value)}
              placeholder="请输入您的反馈意见，例如：添加更多功能细节、优化用户体验等..."
              className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
              rows={4}
            />
          </div>

          {/* 迭代按钮 */}
          <button
            onClick={handleIterate}
            disabled={isIterating || !feedback.trim()}
            className="w-full px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:bg-gray-400 disabled:cursor-not-allowed"
          >
            {isIterating ? '正在优化...' : '执行迭代优化'}
          </button>

          {/* 错误提示 */}
          {error && (
            <div className="bg-red-50 border border-red-200 rounded-lg p-3">
              <p className="text-sm text-red-800">{error}</p>
            </div>
          )}

          {/* 使用说明 */}
          <div className="bg-gray-50 border border-gray-200 rounded-lg p-4">
            <h4 className="text-sm font-medium text-gray-900 mb-2">使用说明</h4>
            <ul className="text-sm text-gray-600 list-disc list-inside space-y-1">
              <li>输入您的反馈意见</li>
              <li>点击"执行迭代优化"按钮</li>
              <li>系统会自动优化 PRD 内容</li>
              <li>支持多轮迭代（建议至少 3 轮）</li>
            </ul>
          </div>
        </div>
      )}
    </div>
  )
}
