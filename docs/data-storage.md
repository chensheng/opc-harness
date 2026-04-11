# OPC-HARNESS 数据存储说明

## 📁 数据存储位置

OPC-HARNESS 采用业界标准做法（参考 OpenClaw、Claude Code），在用户 **home 目录**下创建 `.opc-harness` 隐藏目录来存储所有应用数据。

### 跨平台路径

| 平台        | 路径                              |
| ----------- | --------------------------------- |
| **Windows** | `C:\Users\<用户名>\.opc-harness\` |
| **macOS**   | `/Users/<用户名>/.opc-harness/`   |
| **Linux**   | `/home/<用户名>/.opc-harness/`    |

### 目录结构

```
.opc-harness/
├── opc-harness.db          # SQLite 数据库文件（核心数据）
├── config/                 # 配置文件目录
│   ├── ai-providers.json   # AI 提供商配置
│   └── preferences.json    # 用户偏好设置
├── logs/                   # 日志文件目录
│   ├── app.log             # 应用日志
│   └── agent.log           # Agent 运行日志
├── cache/                  # 缓存文件目录
│   ├── prd-cache/          # PRD 生成缓存
│   └── analysis-cache/     # 分析结果缓存
└── sessions/               # 会话数据目录
    ├── agent-sessions/     # Agent 会话记录
    └── cli-sessions/       # CLI 会话记录
```

## 🔧 自定义数据目录

如果需要自定义数据存储位置，可以设置环境变量：

### Windows (PowerShell)

```powershell
$env:OPC_HARNESS_HOME = "D:\my-opc-data"
```

### macOS/Linux (Bash)

```bash
export OPC_HARNESS_HOME="/path/to/custom/dir"
```

### 永久设置

**Windows:**

```powershell
# 添加到系统环境变量
[Environment]::SetEnvironmentVariable("OPC_HARNESS_HOME", "D:\my-opc-data", "User")
```

**macOS/Linux:**

```bash
# 添加到 ~/.bashrc 或 ~/.zshrc
echo 'export OPC_HARNESS_HOME="/path/to/custom/dir"' >> ~/.bashrc
source ~/.bashrc
```

## 📊 数据库说明

### 主要数据表

- **projects**: 项目信息
- **ai_configs**: AI 配置信息
- **cli_sessions**: CLI 会话记录
- **agent_sessions**: Agent 会话记录
- **milestones**: 里程碑管理
- **issues**: 任务/问题跟踪
- **user_stories**: 用户故事管理

### 备份与迁移

由于所有数据都存储在 `~/.opc-harness/` 目录下，备份和迁移非常简单：

```bash
# 备份
tar -czf opc-harness-backup.tar.gz ~/.opc-harness/

# 恢复
tar -xzf opc-harness-backup.tar.gz -C ~/
```

## 🔒 数据安全

- 数据库文件使用 SQLite WAL 模式，提供更好的并发性能
- 敏感信息（如 API Key）存储在系统密钥链中（通过 keyring 库）
- 日志文件会自动轮转，避免占用过多磁盘空间

## 🧹 清理缓存

如需清理缓存文件：

```bash
# 清理缓存目录（不会影响核心数据）
rm -rf ~/.opc-harness/cache/*

# 清理日志文件
rm -rf ~/.opc-harness/logs/*
```

## 📝 注意事项

1. **不要手动删除** `opc-harness.db` 文件，这会导致所有项目数据丢失
2. **定期备份** `~/.opc-harness/` 目录，防止数据丢失
3. **磁盘空间**: 确保有足够的磁盘空间，特别是 `cache/` 和 `logs/` 目录可能会增长
4. **权限**: 确保当前用户对 `~/.opc-harness/` 目录有读写权限
