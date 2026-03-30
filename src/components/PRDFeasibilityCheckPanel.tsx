import React from 'react'
import { usePRDFeasibilityCheck } from '@/hooks/usePRDFeasibilityCheck'
import type { PRD } from '@/types'

interface PRDFeasibilityCheckPanelProps {
  prd: PRD
  onScoreChange?: (score: number) => void
}

/**
 * PRD 可行性评估面板组件
 */
export const PRDFeasibilityCheckPanel: React.FC<PRDFeasibilityCheckPanelProps> = ({
  prd,
  onScoreChange,
}) => {
  const { report, isAssessing, error, assessFeasibility, reset } = usePRDFeasibilityCheck()

  // 自动评估可行性
  React.useEffect(() => {
    if (prd) {
      assessFeasibility(prd)
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [prd])

  // 通知父组件评分变化
  React.useEffect(() => {
    if (report?.overall_score !== undefined && onScoreChange) {
      onScoreChange(report.overall_score)
    }
  }, [report?.overall_score, onScoreChange])

  // 获取可行性等级和颜色
  const getFeasibilityGrade = (level: string | null) => {
    if (!level) return { grade: '-', color: 'text-gray-400', bg: 'bg-gray-100' }

    switch (level) {
      case 'high':
        return { grade: '高', color: 'text-green-600', bg: 'bg-green-100' }
      case 'medium':
        return { grade: '中', color: 'text-yellow-600', bg: 'bg-yellow-100' }
      case 'low':
        return { grade: '低', color: 'text-red-600', bg: 'bg-red-100' }
      default:
        return { grade: '-', color: 'text-gray-400', bg: 'bg-gray-100' }
    }
  }

  const grade = getFeasibilityGrade(report?.feasibility_level ?? null)

  return (
    <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
      {/* 标题 */}
      <div className="flex items-center justify-between mb-6">
        <h3 className="text-lg font-semibold text-gray-900">PRD 可行性评估</h3>
        {report && (
          <button onClick={reset} className="text-sm text-blue-600 hover:text-blue-700">
            重新评估
          </button>
        )}
      </div>

      {/* 加载状态 */}
      {isAssessing && (
        <div className="flex items-center justify-center py-12">
          <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
          <span className="ml-3 text-gray-600">正在评估 PRD 可行性...</span>
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

      {/* 评估结果 */}
      {report && !isAssessing && !error && (
        <div className="space-y-6">
          {/* 总体可行性 */}
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm font-medium text-gray-600">总体可行性</p>
              <p className={`text-3xl font-bold ${grade.color}`}>
                {report.overall_score}
                <span className="text-lg ml-1">/ 100</span>
              </p>
            </div>
            <div className={`px-6 py-3 rounded-lg font-bold ${grade.bg} ${grade.color}`}>
              可行性：{grade.grade}
            </div>
          </div>

          {/* 各维度评估 */}
          <div>
            <h4 className="text-md font-semibold text-gray-900 mb-3">各维度评估</h4>
            <div className="grid grid-cols-2 gap-3">
              <AssessmentCard
                title="技术可行性"
                score={report.technical.feasibility_score}
                details={[
                  `复杂度：${(report.technical.complexity * 100).toFixed(0)}%`,
                  `技能匹配：${(report.technical.team_skill_match * 100).toFixed(0)}%`,
                ]}
              />
              <AssessmentCard
                title="资源充足度"
                score={Math.round(report.resource.resource_adequacy * 100)}
                details={[
                  `所需人力：${report.resource.required_people_months.toFixed(1)} 人月`,
                  `团队规模：${report.resource.available_team_size} 人`,
                ]}
              />
              <AssessmentCard
                title="时间合理性"
                score={report.timeline.reasonableness_score}
                details={[
                  `预估：${report.timeline.estimated_weeks.toFixed(1)} 周`,
                  `合理：${report.timeline.reasonable_min_weeks.toFixed(1)}-${report.timeline.reasonable_max_weeks.toFixed(1)} 周`,
                ]}
              />
              <AssessmentCard
                title="风险数量"
                score={Math.max(0, 100 - report.risks.length * 20)}
                details={[`已识别 ${report.risks.length} 个风险`]}
              />
            </div>
          </div>

          {/* 风险列表 */}
          {report.risks.length > 0 && (
            <div>
              <h4 className="text-md font-semibold text-gray-900 mb-3">
                识别的风险 ({report.risks.length})
              </h4>
              <div className="space-y-2">
                {report.risks.map((risk, idx) => (
                  <div
                    key={idx}
                    className={`border-l-4 rounded p-3 ${
                      risk.level === 'critical'
                        ? 'border-red-500 bg-red-50'
                        : risk.level === 'high'
                          ? 'border-orange-500 bg-orange-50'
                          : risk.level === 'medium'
                            ? 'border-yellow-500 bg-yellow-50'
                            : 'border-blue-500 bg-blue-50'
                    }`}
                  >
                    <div className="flex items-start justify-between">
                      <div className="flex-1">
                        <p className="text-sm font-medium text-gray-900">{risk.description}</p>
                        <p className="text-xs text-gray-600 mt-1">影响：{risk.impact}</p>
                        {risk.mitigation && (
                          <p className="text-xs text-gray-600 mt-1">💡 {risk.mitigation}</p>
                        )}
                      </div>
                      <span
                        className={`text-xs px-2 py-1 rounded ${
                          risk.level === 'critical'
                            ? 'bg-red-200 text-red-800'
                            : risk.level === 'high'
                              ? 'bg-orange-200 text-orange-800'
                              : risk.level === 'medium'
                                ? 'bg-yellow-200 text-yellow-800'
                                : 'bg-blue-200 text-blue-800'
                        }`}
                      >
                        {risk.level === 'critical'
                          ? '严重'
                          : risk.level === 'high'
                            ? '高'
                            : risk.level === 'medium'
                              ? '中'
                              : '低'}
                      </span>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          )}

          {/* 改进建议 */}
          {report.recommendations.length > 0 && (
            <div>
              <h4 className="text-md font-semibold text-gray-900 mb-3">改进建议</h4>
              <ul className="list-disc list-inside space-y-1">
                {report.recommendations.map((recommendation, idx) => (
                  <li key={idx} className="text-sm text-gray-700">
                    {recommendation}
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
 * 评估卡片组件
 */
interface AssessmentCardProps {
  title: string
  score: number
  details: string[]
}

const AssessmentCard: React.FC<AssessmentCardProps> = ({ title, score, details }) => {
  const getColor = (s: number) => {
    if (s >= 80) return 'text-green-600'
    if (s >= 60) return 'text-blue-600'
    if (s >= 40) return 'text-yellow-600'
    return 'text-red-600'
  }

  return (
    <div className="bg-gray-50 rounded-lg p-3">
      <p className="text-xs text-gray-600 mb-1">{title}</p>
      <p className={`text-2xl font-bold ${getColor(score)}`}>{score}</p>
      <ul className="mt-2 space-y-1">
        {details.map((detail, idx) => (
          <li key={idx} className="text-xs text-gray-500">
            {detail}
          </li>
        ))}
      </ul>
    </div>
  )
}
