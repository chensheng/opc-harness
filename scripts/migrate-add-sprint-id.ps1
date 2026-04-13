# 手动执行数据库迁移脚本
# 用途：为 user_stories 表添加 sprint_id 字段和索引

$ErrorActionPreference = "Stop"

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  数据库迁移：添加 sprint_id 支持" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# 获取数据库路径
$appRoot = Join-Path $env:USERPROFILE ".opc-harness"
$dbPath = Join-Path $appRoot "opc-harness.db"

Write-Host "[1/4] 检查数据库文件..." -ForegroundColor Yellow
if (-not (Test-Path $dbPath)) {
    Write-Host "  ✗ 数据库文件不存在: $dbPath" -ForegroundColor Red
    Write-Host "  提示：请先启动应用以创建数据库" -ForegroundColor Yellow
    exit 1
}
Write-Host "  ✓ 数据库文件存在: $dbPath" -ForegroundColor Green
Write-Host ""

# 检查 sqlite3 命令是否可用
Write-Host "[2/4] 检查 SQLite 工具..." -ForegroundColor Yellow
$sqlite3Path = Get-Command sqlite3 -ErrorAction SilentlyContinue
if (-not $sqlite3Path) {
    Write-Host "  ✗ 未找到 sqlite3 命令" -ForegroundColor Red
    Write-Host "  请安装 SQLite 命令行工具，或使用 Tauri 应用内置的迁移功能" -ForegroundColor Yellow
    exit 1
}
Write-Host "  ✓ SQLite 工具可用: $( $sqlite3Path.Source)" -ForegroundColor Green
Write-Host ""

# 检查 sprint_id 列是否存在
Write-Host "[3/4] 检查 sprint_id 列..." -ForegroundColor Yellow
$columnCheck = & sqlite3 $dbPath "SELECT COUNT(*) FROM pragma_table_info('user_stories') WHERE name='sprint_id';"
if ($columnCheck -eq "0") {
    Write-Host "  → sprint_id 列不存在，准备添加..." -ForegroundColor Yellow
    
    # 添加 sprint_id 列
    & sqlite3 $dbPath "ALTER TABLE user_stories ADD COLUMN sprint_id TEXT;"
    if ($LASTEXITCODE -eq 0) {
        Write-Host "  ✓ sprint_id 列添加成功" -ForegroundColor Green
    } else {
        Write-Host "  ✗ 添加 sprint_id 列失败" -ForegroundColor Red
        exit 1
    }
} else {
    Write-Host "  ✓ sprint_id 列已存在，跳过" -ForegroundColor Green
}
Write-Host ""

# 检查并创建索引
Write-Host "[4/4] 检查索引..." -ForegroundColor Yellow
$indexCheck = & sqlite3 $dbPath "SELECT COUNT(*) FROM sqlite_master WHERE type='index' AND name='idx_user_stories_sprint_id';"
if ($indexCheck -eq "0") {
    Write-Host "  → 索引不存在，准备创建..." -ForegroundColor Yellow
    
    # 创建索引
    & sqlite3 $dbPath "CREATE INDEX IF NOT EXISTS idx_user_stories_sprint_id ON user_stories(sprint_id);"
    if ($LASTEXITCODE -eq 0) {
        Write-Host "  ✓ 索引创建成功" -ForegroundColor Green
    } else {
        Write-Host "  ✗ 创建索引失败" -ForegroundColor Red
        exit 1
    }
} else {
    Write-Host "  ✓ 索引已存在，跳过" -ForegroundColor Green
}
Write-Host ""

# 验证迁移结果
Write-Host "验证迁移结果..." -ForegroundColor Cyan
$tableInfo = & sqlite3 $dbPath ".schema user_stories"
Write-Host ""
Write-Host "当前 user_stories 表结构：" -ForegroundColor Cyan
Write-Host $tableInfo
Write-Host ""

Write-Host "========================================" -ForegroundColor Green
Write-Host "  迁移完成！" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host ""
Write-Host "下一步：" -ForegroundColor Yellow
Write-Host "1. 重启 Tauri 应用（如果正在运行）" -ForegroundColor White
Write-Host "2. 刷新浏览器窗口（F5）" -ForegroundColor White
Write-Host "3. 检查控制台是否还有 'Invalid column name: sprint_id' 错误" -ForegroundColor White
