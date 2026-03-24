#!/usr/bin/env pwsh
# Harness Engineering 架构健康检查脚本
# 用法：.\scripts\harness-check.ps1

param(
    [switch]$Fix,          # 自动修复问题
    [switch]$Verbose,      # 详细输出
    [switch]$Json,         # JSON 格式输出
    [switch]$NoDocCheck,   # 跳过文档一致性检查（默认执行）
    [switch]$NoDeadCode,   # 跳过死代码检测（默认执行）
    [switch]$Quick         # 快速模式（仅核心 8 项检查）
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
Write-Host "[4/8] Rust Compilation Check..." -ForegroundColor Yellow
$originalLocation = Get-Location
Set-Location src-tauri

# Check if cargo is available
$cargoAvailable = $false
try {
    $null = Get-Command cargo -ErrorAction Stop
    $cargoAvailable = $true
} catch {
    # Cargo not found
}

if ($cargoAvailable) {
    # Run cargo check, suppress output but capture exit code
    $null = & cargo check
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "  [PASS] Rust compilation check passed" -ForegroundColor Green
    } else {
        Write-Host "  [FAIL] Rust compilation check failed" -ForegroundColor Red
        $Issues += @{ Type = "Rust"; Severity = "Error"; Message = "Compilation error" }
        $Score -= 25
        if ($Verbose) {
            Write-Host "Run 'cd src-tauri; cargo check' for details" -ForegroundColor Gray
        }
    }
} else {
    Write-Host "  [WARN] Cannot execute Cargo check (Rust may not be installed)" -ForegroundColor Yellow
    $Issues += @{ Type = "Rust"; Severity = "Warning"; Message = "Rust environment not ready" }
}

Set-Location $originalLocation

# 5. Rust Unit Tests Check
Write-Host "[5/8] Rust Unit Tests Check..." -ForegroundColor Yellow

if ($cargoAvailable) {
    Write-Host "  Running Rust unit tests..." -ForegroundColor Gray
    
    try {
        # Use the dedicated Rust test script
        $rustTestScript = Join-Path $PSScriptRoot "harness-rust-tests.ps1"
        
        if (-not (Test-Path $rustTestScript)) {
            Write-Host "  [ERROR] Rust test script not found: $rustTestScript" -ForegroundColor Red
            $Issues += @{ Type = "Rust Tests"; Severity = "Error"; Message = "Test script missing" }
            $Score -= 20
        } else {
            # Execute the Rust test script and capture output
            $testOutputFile = [System.IO.Path]::GetTempFileName()
            
            & powershell -ExecutionPolicy Bypass -File $rustTestScript -Verbose > $testOutputFile 2>&1
            $rustTestExitCode = $LASTEXITCODE
            
            # Read the output
            $testOutput = Get-Content $testOutputFile -Raw -Encoding UTF8
            Remove-Item $testOutputFile -Force
            
            # Parse results
            if ($testOutput -match "\[PASS\] All (\d+) Rust tests passed") {
                $testCount = $matches[1]
                Write-Host "  [PASS] All $testCount Rust tests passed" -ForegroundColor Green
                
                if ($Verbose) {
                    Write-Host $testOutput -ForegroundColor Gray
                }
            } elseif ($testOutput -match "\[FAIL\].*panic|\[ERROR\]") {
                Write-Host "  [FAIL] Rust tests encountered an error" -ForegroundColor Red
                $Issues += @{ Type = "Rust Tests"; Severity = "Error"; Message = "Test execution error" }
                $Score -= 20
                
                if ($Verbose) {
                    Write-Host $testOutput -ForegroundColor Gray
                }
            } elseif ($testOutput -match "\[FAIL\] Rust tests:.*failed") {
                Write-Host "  [FAIL] Some Rust tests failed" -ForegroundColor Red
                $Issues += @{ Type = "Rust Tests"; Severity = "Error"; Message = "Test failures" }
                $Score -= 20
                
                if ($Verbose) {
                    Write-Host $testOutput -ForegroundColor Gray
                }
            } elseif ($rustTestExitCode -ne 0) {
                Write-Host "  [ERROR] Rust test execution failed (exit code: $rustTestExitCode)" -ForegroundColor Red
                $Issues += @{ Type = "Rust Tests"; Severity = "Error"; Message = "Test execution failed" }
                $Score -= 20
                
                if ($Verbose) {
                    Write-Host $testOutput -ForegroundColor Gray
                }
            } else {
                Write-Host "  [WARN] Could not parse Rust test results" -ForegroundColor Yellow
                $Issues += @{ Type = "Rust Tests"; Severity = "Warning"; Message = "Unknown test result" }
                $Score -= 10
                
                if ($Verbose) {
                    Write-Host $testOutput -ForegroundColor Gray
                }
            }
        }
    } catch {
        Write-Host "  [ERROR] Rust test execution failed: $_" -ForegroundColor Red
        $Issues += @{ Type = "Rust Tests"; Severity = "Error"; Message = "Test execution exception" }
        $Score -= 20
        
        if ($Verbose) {
            Write-Host "Exception details: $_" -ForegroundColor Red
        }
    }
} else {
    Write-Host "  [WARN] Cannot execute Rust tests (Cargo not available)" -ForegroundColor Yellow
    $Issues += @{ Type = "Rust Tests"; Severity = "Warning"; Message = "Rust environment not ready" }
}

# 6. TypeScript Unit Tests Check
Write-Host "[6/8] TypeScript Unit Tests Check..." -ForegroundColor Yellow

# Check if npm and node_modules are available
$npmAvailable = $false
try {
    $null = Get-Command npm -ErrorAction Stop
    $npmAvailable = $true
} catch {
    # Npm not found
}

if ($npmAvailable -and (Test-Path "node_modules")) {
    # Run npm test:unit and capture output with timeout
    Write-Host "  Running TypeScript unit tests..." -ForegroundColor Gray
    
    try {
        # Use timeout to prevent hanging (30 seconds max)
        $testJob = Start-Job -ScriptBlock {
            Set-Location $using:originalLocation
            & npm run test:unit 2>&1
        }
        
        # Wait for job with timeout
        $waited = $testJob | Wait-Job -Timeout 30
        
        if ($waited) {
            $testOutput = Receive-Job $testJob
            Stop-Job $testJob | Remove-Job -Force
            
            # Parse test results
            if ($testOutput -match "Test Files\s+(\d+) passed") {
                $testFiles = $matches[1]
                Write-Host "  [PASS] All TS tests passed ($testFiles files)" -ForegroundColor Green
            } elseif ($testOutput -match "FAILURES") {
                Write-Host "  [FAIL] Some TS tests failed" -ForegroundColor Red
                $Issues += @{ Type = "TS Tests"; Severity = "Error"; Message = "Test failures" }
                $Score -= 20
                
                if ($Verbose) {
                    Write-Host $testOutput -ForegroundColor Gray
                }
            } else {
                Write-Host "  [WARN] Could not parse TS test results" -ForegroundColor Yellow
                $Issues += @{ Type = "TS Tests"; Severity = "Warning"; Message = "Unknown result" }
                $Score -= 10
                
                if ($Verbose) {
                    Write-Host $testOutput -ForegroundColor Gray
                }
            }
        } else {
            Write-Host "  [ERROR] TS tests timed out (>30s)" -ForegroundColor Red
            $Issues += @{ Type = "TS Tests"; Severity = "Error"; Message = "Test timeout" }
            $Score -= 10
            
            Stop-Job $testJob | Remove-Job -Force
        }
    } catch {
        Write-Host "  [ERROR] TS test execution failed: $_" -ForegroundColor Red
        $Issues += @{ Type = "TS Tests"; Severity = "Error"; Message = "Test execution failed" }
        $Score -= 20
        
        if ($Verbose) {
            Write-Host "Exception details: $_" -ForegroundColor Red
        }
    }
} else {
    Write-Host "  [WARN] Cannot execute TS tests (npm/node_modules not available)" -ForegroundColor Yellow
    $Issues += @{ Type = "TS Tests"; Severity = "Warning"; Message = "TS environment not ready" }
}

# 7. Dependency Integrity Check
Write-Host "[7/8] Dependency Integrity Check..." -ForegroundColor Yellow

$depIssues = 0

# Check package-lock.json exists
if (-not (Test-Path "package-lock.json")) {
    Write-Host "  [WARN] package-lock.json missing" -ForegroundColor Yellow
    $depIssues++
}

# Check Cargo.lock exists
if (-not (Test-Path "src-tauri\Cargo.lock")) {
    Write-Host "  [WARN] Cargo.lock missing" -ForegroundColor Yellow
    $depIssues++
}

# Check package.json exists
if (-not (Test-Path "package.json")) {
    Write-Host "  [ERROR] package.json missing" -ForegroundColor Red
    $depIssues++
}

# Check Cargo.toml exists
if (-not (Test-Path "src-tauri\Cargo.toml")) {
    Write-Host "  [ERROR] Cargo.toml missing" -ForegroundColor Red
    $depIssues++
}

if ($depIssues -eq 0) {
    Write-Host "  [PASS] All dependencies present" -ForegroundColor Green
} else {
    Write-Host "  [INFO] Found $depIssues dependency issue(s)" -ForegroundColor Yellow
    $Issues += @{ Type = "Dependencies"; Severity = "Warning"; Message = "$depIssues missing file(s)" }
    $Score -= 5
}

# 8. Directory Structure Check
Write-Host "[8/8] Directory Structure Check..." -ForegroundColor Yellow

$requiredDirs = @(
    "src",
    "src-tauri",
    "scripts",
    "docs"
)

$dirIssues = 0
foreach ($dir in $requiredDirs) {
    if (-not (Test-Path $dir)) {
        Write-Host "  [WARN] Required directory missing: $dir" -ForegroundColor Yellow
        $dirIssues++
    }
}

if ($dirIssues -eq 0) {
    Write-Host "  [PASS] Directory structure valid" -ForegroundColor Green
} else {
    Write-Host "  [INFO] Found $dirIssues directory issue(s)" -ForegroundColor Yellow
    $Issues += @{ Type = "Directory"; Severity = "Warning"; Message = "$dirIssues missing dir(s)" }
    $Score -= 5
}

Set-Location $originalLocation
