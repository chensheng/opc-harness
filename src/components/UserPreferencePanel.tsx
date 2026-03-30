import React, { useState } from 'react'
import { useUserPreference } from '@/hooks/useUserPreference'
import type { PreferenceModel, Feedback } from '@/types/user-preference'

interface UserPreferencePanelProps {
  onPreferenceChange?: (preferences: PreferenceModel) => void
}

/**
 * 用户偏好面板组件
 */
export const UserPreferencePanel: React.FC<UserPreferencePanelProps> = ({ onPreferenceChange }) => {
  const { preferences, isLoading, error, loadPreferences, updatePreferences, analyzeFromFeedback } =
    useUserPreference()

  const [showAnalyzer, setShowAnalyzer] = useState(false)
  const [feedbackInput, setFeedbackInput] = useState('')

  // 加载偏好
  const handleLoad = async () => {
    try {
      const model = await loadPreferences()
      if (onPreferenceChange) {
        onPreferenceChange(model)
      }
    } catch (err) {
      console.error('Failed to load preferences:', err)
    }
  }

  // 分析反馈
  const handleAnalyze = async () => {
    if (!feedbackInput.trim()) return

    const feedbacks: Feedback[] = [
      {
        content: feedbackInput,
        timestamp: Date.now(),
        feedbackType: 'manual',
      },
    ]

    try {
      const model = await analyzeFromFeedback(feedbacks)
      await updatePreferences(model)
      if (onPreferenceChange) {
        onPreferenceChange(model)
      }
      setFeedbackInput('')
      setShowAnalyzer(false)
    } catch (err) {
      console.error('Failed to analyze:', err)
    }
  }

  // 更新复杂度偏好
  const handleComplexityChange = (value: number) => {
    if (!preferences) return

    const updated: PreferenceModel = {
      ...preferences,
      preferredFeatureComplexity: value,
    }
    updatePreferences(updated)
    if (onPreferenceChange) {
      onPreferenceChange(updated)
    }
  }

  // 更新详细程度偏好
  const handleDetailLevelChange = (value: number) => {
    if (!preferences) return

    const updated: PreferenceModel = {
      ...preferences,
      preferredDetailLevel: value,
    }
    updatePreferences(updated)
    if (onPreferenceChange) {
      onPreferenceChange(updated)
    }
  }

  if (!preferences) {
    return (
      <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
        <h3 className="text-lg font-semibold text-gray-900 mb-4">用户偏好</h3>
        <div className="text-center py-8">
          <p className="text-gray-500 mb-4">暂无偏好数据</p>
          <button
            onClick={handleLoad}
            className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700"
          >
            加载偏好
          </button>
        </div>
      </div>
    )
  }

  return (
    <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
      {/* 标题和操作按钮 */}
      <div className="flex items-center justify-between mb-6">
        <h3 className="text-lg font-semibold text-gray-900">用户偏好</h3>
        <div className="space-x-2">
          <button
            onClick={() => setShowAnalyzer(!showAnalyzer)}
            className="px-3 py-1.5 text-sm bg-purple-600 text-white rounded-lg hover:bg-purple-700"
          >
            {showAnalyzer ? '关闭分析器' : '反馈分析器'}
          </button>
          <button
            onClick={handleLoad}
            disabled={isLoading}
            className="px-3 py-1.5 text-sm bg-gray-100 text-gray-700 rounded-lg hover:bg-gray-200 disabled:bg-gray-300"
          >
            刷新
          </button>
        </div>
      </div>

      {/* 加载状态 */}
      {isLoading && (
        <div className="flex items-center justify-center py-8">
          <div className="animate-spin rounded-full h-6 w-6 border-b-2 border-blue-600"></div>
          <span className="ml-2 text-gray-600">加载中...</span>
        </div>
      )}

      {/* 错误提示 */}
      {error && (
        <div className="mb-4 bg-red-50 border border-red-200 rounded-lg p-3">
          <p className="text-sm text-red-800">{error}</p>
        </div>
      )}

      {/* 偏好展示 */}
      <div className="space-y-6">
        {/* 功能复杂度 */}
        <div>
          <div className="flex items-center justify-between mb-2">
            <label className="text-sm font-medium text-gray-700">功能复杂度偏好</label>
            <span className="text-sm text-gray-600">
              {(preferences.preferredFeatureComplexity * 100).toFixed(0)}%
            </span>
          </div>
          <input
            type="range"
            min="0"
            max="1"
            step="0.1"
            value={preferences.preferredFeatureComplexity}
            onChange={e => handleComplexityChange(parseFloat(e.target.value))}
            className="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer"
          />
          <div className="flex justify-between mt-1 text-xs text-gray-500">
            <span>简单</span>
            <span>复杂</span>
          </div>
        </div>

        {/* 详细程度 */}
        <div>
          <div className="flex items-center justify-between mb-2">
            <label className="text-sm font-medium text-gray-700">详细程度偏好</label>
            <span className="text-sm text-gray-600">
              {(preferences.preferredDetailLevel * 100).toFixed(0)}%
            </span>
          </div>
          <input
            type="range"
            min="0"
            max="1"
            step="0.1"
            value={preferences.preferredDetailLevel}
            onChange={e => handleDetailLevelChange(parseFloat(e.target.value))}
            className="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer"
          />
          <div className="flex justify-between mt-1 text-xs text-gray-500">
            <span>简洁</span>
            <span>详细</span>
          </div>
        </div>

        {/* 技术栈偏好 */}
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">偏好技术栈</label>
          <div className="flex flex-wrap gap-2">
            {preferences.preferredTechStack.length > 0 ? (
              preferences.preferredTechStack.map((tech, index) => (
                <span
                  key={index}
                  className="px-3 py-1 bg-blue-100 text-blue-800 text-sm rounded-full"
                >
                  {tech}
                </span>
              ))
            ) : (
              <span className="text-sm text-gray-500">暂无偏好技术栈</span>
            )}
          </div>
        </div>

        {/* 反馈关键词 */}
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">高频反馈关键词</label>
          <div className="flex flex-wrap gap-2">
            {preferences.feedbackKeywords.length > 0 ? (
              preferences.feedbackKeywords.map((keyword, index) => (
                <span
                  key={index}
                  className="px-3 py-1 bg-purple-100 text-purple-800 text-sm rounded-full"
                >
                  {keyword}
                </span>
              ))
            ) : (
              <span className="text-sm text-gray-500">暂无高频关键词</span>
            )}
          </div>
        </div>
      </div>

      {/* 反馈分析器 */}
      {showAnalyzer && (
        <div className="mt-6 border-t pt-6">
          <h4 className="text-md font-semibold text-gray-900 mb-3">反馈分析器</h4>
          <p className="text-sm text-gray-600 mb-3">输入您的历史反馈，系统将自动学习您的偏好</p>
          <textarea
            value={feedbackInput}
            onChange={e => setFeedbackInput(e.target.value)}
            placeholder="例如：请添加更多功能、使用 React 和 Rust 实现、需要详细说明等..."
            className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-purple-500"
            rows={4}
          />
          <button
            onClick={handleAnalyze}
            disabled={!feedbackInput.trim()}
            className="mt-3 w-full px-4 py-2 bg-purple-600 text-white rounded-lg hover:bg-purple-700 disabled:bg-gray-400 disabled:cursor-not-allowed"
          >
            分析并应用偏好
          </button>
        </div>
      )}
    </div>
  )
}
