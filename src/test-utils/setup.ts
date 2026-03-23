import '@testing-library/jest-dom'
import { beforeEach, afterAll, vi } from 'vitest'

// Mock Tauri APIs
declare global {
  interface Window {
    __TAURI__?: {
      invoke: (cmd: string, args?: unknown) => Promise<unknown>
      convertFileSrc: (path: string) => string
    }
  }
}

window.__TAURI__ = {
  invoke: async () => ({}),
  convertFileSrc: (path: string) => `file://${path}`,
} as any

// Reset mocks before each test
beforeEach(() => {
  vi.clearAllMocks()
})

// Cleanup after all tests
afterAll(() => {
  vi.resetAllMocks()
})
