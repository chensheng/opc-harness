import { useState } from 'react'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Switch } from '@/components/ui/switch'
import { Card, CardContent } from '@/components/ui/card'
import { Filter, X } from 'lucide-react'
import type { DataFilters } from '@/types'

interface DataFilterProps {
  filters: DataFilters
  onChange: (filters: DataFilters) => void
}

export function DataFilter({ filters, onChange }: DataFilterProps) {
  const [isOpen, setIsOpen] = useState(false)
  const [localFilters, setLocalFilters] = useState<DataFilters>(filters)

  const handleApply = () => {
    onChange(localFilters)
  }

  const handleReset = () => {
    setLocalFilters({})
    onChange({})
  }

  const hasActiveFilters = Object.keys(filters).length > 0

  return (
    <Card className={isOpen ? 'border-primary' : ''}>
      <CardContent className="p-4 space-y-4">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            <Button variant="ghost" size="sm" onClick={() => setIsOpen(!isOpen)}>
              <Filter className="h-4 w-4 mr-2" />
              数据过滤
              {hasActiveFilters && (
                <span className="ml-2 bg-primary text-primary-foreground rounded-full w-5 h-5 flex items-center justify-center text-xs">
                  {Object.keys(filters).length}
                </span>
              )}
            </Button>
            {hasActiveFilters && (
              <Button variant="ghost" size="sm" onClick={handleReset}>
                <X className="h-4 w-4" />
              </Button>
            )}
          </div>
        </div>

        {isOpen && (
          <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3 pt-4 border-t">
            <div className="space-y-2">
              <Label htmlFor="minMarketShare">最小市场份额 (%)</Label>
              <Input
                id="minMarketShare"
                type="number"
                placeholder="例如：20"
                value={localFilters.minMarketShare || ''}
                onChange={e =>
                  setLocalFilters({
                    ...localFilters,
                    minMarketShare: e.target.value ? parseFloat(e.target.value) : undefined,
                  })
                }
              />
            </div>

            <div className="space-y-2">
              <Label htmlFor="maxEmployeeCount">最大员工数</Label>
              <Input
                id="maxEmployeeCount"
                type="number"
                placeholder="例如：1000"
                value={localFilters.maxEmployeeCount || ''}
                onChange={e =>
                  setLocalFilters({
                    ...localFilters,
                    maxEmployeeCount: e.target.value ? parseInt(e.target.value) : undefined,
                  })
                }
              />
            </div>

            <div className="space-y-2">
              <Label htmlFor="foundedAfter">成立年份之后</Label>
              <Input
                id="foundedAfter"
                type="number"
                placeholder="例如：2010"
                value={localFilters.foundedAfter || ''}
                onChange={e =>
                  setLocalFilters({
                    ...localFilters,
                    foundedAfter: e.target.value ? parseInt(e.target.value) : undefined,
                  })
                }
              />
            </div>

            <div className="flex items-center space-x-2">
              <Switch
                id="hasFunding"
                checked={localFilters.hasFunding || false}
                onCheckedChange={checked =>
                  setLocalFilters({
                    ...localFilters,
                    hasFunding: checked || undefined,
                  })
                }
              />
              <Label htmlFor="hasFunding">只显示有融资的公司</Label>
            </div>

            <div className="flex items-center gap-2">
              <Button onClick={handleApply} size="sm" className="w-full">
                应用过滤
              </Button>
            </div>
          </div>
        )}
      </CardContent>
    </Card>
  )
}
