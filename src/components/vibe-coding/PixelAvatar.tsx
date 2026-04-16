/**
 * Pixel Avatar Component (像素角色组件)
 *
 * 为 Vibe Coding 智能体提供拟人化的像素动画展示
 * 参考 openClaw Star Office UI 设计风格
 *
 * @module components/vibe-coding/PixelAvatar
 */

import React, { useEffect, useRef, useState } from 'react'
import type { AgentInfo } from './CodingWorkspaceTypes'

/**
 * 像素角色状态类型
 */
export type PixelAvatarState =
  | 'idle-breathe' // 空闲呼吸
  | 'running-type' // 运行中打字
  | 'paused-think' // 暂停思考
  | 'completed-celebrate' // 完成庆祝
  | 'failed-confused' // 失败困惑
  | 'walking' // 行走移动

/**
 * 智能体类型对应的颜色主题
 */
const AGENT_TYPE_COLORS = {
  initializer: {
    primary: '#3B82F6', // 蓝色
    secondary: '#1E40AF',
    accent: '#60A5FA',
  },
  coding: {
    primary: '#10B981', // 绿色
    secondary: '#059669',
    accent: '#34D399',
  },
  mr_creation: {
    primary: '#8B5CF6', // 紫色
    secondary: '#7C3AED',
    accent: '#A78BFA',
  },
}

/**
 * 状态到动画类型的映射
 */
const STATE_TO_ANIMATION: Record<AgentInfo['status'], PixelAvatarState> = {
  idle: 'idle-breathe',
  running: 'running-type',
  paused: 'paused-think',
  completed: 'completed-celebrate',
  failed: 'failed-confused',
  stopped: 'idle-breathe',
}

/**
 * PixelAvatar 组件 Props
 */
export interface PixelAvatarProps {
  agent: AgentInfo
  size?: number // 显示尺寸（默认 64px）
  showBubble?: boolean // 是否显示对话气泡
  onClick?: () => void // 点击回调
  className?: string // 自定义类名
}

/**
 * 像素网格绘制工具
 */
class PixelDrawer {
  private ctx: CanvasRenderingContext2D
  private gridSize: number
  private pixelSize: number

  constructor(ctx: CanvasRenderingContext2D, gridSize: number = 16, pixelSize: number = 4) {
    this.ctx = ctx
    this.gridSize = gridSize
    this.pixelSize = pixelSize
  }

  /**
   * 绘制单个像素
   */
  drawPixel(x: number, y: number, color: string): void {
    this.ctx.fillStyle = color
    this.ctx.fillRect(x * this.pixelSize, y * this.pixelSize, this.pixelSize, this.pixelSize)
  }

  /**
   * 批量绘制像素矩阵
   */
  drawPixels(matrix: string[][], colors: Record<string, string>): void {
    matrix.forEach((row, y) => {
      row.forEach((colorKey, x) => {
        if (colorKey && colors[colorKey]) {
          this.drawPixel(x, y, colors[colorKey])
        }
      })
    })
  }

  /**
   * 清空画布
   */
  clear(): void {
    const size = this.gridSize * this.pixelSize
    this.ctx.clearRect(0, 0, size, size)
  }
}

/**
 * 角色基础形态绘制（16x16 网格）
 */
const CHARACTER_BASE = [
  [' ', ' ', ' ', 'P', 'P', 'P', 'P', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
  [' ', ' ', 'P', 'P', 'P', 'P', 'P', 'P', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
  [' ', 'P', 'P', 'W', 'W', 'P', 'P', 'W', 'W', 'P', ' ', ' ', ' ', ' ', ' ', ' '],
  [' ', 'P', 'P', 'W', 'W', 'P', 'P', 'W', 'W', 'P', ' ', ' ', ' ', ' ', ' ', ' '],
  [' ', 'P', 'P', 'P', 'P', 'M', 'P', 'P', 'P', 'P', ' ', ' ', ' ', ' ', ' ', ' '],
  [' ', ' ', 'P', 'P', 'P', 'P', 'P', 'P', 'P', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
  [' ', ' ', ' ', 'S', 'S', 'S', 'S', 'S', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
  [' ', ' ', 'S', 'S', 'S', 'S', 'S', 'S', 'S', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
  [' ', 'S', 'S', 'S', 'S', 'S', 'S', 'S', 'S', 'S', ' ', ' ', ' ', ' ', ' ', ' '],
  ['S', 'S', 'S', 'S', 'S', 'S', 'S', 'S', 'S', 'S', 'S', ' ', ' ', ' ', ' ', ' '],
  ['S', 'S', 'S', 'S', 'S', 'S', 'S', 'S', 'S', 'S', 'S', ' ', ' ', ' ', ' ', ' '],
  [' ', 'S', 'S', 'S', 'S', 'S', 'S', 'S', 'S', 'S', ' ', ' ', ' ', ' ', ' ', ' '],
  [' ', ' ', 'L', 'L', ' ', ' ', ' ', 'L', 'L', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
  [' ', ' ', 'L', 'L', ' ', ' ', ' ', 'L', 'L', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
  [' ', ' ', 'L', 'L', ' ', ' ', ' ', 'L', 'L', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
  [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
]

/**
 * 不同状态的帧动画定义 (Star-Office-UI 风格优化)
 */
const ANIMATION_FRAMES: Record<PixelAvatarState, string[][][]> = {
  // 空闲呼吸 - 轻微的上下浮动
  'idle-breathe': [
    CHARACTER_BASE,
    CHARACTER_BASE.map((row, i) =>
      i >= 12 && i <= 14
        ? row.map((cell, j) => ((j >= 2 && j <= 3) || (j >= 7 && j <= 8) ? 'L' : cell))
        : row
    ),
    CHARACTER_BASE,
  ],

  // 运行中打字 - 手部快速移动 + 键盘闪烁
  'running-type': [
    CHARACTER_BASE,
    CHARACTER_BASE.map((row, i) =>
      i === 9 || i === 10
        ? row.map((cell, j) => (j === 2 || j === 3 || j === 7 || j === 8 ? 'K' : cell))
        : row
    ),
    CHARACTER_BASE.map((row, i) =>
      i === 9 || i === 10
        ? row.map((cell, j) => (j === 1 || j === 4 || j === 6 || j === 9 ? 'K' : cell))
        : row
    ),
    CHARACTER_BASE.map((row, i) =>
      i === 2 || i === 3
        ? row.map((cell, j) => (j === 3 || j === 4 || j === 6 || j === 7 ? 'W' : cell))
        : row
    ),
  ],

  // 暂停思考 - 头顶出现思考泡泡
  'paused-think': [
    CHARACTER_BASE,
    CHARACTER_BASE.map((row, i) =>
      i === 0
        ? [' ', ' ', ' ', ' ', 'T', 'T', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ']
        : row
    ),
    CHARACTER_BASE.map((row, i) =>
      i === 0
        ? [' ', ' ', ' ', 'T', 'T', 'T', 'T', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ']
        : row
    ),
    CHARACTER_BASE.map((row, i) =>
      i === 2 || i === 3
        ? row.map((cell, j) => (j === 3 || j === 4 || j === 6 || j === 7 ? 'T' : cell))
        : row
    ),
  ],

  // 完成庆祝 - 跳跃动作 + 星星特效
  'completed-celebrate': [
    CHARACTER_BASE,
    CHARACTER_BASE.map((row, i) => (i >= 1 && i <= 3 ? row.map(() => ' ') : row)),
    CHARACTER_BASE.map((row, i) =>
      i === 0
        ? [' ', ' ', '*', ' ', ' ', '*', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ']
        : i >= 2 && i <= 4
          ? row.map(() => ' ')
          : row
    ),
    CHARACTER_BASE.map((row, i) =>
      i === 0
        ? [' ', '*', ' ', '*', ' ', ' ', '*', ' ', '*', ' ', ' ', ' ', ' ', ' ', ' ', ' ']
        : row
    ),
  ],

  // 失败困惑 - 眼睛变成 XX,头顶问号
  'failed-confused': [
    CHARACTER_BASE.map((row, i) =>
      i === 2 || i === 3
        ? row.map((cell, j) => (j === 3 || j === 4 || j === 6 || j === 7 ? 'X' : cell))
        : row
    ),
    CHARACTER_BASE.map((row, i) =>
      i === 0
        ? [' ', ' ', ' ', ' ', '?', '?', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ']
        : i === 2 || i === 3
          ? row.map((cell, j) => (j === 3 || j === 4 || j === 6 || j === 7 ? 'X' : cell))
          : row
    ),
    CHARACTER_BASE.map((row, i) =>
      i === 0
        ? [' ', ' ', ' ', '?', '?', '?', '?', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ']
        : row
    ),
  ],

  // 行走移动 - 腿部交替摆动
  walking: [
    CHARACTER_BASE,
    CHARACTER_BASE.map((row, i) =>
      i === 12 || i === 13
        ? row.map((cell, j) => (j === 2 || j === 3 ? 'L' : j === 7 || j === 8 ? ' ' : cell))
        : row
    ),
    CHARACTER_BASE.map((row, i) =>
      i === 12 || i === 13
        ? row.map((cell, j) => (j === 2 || j === 3 ? ' ' : j === 7 || j === 8 ? 'L' : cell))
        : row
    ),
  ],
}

/**
 * 特殊符号的颜色映射
 */
const SPECIAL_COLORS: Record<string, string> = {
  W: '#FFFFFF', // 白色眼睛
  M: '#FF6B6B', // 红色嘴巴
  K: '#FFD93D', // 黄色键盘
  T: '#87CEEB', // 天蓝色思考泡泡
  X: '#FF0000', // 红色错误叉号
  '?': '#FFA500', // 橙色问号
  '*': '#FFD700', // 金色星星
  L: '#8B4513', // 棕色腿部
}

/**
 * PixelAvatar 组件实现
 */
export function PixelAvatar({
  agent,
  size = 64,
  showBubble = true,
  onClick,
  className = '',
}: PixelAvatarProps) {
  const canvasRef = useRef<HTMLCanvasElement>(null)
  const [_currentFrame, setCurrentFrame] = useState(0) // eslint-disable-line @typescript-eslint/no-unused-vars
  const animationState = STATE_TO_ANIMATION[agent.status]

  useEffect(() => {
    const canvas = canvasRef.current
    if (!canvas) return

    const ctx = canvas.getContext('2d')
    if (!ctx) return

    const drawer = new PixelDrawer(ctx, 16, size / 16)
    const colors = {
      P: AGENT_TYPE_COLORS[agent.type].primary,
      S: AGENT_TYPE_COLORS[agent.type].secondary,
      L: AGENT_TYPE_COLORS[agent.type].accent,
      ...SPECIAL_COLORS,
    }

    let frameIndex = 0
    const frames = ANIMATION_FRAMES[animationState]
    const fps = 8
    const frameDuration = 1000 / fps

    const animate = () => {
      drawer.clear()
      drawer.drawPixels(frames[frameIndex], colors)

      frameIndex = (frameIndex + 1) % frames.length
      setCurrentFrame(frameIndex)
    }

    const interval = setInterval(animate, frameDuration)

    return () => clearInterval(interval)
  }, [agent.type, agent.status, size, animationState])

  // 生成状态提示文本
  const getStatusText = (): string => {
    switch (agent.status) {
      case 'running':
        return agent.currentTask || '工作中...'
      case 'paused':
        return '已暂停'
      case 'completed':
        return '已完成 ✓'
      case 'failed':
        return '遇到错误 ✗'
      case 'idle':
        return '待命中'
      default:
        return ''
    }
  }

  return (
    <div className={`relative inline-block ${className}`} onClick={onClick}>
      {/* 像素角色 Canvas */}
      <canvas
        ref={canvasRef}
        width={size}
        height={size}
        className="cursor-pointer transition-transform hover:scale-110"
        style={{ imageRendering: 'pixelated' }}
        title={`${agent.name || agent.type} - ${agent.status}`}
      />

      {/* 状态标签 */}
      <div
        className={`absolute -bottom-1 left-1/2 transform -translate-x-1/2 px-2 py-0.5 text-xs font-bold rounded-full whitespace-nowrap`}
        style={{
          backgroundColor: AGENT_TYPE_COLORS[agent.type].primary,
          color: 'white',
        }}
      >
        {agent.type === 'initializer' ? '初始化' : agent.type === 'coding' ? '编码' : 'MR'}
      </div>

      {/* 对话气泡 */}
      {showBubble && agent.status === 'running' && (
        <div className="absolute -top-8 left-1/2 transform -translate-x-1/2 bg-white dark:bg-gray-800 border-2 border-gray-300 dark:border-gray-600 rounded-lg px-2 py-1 text-xs shadow-lg animate-pulse max-w-[120px] text-center">
          {getStatusText()}
          <div className="absolute -bottom-1 left-1/2 transform -translate-x-1/2 w-2 h-2 bg-white dark:bg-gray-800 border-r-2 border-b-2 border-gray-300 dark:border-gray-600 rotate-45"></div>
        </div>
      )}

      {/* 进度指示器 */}
      {agent.status === 'running' && agent.progress > 0 && (
        <div className="absolute -right-2 top-0 w-6 h-6 rounded-full bg-gradient-to-br from-blue-500 to-purple-500 flex items-center justify-center text-[10px] font-bold text-white shadow-md">
          {Math.round(agent.progress)}%
        </div>
      )}
    </div>
  )
}
