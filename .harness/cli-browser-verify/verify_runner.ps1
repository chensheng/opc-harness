#!/usr/bin/env pwsh
# CLI Browser 验证运行器
# 利用现有 CLI 工具的浏览器能力进行验证

param(
    [string]$Suite = "smoke",
    [string]$ForceCLI = "",
    [switch]$Interactive
)

$ErrorActionPreference = "Stop"
$Colors = @{
    Success = "Green"
    Error = "Red"
    Warning = "Yellow"
    Info = "Cyan"
}

function Write-Header {
    param([string]$Title)
    Write-Host ""
    Write-Host "========================================" -ForegroundColor $Colors.Info
    Write-Host "  $Title" -ForegroundColor $Colors.Info
    Write-Host "========================================" -ForegroundColor $Colors.Info
    Write-Host ""
}

Write-Header "CLI Browser Verification"

# 1. 检测 CLI 工具
Write-Host "[1/4] 检测 CLI 浏览器工具..." -ForegroundColor $Colors.Warning

$CLIDetector = & "$PSScriptRoot/cli_detector.ps1" -ForceCLI $ForceCLI

if (-not $CLIDetector.Name) {
    Write-Host "  [FAIL] 未检测到支持的 CLI 工具" -ForegroundColor $Colors.Error
    Write-Host ""
    Write-Host "解决方案：" -ForegroundColor $Colors.Warning
    Write-Host "  1. 确保你在 Kimi Code CLI / Claude Code / OpenCode 中运行此命令" -ForegroundColor Gray
    Write-Host "  2. 或强制指定 CLI: .\verify_runner.ps1 -ForceCLI kimi" -ForegroundColor Gray
    Write-Host ""
    exit 1
}

Write-Host "  [PASS] 检测到 $($CLIDetector.Name) ($($CLIDetector.Type))" -ForegroundColor $Colors.Success
if ($CLIDetector.Version) {
    Write-Host "  版本: $($CLIDetector.Version)" -ForegroundColor Gray
}

# 2. 检查开发服务器
Write-Host "[2/4] 检查开发服务器..." -ForegroundColor $Colors.Warning

try {
    $response = Invoke-WebRequest -Uri "http://localhost:1420" -TimeoutSec 3 -ErrorAction Stop
    Write-Host "  [PASS] 开发服务器运行中 (http://localhost:1420)" -ForegroundColor $Colors.Success
} catch {
    Write-Host "  [WARN] 开发服务器未响应" -ForegroundColor $Colors.Warning
    Write-Host "  请先运行: npm run tauri:dev" -ForegroundColor Gray
}

# 3. 加载验证任务
Write-Host "[3/4] 加载验证任务..." -ForegroundColor $Colors.Warning

$TasksDir = "$PSScriptRoot/tasks"
$TaskFile = "$TasksDir/$Suite.yaml"

if (-not (Test-Path $TaskFile)) {
    Write-Host "  [FAIL] 未找到测试套件: $Suite" -ForegroundColor $Colors.Error
    Write-Host "  可用套件:" -ForegroundColor Gray
    Get-ChildItem $TasksDir -Filter "*.yaml" | ForEach-Object {
        Write-Host "    - $($_.BaseName)" -ForegroundColor Gray
    }
    exit 1
}

# 简单解析 YAML (简化版)
$TaskContent = Get-Content $TaskFile -Raw
Write-Host "  [PASS] 已加载测试套件: $Suite" -ForegroundColor $Colors.Success

# 4. 生成验证指令
Write-Host "[4/4] 生成 CLI 验证指令..." -ForegroundColor $Colors.Warning

$Instructions = @"
请使用浏览器工具验证以下任务：

目标URL: http://localhost:1420
测试套件: $Suite

$TaskContent

请按步骤执行验证，并报告：
1. 每个步骤的结果（通过/失败）
2. 发现的任何问题
3. 截图（如可能）

使用 $($CLIDetector.BrowserCommand) 命令访问页面。
"@

# 保存指令到文件
$InstructionFile = ".harness/cli-browser-verify/instructions.txt"
$Instructions | Out-File -FilePath $InstructionFile -Encoding UTF8

Write-Host "  [PASS] 验证指令已生成" -ForegroundColor $Colors.Success
Write-Host ""

# 显示结果
Write-Host "========================================" -ForegroundColor $Colors.Info
Write-Host "  验证准备完成" -ForegroundColor $Colors.Info
Write-Host "========================================" -ForegroundColor $Colors.Info
Write-Host ""
Write-Host "CLI 工具: $($CLIDetector.Name) ($($CLIDetector.Type))" -ForegroundColor White
Write-Host "测试套件: $Suite" -ForegroundColor White
Write-Host "目标URL: http://localhost:1420" -ForegroundColor White
Write-Host ""

if ($Interactive) {
    Write-Host "请在当前 CLI 中执行以下指令：" -ForegroundColor $Colors.Warning
    Write-Host ""
    Write-Host $CLIDetector.BrowserCommand "http://localhost:1420" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "然后参考验证任务文件：$InstructionFile" -ForegroundColor Gray
} else {
    Write-Host "验证指令已保存到: $InstructionFile" -ForegroundColor Gray
    Write-Host ""
    Write-Host "请在 Kimi / Claude / OpenCode 中执行：" -ForegroundColor $Colors.Warning
    Write-Host ""
    Write-Host "  @browser http://localhost:1420" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "然后查看验证任务：" -ForegroundColor Gray
    Write-Host "  Get-Content $InstructionFile" -ForegroundColor Gray
}

Write-Host ""
