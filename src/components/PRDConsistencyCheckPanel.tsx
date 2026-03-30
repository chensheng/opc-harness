import React from 'react'
import { usePRDConsistencyCheck } from '@/hooks/usePRDConsistencyCheck'
import type { PRD } from '@/types'

interface PRDConsistencyCheckPanelProps {
  prd: PRD
  onScoreChange?: (score: number) => void
}

/**
 * PRD 一致性检查面板组件
 */
export const PRDConsistencyCheckPanel: React.FC<PRDConsistencyCheckPanelProps> = ({
  prd,
  onScoreChange,
}) => {
  const { report, isChecking, error, checkConsistency, reset } = usePRDConsistencyCheck()

  // 自动检查一致性
  React.useEffect(() => {
    if (prd) {
      checkConsistency(prd)
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [prd])

  // 通知父组件评分变化
  React.useEffect(() => {
    if (report?.overall_score !== undefined && onScoreChange) {
      onScoreChange(report.overall_score)
    }
  }, [report?.overall_score, onScoreChange])

  // 获取评分等级和颜色
  const getScoreGrade = (score: number | null) => {
    if (score === null) return { grade: '-', color: 'text-gray-400' }

    if (score >= 90) return { grade: 'A', color: 'text-green-600' }
    if (score >= 80) return { grade: 'B', color: 'text-blue-600' }
    if (score >= 70) return { grade: 'C', color: 'text-yellow-600' }
    if (score >= 60) return { grade: 'D', color: 'text-orange-600' }
    return { grade: 'F', color: 'text-red-600' }
  }

  const grade = getScoreGrade(report?.overall_score ?? null)

  return (
    <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
      {/* 标题 */}
      <div className="flex items-center justify-between mb-6">
        <h3 className="text-lg font-semibold text-gray-900">PRD 一致性检查</h3>
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
          <span className="ml-3 text-gray-600">正在检查 PRD 一致性...</span>
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
              <p className="text-sm font-medium text-gray-600">总体一致性</p>
              <p className={`text-3xl font-bold ${grade.color}`}>
                {report.overall_score}
                <span className="text-lg ml-1">/ 100</span>
              </p>
            </div>
            <div className={`text-5xl font-bold ${grade.color}`}>{grade.grade}</div>
          </div>

          {/* 各维度评分 */}
          <div>
            <h4 className="text-md font-semibold text-gray-900 mb-3">各维度评分</h4>
            <div className="grid grid-cols-2 gap-3">
              <DimensionScore
                label="用户 - 功能对齐"
                score={report.dimensions.user_feature_alignment}
              />
              <DimensionScore
                label="技术 - 功能对齐"
                score={report.dimensions.tech_feature_alignment}
              />
              <DimensionScore
                label="工作量合理性"
                score={report.dimensions.effort_reasonableness}
              />
              <DimensionScore
                label="术语一致性"
                score={report.dimensions.terminology_consistency}
              />
              <DimensionScore label="逻辑一致性" score={report.dimensions.logical_consistency} />
            </div>
          </div>

          {/* 不一致性问题 */}
          {report.inconsistencies.length > 0 && (
            <div>
              <h4 className="text-md font-semibold text-gray-900 mb-3">
                发现的不一致性问题 ({report.inconsistencies.length})
              </h4>
              <div className="space-y-2">
                {report.inconsistencies.map((issue, idx) => (
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
                        <p className="text-sm font-medium text-gray-900">{issue.description}</p>
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

/**
 * 维度评分展示组件
 */
interface DimensionScoreProps {
  label: string
  score: number
}

const DimensionScore: React.FC<DimensionScoreProps> = ({ label, score }) => {
  const getColor = (s: number) => {
    if (s >= 90) return 'text-green-600'
    if (s >= 80) return 'text-blue-600'
    if (s >= 70) return 'text-yellow-600'
    if (s >= 60) return 'text-orange-600'
    return 'text-red-600'
  }

  return (
    <div className="bg-gray-50 rounded-lg p-3">
      <p className="text-xs text-gray-600 mb-1">{label}</p>
      <p className={`text-2xl font-bold ${getColor(score)}`}>{score}</p>
    </div>
  )
}
