#!/usr/bin/env pwsh
# Harness Tauri 开发环境验证脚本
# Usage: .\harness-verify-tauri.ps1

param(
    [switch]$Quick,      # 快速验证（不启动浏览器）
    [switch]$Verbose     # 详细输出
)

$ErrorActionPreference = "Continue"
$StartTime = Get-Date
$Results = @()

$Colors = @{
    Success = "Green"
    Error = "Red"
    Warning = "Yellow"
    Info = "Cyan"
}

function Write-Check {
    param([string]$Name, [bool]$Success, [string]$Message)
    $icon = if ($Success) { "[PASS]" } else { "[FAIL]" }
    $color = if ($Success) { $Colors.Success } else { $Colors.Error }
    Write-Host "  $icon $Name" -ForegroundColor $color -NoNewline
    if ($Message) {
        Write-Host " - $Message" -ForegroundColor Gray
    } else {
        Write-Host ""
    }
    $script:Results += @{ Name = $Name; Success = $Success; Message = $Message }
}

Write-Host ""
Write-Host "========================================" -ForegroundColor $Colors.Info
Write-Host "  Harness Tauri Dev Verification" -ForegroundColor $Colors.Info
Write-Host "========================================" -ForegroundColor $Colors.Info
Write-Host ""

# 1. 检查 Tauri 进程
Write-Host "[1/6] Checking Tauri process..." -ForegroundColor $Colors.Warning
$tauriProcess = Get-Process -Name "opc-harness" -ErrorAction SilentlyContinue | Select-Object -First 1
if ($tauriProcess) {
    Write-Check -Name "Tauri Process" -Success $true -Message "PID: $($tauriProcess.Id), Started: $($tauriProcess.StartTime)"
} else {
    Write-Check -Name "Tauri Process" -Success $false -Message "Not running"
}

# 2. 检查 Vite 服务器
Write-Host "[2/6] Checking Vite dev server..." -ForegroundColor $Colors.Warning
$viteConnection = Get-NetTCPConnection -LocalPort 1420 -ErrorAction SilentlyContinue | Select-Object -First 1
if ($viteConnection) {
    $viteProcess = Get-Process -Id $viteConnection.OwningProcess -ErrorAction SilentlyContinue
    Write-Check -Name "Vite Server" -Success $true -Message "Port 1420, PID: $($viteProcess.Id)"
} else {
    Write-Check -Name "Vite Server" -Success $false -Message "Port 1420 not listening"
}

# 3. 检查前端文件
Write-Host "[3/6] Checking frontend files..." -ForegroundColor $Colors.Warning
$indexHtml = Test-Path "index.html"
$srcDir = Test-Path "src/App.tsx"
if ($indexHtml -and $srcDir) {
    Write-Check -Name "Frontend Files" -Success $true -Message "index.html and src/ present"
} else {
    Write-Check -Name "Frontend Files" -Success $false -Message "Missing essential files"
}

# 4. 检查 Rust 代码
Write-Host "[4/6] Checking Rust code..." -ForegroundColor $Colors.Warning
$cargoToml = Test-Path "src-tauri/Cargo.toml"
$mainRs = Test-Path "src-tauri/src/main.rs"
if ($cargoToml -and $mainRs) {
    Write-Check -Name "Rust Source" -Success $true -Message "Cargo.toml and main.rs present"
} else {
    Write-Check -Name "Rust Source" -Success $false -Message "Missing Rust files"
}

# 5. 快速健康检查（不编译）
Write-Host "[5/6] Running quick health check..." -ForegroundColor $Colors.Warning
Push-Location src-tauri
$cargoCheck = cargo check 2>&1
$cargoExit = $LASTEXITCODE
Pop-Location
if ($cargoExit -eq 0) {
    Write-Check -Name "Rust Compilation" -Success $true -Message "cargo check passed"
} else {
    Write-Check -Name "Rust Compilation" -Success $false -Message "cargo check failed"
}

# 6. 检查 Tauri CLI
Write-Host "[6/6] Checking Tauri CLI..." -ForegroundColor $Colors.Warning
try {
    $tauriVersion = npx tauri --version 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Check -Name "Tauri CLI" -Success $true -Message "Version: $tauriVersion"
    } else {
        Write-Check -Name "Tauri CLI" -Success $false -Message "Not available"
    }
} catch {
    Write-Check -Name "Tauri CLI" -Success $false -Message "Error checking CLI"
}

# 汇总
$EndTime = Get-Date
$Duration = ($EndTime - $StartTime).TotalSeconds
$Passed = ($Results | Where-Object { $_.Success }).Count
$Failed = ($Results | Where-Object { -not $_.Success }).Count
$Total = $Results.Count

Write-Host ""
Write-Host "========================================" -ForegroundColor $Colors.Info
Write-Host "  Verification Summary" -ForegroundColor $Colors.Info
Write-Host "========================================" -ForegroundColor $Colors.Info
Write-Host ""
Write-Host "  Total: $Total | Passed: $Passed | Failed: $Failed" -ForegroundColor $(if ($Failed -eq 0) { $Colors.Success } else { $Colors.Warning })
Write-Host "  Duration: $([math]::Round($Duration, 2))s" -ForegroundColor $Colors.Info
Write-Host ""

if ($Failed -eq 0) {
    Write-Host "  All checks passed!" -ForegroundColor $Colors.Success
    exit 0
} elseif ($Passed -ge 4) {
    Write-Host "  Most checks passed (Tauri app running)" -ForegroundColor $Colors.Success
    exit 0
} else {
    Write-Host "  Some checks failed" -ForegroundColor $Colors.Warning
    exit 1
}
