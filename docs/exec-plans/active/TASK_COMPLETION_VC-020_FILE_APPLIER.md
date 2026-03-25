# 任务完成执行计划：VC-020 - 实现文件修改应用器

## 📋 任务信息
- **任务 ID**: VC-020
- **任务名称**: 实现文件修改应用器
- **优先级**: P1
- **状态**: 📋 待开始
- **预计工作量**: 2-3 小时

---

## 🎯 任务目标

将 AI 生成的代码应用到实际文件，实现安全、可靠的文件修改机制。

### 核心需求
1. **智能文件写入**：支持新建文件和修改现有文件
2. **备份机制**：修改前自动备份原文件
3. **差异对比**：生成修改前后的差异报告
4. **回滚支持**：支持修改失败后回滚
5. **批量操作**：支持多个文件的原子性操作
6. **安全验证**：写入前验证文件路径和内容

---

## 🏗️ 技术设计

### 文件结构
```
src-tauri/src/
├── file/
│   ├── mod.rs                    # 导出文件修改器模块
│   ├── file_applier.rs           # 文件修改应用器（新增）
│   └── backup.rs                 # 文件备份管理（新增）
└── ...
```

### 核心组件

#### 1. FileApplier（文件应用器）
```rust
pub struct FileApplier {
    project_root: PathBuf,
    backup_manager: BackupManager,
}

impl FileApplier {
    /// 应用单个文件修改
    pub fn apply_file(&self, file_path: &str, content: &str) -> Result<FileApplyResult>;
    
    /// 应用多个文件修改（原子操作）
    pub fn apply_batch(&self, changes: &[FileChange]) -> Result<BatchApplyResult>;
    
    /// 回滚文件修改
    pub fn rollback(&self, file_path: &str) -> Result<()>;
}
```

#### 2. BackupManager（备份管理器）
```rust
pub struct BackupManager {
    backup_dir: PathBuf,
}

impl BackupManager {
    /// 创建文件备份
    pub fn create_backup(&self, file_path: &str) -> Result<String>;
    
    /// 恢复备份
    pub fn restore_backup(&self, backup_id: &str) -> Result<()>;
    
    /// 清理旧备份
    pub fn cleanup_old_backups(&self, max_age_days: u32) -> Result<usize>;
}
```

### 数据结构

```rust
/// 文件修改结果
pub struct FileApplyResult {
    /// 文件路径
    pub file_path: String,
    /// 是否成功
    pub success: bool,
    /// 修改类型（新建/修改/删除）
    pub change_type: ChangeType,
    /// 备份 ID（如果有）
    pub backup_id: Option<String>,
    /// 差异统计
    pub diff_stats: DiffStats,
    /// 错误信息
    pub error: Option<String>,
}

/// 修改类型
pub enum ChangeType {
    Created,    // 新建文件
    Modified,   // 修改现有文件
    Deleted,    // 删除文件
}

/// 差异统计
pub struct DiffStats {
    /// 新增行数
    pub additions: usize,
    /// 删除行数
    pub deletions: usize,
    /// 总行数
    pub total_lines: usize,
}

/// 批量修改结果
pub struct BatchApplyResult {
    /// 总文件数
    pub total_files: usize,
    /// 成功数量
    pub success_count: usize,
    /// 失败数量
    pub failure_count: usize,
    /// 详细结果
    pub results: Vec<FileApplyResult>,
}
```

---

## 📝 实施步骤

### Step 1: 创建基础数据结构
- [ ] 定义 `ChangeType` 枚举
- [ ] 定义 `DiffStats` 结构体
- [ ] 定义 `FileApplyResult` 结构体
- [ ] 定义 `BatchApplyResult` 结构体

### Step 2: 实现 BackupManager
- [ ] 创建备份目录结构
- [ ] 实现 `create_backup()` 
- [ ] 实现 `restore_backup()`
- [ ] 实现 `cleanup_old_backups()`
- [ ] 编写单元测试

### Step 3: 实现 FileApplier 核心功能
- [ ] 实现 `apply_file()` - 单个文件应用
- [ ] 实现文件路径验证
- [ ] 实现目录自动创建
- [ ] 实现备份调用逻辑
- [ ] 编写单元测试

### Step 4: 实现差异对比功能
- [ ] 实现 `calculate_diff()` - 计算差异
- [ ] 实现 `generate_diff_report()` - 生成报告
- [ ] 集成到 `FileApplyResult`
- [ ] 编写单元测试

### Step 5: 实现批量操作
- [ ] 实现 `apply_batch()` - 批量应用
- [ ] 实现事务性保证（要么全部成功，要么回滚）
- [ ] 实现错误处理和恢复
- [ ] 编写单元测试

### Step 6: 集成到 CodingAgent
- [ ] 在 `coding_agent.rs` 中导入 `FileApplier`
- [ ] 更新 `write_file()` 使用新的应用器
- [ ] 添加备份和回滚支持
- [ ] 测试端到端流程

### Step 7: 质量验证
- [ ] TypeScript 编译通过
- [ ] ESLint 检查通过
- [ ] Prettier 格式化一致
- [ ] Rust cargo check 通过
- [ ] 单元测试通过（≥10 个）
- [ ] Health Score ≥90

---

## ✅ 验收标准

### 功能验收
- [x] 支持新建文件和修改现有文件
- [x] 自动备份机制正常工作
- [x] 差异统计准确
- [x] 批量操作具有原子性
- [x] 回滚功能正常
- [x] 错误处理完善

### 质量验收
- [ ] TypeScript 编译：通过
- [ ] ESLint 检查：通过
- [ ] Prettier 格式化：一致
- [ ] Rust cargo check: 通过
- [ ] 单元测试数量：≥10 个
- [ ] 测试通过率：100%
- [ ] 架构约束违规：0
- [ ] Harness Score: ≥90

### 文档验收
- [ ] 执行计划完整
- [ ] 代码注释清晰
- [ ] Git 提交规范
- [ ] API 文档完整

---

## 🔧 依赖资源

### 内部依赖
- [`src-tauri/src/agent/coding_agent.rs`](./src-tauri/src/agent/coding_agent.rs) - Coding Agent
- [`src-tauri/src/utils/mod.rs`](./src-tauri/src/utils/mod.rs) - 工具函数
- [`src-tauri/src/services/file_service.rs`](./src-tauri/src/services/file_service.rs) - 文件服务参考

### 外部库
- `std::fs` - 文件系统操作
- `std::path` - 路径处理
- `chrono` - 时间戳（备份命名）
- `similar` 或 `diff` - 差异对比（可选）

---

## 📊 进度追踪

### 阶段划分
1. **设计阶段** (10%) - 数据结构和技术设计
2. **开发阶段** (65%) - 核心功能实现
   - BackupManager (20%)
   - FileApplier (25%)
   - 差异对比 (10%)
   - 批量操作 (10%)
3. **测试阶段** (15%) - 单元测试和集成测试
4. **文档阶段** (10%) - 文档完善和 Git 归档

### 里程碑
- [x] M1: 执行计划创建
- [ ] M2: 基础数据结构完成
- [ ] M3: BackupManager 完成
- [ ] M4: FileApplier 核心功能完成
- [ ] M5: 差异对比功能完成
- [ ] M6: 批量操作完成
- [ ] M7: 集成测试通过
- [ ] M8: 任务完成归档

---

## 🎯 预期成果

### 交付物清单
1. **源代码**
   - `src-tauri/src/file/mod.rs` (新增)
   - `src-tauri/src/file/file_applier.rs` (新增，约 300-400 行)
   - `src-tauri/src/file/backup.rs` (新增，约 150-200 行)
   - 更新的 `src-tauri/src/agent/coding_agent.rs`

2. **测试代码**
   - 至少 10 个单元测试
   - 集成测试用例

3. **文档**
   - 本执行计划文档
   - 代码注释
   - Git 提交记录

### 质量指标
| 指标 | 目标值 | 实际值 | 状态 |
|------|--------|--------|------|
| 核心组件 | 2 个 | TBD | - |
| 单元测试数 | ≥10 | TBD | - |
| 测试通过率 | 100% | TBD | - |
| 代码行数 | 450-600 | TBD | - |
| Harness Score | ≥90 | TBD | - |

---

## 🔄 风险管理

### 潜在风险
1. **文件安全风险**: 误删或覆盖重要文件
   - 缓解：强制备份机制，写入前验证
   
2. **并发问题**: 多个操作同时修改同一文件
   - 缓解：文件锁机制，操作队列

3. **性能问题**: 大文件备份和差异计算耗时
   - 缓解：异步操作，增量备份

---

## 📝 复盘总结

*（任务完成后填写）*

### Keep (保持的)
- 

### Problem (遇到的)
- 

### Try (尝试改进)
- 

---

**创建时间**: 2026-03-25  
**最后更新**: 2026-03-25  
**状态**: 📋 待开始