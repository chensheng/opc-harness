/**
 * PRD 反馈和重新生成相关类型定义
 */

/** 反馈情感倾向 */
export type FeedbackSentiment = 'positive' | 'neutral' | 'negative' | 'suggestion'

/** 反馈优先级 */
export type FeedbackPriority = 'low' | 'medium' | 'high' | 'critical'

/** 用户反馈 */
export interface Feedback {
  /** 反馈 ID */
  id: string
  /** PRD ID */
  prd_id: string
  /** 反馈针对的章节 */
  section?: string
  /** 反馈内容 */
  content: string
  /** 情感倾向 */
  sentiment: FeedbackSentiment
  /** 优先级 */
  priority: FeedbackPriority
  /** 时间戳 */
  timestamp: string
}

/** 提交反馈请求 */
export interface SubmitFeedbackRequest {
  /** PRD ID */
  prd_id: string
  /** PRD 内容 */
  prd_content: string
  /** 反馈内容 */
  feedback_content: string
  /** 反馈针对的章节 */
  section?: string
  /** 当前迭代次数 */
  iteration_count: number
}

/** 提交反馈响应 */
export interface SubmitFeedbackResponse {
  /** 新的 PRD 内容 */
  new_prd_content: string
  /** 变更的章节列表 */
  changed_sections: string[]
  /** 迭代前的质量评分 */
  quality_score_before: number
  /** 迭代后的质量评分 */
  quality_score_after: number
  /** 当前迭代轮次 */
  iteration_number: number
  /** 是否成功 */
  success: boolean
}

/** 解析后的反馈 */
export interface ParsedFeedback {
  /** 反馈内容 */
  content: string
  /** 情感倾向 */
  sentiment: FeedbackSentiment
  /** 优先级 */
  priority: FeedbackPriority
  /** 改进点列表 */
  improvement_points: string[]
}

/** 反馈处理器状态 */
export interface FeedbackState {
  /** 是否正在加载 */
  isLoading: boolean
  /** 错误信息 */
  error: string | null
  /** 反馈历史 */
  feedbacks: Feedback[]
  /** 当前迭代次数 */
  iterationCount: number
  /** 最后一次再生成结果 */
  lastResult: SubmitFeedbackResponse | null
}
