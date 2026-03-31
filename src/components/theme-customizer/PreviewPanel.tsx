import React from 'react'
import { useTheme } from '@/contexts/ThemeContext'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { TrendingUp, Users, Target } from 'lucide-react'

/**
 * 实时预览面板组件
 */
export const PreviewPanel: React.FC = () => {
  const { theme } = useTheme()

  // 根据主题配置计算样式类
  const getRadiusClass = () => {
    const radiusMap: Record<string, string> = {
      none: 'rounded-none',
      small: 'rounded',
      medium: 'rounded-md',
      large: 'rounded-lg',
    }
    return radiusMap[theme.cardRadius] || 'rounded-md'
  }

  const getShadowClass = () => {
    const shadowMap: Record<string, string> = {
      none: 'shadow-none',
      small: 'shadow-sm',
      medium: 'shadow-md',
      large: 'shadow-lg',
    }
    return shadowMap[theme.cardShadow] || 'shadow-md'
  }

  const getFontSizeClass = () => {
    const sizeMap: Record<string, string> = {
      small: 'text-sm',
      medium: 'text-base',
      large: 'text-lg',
    }
    return sizeMap[theme.fontSize] || 'text-base'
  }

  const getColorClasses = () => {
    const colorMap: Record<string, { bg: string; text: string }> = {
      blue: { bg: 'bg-blue-500', text: 'text-blue-600' },
      green: { bg: 'bg-green-500', text: 'text-green-600' },
      purple: { bg: 'bg-purple-500', text: 'text-purple-600' },
      orange: { bg: 'bg-orange-500', text: 'text-orange-600' },
    }
    return colorMap[theme.colorScheme] || colorMap.blue
  }

  const radiusClass = getRadiusClass()
  const shadowClass = getShadowClass()
  const fontSizeClass = getFontSizeClass()
  const colors = getColorClasses()

  return (
    <Card className={`${radiusClass} ${shadowClass}`}>
      <CardHeader>
        <CardTitle className={fontSizeClass}>实时预览效果</CardTitle>
      </CardHeader>
      <CardContent className="space-y-4">
        {/* 示例卡片 1 - 指标卡 */}
        <div className={`p-4 border ${radiusClass} ${shadowClass} bg-card`}>
          <div className="flex items-center gap-3">
            <div className={`p-2 ${colors.bg} text-white ${radiusClass}`}>
              <TrendingUp className="w-5 h-5" />
            </div>
            <div>
              <p className={`text-muted-foreground ${fontSizeClass}`}>市场份额</p>
              <p className={`font-semibold ${colors.text} ${fontSizeClass}`}>35.7%</p>
            </div>
          </div>
        </div>

        {/* 示例卡片 2 - 用户画像 */}
        <div className={`p-4 border ${radiusClass} ${shadowClass} bg-card`}>
          <div className="flex items-start justify-between">
            <div className="flex items-center gap-3">
              <div
                className={`w-10 h-10 ${colors.bg} text-white flex items-center justify-center ${radiusClass}`}
              >
                <Users className="w-5 h-5" />
              </div>
              <div>
                <p className={`font-medium ${fontSizeClass}`}>产品经理</p>
                <p className={`text-sm text-muted-foreground`}>张三 · 3 年经验</p>
              </div>
            </div>
            <Badge className={colors.bg}>核心用户</Badge>
          </div>
        </div>

        {/* 示例卡片 3 - 功能特性 */}
        <div className={`p-4 border ${radiusClass} ${shadowClass} bg-card`}>
          <div className="flex items-center gap-3 mb-2">
            <div className={`p-2 ${colors.bg} text-white ${radiusClass}`}>
              <Target className="w-5 h-5" />
            </div>
            <p className={`font-medium ${fontSizeClass}`}>核心功能</p>
          </div>
          <ul className={`list-disc list-inside space-y-1 text-muted-foreground ${fontSizeClass}`}>
            <li>PRD 自动生成</li>
            <li>质量检查系统</li>
            <li>迭代优化流程</li>
          </ul>
        </div>

        {/* 配置信息 */}
        <div className={`p-3 ${colors.bg} bg-opacity-10 ${radiusClass}`}>
          <p className={`text-xs ${colors.text} font-medium`}>
            当前：{theme.mode === 'light' ? '明亮' : '暗黑'}模式 ·{theme.colorScheme}配色 ·
            {theme.fontSize === 'small' ? '小' : theme.fontSize === 'medium' ? '中' : '大'}字体
          </p>
        </div>
      </CardContent>
    </Card>
  )
}
