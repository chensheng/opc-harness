import React from 'react'
import { useTheme } from '@/contexts/ThemeContext'
import type { CardRadius, CardShadow } from '@/types'
import { Card, CardContent } from '@/components/ui/card'
import { Label } from '@/components/ui/label'

const RADIUS_OPTIONS: Array<{ value: CardRadius; label: string }> = [
  { value: 'none', label: '无圆角' },
  { value: 'small', label: '小圆角' },
  { value: 'medium', label: '中圆角' },
  { value: 'large', label: '大圆角' },
]

const SHADOW_OPTIONS: Array<{ value: CardShadow; label: string }> = [
  { value: 'none', label: '无阴影' },
  { value: 'small', label: '小阴影' },
  { value: 'medium', label: '中阴影' },
  { value: 'large', label: '大阴影' },
]

interface OptionButtonProps {
  value: string
  label: string
  isSelected: boolean
  onClick: () => void
}

const OptionButton: React.FC<OptionButtonProps> = ({ label, isSelected, onClick }) => (
  <button
    onClick={onClick}
    className={`
      w-full px-3 py-2 text-sm rounded transition-all
      ${
        isSelected
          ? 'bg-primary text-primary-foreground'
          : 'bg-muted text-muted-foreground hover:bg-primary/20'
      }
    `}
  >
    {label}
  </button>
)

/**
 * 卡片样式配置组件
 */
export const CardStyleConfig: React.FC = () => {
  const { theme, setTheme } = useTheme()

  return (
    <Card>
      <CardContent className="pt-6">
        <div className="space-y-4">
          <h3 className="text-sm font-medium">卡片样式</h3>

          {/* 圆角选项 */}
          <div className="space-y-2">
            <Label>圆角</Label>
            <div className="grid grid-cols-4 gap-2">
              {RADIUS_OPTIONS.map(option => (
                <OptionButton
                  key={option.value}
                  value={option.value}
                  label={option.label}
                  isSelected={theme.cardRadius === option.value}
                  onClick={() => setTheme({ cardRadius: option.value })}
                />
              ))}
            </div>
          </div>

          {/* 阴影选项 */}
          <div className="space-y-2">
            <Label>阴影</Label>
            <div className="grid grid-cols-4 gap-2">
              {SHADOW_OPTIONS.map(option => (
                <OptionButton
                  key={option.value}
                  value={option.value}
                  label={option.label}
                  isSelected={theme.cardShadow === option.value}
                  onClick={() => setTheme({ cardShadow: option.value })}
                />
              ))}
            </div>
          </div>
        </div>
      </CardContent>
    </Card>
  )
}
