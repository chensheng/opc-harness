# OPC-HARNESS 一键启动脚本
# 需要先安装Rust环境

param(
    [switch]$Dev,
    [switch]$Build,
    [switch]$Check
)

# 颜色输出函数
function Write-ColorOutput($ForegroundColor) {
    $fc = $host.UI.RawUI.ForegroundColor
    $host.UI.RawUI.ForegroundColor = $ForegroundColor
    if ($args) {
        Write-Output $args
    }
    $host.UI.RawUI.ForegroundColor = $fc
}

Write-ColorOutput Green "=========================================="
Write-ColorOutput Green "  OPC-HARNESS 开发环境检查和启动脚本"
Write-ColorOutput Green "=========================================="
Write-Output ""

# 检查Rust环境
Write-ColorOutput Yellow "[1/4] 检查Rust环境..."
$rustc = Get-Command rustc -ErrorAction SilentlyContinue
$cargo = Get-Command cargo -ErrorAction SilentlyContinue

if (-not $rustc -or -not $cargo) {
    Write-ColorOutput Red "❌ Rust未安装!"
    Write-Output ""
    Write-ColorOutput Yellow "请按以下步骤安装Rust:"
    Write-Output "  1. 以管理员身份打开PowerShell"
    Write-Output "  2. 运行: winget install Rustlang.Rustup"
    Write-Output "  3. 重启终端"
    Write-Output ""
    Write-Output "详细说明请参考: docs/Rust安装与启动指南.md"
    exit 1
}

Write-ColorOutput Green "✓ Rust已安装: $(rustc --version)"
Write-ColorOutput Green "✓ Cargo已安装: $(cargo --version)"
Write-Output ""

# 检查Node.js
Write-ColorOutput Yellow "[2/4] 检查Node.js环境..."
$node = Get-Command node -ErrorAction SilentlyContinue
if (-not $node) {
    Write-ColorOutput Red "❌ Node.js未安装!"
    Write-Output "请访问 https://nodejs.org/ 下载安装Node.js 18+"
    exit 1
}

Write-ColorOutput Green "✓ Node.js已安装: $(node --version)"
Write-Output ""

# 检查项目目录
Write-ColorOutput Yellow "[3/4] 检查项目结构..."
$projectRoot = Split-Path -Parent $PSScriptRoot
Set-Location $projectRoot

$requiredFiles = @(
    "package.json",
    "vite.config.ts",
    "src-tauri/Cargo.toml",
    "src-tauri/tauri.conf.json"
)

$allExists = $true
foreach ($file in $requiredFiles) {
    $path = Join-Path $projectRoot $file
    if (Test-Path $path) {
        Write-ColorOutput Green "  ✓ $file"
    } else {
        Write-ColorOutput Red "  ✗ $file (缺失)"
        $allExists = $false
    }
}

if (-not $allExists) {
    Write-ColorOutput Red "❌ 项目结构不完整，请检查文件是否齐全"
    exit 1
}
Write-Output ""

# 检查/安装依赖
Write-ColorOutput Yellow "[4/4] 检查项目依赖..."
if (-not (Test-Path (Join-Path $projectRoot "node_modules"))) {
    Write-ColorOutput Yellow "  → 正在安装npm依赖..."
    npm install
    if ($LASTEXITCODE -ne 0) {
        Write-ColorOutput Red "❌ npm install 失败"
        exit 1
    }
} else {
    Write-ColorOutput Green "✓ npm依赖已安装"
}
Write-Output ""

# 执行操作
if ($Check) {
    Write-ColorOutput Green "=========================================="
    Write-ColorOutput Green "  ✓ 环境检查通过!"
    Write-ColorOutput Green "=========================================="
    Write-Output ""
    Write-Output "可以运行以下命令启动应用:"
    Write-Output "  .\scripts\setup-and-run.ps1 -Dev    # 开发模式"
    Write-Output "  .\scripts\setup-and-run.ps1 -Build  # 构建发布"
    exit 0
}

if ($Dev) {
    Write-ColorOutput Green "=========================================="
    Write-ColorOutput Green "  启动开发服务器..."
    Write-ColorOutput Green "=========================================="
    Write-Output ""
    npm run tauri:dev
}
elseif ($Build) {
    Write-ColorOutput Green "=========================================="
    Write-ColorOutput Green "  构建生产版本..."
    Write-ColorOutput Green "=========================================="
    Write-Output ""
    npm run tauri:build
    
    Write-Output ""
    Write-ColorOutput Green "构建完成! 安装包位置:"
    Write-Output "  src-tauri/target/release/bundle/"
}
else {
    Write-ColorOutput Green "=========================================="
    Write-ColorOutput Green "  ✓ 环境检查完成!"
    Write-ColorOutput Green "=========================================="
    Write-Output ""
    Write-Output "使用方式:"
    Write-Output "  .\scripts\setup-and-run.ps1 -Check   # 仅检查环境"
    Write-Output "  .\scripts\setup-and-run.ps1 -Dev    # 启动开发服务器"
    Write-Output "  .\scripts\setup-and-run.ps1 -Build  # 构建发布版本"
}
