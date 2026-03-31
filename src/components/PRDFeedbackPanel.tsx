/**
 * PRD 反馈面板组件
 */

import React, { useState } from 'react'
import { usePRDFeedback } from '../hooks/usePRDFeedback'

export interface PRDFeedbackPanelProps {
  /** PRD ID */
  prdId: string
  /** PRD 内容 */
  prdContent: string
  /** 当前章节（可选） */
  currentSection?: string
  /** 反馈提交成功后的回调 */
  onFeedbackSubmitted?: (newPrdContent: string) => void
}

export const PRDFeedbackPanel: React.FC<PRDFeedbackPanelProps> = ({
  prdId,
  prdContent,
  currentSection,
  onFeedbackSubmitted,
}) => {
  const [feedbackText, setFeedbackText] = useState('')
  const { isLoading, error, iterationCount, lastResult, submitFeedback, clearError } =
    usePRDFeedback(prdId)

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()

    if (!feedbackText.trim()) {
      return
    }

    const result = await submitFeedback(prdContent, feedbackText, currentSection)

    if (result && result.success) {
      setFeedbackText('')
      if (onFeedbackSubmitted && result.new_prd_content) {
        onFeedbackSubmitted(result.new_prd_content)
      }
    }
  }

  return (
    <div className="bg-white rounded-lg shadow-md p-6">
      {/* 头部 */}
      <div className="mb-4">
        <h2 className="text-xl font-semibold text-gray-800">💡 反馈与重新生成</h2>
        <p className="text-sm text-gray-600 mt-1">
          提供反馈意见，AI 将自动优化 PRD 内容
          {currentSection && (
            <span className="ml-2 px-2 py-1 bg-blue-100 text-blue-700 rounded text-xs">
              当前章节：{currentSection}
            </span>
          )}
        </p>
        {iterationCount > 0 && (
          <p className="text-xs text-gray-500 mt-2">已迭代 {iterationCount} 轮</p>
        )}
      </div>

      {/* 错误提示 */}
      {error && (
        <div className="mb-4 p-3 bg-red-50 border border-red-200 rounded-md">
          <div className="flex items-start">
            <span className="text-red-600 mr-2">❌</span>
            <span className="text-red-700 text-sm flex-1">{error}</span>
            <button onClick={clearError} className="text-red-500 hover:text-red-700 text-sm ml-2">
              ✕
            </button>
          </div>
        </div>
      )}

      {/* 上一次结果 */}
      {lastResult && (
        <div className="mb-4 p-3 bg-green-50 border border-green-200 rounded-md">
          <div className="flex items-start justify-between">
            <div>
              <p className="text-green-800 font-medium text-sm">✅ 重新生成成功</p>
              <div className="mt-2 text-xs text-green-700 space-y-1">
                <p>
                  质量评分：{lastResult.quality_score_before.toFixed(1)} →{' '}
                  {lastResult.quality_score_after.toFixed(1)}
                  <span className="text-green-600 font-medium">
                    {' '}
                    (+
                    {(lastResult.quality_score_after - lastResult.quality_score_before).toFixed(1)})
                  </span>
                </p>
                {lastResult.changed_sections.length > 0 && (
                  <p>变更章节：{lastResult.changed_sections.join(', ')}</p>
                )}
                <p>迭代轮次：第 {lastResult.iteration_number} 轮</p>
              </div>
            </div>
          </div>
        </div>
      )}

      {/* 反馈表单 */}
      <form onSubmit={handleSubmit} className="space-y-3">
        <div>
          <label htmlFor="feedback" className="block text-sm font-medium text-gray-700 mb-1">
            反馈意见
          </label>
          <textarea
            id="feedback"
            value={feedbackText}
            onChange={e => setFeedbackText(e.target.value)}
            placeholder="请描述您希望改进的地方，例如：&#10;- 用户画像部分需要更详细&#10;- 功能需求描述不够清晰&#10;- 技术架构需要补充说明..."
            rows={4}
            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent text-sm"
            disabled={isLoading}
          />
        </div>

        <div className="flex items-center justify-between">
          <p className="text-xs text-gray-500">AI 将根据您的反馈重新生成相关章节</p>
          <button
            type="submit"
            disabled={isLoading || !feedbackText.trim()}
            className={`px-4 py-2 rounded-md text-sm font-medium transition-colors ${
              isLoading || !feedbackText.trim()
                ? 'bg-gray-300 text-gray-500 cursor-not-allowed'
                : 'bg-blue-600 text-white hover:bg-blue-700'
            }`}
          >
            {isLoading ? (
              <span className="flex items-center">
                <svg
                  className="animate-spin -ml-1 mr-2 h-4 w-4 text-white"
                  xmlns="http://www.w3.org/2000/svg"
                  fill="none"
                  viewBox="0 0 24 24"
                >
                  <circle
                    className="opacity-25"
                    cx="12"
                    cy="12"
                    r="10"
                    stroke="currentColor"
                    strokeWidth="4"
                  ></circle>
                  <path
                    className="opacity-75"
                    fill="currentColor"
                    d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                  ></path>
                </svg>
                处理中...
              </span>
            ) : (
              '🔄 重新生成'
            )}
          </button>
        </div>
      </form>

      {/* 使用说明 */}
      <div className="mt-4 p-3 bg-gray-50 rounded-md">
        <p className="text-xs text-gray-600 font-medium mb-2">💡 使用建议：</p>
        <ul className="text-xs text-gray-600 space-y-1 list-disc list-inside">
          <li>尽量具体描述需要改进的内容</li>
          <li>可以针对特定章节或整体 PRD 提供反馈</li>
          <li>支持多轮迭代，逐步完善 PRD</li>
          <li>每轮迭代后质量评分会提升</li>
        </ul>
      </div>
    </div>
  )
}
