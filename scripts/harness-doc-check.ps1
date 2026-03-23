#!/usr/bin/env pwsh
# Harness Engineering - 文档一致性检查
# 用途：验证代码与文档的一致性，防止"注释漂移"

param(
    [switch]$Fix,           # 自动修复（如果可能）
    [switch]$Verbose,       # 详细输出
    [switch]$Json           # JSON 格式输出
)

$ErrorActionPreference = "Stop"
$StartTime = Get-Date
$Issues = @()
$Score = 100

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  Documentation Consistency Check" -ForegroundColor Cyan
Write-Host ""

# 1. 检查 AGENTS.md 中的链接是否有效
Write-Host "[1/4] Checking AGENTS.md Links..." -ForegroundColor Yellow
$agentsFiles = @(
    "AGENTS.md",
    "src/AGENTS.md",
    "src-tauri/AGENTS.md"
)

foreach ($agentsFile in $agentsFiles) {
    if (Test-Path $agentsFile) {
        $content = Get-Content $agentsFile -Raw
        $markdownLinks = [regex]::Matches($content, '\[([^\]]+)\]\(([^\)]+)\)')
        
        foreach ($match in $markdownLinks) {
            $linkPath = $match.Groups[2].Value
            
            # 跳过外部链接
            if ($linkPath -match '^https?://') {
                continue
            }
            
            # 跳过锚点链接
            if ($linkPath.StartsWith('#')) {
                continue
            }

            # 检查文件是否存在
            $fullPath = Join-Path (Split-Path $agentsFile) $linkPath
            if (-not (Test-Path $fullPath)) {
                Write-Host "  [WARN] Broken link in $agentsFile : $linkPath" -ForegroundColor Yellow
                $Issues += @{ 
                    Type = "BrokenLink"
                    File = $agentsFile
                    Link = $linkPath
                    Severity = "Warning"
                }
                $Score -= 2
            }
        }
    }
}

if ($Issues.Count -eq 0) {
    Write-Host "  [PASS] All links valid" -ForegroundColor Green
}

# 2. 检查代码注释是否与实现一致
Write-Host "[2/4] Checking Code Comments..." -ForegroundColor Yellow

$tsFiles = Get-ChildItem -Path "src" -Include "*.ts", "*.tsx" -Recurse
$outdatedComments = 0

foreach ($file in $tsFiles) {
    $content = Get-Content $file.FullName -Raw
    
    # 查找 TODO/FIXME/HACK 等标记
    $todoPattern = '(TODO|FIXME|HACK|XXX|BUG):\s*(.+)'
    $todos = [regex]::Matches($content, $todoPattern)
    
    foreach ($todo in $todos) {
        $lineNumber = ($content.Substring(0, $todo.Index) -split "`n").Count
        Write-Host "  [INFO] $($file.Name):$lineNumber - $($todo.Value)" -ForegroundColor Gray
    }
    
    # 检查过时的注释（包含"old", "deprecated", "legacy"等词汇）
    $deprecatedPattern = '(deprecated|obsolete|outdated|legacy|old version)'
    if ($content -match $deprecatedPattern) {
        $outdatedComments++
    }
}

Write-Host "  [INFO] Found $outdatedComments potentially outdated comments" -ForegroundColor Yellow

# 3. 检查决策记录（ADR）是否更新
Write-Host "[3/4] Checking Architecture Decision Records..." -ForegroundColor Yellow

$adrDir = "docs/design-docs/decision-records"
if (Test-Path $adrDir) {
    $adrs = Get-ChildItem -Path $adrDir -Filter "*.md"
    Write-Host "  [INFO] Found $($adrs.Count) ADRs" -ForegroundColor Cyan
    
    foreach ($adr in $adrs) {
        $content = Get-Content $adr.FullName -Raw
        
        # 检查是否有 Status 字段
        if ($content -notmatch 'Status:') {
            Write-Host "  [WARN] Missing status in $($adr.Name)" -ForegroundColor Yellow
            $Issues += @{ 
                Type = "MissingADRStatus"
                File = $adr.FullName
                Severity = "Warning"
            }
            $Score -= 3
        }
        
        # 检查日期格式
        if ($content -match '(\d{4}-\d{2}-\d{2})') {
            $dateStr = $matches[1]
            try {
                $date = [DateTime]::Parse($dateStr)
                $daysOld = (New-TimeSpan -Start $date -End (Get-Date)).Days
                
                if ($daysOld -gt 180) {
                    Write-Host "  [INFO] $($adr.Name) is $daysOld days old" -ForegroundColor Gray
                }
            } catch {
                # 忽略日期解析错误
            }
        }
    }
} else {
    Write-Host "  [INFO] No ADR directory found" -ForegroundColor Gray
}

# 4. 检查产品文档是否同步
Write-Host "[4/4] Checking Product Documentation Sync..." -ForegroundColor Yellow

$docFiles = Get-ChildItem -Path "docs" -Filter "*.md" -Recurse
$lastModifiedThreshold = (Get-Date).AddMonths(-3)

foreach ($doc in $docFiles) {
    $lastWriteTime = $doc.LastWriteTime
    
    if ($lastWriteTime -lt $lastModifiedThreshold) {
        $daysSinceUpdate = (New-TimeSpan -Start $lastWriteTime -End (Get-Date)).Days
        Write-Host "  [INFO] $($doc.Name) - Last updated $daysSinceUpdate days ago" -ForegroundColor Gray
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
    Write-Host "  [EXCELLENT] Documentation Health Score: $Score/100" -ForegroundColor Green
} elseif ($Score -ge 70) {
    Write-Host "  [GOOD] Documentation Health Score: $score/100" -ForegroundColor Yellow
} else {
    Write-Host "  [NEEDS ATTENTION] Documentation Health Score: $score/100" -ForegroundColor Red
}

Write-Host ""
Write-Host "  Duration: $([math]::Round($Duration, 2)) seconds" -ForegroundColor Cyan
Write-Host "  Issues Found: $($Issues.Count)" -ForegroundColor $(if ($Issues.Count -eq 0) { "Green" } else { "Red" })
Write-Host ""

if ($Issues.Count -gt 0) {
    Write-Host "Issue List:" -ForegroundColor Yellow
    foreach ($issue in $Issues) {
        Write-Host "  [$($issue.Severity)] [$($issue.Type)] $($issue.File)" -ForegroundColor $(if ($issue.Severity -eq "Error") { "Red" } else { "Yellow" })
    }
    Write-Host ""
}

# Recommendations
Write-Host "Recommendations:" -ForegroundColor Blue
Write-Host "  1. Review broken links and update or remove them" -ForegroundColor Gray
Write-Host "  2. Address TODO/FIXME comments regularly" -ForegroundColor Gray
Write-Host "  3. Keep ADRs up to date with current decisions" -ForegroundColor Gray
Write-Host "  4. Review documentation older than 3 months" -ForegroundColor Gray
Write-Host ""

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
