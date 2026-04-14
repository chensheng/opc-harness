import React from 'react'
import { Card, CardContent } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Search, X } from 'lucide-react'
import type { UserStory } from '@/types'

interface UserStoryFilterBarProps {
  filterKeyword: string
  onFilterKeywordChange: (keyword: string) => void
  filterStatus: string
  onFilterStatusChange: (status: string) => void
  filterPriority: string
  onFilterPriorityChange: (priority: string) => void
  filterSprint: string
  onFilterSprintChange: (sprint: string) => void
  displayStories: UserStory[]
  filteredStories: UserStory[]
  sprints: Array<{ id: string; name: string }>
  onClearFilters: () => void
}

const statusLabels: Record<string, string> = {
  draft: '草稿',
  refined: '已细化',
  approved: '已批准',
  in_development: '开发中',
  completed: '已完成',
}

export function UserStoryFilterBar({
  filterKeyword,
  onFilterKeywordChange,
  filterStatus,
  onFilterStatusChange,
  filterPriority,
  onFilterPriorityChange,
  filterSprint,
  onFilterSprintChange,
  displayStories,
  filteredStories,
  sprints,
  onClearFilters,
}: UserStoryFilterBarProps) {
  // 获取所有唯一的筛选选项
  const uniqueStatuses = React.useMemo(() => {
    const statuses = new Set(displayStories.map(s => s.status))
    return Array.from(statuses).sort()
  }, [displayStories])

  const uniquePriorities = React.useMemo(() => {
    const priorities = new Set(displayStories.map(s => s.priority))
    return Array.from(priorities).sort()
  }, [displayStories])

  return (
    <Card>
      <CardContent className="p-2">
        <div className="flex items-center gap-2">
          {/* 关键词搜索 */}
          <div className="relative flex-1 max-w-xs">
            <Search className="absolute left-2 top-1/2 -translate-y-1/2 w-3 h-3 text-muted-foreground" />
            <input
              type="text"
              placeholder="搜索标题、功能、角色..."
              value={filterKeyword}
              onChange={e => onFilterKeywordChange(e.target.value)}
              className="w-full pl-7 pr-7 py-1 text-[10px] border rounded bg-background focus:outline-none focus:ring-1 focus:ring-primary"
            />
            {filterKeyword && (
              <button
                onClick={() => onFilterKeywordChange('')}
                className="absolute right-1.5 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground"
              >
                <X className="w-3 h-3" />
              </button>
            )}
          </div>

          {/* 状态筛选 */}
          <select
            value={filterStatus}
            onChange={e => onFilterStatusChange(e.target.value)}
            className="px-2 py-1 text-[10px] border rounded bg-background focus:outline-none focus:ring-1 focus:ring-primary"
          >
            <option value="">所有状态</option>
            {uniqueStatuses.map(status => (
              <option key={status} value={status}>
                {statusLabels[status] || status}
              </option>
            ))}
          </select>

          {/* 优先级筛选 */}
          <select
            value={filterPriority}
            onChange={e => onFilterPriorityChange(e.target.value)}
            className="px-2 py-1 text-[10px] border rounded bg-background focus:outline-none focus:ring-1 focus:ring-primary"
          >
            <option value="">所有优先级</option>
            {uniquePriorities.map(priority => (
              <option key={priority} value={priority}>
                {priority}
              </option>
            ))}
          </select>

          {/* Sprint筛选 */}
          <select
            value={filterSprint}
            onChange={e => onFilterSprintChange(e.target.value)}
            className="px-2 py-1 text-[10px] border rounded bg-background focus:outline-none focus:ring-1 focus:ring-primary max-w-[150px]"
            title="按Sprint筛选"
          >
            <option value="">所有Sprint</option>
            <option value="unassigned">未分配</option>
            {sprints.map(sprint => (
              <option key={sprint.id} value={sprint.id}>
                {sprint.name}
              </option>
            ))}
          </select>

          {/* 清除筛选按钮 */}
          {(filterKeyword || filterStatus || filterPriority || filterSprint) && (
            <Button
              variant="ghost"
              size="sm"
              onClick={onClearFilters}
              className="h-6 px-2 text-[10px]"
            >
              <X className="w-3 h-3 mr-1" />
              清除
            </Button>
          )}

          {/* 筛选结果统计 */}
          <span className="text-[10px] text-muted-foreground ml-auto">
            {filteredStories.length} / {displayStories.length} 条
          </span>
        </div>
      </CardContent>
    </Card>
  )
}
