#!/usr/bin/env pwsh
# Harness Engineering Garbage Collection Script
# Purpose: Clean up outdated documents, dead code, and redundant configurations
# Usage: .\scripts\harness-gc.ps1

param(
    [switch]$DryRun,         # Dry run mode (don't actually delete)
    [switch]$Verbose,        # Verbose output
    [switch]$Force           # Force delete (no confirmation)
)

$ErrorActionPreference = "Continue"
$StartTime = Get-Date
$DeletedCount = 0
$SkippedCount = 0
$TotalSize = 0

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  OPC-HARNESS Garbage Collection" -ForegroundColor Cyan
Write-Host ""

if ($DryRun) {
    Write-Host "[INFO] DRY RUN MODE - No files will be deleted" -ForegroundColor Yellow
    Write-Host ""
}

# Helper function: Safe file removal
function Remove-SafeFile {
    param([string]$Path)
    
    if (-not (Test-Path $Path)) {
        return
    }
    
    $fileSize = (Get-Item $Path).Length
    $fileSizeKB = [math]::Round($fileSize / 1KB, 2)
    
    if ($DryRun) {
        Write-Host "  [DRY RUN] Would delete: $Path ($fileSizeKB KB)" -ForegroundColor Gray
        $DeletedCount++
    } else {
        if ($Force) {
            Remove-Item $Path -Force
            Write-Host "  [DELETED] $Path ($fileSizeKB KB)" -ForegroundColor Green
            $TotalSize += $fileSize
            $DeletedCount++
        } else {
            $confirm = Read-Host "  Confirm delete: $Path? (y/n)"
            if ($confirm -eq 'y' -or $confirm -eq 'Y') {
                Remove-Item $Path -Force
                Write-Host "  [DELETED] $Path ($fileSizeKB KB)" -ForegroundColor Green
                $TotalSize += $fileSize
                $DeletedCount++
            } else {
                Write-Host "  [SKIPPED] $Path" -ForegroundColor Yellow
                $SkippedCount++
            }
        }
    }
}

# 1. Clean up temporary files
Write-Host "[1/7] Cleaning temporary files..." -ForegroundColor Yellow
$tempPatterns = @(
    "*.tmp",
    "*.bak",
    "*.old",
    "*.log",
    "*~",
    ".DS_Store",
    "Thumbs.db"
)

foreach ($pattern in $tempPatterns) {
    $files = Get-ChildItem -Recurse -Filter $pattern -ErrorAction SilentlyContinue
    foreach ($file in $files) {
        Remove-SafeFile -Path $file.FullName
    }
}

# 2. Clean up Node.js build artifacts
Write-Host "[2/7] Cleaning Node.js build artifacts..." -ForegroundColor Yellow
$buildArtifacts = @(
    "dist",
    "build",
    ".vite",
    "tsconfig.tsbuildinfo"
)

foreach ($artifact in $buildArtifacts) {
    if (Test-Path $artifact) {
        if ($DryRun) {
            $size = (Get-ChildItem $artifact -Recurse | Measure-Object -Property Length -Sum).Sum
            Write-Host "  [DRY RUN] Would clean directory: $artifact ([math]::Round($size / 1MB, 2) MB)" -ForegroundColor Gray
            $DeletedCount++
        } else {
            if ($Force) {
                Remove-Item $artifact -Recurse -Force
                Write-Host "  [CLEANED] $artifact" -ForegroundColor Green
                $DeletedCount++
            } else {
                $confirm = Read-Host "  Confirm cleaning build directory $artifact? (y/n)"
                if ($confirm -eq 'y' -or $confirm -eq 'Y') {
                    Remove-Item $artifact -Recurse -Force
                    Write-Host "  [CLEANED] $artifact" -ForegroundColor Green
                    $DeletedCount++
                } else {
                    Write-Host "  [SKIPPED] $artifact" -ForegroundColor Yellow
                    $SkippedCount++
                }
            }
        }
    }
}

# 3. Clean up Rust build artifacts
Write-Host "[3/7] Cleaning Rust build artifacts..." -ForegroundColor Yellow
$rustArtifacts = @(
    "src-tauri\target"
)

foreach ($artifact in $rustArtifacts) {
    if (Test-Path $artifact) {
        if ($DryRun) {
            $size = (Get-ChildItem $artifact -Recurse -ErrorAction SilentlyContinue | 
                     Where-Object { -not $_.PSIsContainer } | 
                     Measure-Object -Property Length -Sum).Sum
            Write-Host "  [DRY RUN] Would clean directory: $artifact ([math]::Round($size / 1MB, 2) MB)" -ForegroundColor Gray
            $DeletedCount++
        } else {
            if ($Force) {
                Remove-Item $artifact -Recurse -Force
                Write-Host "  [CLEANED] $artifact" -ForegroundColor Green
                $DeletedCount++
            } else {
                $confirm = Read-Host "  Confirm cleaning Rust build directory $artifact? (y/n)"
                if ($confirm -eq 'y' -or $confirm -eq 'Y') {
                    Remove-Item $artifact -Recurse -Force
                    Write-Host "  [CLEANED] $artifact" -ForegroundColor Green
                    $DeletedCount++
                } else {
                    Write-Host "  [SKIPPED] $artifact" -ForegroundColor Yellow
                    $SkippedCount++
                }
            }
        }
    }
}

# 4. Scan for unused dependencies (simple check)
Write-Host "[4/7] Scanning for unused dependencies..." -ForegroundColor Yellow
try {
    $packageJson = Get-Content "package.json" -Raw | ConvertFrom-Json
    $allDeps = @()
    if ($packageJson.dependencies) {
        $allDeps += $packageJson.dependencies.PSObject.Properties.Name
    }
    if ($packageJson.devDependencies) {
        $allDeps += $packageJson.devDependencies.PSObject.Properties.Name
    }
    
    if (Test-Path "node_modules") {
        Write-Host "  [INFO] Found $($allDeps.Count) dependency packages" -ForegroundColor Cyan
        Write-Host "  [TIP] Use depcheck tool for deeper analysis" -ForegroundColor Gray
    }
} catch {
    Write-Host "  [WARN] Cannot analyze dependencies" -ForegroundColor Yellow
}

# 5. Check for outdated documents (>30 days old)
Write-Host "[5/7] Checking outdated documents..." -ForegroundColor Yellow
$cutoffDate = (Get-Date).AddDays(-30)
$docDirs = @(
    "scripts/context-engineering/execution-logs",
    "scripts/context-engineering/decision-records"
)

foreach ($dir in $docDirs) {
    if (Test-Path $dir) {
        $oldFiles = Get-ChildItem $dir -File | Where-Object {
            $_.LastWriteTime -lt $cutoffDate -and $_.Name -notlike "*.md"
        }
        
        foreach ($file in $oldFiles) {
            $daysOld = (New-TimeSpan -Start $file.LastWriteTime -End (Get-Date)).Days
            Write-Host "  [OUTDATED] $($file.Name) ($daysOld days old)" -ForegroundColor Yellow
            Remove-SafeFile -Path $file.FullName
        }
    }
}

# 6. Scan code comments (TODO/FIXME/HACK)
Write-Host "[6/7] Scanning code comment markers..." -ForegroundColor Yellow
$todoCount = 0
$fixmeCount = 0
$hackCount = 0

$sourceFiles = Get-ChildItem "src", "src-tauri/src" -Include *.ts,*.tsx,*.rs -Recurse -ErrorAction SilentlyContinue
foreach ($file in $sourceFiles) {
    $content = Get-Content $file.FullName -Raw
    $todoCount += ([regex]::Matches($content, 'TODO')).Count
    $fixmeCount += ([regex]::Matches($content, 'FIXME')).Count
    $hackCount += ([regex]::Matches($content, 'HACK')).Count
}

Write-Host "  TODO: $todoCount" -ForegroundColor Cyan
Write-Host "  FIXME: $fixmeCount" -ForegroundColor Cyan
Write-Host "  HACK: $hackCount" -ForegroundColor Yellow

if ($fixmeCount -gt 0 -or $hackCount -gt 0) {
    Write-Host "  [TIP] Prioritize fixing FIXME and HACK markers" -ForegroundColor Gray
}

# 7. Verify critical file integrity
Write-Host "[7/7] Verifying critical files..." -ForegroundColor Yellow
$criticalFiles = @(
    "package.json",
    "src-tauri/Cargo.toml",
    "src/App.tsx",
    "src-tauri/src/main.rs",
    "AGENTS.md"
)

$missingFiles = @()
foreach ($file in $criticalFiles) {
    if (-not (Test-Path $file)) {
        $missingFiles += $file
        Write-Host "  [MISSING] Critical file: $file" -ForegroundColor Red
    }
}

if ($missingFiles.Count -eq 0) {
    Write-Host "  [PASS] All critical files present" -ForegroundColor Green
}

# Summary Report
$EndTime = Get-Date
$Duration = ($EndTime - $StartTime).TotalSeconds
$TotalSizeMB = [math]::Round($TotalSize / 1MB, 2)

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  Garbage Collection Report" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

Write-Host "  Duration: $([math]::Round($Duration, 2)) seconds" -ForegroundColor Cyan
Write-Host "  Files Deleted: $DeletedCount" -ForegroundColor $(if ($DeletedCount -eq 0) { "Green" } else { "Yellow" })
Write-Host "  Files Skipped: $SkippedCount" -ForegroundColor Yellow
Write-Host "  Space Freed: $TotalSizeMB MB" -ForegroundColor Green
Write-Host ""

if ($DryRun) {
    Write-Host "[INFO] Remove --DryRun parameter to actually delete files" -ForegroundColor Yellow
} else {
    Write-Host "[SUCCESS] Garbage collection complete!" -ForegroundColor Green
}

Write-Host ""
Write-Host "Recommendations:" -ForegroundColor Blue
Write-Host "  1. Run this script regularly to keep project clean" -ForegroundColor Gray
Write-Host "  2. Use '.\scripts\harness-check.ps1' to verify architecture health" -ForegroundColor Gray
Write-Host "  3. Fix FIXME and HACK markers promptly" -ForegroundColor Gray
Write-Host ""


