import React from 'react'
import { usePRDQualityCheck } from '@/hooks/usePRDQualityCheck'
import type { PRD } from '@/types'

interface PRDQualityCheckPanelProps {
  prd: PRD
  onScoreChange?: (score: number) => void
}

/**
 * PRD 质量检查面板组件
 */
export const PRDQualityCheckPanel: React.FC<PRDQualityCheckPanelProps> = ({
  prd,
  onScoreChange,
}) => {
  const { report, isChecking, error, checkQuality, reset } = usePRDQualityCheck()

  // 自动检查质量
  React.useEffect(() => {
    if (prd) {
      checkQuality(prd)
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [prd])

  // 通知父组件评分变化
  React.useEffect(() => {
    if (report?.overallScore !== undefined && onScoreChange) {
      onScoreChange(report.overallScore)
    }
  }, [report?.overallScore, onScoreChange])

  // 获取评分等级和颜色
  const getScoreGrade = (score: number | null) => {
    if (score === null) return { grade: '-', color: 'text-gray-400' }

    if (score >= 90) return { grade: 'A', color: 'text-green-600' }
    if (score >= 80) return { grade: 'B', color: 'text-blue-600' }
    if (score >= 70) return { grade: 'C', color: 'text-yellow-600' }
    if (score >= 60) return { grade: 'D', color: 'text-orange-600' }
    return { grade: 'F', color: 'text-red-600' }
  }

  const grade = getScoreGrade(report?.overallScore ?? null)

  return (
    <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
      {/* 标题 */}
      <div className="flex items-center justify-between mb-6">
        <h3 className="text-lg font-semibold text-gray-900">PRD 质量检查</h3>
        {report && (
          <button onClick={reset} className="text-sm text-blue-600 hover:text-blue-700">
            重新检查
          </button>
        )}
      </div>

      {/* 加载状态 */}
      {isChecking && (
        <div className="flex items-center justify-center py-12">
          <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
          <span className="ml-3 text-gray-600">正在检查 PRD 质量...</span>
        </div>
      )}

      {/* 错误状态 */}
      {error && (
        <div className="bg-red-50 border border-red-200 rounded-lg p-4">
          <div className="flex items-start">
            <div className="flex-shrink-0">
              <svg className="h-5 w-5 text-red-400" fill="currentColor" viewBox="0 0 20 20">
                <path
                  fillRule="evenodd"
                  d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z"
                  clipRule="evenodd"
                />
              </svg>
            </div>
            <div className="ml-3">
              <p className="text-sm text-red-800">{error}</p>
            </div>
          </div>
        </div>
      )}

      {/* 检查结果 */}
      {report && !isChecking && !error && (
        <div className="space-y-6">
          {/* 总体评分 */}
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm font-medium text-gray-600">总体评分</p>
              <p className={`text-3xl font-bold ${grade.color}`}>
                {report.overallScore}
                <span className="text-lg ml-1">/ 100</span>
              </p>
            </div>
            <div className={`text-5xl font-bold ${grade.color}`}>{grade.grade}</div>
          </div>

          {/* 完整性检查 */}
          <div>
            <h4 className="text-md font-semibold text-gray-900 mb-3">完整性检查</h4>
            <div className="bg-gray-50 rounded-lg p-4">
              <div className="flex items-center justify-between mb-2">
                <span className="text-sm text-gray-600">完整性得分</span>
                <span className="text-sm font-semibold text-gray-900">
                  {report.completeness.score ?? 0} / 100
                </span>
              </div>
              {report.completeness.missingSections.length > 0 && (
                <div className="mt-3">
                  <p className="text-sm font-medium text-red-600 mb-2">缺失的章节:</p>
                  <ul className="list-disc list-inside space-y-1">
                    {report.completeness.missingSections.map((section, idx) => (
                      <li key={idx} className="text-sm text-gray-700">
                        {section}
                      </li>
                    ))}
                  </ul>
                </div>
              )}
            </div>
          </div>

          {/* 质量问题 */}
          {report.issues.length > 0 && (
            <div>
              <h4 className="text-md font-semibold text-gray-900 mb-3">
                发现的质量问题 ({report.issues.length})
              </h4>
              <div className="space-y-2">
                {report.issues.map((issue, idx) => (
                  <div
                    key={idx}
                    className={`border-l-4 rounded p-3 ${
                      issue.severity === 'critical'
                        ? 'border-red-500 bg-red-50'
                        : issue.severity === 'major'
                          ? 'border-yellow-500 bg-yellow-50'
                          : 'border-blue-500 bg-blue-50'
                    }`}
                  >
                    <div className="flex items-start justify-between">
                      <div className="flex-1">
                        <p className="text-sm font-medium text-gray-900">
                          {issue.section}: {issue.description}
                        </p>
                        {issue.suggestion && (
                          <p className="text-xs text-gray-600 mt-1">💡 {issue.suggestion}</p>
                        )}
                      </div>
                      <span
                        className={`text-xs px-2 py-1 rounded ${
                          issue.severity === 'critical'
                            ? 'bg-red-200 text-red-800'
                            : issue.severity === 'major'
                              ? 'bg-yellow-200 text-yellow-800'
                              : 'bg-blue-200 text-blue-800'
                        }`}
                      >
                        {issue.severity === 'critical'
                          ? '严重'
                          : issue.severity === 'major'
                            ? '重要'
                            : '轻微'}
                      </span>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          )}

          {/* 改进建议 */}
          {report.suggestions.length > 0 && (
            <div>
              <h4 className="text-md font-semibold text-gray-900 mb-3">改进建议</h4>
              <ul className="list-disc list-inside space-y-1">
                {report.suggestions.map((suggestion, idx) => (
                  <li key={idx} className="text-sm text-gray-700">
                    {suggestion}
                  </li>
                ))}
              </ul>
            </div>
          )}
        </div>
      )}
    </div>
  )
}
