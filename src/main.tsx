import React from 'react'
import ReactDOM from 'react-dom/client'
import { BrowserRouter } from 'react-router-dom'
import App from './App'
import './index.css'
import { initializeConsoleBridge } from './hooks/useConsoleBridge'

// ImportMeta 类型扩展 - 用于 import.meta.env 的类型检查
/* eslint-disable @typescript-eslint/no-unused-vars */
interface ImportMetaEnv {
  readonly DEV: boolean
  readonly VITE_ENABLE_CONSOLE_BRIDGE?: string
}

interface ImportMeta {
  readonly env: ImportMetaEnv
}
/* eslint-enable @typescript-eslint/no-unused-vars */

// Initialize Console Bridge in development mode
// This forwards frontend console logs to the Rust backend
// Enabled by VITE_ENABLE_CONSOLE_BRIDGE env var or DEV mode
const shouldEnable =
  import.meta.env.VITE_ENABLE_CONSOLE_BRIDGE === 'true' || import.meta.env.DEV === true
initializeConsoleBridge(shouldEnable)

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <BrowserRouter>
      <App />
    </BrowserRouter>
  </React.StrictMode>
)
