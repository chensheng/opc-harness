# 快速编译脚本 - 仅检查语法,不链接
# 使用方法: .\scripts\fast-check.ps1

Write-Host "🚀 快速语法检查 (跳过链接阶段)..." -ForegroundColor Cyan

$startTime = Get-Date

# 只检查库文件,速度提升 5-10 倍
cargo check --lib --quiet

$endTime = Get-Date
$duration = ($endTime - $startTime).TotalSeconds

if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ 语法检查通过! 耗时: $([math]::Round($duration, 2)) 秒" -ForegroundColor Green
} else {
    Write-Host "❌ 发现语法错误,请查看上方详细信息" -ForegroundColor Red
    exit 1
}
