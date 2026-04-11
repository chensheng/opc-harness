/**
 * ESLint 规则：UI 组件纯度检查
 *
 * 确保 UI 组件（src/components/ui/）不包含业务逻辑：
 * - 不允许调用 Tauri invoke
 * - 不允许直接 HTTP 请求（axios/fetch）
 * - 不允许复杂的异步操作
 */

module.exports = {
  meta: {
    type: 'problem',
    docs: {
      description: 'Ensure UI components remain pure and do not contain business logic',
      category: 'Architecture',
      recommended: true,
    },
    fixable: null,
    schema: [],
    messages: {
      tauriInvoke:
        "UI component should not call Tauri invoke directly. Move business logic to hooks or stores. Found in: '{{componentName}}'",
      httpCall:
        "UI component should not make HTTP calls directly. Use hooks or stores for API calls. Found in: '{{componentName}}'",
      complexAsync:
        "UI component should not contain complex async operations. Move to hooks. Found in: '{{componentName}}'",
      storeImport:
        "UI component should not import stores directly. Use hooks as abstraction. Found in: '{{componentName}}'",
    },
  },
  create(context) {
    const path = require('path')

    /**
     * 检查文件是否在 UI 组件目录中
     */
    function isUIComponent(filePath) {
      return filePath.includes(path.join('src', 'components', 'ui'))
    }

    /**
     * 获取组件名称
     */
    function getComponentName(filePath) {
      return path.basename(filePath)
    }

    return {
      Program(node) {
        const filePath = context.getFilename()

        // 只检查 UI 组件
        if (!isUIComponent(filePath)) {
          return
        }

        const componentName = getComponentName(filePath)
        const sourceCode = context.getSourceCode()

        // 检查是否包含 Tauri invoke 调用
        const text = sourceCode.getText()

        if (/invoke\s*\(/.test(text)) {
          context.report({
            node,
            messageId: 'tauriInvoke',
            data: { componentName },
          })
        }

        // 检查是否包含直接的 HTTP 调用
        if (/axios\./.test(text) || /fetch\s*\([^)]*http/.test(text)) {
          context.report({
            node,
            messageId: 'httpCall',
            data: { componentName },
          })
        }

        // 检查是否包含复杂的异步操作（useEffect 中的 async）
        const hasComplexAsync = /useEffect\s*\(\s*async\s*\(/.test(text)
        if (hasComplexAsync) {
          context.report({
            node,
            messageId: 'complexAsync',
            data: { componentName },
          })
        }
      },

      ImportDeclaration(node) {
        const filePath = context.getFilename()

        if (!isUIComponent(filePath)) {
          return
        }

        const importSource = node.source.value
        const componentName = getComponentName(filePath)

        // 检查是否直接导入 stores
        if (importSource.includes('/stores/') || importSource.startsWith('@/stores/')) {
          context.report({
            node,
            messageId: 'storeImport',
            data: { componentName },
          })
        }
      },
    }
  },
}
