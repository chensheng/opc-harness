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
 * 一致性问题类型
 */
export interface ConsistencyIssue {
  /** 问题类型 */
  type: 'contradiction' | 'inconsistency' | 'ambiguity'
  /** 涉及的章节 */
  sections: string[]
  /** 问题描述 */
  description: string
  /** 不一致的具体内容 */
  details: {
    /** 第一个描述 */
    statement1: string
    /** 第二个描述（矛盾的） */
    statement2: string
  }
  /** 改进建议 */
  suggestion: string
}

/**
 * 一致性报告
 */
export interface ConsistencyReport {
  /** 是否通过一致性检查 */
  passed: boolean
  /** 一致性得分 (0-100) */
  score: number
  /** 发现的一致性问题 */
  issues: ConsistencyIssue[]
  /** 验证通过的项 */
  verified: string[]
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
  /** 一致性报告 */
  consistency?: ConsistencyReport
  /** 可行性报告 */
  feasibility?: FeasibilityReport
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
 * 可行性风险等级
 */
export type FeasibilityRiskLevel = 'high' | 'medium' | 'low'

/**
 * 可行性风险项
 */
export interface FeasibilityRisk {
  /** 风险等级 */
  level: FeasibilityRiskLevel
  /** 风险类别 */
  category: 'technical' | 'resource' | 'timeline' | 'scope'
  /** 风险描述 */
  description: string
  /** 影响分析 */
  impact: string
  /** 缓解建议 */
  mitigation: string
}

/**
 * 可行性评估维度
 */
export interface FeasibilityDimension {
  /** 维度名称 */
  name: string
  /** 得分 (0-100) */
  score: number
  /** 评估说明 */
  assessment: string
  /** 发现的问题 */
  issues: string[]
  /** 优势项 */
  strengths: string[]
}

/**
 * 可行性报告
 */
export interface FeasibilityReport {
  /** 是否可行 */
  feasible: boolean
  /** 总体可行性得分 (0-100) */
  score: number
  /** 风险等级 */
  riskLevel: FeasibilityRiskLevel
  /** 各维度评估 */
  dimensions: {
    /** 技术可行性 */
    technical: FeasibilityDimension
    /** 资源可行性 */
    resource: FeasibilityDimension
    /** 时间可行性 */
    timeline: FeasibilityDimension
    /** 范围可行性 */
    scope: FeasibilityDimension
  }
  /** 识别的风险列表 */
  risks: FeasibilityRisk[]
  /** 建议措施 */
  recommendations: string[]
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
