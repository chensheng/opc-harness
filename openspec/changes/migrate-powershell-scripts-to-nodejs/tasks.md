## 1. 准备工作和依赖安装

- [x] 1.1 安装 chalk 包: `npm install chalk`
- [x] 1.2 安装 execa 包: `npm install execa`
- [x] 1.3 验证 package.json 中 `"type": "module"` 已设置
- [x] 1.4 创建简单的测试脚本验证 chalk 和 execa 工作正常

## 2. 迁移简单脚本(验证基础框架)

- [x] 2.1 迁移 fast-check.ps1 → fast-check.js
- [x] 2.2 测试 fast-check.js 输出与原版一致
- [x] 2.3 迁移 test-rust-simple.ps1 → test-rust-simple.js
- [x] 2.4 测试 test-rust-simple.js 输出与原版一致

## 3. 迁移测试相关脚本

- [x] 3.1 迁移 harness-rust-tests.ps1 → harness-rust-tests.js
- [x] 3.2 测试 harness-rust-tests.js (运行 Rust 测试)
- [x] 3.3 迁移 harness-ts-tests.ps1 → harness-ts-tests.js
- [x] 3.4 测试 harness-ts-tests.js (运行 TS 测试)
- [x] 3.5 迁移 test-decentralized.ps1 → test-decentralized.js
- [x] 3.6 测试 test-decentralized.js

## 4. 迁移 E2E 和质量修复脚本

- [x] 4.1 迁移 harness-e2e.ps1 → harness-e2e.js
- [ ] 4.2 测试 harness-e2e.js
- [x] 4.3 迁移 fix-code-quality.ps1 → fix-code-quality.js
- [x] 4.4 测试 fix-code-quality.js (自动修复功能)

## 5. 迁移复杂脚本

- [ ] 5.1 迁移 harness-gc.ps1 → harness-gc.js
- [ ] 5.2 测试 harness-gc.js (垃圾清理功能)
- [ ] 5.3 迁移 harness-check.ps1 → harness-check.js (最复杂,最后迁移)
- [ ] 5.4 测试 harness-check.js (完整健康检查)

## 6. 更新配置和清理

- [ ] 6.1 更新 package.json 中的 npm scripts 配置
- [ ] 6.2 删除所有 .ps1 文件
- [ ] 6.3 运行 `npm run harness:check` 验证完整流程
- [ ] 6.4 运行 `npm run harness:fix` 验证修复功能
- [ ] 6.5 运行 `npm run harness:gc` 验证清理功能
- [ ] 6.6 在 Windows 上测试所有脚本
- [ ] 6.7 验证 Health Score 计算正确(应为 100/100)

## 7. 文档和最终验证

- [ ] 7.1 更新 README 或相关文档(如提及 PowerShell)
- [ ] 7.2 验证所有 npm scripts 正常工作
- [ ] 7.3 确认无 PowerShell 依赖残留
- [ ] 7.4 提交变更到 Git
