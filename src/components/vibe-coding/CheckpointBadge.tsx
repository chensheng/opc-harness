import { Bell } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'

interface CheckpointBadgeProps {
  count: number
  onClick?: () => void
}

export function CheckpointBadge({ count, onClick }: CheckpointBadgeProps) {
  if (count === 0) return null

  return (
    <Button
      variant="ghost"
      size="sm"
      className="relative"
      onClick={onClick}
      title={`${count} 个待审批的检查点`}
    >
      <Bell className="w-5 h-5" />
      <Badge
        variant="destructive"
        className="absolute -top-1 -right-1 h-5 w-5 flex items-center justify-center p-0 text-xs"
      >
        {count > 99 ? '99+' : count}
      </Badge>
    </Button>
  )
}
