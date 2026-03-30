import React, { useState } from 'react'
import { usePRDVersionHistory } from '@/hooks/usePRDVersionHistory'
import type { PRD } from '@/types'

interface PRDVersionHistoryProps {
  currentPrd?: PRD
  onRollback?: (prd: PRD) => void
}

/**
 * PRD 版本历史组件
 */
export const PRDVersionHistory: React.FC<PRDVersionHistoryProps> = ({ currentPrd, onRollback }) => {
  const { history, isLoading, error, loadHistory, compareVersions, rollbackToVersion } =
    usePRDVersionHistory()

  const [selectedVersionId, setSelectedVersionId] = useState<string | null>(null)
  const [compareMode, setCompareMode] = useState(false)
  const [compareVersionIds, setCompareVersionIds] = useState<[string, string] | null>(null)
  const [showRollbackConfirm, setShowRollbackConfirm] = useState(false)

  // 加载历史
  const handleLoadHistory = async () => {
    try {
      await loadHistory()
    } catch (err) {
      console.error('Failed to load history:', err)
    }
  }

  // 选择版本
  const handleSelectVersion = (versionId: string) => {
    if (compareMode) {
      if (compareVersionIds === null) {
        setCompareVersionIds([versionId, versionId])
      } else {
        setCompareVersionIds([compareVersionIds[0], versionId])
      }
    } else {
      setSelectedVersionId(versionId)
    }
  }

  // 回滚确认
  const handleRollbackConfirm = async (versionId: string) => {
    try {
      const version = await rollbackToVersion(versionId)
      if (onRollback) {
        onRollback(version.prd)
      }
      setShowRollbackConfirm(false)
      setSelectedVersionId(null)
    } catch (err) {
      console.error('Rollback failed:', err)
    }
  }

  // 格式化时间
  const formatTimestamp = (timestamp: number) => {
    return new Date(timestamp * 1000).toLocaleString('zh-CN')
  }

  if (!history || history.versions.length === 0) {
    return (
      <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
        <h3 className="text-lg font-semibold text-gray-900 mb-4">版本历史</h3>
        <div className="text-center py-8">
          <p className="text-gray-500 mb-4">暂无版本历史</p>
          <button
            onClick={handleLoadHistory}
            className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700"
          >
            加载历史
          </button>
        </div>
      </div>
    )
  }

  return (
    <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
      {/* 标题和操作按钮 */}
      <div className="flex items-center justify-between mb-6">
        <h3 className="text-lg font-semibold text-gray-900">版本历史</h3>
        <div className="space-x-2">
          <button
            onClick={() => setCompareMode(!compareMode)}
            className={`px-3 py-1.5 text-sm rounded-lg ${
              compareMode ? 'bg-blue-600 text-white' : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
            }`}
          >
            {compareMode ? '取消对比' : '版本对比'}
          </button>
          <button
            onClick={handleLoadHistory}
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

      {/* 版本列表 */}
      <div className="space-y-3">
        {history.versions.map((version, index) => {
          const isCurrentVersion = version.versionId === history.currentVersionId
          const isSelected = selectedVersionId === version.versionId
          const isInCompare = compareVersionIds?.includes(version.versionId)

          return (
            <div
              key={version.versionId}
              onClick={() => handleSelectVersion(version.versionId)}
              className={`border rounded-lg p-4 cursor-pointer transition-all ${
                isCurrentVersion
                  ? 'border-green-500 bg-green-50'
                  : isSelected || isInCompare
                    ? 'border-blue-500 bg-blue-50'
                    : 'border-gray-200 hover:border-gray-300'
              }`}
            >
              <div className="flex items-start justify-between">
                <div className="flex-1">
                  <div className="flex items-center space-x-2 mb-2">
                    <span className="text-sm font-medium text-gray-900">
                      版本 {version.iterationNumber}
                    </span>
                    {isCurrentVersion && (
                      <span className="px-2 py-0.5 text-xs bg-green-200 text-green-800 rounded">
                        当前版本
                      </span>
                    )}
                    {isSelected && (
                      <span className="px-2 py-0.5 text-xs bg-blue-200 text-blue-800 rounded">
                        已选择
                      </span>
                    )}
                    {isInCompare && (
                      <span className="px-2 py-0.5 text-xs bg-purple-200 text-purple-800 rounded">
                        对比中
                      </span>
                    )}
                  </div>

                  <p className="text-xs text-gray-500 mb-2">{formatTimestamp(version.timestamp)}</p>

                  {version.feedback && (
                    <p className="text-sm text-gray-700 bg-gray-50 rounded p-2">
                      反馈：{version.feedback}
                    </p>
                  )}
                </div>

                {!isCurrentVersion && !compareMode && (
                  <button
                    onClick={e => {
                      e.stopPropagation()
                      setShowRollbackConfirm(true)
                      setSelectedVersionId(version.versionId)
                    }}
                    className="px-3 py-1 text-sm bg-red-600 text-white rounded hover:bg-red-700"
                  >
                    回滚
                  </button>
                )}
              </div>
            </div>
          )
        })}
      </div>

      {/* 版本对比结果 */}
      {compareMode && compareVersionIds && compareVersionIds[0] !== compareVersionIds[1] && (
        <div className="mt-6 border-t pt-6">
          <h4 className="text-md font-semibold text-gray-900 mb-3">版本对比结果</h4>
          <div className="bg-gray-50 border border-gray-200 rounded-lg p-4">
            <p className="text-sm text-gray-700">
              对比版本{' '}
              {history.versions.find(v => v.versionId === compareVersionIds[0])?.iterationNumber} vs{' '}
              {history.versions.find(v => v.versionId === compareVersionIds[1])?.iterationNumber}
            </p>
            {/* TODO: 显示详细的差异对比 */}
          </div>
        </div>
      )}

      {/* 回滚确认对话框 */}
      {showRollbackConfirm && selectedVersionId && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white rounded-lg p-6 max-w-md">
            <h4 className="text-lg font-semibold text-gray-900 mb-4">确认回滚</h4>
            <p className="text-gray-700 mb-6">
              确定要回滚到这个版本吗？回滚后将创建一个新的版本记录。
            </p>
            <div className="flex space-x-3">
              <button
                onClick={() => setShowRollbackConfirm(false)}
                className="flex-1 px-4 py-2 bg-gray-100 text-gray-700 rounded-lg hover:bg-gray-200"
              >
                取消
              </button>
              <button
                onClick={() => handleRollbackConfirm(selectedVersionId)}
                className="flex-1 px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700"
              >
                确认回滚
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  )
}
