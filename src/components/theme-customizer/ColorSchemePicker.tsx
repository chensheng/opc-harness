import React from 'react'
import { useTheme } from '@/contexts/ThemeContext'
import type { ColorScheme } from '@/types'
import { Card, CardContent } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'

interface ColorOptionProps {
  scheme: ColorScheme
  colors: string[]
}

const ColorOption: React.FC<ColorOptionProps> = ({ scheme, colors }) => {
  const { theme, setTheme } = useTheme()
  const isSelected = theme.colorScheme === scheme

  return (
    <button
      onClick={() => setTheme({ colorScheme: scheme })}
      className={`
        flex flex-col gap-2 p-3 rounded-lg border-2 transition-all w-full
        ${
          isSelected
            ? 'border-primary bg-primary/10'
            : 'border-border bg-card hover:border-primary/50'
        }
      `}
    >
      <div className="flex gap-1">
        {colors.map((color, index) => (
          <div key={index} className="w-6 h-6 rounded" style={{ backgroundColor: color }} />
        ))}
      </div>
      <Badge variant={isSelected ? 'default' : 'secondary'} className="capitalize">
        {scheme}
      </Badge>
    </button>
  )
}

/**
 * 配色方案选择器组件
 */
export const ColorSchemePicker: React.FC = () => {
  const colorSchemes: Record<ColorScheme, string[]> = {
    blue: ['#dbeafe', '#93c5fd', '#3b82f6', '#1d4ed8'],
    green: ['#dcfce7', '#86efac', '#22c55e', '#15803d'],
    purple: ['#f3e8ff', '#d8b4fe', '#a855f7', '#6b21a8'],
    orange: ['#ffedd5', '#fdba74', '#f97316', '#c2410c'],
  }

  return (
    <Card>
      <CardContent className="pt-6">
        <div className="space-y-4">
          <h3 className="text-sm font-medium">配色方案</h3>
          <div className="grid grid-cols-2 gap-4">
            {(Object.keys(colorSchemes) as ColorScheme[]).map(scheme => (
              <ColorOption key={scheme} scheme={scheme} colors={colorSchemes[scheme]} />
            ))}
          </div>
        </div>
      </CardContent>
    </Card>
  )
}
