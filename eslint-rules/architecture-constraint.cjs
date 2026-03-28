/**
 * ESLint 规则：架构约束检查
 * 
 * 强制执行以下架构规则：
 * 1. Store 层不得依赖组件层
 * 2. Hook 层不得依赖业务组件层（UI 组件除外）
 * 3. UI 基础组件不得导入 Stores（保持纯净）
 * 4. 业务组件可以直接导入 Stores（允许，但推荐通过 Hooks）
 */

module.exports = {
  meta: {
    type: 'problem',
    docs: {
      description: 'Enforce architecture layer constraints',
      category: 'Architecture',
      recommended: true,
    },
    fixable: null,
    schema: [],
    messages: {
      storeImportComponent:
        "Store layer cannot import from components directory. Store at '{{storePath}}' tried to import '{{importPath}}'",
      hookImportBusinessComponent:
        "Hooks should not import business components (only UI components allowed). Hook at '{{hookPath}}' tried to import '{{importPath}}'",
      componentImportRust:
        "Frontend components cannot directly import Rust code. Use Tauri commands instead. Component at '{{componentPath}}' tried to import '{{importPath}}'",
      uiComponentImportStore:
        "UI components should not import stores directly. Use hooks as abstraction. UI Component at '{{uiPath}}' tried to import '{{importPath}}'",
      layerViolation:
        "Layer violation: '{{fromLayer}}' cannot import '{{toLayer}}'. File: '{{filePath}}'",
    },
  },
  create(context) {
    const path = require('path');

    /**
     * 获取文件所在的层级
     */
    function getLayer(filePath) {
      const relativePath = path.relative(
        path.resolve(__dirname, '../..'),
        filePath
      );

      // 测试文件不算独立层级，根据其实际位置判断
      const testFilePattern = /\.(test|spec)\.(ts|tsx)$/;
      
      if (relativePath.includes(path.join('src', 'stores'))) {
        return 'stores';
      }
      if (relativePath.includes(path.join('src', 'hooks'))) {
        return 'hooks';
      }
      // UI 基础组件单独分类
      if (relativePath.includes(path.join('src', 'components', 'ui'))) {
        return 'ui-components';
      }
      // 其他业务组件（包括 common, vibe-coding, vibe-design 等）
      if (relativePath.includes(path.join('src', 'components'))) {
        return 'business-components';
      }
      if (relativePath.includes(path.join('src', 'lib'))) {
        return 'lib';
      }
      if (relativePath.includes(path.join('src', 'types'))) {
        return 'types';
      }
      return 'unknown';
    }

    /**
     * 检查导入路径属于哪个层级
     */
    function getImportLayer(importSource, currentFile) {
      // 处理相对路径
      if (importSource.startsWith('.')) {
        const currentDir = path.dirname(currentFile);
        const resolvedPath = path.resolve(currentDir, importSource);
        return getLayer(resolvedPath);
      }

      // 处理 @/ 别名
      if (importSource.startsWith('@/')) {
        const basePath = path.resolve(__dirname, '../..', 'src');
        const resolvedPath = path.resolve(basePath, importSource.slice(2));
        return getLayer(resolvedPath);
      }

      // 第三方库
      return 'external';
    }

    /**
     * 定义允许的依赖关系
     * fromLayer -> [allowed toLayers]
     * 
     * 核心原则：
     * - Stores 只能依赖 lib 和 types（数据层最底层）
     * - Hooks 可以依赖 stores（状态层）
     * - Components 可以依赖 hooks（业务逻辑层）
     * - UI Components 保持纯净，只能依赖 lib/types
     */
    const ALLOWED_DEPENDENCIES = {
      stores: ['lib', 'types', 'external'],
      hooks: ['stores', 'lib', 'types', 'ui-components', 'hooks', 'external'], // hooks 之间可以互导
      'business-components': ['hooks', 'lib', 'types', 'ui-components', 'business-components', 'stores', 'external'], // 业务组件可以导入 stores（允许但不推荐）
      'ui-components': ['lib', 'types', 'ui-components', 'external'], // UI 组件保持纯净，不能导入 stores
      lib: ['lib', 'types', 'external'],
      types: ['types', 'external'],
    };

    return {
      ImportDeclaration(node) {
        const currentFile = context.getFilename();
        const importSource = node.source.value;
        
        const fromLayer = getLayer(currentFile);
        const toLayer = getImportLayer(importSource, currentFile);

        // 跳过未知层级或外部依赖的检查
        if (fromLayer === 'unknown' || toLayer === 'external' || toLayer === 'unknown') {
          return;
        }

        // 跳过测试文件的检查（测试文件需要导入被测试的代码）
        if (currentFile.match(/\.(test|spec)\.(ts|tsx)$/)) {
          return;
        }

        // 特殊检查：UI 组件导入 stores 要报更具体的错误
        if (fromLayer === 'ui-components' && toLayer === 'stores') {
          context.report({
            node,
            messageId: 'uiComponentImportStore',
            data: {
              uiPath: currentFile,
              importPath: importSource,
            },
          });
          return;
        }

        // 检查是否违反依赖规则
        const allowedLayers = ALLOWED_DEPENDENCIES[fromLayer];
        if (allowedLayers && !allowedLayers.includes(toLayer)) {
          context.report({
            node,
            messageId: 'layerViolation',
            data: {
              fromLayer,
              toLayer,
              filePath: currentFile,
            },
          });
        }
      },
    };
  },
};
