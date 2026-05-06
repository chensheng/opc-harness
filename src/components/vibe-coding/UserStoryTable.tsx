import React from 'react'
import { Card, CardContent } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import { ScrollArea } from '@/components/ui/scroll-area'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { Edit2, Trash2, ArrowUpDown, ArrowUp, ArrowDown, RefreshCw, History } from 'lucide-react'
import type { UserStory } from '@/types'

interface UserStoryTableProps {
  stories: UserStory[]
  sortConfigs: Array<{ field: 'priority' | 'storyPoints'; order: 'asc' | 'desc' }>
  onSort: (field: 'priority' | 'storyPoints', event?: React.MouseEvent) => void
  getSortConfig: (
    field: 'priority' | 'storyPoints'
  ) => { field: 'priority' | 'storyPoints'; order: 'asc' | 'desc' } | undefined
  getSortIndex: (field: 'priority' | 'storyPoints') => number | null
  editingSprintStoryId: string | null
  onEditSprint: (storyId: string) => void
  onCancelQuickEdit: () => void
  onQuickUpdateSprint: (storyId: string, sprintId: string | undefined) => Promise<void>
  sprints: Array<{ id: string; name: string }>
  onEditStory: (story: UserStory) => void
  onDeleteStory: (story: UserStory) => void
  onRetryStory?: (story: UserStory) => void // 可选的重试处理函数
  onViewRetryHistory?: (storyId: string) => void // 查看重试历史
}

const priorityColors: Record<string, string> = {
  P0: 'bg-red-500 text-white',
  P1: 'bg-orange-500 text-white',
  P2: 'bg-yellow-500 text-white',
  P3: 'bg-gray-500 text-white',
}

const statusColors: Record<string, string> = {
  draft: 'bg-gray-100 text-gray-700 border border-gray-300',
  refined: 'bg-blue-50 text-blue-700 border border-blue-200',
  approved: 'bg-green-50 text-green-700 border border-green-200',
  in_development: 'bg-purple-50 text-purple-700 border border-purple-200',
  completed: 'bg-emerald-50 text-emerald-700 border border-emerald-200',
  failed: 'bg-red-50 text-red-700 border border-red-200',
  scheduled_retry: 'bg-orange-50 text-orange-700 border border-orange-200',
}

const statusLabels: Record<string, string> = {
  draft: '草稿',
  refined: '已细化',
  approved: '已批准',
  in_development: '开发中',
  completed: '已完成',
  failed: '失败',
  scheduled_retry: '等待重试',
}

export function UserStoryTable({
  stories,
  sortConfigs,
  onSort,
  getSortConfig,
  getSortIndex,
  editingSprintStoryId,
  onEditSprint,
  onCancelQuickEdit,
  onQuickUpdateSprint,
  sprints,
  onEditStory,
  onDeleteStory,
  onRetryStory,
  onViewRetryHistory,
}: UserStoryTableProps) {
  return (
    <Card>
      <CardContent className="p-0">
        <ScrollArea className="h-[calc(100vh-350px)]">
          <table className="w-full border-collapse text-xs">
            <thead className="sticky top-0 bg-muted/90 backdrop-blur-sm z-10">
              <tr className="border-b border-border">
                <th className="text-left py-1.5 px-2 font-semibold text-[10px] w-16">序号</th>
                <th
                  className="text-left py-1.5 px-2 font-semibold text-[10px] w-16 cursor-pointer hover:bg-muted/50 transition-colors select-none"
                  onClick={e => onSort('priority', e)}
                  title="点击切换排序，多次点击可添加多条件排序"
                >
                  <div className="flex items-center gap-0.5">
                    <span>优先</span>
                    {(() => {
                      const config = getSortConfig('priority')
                      const index = getSortIndex('priority')
                      if (!config) {
                        return <ArrowUpDown className="w-2.5 h-2.5 text-muted-foreground" />
                      }
                      return (
                        <div className="flex items-center gap-0.5">
                          {config.order === 'asc' ? (
                            <ArrowUp className="w-2.5 h-2.5" />
                          ) : (
                            <ArrowDown className="w-2.5 h-2.5" />
                          )}
                          {sortConfigs.length > 1 && index && (
                            <span className="text-[8px] bg-primary text-primary-foreground rounded-full w-3 h-3 flex items-center justify-center">
                              {index}
                            </span>
                          )}
                        </div>
                      )
                    })()}
                  </div>
                </th>
                <th className="text-left py-1.5 px-2 font-semibold text-[10px]">标题</th>
                <th className="text-left py-1.5 px-2 font-semibold text-[10px] w-24">角色</th>
                <th
                  className="text-left py-1.5 px-2 font-semibold text-[10px] w-16 cursor-pointer hover:bg-muted/50 transition-colors select-none"
                  onClick={e => onSort('storyPoints', e)}
                  title="点击切换排序，多次点击可添加多条件排序"
                >
                  <div className="flex items-center gap-0.5">
                    <span>点数</span>
                    {(() => {
                      const config = getSortConfig('storyPoints')
                      const index = getSortIndex('storyPoints')
                      if (!config) {
                        return <ArrowUpDown className="w-2.5 h-2.5 text-muted-foreground" />
                      }
                      return (
                        <div className="flex items-center gap-0.5">
                          {config.order === 'asc' ? (
                            <ArrowUp className="w-2.5 h-2.5" />
                          ) : (
                            <ArrowDown className="w-2.5 h-2.5" />
                          )}
                          {sortConfigs.length > 1 && index && (
                            <span className="text-[8px] bg-primary text-primary-foreground rounded-full w-3 h-3 flex items-center justify-center">
                              {index}
                            </span>
                          )}
                        </div>
                      )
                    })()}
                  </div>
                </th>
                <th className="text-left py-1.5 px-2 font-semibold text-[10px] w-16">状态</th>
                <th className="text-left py-1.5 px-2 font-semibold text-[10px] w-24">Sprint</th>
                <th className="text-left py-1.5 px-2 font-semibold text-[10px] w-16">操作</th>
              </tr>
            </thead>
            <tbody>
              {stories.map((story, index) => (
                <tr
                  key={story.id}
                  className={`border-b border-border/30 hover:bg-muted/20 transition-colors ${
                    index % 2 === 0 ? 'bg-background' : 'bg-muted/5'
                  }`}
                >
                  <td className="py-1.5 px-2 align-middle">
                    <span className="font-mono text-[10px]">{story.storyNumber}</span>
                  </td>
                  <td className="py-1.5 px-2 align-middle">
                    <Badge className={`${priorityColors[story.priority]} text-[9px] px-1 py-0 h-4`}>
                      {story.priority}
                    </Badge>
                  </td>
                  <td className="py-1.5 px-2 align-middle">
                    <div className="space-y-0.5">
                      <div className="font-medium text-xs leading-tight">{story.title}</div>
                      <div className="text-[10px] text-muted-foreground line-clamp-1">
                        {story.feature}
                      </div>
                    </div>
                  </td>
                  <td className="py-1.5 px-2 align-middle">
                    <div className="text-[10px] truncate" title={story.role}>
                      {story.role}
                    </div>
                  </td>
                  <td className="py-1.5 px-2 align-middle text-center">
                    {story.storyPoints && <span className="text-[10px]">{story.storyPoints}</span>}
                  </td>
                  <td className="py-1.5 px-2 align-middle">
                    <Badge
                      className={`${statusColors[story.status] || 'bg-gray-100 text-gray-700'} text-[9px] px-1.5 py-0 h-4 font-medium`}
                    >
                      {statusLabels[story.status] || story.status}
                    </Badge>
                    {/* 重试次数徽章 */}
                    {story.retryCount && story.retryCount > 0 && (
                      <Badge
                        variant="outline"
                        className="ml-1 text-[8px] px-1 py-0 h-3 bg-orange-50 text-orange-600 border-orange-200"
                      >
                        {story.retryCount}次重试
                      </Badge>
                    )}
                  </td>
                  <td className="py-1.5 px-2 align-middle">
                    {editingSprintStoryId === story.id ? (
                      // 快速编辑模式：显示下拉选择器
                      <Select
                        value={story.sprintId || 'none'}
                        onValueChange={value => {
                          const newSprintId = value === 'none' ? undefined : value
                          onQuickUpdateSprint(story.id, newSprintId)
                        }}
                        onOpenChange={open => {
                          if (!open) {
                            onCancelQuickEdit()
                          }
                        }}
                      >
                        <SelectTrigger className="h-6 text-[10px] px-2">
                          <SelectValue placeholder="选择 Sprint" />
                        </SelectTrigger>
                        <SelectContent>
                          <SelectItem value="none" className="text-[10px]">
                            无
                          </SelectItem>
                          {sprints.map(sprint => (
                            <SelectItem key={sprint.id} value={sprint.id} className="text-[10px]">
                              {sprint.name}
                            </SelectItem>
                          ))}
                        </SelectContent>
                      </Select>
                    ) : (
                      // 显示模式：可点击切换到编辑
                      <div
                        className="text-[10px] truncate cursor-pointer hover:bg-muted/50 rounded px-1 py-0.5 transition-colors"
                        title={sprints.find(s => s.id === story.sprintId)?.name || ''}
                        onClick={() => onEditSprint(story.id)}
                      >
                        {story.sprintId ? (
                          sprints.find(s => s.id === story.sprintId)?.name || '-'
                        ) : (
                          <span className="text-muted-foreground">未分配</span>
                        )}
                      </div>
                    )}
                  </td>
                  <td className="py-1.5 px-2 align-middle">
                    <div className="flex gap-0.5">
                      {/* 编辑按钮 */}
                      <Button
                        variant="ghost"
                        size="sm"
                        className="h-5 w-5 p-0"
                        onClick={() => onEditStory(story)}
                        title="编辑用户故事"
                      >
                        <Edit2 className="w-2.5 h-2.5" />
                      </Button>
                      {/* 查看重试历史按钮（仅在有重试记录时显示） */}
                      {story.retryCount && story.retryCount > 0 && onViewRetryHistory && (
                        <Button
                          variant="ghost"
                          size="sm"
                          className="h-5 w-5 p-0 hover:bg-blue-100 hover:text-blue-600"
                          onClick={() => onViewRetryHistory(story.id)}
                          title="查看重试历史"
                        >
                          <History className="w-2.5 h-2.5" />
                        </Button>
                      )}
                      {/* 重试按钮（仅失败状态显示） */}
                      {story.status === 'failed' && onRetryStory && (
                        <Button
                          variant="ghost"
                          size="sm"
                          className="h-5 w-5 p-0 hover:bg-orange-100 hover:text-orange-600"
                          onClick={() => onRetryStory(story)}
                          title="重试用户故事"
                        >
                          <RefreshCw className="w-2.5 h-2.5" />
                        </Button>
                      )}
                      {/* 删除按钮 */}
                      <Button
                        variant="ghost"
                        size="sm"
                        className="h-5 w-5 p-0 hover:bg-destructive/10 hover:text-destructive"
                        onClick={() => onDeleteStory(story)}
                        title="删除用户故事"
                      >
                        <Trash2 className="w-2.5 h-2.5" />
                      </Button>
                    </div>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </ScrollArea>
      </CardContent>
    </Card>
  )
}
