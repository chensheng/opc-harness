## ADDED Requirements

### Requirement: 安全的文件读取操作

系统 SHALL 提供安全的文件读取工具，限制访问范围在工作空间内。

#### Scenario: 成功读取文件
- **WHEN** AI 调用 read_file 工具并提供有效路径
- **THEN** 系统验证路径在工作空间根目录内
- **AND** 读取文件内容
- **AND** 返回文件内容字符串（最大 500KB）

#### Scenario: 路径越界拒绝访问
- **WHEN** AI 尝试读取工作空间外的文件（如 `../../etc/passwd`）
- **THEN** 系统拒绝访问
- **AND** 返回错误信息 "Access denied: path outside workspace"
- **AND** 记录安全审计日志

#### Scenario: 大文件截断
- **WHEN** 文件大小超过 500KB
- **THEN** 系统只读取前 500KB
- **AND** 返回警告信息 "File truncated to 500KB"

---

### Requirement: 安全的文件写入操作

系统 SHALL 提供安全的文件写入工具，支持自动创建目录结构。

#### Scenario: 成功写入新文件
- **WHEN** AI 调用 write_file 工具提供路径和内容
- **THEN** 系统验证路径合法性
- **AND** 自动创建父目录（如果不存在）
- **AND** 写入文件内容
- **AND** 返回成功消息 "File written successfully"

#### Scenario: 覆盖现有文件
- **WHEN** AI 写入已存在的文件
- **THEN** 系统备份原文件到 `.backup/` 目录
- **AND** 写入新内容
- **AND** 记录备份路径用于回滚

#### Scenario: 禁止写入敏感文件
- **WHEN** AI 尝试写入 `.env`、`package-lock.json` 等受保护文件
- **THEN** 系统拒绝写入
- **AND** 返回错误 "Cannot modify protected file"

---

### Requirement: 目录列表操作

系统 SHALL 提供目录列表工具，支持递归和深度限制。

#### Scenario: 列出当前目录
- **WHEN** AI 调用 list_directory 工具
- **THEN** 系统返回当前目录下的文件和子目录列表
- **AND** 区分文件类型（file/directory）
- **AND** 包含文件大小和修改时间

#### Scenario: 递归列出子目录
- **WHEN** AI 指定 recursive=true 参数
- **THEN** 系统递归遍历所有子目录
- **AND** 限制最大深度为 5 层
- **AND** 返回扁平化的文件列表

---

### Requirement: 增量文件编辑操作

系统 SHALL 提供基于行号的增量编辑工具，支持插入、删除、替换操作。

#### Scenario: 替换指定行
- **WHEN** AI 调用 edit_file 工具提供 start_line、end_line 和新内容
- **THEN** 系统读取原文件
- **AND** 替换指定行范围的内容
- **AND** 保存修改后的文件
- **AND** 返回变更统计（新增/删除行数）

#### Scenario: 插入新代码块
- **WHEN** AI 指定 insert_after_line 参数
- **THEN** 系统在指定行后插入新内容
- **AND** 保持原有缩进格式
- **AND** 返回插入位置
