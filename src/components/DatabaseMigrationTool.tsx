import { useState } from 'react'
import { Button } from '@/components/ui/button'
import { invoke } from '@tauri-apps/api/core'

export function DatabaseMigrationTool() {
  const [isRunning, setIsRunning] = useState(false)
  const [result, setResult] = useState<string | null>(null)
  const [error, setError] = useState<string | null>(null)

  const runMigration = async () => {
    setIsRunning(true)
    setResult(null)
    setError(null)

    try {
      console.log('[Migration] Starting database migration...')
      const message = await invoke<string>('run_database_migration')
      console.log('[Migration] Success:', message)
      setResult(message)
    } catch (err) {
      console.error('[Migration] Failed:', err)
      setError(String(err))
    } finally {
      setIsRunning(false)
    }
  }

  return (
    <div className="p-4 border rounded-lg bg-muted/50">
      <h3 className="text-lg font-semibold mb-2">数据库迁移工具</h3>
      <p className="text-sm text-muted-foreground mb-4">
        为 user_stories 表添加 sprint_id 字段支持（一次性操作）
      </p>

      <Button onClick={runMigration} disabled={isRunning} variant={result ? 'outline' : 'default'}>
        {isRunning ? '执行中...' : '执行迁移'}
      </Button>

      {result && (
        <div className="mt-4 p-3 bg-green-100 dark:bg-green-900/20 border border-green-300 rounded">
          <p className="text-green-800 dark:text-green-200 text-sm">✓ {result}</p>
        </div>
      )}

      {error && (
        <div className="mt-4 p-3 bg-red-100 dark:bg-red-900/20 border border-red-300 rounded">
          <p className="text-red-800 dark:text-red-200 text-sm">✗ {error}</p>
        </div>
      )}
    </div>
  )
}
