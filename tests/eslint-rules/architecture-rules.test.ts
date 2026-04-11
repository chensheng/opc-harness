/**
 * 自定义 ESLint 规则单元测试
 *
 * 测试架构约束规则是否正确工作
 */

import { describe, it } from 'vitest'
import { RuleTester } from 'eslint'
import path from 'path'
import { createRequire } from 'module'

const require = createRequire(import.meta.url)
const architectureConstraintRule = require(
  path.resolve(__dirname, '../../eslint-rules/architecture-constraint.cjs')
)
const uiComponentPurityRule = require(
  path.resolve(__dirname, '../../eslint-rules/ui-component-purity.cjs')
)
const storeApiCheckRule = require(path.resolve(__dirname, '../../eslint-rules/store-api-check.cjs'))

describe('Custom ESLint Architecture Rules', () => {
  const ruleTester = new RuleTester({
    parserOptions: {
      ecmaVersion: 2020,
      sourceType: 'module',
    },
  })

  describe('architecture-constraint rule', () => {
    it('should allow stores to import lib and types', () => {
      ruleTester.run('architecture-constraint', architectureConstraintRule, {
        valid: [
          {
            code: "import { helper } from '@/lib/utils';",
            filename: path.resolve(__dirname, '../../src/stores/appStore.ts'),
          },
          {
            code: "import type { AppState } from '@/types';",
            filename: path.resolve(__dirname, '../../src/stores/appStore.ts'),
          },
        ],
        invalid: [],
      })
    })

    it('should prevent stores from importing components', () => {
      ruleTester.run('architecture-constraint', architectureConstraintRule, {
        valid: [],
        invalid: [
          {
            code: "import { AppLayout } from '@/components/common/AppLayout';",
            filename: path.resolve(__dirname, '../../src/stores/appStore.ts'),
            errors: [{ messageId: 'layerViolation' }],
          },
        ],
      })
    })

    it('should allow hooks to import stores and lib', () => {
      ruleTester.run('architecture-constraint', architectureConstraintRule, {
        valid: [
          {
            code: "import { useAppStore } from '@/stores/appStore';",
            filename: path.resolve(__dirname, '../../src/hooks/useAgent.ts'),
          },
          {
            code: "import { cn } from '@/lib/utils';",
            filename: path.resolve(__dirname, '../../src/hooks/useAgent.ts'),
          },
        ],
        invalid: [],
      })
    })

    it('should prevent hooks from importing business components', () => {
      ruleTester.run('architecture-constraint', architectureConstraintRule, {
        valid: [],
        invalid: [
          {
            code: "import { CodeEditor } from '@/components/vibe-coding/CodeEditor';",
            filename: path.resolve(__dirname, '../../src/hooks/useAgent.ts'),
            errors: [{ messageId: 'layerViolation' }],
          },
        ],
      })
    })

    it('should allow UI components to import only lib and types', () => {
      ruleTester.run('architecture-constraint', architectureConstraintRule, {
        valid: [
          {
            code: "import { cn } from '@/lib/utils';",
            filename: path.resolve(__dirname, '../../src/components/ui/button.tsx'),
          },
          {
            code: "import type { ButtonProps } from '@/types';",
            filename: path.resolve(__dirname, '../../src/components/ui/button.tsx'),
          },
        ],
        invalid: [],
      })
    })

    it('should prevent UI components from importing stores', () => {
      ruleTester.run('architecture-constraint', architectureConstraintRule, {
        valid: [],
        invalid: [
          {
            code: "import { useAppStore } from '@/stores/appStore';",
            filename: path.resolve(__dirname, '../../src/components/ui/button.tsx'),
            errors: [{ messageId: 'uiComponentImportStore' }],
          },
        ],
      })
    })
  })

  describe('ui-component-purity rule', () => {
    it('should allow pure UI components', () => {
      ruleTester.run('ui-component-purity', uiComponentPurityRule, {
        valid: [
          {
            code: `
              export function Button({ children, onClick }) {
                return <button onClick={onClick}>{children}</button>;
              }
            `,
            filename: path.resolve(__dirname, '../../src/components/ui/button.tsx'),
          },
        ],
        invalid: [],
      })
    })

    it('should prevent Tauri invoke in UI components', () => {
      ruleTester.run('ui-component-purity', uiComponentPurityRule, {
        valid: [],
        invalid: [
          {
            code: `
              export function Button() {
                const handleClick = async () => {
                  await invoke('some_command');
                };
                return <button onClick={handleClick}>Click</button>;
              }
            `,
            filename: path.resolve(__dirname, '../../src/components/ui/button.tsx'),
            errors: [{ messageId: 'tauriInvoke' }],
          },
        ],
      })
    })

    it('should prevent HTTP calls in UI components', () => {
      ruleTester.run('ui-component-purity', uiComponentPurityRule, {
        valid: [],
        invalid: [
          {
            code: `
              export function Button() {
                useEffect(() => {
                  fetch('/api/data').then(r => r.json());
                }, []);
                return <button>Click</button>;
              }
            `,
            filename: path.resolve(__dirname, '../../src/components/ui/button.tsx'),
            errors: [{ messageId: 'httpCall' }],
          },
        ],
      })
    })
  })

  describe('store-api-check rule', () => {
    it('should allow stores without API calls', () => {
      ruleTester.run('store-api-check', storeApiCheckRule, {
        valid: [
          {
            code: `
              export const useAppStore = create((set) => ({
                count: 0,
                increment: () => set((state) => ({ count: state.count + 1 }))
              }));
            `,
            filename: path.resolve(__dirname, '../../src/stores/appStore.ts'),
          },
        ],
        invalid: [],
      })
    })

    it('should prevent axios calls in stores', () => {
      ruleTester.run('store-api-check', storeApiCheckRule, {
        valid: [],
        invalid: [
          {
            code: `
              import axios from 'axios';
              export const useAppStore = create((set) => ({
                loadData: async () => {
                  const res = await axios.get('/api/data');
                  set({ data: res.data });
                }
              }));
            `,
            filename: path.resolve(__dirname, '../../src/stores/appStore.ts'),
            errors: [{ messageId: 'axiosCall' }],
          },
        ],
      })
    })

    it('should prevent fetch calls in stores', () => {
      ruleTester.run('store-api-check', storeApiCheckRule, {
        valid: [],
        invalid: [
          {
            code: `
              export const useAppStore = create((set) => ({
                loadData: async () => {
                  const res = await fetch('http://api.example.com/data');
                  const data = await res.json();
                  set({ data });
                }
              }));
            `,
            filename: path.resolve(__dirname, '../../src/stores/appStore.ts'),
            errors: [{ messageId: 'fetchCall' }],
          },
        ],
      })
    })
  })
})
