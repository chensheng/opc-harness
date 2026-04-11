/**
 * 自定义 ESLint 插件：OPC-HARNESS 架构规则
 *
 * 提供以下规则：
 * - architecture-constraint: 架构分层约束
 * - ui-component-purity: UI 组件纯度检查
 * - store-api-check: Store 层 API 调用检查
 */

module.exports = {
  rules: {
    'architecture-constraint': require('./architecture-constraint'),
    'ui-component-purity': require('./ui-component-purity'),
    'store-api-check': require('./store-api-check'),
  },
}
