import { useState } from 'react'
import { useParams, useNavigate } from 'react-router-dom'
import {
  CheckCircle,
  XCircle,
  AlertTriangle,
  Clock,
  TrendingUp,
  GitBranch,
  Calendar,
  Users,
  MessageSquare,
  ChevronRight,
  Filter,
  SortAsc,
  Edit2,
  Save,
  X,
} from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import type { Milestone, Issue, Checkpoint } from '@/types'

// Mock data for CP-002 checkpoint
const mockCheckpoint: Checkpoint = {
  id: 'CP-002',
  name: '任务分解审查',
  description: '审查 Initializer Agent 生成的 Issues 列表，确保任务分解合理、优先级清晰',
  triggeredAt: new Date().toISOString(),
  agentId: 'initializer-agent-1',
  status: 'pending',
  reviewItems: [],
  autoAcceptEnabled: false,
  trustThreshold: 0.8,
}

const mockMilestones: Milestone[] = [
  {
    id: 'm-1',
    iid: 1,
    title: 'M1: 用户认证系统',
    description: '实现用户注册、登录、JWT 认证功能',
    issues: [1, 2, 3, 4],
    dueDate: '2026-04-05',
  },
  {
    id: 'm-2',
    iid: 2,
    title: 'M2: 项目管理核心',
    description: '项目 CRUD、看板管理、任务分配',
    issues: [5, 6, 7, 8, 9],
    dueDate: '2026-04-10',
  },
  {
    id: 'm-3',
    iid: 3,
    title: 'M3: 时间追踪',
    description: '番茄钟、时间记录、统计报表',
    issues: [10, 11, 12],
    dueDate: '2026-04-15',
  },
]

const mockIssues: Issue[] = [
  {
    id: 'i-1',
    iid: 1,
    title: '实现用户注册 API',
    description: '创建 POST /api/auth/register 接口，支持邮箱和密码注册',
    acceptanceCriteria: [
      '验证邮箱格式',
      '密码强度检查（至少 8 位，包含大小写和数字）',
      '邮箱唯一性验证',
      '发送验证邮件',
    ],
    priority: 'P0',
    status: 'todo',
    estimatedHours: 4,
    dependencies: [],
    labels: ['backend', 'auth', 'api'],
  },
  {
    id: 'i-2',
    iid: 2,
    title: '实现用户登录 API',
    description: '创建 POST /api/auth/login 接口，返回 JWT token',
    acceptanceCriteria: [
      '验证邮箱和密码',
      '生成 JWT token（有效期 7 天）',
      '返回用户信息和 token',
      '记录登录日志',
    ],
    priority: 'P0',
    status: 'todo',
    estimatedHours: 3,
    dependencies: [1],
    labels: ['backend', 'auth', 'api'],
  },
  {
    id: 'i-3',
    iid: 3,
    title: '实现 JWT 中间件',
    description: '创建认证中间件，验证请求中的 JWT token',
    acceptanceCriteria: [
      '解析 Authorization header',
      '验证 token 有效性',
      '过期 token 处理',
      '刷新 token 机制',
    ],
    priority: 'P0',
    status: 'todo',
    estimatedHours: 3,
    dependencies: [2],
    labels: ['backend', 'auth', 'middleware'],
  },
  {
    id: 'i-4',
    iid: 4,
    title: '实现登出功能',
    description: '创建 POST /api/auth/logout 接口，使 token 失效',
    acceptanceCriteria: ['将 token 加入黑名单', '清除客户端存储', '记录登出日志'],
    priority: 'P1',
    status: 'todo',
    estimatedHours: 2,
    dependencies: [2],
    labels: ['backend', 'auth'],
  },
  {
    id: 'i-5',
    iid: 5,
    title: '创建项目数据模型',
    description: '设计 Project、Task、User 等数据库表结构',
    acceptanceCriteria: [
      'Project 表（id, name, description, owner_id, created_at）',
      'Task 表（id, project_id, title, status, assignee_id）',
      'User-Project 关联表',
      '添加索引优化查询',
    ],
    priority: 'P0',
    status: 'todo',
    estimatedHours: 3,
    dependencies: [],
    labels: ['backend', 'database'],
  },
  {
    id: 'i-6',
    iid: 6,
    title: '实现项目 CRUD API',
    description: '创建项目的增删改查接口',
    acceptanceCriteria: [
      'POST /api/projects - 创建项目',
      'GET /api/projects - 获取项目列表',
      'GET /api/projects/:id - 获取项目详情',
      'PUT /api/projects/:id - 更新项目',
      'DELETE /api/projects/:id - 删除项目',
    ],
    priority: 'P0',
    status: 'todo',
    estimatedHours: 6,
    dependencies: [5],
    labels: ['backend', 'api'],
  },
  {
    id: 'i-7',
    iid: 7,
    title: '实现看板视图组件',
    description: '创建看板 UI，支持拖拽任务卡片',
    acceptanceCriteria: [
      '显示待办/进行中/完成三列',
      '任务卡片展示标题、负责人、优先级',
      '支持拖拽改变任务状态',
      '响应式布局',
    ],
    priority: 'P0',
    status: 'todo',
    estimatedHours: 8,
    dependencies: [6],
    labels: ['frontend', 'ui'],
  },
  {
    id: 'i-8',
    iid: 8,
    title: '实现任务分配功能',
    description: '支持将任务分配给项目成员',
    acceptanceCriteria: ['选择项目成员', '分配任务并通知', '查看我的任务列表', '任务筛选和排序'],
    priority: 'P1',
    status: 'todo',
    estimatedHours: 5,
    dependencies: [6],
    labels: ['frontend', 'backend'],
  },
  {
    id: 'i-9',
    iid: 9,
    title: '实现项目搜索和过滤',
    description: '支持按名称、状态、标签搜索项目',
    acceptanceCriteria: ['全文搜索', '按状态过滤', '按标签过滤', '搜索结果高亮'],
    priority: 'P2',
    status: 'todo',
    estimatedHours: 4,
    dependencies: [6],
    labels: ['frontend', 'backend'],
  },
  {
    id: 'i-10',
    iid: 10,
    title: '实现番茄钟计时器',
    description: '25 分钟倒计时，休息提醒',
    acceptanceCriteria: ['25 分钟工作计时', '5 分钟休息计时', '开始/暂停/重置控制', '声音提醒'],
    priority: 'P1',
    status: 'todo',
    estimatedHours: 4,
    dependencies: [],
    labels: ['frontend'],
  },
  {
    id: 'i-11',
    iid: 11,
    title: '记录时间日志',
    description: '自动记录每个任务的耗时',
    acceptanceCriteria: ['自动开始计时', '手动调整时间', '保存时间日志到数据库', '查看时间历史'],
    priority: 'P1',
    status: 'todo',
    estimatedHours: 4,
    dependencies: [10],
    labels: ['frontend', 'backend'],
  },
  {
    id: 'i-12',
    iid: 12,
    title: '生成统计报表',
    description: '可视化展示时间分配和效率',
    acceptanceCriteria: ['每日/周/月统计', '项目时间分布图', '效率趋势图', '导出 CSV 报告'],
    priority: 'P2',
    status: 'todo',
    estimatedHours: 6,
    dependencies: [11],
    labels: ['frontend', 'charts'],
  },
]

export function CheckpointReview() {
  const { projectId, checkpointId } = useParams<{ projectId: string; checkpointId: string }>()
  const navigate = useNavigate()
  const [selectedIssue, setSelectedIssue] = useState<number | null>(null)
  const [editingIssue, setEditingIssue] = useState<number | null>(null)
  const [filterPriority, setFilterPriority] = useState<string>('all')
  const [sortBy, setSortBy] = useState<string>('priority')

  // 统计数据
  const totalIssues = mockIssues.length
  const p0Count = mockIssues.filter(i => i.priority === 'P0').length
  const p1Count = mockIssues.filter(i => i.priority === 'P1').length
  const totalHours = mockIssues.reduce((sum, i) => sum + (i.estimatedHours || 0), 0)
  const hasDependencies = mockIssues.filter(i => i.dependencies && i.dependencies.length > 0).length

  // 风险识别
  const risks = []
  if (p0Count > 5) {
    risks.push({ level: 'high', message: `P0 优先级的任务过多 (${p0Count}个)，建议重新评估优先级` })
  }
  if (totalIssues > 20) {
    risks.push({
      level: 'medium',
      message: `任务总数较多 (${totalIssues}个)，考虑拆分为多个里程碑`,
    })
  }
  if (hasDependencies > totalIssues * 0.5) {
    risks.push({
      level: 'medium',
      message: `依赖关系复杂 (${hasDependencies}个任务有依赖)，可能影响并行开发`,
    })
  }

  const filteredIssues = mockIssues.filter(issue => {
    if (filterPriority !== 'all' && issue.priority !== filterPriority) return false
    return true
  })

  const sortedIssues = [...filteredIssues].sort((a, b) => {
    if (sortBy === 'priority') {
      const priorityOrder = { P0: 0, P1: 1, P2: 2, P3: 3 }
      return priorityOrder[a.priority] - priorityOrder[b.priority]
    }
    if (sortBy === 'hours') {
      return (b.estimatedHours || 0) - (a.estimatedHours || 0)
    }
    if (sortBy === 'iid') {
      return a.iid - b.iid
    }
    return 0
  })

  const handleApprove = () => {
    // TODO: Call Tauri command to approve checkpoint
    console.log('Checkpoint approved')
    navigate(`/coding/${projectId}`)
  }

  const handleReject = () => {
    // TODO: Call Tauri command to reject with feedback
    console.log('Checkpoint rejected')
  }

  const getPriorityColor = (priority: string) => {
    switch (priority) {
      case 'P0':
        return 'bg-red-500 text-white'
      case 'P1':
        return 'bg-orange-500 text-white'
      case 'P2':
        return 'bg-yellow-500 text-black'
      case 'P3':
        return 'bg-blue-500 text-white'
      default:
        return 'bg-gray-500 text-white'
    }
  }

  return (
    <div className="h-[calc(100vh-8rem)] flex flex-col overflow-hidden">
      {/* Header */}
      <div className="flex items-center justify-between mb-4 pb-4 border-b">
        <div>
          <div className="flex items-center gap-2 mb-1">
            <Badge variant="outline" className="text-sm">
              {checkpointId}
            </Badge>
            <Badge variant={mockCheckpoint.status === 'pending' ? 'default' : 'secondary'}>
              {mockCheckpoint.status === 'pending' ? '待审查' : '已审查'}
            </Badge>
          </div>
          <h1 className="text-2xl font-bold">{mockCheckpoint.name}</h1>
          <p className="text-sm text-muted-foreground mt-1">{mockCheckpoint.description}</p>
        </div>
        <div className="flex gap-2">
          <Button variant="outline" size="sm" onClick={() => navigate(`/coding/${projectId}`)}>
            <X className="w-4 h-4 mr-2" />
            取消
          </Button>
          <Button variant="destructive" size="sm" onClick={handleReject}>
            <XCircle className="w-4 h-4 mr-2" />
            拒绝修改
          </Button>
          <Button variant="default" size="sm" onClick={handleApprove}>
            <CheckCircle className="w-4 h-4 mr-2" />
            批准继续
          </Button>
        </div>
      </div>

      {/* Statistics Cards */}
      <div className="grid grid-cols-6 gap-4 mb-4 shrink-0">
        <Card className="p-4">
          <div className="flex items-center gap-2">
            <MessageSquare className="w-5 h-5 text-muted-foreground" />
            <div>
              <div className="text-2xl font-bold">{totalIssues}</div>
              <div className="text-xs text-muted-foreground">总任务数</div>
            </div>
          </div>
        </Card>
        <Card className="p-4">
          <div className="flex items-center gap-2">
            <AlertTriangle className="w-5 h-5 text-red-500" />
            <div>
              <div className="text-2xl font-bold text-red-500">{p0Count}</div>
              <div className="text-xs text-muted-foreground">P0 优先级</div>
            </div>
          </div>
        </Card>
        <Card className="p-4">
          <div className="flex items-center gap-2">
            <TrendingUp className="w-5 h-5 text-orange-500" />
            <div>
              <div className="text-2xl font-bold text-orange-500">{p1Count}</div>
              <div className="text-xs text-muted-foreground">P1 优先级</div>
            </div>
          </div>
        </Card>
        <Card className="p-4">
          <div className="flex items-center gap-2">
            <Clock className="w-5 h-5 text-blue-500" />
            <div>
              <div className="text-2xl font-bold text-blue-500">{totalHours}</div>
              <div className="text-xs text-muted-foreground">预估工时 (小时)</div>
            </div>
          </div>
        </Card>
        <Card className="p-4">
          <div className="flex items-center gap-2">
            <GitBranch className="w-5 h-5 text-purple-500" />
            <div>
              <div className="text-2xl font-bold text-purple-500">{hasDependencies}</div>
              <div className="text-xs text-muted-foreground">有依赖关系</div>
            </div>
          </div>
        </Card>
        <Card className="p-4">
          <div className="flex items-center gap-2">
            <Calendar className="w-5 h-5 text-green-500" />
            <div>
              <div className="text-2xl font-bold text-green-500">{mockMilestones.length}</div>
              <div className="text-xs text-muted-foreground">里程碑</div>
            </div>
          </div>
        </Card>
      </div>

      {/* Risk Alerts */}
      {risks.length > 0 && (
        <Card className="mb-4 p-4 bg-orange-50 border-orange-200 shrink-0">
          <div className="flex items-start gap-2">
            <AlertTriangle className="w-5 h-5 text-orange-500 mt-0.5" />
            <div className="space-y-1">
              <div className="font-medium text-orange-800">风险识别</div>
              {risks.map((risk, index) => (
                <div key={index} className="text-sm text-orange-700 flex items-center gap-1">
                  <Badge
                    variant="outline"
                    className={`text-xs ${
                      risk.level === 'high'
                        ? 'border-red-500 text-red-700'
                        : 'border-orange-500 text-orange-700'
                    }`}
                  >
                    {risk.level === 'high' ? '高' : '中'}
                  </Badge>
                  {risk.message}
                </div>
              ))}
            </div>
          </div>
        </Card>
      )}

      {/* Main Content */}
      <Tabs defaultValue="milestones" className="flex-1 min-h-0 flex flex-col">
        <TabsList className="shrink-0">
          <TabsTrigger value="milestones">里程碑视图</TabsTrigger>
          <TabsTrigger value="issues">任务列表</TabsTrigger>
          <TabsTrigger value="dependencies">依赖关系</TabsTrigger>
        </TabsList>

        <TabsContent value="milestones" className="flex-1 overflow-auto m-0">
          <div className="grid grid-cols-3 gap-4 p-4">
            {mockMilestones.map(milestone => (
              <Card key={milestone.id} className="p-4">
                <div className="flex items-start justify-between mb-2">
                  <div>
                    <h3 className="font-semibold text-lg">{milestone.title}</h3>
                    <p className="text-sm text-muted-foreground mt-1">{milestone.description}</p>
                  </div>
                  <Badge variant="outline">{milestone.issues.length} 个任务</Badge>
                </div>
                {milestone.dueDate && (
                  <div className="flex items-center gap-2 text-xs text-muted-foreground mt-3">
                    <Calendar className="w-3 h-3" />
                    截止：{new Date(milestone.dueDate).toLocaleDateString('zh-CN')}
                  </div>
                )}
                <div className="mt-3 space-y-1">
                  {milestone.issues.map(issueIid => {
                    const issue = mockIssues.find(i => i.iid === issueIid)
                    if (!issue) return null
                    return (
                      <div
                        key={issue.id}
                        className="flex items-center justify-between text-sm p-2 rounded hover:bg-accent cursor-pointer"
                        onClick={() => setSelectedIssue(issue.iid)}
                      >
                        <div className="flex items-center gap-2">
                          <span className="text-muted-foreground">#{issue.iid}</span>
                          <span className="truncate flex-1">{issue.title}</span>
                        </div>
                        <Badge className={getPriorityColor(issue.priority)} variant="secondary">
                          {issue.priority}
                        </Badge>
                      </div>
                    )
                  })}
                </div>
              </Card>
            ))}
          </div>
        </TabsContent>

        <TabsContent value="issues" className="flex-1 min-h-0 flex flex-col m-0">
          {/* Filter and Sort */}
          <div className="flex items-center justify-between p-4 border-b shrink-0">
            <div className="flex items-center gap-2">
              <Filter className="w-4 h-4 text-muted-foreground" />
              <select
                value={filterPriority}
                onChange={e => setFilterPriority(e.target.value)}
                className="text-sm border rounded px-2 py-1"
              >
                <option value="all">全部优先级</option>
                <option value="P0">P0</option>
                <option value="P1">P1</option>
                <option value="P2">P2</option>
                <option value="P3">P3</option>
              </select>
            </div>
            <div className="flex items-center gap-2">
              <SortAsc className="w-4 h-4 text-muted-foreground" />
              <select
                value={sortBy}
                onChange={e => setSortBy(e.target.value)}
                className="text-sm border rounded px-2 py-1"
              >
                <option value="priority">按优先级</option>
                <option value="hours">按工时</option>
                <option value="iid">按编号</option>
              </select>
            </div>
          </div>

          {/* Issue List */}
          <div className="flex-1 overflow-auto p-4">
            <div className="space-y-2">
              {sortedIssues.map(issue => (
                <Card
                  key={issue.id}
                  className={`p-4 cursor-pointer transition-colors ${
                    selectedIssue === issue.iid ? 'ring-2 ring-primary' : ''
                  }`}
                  onClick={() => setSelectedIssue(issue.iid)}
                >
                  <div className="flex items-start justify-between">
                    <div className="flex-1">
                      <div className="flex items-center gap-2 mb-2">
                        <span className="text-muted-foreground text-sm">#{issue.iid}</span>
                        <Badge className={getPriorityColor(issue.priority)} variant="secondary">
                          {issue.priority}
                        </Badge>
                        {issue.dependencies && issue.dependencies.length > 0 && (
                          <Badge variant="outline" className="text-xs">
                            <GitBranch className="w-3 h-3 mr-1" />
                            依赖 {issue.dependencies.join(', ')}
                          </Badge>
                        )}
                      </div>
                      <h3 className="font-medium mb-2">{issue.title}</h3>
                      <p className="text-sm text-muted-foreground mb-3">{issue.description}</p>
                      <div className="flex items-center gap-4 text-xs text-muted-foreground">
                        <div className="flex items-center gap-1">
                          <Clock className="w-3 h-3" />
                          {issue.estimatedHours} 小时
                        </div>
                        <div className="flex items-center gap-1">
                          <Users className="w-3 h-3" />
                          {issue.labels.join(', ')}
                        </div>
                      </div>
                    </div>
                    {editingIssue === issue.iid ? (
                      <div className="flex gap-2 ml-4">
                        <Button size="sm" onClick={() => setEditingIssue(null)}>
                          <Save className="w-4 h-4" />
                        </Button>
                        <Button size="sm" variant="ghost" onClick={() => setEditingIssue(null)}>
                          <X className="w-4 h-4" />
                        </Button>
                      </div>
                    ) : (
                      <Button
                        size="sm"
                        variant="ghost"
                        onClick={e => {
                          e.stopPropagation()
                          setEditingIssue(issue.iid)
                        }}
                      >
                        <Edit2 className="w-4 h-4" />
                      </Button>
                    )}
                  </div>
                </Card>
              ))}
            </div>
          </div>
        </TabsContent>

        <TabsContent value="dependencies" className="flex-1 overflow-auto m-0 p-4">
          <Card className="p-4">
            <h3 className="font-semibold mb-4">任务依赖关系图</h3>
            <div className="text-muted-foreground text-sm">
              <p>依赖关系可视化将在后续版本中实现。</p>
              <p className="mt-2">当前显示有依赖关系的任务列表：</p>
            </div>
            <div className="mt-4 space-y-2">
              {mockIssues
                .filter(i => i.dependencies && i.dependencies.length > 0)
                .map(issue => (
                  <div key={issue.id} className="flex items-center gap-2 text-sm">
                    <span className="font-medium">
                      #{issue.iid} {issue.title}
                    </span>
                    <ChevronRight className="w-4 h-4 text-muted-foreground" />
                    <span className="text-muted-foreground">依赖于</span>
                    <div className="flex gap-1">
                      {issue.dependencies!.map(depIid => {
                        return (
                          <Badge key={depIid} variant="outline">
                            #{depIid}
                          </Badge>
                        )
                      })}
                    </div>
                  </div>
                ))}
            </div>
          </Card>
        </TabsContent>
      </Tabs>

      {/* Footer Info */}
      <Card className="mt-4 p-3 bg-muted shrink-0">
        <div className="flex items-center justify-between text-xs text-muted-foreground">
          <div>
            自动接受条件：Issue 数量 &lt; 20 且 无 P0 依赖。当前状态：
            <Badge
              variant={totalIssues < 20 && p0Count === 0 ? 'default' : 'secondary'}
              className="ml-2"
            >
              {totalIssues < 20 && p0Count === 0 ? '满足' : '不满足'}
            </Badge>
          </div>
          <div>点击任务卡片可查看详情和编辑</div>
        </div>
      </Card>
    </div>
  )
}
