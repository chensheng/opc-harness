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

function App() {
  return (
    <AppLayout>
      <Routes>
        <Route path="/" element={<Dashboard />} />
        <Route path="/idea" element={<IdeaInput />} />
        <Route path="/prd/:projectId" element={<PRDDisplay />} />
        <Route path="/personas/:projectId" element={<UserPersonas />} />
        <Route path="/competitors/:projectId" element={<CompetitorAnalysis />} />
        <Route path="/coding/:projectId" element={<CodingWorkspace />} />
        <Route path="/initializer/:projectId" element={<InitializerWorkflow />} />
        <Route path="/agent-monitor/:projectId" element={<AgentMonitor />} />
        <Route path="/progress/:projectId" element={<ProgressVisualization />} />
        <Route path="/logs/:projectId" element={<LogTerminal />} />
        <Route path="/checkpoint/:projectId/:checkpointId" element={<CheckpointReview />} />
        <Route path="/marketing/:projectId" element={<MarketingStrategy />} />
        <Route path="/ai-config" element={<AIConfig />} />
        <Route path="/settings" element={<Settings />} />
      </Routes>
    </AppLayout>
  )
}

export default App
