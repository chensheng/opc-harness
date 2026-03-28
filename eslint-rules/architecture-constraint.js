/**
 * ESLint 规则：架构约束检查
 * 
 * 强制执行以下架构规则：
 * 1. Store 层不得依赖组件层
 * 2. Hook 层不得依赖业务组件层（UI 组件除外）
 * 3. 组件层不得直接导入后端 Rust 代码
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

      if (relativePath.includes(path.join('src', 'stores'))) {
        return 'stores';
      }
      if (relativePath.includes(path.join('src', 'hooks'))) {
        return 'hooks';
      }
      if (relativePath.includes(path.join('src', 'components', 'ui'))) {
        return 'ui-components';
      }
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
     */
    const ALLOWED_DEPENDENCIES = {
      stores: ['lib', 'types', 'external'],
      hooks: ['stores', 'lib', 'types', 'ui-components', 'external'],
      'business-components': ['hooks', 'lib', 'types', 'ui-components', 'external'],
      'ui-components': ['lib', 'types', 'external'],
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
