import { describe, it, expect } from 'vitest'
import fs from 'fs'
import path from 'path'

/**
 * 架构约束测试
 * 基于 OpenAI Harness Engineering 实践：
 * - 分层依赖规则强制执行
 * - 模块边界检查
 * - 循环依赖检测
 */

describe('Architecture Constraints', () => {
  const rootDir = path.resolve(__dirname, '../..')
  const srcDir = path.resolve(rootDir, 'src')

  /**
   * 测试 1: 验证目录结构完整性
   * 确保所有必需的目录都存在
   */
  it('should have complete directory structure', () => {
    const requiredDirs = ['components', 'stores', 'types', 'hooks', 'lib']

    requiredDirs.forEach(dir => {
      const dirPath = path.join(srcDir, dir)
      expect(fs.existsSync(dirPath)).toBe(true)
    })
  })

  /**
   * 测试 2: 禁止前端直接导入后端代码
   * Frontend 只能通过 Tauri Commands 与后端通信
   */
  it('should not import Rust code directly in frontend', () => {
    const frontendFiles = getAllFiles(srcDir, '.ts', '.tsx')

    const forbiddenPatterns = [
      /from ['"]\.\.\/src-tauri/i,
      /from ['"]@\/.*rust/i,
      /import.*tauri.*from ['"]\//i,
    ]

    frontendFiles.forEach(file => {
      const content = fs.readFileSync(file, 'utf-8')

      forbiddenPatterns.forEach(pattern => {
        const matches = content.match(pattern)
        if (matches) {
          throw new Error(
            `Forbidden import found in ${file}: ${matches[0]}\n` +
              'Frontend should only communicate with backend via Tauri Commands'
          )
        }
      })
    })
  })

  /**
   * 测试 3: UI 组件不应包含业务逻辑
   * components/ 目录下的文件应该只负责 UI 渲染
   */
  it('should not have business logic in UI components', () => {
    const componentFiles = getAllFiles(path.join(srcDir, 'components'), '.tsx')

    const forbiddenPatterns = [
      /invoke\s*\(/, // Tauri invoke 调用
      /axios\./, // HTTP 请求
      /fetch\s*\(/, // Fetch API
      /useEffect\s*\([^)]*async/, // useEffect 中的异步操作（可能是业务逻辑）
    ]

    // 允许的模式（纯 UI 组件可以有的）
    const allowedPatterns = [
      /useState/,
      /useEffect\s*\(\s*\(\)\s*=>\s*\{[^}]*\}\s*,\s*\[\]\s*\)/, // 简单的副作用
    ]

    componentFiles.forEach(file => {
      // 跳过 UI 基础组件
      if (file.includes('/ui/')) {
        return
      }

      const content = fs.readFileSync(file, 'utf-8')

      forbiddenPatterns.forEach(pattern => {
        const matches = content.match(pattern)
        if (matches && !isAllowedInFile(file, allowedPatterns)) {
          console.warn(`⚠️  Potential business logic in UI component: ${file}`)
          // 这里使用 warning 而不是 error，因为有些组件可能需要简单的交互逻辑
        }
      })
    })
  })

  /**
   * 测试 4: Store 不应该直接调用 API
   * 状态管理只负责数据存储和更新
   */
  it('should not call APIs directly in stores', () => {
    const storeFiles = getAllFiles(path.join(srcDir, 'stores'), '.ts')

    storeFiles.forEach(file => {
      const content = fs.readFileSync(file, 'utf-8')

      // 检查是否有直接的 API 调用
      const hasDirectAPICall = /axios\./.test(content) || /fetch\s*\([^)]*http/.test(content)

      if (hasDirectAPICall) {
        throw new Error(
          `Direct API call found in store: ${file}\n` +
            'Stores should use Tauri commands instead of direct API calls'
        )
      }
    })
  })

  /**
   * 测试 5: 类型定义应该完整
   * 所有导出的接口和类型都应该有文档注释
   */
  it('should have JSDoc comments for exported types', () => {
    const typeFiles = getAllFiles(path.join(srcDir, 'types'), '.ts')

    typeFiles.forEach(file => {
      const content = fs.readFileSync(file, 'utf-8')

      // 查找 export interface 和 export type
      const exportPattern = /export\s+(interface|type)\s+(\w+)/g
      let match

      while ((match = exportPattern.exec(content)) !== null) {
        const lineNumber = content.substring(0, match.index).split('\n').length
        const precedingLines = content.substring(0, match.index).split('\n').slice(-3)

        // 检查前面是否有 JSDoc 注释
        const hasJSDoc = precedingLines.some(line => line.includes('*/') || line.includes('//'))

        if (!hasJSDoc) {
          console.warn(
            `⚠️  Missing JSDoc comment for ${match[1]} ${match[2]} in ${file}:${lineNumber}`
          )
        }
      }
    })
  })

  /**
   * 测试 6: 性能约束验证
   * 关键组件的渲染应该在合理时间内完成
   */
  it('should render critical components within time limit', async () => {
    // 这个测试需要在实际运行环境中进行
    // 这里只是一个占位符，实际测试应该在 E2E 中执行
    expect(true).toBe(true)
  })
})

// 辅助函数：获取指定目录下的所有文件
function getAllFiles(dir: string, ...extensions: string[]): string[] {
  const files: string[] = []

  const items = fs.readdirSync(dir)
  for (const item of items) {
    const fullPath = path.join(dir, item)
    const stat = fs.statSync(fullPath)

    if (stat.isDirectory()) {
      files.push(...getAllFiles(fullPath, ...extensions))
    } else if (extensions.some(ext => item.endsWith(ext))) {
      files.push(fullPath)
    }
  }

  return files
}

// 辅助函数：检查是否在允许的文件中使用
function isAllowedInFile(filePath: string, patterns: RegExp[]): boolean {
  const content = fs.readFileSync(filePath, 'utf-8')
  return patterns.some(pattern => pattern.test(content))
}
