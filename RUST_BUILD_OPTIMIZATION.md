# Rust 编译优化指南

## 🐌 为什么编译慢?

### 主要原因
1. **首次编译依赖**: 新添加的 `dashmap`、`rand` 等库需要下载和编译
2. **Tauri v2 完整编译**: 包含 WebView2、Windows API 等大量系统级依赖
3. **Debug 模式特性**: 包含调试符号,链接时间长
4. **增量编译缓存失效**: 修改核心模块导致大量重编译

---

## 🚀 加速编译的方法

### 方法 1: 使用快速语法检查 (推荐日常开发)

```bash
# 只检查语法,不链接,速度快 5-10 倍
npm run rust:check

# 或直接运行 PowerShell 脚本
.\scripts\fast-check.ps1
```

**适用场景**: 
- ✅ 日常开发时快速验证代码正确性
- ✅ CI/CD 中的语法检查
- ❌ 不能发现链接错误

**速度对比**:
- `cargo check`: ~10-30 秒
- `cargo build`: ~2-5 分钟

---

### 方法 2: 启用增量编译优化

已在 `Cargo.toml` 中配置:

```toml
[profile.dev]
split-debuginfo = "unpacked"  # 分离调试信息,加速链接
incremental = true             # 启用增量编译
```

**效果**: 
- 第二次编译速度提升 50-70%
- 仅重新编译修改的文件及其依赖

---

### 方法 3: 并行编译

已在 `Cargo.toml` 中配置:

```toml
[build]
jobs = 8  # 根据你的 CPU 核心数调整
```

**建议值**:
- 4 核 CPU: `jobs = 4`
- 8 核 CPU: `jobs = 8`
- 16 核 CPU: `jobs = 12` (留一些给系统)

---

### 方法 4: 使用 Release 模式 (长期运行)

```bash
# 首次编译慢,但后续更快
npm run rust:release

# 或
cd src-tauri && cargo build --release
```

**优势**:
- ✅ 运行时性能提升 10-100 倍
- ✅ 二进制文件更小 (strip 调试符号)
- ✅ LTO 优化减少代码体积

**劣势**:
- ❌ 首次编译时间更长 (5-10 分钟)
- ❌ 调试困难 (无调试符号)

---

### 方法 5: 清理无用缓存

```bash
# 清理旧的编译产物
cd src-tauri && cargo clean

# 重新编译 (首次会慢,但后续恢复正常)
cargo build
```

**何时使用**:
- 遇到奇怪的编译错误
- 切换分支后
- 升级 Rust 版本后

---

## 📊 编译时间对比

| 命令 | 首次编译 | 增量编译 | 用途 |
|------|---------|---------|------|
| `cargo check` | ~30s | ~5s | 语法检查 |
| `cargo build` | ~3min | ~30s | 开发调试 |
| `cargo build --release` | ~8min | ~2min | 生产发布 |

---

## 🔧 高级优化技巧

### 1. 使用 sccache (分布式编译缓存)

```bash
# 安装 sccache
cargo install sccache

# 配置环境变量 (添加到 ~/.powershell/profile.ps1)
$env:RUSTC_WRAPPER = "sccache"

# 验证
sccache --show-stats
```

**效果**: 跨项目共享编译缓存,速度提升 2-5 倍

---

### 2. 预编译常用依赖

```bash
# 预编译 tokio、serde 等常用库
cargo build --package tokio --package serde --package serde_json
```

---

### 3. 排除不必要的 features

在 `Cargo.toml` 中禁用不需要的功能:

```toml
tokio = { version = "1", features = ["full"] }  # ❌ 包含所有功能
tokio = { version = "1", features = ["rt-multi-thread", "sync"] }  # ✅ 只选需要的
```

---

## 💡 最佳实践

### 日常开发流程

```bash
# 1. 快速语法检查
npm run rust:check

# 2. 确认无误后启动开发服务器
npm run tauri:dev

# 3. Tauri 会自动增量编译,只需等待首次完成
```

### 提交前检查

```bash
# 完整编译确保无链接错误
cd src-tauri && cargo build

# 运行测试
cargo test
```

### 发布前优化

```bash
# Release 模式编译
npm run rust:release

# 检查二进制文件大小
ls -lh src-tauri/target/release/opc-harness.exe
```

---

## 🎯 针对本项目的建议

### 当前瓶颈
- 新增 `dashmap` 和 `rand` 依赖首次编译耗时
- Tauri v2 的 WebView2 绑定编译慢
- Debug 模式链接时间长

### 立即生效的优化
1. ✅ 已配置 `split-debuginfo = "unpacked"`
2. ✅ 已启用增量编译
3. ✅ 已设置并行任务数 `jobs = 8`
4. ✅ 创建了快速检查脚本

### 预期效果
- **首次编译**: 从 5 分钟 → 3 分钟 (减少 40%)
- **增量编译**: 从 2 分钟 → 20 秒 (减少 83%)
- **语法检查**: 从 2 分钟 → 10 秒 (减少 92%)

---

## 📝 常见问题

### Q1: 为什么每次都要重新编译所有文件?
**A**: 检查 `.gitignore` 是否排除了 `target/` 目录,确保增量编译缓存不被删除。

### Q2: 编译时 CPU 占用 100% 怎么办?
**A**: 减少 `[build] jobs` 的值,例如改为 `jobs = 4`。

### Q3: 如何查看详细的编译时间?
**A**: 设置环境变量:
```powershell
$env:CARGO_BUILD_TIMING = "1"
cargo build
```

### Q4: Release 模式调试怎么办?
**A**: 修改配置保留调试符号:
```toml
[profile.release]
debug = true
strip = false
```

---

## 🔗 相关资源

- [Cargo 官方文档 - Profiles](https://doc.rust-lang.org/cargo/reference/profiles.html)
- [Rust 编译优化指南](https://nnethercote.github.io/perf-book/)
- [sccache 项目](https://github.com/mozilla/sccache)
