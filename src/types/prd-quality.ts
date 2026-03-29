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
