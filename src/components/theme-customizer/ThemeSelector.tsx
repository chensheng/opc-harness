import React from 'react'
import { useTheme } from '@/contexts/ThemeContext'
import type { ThemeMode } from '@/types'
import { Card, CardContent } from '@/components/ui/card'
import { Label } from '@/components/ui/label'
import { Sun, Moon } from 'lucide-react'

interface ThemeOptionProps {
  mode: ThemeMode
  icon: React.ReactNode
  label: string
}

const ThemeOption: React.FC<ThemeOptionProps> = ({ mode, icon, label }) => {
  const { theme, setTheme } = useTheme()
  const isSelected = theme.mode === mode

  return (
    <button
      onClick={() => setTheme({ mode })}
      className={`
        flex flex-col items-center gap-2 p-4 rounded-lg border-2 transition-all
        ${
          isSelected
            ? 'border-primary bg-primary/10 text-primary'
            : 'border-border bg-card hover:border-primary/50'
        }
      `}
    >
      {icon}
      <Label className="cursor-pointer">{label}</Label>
    </button>
  )
}

/**
 * 主题模式选择器组件
 */
export const ThemeSelector: React.FC = () => {
  return (
    <Card>
      <CardContent className="pt-6">
        <div className="space-y-4">
          <h3 className="text-sm font-medium">主题模式</h3>
          <div className="grid grid-cols-2 gap-4">
            <ThemeOption mode="light" icon={<Sun className="w-8 h-8" />} label="明亮模式" />
            <ThemeOption mode="dark" icon={<Moon className="w-8 h-8" />} label="暗黑模式" />
          </div>
        </div>
      </CardContent>
    </Card>
  )
}
