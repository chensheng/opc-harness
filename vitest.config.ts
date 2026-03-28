import { defineConfig } from 'vitest/config'
import react from '@vitejs/plugin-react'
import path from 'path'

export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  },
  test: {
    globals: true,
    environment: 'jsdom',
    setupFiles: ['./src/test-utils/setup.ts'],
    include: [
      'src/**/*.{test,spec}.{ts,tsx}',
      'tests/**/*.test.ts',
      'e2e/**/*.{test,spec}.{ts,tsx}',  // E2E 测试
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
        'e2e/**', // E2E 测试文件不计入覆盖率
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
})
