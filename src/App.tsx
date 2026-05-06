import { useEffect } from 'react'
import { Routes, Route } from 'react-router-dom'
import { AppLayout } from './components/common/AppLayout'
import { Dashboard } from './components/common/Dashboard'
import { IdeaInput } from './components/vibe-design/IdeaInput'
import { PRDDisplay } from './components/vibe-design/PRDDisplay'
import { UserPersonas } from './components/vibe-design/UserPersonas'
import { CompetitorAnalysis } from './components/vibe-design/CompetitorAnalysis'
import {
  CodingWorkspace,
  InitializerWorkflow,
  AgentMonitor,
  ProgressVisualization,
  LogTerminal,
} from './components/vibe-coding/CodingWorkspace'
import { CheckpointReview } from './components/vibe-coding/CheckpointReview'
import { MarketingStrategy } from './components/vibe-marketing/MarketingStrategy'
import { AIConfig } from './components/common/AIConfig'
import { Settings } from './components/common/Settings'
import { useProjectStore } from './stores'

function App() {
  const { loadProjectsFromDatabase } = useProjectStore()

  // 应用启动时从数据库加载项目
  useEffect(() => {
    loadProjectsFromDatabase()

    // Test Console Bridge functionality
    console.log('[Test] App initialized - Console Bridge test')
    console.info('[Test] This is an info message', { timestamp: new Date().toISOString() })
    console.warn('[Test] This is a warning message')
    console.error('[Test] This is an error message', new Error('Test error'))
    console.debug('[Test] Debug message with object', { user: 'test', id: 123 })

    // Test circular reference handling
    const circularObj: Record<string, unknown> = { name: 'circular' }
    Object.defineProperty(circularObj, 'self', {
      value: circularObj,
      writable: true,
      enumerable: true,
      configurable: true,
    }) // Create circular reference
    console.log('[Test] Circular reference test', circularObj)
  }, [loadProjectsFromDatabase])

  return (
    <AppLayout>
      <Routes>
        <Route path="/" element={<Dashboard />} />
        <Route path="/idea" element={<IdeaInput />} />
        <Route path="/prd/:projectId" element={<PRDDisplay />} />
        <Route path="/personas/:projectId" element={<UserPersonas />} />
        <Route path="/competitors/:projectId" element={<CompetitorAnalysis />} />
        <Route path="/coding" element={<CodingWorkspace />} />
        <Route path="/coding/:projectId" element={<CodingWorkspace />} />
        <Route path="/initializer/:projectId" element={<InitializerWorkflow />} />
        <Route path="/agent-monitor/:projectId" element={<AgentMonitor />} />
        <Route path="/progress/:projectId" element={<ProgressVisualization />} />
        <Route path="/logs/:projectId" element={<LogTerminal />} />
        {/* FileExplorer 需要在 CodingWorkspace 中使用，不单独作为路由 */}
        <Route path="/checkpoint/:projectId/:checkpointId" element={<CheckpointReview />} />
        <Route path="/marketing" element={<MarketingStrategy />} />
        <Route path="/marketing/:projectId" element={<MarketingStrategy />} />
        <Route path="/ai-config" element={<AIConfig />} />
        <Route path="/settings" element={<Settings />} />
      </Routes>
    </AppLayout>
  )
}

export default App
