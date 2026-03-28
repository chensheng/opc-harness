import js from '@eslint/js'
import globals from 'globals'
import reactHooks from 'eslint-plugin-react-hooks'
import reactRefresh from 'eslint-plugin-react-refresh'
import tseslint from 'typescript-eslint'
import { createRequire } from 'module'

const require = createRequire(import.meta.url)
const architecturePlugin = require('./eslint-rules/index.cjs')

export default tseslint.config(
  { ignores: ['dist', 'coverage', 'src-tauri/target', 'eslint-rules'] },
  {
    extends: [js.configs.recommended, ...tseslint.configs.recommended],
    files: ['**/*.{ts,tsx}'],
    languageOptions: {
      ecmaVersion: 2020,
      globals: globals.browser,
    },
    plugins: {
      'react-hooks': reactHooks,
      'react-refresh': reactRefresh,
      'architecture': architecturePlugin,
    },
    rules: {
      ...reactHooks.configs.recommended.rules,
      'react-refresh/only-export-components': [
        'warn',
        { allowConstantExport: true },
      ],
      '@typescript-eslint/no-explicit-any': 'warn',
      '@typescript-eslint/no-unused-vars': ['warn', { argsIgnorePattern: '^_', varsIgnorePattern: '^_' }],
      // 允许工具函数和组件一起导出
      'react-refresh/only-export-components': 'off',
      
      // 架构约束规则 🔥
      'architecture/architecture-constraint': 'error',
      'architecture/ui-component-purity': 'error',
      'architecture/store-api-check': 'error',
    },
  }
)
