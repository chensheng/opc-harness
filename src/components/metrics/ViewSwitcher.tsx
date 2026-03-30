import { Button } from '@/components/ui/button'
import { BarChart3, LineChart, PieChart as PieChartIcon, Grid3X3, Radar } from 'lucide-react'
import type { ViewMode } from '@/types'

interface ViewSwitcherProps {
  currentMode: ViewMode
  onChange: (mode: ViewMode) => void
}

const VIEW_MODES: Array<{ mode: ViewMode; label: string; icon: React.ReactNode }> = [
  { mode: 'bar', label: '柱状图', icon: <BarChart3 className="h-4 w-4" /> },
  { mode: 'line', label: '折线图', icon: <LineChart className="h-4 w-4" /> },
  { mode: 'pie', label: '饼图', icon: <PieChartIcon className="h-4 w-4" /> },
  { mode: 'radar', label: '雷达图', icon: <Radar className="h-4 w-4" /> },
  { mode: 'cards', label: '卡片', icon: <Grid3X3 className="h-4 w-4" /> },
]

export function ViewSwitcher({ currentMode, onChange }: ViewSwitcherProps) {
  return (
    <div className="flex items-center gap-1 border rounded-md p-1 bg-card">
      {VIEW_MODES.map(({ mode, label, icon }) => (
        <Button
          key={mode}
          variant={currentMode === mode ? 'default' : 'ghost'}
          size="sm"
          onClick={() => onChange(mode)}
          title={label}
        >
          {icon}
          <span className="ml-1 hidden lg:inline">{label}</span>
        </Button>
      ))}
    </div>
  )
}
