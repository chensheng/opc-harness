/**
 * Office Background Component (统一办公室背景)
 *
 * 参考 Star-Office-UI 的俯视角像素办公室设计
 * 将所有区域整合到一个连贯的办公室场景中
 *
 * @module components/vibe-coding/OfficeBackground
 */

import React, { useEffect, useRef } from 'react'

/**
 * 办公室背景绘制组件
 * 使用 Canvas 绘制完整的俯视角办公室场景
 */
export function OfficeBackground({
  width = 1200,
  height = 800,
}: {
  width?: number
  height?: number
}) {
  const canvasRef = useRef<HTMLCanvasElement>(null)

  useEffect(() => {
    const canvas = canvasRef.current
    if (!canvas) return

    const ctx = canvas.getContext('2d')
    if (!ctx) return

    // 清空画布
    ctx.clearRect(0, 0, width, height)

    // 绘制办公室背景 (俯视角)
    drawOfficeBackground(ctx, width, height)
  }, [width, height])

  return (
    <canvas
      ref={canvasRef}
      width={width}
      height={height}
      className="absolute inset-0 w-full h-full"
      style={{ imageRendering: 'pixelated' }}
    />
  )
}

/**
 * 绘制完整的办公室背景
 */
function drawOfficeBackground(ctx: CanvasRenderingContext2D, width: number, height: number) {
  const pixelSize = 4 // 每个像素块的大小

  // 1. 绘制地板 (浅蓝色池塘风格 - Star-Office-UI 特色)
  drawFloor(ctx, width, height, pixelSize)

  // 2. 绘制墙壁和隔断
  drawWalls(ctx, width, height, pixelSize)

  // 3. 绘制工作区 (左侧) - 办公桌、电脑、台灯
  drawWorkArea(ctx, pixelSize)

  // 4. 绘制休息区 (中间) - 沙发、咖啡机、茶几
  drawRestArea(ctx, pixelSize)

  // 5. 绘制调试区 (右上) - 服务器机柜、警告灯
  drawDebugArea(ctx, pixelSize)

  // 6. 绘制同步区 (右下) - 文件柜、床
  drawSyncArea(ctx, pixelSize)

  // 7. 绘制装饰元素
  drawDecorations(ctx, width, height, pixelSize)
}

/**
 * 绘制地板 (池塘风格)
 */
function drawFloor(
  ctx: CanvasRenderingContext2D,
  width: number,
  height: number,
  _pixelSize: number
) {
  const baseColor = '#7FB5D0' // 浅蓝色水面
  ctx.fillStyle = baseColor
  ctx.fillRect(0, 0, width, height)

  // 添加睡莲叶子装饰
  const lilyPadColor = '#4A8B5C'
  for (let i = 0; i < 20; i++) {
    const x = Math.random() * width
    const y = Math.random() * height
    drawLilyPad(ctx, x, y, 20 + Math.random() * 15, lilyPadColor)
  }

  // 添加荷花
  const lotusColor = '#FF69B4'
  for (let i = 0; i < 8; i++) {
    const x = Math.random() * width
    const y = Math.random() * height
    drawLotus(ctx, x, y, lotusColor)
  }
}

/**
 * 绘制睡莲叶子
 */
function drawLilyPad(
  ctx: CanvasRenderingContext2D,
  x: number,
  y: number,
  size: number,
  color: string
) {
  ctx.fillStyle = color
  ctx.beginPath()
  ctx.arc(x, y, size, 0, Math.PI * 2)
  ctx.fill()
}

/**
 * 绘制荷花
 */
function drawLotus(ctx: CanvasRenderingContext2D, x: number, y: number, color: string) {
  ctx.fillStyle = color
  for (let i = 0; i < 6; i++) {
    const angle = (i * Math.PI * 2) / 6
    const petalX = x + Math.cos(angle) * 8
    const petalY = y + Math.sin(angle) * 8
    ctx.beginPath()
    ctx.arc(petalX, petalY, 5, 0, Math.PI * 2)
    ctx.fill()
  }
}

/**
 * 绘制墙壁和隔断
 */
function drawWalls(
  ctx: CanvasRenderingContext2D,
  width: number,
  height: number,
  _pixelSize: number
) {
  // 绘制墙壁边框
  const wallColor = '#8B7355'
  ctx.fillStyle = wallColor

  // 上墙
  ctx.fillRect(0, 0, width, 20)
  // 左墙
  ctx.fillRect(0, 0, 20, height)
  // 右墙
  ctx.fillRect(width - 20, 0, 20, height)
  // 下墙
  ctx.fillRect(0, height - 20, width, 20)

  // 绘制隔断 (工作区与休息区之间)
  ctx.fillStyle = '#6B5B4F'
  ctx.fillRect(380, 100, 20, 400)

  // 绘制隔断 (调试区与同步区之间)
  ctx.fillRect(750, 400, 20, 300)
}

/**
 * 绘制工作区 (左侧)
 */
function drawWorkArea(ctx: CanvasRenderingContext2D, _pixelSize: number) {
  const x = 50
  const y = 150

  // 地毯
  ctx.fillStyle = '#A8D5BA'
  ctx.fillRect(x, y, 300, 350)

  // 办公桌
  ctx.fillStyle = '#8B4513'
  ctx.fillRect(x + 50, y + 100, 180, 100)

  // 电脑显示器
  ctx.fillStyle = '#DEB887'
  ctx.fillRect(x + 100, y + 60, 80, 50)
  ctx.fillStyle = '#87CEEB' // 屏幕
  ctx.fillRect(x + 105, y + 65, 70, 40)

  // 键盘
  ctx.fillStyle = '#2F4F4F'
  ctx.fillRect(x + 90, y + 120, 100, 30)

  // 台灯
  ctx.fillStyle = '#FFD700'
  ctx.fillRect(x + 30, y + 50, 10, 70)
  ctx.beginPath()
  ctx.arc(x + 35, y + 45, 20, 0, Math.PI * 2)
  ctx.fill()

  // 书架
  ctx.fillStyle = '#654321'
  ctx.fillRect(x + 10, y + 10, 60, 120)
  // 书籍
  const bookColors = ['#FF6B6B', '#4ECDC4', '#45B7D1', '#FFA07A']
  for (let i = 0; i < 4; i++) {
    ctx.fillStyle = bookColors[i]
    ctx.fillRect(x + 15 + i * 12, y + 20, 10, 50)
  }

  // 盆栽
  drawPlant(ctx, x + 220, y + 30, 40)

  // 向日葵
  drawSunflower(ctx, x + 200, y + 80, 30)

  // 猫咪睡觉的垫子 (左下角)
  ctx.fillStyle = '#F5DEB3'
  ctx.beginPath()
  ctx.arc(x + 50, y + 300, 35, 0, Math.PI * 2)
  ctx.fill()
}

/**
 * 绘制休息区 (中间)
 */
function drawRestArea(ctx: CanvasRenderingContext2D, _pixelSize: number) {
  const x = 420
  const y = 100

  // 地毯
  ctx.fillStyle = '#B8D4E3'
  ctx.fillRect(x, y, 300, 350)

  // 沙发
  ctx.fillStyle = '#F5F5DC'
  ctx.fillRect(x + 100, y + 80, 120, 80)
  // 靠垫
  ctx.fillStyle = '#D2691E'
  ctx.fillRect(x + 130, y + 90, 60, 60)

  // 茶几
  ctx.fillStyle = '#8B4513'
  ctx.fillRect(x + 80, y + 180, 100, 60)

  // 咖啡机
  ctx.fillStyle = '#6F4E37'
  ctx.fillRect(x + 100, y + 190, 60, 40)
  // 咖啡杯
  ctx.fillStyle = '#FFFFFF'
  ctx.beginPath()
  ctx.arc(x + 120, y + 185, 8, 0, Math.PI * 2)
  ctx.fill()

  // 盆栽
  drawPlant(ctx, x + 20, y + 20, 50)
  drawPlant(ctx, x + 250, y + 150, 35)

  // 落地灯
  ctx.fillStyle = '#FFD700'
  ctx.fillRect(x + 20, y + 250, 8, 80)
  ctx.beginPath()
  ctx.arc(x + 24, y + 245, 18, 0, Math.PI * 2)
  ctx.fill()

  // 书架
  ctx.fillStyle = '#654321'
  ctx.fillRect(x + 200, y + 10, 80, 120)
  const bookColors = ['#9B59B6', '#3498DB', '#2ECC71', '#F39C12']
  for (let i = 0; i < 4; i++) {
    ctx.fillStyle = bookColors[i]
    ctx.fillRect(x + 210 + i * 15, y + 20, 12, 60)
  }
}

/**
 * 绘制调试区 (右上)
 */
function drawDebugArea(ctx: CanvasRenderingContext2D, _pixelSize: number) {
  const x = 780
  const y = 50

  // 地毯
  ctx.fillStyle = '#E8D5E0'
  ctx.fillRect(x, y, 350, 300)

  // 服务器机柜 (2个)
  ctx.fillStyle = '#2C3E50'
  ctx.fillRect(x + 30, y + 30, 80, 200)
  ctx.fillRect(x + 120, y + 30, 80, 200)

  // 服务器指示灯 (绿色闪烁效果)
  const indicatorColor = '#27AE60'
  for (let row = 0; row < 10; row++) {
    for (let col = 0; col < 2; col++) {
      ctx.fillStyle = indicatorColor
      ctx.fillRect(x + 40 + col * 90, y + 50 + row * 18, 40, 8)
    }
  }

  // 警告灯
  ctx.fillStyle = '#FF0000'
  ctx.beginPath()
  ctx.arc(x + 280, y + 80, 15, 0, Math.PI * 2)
  ctx.fill()

  // 警告标志
  ctx.fillStyle = '#FFA500'
  ctx.fillRect(x + 220, y + 100, 100, 40)
  ctx.fillStyle = '#000000'
  ctx.font = '12px monospace'
  ctx.fillText('WARNING', x + 235, y + 125)

  // 文件柜
  ctx.fillStyle = '#D2691E'
  ctx.fillRect(x + 250, y + 180, 80, 100)
  // 抽屉
  ctx.fillStyle = '#F4A460'
  for (let i = 0; i < 3; i++) {
    ctx.fillRect(x + 255, y + 190 + i * 30, 70, 20)
  }

  // 盆栽
  drawPlant(ctx, x + 150, y + 250, 40)
}

/**
 * 绘制同步区 (右下)
 */
function drawSyncArea(ctx: CanvasRenderingContext2D, _pixelSize: number) {
  const x = 780
  const y = 420

  // 地毯
  ctx.fillStyle = '#D5E8D4'
  ctx.fillRect(x, y, 350, 300)

  // 床
  ctx.fillStyle = '#F5DEB3'
  ctx.fillRect(x + 80, y + 50, 200, 120)
  // 枕头
  ctx.fillStyle = '#FFFFFF'
  ctx.fillRect(x + 90, y + 60, 40, 30)
  ctx.fillRect(x + 150, y + 60, 40, 30)
  // 被子
  ctx.fillStyle = '#FFE4B5'
  ctx.fillRect(x + 80, y + 100, 200, 70)

  // 床头柜
  ctx.fillStyle = '#8B4513'
  ctx.fillRect(x + 300, y + 80, 40, 60)

  // 台灯
  ctx.fillStyle = '#FFD700'
  ctx.fillRect(x + 315, y + 60, 10, 20)
  ctx.beginPath()
  ctx.arc(x + 320, y + 55, 15, 0, Math.PI * 2)
  ctx.fill()

  // 墙上的画 (Friends 海报)
  ctx.fillStyle = '#000000'
  ctx.fillRect(x + 100, y + 200, 120, 80)
  ctx.fillStyle = '#FFFFFF'
  ctx.font = '14px monospace'
  ctx.fillText('FRIENDS', x + 125, y + 245)

  // 盆栽
  drawPlant(ctx, x + 50, y + 100, 45)
}

/**
 * 绘制盆栽
 */
function drawPlant(ctx: CanvasRenderingContext2D, x: number, y: number, size: number) {
  // 花盆
  ctx.fillStyle = '#8B4513'
  ctx.beginPath()
  ctx.arc(x, y + size, size / 3, 0, Math.PI * 2)
  ctx.fill()

  // 植物叶子
  ctx.fillStyle = '#228B22'
  for (let i = 0; i < 5; i++) {
    const angle = (i * Math.PI * 2) / 5 - Math.PI / 2
    const leafX = x + Math.cos(angle) * size * 0.6
    const leafY = y + Math.sin(angle) * size * 0.6
    ctx.beginPath()
    ctx.ellipse(leafX, leafY, size / 4, size / 6, angle, 0, Math.PI * 2)
    ctx.fill()
  }
}

/**
 * 绘制向日葵
 */
function drawSunflower(ctx: CanvasRenderingContext2D, x: number, y: number, size: number) {
  // 花瓣
  ctx.fillStyle = '#FFD700'
  for (let i = 0; i < 12; i++) {
    const angle = (i * Math.PI * 2) / 12
    const petalX = x + Math.cos(angle) * size * 0.7
    const petalY = y + Math.sin(angle) * size * 0.7
    ctx.beginPath()
    ctx.ellipse(petalX, petalY, size / 4, size / 6, angle, 0, Math.PI * 2)
    ctx.fill()
  }

  // 花心
  ctx.fillStyle = '#8B4513'
  ctx.beginPath()
  ctx.arc(x, y, size / 3, 0, Math.PI * 2)
  ctx.fill()

  // 花瓶
  ctx.fillStyle = '#FFD700'
  ctx.fillRect(x - size / 4, y + size / 2, size / 2, size / 2)
}

/**
 * 绘制装饰元素
 */
function drawDecorations(
  _ctx: CanvasRenderingContext2D,
  _width: number,
  _height: number,
  _pixelSize: number
) {
  // 可以在这里添加更多装饰,如:
  // - 墙上的画框
  // - 地毯图案
  // - 窗帘
  // - 时钟
  // - 咖啡杯
  // - 窗帘
  // - 时钟
  // - 咖啡杯
}
