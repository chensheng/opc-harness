## 1. 类型定义更新

- [x] 1.1 在 `src/types/index.ts` 的 UserStory 接口中添加 'failed' 到 status 联合类型
- [x] 1.2 在 `src/stores/userStoryStore.ts` 的 BackendUserStory 接口中添加 'failed' 状态

## 2. UI 组件 - 状态视觉样式

- [x] 2.1 在 `src/components/vibe-coding/UserStoryTable.tsx` 的 statusColors 对象中添加 failed 的红色样式
- [x] 2.2 在 `src/components/vibe-coding/UserStoryTable.tsx` 的 statusLabels 对象中添加 failed 的标签"失败"
- [x] 2.3 验证表格中 failed 状态的徽章正确显示为红色背景

## 3. UI 组件 - 编辑对话框

- [x] 3.1 在 `src/components/vibe-coding/UserStoryEditDialog.tsx` 的状态选择器中添加"失败"选项
- [x] 3.2 在编辑对话框中添加条件渲染的失败信息展示区域(仅在 status === 'failed' 时显示)
- [x] 3.3 实现失败信息区域的只读展示: error_message、retry_count、failed_at
- [x] 3.4 测试打开 failed 状态的 Story 时正确显示失败详情

## 4. UI 组件 - 筛选功能

- [x] 4.1 在 `src/components/vibe-coding/UserStoryManager.tsx` 的状态筛选下拉框中添加"失败"选项
- [x] 4.2 测试按"失败"状态筛选能正确过滤出所有 failed 的 Story
- [x] 4.3 测试多条件筛选(状态 + 优先级/Sprint)包含 failed 时的正确性

## 5. 重试功能实现

- [x] 5.1 在 `src/components/vibe-coding/UserStoryManager.tsx` 中添加 handleRetryStory 函数
- [x] 5.2 在用户故事表格的操作列中为 failed 状态的 Story 添加"重试"按钮
- [x] 5.3 实现重试逻辑:调用 updateStory 将状态从 failed 更新为 draft
- [x] 5.4 添加重试确认对话框,提示用户该操作会将 Story 重置为草稿状态
- [x] 5.5 测试重试后 Story 状态正确变更为 draft 并可在待处理列表中看到

## 6. 防御性编程增强

- [x] 6.1 在 UserStoryTable.tsx 中为 statusColors 访问添加默认值回退逻辑
- [x] 6.2 在 UserStoryTable.tsx 中为 statusLabels 访问添加默认值回退逻辑
- [x] 6.3 审查其他使用 story.status 的地方,确保都有适当的默认值处理

## 7. 后端数据模型补充(可选)

- [x] 7.1 检查 `src-tauri/src/models/mod.rs` 中的 UserStory 结构体是否包含 error_message、retry_count、failed_at 字段的前端映射
- [x] 7.2 如有缺失,在前端类型定义中添加这些可选字段

## 8. 测试与验证

- [x] 8.1 手动测试:创建一个测试项目,模拟 Agent Worker 执行失败的场景
- [x] 8.2 验证失败的 Story 在列表中正确显示为红色"失败"徽章
- [x] 8.3 验证点击编辑按钮能看到完整的失败信息
- [x] 8.4 验证筛选功能能正确过滤出失败的 Story
- [x] 8.5 验证重试功能能将 failed 状态的 Story 重置为 draft
- [x] 8.6 运行 TypeScript 编译检查,确保没有类型错误
- [x] 8.7 运行 ESLint 检查,确保代码质量符合规范

## 9. 文档更新

- [x] 9.1 在相关文档中更新用户故事状态机的说明,包含 failed 状态
- [x] 9.2 更新用户故事管理的用户指南,添加重试功能的说明
