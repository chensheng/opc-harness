#!/usr/bin/env pwsh
# Harness Quick Verify Script
# Usage: .\scripts\harness-quick-verify.ps1

param(
    [switch]$Watch,
    [switch]$SkipBuild
)

$ErrorActionPreference = "Continue"
$StartTime = Get-Date

$Colors = @{
    Success = "Green"
    Error = "Red"
    Warning = "Yellow"
    Info = "Cyan"
}

function Write-Step {
    param([string]$Message)
    Write-Host "[QUICK] $Message" -ForegroundColor $Colors.Info
}

function Write-Ok {
    param([string]$Message)
    Write-Host "  [OK] $Message" -ForegroundColor $Colors.Success
}

function Write-Fail {
    param([string]$Message)
    Write-Host "  [FAIL] $Message" -ForegroundColor $Colors.Error
}

Write-Host ""
Write-Host "========================================" -ForegroundColor $Colors.Info
Write-Host "  Harness Quick Verify" -ForegroundColor $Colors.Info
Write-Host "========================================" -ForegroundColor $Colors.Info
Write-Host ""

$hasError = $false

# 1. Quick TypeScript check
Write-Step "TypeScript type checking..."
$tscResult = npx tsc --noEmit 2>&1
if ($LASTEXITCODE -eq 0) {
    Write-Ok "TypeScript check passed"
} else {
    Write-Fail "TypeScript has errors"
    $hasError = $true
}

# 2. Quick ESLint check
Write-Step "ESLint code checking..."
$eslintResult = npm run lint 2>&1
if ($LASTEXITCODE -eq 0) {
    Write-Ok "ESLint check passed"
} else {
    Write-Fail "ESLint found issues"
    $hasError = $true
}

# 3. Rust quick check
Write-Step "Rust compilation check..."
Push-Location src-tauri
$cargoResult = cargo check 2>&1
if ($LASTEXITCODE -eq 0) {
    Write-Ok "Rust check passed"
} else {
    Write-Fail "Rust has compilation errors"
    $hasError = $true
}
Pop-Location

# 4. Check dev server status
Write-Step "Development server status..."
try {
    $response = Invoke-WebRequest -Uri "http://localhost:1420" -TimeoutSec 2 -ErrorAction SilentlyContinue
    if ($response.StatusCode -eq 200) {
        Write-Ok "Development server running (http://localhost:1420)"
    } else {
        Write-Fail "Development server response abnormal"
    }
} catch {
    Write-Fail "Development server not running"
    Write-Host "  Tip: Run 'npm run dev' to start development server" -ForegroundColor $Colors.Warning
}

# Summary
$EndTime = Get-Date
$Duration = ($EndTime - $StartTime).TotalSeconds

Write-Host ""
Write-Host "========================================" -ForegroundColor $Colors.Info
if (-not $hasError) {
    Write-Host "  Quick verify PASSED ($([math]::Round($Duration, 1))s)" -ForegroundColor $Colors.Success
} else {
    Write-Host "  Quick verify FAILED ($([math]::Round($Duration, 1))s)" -ForegroundColor $Colors.Error
}
Write-Host "========================================" -ForegroundColor $Colors.Info
Write-Host ""

exit $(if ($hasError) { 1 } else { 0 })
