#!/usr/bin/env pwsh
# Harness Engineering 代码质量自动修复脚本
# 用法：.\scripts\fix-code-quality.ps1

param(
    [switch]$SkipTypeCheck,   # 跳过类型检查
    [switch]$DryRun           # 仅显示将要执行的命令，不实际执行
)

$ErrorActionPreference = "Stop"
$StartTime = Get-Date

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  OPC-HARNESS Code Quality Auto-Fix" -ForegroundColor Cyan
Write-Host ""

if ($DryRun) {
    Write-Host "[DRY RUN] The following actions will be performed:" -ForegroundColor Yellow
    Write-Host ""
}

# Step 1: TypeScript type check
Write-Host "[Step 1/3] Running TypeScript Type Check..." -ForegroundColor Yellow
if (-not $SkipTypeCheck) {
    if ($DryRun) {
        Write-Host "  Would run: npx tsc --noEmit" -ForegroundColor Gray
    } else {
        $tscResult = npx tsc --noEmit 2>&1
        if ($LASTEXITCODE -ne 0) {
            Write-Host "  [ERROR] TypeScript type check failed!" -ForegroundColor Red
            Write-Host $tscResult -ForegroundColor Gray
            Write-Host ""
            Write-Host "Please fix TypeScript errors before continuing." -ForegroundColor Yellow
            exit 1
        } else {
            Write-Host "  [PASS] TypeScript type check passed" -ForegroundColor Green
        }
    }
} else {
    Write-Host "  [SKIP] TypeScript check skipped" -ForegroundColor Yellow
}

# Step 2: Prettier formatting
Write-Host ""
Write-Host "[Step 2/3] Formatting Code with Prettier..." -ForegroundColor Yellow
if ($DryRun) {
    Write-Host "  Would run: npm run format" -ForegroundColor Gray
} else {
    npm run format | Out-Null
    if ($LASTEXITCODE -eq 0) {
        Write-Host "  [OK] Code formatted successfully" -ForegroundColor Green
    } else {
        Write-Host "  [ERROR] Prettier formatting failed" -ForegroundColor Red
        exit 1
    }
}

# Step 3: ESLint auto-fix (if available)
Write-Host ""
Write-Host "[Step 3/3] Attempting ESLint Auto-Fix..." -ForegroundColor Yellow
if ($DryRun) {
    Write-Host "  Would run: npm run lint:fix" -ForegroundColor Gray
} else {
    try {
        $eslintResult = npm run lint:fix 2>&1
        if ($LASTEXITCODE -eq 0) {
            Write-Host "  [OK] ESLint auto-fix completed" -ForegroundColor Green
        } else {
            Write-Host "  [WARN] ESLint has some issues that couldn't be auto-fixed" -ForegroundColor Yellow
        }
    } catch {
        Write-Host "  [WARN] ESLint not available, skipping auto-fix" -ForegroundColor Yellow
    }
}

# Summary
$EndTime = Get-Date
$Duration = ($EndTime - $StartTime).TotalSeconds

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  Fix Summary" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "  Duration: $([math]::Round($Duration, 2)) seconds" -ForegroundColor Cyan
Write-Host "  Status: Completed" -ForegroundColor Green
Write-Host ""

if (-not $DryRun) {
    Write-Host "Next steps:" -ForegroundColor Blue
    Write-Host "  1. Run '.\scripts\harness-check.ps1' to verify overall health" -ForegroundColor Gray
    Write-Host "  2. Review git diff to see what changed" -ForegroundColor Gray
    Write-Host "  3. Commit your changes" -ForegroundColor Gray
    Write-Host ""
}

exit 0
