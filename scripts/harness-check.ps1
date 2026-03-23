#!/usr/bin/env pwsh
# Harness Engineering 架构健康检查脚本
# 用法：.\scripts\harness-check.ps1

param(
    [switch]$Fix,          # 自动修复问题
    [switch]$Verbose,      # 详细输出
    [switch]$Json,         # JSON 格式输出
    [switch]$DocCheck,     # 包含文档一致性检查
    [switch]$DeadCode,     # 包含死代码检测
    [switch]$All           # 运行所有检查（包括文档和死代码）
)

$ErrorActionPreference = "Stop"
$StartTime = Get-Date
$Score = 100
$Issues = @()

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  OPC-HARNESS Architecture Health Check" -ForegroundColor Cyan
Write-Host ""

# 1. TypeScript type checking
Write-Host "[1/6] TypeScript Type Checking..." -ForegroundColor Yellow
try {
    $tscResult = npx tsc --noEmit 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host "  [PASS] TypeScript type checking passed" -ForegroundColor Green
    } else {
        Write-Host "  [FAIL] TypeScript type checking failed" -ForegroundColor Red
        $Issues += @{ Type = "TypeScript"; Severity = "Error"; Message = "Type check failed" }
        $Score -= 20
        if ($Verbose) {
            Write-Host $tscResult -ForegroundColor Gray
        }
    }
} catch {
    Write-Host "  [WARN] Cannot execute TypeScript check" -ForegroundColor Yellow
    $Issues += @{ Type = "TypeScript"; Severity = "Warning"; Message = "Check tool unavailable" }
}

# 2. ESLint code quality check
Write-Host "[2/6] ESLint Code Quality Check..." -ForegroundColor Yellow
try {
    $eslintResult = npm run lint 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host "  [PASS] ESLint check passed" -ForegroundColor Green
    } else {
        Write-Host "  [FAIL] ESLint check has warnings/errors" -ForegroundColor Red
        $Issues += @{ Type = "ESLint"; Severity = "Error"; Message = "Code style violation" }
        $Score -= 15
        if ($Verbose -or $Fix) {
            Write-Host $eslintResult -ForegroundColor Gray
        }
        
        if ($Fix) {
            Write-Host "  [FIX] Attempting auto-fix..." -ForegroundColor Blue
            npm run lint:fix | Out-Null
            Write-Host "  [OK] Auto-fix completed" -ForegroundColor Green
        }
    }
} catch {
    Write-Host "  [WARN] Cannot execute ESLint check" -ForegroundColor Yellow
    $Issues += @{ Type = "ESLint"; Severity = "Warning"; Message = "Check tool unavailable" }
}

# 3. Prettier formatting check
Write-Host "[3/6] Prettier Formatting Check..." -ForegroundColor Yellow
try {
    $prettierResult = npm run format:check 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host "  [PASS] Prettier formatting passed" -ForegroundColor Green
    } else {
        Write-Host "  [FAIL] Prettier formatting failed" -ForegroundColor Red
        $Issues += @{ Type = "Prettier"; Severity = "Error"; Message = "Code format not standard" }
        $Score -= 10
        
        if ($Fix) {
            Write-Host "  [FIX] Auto-formatting code..." -ForegroundColor Blue
            npm run format | Out-Null
            Write-Host "  [OK] Code formatted" -ForegroundColor Green
        }
    }
} catch {
    Write-Host "  [WARN] Cannot execute Prettier check" -ForegroundColor Yellow
    $Issues += @{ Type = "Prettier"; Severity = "Warning"; Message = "Check tool unavailable" }
}

# 4. Rust/Cargo compilation check
Write-Host "[4/6] Rust Compilation Check..." -ForegroundColor Yellow
Set-Location src-tauri
try {
    $cargoResult = cargo check 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host "  [PASS] Rust compilation check passed" -ForegroundColor Green
    } else {
        Write-Host "  [FAIL] Rust compilation check failed" -ForegroundColor Red
        $Issues += @{ Type = "Rust"; Severity = "Error"; Message = "Compilation error" }
        $Score -= 25
        if ($Verbose) {
            Write-Host $cargoResult -ForegroundColor Gray
        }
    }
} catch {
    Write-Host "  [WARN] Cannot execute Cargo check (Rust may not be installed)" -ForegroundColor Yellow
    $Issues += @{ Type = "Rust"; Severity = "Warning"; Message = "Rust environment not ready" }
}
Set-Location ..

# 5. Dependency integrity check
Write-Host "[5/6] Dependency Integrity Check..." -ForegroundColor Yellow
$packageLockExists = Test-Path "package-lock.json"
$nodeModulesExists = Test-Path "node_modules"
$cargoLockExists = Test-Path "src-tauri\Cargo.lock"

if ($packageLockExists -and $nodeModulesExists -and $cargoLockExists) {
    Write-Host "  [PASS] Dependency files intact" -ForegroundColor Green
} else {
    Write-Host "  [WARN] Some dependency files missing" -ForegroundColor Yellow
    $Issues += @{ Type = "Dependencies"; Severity = "Warning"; Message = "Incomplete dependencies" }
    $Score -= 5
    
    if (-not $packageLockExists) {
        Write-Host "    - Missing package-lock.json" -ForegroundColor Gray
    }
    if (-not $nodeModulesExists) {
        Write-Host "    - Missing node_modules/" -ForegroundColor Gray
    }
    if (-not $cargoLockExists) {
        Write-Host "    - Missing Cargo.lock" -ForegroundColor Gray
    }
}

# 6. Directory structure check
Write-Host "[6/8] Directory Structure Check..." -ForegroundColor Yellow
$requiredDirs = @(
    "src/components",
    "src/stores",
    "src/types",
    "src-tauri/src/commands",
    "src-tauri/src/services",
    "scripts"
)

$optionalDirs = @(
    "scripts/constraints",           # Optional: Configuration directory (can be empty)
    "scripts/architecture-guardrails" # Optional: Future feature directory
)

$missingDirs = @()
foreach ($dir in $requiredDirs) {
    if (-not (Test-Path $dir)) {
        $missingDirs += $dir
    }
}

# Check optional directories (don't fail if missing, just warn)
$missingOptionalDirs = @()
foreach ($dir in $optionalDirs) {
    if (-not (Test-Path $dir)) {
        $missingOptionalDirs += $dir
    }
}

if ($missingDirs.Count -eq 0) {
    Write-Host "  [PASS] Directory structure complete" -ForegroundColor Green
    
    # Warn about missing optional directories
    if ($missingOptionalDirs.Count -gt 0) {
        Write-Host "  [INFO] Optional directories not present:" -ForegroundColor Gray
        foreach ($dir in $missingOptionalDirs) {
            Write-Host "    - $dir" -ForegroundColor Gray
        }
    }
} else {
    Write-Host "  [WARN] Missing directories:" -ForegroundColor Yellow
    foreach ($dir in $missingDirs) {
        Write-Host "    - $dir" -ForegroundColor Gray
    }
    $Issues += @{ Type = "Structure"; Severity = "Warning"; Message = "Incomplete directory structure" }
    $Score -= 5
}

# 7. Documentation consistency check (optional)
if ($DocCheck -or $All) {
    Write-Host "[7/8] Documentation Consistency Check..." -ForegroundColor Yellow
    try {
        & ./scripts/harness-doc-check.ps1 -Verbose:$Verbose | Out-Null
        if ($LASTEXITCODE -eq 0) {
            Write-Host "  [PASS] Documentation consistency check passed" -ForegroundColor Green
        } else {
            Write-Host "  [WARN] Documentation consistency check has warnings" -ForegroundColor Yellow
            $Issues += @{ Type = "Documentation"; Severity = "Warning"; Message = "Doc inconsistency" }
            $Score -= 5
        }
    } catch {
        Write-Host "  [WARN] Cannot execute documentation check" -ForegroundColor Yellow
        $Issues += @{ Type = "Documentation"; Severity = "Warning"; Message = "Check unavailable" }
    }
}

# 8. Dead code detection (optional)
if ($DeadCode -or $All) {
    Write-Host "[8/8] Dead Code Detection..." -ForegroundColor Yellow
    try {
        & ./scripts/harness-dead-code.ps1 -Verbose:$Verbose | Out-Null
        if ($LASTEXITCODE -eq 0) {
            Write-Host "  [PASS] Dead code detection passed" -ForegroundColor Green
        } else {
            Write-Host "  [WARN] Dead code detected" -ForegroundColor Yellow
            $Issues += @{ Type = "DeadCode"; Severity = "Warning"; Message = "Unused code found" }
            $Score -= 5
        }
    } catch {
        Write-Host "  [WARN] Cannot execute dead code detection" -ForegroundColor Yellow
        $Issues += @{ Type = "DeadCode"; Severity = "Warning"; Message = "Detection unavailable" }
    }
}

# Summary
$EndTime = Get-Date
$Duration = ($EndTime - $StartTime).TotalSeconds

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  Check Summary" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

if ($Score -ge 90) {
    Write-Host "  [EXCELLENT] Health Score: $Score/100" -ForegroundColor Green
    Write-Host "  Status: Excellent" -ForegroundColor Green
} elseif ($Score -ge 70) {
    Write-Host "  [GOOD] Health Score: $Score/100" -ForegroundColor Yellow
    Write-Host "  Status: Good, some improvements needed" -ForegroundColor Yellow
} else {
    Write-Host "  [NEEDS FIX] Health Score: $Score/100" -ForegroundColor Red
    Write-Host "  Status: Requires immediate attention" -ForegroundColor Red
}

Write-Host ""
Write-Host "  Duration: $([math]::Round($Duration, 2)) seconds" -ForegroundColor Cyan
Write-Host "  Issues Found: $($Issues.Count)" -ForegroundColor $(if ($Issues.Count -eq 0) { "Green" } else { "Red" })
Write-Host ""

if ($Issues.Count -gt 0) {
    Write-Host "Issue List:" -ForegroundColor Yellow
    foreach ($issue in $Issues) {
        $icon = if ($issue.Severity -eq "Error") { "[ERROR]" } else { "[WARN]" }
        Write-Host "  $icon [$($issue.Type)] $($issue.Message)" -ForegroundColor $(if ($issue.Severity -eq "Error") { "Red" } else { "Yellow" })
    }
    Write-Host ""
}

# Recommendations
if ($Score -lt 100) {
    Write-Host "Recommendations:" -ForegroundColor Blue
    if ($Score -lt 80) {
        Write-Host "  1. Run '.\scripts\fix-code-quality.ps1' to auto-fix code style issues" -ForegroundColor Gray
        Write-Host "  2. Run 'npm run format' to format code" -ForegroundColor Gray
        Write-Host "  3. Run 'npx tsc --noEmit' to see specific type errors" -ForegroundColor Gray
        Write-Host "  4. Run 'cargo check' in src-tauri/ to see Rust errors" -ForegroundColor Gray
    }
    if ($Score -lt 60) {
        Write-Host "  5. Consider running 'npm install' and 'cargo fetch' to reinstall dependencies" -ForegroundColor Gray
        Write-Host "  6. Check if scripts/ directory structure is complete" -ForegroundColor Gray
    }
    Write-Host ""
}

# JSON output for CI/CD
if ($Json) {
    $result = @{
        Score = $Score
        Duration = [math]::Round($Duration, 2)
        Issues = $Issues
        Timestamp = (Get-Date -Format "yyyy-MM-dd HH:mm:ss")
    }
    $result | ConvertTo-Json -Depth 3
}

# Exit code
if ($Score -ge 70) {
    exit 0
} else {
    exit 1
}
