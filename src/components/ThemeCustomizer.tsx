import React from 'react'
import { useTheme } from '@/contexts/ThemeContext'
import { ThemeSelector } from './theme-customizer/ThemeSelector'
import { ColorSchemePicker } from './theme-customizer/ColorSchemePicker'
import { FontSizeSlider } from './theme-customizer/FontSizeSlider'
import { CardStyleConfig } from './theme-customizer/CardStyleConfig'
import { PreviewPanel } from './theme-customizer/PreviewPanel'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Palette, RotateCcw } from 'lucide-react'

interface ThemeCustomizerProps {
  onClose?: () => void
}

/**
 * 主题定制器面板组件
 */
export const ThemeCustomizer: React.FC<ThemeCustomizerProps> = ({ onClose }) => {
  const { theme, resetTheme } = useTheme()

  return (
    <Card className="w-full max-w-4xl">
      <CardHeader>
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            <Palette className="w-5 h-5" />
            <CardTitle>自定义可视化样式</CardTitle>
          </div>
          <div className="flex items-center gap-2">
            <Button variant="outline" size="sm" onClick={resetTheme} title="重置为默认主题">
              <RotateCcw className="w-4 h-4 mr-1" />
              重置
            </Button>
            {onClose && (
              <Button variant="ghost" size="sm" onClick={onClose}>
                关闭
              </Button>
            )}
          </div>
        </div>
      </CardHeader>
      <CardContent className="space-y-6">
        {/* 第一行：主题模式 + 配色方案 */}
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          <ThemeSelector />
          <ColorSchemePicker />
        </div>

        {/* 第二行：字体大小 + 卡片样式 */}
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          <FontSizeSlider />
          <CardStyleConfig />
        </div>

        {/* 第三行：实时预览 */}
        <PreviewPanel />

        {/* 当前配置显示 */}
        <div className="bg-muted rounded-lg p-4">
          <h4 className="text-sm font-medium mb-2">当前配置</h4>
          <pre className="text-xs text-muted-foreground overflow-auto">
            {JSON.stringify(theme, null, 2)}
          </pre>
        </div>
      </CardContent>
    </Card>
  )
}
