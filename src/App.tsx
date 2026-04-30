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
import { DecentralizedTestPage } from './pages/DecentralizedTestPage'
import { useProjectStore } from './stores'
import { useAgentLoop } from './hooks/useAgentLoop'

function App() {
  const { loadProjectsFromDatabase } = useProjectStore()
  const { startAgentLoop, checkStatus } = useAgentLoop()

  // 应用启动时从数据库加载项目
  useEffect(() => {
    loadProjectsFromDatabase()
  }, [loadProjectsFromDatabase])

  // 应用启动时自动启动 Agent Loop(如果尚未运行)
  useEffect(() => {
    const initAgentLoop = async () => {
      try {
        // 检查是否已经在运行
        const running = await checkStatus()
        
        if (!running) {
          // 获取当前选中的项目 ID(如果有)
          // TODO: 从 projectStore 获取当前项目
          const currentProjectId = localStorage.getItem('currentProjectId')
          
          if (currentProjectId) {
            console.log('[App] Starting Agent Loop for project:', currentProjectId)
            await startAgentLoop(currentProjectId, 60) // 每 60 秒检测一次
          } else {
            console.log('[App] No project selected, Agent Loop will start when project is loaded')
          }
        } else {
          console.log('[App] Agent Loop already running')
        }
      } catch (err) {
        console.error('[App] Failed to initialize Agent Loop:', err)
        // 不阻断应用启动,仅记录错误
      }
    }

    // 延迟启动,确保数据库和项目已加载
    const timer = setTimeout(initAgentLoop, 2000)
    
    return () => clearTimeout(timer)
  }, [startAgentLoop, checkStatus])

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
        <Route path="/decentralized-test" element={<DecentralizedTestPage />} />
      </Routes>
    </AppLayout>
  )
}

export default App
