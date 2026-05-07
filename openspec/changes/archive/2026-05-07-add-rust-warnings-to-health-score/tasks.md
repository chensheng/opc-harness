## 1. 修改 harness-check.js 脚本

- [x] 1.1 在 Rust Compilation Check 部分添加警告检测逻辑
- [x] 1.2 实现正则表达式解析 cargo check 输出统计警告数量
- [x] 1.3 实现评分算法：每个警告扣 2 分，最低 0 分
- [x] 1.4 更新输出格式，显示警告数量和扣分情况
- [x] 1.5 限制显示前 5 个警告摘要避免输出过长

## 2. 更新开发工作流规范

- [x] 2.1 修改 `openspec/specs/development-workflow/spec.md` 添加 Rust 警告检查要求
- [x] 2.2 添加场景：无警告、少量警告、大量警告的处理方式
- [x] 2.3 明确健康评分计算规则

## 3. 测试验证

- [x] 3.1 故意引入一个 Rust 警告（如未使用的变量）
- [x] 3.2 运行 `npm run harness:check` 验证警告检测和扣分
- [x] 3.3 确认输出格式正确显示警告信息
- [x] 3.4 清理警告后验证评分恢复到 100 分
- [x] 3.5 测试多个警告场景（1个、3个、7个以上）

## 4. 代码质量检查

- [x] 4.1 运行 `npm run harness:check` 确保新代码无问题
- [x] 4.2 确认 Health Score = 100/100
- [x] 4.3 验证所有测试通过
- [x] 4.4 检查 JavaScript 代码符合 ESLint 规范
