# 数据目录迁移指南

## 📋 概述

OPC-HARNESS 已优化数据存储位置，从 Tauri 默认的 `AppData` 目录迁移到用户 home 目录下的 `.opc-harness` 目录。

### 变更对比

| 项目 | 旧路径 | 新路径 |
|------|--------|--------|
| **Windows** | `%LOCALAPPDATA%\com.opc-harness.app\opc-harness.db` | `%USERPROFILE%\.opc-harness\opc-harness.db` |
| **macOS** | `~/Library/Application Support/com.opc-harness.app/opc-harness.db` | `~/.opc-harness/opc-harness.db` |
| **Linux** | `~/.local/share/com.opc-harness.app/opc-harness.db` | `~/.opc-harness/opc-harness.db` |

## 🔄 自动迁移

首次启动新版本时，系统会**自动检测并迁移**旧数据：

1. 检查旧数据目录是否存在
2. 如果存在，复制到新目录
3. 验证数据完整性
4. 保留旧数据作为备份（不会删除）

## 🛠️ 手动迁移

如果需要手动迁移数据，请按照以下步骤操作：

### Windows (PowerShell)

```powershell
# 1. 定义路径
$oldPath = "$env:LOCALAPPDATA\com.opc-harness.app"
$newPath = "$env:USERPROFILE\.opc-harness"

# 2. 创建新目录
New-Item -ItemType Directory -Force -Path $newPath

# 3. 复制数据库文件
if (Test-Path "$oldPath\opc-harness.db") {
    Copy-Item "$oldPath\opc-harness.db" "$newPath\opc-harness.db"
    Write-Host "✅ 数据库文件已迁移" -ForegroundColor Green
} else {
    Write-Host "⚠️  未找到旧数据库文件" -ForegroundColor Yellow
}

# 4. 验证新文件
if (Test-Path "$newPath\opc-harness.db") {
    Write-Host "✅ 迁移完成！新数据库位置: $newPath" -ForegroundColor Green
}
```

### macOS/Linux (Bash)

```bash
#!/bin/bash

# 1. 定义路径
OLD_PATH="$HOME/Library/Application Support/com.opc-harness.app"  # macOS
# OLD_PATH="$HOME/.local/share/com.opc-harness.app"  # Linux
NEW_PATH="$HOME/.opc-harness"

# 2. 创建新目录
mkdir -p "$NEW_PATH"

# 3. 复制数据库文件
if [ -f "$OLD_PATH/opc-harness.db" ]; then
    cp "$OLD_PATH/opc-harness.db" "$NEW_PATH/opc-harness.db"
    echo "✅ 数据库文件已迁移"
else
    echo "⚠️  未找到旧数据库文件"
fi

# 4. 验证新文件
if [ -f "$NEW_PATH/opc-harness.db" ]; then
    echo "✅ 迁移完成！新数据库位置: $NEW_PATH"
fi
```

## ✅ 验证迁移

迁移完成后，可以通过以下方式验证：

### 方法 1: 检查文件存在性

**Windows:**
```powershell
Test-Path "$env:USERPROFILE\.opc-harness\opc-harness.db"
```

**macOS/Linux:**
```bash
ls -lh ~/.opc-harness/opc-harness.db
```

### 方法 2: 启动应用

启动 OPC-HARNESS 应用，检查是否能正常加载之前的项目数据。

### 方法 3: 查看日志

应用启动时会输出数据库路径：
```
Initializing database at: "C:\\Users\\<username>\\.opc-harness\\opc-harness.db"
```

## 🔙 回滚

如果遇到问题需要回滚到旧版本：

1. 停止 OPC-HARNESS 应用
2. 将新数据库文件复制回旧位置
3. 使用旧版本的应用

**Windows:**
```powershell
Copy-Item "$env:USERPROFILE\.opc-harness\opc-harness.db" "$env:LOCALAPPDATA\com.opc-harness.app\opc-harness.db"
```

**macOS/Linux:**
```bash
cp ~/.opc-harness/opc-harness.db "~/Library/Application Support/com.opc-harness.app/opc-harness.db"
```

## ❓ 常见问题

### Q1: 迁移后找不到之前的项目？

**A:** 请确认：
1. 旧数据库文件确实存在于原位置
2. 迁移过程中没有发生错误
3. 新数据库文件已成功创建

### Q2: 可以同时保留两个版本的数据库吗？

**A:** 可以。迁移过程只是复制，不会删除旧文件。但建议：
1. 确认新版本工作正常后
2. 再考虑删除旧数据库文件以释放空间

### Q3: 如何自定义数据存储位置？

**A:** 设置环境变量 `OPC_HARNESS_HOME`：

```powershell
# Windows
$env:OPC_HARNESS_HOME = "D:\my-opc-data"
```

```bash
# macOS/Linux
export OPC_HARNESS_HOME="/path/to/custom/dir"
```

### Q4: 迁移会影响性能吗？

**A:** 不会。新位置（home 目录）通常比 AppData 目录访问更快，且：
- 更符合开发者工具惯例
- 便于备份和迁移
- 避免系统清理误删

## 📞 技术支持

如遇到迁移问题，请：

1. 检查应用日志文件：`~/.opc-harness/logs/app.log`
2. 确认文件权限正确
3. 联系技术支持团队

---

**最后更新**: 2026-04-11  
**版本**: OPC-HARNESS v0.1.0+
