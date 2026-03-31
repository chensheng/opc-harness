import React from 'react'
import { useTheme } from '@/contexts/ThemeContext'
import type { FontSize } from '@/types'
import { Card, CardContent } from '@/components/ui/card'
import { Label } from '@/components/ui/label'
import { Slider } from '@/components/ui/slider'

const FONT_SIZE_LABELS: Record<FontSize, string> = {
  small: '小',
  medium: '中',
  large: '大',
}

const FONT_SIZE_VALUES: Record<FontSize, string> = {
  small: 'text-sm',
  medium: 'text-base',
  large: 'text-lg',
}

/**
 * 字体大小滑块组件
 */
export const FontSizeSlider: React.FC = () => {
  const { theme, setTheme } = useTheme()

  const fontSizeIndex = Object.keys(FONT_SIZE_LABELS).indexOf(theme.fontSize)

  return (
    <Card>
      <CardContent className="pt-6">
        <div className="space-y-4">
          <h3 className="text-sm font-medium">字体大小</h3>
          <div className="px-2">
            <Slider
              value={[fontSizeIndex]}
              min={0}
              max={Object.keys(FONT_SIZE_LABELS).length - 1}
              step={1}
              onValueChange={([index]) => {
                const size = Object.keys(FONT_SIZE_LABELS)[index] as FontSize
                setTheme({ fontSize: size })
              }}
              className="w-full"
            />
            <div className="flex justify-between mt-2">
              {Object.entries(FONT_SIZE_LABELS).map(([value, label]) => (
                <Label
                  key={value}
                  className={`
                    cursor-pointer text-xs px-3 py-1 rounded transition-all
                    ${
                      theme.fontSize === value
                        ? 'bg-primary text-primary-foreground'
                        : 'bg-muted text-muted-foreground hover:bg-primary/20'
                    }
                    ${FONT_SIZE_VALUES[value as FontSize]}
                  `}
                  onClick={() => setTheme({ fontSize: value as FontSize })}
                >
                  {label}
                </Label>
              ))}
            </div>
          </div>
        </div>
      </CardContent>
    </Card>
  )
}
