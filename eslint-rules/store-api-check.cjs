/**
 * ESLint 规则：Store 层 API 调用检查
 * 
 * 确保 Store 层不直接调用外部 API：
 * - 不允许使用 axios
 * - 不允许使用 fetch 访问 HTTP 端点
 * - 应该通过 Tauri Commands 与后端通信
 */

module.exports = {
  meta: {
    type: 'problem',
    docs: {
      description: 'Ensure stores do not call external APIs directly, use Tauri commands instead',
      category: 'Architecture',
      recommended: true,
    },
    fixable: null,
    schema: [],
    messages: {
      axiosCall:
        "Store should not use axios for API calls. Use Tauri commands to communicate with backend. Found in: '{{storeFile}}'",
      fetchCall:
        "Store should not use fetch for API calls. Use Tauri commands to communicate with backend. Found in: '{{storeFile}}'",
      httpImport:
        "Store should not import HTTP client libraries directly. Found in: '{{storeFile}}'",
    },
  },
  create(context) {
    const path = require('path');

    /**
     * 检查文件是否在 Store 目录中
     */
    function isStoreFile(filePath) {
      return filePath.includes(path.join('src', 'stores')) && 
             (filePath.endsWith('.ts') || filePath.endsWith('.tsx'));
    }

    /**
     * 获取 Store 名称
     */
    function getStoreName(filePath) {
      return path.basename(filePath);
    }

    return {
      Program(node) {
        const filePath = context.getFilename();
        
        // 只检查 Store 文件
        if (!isStoreFile(filePath)) {
          return;
        }

        const storeName = getStoreName(filePath);
        const sourceCode = context.getSourceCode();
        const text = sourceCode.getText();

        // 检查是否包含 axios 调用
        if (/axios\./.test(text)) {
          context.report({
            node,
            messageId: 'axiosCall',
            data: { storeFile: storeName },
          });
        }

        // 检查是否包含 fetch 调用（特别是 HTTP 请求）
        if (/fetch\s*\([^)]*['"`]http/.test(text)) {
          context.report({
            node,
            messageId: 'fetchCall',
            data: { storeFile: storeName },
          });
        }
      },

      ImportDeclaration(node) {
        const filePath = context.getFilename();
        
        if (!isStoreFile(filePath)) {
          return;
        }

        const importSource = node.source.value;
        const storeName = getStoreName(filePath);

        // 检查是否导入了 axios 或其他 HTTP 客户端库
        if (importSource === 'axios' || 
            importSource.includes('axios/') ||
            /http-client|superagent|got|node-fetch/.test(importSource)) {
          context.report({
            node,
            messageId: 'httpImport',
            data: { storeFile: storeName },
          });
        }
      },
    };
  },
};
