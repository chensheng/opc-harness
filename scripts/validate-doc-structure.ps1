#!/usr/bin/env pwsh
# Harness Engineering - 文档结构验证
# 用途：验证文档结构的完整性和链接有效性

param(
    [switch]$Verbose,       # 详细输出
    [switch]$Json           # JSON 格式输出
)

$ErrorActionPreference = "Stop"
$StartTime = Get-Date
$Issues = @()
$Score = 100

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  Documentation Structure Validator" -ForegroundColor Cyan
Write-Host ""

# 1. 检查关键文档是否存在
Write-Host "[1/3] Checking Key Documents..." -ForegroundColor Yellow
$KeyDocuments = @(
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

$MissingDocs = @()
foreach ($doc in $KeyDocuments) {
    if (Test-Path $doc) {
        Write-Host "  ✅ $doc" -ForegroundColor Green
    } else {
        Write-Host "  ❌ $doc (MISSING)" -ForegroundColor Red
        $MissingDocs += $doc
        $Score -= 10
    }
}

# 2. 检查 index.md 文件是否包含目录列表
Write-Host ""
Write-Host "[2/3] Checking Index Files..." -ForegroundColor Yellow
$IndexFiles = @(
    "docs/design-docs/index.md",
    "docs/exec-plans/index.md",
    "docs/product-specs/index.md",
    "docs/references/index.md"
)

foreach ($indexFile in $IndexFiles) {
    if (Test-Path $indexFile) {
        $content = Get-Content $indexFile -Raw
        if ($content -match '\[.*\]\(.*\)') {
            Write-Host "  ✅ $indexFile (has links)" -ForegroundColor Green
        } else {
            Write-Host "  ⚠️  $indexFile (no links found)" -ForegroundColor Yellow
            $Score -= 5
        }
    }
}

# 3. 检查文档最后更新日期
Write-Host ""
Write-Host "[3/3] Checking Document Freshness..." -ForegroundColor Yellow
$MDDocuments = Get-ChildItem -Path "docs" -Recurse -Filter "*.md" | Where-Object { $_.Name -ne "node_modules" }
$OldDocs = @()

foreach ($file in $MDDocuments) {
    $content = Get-Content $file.FullName -Raw
    
    # 简单匹配日期格式 YYYY-MM-DD
    $dates = [regex]::Matches($content, '\d{4}-\d{2}-\d{2}')
    if ($dates.Count -gt 0) {
        # 使用最后一个日期
        $lastDate = $dates[-1].Value
        try {
            $lastUpdate = [datetime]::ParseExact($lastDate, 'yyyy-MM-dd', $null)
            $daysOld = (Get-Date) - $lastUpdate
            
            if ($daysOld.Days -gt 90) {
                Write-Host "  ⚠️  $($file.RelativePath) ($($daysOld.Days) days old)" -ForegroundColor Yellow
                $OldDocs += $file
                $Score -= 2
            } elseif ($Verbose) {
                Write-Host "  ✅ $($file.RelativePath) ($($daysOld.Days) days)" -ForegroundColor Green
            }
        } catch {
            if ($Verbose) {
                Write-Host "  ⚠️  $($file.RelativePath) (invalid date: $lastDate)" -ForegroundColor Yellow
            }
        }
    } else {
        Write-Host "  ⚠️  $($file.RelativePath) (no update date)" -ForegroundColor Yellow
        $OldDocs += $file
        $Score -= 1
    }
}

# Calculate final score
$Score = [Math]::Max(0, $Score)

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  Results" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "Documentation Score: $Score/100" -ForegroundColor $(if ($Score -ge 90) { "Green" } elseif ($Score -ge 70) { "Yellow" } else { "Red" })
Write-Host ""

if ($MissingDocs.Count -gt 0) {
    Write-Host "Missing Documents:" -ForegroundColor Red
    foreach ($doc in $MissingDocs) {
        Write-Host "  - $doc" -ForegroundColor Red
    }
    Write-Host ""
}

if ($OldDocs.Count -gt 0) {
    Write-Host "Outdated Documents (>90 days or no date):" -ForegroundColor Yellow
    foreach ($doc in $OldDocs) {
        Write-Host "  - $($doc.RelativePath)" -ForegroundColor Yellow
    }
    Write-Host ""
}

if ($Score -eq 100) {
    Write-Host "🎉 All documentation checks passed!" -ForegroundColor Green
} elseif ($Score -ge 90) {
    Write-Host "👍 Documentation is in good shape with minor issues." -ForegroundColor Green
} elseif ($Score -ge 70) {
    Write-Host "⚠️  Some documentation issues need attention." -ForegroundColor Yellow
} else {
    Write-Host "❌ Critical documentation issues found. Please review." -ForegroundColor Red
}

Write-Host ""
Write-Host "Elapsed Time: $((Get-Date) - $StartTime).TotalSeconds seconds" -ForegroundColor Gray

exit $(if ($Score -ge 70) { 0 } else { 1 })
