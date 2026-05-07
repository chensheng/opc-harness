## Why

Rust 编译器当前产生 93 个警告,主要包括未使用的导入、未使用的变量和 unreachable patterns。这些警告降低了代码质量评分,影响开发体验,并可能隐藏潜在的逻辑问题。清理这些警告可以提升代码可维护性,符合 Rust 最佳实践。

## What Changes

- 移除所有未使用的 imports (约 15+ 处)
- 移除或正确使用未使用的变量 (约 10+ 处)
- 修复 unreachable pattern 警告
- 清理未使用的 manifest key
- 保持所有功能不变,仅清理警告

## Capabilities

### New Capabilities
<!-- No new capabilities - this is a code quality improvement -->

### Modified Capabilities
<!-- No requirement changes - implementation cleanup only -->

## Impact

**Affected Files**:
- `src-tauri/src/commands/observability.rs` - 移除未使用的 repository imports
- `src-tauri/src/commands/quality/mod.rs` - 移除未使用的 quality_check re-export
- `src-tauri/src/commands/retry.rs` - 移除未使用的 tauri::State import
- `src-tauri/src/db/mod.rs` - 移除未使用的 retry 函数 exports
- `src-tauri/src/services/observability_service.rs` - 移除未使用的 repository imports
- `src-tauri/src/agent/agent_worker.rs` - 修复未使用变量和 unreachable pattern
- `src-tauri/Cargo.toml` - 清理未使用的 manifest key

**Impact Level**: Low (仅清理警告,不改变功能)
**Breaking Changes**: None
**Testing Required**: cargo check 通过且无警告
