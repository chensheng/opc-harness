## Context

当前 Rust 后端代码在 `cargo check` 时产生 93 个警告,主要包括:
- 未使用的 imports (unused_imports)
- 未使用的变量 (unused_variables)  
- Unreachable patterns
- 未使用的 manifest key

这些警告虽然不影响编译,但:
1. 降低代码质量和可维护性
2. 可能隐藏真正的潜在问题
3. 影响 harness:check 健康评分
4. 不符合 Rust 最佳实践 (warnings as errors 理念)

## Goals / Non-Goals

**Goals:**
- 消除所有 Rust 编译警告
- 保持功能完全不变
- 提升代码可读性和可维护性
- 通过 cargo check 无警告验证

**Non-Goals:**
- 不重构代码逻辑
- 不添加新功能
- 不改变 API 接口
- 不优化性能

## Decisions

### Decision 1: 移除 vs 注释未使用的 Imports

**Choice**: 直接移除未使用的 imports

**Rationale**:
- 未使用的 import 通常是遗留代码或复制粘贴的产物
- 保留它们会增加维护负担
- Git 历史可以追溯之前的使用情况
- 如果将来需要,可以重新添加

**Alternatives Considered**:
- 使用 `#[allow(unused_imports)]` - 拒绝,这会掩盖问题而非解决
- 注释掉而非删除 - 拒绝,注释会随时间过时

### Decision 2: 处理未使用的变量

**Choice**: 根据情况采用不同策略

**Rationale**:
- **预留参数** (如 `app_handle`): 使用前缀 `_` 命名 (如 `_app_handle`)
- **调试遗留**: 如果确实不需要,直接移除
- **未来扩展**: 如果计划使用,添加 TODO 注释说明用途

**Pattern**:
```rust
// 预留但未使用的参数
fn example(_app_handle: &AppHandle) { ... }

// 暂时未使用但计划使用
// TODO: Use this for WebSocket notifications
let unused_var = some_value;
```

### Decision 3: 修复 Unreachable Pattern

**Choice**: 分析逻辑,移除或调整 match arms

**Rationale**:
- Unreachable pattern 通常表示逻辑错误或冗余分支
- 需要仔细检查是否应该合并 cases 或移除 dead code
- 确保不改变业务逻辑

### Decision 4: Cargo.toml Manifest Key

**Choice**: 移除或修正无效的 manifest key

**Rationale**:
- `build` key 在错误位置会导致警告
- 应该放在 `[package]` 或其他正确 section 下
- 或者如果不需要,直接删除

## Risks / Trade-offs

### Risk 1: 误删需要的 Import
**Probability**: Low  
**Impact**: Medium (编译错误)  
**Mitigation**: 
- 使用 `cargo check` 频繁验证
- 每次修改后立即测试编译
- Git 提交小批量变更便于回滚

### Risk 2: 破坏隐性依赖
**Probability**: Very Low  
**Impact**: High  
**Mitigation**:
- 仅移除编译器明确标记为未使用的 items
- 不修改任何实际使用的代码
- 运行完整测试套件验证

### Risk 3: 大量文件修改导致 Review 困难
**Probability**: Medium  
**Impact**: Low  
**Mitigation**:
- 按模块分组提交 (commands/, db/, agent/ 等)
- 每个 commit 专注于一个目录
- 清晰的 commit message 说明变更范围

## Migration Plan

1. **Phase 1**: 清理 commands/ 目录的 warnings
2. **Phase 2**: 清理 db/ 和 services/ 目录
3. **Phase 3**: 清理 agent/ 目录 (最复杂,包含未使用变量)
4. **Phase 4**: 修复 Cargo.toml manifest
5. **Validation**: 运行 `cargo check` 确认 0 warnings

**Rollback Strategy**: 
- 每个 phase 独立 commit
- 如有问题,`git revert` 单个 commit
- 不影响运行时行为,风险极低

## Open Questions

None - 这是纯粹的代码清理工作,技术方案明确。
