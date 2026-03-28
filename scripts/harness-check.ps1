#!/usr/bin/env pwsh
# Harness Engineering 架构健康检查脚本
# 用法：.\scripts\harness-check.ps1
# 版本：2.2 (优化日志输出 - 直接在命令行显示，无日志文件)

param(
    [switch]$Verbose,      # 详细输出
    [switch]$Json,         # JSON 格式输出
    [switch]$Silent        # 静默模式（仅显示摘要）
)

# =============================================
# 配置区域
# =============================================

$Script:Config = @{
    ScoreWeights = @{
        TypeScript      = 20
        ESLint          = 15
        Prettier        = 10
        Rust            = 25
        RustTests       = 20
        TSTests         = 20
        Dependencies    = 5
        Directory       = 5
        Documentation   = 10
    }
    
    RequiredDirs = @(
        "src",
        "src-tauri",
        "scripts",
        "docs"
    )
    
    RequiredFiles = @(
        "package.json",
        "package-lock.json",
        "src-tauri/Cargo.toml",
        "src-tauri/Cargo.lock"
    )
    
    KeyDocuments = @(
        "AGENTS.md",
        "README.md",
        "ARCHITECTURE.md",
        "src/AGENTS.md",
        "src-tauri/AGENTS.md",
        "docs/README.md",
        "docs/design-docs/index.md",
        "docs/exec-plans/index.md",
        "docs/product-specs/index.md",
        "docs/references/index.md"
    )
    
    IndexFiles = @(
        "docs/design-docs/index.md",
        "docs/exec-plans/index.md",
        "docs/product-specs/index.md",
        "docs/references/index.md"
    )
}

$Script:State = @{
    Score           = 100
    Issues          = @()
    OriginalLocation = Get-Location
    CargoAvailable  = $false
    NpmAvailable    = $false
    StartTime       = $null
}

# =============================================
# 工具函数
# =============================================

function Write-Header {
    if (-not $Silent) {
        Write-Host ""
        Write-Host "========================================" -ForegroundColor Cyan
        Write-Host "  Harness Engineering Health Check" -ForegroundColor Cyan
        Write-Host ""
    }
}

function Write-CheckStart {
    param([string]$Name, [string]$Index)
    if (-not $Silent) {
        Write-Host "[$Index] $Name..." -ForegroundColor Yellow
    }
}

function Write-CheckResult {
    param(
        [ValidateSet("PASS", "FAIL", "WARN", "FIX")]
        [string]$Status,
        [string]$Message
    )
    
    if (-not $Silent) {
        $color = switch ($Status) {
            "PASS" { "Green" }
            "FAIL" { "Red" }
            "WARN" { "Yellow" }
            "FIX"  { "Blue" }
        }
        Write-Host "  [$Status] $Message" -ForegroundColor $color
    }
}

function Add-Issue {
    param(
        [string]$Type,
        [ValidateSet("Error", "Warning")]
        [string]$Severity,
        [string]$Message,
        [int]$ScorePenalty = 0
    )
    
    $Script:State.Issues += @{
        Type     = $Type
        Severity = $Severity
        Message  = $Message
    }
    
    if ($ScorePenalty -gt 0) {
        $Script:State.Score -= $ScorePenalty
    }
}

function Test-CommandAvailable {
    param([string]$CommandName)
    try {
        $null = Get-Command $CommandName -ErrorAction Stop
        return $true
    } catch {
        return $false
    }
}

# =============================================
# 检查函数
# =============================================

function Invoke-TypeScriptCheck {
    Write-CheckStart -Name "TypeScript Type Checking" -Index "1/8"
    
    try {
        $tscResult = npx tsc --noEmit 2>&1
        if ($LASTEXITCODE -eq 0) {
            Write-CheckResult -Status "PASS" -Message "TypeScript type checking passed"
        } else {
            Write-CheckResult -Status "FAIL" -Message "TypeScript type checking failed"
            Add-Issue -Type "TypeScript" -Severity "Error" -Message "Type check failed" -ScorePenalty $Script:Config.ScoreWeights.TypeScript
            
            if ($Verbose -and -not $Silent) {
                Write-Host $tscResult -ForegroundColor Gray
            }
        }
    } catch {
        Write-CheckResult -Status "WARN" -Message "Cannot execute TypeScript check"
        Add-Issue -Type "TypeScript" -Severity "Warning" -Message "Check tool unavailable"
    }
}

function Invoke-ESLintCheck {
    Write-CheckStart -Name "ESLint Code Quality Check" -Index "2/8"
    
    try {
        $eslintResult = npm run lint 2>&1
        if ($LASTEXITCODE -eq 0) {
            Write-CheckResult -Status "PASS" -Message "ESLint check passed"
        } else {
            Write-CheckResult -Status "FAIL" -Message "ESLint check has warnings/errors"
            Add-Issue -Type "ESLint" -Severity "Error" -Message "Code style violation" -ScorePenalty $Script:Config.ScoreWeights.ESLint
            
            if ($Verbose -and -not $Silent) {
                Write-Host $eslintResult -ForegroundColor Gray
            }
        }
    } catch {
        Write-CheckResult -Status "WARN" -Message "Cannot execute ESLint check"
        Add-Issue -Type "ESLint" -Severity "Warning" -Message "Check tool unavailable"
    }
}

function Invoke-PrettierCheck {
    Write-CheckStart -Name "Prettier Formatting Check" -Index "3/8"
    
    try {
        $prettierResult = npm run format:check 2>&1
        if ($LASTEXITCODE -eq 0) {
            Write-CheckResult -Status "PASS" -Message "Prettier formatting passed"
        } else {
            Write-CheckResult -Status "FAIL" -Message "Prettier formatting failed"
            Add-Issue -Type "Prettier" -Severity "Error" -Message "Code format not standard" -ScorePenalty $Script:Config.ScoreWeights.Prettier
            
            if ($Verbose -and -not $Silent) {
                Write-Host $prettierResult -ForegroundColor Gray
            }
        }
    } catch {
        Write-CheckResult -Status "WARN" -Message "Cannot execute Prettier check"
        Add-Issue -Type "Prettier" -Severity "Warning" -Message "Check tool unavailable"
    }
}

function Invoke-RustCompilationCheck {
    Write-CheckStart -Name "Rust Compilation Check" -Index "4/8"
    
    $Script:State.CargoAvailable = Test-CommandAvailable -CommandName "cargo"
    
    if (-not $Script:State.CargoAvailable) {
        Write-CheckResult -Status "WARN" -Message "Cannot execute Cargo check (Rust may not be installed)"
        Add-Issue -Type "Rust" -Severity "Warning" -Message "Rust environment not ready"
        return
    }
    
    Set-Location src-tauri
    try {
        $cargoOutput = cargo check 2>&1
        
        if ($LASTEXITCODE -eq 0) {
            Write-CheckResult -Status "PASS" -Message "Rust compilation check passed"
        } else {
            Write-CheckResult -Status "FAIL" -Message "Rust compilation check failed"
            Add-Issue -Type "Rust" -Severity "Error" -Message "Compilation error" -ScorePenalty $Script:Config.ScoreWeights.Rust
            
            if ($Verbose -and -not $Silent) {
                Write-Host $cargoOutput -ForegroundColor Gray
            }
        }
    } finally {
        Set-Location $Script:State.OriginalLocation
    }
}

function Invoke-RustTestsCheck {
    Write-CheckStart -Name "Rust Unit Tests Check" -Index "5/8"
    
    if (-not $Script:State.CargoAvailable) {
        Write-CheckResult -Status "WARN" -Message "Cannot execute Rust tests (Cargo not available)"
        Add-Issue -Type "Rust Tests" -Severity "Warning" -Message "Rust environment not ready"
        return
    }
    
    if (-not $Silent) {
        Write-Host "  Running Rust unit tests..." -ForegroundColor Gray
    }
    
    try {
        $rustTestScript = Join-Path $PSScriptRoot "harness-rust-tests.ps1"
        
        if (-not (Test-Path $rustTestScript)) {
            Write-CheckResult -Status "FAIL" -Message "Rust test script not found"
            Add-Issue -Type "Rust Tests" -Severity "Error" -Message "Test script missing" -ScorePenalty $Script:Config.ScoreWeights.RustTests
            return
        }
        
        $testOutputFile = [System.IO.Path]::GetTempFileName()
        & powershell -ExecutionPolicy Bypass -File $rustTestScript -Verbose > $testOutputFile 2>&1
        $rustTestExitCode = $LASTEXITCODE
        
        $testOutput = Get-Content $testOutputFile -Raw -Encoding UTF8
        Remove-Item $testOutputFile -Force
        
        if ($testOutput -match "\[PASS\] All (\d+) Rust tests passed") {
            $testCount = $matches[1]
            Write-CheckResult -Status "PASS" -Message "All $testCount Rust tests passed"
            
            if ($Verbose -and -not $Silent) {
                Write-Host $testOutput -ForegroundColor Gray
            }
        } elseif ($testOutput -match "\[FAIL\].*panic|\[ERROR\]") {
            Write-CheckResult -Status "FAIL" -Message "Rust tests encountered an error"
            Add-Issue -Type "Rust Tests" -Severity "Error" -Message "Test execution error" -ScorePenalty $Script:Config.ScoreWeights.RustTests
            
            if ($Verbose -and -not $Silent) {
                Write-Host $testOutput -ForegroundColor Gray
            }
        } elseif ($testOutput -match "\[FAIL\] Rust tests:.*failed") {
            Write-CheckResult -Status "FAIL" -Message "Some Rust tests failed"
            Add-Issue -Type "Rust Tests" -Severity "Error" -Message "Test failures" -ScorePenalty $Script:Config.ScoreWeights.RustTests
            
            if ($Verbose -and -not $Silent) {
                Write-Host $testOutput -ForegroundColor Gray
            }
        } elseif ($rustTestExitCode -ne 0) {
            Write-CheckResult -Status "FAIL" -Message "Rust test execution failed (exit code: $rustTestExitCode)"
            Add-Issue -Type "Rust Tests" -Severity "Error" -Message "Test execution failed" -ScorePenalty $Script:Config.ScoreWeights.RustTests
            
            if ($Verbose -and -not $Silent) {
                Write-Host $testOutput -ForegroundColor Gray
            }
        } else {
            Write-CheckResult -Status "WARN" -Message "Could not parse Rust test results"
            Add-Issue -Type "Rust Tests" -Severity "Warning" -Message "Unknown test result" -ScorePenalty 10
            
            if ($Verbose -and -not $Silent) {
                Write-Host $testOutput -ForegroundColor Gray
            }
        }
    } catch {
        Write-CheckResult -Status "FAIL" -Message "Rust test execution failed: $_"
        Add-Issue -Type "Rust Tests" -Severity "Error" -Message "Test execution exception" -ScorePenalty $Script:Config.ScoreWeights.RustTests
        
        if ($Verbose -and -not $Silent) {
            Write-Host "Exception details: $_" -ForegroundColor Red
        }
    }
}

function Invoke-TSTestsCheck {
    Write-CheckStart -Name "TypeScript Unit Tests Check" -Index "6/8"
    
    $Script:State.NpmAvailable = Test-CommandAvailable -CommandName "npm"
    
    if (-not $Script:State.NpmAvailable -or -not (Test-Path "node_modules")) {
        Write-CheckResult -Status "WARN" -Message "Cannot execute TS tests (npm/node_modules not available)"
        Add-Issue -Type "TS Tests" -Severity "Warning" -Message "TS environment not ready"
        return
    }
    
    if (-not $Silent) {
        Write-Host "  Running TypeScript unit tests..." -ForegroundColor Gray
    }
    
    try {
        $tsTestScript = Join-Path $PSScriptRoot "harness-ts-tests.ps1"
        
        if (-not (Test-Path $tsTestScript)) {
            Write-CheckResult -Status "FAIL" -Message "TS test script not found"
            Add-Issue -Type "TS Tests" -Severity "Error" -Message "Test script missing" -ScorePenalty $Script:Config.ScoreWeights.TSTests
            return
        }
        
        $testOutputFile = [System.IO.Path]::GetTempFileName()
        & powershell -ExecutionPolicy Bypass -File $tsTestScript -Verbose > $testOutputFile 2>&1
        $exitCode = $LASTEXITCODE
        
        $testOutput = Get-Content $testOutputFile -Raw -Encoding UTF8
        Remove-Item $testOutputFile -Force
        
        if ($exitCode -eq 0) {
            if ($testOutput -match "\[PASS\].*\((\d+)\s+files.*(\d+)\s+tests\)") {
                $testFiles = $matches[1]
                $totalTests = $matches[2]
                Write-CheckResult -Status "PASS" -Message "All TS tests passed ($testFiles files, $totalTests tests)"
            } elseif ($testOutput -match "\[PASS\].*\((\d+)\s+files\)") {
                $testFiles = $matches[1]
                Write-CheckResult -Status "PASS" -Message "All TS tests passed ($testFiles files)"
            } else {
                Write-CheckResult -Status "PASS" -Message "TS tests completed"
            }
            
            if ($Verbose -and -not $Silent) {
                Write-Host $testOutput -ForegroundColor Gray
            }
        } else {
            Write-CheckResult -Status "FAIL" -Message "TS test execution failed (exit code: $exitCode)"
            Add-Issue -Type "TS Tests" -Severity "Error" -Message "Test execution failed" -ScorePenalty $Script:Config.ScoreWeights.TSTests
            
            if ($Verbose -and -not $Silent) {
                $summaryLines = $testOutput -split "`n" | Where-Object { $_ -match "\[FAIL\]|\[PASS\]|\[WARN\]|Duration" }
                Write-Host ($summaryLines -join "`n") -ForegroundColor Gray
            }
        }
    } catch {
        Write-CheckResult -Status "FAIL" -Message "TS test execution failed: $_"
        Add-Issue -Type "TS Tests" -Severity "Error" -Message "Test execution exception" -ScorePenalty $Script:Config.ScoreWeights.TSTests
        
        if ($Verbose -and -not $Silent) {
            Write-Host "Exception details: $_" -ForegroundColor Red
        }
    }
}

function Invoke-DependencyCheck {
    Write-CheckStart -Name "Dependency Integrity Check" -Index "7/8"
    
    $depIssues = 0
    
    foreach ($file in $Script:Config.RequiredFiles) {
        if (-not (Test-Path $file)) {
            $severity = if ($file -like "*.json") { "Error" } else { "Warning" }
            if (-not $Silent) {
                Write-Host "  [$severity] Required file missing: $file" -ForegroundColor $(if ($severity -eq "Error") { "Red" } else { "Yellow" })
            }
            $depIssues++
        }
    }
    
    if ($depIssues -eq 0) {
        Write-CheckResult -Status "PASS" -Message "All dependencies present"
    } else {
        Write-CheckResult -Status "INFO" -Message "Found $depIssues dependency issue(s)"
        Add-Issue -Type "Dependencies" -Severity "Warning" -Message "$depIssues missing file(s)" -ScorePenalty $Script:Config.ScoreWeights.Dependencies
    }
}

function Invoke-DirectoryCheck {
    Write-CheckStart -Name "Directory Structure Check" -Index "8/9"
    
    $dirIssues = 0
    foreach ($dir in $Script:Config.RequiredDirs) {
        if (-not (Test-Path $dir)) {
            if (-not $Silent) {
                Write-Host "  [WARN] Required directory missing: $dir" -ForegroundColor Yellow
            }
            $dirIssues++
        }
    }
    
    if ($dirIssues -eq 0) {
        Write-CheckResult -Status "PASS" -Message "Directory structure valid"
    } else {
        Write-CheckResult -Status "INFO" -Message "Found $dirIssues directory issue(s)"
        Add-Issue -Type "Directory" -Severity "Warning" -Message "$dirIssues missing dir(s)" -ScorePenalty $Script:Config.ScoreWeights.Directory
    }
}

function Invoke-DocumentationCheck {
    Write-CheckStart -Name "Documentation Structure Check" -Index "9/9"
    
    $docIssues = 0
    
    foreach ($doc in $Script:Config.KeyDocuments) {
        if (Test-Path $doc) {
            if ($Verbose -and -not $Silent) {
                Write-Host "  ✅ $doc" -ForegroundColor Green
            }
        } else {
            if (-not $Silent) {
                Write-Host "  ❌ $doc (MISSING)" -ForegroundColor Red
            }
            $docIssues++
        }
    }
    
    foreach ($indexFile in $Script:Config.IndexFiles) {
        if (Test-Path $indexFile) {
            $content = Get-Content $indexFile -Raw
            if ($content -match '\[.*\]\(.*\)') {
                if ($Verbose -and -not $Silent) {
                    Write-Host "  ✅ $indexFile (has links)" -ForegroundColor Green
                }
            } else {
                if (-not $Silent) {
                    Write-Host "  ⚠️  $indexFile (no links found)" -ForegroundColor Yellow
                }
                $docIssues++
            }
        }
    }
    
    if ($docIssues -eq 0) {
        Write-CheckResult -Status "PASS" -Message "Documentation structure valid"
    } else {
        Write-CheckResult -Status "FAIL" -Message "Found $docIssues documentation issue(s)"
        Add-Issue -Type "Documentation" -Severity "Error" -Message "$docIssues document issue(s)" -ScorePenalty $Script:Config.ScoreWeights.Documentation
    }
}

# =============================================
# 结果汇总
# =============================================

function Show-Summary {
    Write-Host ""
    Write-Host "========================================" -ForegroundColor Cyan
    Write-Host "  Health Check Summary" -ForegroundColor Cyan
    Write-Host ""
    
    $finalScore = $Script:State.Score
    $scoreColor = if ($finalScore -ge 80) { "Green" } elseif ($finalScore -ge 60) { "Yellow" } else { "Red" }
    
    Write-Host "  Overall Score: $finalScore / 100" -ForegroundColor $scoreColor
    Write-Host "  Total Issues: $($Script:State.Issues.Count)" -ForegroundColor $(if ($Script:State.Issues.Count -gt 0) { "Yellow" } else { "Green" })
    Write-Host ""
    
    if ($Script:State.Issues.Count -gt 0) {
        Write-Host "  Issues Breakdown:" -ForegroundColor Yellow
        foreach ($issue in $Script:State.Issues) {
            $severityColor = if ($issue.Severity -eq "Error") { "Red" } else { "Yellow" }
            Write-Host "    - [$($issue.Severity)] $($issue.Type): $($issue.Message)" -ForegroundColor $severityColor
        }
        Write-Host ""
    } else {
        Write-Host '  Status: All checks passed!' -ForegroundColor Green
        Write-Host ''
    }
    
    $duration = (Get-Date) - $Script:State.StartTime
    Write-Host ('  Duration: {0}m{1}s' -f [int]$duration.TotalMinutes, $duration.Seconds) -ForegroundColor Gray
    Write-Host ''
    Write-Host '========================================' -ForegroundColor Cyan
}

# =============================================
# 主执行流程
# =============================================

Write-Header
$Script:State.StartTime = Get-Date

Invoke-TypeScriptCheck       # 1/8
Invoke-ESLintCheck           # 2/8
Invoke-PrettierCheck         # 3/8
Invoke-RustCompilationCheck  # 4/8
Invoke-RustTestsCheck        # 5/8
Invoke-TSTestsCheck          # 6/8
Invoke-DependencyCheck       # 7/8
Invoke-DirectoryCheck        # 8/9
Invoke-DocumentationCheck    # 9/9

Show-Summary
