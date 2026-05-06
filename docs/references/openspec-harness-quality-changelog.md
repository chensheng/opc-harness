# Harness Quality Schema v2 - 更新说明

## 📅 更新日期
2026-05-06

## 🎯 更新内容

在 `harness-quality` schema 中新增了 **runtime-check** artifact，实现双重质量保障：

1. ✅ **静态质量检查**（quality-check）：代码质量、测试覆盖率、格式规范
2. ✅ **运行时验证**（runtime-check）：应用启动、功能正常、无错误日志

## 🔄 工作流程变化

### Before (v1)
```
proposal → specs → design → tasks → quality-check → apply → archive
```

### After (v2)
```
proposal → specs → design → tasks → quality-check → runtime-check → apply → archive
```

## 📋 新增 Artifact: runtime-check

### 依赖关系
- **前置依赖**: quality-check
- **生成文件**: `runtime-check.md`

### 检查内容

#### 1. Tauri 应用启动验证
- Vite 开发服务器启动
- Rust 后端编译
- 应用窗口打开
- 启动时间记录

#### 2. 前端检查
- 浏览器控制台无 JavaScript 错误
- 无 TypeScript 运行时错误
- UI 正确渲染
- HMR（热模块替换）正常工作

#### 3. 后端检查
- Rust 日志无 panic
- 无 critical errors
- Tauri 插件加载正常
- 命令执行正常

#### 4. 功能测试
- 核心功能正常工作
- 变更相关功能验证
- 导航和交互正常

#### 5. 性能观察
- 初始加载时间
- UI 响应性
- 内存使用
- CPU 使用

### 模板结构

```markdown
# Runtime Check Report

## Tauri Application Validation
- Status: PASS/FAIL
- Startup Time: XX seconds

## Environment
- Command, OS, Node version, Rust version

## Startup Process
- Frontend (Vite + React) status
- Backend (Tauri + Rust) status

## Runtime Checks
- Frontend Console Errors
- Backend Errors (panics, error logs)

## Feature Testing
- Core functionality tests
- Change-specific tests

## Performance Observations
- Load time, responsiveness, memory, CPU

## Issues Found
- Critical issues (blocking)
- Non-critical issues

## Shutdown
- Clean shutdown verification

## Final Assessment
- Overall status and confidence level
```

## 🔧 Apply 前置条件更新

### Before
```yaml
apply:
  requires: [tasks, quality-check]
```

### After
```yaml
apply:
  requires: [tasks, quality-check, runtime-check]
```

## 💡 使用示例

### 完整流程

```bash
# 1. 创建变更
openspec new change my-feature

# 2. 生成 artifacts
/opsx:propose my-feature

# 3. 实施任务
/opsx:apply my-feature

# 4. 静态质量检查
npm run harness:check
# 记录到 quality-check.md

# 5. 运行时验证 ⭐ 新增
npm run tauri:dev
# 等待应用启动（30-60秒）
# 检查前端控制台
# 检查后端日志
# 测试功能
# Ctrl+C 停止

# 记录到 runtime-check.md
openspec instructions runtime-check --change my-feature

# 6. 归档
/opsx:archive my-feature
```

## ✅ 优势

### 全面的质量保障

| 检查类型 | 检测内容 | 工具 |
|---------|---------|------|
| 静态分析 | 类型错误、代码风格、测试覆盖率 | harness:check |
| 运行时验证 | 集成错误、运行时异常、功能问题 | tauri:dev |

### 捕获的问题类型

**Static Check 能发现**:
- ❌ TypeScript 类型错误
- ❌ ESLint 违规
- ❌ 格式化问题
- ❌ 测试失败
- ❌ Rust 编译错误

**Runtime Check 能发现**:
- ❌ JavaScript 运行时错误
- ❌ Rust panics
- ❌ Tauri 命令失败
- ❌ UI 渲染问题
- ❌ 集成功能故障
- ❌ 性能退化

## 📊 效果对比

### 只有 Static Check
```
✅ 代码质量高
❌ 可能有运行时错误
❌ 可能有集成问题
❌ 用户可能遇到崩溃
```

### Static + Runtime Check
```
✅ 代码质量高
✅ 应用运行正常
✅ 集成功能完好
✅ 用户体验流畅
```

## 🎓 最佳实践

### 1. 检查时机
- **开发中**: 定期运行 `harness:check`
- **任务完成后**: 完整的 static check
- **归档前**: 必须完成 runtime check

### 2. 常见问题处理

**应用启动慢**:
- 记录实际启动时间
- 分析是否是本次变更导致
- 考虑优化建议

**控制台警告**:
- 区分新引入的 vs 预存在的
- 评估影响程度
- 决定是否阻塞

**Rust Warnings**:
- 记录所有 warnings
- 识别是否与变更相关
- 决定是否需要修复

### 3. 文档化
- 在 runtime-check.md 中详细说明测试过程
- 记录任何发现的问题
- 提供重现步骤
- 给出修复建议

## 🔍 验证

```bash
# 验证 schema
openspec schema validate harness-quality
# ✓ Schema 'harness-quality' is valid

# 查看 schemas
openspec schemas
# harness-quality (project)
#   Artifacts: proposal → specs → design → tasks → quality-check → runtime-check
```

## 📁 相关文件

- **Schema 定义**: `openspec/schemas/harness-quality/schema.yaml`
- **Runtime 模板**: `openspec/schemas/harness-quality/templates/runtime-check.md`
- **Quality 模板**: `openspec/schemas/harness-quality/templates/quality-check.md`
- **README**: `openspec/schemas/harness-quality/README.md`
- **快速参考**: `openspec/schemas/harness-quality/QUICKSTART.md`
- **集成文档**: `docs/references/openspec-harness-integration.md`

## 🚀 下一步

1. **试点使用**: 在实际变更中测试新流程
2. **收集反馈**: 了解 runtime-check 的实用性
3. **优化模板**: 根据实际使用调整检查项
4. **自动化**: 考虑让 AI Agent 自动执行检查和填充报告
5. **CI/CD 集成**: 在持续集成中自动运行这些检查

---

**版本**: v2.0  
**更新日期**: 2026-05-06  
**主要变更**: 新增 runtime-check artifact  
**向后兼容**: 是（基于 spec-driven fork，可随时切换）
