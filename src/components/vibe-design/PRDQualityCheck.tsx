import React from 'react'
import { usePRDQuality } from '@/hooks/usePRDQuality'
import type { PRD } from '@/types'
import type { QualityIssue } from '@/types/prd-quality'

interface PRDQualityCheckProps {
  prd: PRD
  onQualityChange?: (score: number) => void
}

/**
 * PRD 质量检查组件
 *
 * 显示 PRD 的完整性评分、发现的问题和改进建议
 */
export const PRDQualityCheck: React.FC<PRDQualityCheckProps> = ({ prd, onQualityChange }) => {
  const { isChecking, progress, qualityReport, overallScore, checkQuality, reset } = usePRDQuality()

  // 自动检查质量
  React.useEffect(() => {
    if (prd) {
      checkQuality(prd)
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [prd])

  // 通知父组件评分变化
  React.useEffect(() => {
    if (overallScore !== null && onQualityChange) {
      onQualityChange(overallScore)
    }
  }, [overallScore, onQualityChange])

  // 获取评分对应的等级和颜色
  const getScoreGrade = (score: number | null) => {
    if (score === null) return { grade: '-', color: 'text-gray-400' }

    if (score >= 90) return { grade: 'A', color: 'text-green-600' }
    if (score >= 80) return { grade: 'B', color: 'text-blue-600' }
    if (score >= 70) return { grade: 'C', color: 'text-yellow-600' }
    if (score >= 60) return { grade: 'D', color: 'text-orange-600' }
    return { grade: 'F', color: 'text-red-600' }
  }

  const grade = getScoreGrade(overallScore)

  return (
    <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
      {/* 标题 */}
      <div className="flex items-center justify-between mb-6">
        <h3 className="text-lg font-semibold text-gray-900">PRD 质量检查</h3>
        {qualityReport && (
          <button onClick={reset} className="text-sm text-blue-600 hover:text-blue-700">
            重新检查
          </button>
        )}
      </div>

      {/* 加载状态 */}
      {isChecking && (
        <div className="space-y-4">
          <div className="flex items-center justify-center py-8">
            <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
          </div>
          <div className="space-y-2">
            <div className="flex justify-between text-sm text-gray-600">
              <span>正在分析 PRD...</span>
              <span>{progress}%</span>
            </div>
            <div className="w-full bg-gray-200 rounded-full h-2">
              <div
                className="bg-blue-600 h-2 rounded-full transition-all duration-300"
                style={{ width: `${progress}%` }}
              />
            </div>
          </div>
        </div>
      )}

      {/* 检查结果 */}
      {!isChecking && qualityReport && (
        <div className="space-y-6">
          {/* 总体评分 */}
          <div className="text-center py-4 bg-gradient-to-r from-blue-50 to-purple-50 rounded-lg">
            <div className="text-sm text-gray-600 mb-2">总体评分</div>
            <div className="flex items-center justify-center gap-3">
              <span className={`text-5xl font-bold ${grade.color}`}>{overallScore}</span>
              <span className={`text-2xl font-semibold ${grade.color}`}>等级 {grade.grade}</span>
            </div>
          </div>

          {/* 完整性详情 */}
          {qualityReport.completeness.sections && (
            <div className="space-y-3">
              <h4 className="font-medium text-gray-900">章节完整性</h4>

              {/* 产品标题 */}
              {qualityReport.completeness.sections.title && (
                <SectionProgress
                  label="产品标题"
                  score={qualityReport.completeness.sections.title.score}
                  issues={qualityReport.completeness.sections.title.issues}
                />
              )}

              {/* 产品概述 */}
              {qualityReport.completeness.sections.overview && (
                <SectionProgress
                  label="产品概述"
                  score={qualityReport.completeness.sections.overview.score}
                  issues={qualityReport.completeness.sections.overview.issues}
                />
              )}

              {/* 目标用户 */}
              {qualityReport.completeness.sections.targetUsers && (
                <SectionProgress
                  label="目标用户"
                  score={qualityReport.completeness.sections.targetUsers.score}
                  issues={qualityReport.completeness.sections.targetUsers.issues}
                />
              )}

              {/* 核心功能 */}
              {qualityReport.completeness.sections.coreFeatures && (
                <SectionProgress
                  label="核心功能"
                  score={qualityReport.completeness.sections.coreFeatures.score}
                  issues={qualityReport.completeness.sections.coreFeatures.issues}
                />
              )}

              {/* 技术栈 */}
              {qualityReport.completeness.sections.techStack && (
                <SectionProgress
                  label="技术栈"
                  score={qualityReport.completeness.sections.techStack.score}
                  issues={qualityReport.completeness.sections.techStack.issues}
                />
              )}

              {/* 预估工作量 */}
              {qualityReport.completeness.sections.estimatedEffort && (
                <SectionProgress
                  label="预估工作量"
                  score={qualityReport.completeness.sections.estimatedEffort.score}
                  issues={qualityReport.completeness.sections.estimatedEffort.issues}
                />
              )}

              {/* 缺失章节 */}
              {qualityReport.completeness.missingSections.length > 0 && (
                <div className="mt-4 p-3 bg-red-50 border border-red-200 rounded-lg">
                  <div className="text-sm font-medium text-red-800 mb-2">缺失的章节</div>
                  <ul className="list-disc list-inside space-y-1">
                    {qualityReport.completeness.missingSections.map((section, index) => (
                      <li key={index} className="text-sm text-red-700">
                        {section}
                      </li>
                    ))}
                  </ul>
                </div>
              )}
            </div>
          )}

          {/* 质量问题 */}
          {qualityReport.issues.length > 0 && (
            <div className="space-y-3">
              <h4 className="font-medium text-gray-900">发现的质量问题</h4>
              <div className="space-y-2">
                {qualityReport.issues.map((issue, index) => (
                  <IssueCard key={index} issue={issue} />
                ))}
              </div>
            </div>
          )}

          {/* 改进建议 */}
          {qualityReport.suggestions.length > 0 && (
            <div className="space-y-3">
              <h4 className="font-medium text-gray-900">改进建议</h4>
              <div className="space-y-2">
                {qualityReport.suggestions.map((suggestion, index) => (
                  <div
                    key={index}
                    className="flex items-start gap-2 p-3 bg-blue-50 border border-blue-200 rounded-lg"
                  >
                    <svg
                      className="w-5 h-5 text-blue-600 mt-0.5 flex-shrink-0"
                      fill="none"
                      stroke="currentColor"
                      viewBox="0 0 24 24"
                    >
                      <path
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        strokeWidth={2}
                        d="M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z"
                      />
                    </svg>
                    <span className="text-sm text-blue-800">{suggestion}</span>
                  </div>
                ))}
              </div>
            </div>
          )}
        </div>
      )}
    </div>
  )
}

// 章节进度组件
interface SectionProgressProps {
  label: string
  score: number
  issues: string[]
}

const SectionProgress: React.FC<SectionProgressProps> = ({ label, score, issues }) => {
  const getScoreColor = (score: number) => {
    if (score >= 80) return 'bg-green-500'
    if (score >= 60) return 'bg-yellow-500'
    return 'bg-red-500'
  }

  return (
    <div className="space-y-2">
      <div className="flex justify-between items-center">
        <span className="text-sm text-gray-700">{label}</span>
        <span className="text-sm font-medium text-gray-900">{score}分</span>
      </div>
      <div className="w-full bg-gray-200 rounded-full h-2">
        <div
          className={`${getScoreColor(score)} h-2 rounded-full transition-all duration-300`}
          style={{ width: `${score}%` }}
        />
      </div>
      {issues.length > 0 && (
        <ul className="list-disc list-inside space-y-1">
          {issues.map((issue, index) => (
            <li key={index} className="text-xs text-gray-600">
              {issue}
            </li>
          ))}
        </ul>
      )}
    </div>
  )
}

// 问题卡片组件
interface IssueCardProps {
  issue: QualityIssue
}

const IssueCard: React.FC<IssueCardProps> = ({ issue }) => {
  const severityLabels = {
    critical: '严重',
    major: '重要',
    minor: '提示',
  }

  const getSeverityColor = (severity: string) => {
    switch (severity) {
      case 'critical':
        return 'text-red-600 bg-red-50 border-red-200'
      case 'major':
        return 'text-orange-600 bg-orange-50 border-orange-200'
      case 'minor':
        return 'text-yellow-600 bg-yellow-50 border-yellow-200'
      default:
        return 'text-gray-600 bg-gray-50 border-gray-200'
    }
  }

  return (
    <div className={`p-3 border rounded-lg ${getSeverityColor(issue.severity)}`}>
      <div className="flex items-start gap-2">
        <span className="text-xs font-medium px-2 py-0.5 rounded bg-white/50">
          {severityLabels[issue.severity]}
        </span>
        <div className="flex-1">
          <div className="font-medium text-sm">{issue.description}</div>
          <div className="text-xs mt-1 opacity-75">{issue.section}</div>
          <div className="text-xs mt-2 font-medium">💡 {issue.suggestion}</div>
        </div>
      </div>
    </div>
  )
}

export default PRDQualityCheck
