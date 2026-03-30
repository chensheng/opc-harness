import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import path from 'path'

export default defineConfig(() => ({
  plugins: [react()],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  },
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      ignored: ['**/src-tauri/**'],
    },
  },
  envPrefix: ['VITE_', 'TAURI_'],
  build: {
    target: process.env.TAURI_PLATFORM === 'windows' ? 'chrome105' : 'safari13',
    minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
    sourcemap: !!process.env.TAURI_DEBUG,
  },
  test: {
    globals: true,
    environment: 'jsdom',
    setupFiles: './src/test-utils/setup.ts',
    include: [
      'src/**/*.{test,spec}.{ts,tsx}',
      'tests/**/*.test.ts',
      'e2e/**/*.spec.ts', // VC-036: 添加 E2E 测试支持
      'e2e/browser/**/*.test.{ts,tsx}', // 添加浏览器 E2E 测试支持
    ],
    exclude: [
      'node_modules',
      'dist',
      '.git',
      '.husky',
      'scripts',
      'src-tauri',
      '**/*.d.ts',
      '**/*.config.*',
    ],
    coverage: {
      provider: 'v8',
      reporter: ['text', 'json', 'html'],
      exclude: [
        'node_modules',
        'src/test-utils/**',
        '**/*.d.ts',
        '**/*.config.*',
        '**/mocks/**',
        'dist',
        'coverage',
      ],
      thresholds: {
        global: {
          branches: 70,
          functions: 70,
          lines: 70,
          statements: 70,
        },
      },
    },
    server: {
      deps: {
        inline: [
          // Tauri packages need to be inlined for tests
          '@tauri-apps/api',
        ],
      },
    },
  },
}))
