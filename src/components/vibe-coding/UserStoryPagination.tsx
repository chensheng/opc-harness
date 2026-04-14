import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import { ChevronLeft, ChevronRight } from 'lucide-react'

interface UserStoryPaginationProps {
  currentPage: number
  totalPages: number
  pageSize: number
  sortConfigs: Array<{ field: string; order: 'asc' | 'desc' }>
  onPageChange: (page: number) => void
  onPageSizeChange: (size: number) => void
  onClearSort: () => void
}

export function UserStoryPagination({
  currentPage,
  totalPages,
  pageSize,
  sortConfigs,
  onPageChange,
  onPageSizeChange,
  onClearSort,
}: UserStoryPaginationProps) {
  return (
    <div className="flex items-center justify-between py-1 px-1.5 border-t">
      <div className="flex items-center gap-1 text-[10px]">
        <span className="text-muted-foreground">每页:</span>
        <div className="flex gap-0.5">
          {[10, 20, 50].map(size => (
            <Button
              key={size}
              variant={pageSize === size ? 'default' : 'outline'}
              size="sm"
              onClick={() => onPageSizeChange(size)}
              className="h-5 px-1.5 text-[9px]"
            >
              {size}
            </Button>
          ))}
        </div>

        {/* 排序状态显示和清除按钮 */}
        {sortConfigs.length > 0 && (
          <>
            <span className="text-muted-foreground ml-2">|</span>
            <div className="flex items-center gap-1">
              <span className="text-muted-foreground">排序:</span>
              <div className="flex gap-0.5">
                {sortConfigs.map((config, idx) => (
                  <Badge
                    key={config.field}
                    variant="secondary"
                    className="text-[9px] h-4 px-1 flex items-center gap-0.5"
                  >
                    <span>{config.field === 'priority' ? '优先' : '点数'}</span>
                    {config.order === 'asc' ? '↑' : '↓'}
                    {sortConfigs.length > 1 && (
                      <span className="text-[8px] opacity-70">{idx + 1}</span>
                    )}
                  </Badge>
                ))}
              </div>
              <Button
                variant="ghost"
                size="sm"
                onClick={onClearSort}
                className="h-4 px-1 text-[9px] hover:bg-destructive/10 hover:text-destructive"
                title="清除所有排序"
              >
                ✕
              </Button>
            </div>
          </>
        )}
      </div>

      {totalPages > 1 && (
        <div className="flex items-center gap-0.5">
          <Button
            variant="outline"
            size="sm"
            onClick={() => onPageChange(currentPage - 1)}
            disabled={currentPage === 1}
            className="gap-0.5 h-5 px-1.5 text-[9px]"
          >
            <ChevronLeft className="w-2.5 h-2.5" />
            上页
          </Button>

          <div className="flex items-center gap-0.5">
            {Array.from({ length: Math.min(5, totalPages) }, (_, i) => {
              let pageNum
              if (totalPages <= 5) {
                pageNum = i + 1
              } else if (currentPage <= 3) {
                pageNum = i + 1
              } else if (currentPage >= totalPages - 2) {
                pageNum = totalPages - 4 + i
              } else {
                pageNum = currentPage - 2 + i
              }

              return (
                <Button
                  key={pageNum}
                  variant={currentPage === pageNum ? 'default' : 'outline'}
                  size="sm"
                  onClick={() => onPageChange(pageNum)}
                  className="w-5 h-5 p-0 text-[9px]"
                >
                  {pageNum}
                </Button>
              )
            })}
          </div>

          <Button
            variant="outline"
            size="sm"
            onClick={() => onPageChange(currentPage + 1)}
            disabled={currentPage === totalPages}
            className="gap-0.5 h-5 px-1.5 text-[9px]"
          >
            下页
            <ChevronRight className="w-2.5 h-2.5" />
          </Button>
        </div>
      )}
    </div>
  )
}
