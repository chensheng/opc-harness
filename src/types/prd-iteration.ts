/**
 * PRD 迭代相关类型定义
 */

import type { PRD } from './index'
import type { QualityReport } from './prd-quality'

/**
 * PRD 版本
 */
export interface PRDVersion {
  /** 版本 ID */
  versionId: string
  /** 时间戳 */
  timestamp: number
  /** PRD 内容 */
  prd: PRD
  /** 迭代轮数 */
  iterationNumber: number
  /** 用户反馈 */
  feedback?: string
}

/**
 * 版本差异
 */
export interface PRDDiff {
  /** 新增的功能 */
  addedFeatures: string[]
  /** 移除的功能 */
  removedFeatures: string[]
  /** 修改的字段数量 */
  modifiedFieldsCount: number
}

/**
 * 迭代响应
 */
export interface IterationResponse {
  /** 新版本 ID */
  newVersionId: string
  /** 优化后的 PRD */
  optimizedPrd: PRD
  /** 版本差异 */
  diff: PRDDiff
  /** 迭代轮数 */
  iterationNumber: number
}

/**
 * 迭代历史
 */
export interface IterationHistory {
  /** 当前版本 ID */
  currentVersionId: string
  /** 所有版本 */
  versions: PRDVersion[]
}
