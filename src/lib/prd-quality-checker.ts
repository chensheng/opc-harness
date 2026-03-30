import type { PRD } from '@/types'
import type {
  PRDQualityReport,
  CompletenessReport,
  SectionScore,
  QualityIssue,
  ScoringWeights,
  ConsistencyReport,
  ConsistencyIssue,
  FeasibilityReport,
  FeasibilityDimension,
  FeasibilityRisk,
  FeasibilityRiskLevel,
} from '@/types/prd-quality'
import { DEFAULT_WEIGHTS } from '@/types/prd-quality'

/**
 * PRD 质量检查规则引擎
 *
 * 负责：
 * - 完整性检查（各章节是否存在）
 * - 一致性验证（检查内部矛盾）
 * - 评分计算（加权平均）
 * - 问题检测（发现质量问题）
 * - 建议生成（提供改进方向）
 * - 可行性评估（技术/资源/时间/范围）
 */

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

    // 2. 验证一致性
    const consistency = this.checkConsistency(prd)

    // 3. 评估可行性
    const feasibility = this.evaluateFeasibility(prd)

    // 4. 检测质量问题
    const issues = this.detectIssues(prd)

    // 5. 生成改进建议
    const suggestions = this.generateSuggestions(completeness, issues, consistency, feasibility)

    // 6. 计算总体评分（包含一致性和可行性）
    const overallScore = this.calculateOverallScore(completeness, issues, consistency, feasibility)

    return {
      overallScore,
      completeness,
      consistency,
      feasibility,
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
  private generateSuggestions(
    completeness: CompletenessReport,
    issues: QualityIssue[],
    consistency: ConsistencyReport,
    feasibility?: FeasibilityReport
  ): string[] {
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

    // 基于一致性问题的建议
    consistency.issues.forEach(issue => {
      if (!suggestions.includes(issue.suggestion)) {
        suggestions.push(issue.suggestion)
      }
    })

    // 基于可行性问题的建议
    if (feasibility && feasibility.recommendations.length > 0) {
      feasibility.recommendations.forEach(rec => {
        if (!suggestions.includes(rec)) {
          suggestions.push(rec)
        }
      })
    }

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

    // 基于一致性检查的建议
    if (consistency.issues.length > 0) {
      suggestions.push(
        `发现 ${consistency.issues.length} 个一致性问题，建议检查需求之间是否存在矛盾`
      )
    }

    // 基于可行性评估的建议
    if (feasibility) {
      if (feasibility.riskLevel === 'high') {
        suggestions.push('项目可行性风险较高，建议重新评估技术方案或资源投入')
      }

      if (feasibility.dimensions.technical.score < 70) {
        suggestions.push('技术可行性评分较低，建议优化技术栈选型')
      }

      if (feasibility.dimensions.timeline.score < 70) {
        suggestions.push('时间规划可能存在风险，建议重新评估工作量')
      }

      if (feasibility.dimensions.scope.score < 70) {
        suggestions.push('需求范围不够清晰，建议细化产品功能和用户定位')
      }

      // 添加可行性报告中的具体建议
      suggestions.push(...feasibility.recommendations)
    }

    return suggestions
  }

  /**
   * 计算总体评分
   */
  private calculateOverallScore(
    completeness: CompletenessReport,
    issues: QualityIssue[],
    consistency: ConsistencyReport,
    feasibility: FeasibilityReport
  ): number {
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

    // 根据一致性问题扣分
    const consistencyIssues = consistency.issues.length
    score -= consistencyIssues * 5

    // 根据可行性问题扣分
    const feasibilityRisks = feasibility.risks.filter(r => r.level === 'high').length
    const mediumRisks = feasibility.risks.filter(r => r.level === 'medium').length
    score -= feasibilityRisks * 15
    score -= mediumRisks * 8

    // 确保分数在 0-100 之间
    return Math.max(0, Math.min(100, score))
  }

  /**
   * 验证 PRD 一致性
   *
   * 检查内容：
   * 1. 目标用户与核心功能的一致性
   * 2. 产品概述与核心功能的一致性
   * 3. 技术栈与功能的兼容性
   * 4. 工作量与功能复杂度的匹配度
   */
  private checkConsistency(prd: PRD): ConsistencyReport {
    const issues: ConsistencyIssue[] = []
    const verified: string[] = []

    // 1. 检查目标用户与核心功能的一致性
    if (
      prd.targetUsers &&
      prd.targetUsers.length > 0 &&
      prd.coreFeatures &&
      prd.coreFeatures.length > 0
    ) {
      // 检查是否有针对每个用户群体的功能
      if (prd.targetUsers.length > 3 && prd.coreFeatures.length < 2) {
        issues.push({
          type: 'inconsistency',
          sections: ['目标用户', '核心功能'],
          description: '目标用户群体较多，但核心功能数量不足，可能无法满足所有用户需求',
          details: {
            statement1: `定义了 ${prd.targetUsers.length} 个目标用户群体`,
            statement2: `只有 ${prd.coreFeatures.length} 个核心功能`,
          },
          suggestion: '增加核心功能数量，或明确每个功能服务的用户群体',
        })
      } else {
        verified.push('目标用户与核心功能数量匹配')
      }
    }

    // 2. 检查产品概述与核心功能的一致性
    if (prd.overview && prd.coreFeatures && prd.coreFeatures.length > 0) {
      const overviewKeywords = this.extractKeywords(prd.overview)
      const featureText = prd.coreFeatures.join(' ')

      // 检查概述中的核心价值是否在功能中得到体现
      // 只要有部分关键词匹配就算一致（降低要求）
      const matchCount = overviewKeywords.filter(keyword =>
        featureText.toLowerCase().includes(keyword.toLowerCase())
      ).length

      const hasCoreValueInFeatures =
        matchCount >= Math.max(1, Math.floor(overviewKeywords.length * 0.3))

      if (!hasCoreValueInFeatures && overviewKeywords.length > 2) {
        issues.push({
          type: 'ambiguity',
          sections: ['产品概述', '核心功能'],
          description: '产品概述中提到的核心价值在核心功能中没有明确体现',
          details: {
            statement1: `产品概述强调：${overviewKeywords.slice(0, 3).join(', ')}`,
            statement2: `核心功能未直接体现这些价值`,
          },
          suggestion: '在核心功能中明确描述如何实现产品概述中的价值主张',
        })
      } else {
        verified.push('产品概述与核心功能一致')
      }
    }

    // 3. 检查技术栈与功能的兼容性（简单规则检查）
    if (
      prd.techStack &&
      prd.techStack.length > 0 &&
      prd.coreFeatures &&
      prd.coreFeatures.length > 0
    ) {
      const techStackText = prd.techStack.join(' ').toLowerCase()

      // 检查是否包含前后端技术（如果是全栈应用）
      const hasFrontend = prd.coreFeatures.some(
        f => f.includes('界面') || f.includes('交互') || f.includes('展示') || f.includes('UI')
      )

      if (hasFrontend && !techStackText.match(/react|vue|angular|svelte|ui/)) {
        issues.push({
          type: 'contradiction',
          sections: ['核心功能', '技术栈'],
          description: '功能描述涉及前端界面，但技术栈中未明确前端框架',
          details: {
            statement1: '核心功能包含前端界面相关需求',
            statement2: `技术栈：${prd.techStack.join(', ')}`,
          },
          suggestion: '在技术栈中添加前端框架（如 React、Vue 等）',
        })
      } else {
        verified.push('技术栈与功能需求匹配')
      }
    }

    // 4. 检查工作量与功能复杂度的匹配度
    if (prd.estimatedEffort && prd.coreFeatures && prd.coreFeatures.length > 0) {
      const effortText = prd.estimatedEffort.toLowerCase()
      const featureCount = prd.coreFeatures.length

      // 简单规则：功能数量与工作量的关系
      if (featureCount >= 5 && effortText.includes('天')) {
        issues.push({
          type: 'contradiction',
          sections: ['核心功能', '预估工作量'],
          description: '功能数量较多，但预估时间较短，可能存在低估风险',
          details: {
            statement1: `${featureCount} 个核心功能`,
            statement2: `预估工作量：${prd.estimatedEffort}`,
          },
          suggestion: '重新评估工作量，考虑增加时间预算或减少功能范围',
        })
      } else if (featureCount <= 2 && effortText.includes('月')) {
        issues.push({
          type: 'contradiction',
          sections: ['核心功能', '预估工作量'],
          description: '功能数量较少，但预估时间较长，可能存在高估或功能定义不清晰',
          details: {
            statement1: `${featureCount} 个核心功能`,
            statement2: `预估工作量：${prd.estimatedEffort}`,
          },
          suggestion: '细化功能定义，或说明复杂度高的原因',
        })
      } else {
        verified.push('工作量与功能复杂度匹配')
      }
    }

    // 计算一致性得分
    let score = 100

    // 每个问题扣分：矛盾 (contradiction) 扣 30 分，不一致 (inconsistency) 扣 20 分，歧义 (ambiguity) 扣 10 分
    issues.forEach(issue => {
      if (issue.type === 'contradiction') score -= 30
      else if (issue.type === 'inconsistency') score -= 20
      else if (issue.type === 'ambiguity') score -= 10
    })

    score = Math.max(0, Math.min(100, score))

    return {
      passed: issues.length === 0,
      score,
      issues,
      verified,
    }
  }

  /**
   * 评估 PRD 可行性 - 新增四个维度评估
   */
  private evaluateFeasibility(prd: PRD): FeasibilityReport {
    const risks: FeasibilityRisk[] = []

    // 1. 技术可行性评估
    const technicalDimension = this.assessTechnicalFeasibility(prd, risks)

    // 2. 资源可行性评估
    const resourceDimension = this.assessResourceFeasibility(prd, risks)

    // 3. 时间可行性评估
    const timelineDimension = this.assessTimelineFeasibility(prd, risks)

    // 4. 范围可行性评估
    const scopeDimension = this.assessScopeFeasibility(prd, risks)

    // 计算总体可行性得分
    const totalScore =
      (technicalDimension.score +
        resourceDimension.score +
        timelineDimension.score +
        scopeDimension.score) /
      4

    // 确定风险等级
    let riskLevel: FeasibilityRiskLevel = 'low'
    if (risks.some(r => r.level === 'high')) {
      riskLevel = 'high'
    } else if (risks.some(r => r.level === 'medium')) {
      riskLevel = 'medium'
    }

    // 生成建议措施
    const recommendations = this.generateFeasibilityRecommendations(
      technicalDimension,
      resourceDimension,
      timelineDimension,
      scopeDimension,
      risks
    )

    return {
      feasible: totalScore >= 60 && riskLevel !== 'high',
      score: Math.round(totalScore),
      riskLevel,
      dimensions: {
        technical: technicalDimension,
        resource: resourceDimension,
        timeline: timelineDimension,
        scope: scopeDimension,
      },
      risks,
      recommendations,
    }
  }

  /**
   * 评估技术可行性
   */
  private assessTechnicalFeasibility(prd: PRD, risks: FeasibilityRisk[]): FeasibilityDimension {
    const issues: string[] = []
    const strengths: string[] = []
    let score = 100

    // 检查技术栈的现代性
    if (prd.techStack && prd.techStack.length > 0) {
      const techStackText = prd.techStack.join(' ').toLowerCase()

      // 过时技术检测
      if (techStackText.match(/flash|silverlight|delphi|vb6|foxpro|jquery|angularjs/)) {
        issues.push('使用了过时或不再主流的技术栈')
        score -= 30
        risks.push({
          level: 'high',
          category: 'technical',
          description: '技术栈过时',
          impact: '可能导致开发效率低下、维护困难、安全性问题',
          mitigation: '考虑迁移到现代技术栈，如 React/Vue/Svelte 等',
        })
      } else {
        strengths.push('技术栈选择现代且主流')
      }

      // 技术栈成熟度检查
      if (
        techStackText.match(/react|vue|angular|svelte|next\.js|nuxt|spring boot|django|fastapi/)
      ) {
        strengths.push('采用了成熟稳定的技术框架')
        score += 10
      }

      // 技术栈复杂度检查
      if (prd.techStack.length > 8) {
        issues.push('技术栈过于复杂，可能增加学习和维护成本')
        score -= 15
        risks.push({
          level: 'medium',
          category: 'technical',
          description: '技术栈过于庞大',
          impact: '团队学习成本高，技术整合复杂度高',
          mitigation: '精简技术栈，优先使用核心和必要的技术',
        })
      }
    } else {
      issues.push('缺少明确的技术栈说明')
      score -= 40
      risks.push({
        level: 'high',
        category: 'technical',
        description: '技术栈未定义',
        impact: '无法评估技术可行性，开发方向不明确',
        mitigation: '补充详细的技术栈选型和技术方案',
      })
    }

    return {
      name: '技术可行性',
      score: Math.max(0, Math.min(100, score)),
      assessment:
        score >= 80 ? '技术可行性良好' : score >= 60 ? '技术可行性中等' : '技术可行性较差',
      issues,
      strengths,
    }
  }

  /**
   * 评估资源可行性
   */
  private assessResourceFeasibility(prd: PRD, risks: FeasibilityRisk[]): FeasibilityDimension {
    const issues: string[] = []
    const strengths: string[] = []
    let score = 100

    // 检查功能复杂度
    if (prd.coreFeatures && prd.coreFeatures.length > 0) {
      const featureText = prd.coreFeatures.join(' ')

      // 高难度功能检测
      const highComplexityFeatures = [
        '机器学习',
        '深度学习',
        '自然语言处理',
        '计算机视觉',
        '区块链',
        '量子计算',
        '实时音视频',
        '大规模并发',
      ]

      const detectedComplexities = highComplexityFeatures.filter(complexity =>
        featureText.includes(complexity)
      )

      if (detectedComplexities.length > 0) {
        issues.push(`包含高难度技术功能：${detectedComplexities.join(', ')}`)
        score -= 20
        risks.push({
          level: 'medium',
          category: 'resource',
          description: '功能复杂度高',
          impact: '需要专业技能和更多开发资源',
          mitigation: '评估团队技能储备，考虑外部专家支持或分阶段实现',
        })
      } else {
        strengths.push('功能复杂度适中，团队可胜任')
      }

      // 功能数量检查
      if (prd.coreFeatures.length > 20) {
        issues.push('功能点过多，可能需要较多开发资源')
        score -= 15
        risks.push({
          level: 'medium',
          category: 'resource',
          description: '功能范围过大',
          impact: '需要更多人力和时间投入',
          mitigation: '优先考虑核心功能，分期实现次要功能',
        })
      }
    } else {
      issues.push('缺少核心功能描述')
      score -= 40
    }

    return {
      name: '资源可行性',
      score: Math.max(0, Math.min(100, score)),
      assessment: score >= 80 ? '资源需求合理' : score >= 60 ? '资源需求中等' : '资源需求较高',
      issues,
      strengths,
    }
  }

  /**
   * 评估时间可行性
   */
  private assessTimelineFeasibility(prd: PRD, risks: FeasibilityRisk[]): FeasibilityDimension {
    const issues: string[] = []
    const strengths: string[] = []
    let score = 100

    // 检查工作量预估
    if (prd.estimatedEffort) {
      const effortText = prd.estimatedEffort.toLowerCase()

      // 模糊描述检测
      if (effortText.match(/很快 | 马上 | 立即 | 短时间 | 几天/)) {
        issues.push('工作量预估过于模糊，缺乏具体时间')
        score -= 25
        risks.push({
          level: 'medium',
          category: 'timeline',
          description: '时间预估不明确',
          impact: '难以制定合理的开发计划',
          mitigation: '提供具体的时间范围或工作量（如：2-3 周，80-120 人天）',
        })
      } else {
        strengths.push('工作量预估具体明确')
      }

      // 合理性检查（简单基于关键词）
      const timeMatches = effortText.match(/(\d+)(\s*[-~至到]\s*\d+)?\s*(天 | 日|周 | 月|小时)/)
      if (timeMatches) {
        const days = parseInt(timeMatches[1])
        const unit = timeMatches[3]

        // 转换为天数
        let estimatedDays = days
        if (unit === '周') estimatedDays = days * 7
        if (unit === '月') estimatedDays = days * 30

        // 如果功能很多但预估时间很短
        if (prd.coreFeatures && prd.coreFeatures.length > 10 && estimatedDays < 30) {
          issues.push('预估时间相对于功能数量可能不足')
          score -= 20
          risks.push({
            level: 'high',
            category: 'timeline',
            description: '时间预估过于乐观',
            impact: '可能导致延期或质量下降',
            mitigation: '重新评估工作量，考虑增加资源或减少功能范围',
          })
        }
      }
    } else {
      issues.push('缺少工作量预估')
      score -= 40
      risks.push({
        level: 'high',
        category: 'timeline',
        description: '未提供工作量预估',
        impact: '无法制定合理的开发计划',
        mitigation: '补充详细的工作量预估和时间规划',
      })
    }

    return {
      name: '时间可行性',
      score: Math.max(0, Math.min(100, score)),
      assessment: score >= 80 ? '时间规划合理' : score >= 60 ? '时间规划中等' : '时间规划紧张',
      issues,
      strengths,
    }
  }

  /**
   * 评估范围可行性
   */
  private assessScopeFeasibility(prd: PRD, risks: FeasibilityRisk[]): FeasibilityDimension {
    const issues: string[] = []
    const strengths: string[] = []
    let score = 100

    // 检查产品概述的清晰度
    if (prd.overview) {
      if (prd.overview.length < 50) {
        issues.push('产品概述过于简略，可能需求不明确')
        score -= 20
        risks.push({
          level: 'medium',
          category: 'scope',
          description: '需求描述不充分',
          impact: '开发过程中可能出现需求变更',
          mitigation: '补充详细的产品背景和目标描述',
        })
      } else {
        strengths.push('产品概述详细清晰')
      }
    } else {
      issues.push('缺少产品概述')
      score -= 30
    }

    // 检查目标用户的明确性
    if (prd.targetUsers && prd.targetUsers.length > 0) {
      if (prd.targetUsers.length === 1 && prd.targetUsers[0].length < 10) {
        issues.push('目标用户定义不够具体')
        score -= 15
        risks.push({
          level: 'low',
          category: 'scope',
          description: '用户定位不清晰',
          impact: '可能影响产品功能设计和用户体验',
          mitigation: '详细描述目标用户群体的特征和需求',
        })
      } else {
        strengths.push('目标用户群体明确')
      }
    } else {
      issues.push('缺少目标用户定义')
      score -= 20
    }

    // 检查功能边界
    if (prd.coreFeatures && prd.coreFeatures.length > 0) {
      // 检测是否有模糊的功能描述
      const vagueFeatures = prd.coreFeatures.filter(f =>
        f.match(/等等 | 其他 | 相关 | 一些 | 各种 | 多种/)
      )

      if (vagueFeatures.length > 0) {
        issues.push('部分功能描述模糊：' + vagueFeatures.join(', '))
        score -= 15
        risks.push({
          level: 'low',
          category: 'scope',
          description: '功能边界不清晰',
          impact: '可能导致范围蔓延和需求变更',
          mitigation: '明确每个功能的具体内容和边界',
        })
      } else {
        strengths.push('功能边界清晰明确')
      }
    }

    return {
      name: '范围可行性',
      score: Math.max(0, Math.min(100, score)),
      assessment: score >= 80 ? '范围定义清晰' : score >= 60 ? '范围定义中等' : '范围定义模糊',
      issues,
      strengths,
    }
  }

  /**
   * 生成可行性建议
   */
  private generateFeasibilityRecommendations(
    technical: FeasibilityDimension,
    resource: FeasibilityDimension,
    timeline: FeasibilityDimension,
    scope: FeasibilityDimension,
    risks: FeasibilityRisk[]
  ): string[] {
    const recommendations: string[] = []

    // 根据各维度得分生成建议
    if (technical.score < 70) {
      recommendations.push('优化技术栈选型，避免使用过时或过于复杂的技术')
    }

    if (resource.score < 70) {
      recommendations.push('评估团队技能储备，必要时寻求外部支持或培训')
    }

    if (timeline.score < 70) {
      recommendations.push('重新评估工作量，制定更合理的时间规划')
    }

    if (scope.score < 70) {
      recommendations.push('明确需求边界，细化产品功能和用户定位')
    }

    // 根据风险等级生成建议
    if (risks.some(r => r.level === 'high')) {
      recommendations.push('优先解决高风险项，降低项目失败可能性')
    }

    // 如果所有维度都良好
    if (recommendations.length === 0) {
      recommendations.push('项目可行性良好，可以按计划推进')
    }

    return recommendations
  }

  /**
   * 从文本中提取关键词
   */
  private extractKeywords(text: string): string[] {
    // 简单的中文关键词提取（基于分词和停用词过滤）
    const stopwords = new Set([
      '的',
      '了',
      '是',
      '在',
      '就',
      '都',
      '而',
      '及',
      '与',
      '着',
      '一个',
      '没有',
      '但是',
      '如果',
      '只要',
      '只有',
      '虽然',
      '可是',
      '因为',
      '所以',
      '为了',
      '可以',
      '能够',
      '应该',
      '就会',
      '才会',
    ])

    // 简单的分词：按标点符号和空格分割
    const words = text.split(/[,，.。!！?？;；:\s]+/).filter(word => word.trim().length > 0)

    // 过滤停用词和单个字符
    const keywords = words.filter(word => word.length > 1 && !stopwords.has(word))

    return keywords.slice(0, 10) // 只返回前 10 个关键词
  }
}

/**
 * 创建默认的质量检查器实例
 */
export const defaultQualityChecker = new PRDQualityChecker()
