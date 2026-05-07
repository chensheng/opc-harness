import React from 'react'
import ReactDOM from 'react-dom/client'
import { BrowserRouter } from 'react-router-dom'
import App from './App'
import './index.css'
import { initializeConsoleBridge } from './hooks/useConsoleBridge'

// ImportMeta 类型扩展
interface ImportMetaEnv {
  readonly DEV: boolean
  readonly VITE_ENABLE_CONSOLE_BRIDGE?: string
}

interface ImportMeta {
  readonly env: ImportMetaEnv
}

// Initialize Console Bridge in development mode
// This forwards frontend console logs to the Rust backend
// Auto-enabled by VITE_ENABLE_CONSOLE_BRIDGE env var or DEV mode
initializeConsoleBridge()

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <BrowserRouter>
      <App />
    </BrowserRouter>
  </React.StrictMode>
)
