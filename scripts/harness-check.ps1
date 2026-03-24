#!/usr/bin/env pwsh
# Harness Engineering 架构健康检查脚本
# 用法：.\scripts\harness-check.ps1
# 版本：2.1 (简化版 - 默认全量检查)

param(
    [switch]$Verbose,      # 详细输出
    [switch]$Json          # JSON 格式输出
)

# =============================================
# 配置区域 - 集中管理所有配置
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
        "docs/MAINTENANCE.md",
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
    
    Timeouts = @{
        TSTests = 30  # seconds
    }
}

$Script:State = @{
    Score           = 100
    Issues          = @()
    OriginalLocation = Get-Location
    CargoAvailable  = $false
    NpmAvailable    = $false
}

# =============================================
# 工具函数区域 - 通用辅助函数
# =============================================

function Write-Header {
    param([string]$Text)
    Write-Host ""
    Write-Host "========================================" -ForegroundColor Cyan
    Write-Host "  $Text" -ForegroundColor Cyan
    Write-Host ""
}

function Write-CheckStart {
    param(
        [string]$Name,
        [string]$Index
    )
    Write-Host "[$Index] $Name..." -ForegroundColor Yellow
}

function Write-CheckResult {
    param(
        [ValidateSet("PASS", "FAIL", "WARN", "FIX")]
        [string]$Status,
        [string]$Message
    )
    
    $color = switch ($Status) {
        "PASS" { "Green" }
        "FAIL" { "Red" }
        "WARN" { "Yellow" }
        "FIX"  { "Blue" }
    }
    
    Write-Host "  [$Status] $Message" -ForegroundColor $color
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
# 检查函数区域 - 各项独立检查逻辑
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
            
            if ($Verbose) {
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
            
            if ($Verbose -or $Fix) {
                Write-Host $eslintResult -ForegroundColor Gray
            }
            
            if ($Fix) {
                Write-CheckResult -Status "FIX" -Message "Attempting auto-fix..."
                npm run lint:fix | Out-Null
                Write-CheckResult -Status "OK" -Message "Auto-fix completed"
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
            
            if ($Fix) {
                Write-CheckResult -Status "FIX" -Message "Auto-formatting code..."
                npm run format | Out-Null
                Write-CheckResult -Status "OK" -Message "Code formatted"
            }
        }
    } catch {
        Write-CheckResult -Status "WARN" -Message "Cannot execute Prettier check"
        Add-Issue -Type "Prettier" -Severity "Warning" -Message "Check tool unavailable"
    }
}

function Invoke-RustCompilationCheck {
    Write-CheckStart -Name "Rust Compilation Check" -Index "4/8"
    
    # Check if cargo is available
    $Script:State.CargoAvailable = Test-CommandAvailable -CommandName "cargo"
    
    if (-not $Script:State.CargoAvailable) {
        Write-CheckResult -Status "WARN" -Message "Cannot execute Cargo check (Rust may not be installed)"
        Add-Issue -Type "Rust" -Severity "Warning" -Message "Rust environment not ready"
        return
    }
    
    # Run cargo check
    Set-Location src-tauri
    try {
        $null = & cargo check
        
        if ($LASTEXITCODE -eq 0) {
            Write-CheckResult -Status "PASS" -Message "Rust compilation check passed"
        } else {
            Write-CheckResult -Status "FAIL" -Message "Rust compilation check failed"
            Add-Issue -Type "Rust" -Severity "Error" -Message "Compilation error" -ScorePenalty $Script:Config.ScoreWeights.Rust
            
            if ($Verbose) {
                Write-Host "Run 'cd src-tauri; cargo check' for details" -ForegroundColor Gray
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
    
    Write-Host "  Running Rust unit tests..." -ForegroundColor Gray
    
    try {
        $rustTestScript = Join-Path $PSScriptRoot "harness-rust-tests.ps1"
        
        if (-not (Test-Path $rustTestScript)) {
            Write-CheckResult -Status "FAIL" -Message "Rust test script not found: $rustTestScript"
            Add-Issue -Type "Rust Tests" -Severity "Error" -Message "Test script missing" -ScorePenalty $Script:Config.ScoreWeights.RustTests
            return
        }
        
        $testOutputFile = [System.IO.Path]::GetTempFileName()
        & powershell -ExecutionPolicy Bypass -File $rustTestScript -Verbose > $testOutputFile 2>&1
        $rustTestExitCode = $LASTEXITCODE
        
        $testOutput = Get-Content $testOutputFile -Raw -Encoding UTF8
        Remove-Item $testOutputFile -Force
        
        # Parse results
        if ($testOutput -match "\[PASS\] All (\d+) Rust tests passed") {
            $testCount = $matches[1]
            Write-CheckResult -Status "PASS" -Message "All $testCount Rust tests passed"
            
            if ($Verbose) {
                Write-Host $testOutput -ForegroundColor Gray
            }
        } elseif ($testOutput -match "\[FAIL\].*panic|\[ERROR\]") {
            Write-CheckResult -Status "FAIL" -Message "Rust tests encountered an error"
            Add-Issue -Type "Rust Tests" -Severity "Error" -Message "Test execution error" -ScorePenalty $Script:Config.ScoreWeights.RustTests
            
            if ($Verbose) {
                Write-Host $testOutput -ForegroundColor Gray
            }
        } elseif ($testOutput -match "\[FAIL\] Rust tests:.*failed") {
            Write-CheckResult -Status "FAIL" -Message "Some Rust tests failed"
            Add-Issue -Type "Rust Tests" -Severity "Error" -Message "Test failures" -ScorePenalty $Script:Config.ScoreWeights.RustTests
            
            if ($Verbose) {
                Write-Host $testOutput -ForegroundColor Gray
            }
        } elseif ($rustTestExitCode -ne 0) {
            Write-CheckResult -Status "FAIL" -Message "Rust test execution failed (exit code: $rustTestExitCode)"
            Add-Issue -Type "Rust Tests" -Severity "Error" -Message "Test execution failed" -ScorePenalty $Script:Config.ScoreWeights.RustTests
            
            if ($Verbose) {
                Write-Host $testOutput -ForegroundColor Gray
            }
        } else {
            Write-CheckResult -Status "WARN" -Message "Could not parse Rust test results"
            Add-Issue -Type "Rust Tests" -Severity "Warning" -Message "Unknown test result" -ScorePenalty 10
            
            if ($Verbose) {
                Write-Host $testOutput -ForegroundColor Gray
            }
        }
    } catch {
        Write-CheckResult -Status "FAIL" -Message "Rust test execution failed: $_"
        Add-Issue -Type "Rust Tests" -Severity "Error" -Message "Test execution exception" -ScorePenalty $Script:Config.ScoreWeights.RustTests
        
        if ($Verbose) {
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
    
    Write-Host "  Running TypeScript unit tests..." -ForegroundColor Gray
    
    try {
        $testJob = Start-Job -ScriptBlock {
            Set-Location $using:Script:State.OriginalLocation
            & npm run test:unit 2>&1
        }
        
        $waited = $testJob | Wait-Job -Timeout $Script:Config.Timeouts.TSTests
        
        if ($waited) {
            $testOutput = Receive-Job $testJob
            Stop-Job $testJob | Remove-Job -Force
            
            # Simple approach: check for the summary line pattern
            $hasPassed = $testOutput -match 'Test Files\s+\d+\s+passed'
            $hasFailed = $testOutput -match '\d+\s+failed|FAILURES'
            
            if ($hasPassed -and -not $hasFailed) {
                # Extract test file count if possible
                if ($testOutput -match 'Test Files\s+(\d+)\s+passed') {
                    $testFiles = $matches[1]
                    Write-CheckResult -Status "PASS" -Message "All TS tests passed ($testFiles files)"
                } else {
                    Write-CheckResult -Status "PASS" -Message "All TS tests passed"
                }
            } elseif ($hasFailed) {
                Write-CheckResult -Status "FAIL" -Message "Some TS tests failed"
                Add-Issue -Type "TS Tests" -Severity "Error" -Message "Test failures" -ScorePenalty $Script:Config.ScoreWeights.TSTests
                
                if ($Verbose) {
                    # Show summary only
                    $summaryLines = $testOutput -split "`n" | Where-Object { $_ -match "Test Files|Tests\s+\d+" }
                    if ($summaryLines.Count -gt 0) {
                        Write-Host ($summaryLines -join "`n") -ForegroundColor Gray
                    }
                }
            } else {
                # Fallback to warning if we can't determine the result
                Write-CheckResult -Status "WARN" -Message "Could not parse TS test results"
                Add-Issue -Type "TS Tests" -Severity "Warning" -Message "Unknown result" -ScorePenalty 10
                
                if ($Verbose) {
                    Write-Host "Raw output (last 5 lines):" -ForegroundColor Gray
                    ($testOutput -split "`n" | Select-Object -Last 5) | ForEach-Object { Write-Host $_ -ForegroundColor Gray }
                }
            }
        } else {
            Write-CheckResult -Status "FAIL" -Message "TS tests timed out (>$($Script:Config.Timeouts.TSTests)s)"
            Add-Issue -Type "TS Tests" -Severity "Error" -Message "Test timeout" -ScorePenalty 10
            
            Stop-Job $testJob | Remove-Job -Force
        }
    } catch {
        Write-CheckResult -Status "FAIL" -Message "TS test execution failed: $_"
        Add-Issue -Type "TS Tests" -Severity "Error" -Message "Test execution exception" -ScorePenalty $Script:Config.ScoreWeights.TSTests
        
        if ($Verbose) {
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
            Write-Host "  [$severity] Required file missing: $file" -ForegroundColor $(if ($severity -eq "Error") { "Red" } else { "Yellow" })
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
            Write-Host "  [WARN] Required directory missing: $dir" -ForegroundColor Yellow
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
    
    # Check key documents exist
    foreach ($doc in $Script:Config.KeyDocuments) {
        if (Test-Path $doc) {
            if ($Verbose) {
                Write-Host "  ✅ $doc" -ForegroundColor Green
            }
        } else {
            Write-Host "  ❌ $doc (MISSING)" -ForegroundColor Red
            $docIssues++
        }
    }
    
    # Check index files have links
    foreach ($indexFile in $Script:Config.IndexFiles) {
        if (Test-Path $indexFile) {
            $content = Get-Content $indexFile -Raw
            if ($content -match '\[.*\]\(.*\)') {
                if ($Verbose) {
                    Write-Host "  ✅ $indexFile (has links)" -ForegroundColor Green
                }
            } else {
                Write-Host "  ⚠️  $indexFile (no links found)" -ForegroundColor Yellow
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
# 结果汇总区域 - 输出最终报告
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
    }
    
    $endTime = Get-Date
    $duration = $endTime - $StartTime
    Write-Host "  Duration: $($duration.Minutes)m$($duration.Seconds)s" -ForegroundColor Gray
    Write-Host ""
    Write-Host "========================================" -ForegroundColor Cyan
}

# =============================================
# 主执行流程
# =============================================

try {
    Write-Header -Text "OPC-HARNESS Architecture Health Check v2.1"
    
    $startTime = Get-Date
    
    # Execute all checks in order (always run all 9 checks)
    $checks = @(
        { Invoke-TypeScriptCheck },
        { Invoke-ESLintCheck },
        { Invoke-PrettierCheck },
        { Invoke-RustCompilationCheck },
        { Invoke-RustTestsCheck },
        { Invoke-TSTestsCheck },
        { Invoke-DependencyCheck },
        { Invoke-DirectoryCheck },
        { Invoke-DocumentationCheck }
    )
    
    foreach ($check in $checks) {
        & $check
    }
    
    # Show summary
    Show-Summary
    
    # Exit with appropriate code
    if ($Script:State.Issues.Where({ $_.Severity -eq "Error" }).Count -gt 0) {
        exit 1
    }
    
} catch {
    Write-Host "[FATAL] Unexpected error: $_" -ForegroundColor Red
    if ($Verbose) {
        throw
    } else {
        exit 2
    }
} finally {
    # Cleanup: restore original location
    if ($Script:State.OriginalLocation) {
        Set-Location $Script:State.OriginalLocation
    }
}
