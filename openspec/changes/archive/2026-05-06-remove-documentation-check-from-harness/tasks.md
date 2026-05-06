## 1. 移除文档检测配置

- [x] 1.1 从 ScoreWeights 中移除 `Documentation = 10`
- [x] 1.2 从 RequiredDirs 中移除 `"docs"`
- [x] 1.3 移除 KeyDocuments 配置块
- [x] 1.4 移除 IndexFiles 配置块

## 2. 重新分配权重分数

- [x] 2.1 更新 TypeScript 权重: 20 → 22
- [x] 2.2 更新 ESLint 权重: 15 → 17
- [x] 2.3 更新 Prettier 权重: 10 → 11
- [x] 2.4 更新 Rust 权重: 25 → 28
- [x] 2.5 更新 RustTests 权重: 20 → 22
- [x] 2.6 更新 TSTests 权重: 20 → 22
- [x] 2.7 更新 Dependencies 权重: 5 → 6
- [x] 2.8 更新 Directory 权重: 5 → 6

## 3. 移除文档检测函数

- [x] 3.1 删除 `Invoke-DocumentationCheck` 函数(第 424-462 行)

## 4. 更新主流程和步骤编号

- [x] 4.1 更新脚本版本号为 2.2
- [x] 4.2 移除主流程中的 `Invoke-DocumentationCheck` 调用
- [x] 4.3 更新 Directory Check 步骤显示: "8/9" → "8/8"
- [x] 4.4 验证所有检查步骤编号正确(共 8 步)

## 5. 测试验证

- [x] 5.1 运行 `npm run harness:check` 验证无语法错误
- [x] 5.2 确认不再出现 docs 目录相关的警告
- [x] 5.3 验证 Health Score 计算正常
- [x] 5.4 确认所有其他检测项正常工作
