#!/usr/bin/env pwsh
# harness-rust-tests.ps1
# 独立的 Rust 单元测试执行脚本
# 用途：被 harness-check.ps1 调用，或单独运行

param(
    [switch]$Verbose,
    [switch]$Help
)

if ($Help) {
    Write-Host "Rust Unit Test Runner for OPC-HARNESS"
    Write-Host ""
    Write-Host "用法：.scripts\harness-rust-tests.ps1 [-Verbose]"
    Write-Host "参数:"
    Write-Host "  -Verbose  显示详细的测试输出"
    Write-Host "  -Help     显示此帮助信息"
    Write-Host "退出码:"
    Write-Host "  0 - 所有测试通过"
    Write-Host "  1 - 有测试失败"
    Write-Host "  2 - 执行错误（Cargo 不可用等）"
    exit 0
}

# Save original location
$originalLocation = Get-Location

# Check if Cargo is available
$cargoAvailable = $false
try {
    $null = Get-Command cargo -ErrorAction Stop
    $cargoAvailable = $true
} catch {
    Write-Host "[ERROR] Cargo not found" -ForegroundColor Red
    exit 2
}

# Check if src-tauri directory exists
if (-not (Test-Path "src-tauri")) {
    Write-Host "[ERROR] src-tauri directory not found" -ForegroundColor Red
    exit 2
}

Write-Host "Running Rust unit tests..." -ForegroundColor Yellow

# Change to src-tauri directory
Set-Location src-tauri

# Run cargo test and capture output to temp file
$tempOutputFile = [System.IO.Path]::GetTempFileName()

try {
    # Set environment variables to disable any GUI/debug behavior
    $env:RUST_LOG = "warn"
    $env:TAURI_DEBUG = "false"
    $env:HARNESS_TEST_MODE = "1"  # 禁用会启动 GUI 的测试
    
    # Execute cargo test and redirect all output to temp file
    & cargo test --bin opc-harness -- --nocapture > $tempOutputFile 2>&1
    $exitCode = $LASTEXITCODE
} catch {
    Write-Host "[ERROR] Exception during test execution: $_" -ForegroundColor Red
    Set-Location $originalLocation
    Remove-Item $tempOutputFile -Force -ErrorAction SilentlyContinue
    exit 2
}

# Restore original location
Set-Location $originalLocation

# Read output from temp file
if (-not (Test-Path $tempOutputFile)) {
    Write-Host "[ERROR] Test output file was not created" -ForegroundColor Red
    exit 2
}

$testOutput = Get-Content $tempOutputFile -Raw -Encoding UTF8
Remove-Item $tempOutputFile -Force -ErrorAction SilentlyContinue

# Parse and display results
if ($testOutput -match "test result: ok\. (\d+) passed") {
    $testCount = $matches[1]
    Write-Host "[PASS] All $testCount Rust tests passed" -ForegroundColor Green
    
    if ($Verbose) {
        Write-Host "`nTest Output:" -ForegroundColor Cyan
        Write-Host $testOutput -ForegroundColor Gray
    }
    
    exit 0
} elseif ($testOutput -match "test result: FAILED\. (\d+) passed; (\d+) failed") {
    $passed = $matches[1]
    $failed = $matches[2]
    Write-Host "[FAIL] Rust tests: $passed passed, $failed failed" -ForegroundColor Red
    
    if ($Verbose) {
        Write-Host "`nTest Output:" -ForegroundColor Cyan
        Write-Host $testOutput -ForegroundColor Gray
    }
    
    exit 1
} elseif ($testOutput -match "thread '.*' panicked") {
    Write-Host "[FAIL] Rust tests encountered a panic" -ForegroundColor Red
    
    if ($Verbose) {
        Write-Host "`nTest Output:" -ForegroundColor Cyan
        Write-Host $testOutput -ForegroundColor Gray
    }
    
    exit 1
} elseif ($testOutput -match "error: test failed") {
    Write-Host "[FAIL] Rust tests failed" -ForegroundColor Red
    
    if ($Verbose) {
        Write-Host "`nTest Output:" -ForegroundColor Cyan
        Write-Host $testOutput -ForegroundColor Gray
    }
    
    exit 1
} else {
    Write-Host "[WARN] Could not parse test results (exit code: $exitCode)" -ForegroundColor Yellow
    
    if ($Verbose) {
        Write-Host "`nTest Output:" -ForegroundColor Cyan
        Write-Host $testOutput -ForegroundColor Gray
    }
    
    # If exit code is 0, treat as success even if we can't parse
    if ($exitCode -eq 0) {
        Write-Host "Assuming success based on exit code" -ForegroundColor Yellow
        exit 0
    } else {
        exit 1
    }
}