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