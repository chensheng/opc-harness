/**
 * 用户偏好相关类型定义
 */

/**
 * 偏好模型
 */
export interface PreferenceModel {
  /** 偏好的章节顺序 */
  preferredStructure: string[]
  /** 偏好的技术栈 */
  preferredTechStack: string[]
  /** 功能复杂度偏好 (0-1) */
  preferredFeatureComplexity: number
  /** 详细程度偏好 (0-1) */
  preferredDetailLevel: number
  /** 常见修改模式 */
  commonModifications: Modification[]
  /** 反馈关键词 */
  feedbackKeywords: string[]
}

/**
 * 修改模式
 */
export interface Modification {
  /** 修改类型 */
  modificationType: string
  /** 修改内容 */
  content: string
  /** 出现次数 */
  count: number
}

/**
 * 反馈历史
 */
export interface Feedback {
  /** 反馈内容 */
  content: string
  /** 时间戳 */
  timestamp: number
  /** 反馈类型 */
  feedbackType: string
}
