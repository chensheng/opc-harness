/**
 * Agent Appearance Generator (智能体外观生成器)
 *
 * 为每个智能体随机生成独特的外观配置
 * 包括颜色、配饰、体型、表情等
 *
 * @module components/vibe-coding/AgentAppearanceGenerator
 */

import type { AgentInfo } from './CodingWorkspaceTypes'

/**
 * 颜色调色板 (精心挑选的和谐配色方案)
 */
const COLOR_PALETTES = [
  // 海洋蓝系列
  { primary: '#3B82F6', secondary: '#1E40AF', accent: '#60A5FA' },
  { primary: '#0EA5E9', secondary: '#0284C7', accent: '#38BDF8' },
  { primary: '#06B6D4', secondary: '#0891B2', accent: '#22D3EE' },

  // 森林绿系列
  { primary: '#10B981', secondary: '#059669', accent: '#34D399' },
  { primary: '#22C55E', secondary: '#16A34A', accent: '#4ADE80' },
  { primary: '#84CC16', secondary: '#65A30D', accent: '#A3E635' },

  // 暖色系
  { primary: '#F59E0B', secondary: '#D97706', accent: '#FBBF24' },
  { primary: '#F97316', secondary: '#EA580C', accent: '#FB923C' },
  { primary: '#EF4444', secondary: '#DC2626', accent: '#F87171' },

  // 紫色系
  { primary: '#8B5CF6', secondary: '#7C3AED', accent: '#A78BFA' },
  { primary: '#A855F7', secondary: '#9333EA', accent: '#C084FC' },
  { primary: '#D946EF', secondary: '#C026D3', accent: '#E879F9' },

  // 粉色系
  { primary: '#EC4899', secondary: '#DB2777', accent: '#F472B6' },
  { primary: '#F43F5E', secondary: '#E11D48', accent: '#FB7185' },

  // 中性色系列
  { primary: '#6366F1', secondary: '#4F46E5', accent: '#818CF8' },
  { primary: '#14B8A6', secondary: '#0D9488', accent: '#2DD4BF' },
]

/**
 * 配饰类型
 */
const ACCESSORIES = [
  'none',
  'glasses', // 眼镜
  'hat', // 帽子
  'bowtie', // 领结
  'headphones', // 耳机
  'crown', // 皇冠
] as const

/**
 * 体型变体
 */
const BODY_VARIANTS = ['slim', 'normal', 'round', 'tall'] as const

/**
 * 表情风格
 */
const EXPRESSIONS = ['happy', 'serious', 'curious', 'sleepy'] as const

/**
 * 智能体外观配置类型
 */
export interface AgentAppearance {
  primaryColor: string
  secondaryColor: string
  accentColor: string
  accessory: (typeof ACCESSORIES)[number]
  bodyVariant: (typeof BODY_VARIANTS)[number]
  expression: (typeof EXPRESSIONS)[number]
}

/**
 * 使用种子生成确定性随机数 (确保同一 agentId 始终生成相同外观)
 */
function seededRandom(seed: string): () => number {
  let h = 0
  for (let i = 0; i < seed.length; i++) {
    h = (Math.imul(31, h) + seed.charCodeAt(i)) | 0
  }
  return function () {
    h = (Math.imul(31, h) + (h >>> 15)) | 0
    h ^= h >>> 16
    h = Math.imul(h, 0x45d9f3b)
    h = (h + 0x6b28d0c1) | 0
    h ^= h >>> 15
    return (h >>> 0) / 4294967296
  }
}

/**
 * 从数组中随机选择一个元素
 */
function randomPick<T>(array: readonly T[], rng: () => number): T {
  return array[Math.floor(rng() * array.length)]
}

/**
 * 为智能体生成随机外观配置
 * @param agentId 智能体 ID (用作种子,确保同一 ID 始终生成相同外观)
 * @returns 外观配置对象
 */
export function generateAgentAppearance(agentId: string): AgentAppearance {
  const rng = seededRandom(agentId)

  // 随机选择配色方案
  const palette = randomPick(COLOR_PALETTES, rng)

  // 随机选择配饰
  const accessory = randomPick(ACCESSORIES, rng)

  // 随机选择体型
  const bodyVariant = randomPick(BODY_VARIANTS, rng)

  // 随机选择表情
  const expression = randomPick(EXPRESSIONS, rng)

  return {
    primaryColor: palette.primary,
    secondaryColor: palette.secondary,
    accentColor: palette.accent,
    accessory,
    bodyVariant,
    expression,
  }
}

/**
 * 为智能体应用外观配置 (如果尚未存在)
 * @param agent 智能体信息对象
 * @returns 更新后的智能体信息对象
 */
export function applyAgentAppearance(agent: AgentInfo): AgentInfo {
  // 如果已经有外观配置,直接返回
  if (agent.appearance) {
    return agent
  }

  // 生成新的外观配置
  const appearance = generateAgentAppearance(agent.agentId)

  return {
    ...agent,
    appearance,
  }
}

/**
 * 批量为智能体列表应用外观配置
 * @param agents 智能体信息数组
 * @returns 更新后的智能体信息数组
 */
export function applyAppearancesToAgents(agents: AgentInfo[]): AgentInfo[] {
  return agents.map(applyAgentAppearance)
}
