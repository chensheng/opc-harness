import { useEffect } from 'react'
import { Card } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import { Progress } from '@/components/ui/progress'
import { CheckCircle2, XCircle, Package, ExternalLink, RefreshCw } from 'lucide-react'
import { useToolDetector } from '@/hooks/useToolDetector'

export function ToolDetector() {
  const { tools, isLoading, error, detectTools, installedCount, totalCount } = useToolDetector()

  useEffect(() => {
    // 初始加载时自动检测
    detectTools()
  }, [detectTools])

  const progress = totalCount > 0 ? (installedCount / totalCount) * 100 : 0

  return (
    <Card className="p-6">
      <div className="flex items-center justify-between mb-4">
        <div className="flex items-center gap-2">
          <Package className="h-5 w-5 text-primary" />
          <h2 className="text-xl font-semibold">本地工具检测</h2>
        </div>
        <Button variant="outline" size="sm" onClick={detectTools} disabled={isLoading}>
          <RefreshCw className={`h-4 w-4 mr-2 ${isLoading ? 'animate-spin' : ''}`} />
          重新检测
        </Button>
      </div>

      {/* 进度显示 */}
      <div className="mb-6">
        <div className="flex items-center justify-between mb-2">
          <span className="text-sm text-muted-foreground">
            已安装 {installedCount} / {totalCount}
          </span>
          <Badge variant={installedCount === totalCount ? 'default' : 'secondary'}>
            {Math.round(progress)}%
          </Badge>
        </div>
        <Progress value={progress} className="h-2" />
      </div>

      {/* 错误提示 */}
      {error && (
        <div className="mb-4 p-3 bg-destructive/10 text-destructive rounded-md text-sm">
          {error}
        </div>
      )}

      {/* 工具列表 */}
      <div className="space-y-3">
        {tools.map(tool => (
          <div
            key={tool.name}
            className="flex items-center justify-between p-3 border rounded-md hover:bg-accent/50 transition-colors"
          >
            <div className="flex items-center gap-3">
              {tool.is_installed ? (
                <CheckCircle2 className="h-5 w-5 text-green-500" />
              ) : (
                <XCircle className="h-5 w-5 text-red-500" />
              )}

              <div>
                <div className="font-medium">{tool.name}</div>
                {tool.version && (
                  <div className="text-xs text-muted-foreground">版本：{tool.version}</div>
                )}
              </div>
            </div>

            <div className="flex items-center gap-2">
              <Badge variant={tool.is_installed ? 'default' : 'outline'}>
                {tool.is_installed ? '已安装' : '未安装'}
              </Badge>

              {tool.install_url && (
                <a
                  href={tool.install_url}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="text-primary hover:underline"
                >
                  <ExternalLink className="h-4 w-4" />
                </a>
              )}
            </div>
          </div>
        ))}
      </div>

      {/* 空状态 */}
      {tools.length === 0 && !isLoading && (
        <div className="text-center py-8 text-muted-foreground">
          点击"重新检测"按钮开始检测本地工具
        </div>
      )}
    </Card>
  )
}
