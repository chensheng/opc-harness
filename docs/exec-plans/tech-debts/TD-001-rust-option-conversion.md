# TD-001: Rust Option<String>类型转换问题

## 📋 基本信息

- **创建日期**: 2026-03-16
- **优先级**: P2 (中等)
- **状态**: 📋 待解决
- **影响范围**: `src-tauri/src/db/mod.rs`
- **负责人**: 未分配
- **偿还计划**: 2026-03-24 周

---

## 📝 问题描述

`create_project`和 `update_project` 函数中，`Option<String>` 类型与 rusqlite 参数绑定不匹配，导致编译错误。

### 根本原因

- rusqlite 需要 `&str` 类型参数
- `Option<String>.map_or()` 返回类型推断错误

### 当前错误代码

```rust
// ❌ 当前错误代码
.bind(2, project.description.map_or("", |d| d.as_str()))?
```

### 修复方案

```rust
// ✅ 修复方案：使用 as_deref()
.bind(2, project.description.as_deref())?
```

---

## 🎯 解决方案

### 步骤 1: 修改 create_project 函数

```rust
// src-tauri/src/db/mod.rs
pub fn create_project(&self, project: &Project) -> Result<i64> {
    let mut stmt = self.conn.prepare(
        "INSERT INTO projects (name, description, created_at, updated_at) 
         VALUES (?1, ?2, ?3, ?4)"
    )?;
    
    stmt.execute(params![
        project.name,
        project.description.as_deref(), // ✅ 修复
        project.created_at,
        project.updated_at
    ])?;
    
    Ok(self.conn.last_insert_rowid())
}
```

### 步骤 2: 修改 update_project 函数

```rust
pub fn update_project(&self, project: &Project) -> Result<()> {
    let mut stmt = self.conn.prepare(
        "UPDATE projects SET name = ?1, description = ?2, updated_at = ?3 
         WHERE id = ?4"
    )?;
    
    stmt.execute(params![
        project.name,
        project.description.as_deref(), // ✅ 修复
        project.updated_at,
        project.id
    ])?;
    
    Ok(())
}
```

---

## ✅ 验收标准

- [ ] 代码编译通过，无警告
- [ ] 数据库操作测试通过
- [ ] 类型推导正确，无需显式类型注解

---

## 📚 相关资源

- [Rust Option 文档](https://doc.rust-lang.org/std/option/)
- [rusqlite 参数绑定](https://docs.rs/rusqlite/latest/rusqlite/struct.Statement.html#method.execute)

---

**最后更新**: 2026-03-24
