/**
 * PRD 质量检查相关类型定义
 */

/**
 * 章节评分
 */
export interface SectionScore {
  /** 是否存在 */
  present: boolean
  /** 章节得分 (0-100) */
  score: number
  /** 发现的问题 */
  issues: string[]
}

/**
 * 完整性报告 - 各章节评分
 */
export interface CompletenessReport {
  /** 总体完整性得分 (0-100) */
  score: number
  /** 各章节详细评分 */
  sections: {
    title?: SectionScore
    overview?: SectionScore
    targetUsers?: SectionScore
    coreFeatures?: SectionScore
    techStack?: SectionScore
    estimatedEffort?: SectionScore
  }
  /** 缺失的章节列表 */
  missingSections: string[]
}

/**
 * 质量问题严重程度
 */
export type IssueSeverity = 'critical' | 'major' | 'minor'

/**
 * 单个质量问题
 */
export interface QualityIssue {
  /** 严重程度 */
  severity: IssueSeverity
  /** 所属章节 */
  section: string
  /** 问题描述 */
  description: string
  /** 改进建议 */
  suggestion: string
}

/**
 * PRD 质量报告
 */
export interface PRDQualityReport {
  /** 总体评分 (0-100) */
  overallScore: number
  /** 完整性报告 */
  completeness: CompletenessReport
  /** 发现的所有质量问题 */
  issues: QualityIssue[]
  /** 改进建议列表 */
  suggestions: string[]
}

/**
 * 评分权重配置
 */
export interface ScoringWeights {
  /** 产品标题权重 */
  title: number
  /** 产品概述权重 */
  overview: number
  /** 目标用户权重 */
  targetUsers: number
  /** 核心功能权重 */
  coreFeatures: number
  /** 技术栈权重 */
  techStack: number
  /** 预估工作量权重 */
  estimatedEffort: number
}

/**
 * 默认评分权重
 */
export const DEFAULT_WEIGHTS: ScoringWeights = {
  title: 10,
  overview: 25,
  targetUsers: 20,
  coreFeatures: 25,
  techStack: 10,
  estimatedEffort: 10,
}

// ==================== PRD 一致性检查类型 ====================

/**
 * 不一致性类型
 */
export type InconsistencyType =
  | { type: 'user_not_served'; user: string }
  | {
      type: 'tech_stack_mismatch'
      feature: string
      required_techs: string[]
      missing_techs: string[]
    }
  | { type: 'effort_underestimate'; complexity_score: number; estimated_hours: number }
  | { type: 'terminology_inconsistent'; term: string; variants: string[] }
  | {
      type: 'logical_contradiction'
      section1: string
      content1: string
      section2: string
      content2: string
      contradiction: string
    }

/**
 * 不一致性问题
 */
export interface Inconsistency {
  /** 不一致性类型 */
  inconsistency_type: InconsistencyType
  /** 严重程度 */
  severity: IssueSeverity
  /** 问题描述 */
  description: string
  /** 改进建议 */
  suggestion?: string
}

/**
 * 一致性各维度评分
 */
export interface ConsistencyDimensions {
  /** 用户 - 功能对齐度 (0-100) */
  user_feature_alignment: number
  /** 技术 - 功能对齐度 (0-100) */
  tech_feature_alignment: number
  /** 工作量合理性 (0-100) */
  effort_reasonableness: number
  /** 术语一致性 (0-100) */
  terminology_consistency: number
  /** 逻辑一致性 (0-100) */
  logical_consistency: number
}

/**
 * PRD 一致性检查报告
 */
export interface PRDConsistencyReport {
  /** 总体一致性得分 (0-100) */
  overall_score: number
  /** 各维度详细评分 */
  dimensions: ConsistencyDimensions
  /** 检测到的不一致性问题 */
  inconsistencies: Inconsistency[]
  /** 改进建议列表 */
  suggestions: string[]
}
