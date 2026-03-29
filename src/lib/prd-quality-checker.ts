/**
 * PRD 质量检查规则引擎
 *
 * 负责：
 * - 完整性检查（各章节是否存在）
 * - 评分计算（加权平均）
 * - 问题检测（发现质量问题）
 * - 建议生成（提供改进方向）
 */

import type { PRD } from '@/types'
import type {
  PRDQualityReport,
  CompletenessReport,
  SectionScore,
  QualityIssue,
  ScoringWeights,
} from '@/types/prd-quality'
import { DEFAULT_WEIGHTS } from '@/types/prd-quality'

/**
 * PRD 质量检查器类
 */
export class PRDQualityChecker {
  private weights: ScoringWeights

  constructor(weights: ScoringWeights = DEFAULT_WEIGHTS) {
    this.weights = weights
  }

  /**
   * 执行完整的质量检查
   */
  async checkQuality(prd: PRD): Promise<PRDQualityReport> {
    // 1. 检查完整性
    const completeness = this.checkCompleteness(prd)

    // 2. 检测质量问题
    const issues = this.detectIssues(prd)

    // 3. 生成改进建议
    const suggestions = this.generateSuggestions(completeness, issues)

    // 4. 计算总体评分
    const overallScore = this.calculateOverallScore(completeness, issues)

    return {
      overallScore,
      completeness,
      issues,
      suggestions,
    }
  }

  /**
   * 检查 PRD 完整性
   */
  private checkCompleteness(prd: PRD): CompletenessReport {
    const sections: CompletenessReport['sections'] = {}
    const missingSections: string[] = []

    // 检查产品标题
    if (prd.title && prd.title.length > 0) {
      sections.title = this.scoreSection(prd.title, 'title')
    } else {
      missingSections.push('产品标题')
    }

    // 检查产品概述
    if (prd.overview) {
      sections.overview = this.scoreSection(prd.overview, 'overview')
    } else {
      missingSections.push('产品概述')
    }

    // 检查目标用户
    if (prd.targetUsers && prd.targetUsers.length > 0) {
      sections.targetUsers = this.scoreSection(prd.targetUsers, 'targetUsers')
    } else {
      missingSections.push('目标用户')
    }

    // 检查核心功能
    if (prd.coreFeatures && prd.coreFeatures.length > 0) {
      sections.coreFeatures = this.scoreSection(prd.coreFeatures, 'coreFeatures')
    } else {
      missingSections.push('核心功能')
    }

    // 检查技术栈
    if (prd.techStack && prd.techStack.length > 0) {
      sections.techStack = this.scoreSection(prd.techStack, 'techStack')
    } else {
      missingSections.push('技术栈')
    }

    // 检查预估工作量
    if (prd.estimatedEffort) {
      sections.estimatedEffort = this.scoreSection(prd.estimatedEffort, 'estimatedEffort')
    } else {
      missingSections.push('预估工作量')
    }

    // 计算完整性得分
    const score = this.calculateCompletenessScore(sections, missingSections)

    return {
      score,
      sections,
      missingSections,
    }
  }

  /**
   * 对单个章节进行评分
   */
  private scoreSection(content: unknown, sectionType: string): SectionScore {
    const issues: string[] = []
    let score = 100

    // 根据章节类型进行特定检查
    switch (sectionType) {
      case 'title': {
        const text = content as string
        if (text.length < 5) {
          issues.push('产品标题过于简短')
          score -= 20
        }
        break
      }

      case 'overview': {
        const text = content as string
        if (text.length < 50) {
          issues.push('产品概述过于简短，建议至少 50 字')
          score -= 30
        }
        if (text.length < 100) {
          issues.push('产品概述不够详细，建议达到 100 字以上')
          score -= 20
        }
        break
      }

      case 'targetUsers': {
        const users = content as string[]
        if (users.length === 0) {
          issues.push('没有定义目标用户')
          score -= 50
        } else if (users.length < 2) {
          issues.push('目标用户数量不足，建议至少 2 个')
          score -= 20
        }
        break
      }

      case 'coreFeatures': {
        const features = content as string[]
        if (features.length === 0) {
          issues.push('没有定义核心功能')
          score -= 50
        } else if (features.length < 3) {
          issues.push('核心功能数量不足，建议至少 3 个')
          score -= 20
        }
        break
      }

      case 'techStack': {
        const techs = content as string[]
        if (techs.length === 0) {
          issues.push('没有定义技术栈')
          score -= 50
        } else if (techs.length < 2) {
          issues.push('技术栈描述不够详细，建议至少 2 个技术')
          score -= 20
        }
        break
      }

      case 'estimatedEffort': {
        const effort = content as string
        if (!effort || effort.length === 0) {
          issues.push('没有预估工作量')
          score -= 50
        }
        break
      }
    }

    // 确保分数不低于 0
    score = Math.max(0, score)

    return {
      present: true,
      score,
      issues,
    }
  }

  /**
   * 计算完整性得分
   */
  private calculateCompletenessScore(
    sections: CompletenessReport['sections'],
    missingSections: string[]
  ): number {
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    const totalWeight =
      this.weights.title +
      this.weights.overview +
      this.weights.targetUsers +
      this.weights.coreFeatures +
      this.weights.techStack +
      this.weights.estimatedEffort

    let earnedWeight = 0

    // 产品标题
    if (sections.title) {
      earnedWeight += (this.weights.title * sections.title.score) / 100
    }

    // 产品概述
    if (sections.overview) {
      earnedWeight += (this.weights.overview * sections.overview.score) / 100
    }

    // 目标用户
    if (sections.targetUsers) {
      earnedWeight += (this.weights.targetUsers * sections.targetUsers.score) / 100
    }

    // 核心功能
    if (sections.coreFeatures) {
      earnedWeight += (this.weights.coreFeatures * sections.coreFeatures.score) / 100
    }

    // 技术栈
    if (sections.techStack) {
      earnedWeight += (this.weights.techStack * sections.techStack.score) / 100
    }

    // 预估工作量
    if (sections.estimatedEffort) {
      earnedWeight += (this.weights.estimatedEffort * sections.estimatedEffort.score) / 100
    }

    // 缺失章节的惩罚（每个缺失章节扣 10 分）
    const missingPenalty = missingSections.length * 10

    return Math.max(0, Math.min(100, earnedWeight - missingPenalty))
  }

  /**
   * 检测质量问题
   */
  private detectIssues(prd: PRD): QualityIssue[] {
    const issues: QualityIssue[] = []

    // 检查产品标题
    if (!prd.title || prd.title.length === 0) {
      issues.push({
        severity: 'major',
        section: '产品标题',
        description: '缺少产品标题',
        suggestion: '添加简洁明确的产品标题',
      })
    } else if (prd.title.length < 5) {
      issues.push({
        severity: 'minor',
        section: '产品标题',
        description: '产品标题过于简短',
        suggestion: '完善产品标题，使其更具描述性',
      })
    }

    // 检查产品概述
    if (!prd.overview) {
      issues.push({
        severity: 'critical',
        section: '产品概述',
        description: '缺少产品概述',
        suggestion: '添加清晰的产品定位和价值主张描述',
      })
    } else if (prd.overview.length < 50) {
      issues.push({
        severity: 'major',
        section: '产品概述',
        description: '产品概述过于简短',
        suggestion: '扩展产品概述，至少达到 50 字',
      })
    }

    // 检查目标用户
    if (!prd.targetUsers || prd.targetUsers.length === 0) {
      issues.push({
        severity: 'critical',
        section: '目标用户',
        description: '缺少目标用户',
        suggestion: '定义至少 2 个目标用户群体',
      })
    } else if (prd.targetUsers.length < 2) {
      issues.push({
        severity: 'major',
        section: '目标用户',
        description: '目标用户数量不足',
        suggestion: '增加更多目标用户以覆盖不同用户群体',
      })
    }

    // 检查核心功能
    if (!prd.coreFeatures || prd.coreFeatures.length === 0) {
      issues.push({
        severity: 'critical',
        section: '核心功能',
        description: '缺少核心功能',
        suggestion: '列出产品的核心功能需求',
      })
    } else if (prd.coreFeatures.length < 3) {
      issues.push({
        severity: 'major',
        section: '核心功能',
        description: '核心功能数量不足',
        suggestion: '补充更多核心功能，至少 3 个',
      })
    }

    // 检查技术栈
    if (!prd.techStack || prd.techStack.length === 0) {
      issues.push({
        severity: 'major',
        section: '技术栈',
        description: '缺少技术栈描述',
        suggestion: '描述拟采用的技术栈，至少 2 个关键技术',
      })
    }

    // 检查预估工作量
    if (!prd.estimatedEffort) {
      issues.push({
        severity: 'major',
        section: '预估工作量',
        description: '缺少预估工作量',
        suggestion: '提供初步的工作量预估',
      })
    }

    return issues
  }

  /**
   * 生成改进建议
   */
  private generateSuggestions(completeness: CompletenessReport, issues: QualityIssue[]): string[] {
    const suggestions: string[] = []

    // 基于缺失章节的建议
    completeness.missingSections.forEach(section => {
      suggestions.push(`请添加${section}章节`)
    })

    // 基于严重问题的建议
    const criticalIssues = issues.filter(i => i.severity === 'critical')
    criticalIssues.forEach(issue => {
      if (!suggestions.includes(issue.suggestion)) {
        suggestions.push(issue.suggestion)
      }
    })

    // 基于完整性得分的建议
    if (completeness.score < 60) {
      suggestions.push('PRD 完整性较低，建议先补充缺失的核心章节')
    } else if (completeness.score < 80) {
      suggestions.push('PRD 完整性一般，建议进一步完善各个章节')
    }

    // 通用建议
    if (issues.length > 5) {
      suggestions.push('发现多个质量问题，建议逐一修复以提升 PRD 质量')
    }

    return suggestions
  }

  /**
   * 计算总体评分
   */
  private calculateOverallScore(completeness: CompletenessReport, issues: QualityIssue[]): number {
    // 基础分为完整性得分
    let score = completeness.score

    // 根据问题严重程度扣分
    const criticalCount = issues.filter(i => i.severity === 'critical').length
    const majorCount = issues.filter(i => i.severity === 'major').length
    const minorCount = issues.filter(i => i.severity === 'minor').length

    // 严重问题扣 15 分，主要问题扣 8 分，次要问题扣 3 分
    score -= criticalCount * 15
    score -= majorCount * 8
    score -= minorCount * 3

    // 确保分数在 0-100 之间
    return Math.max(0, Math.min(100, score))
  }
}

/**
 * 创建默认的质量检查器实例
 */
export const defaultQualityChecker = new PRDQualityChecker()
