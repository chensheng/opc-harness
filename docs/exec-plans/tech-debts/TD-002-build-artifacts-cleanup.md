# TD-002: 前端构建产物未清理

## 📋 基本信息

- **创建日期**: 2026-03-20
- **优先级**: P3 (轻微)
- **状态**: 📋 待解决
- **影响范围**: 磁盘空间、部署包大小
- **负责人**: 未分配
- **偿还计划**: 2026-04 周

---

## 📝 问题描述

`dist/` 和 `target/` 目录包含多个历史构建版本，未定期清理，导致：

- 磁盘空间浪费（约 2GB）
- Git 仓库体积增大
- 部署时间增加

### 影响分析

1. **开发体验**: 本地构建产物占用大量磁盘空间
2. **CI/CD**: 构建缓存未清理，影响部署速度
3. **版本控制**: 可能误提交构建产物到 Git

---

## 🎯 解决方案

### 方案 1: 使用自动化脚本

```bash
# 运行垃圾回收
npm run harness:gc

# 或手动清理
rm -rf dist/
cargo clean
```

### 方案 2: 集成到 CI/CD 流程

在 `.github/workflows` 或 CI 配置中添加清理步骤：

```yaml
- name: Clean build artifacts
  run: |
    npm run harness:gc
    cargo clean
```

### 方案 3: Git 忽略配置

确保 `.gitignore` 包含：

```gitignore
# Build outputs
dist/
build/
target/

# Tauri
src-tauri/target/
src-tauri/icons/
```

---

## ✅ 验收标准

- [ ] `dist/` 目录大小 < 100MB
- [ ] `target/` 目录大小 < 500MB
- [ ] `.gitignore` 正确配置
- [ ] `harness:gc` 脚本正常工作

---

## 🔧 执行步骤

### 步骤 1: 验证当前状态

```bash
# 查看目录大小
du -sh dist/ target/ src-tauri/target/
```

### 步骤 2: 运行清理

```bash
# 清理前端构建产物
npm run harness:gc

# 清理 Rust 构建产物
cd src-tauri
cargo clean
```

### 步骤 3: 验证清理结果

```bash
# 再次检查目录大小
du -sh dist/ target/ src-tauri/target/
```

---

## 📚 相关资源

- [Cargo clean 文档](https://doc.rust-lang.org/cargo/commands/cargo-clean.html)
- [harness-gc.ps1 脚本](file://d:\workspace\opc-harness\scripts\harness-gc.ps1)

---

**最后更新**: 2026-03-24
