#!/usr/bin/env pwsh
# Harness Engineering - 死代码检测（垃圾回收）
# 用途：扫描未使用的导入、变量、函数和组件

param(
    [switch]$DryRun,        # 预览模式，不实际删除
    [switch]$Verbose,       # 详细输出
    [switch]$Json           # JSON 格式输出
)

$ErrorActionPreference = "Stop"
$StartTime = Get-Date
$DeadCodeItems = @()
$TotalCount = 0

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  Dead Code Detection (Garbage Collection)" -ForegroundColor Cyan
Write-Host ""

# 1. 检测未使用的 imports
Write-Host "[1/4] Detecting Unused Imports..." -ForegroundColor Yellow

$tsFiles = Get-ChildItem -Path "src" -Include "*.ts", "*.tsx" -Recurse -File
$unusedImports = 0

foreach ($file in $tsFiles) {
    $content = Get-Content $file.FullName -Raw
    $lines = Get-Content $file.FullName
    
    # 查找 import 语句
    $importPattern = '^import\s+.*\s+from\s+[''"](.+)[''"]'
    $imports = [regex]::Matches($content, $importPattern)
    
    foreach ($import in $imports) {
        $importLine = $import.Value
        $moduleName = $import.Groups[1].Value
        
        # 提取导入的标识符
        if ($importLine -match 'import\s+{([^}]+)}') {
            $identifiers = $matches[1] -split ',' | ForEach-Object { $_.Trim() }
            
            foreach ($identifier in $identifiers) {
                if (-not [string]::IsNullOrWhiteSpace($identifier)) {
                    # 计算该标识符在文件中出现的次数（除了 import 语句本身）
                    $usageCount = ([regex]::Matches($content, '\b' + [regex]::Escape($identifier) + '\b')).Count - 1
                    
                    if ($usageCount -eq 0) {
                        $lineNumber = ($content.Substring(0, $import.Index) -split "`n").Count
                        Write-Host "  [WARN] Unused import: $identifier in $($file.Name):$lineNumber" -ForegroundColor Yellow
                        
                        $DeadCodeItems += @{
                            Type = "UnusedImport"
                            Identifier = $identifier
                            File = $file.FullName
                            Line = $lineNumber
                        }
                        $unusedImports++
                    }
                }
            }
        }
    }
}

Write-Host "  Found $unusedImports unused imports" -ForegroundColor $(if ($unusedImports -eq 0) { "Green" } else { "Yellow" })

# 2. 检测未使用的函数和组件
Write-Host "[2/4] Detecting Unused Functions/Components..." -ForegroundColor Yellow

$unusedFunctions = 0
$functionPattern = '(?:export\s+)?(?:async\s+)?function\s+(\w+)'
$componentPattern = '(?:export\s+)?(?:const|function)\s+([A-Z]\w*)\s*[:=]'

foreach ($file in $tsFiles) {
    $content = Get-Content $file.FullName -Raw
    
    # 查找函数定义
    $functions = [regex]::Matches($content, $functionPattern)
    foreach ($func in $functions) {
        $funcName = $func.Groups[1].Value
        $usageCount = ([regex]::Matches($content, '\b' + [regex]::Escape($funcName) + '\b')).Count - 1
        
        if ($usageCount -eq 0 -and $funcName -notmatch '^(test|it|describe|expect)$') {
            Write-Host "  [WARN] Potentially unused function: $funcName in $($file.Name)" -ForegroundColor Yellow
            $unusedFunctions++
        }
    }
    
    # 查找 React 组件定义
    $components = [regex]::Matches($content, $componentPattern)
    foreach ($comp in $components) {
        $compName = $comp.Groups[1].Value
        
        # 只检查大写字母开头的组件名
        if ($compName -match '^[A-Z]') {
            $usageCount = ([regex]::Matches($content, '\b' + [regex]::Escape($compName) + '\b')).Count - 1
            
            if ($usageCount -eq 0) {
                Write-Host "  [WARN] Potentially unused component: $compName in $($file.Name)" -ForegroundColor Yellow
            }
        }
    }
}

Write-Host "  Found $unusedFunctions potentially unused functions/components" -ForegroundColor Yellow

# 3. 检测未使用的类型定义
Write-Host "[3/4] Detecting Unused Type Definitions..." -ForegroundColor Yellow

$unusedTypes = 0
$typePattern = '(?:export\s+)?(?:interface|type|enum)\s+([A-Z]\w*)'

foreach ($file in $tsFiles) {
    $content = Get-Content $file.FullName -Raw
    
    $types = [regex]::Matches($content, $typePattern)
    foreach ($type in $types) {
        $typeName = $type.Groups[1].Value
        $usageCount = ([regex]::Matches($content, '\b' + [regex]::Escape($typeName) + '\b')).Count - 1
        
        if ($usageCount -eq 0) {
            Write-Host "  [WARN] Unused type: $typeName in $($file.Name)" -ForegroundColor Yellow
            $unusedTypes++
        }
    }
}

Write-Host "  Found $unusedTypes unused type definitions" -ForegroundColor $(if ($unusedTypes -eq 0) { "Green" } else { "Yellow" })

# 4. 检测过期的 TODO 注释
Write-Host "[4/4] Detecting Stale TODO Comments..." -ForegroundColor Yellow

$staleTODOs = 0
$todoPattern = '(TODO|FIXME|HACK|XXX):\s*(.+)'

foreach ($file in $tsFiles) {
    $content = Get-Content $file.FullName -Raw
    $todos = [regex]::Matches($content, $todoPattern)
    
    foreach ($todo in $todos) {
        $todoText = $todo.Value
        $lineNumber = ($content.Substring(0, $todo.Index) -split "`n").Count
        
        Write-Host "  [INFO] TODO at $($file.Name):$lineNumber - $todoText" -ForegroundColor Gray
        $staleTODOs++
    }
}

Write-Host "  Found $staleTODOs TODO comments" -ForegroundColor Cyan

# Summary
$EndTime = Get-Date
$Duration = ($EndTime - $StartTime).TotalSeconds
$TotalCount = $unusedImports + $unusedFunctions + $unusedTypes

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  Detection Summary" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "  Unused Imports: $unusedImports" -ForegroundColor $(if ($unusedImports -eq 0) { "Green" } else { "Yellow" })
Write-Host "  Unused Functions: $unusedFunctions" -ForegroundColor Yellow
Write-Host "  Unused Types: $unusedTypes" -ForegroundColor $(if ($unusedTypes -eq 0) { "Green" } else { "Yellow" })
Write-Host "  TODO Comments: $staleTODOs" -ForegroundColor Cyan
Write-Host ""
Write-Host "  Total Issues: $TotalCount" -ForegroundColor $(if ($TotalCount -eq 0) { "Green" } else { "Red" })
Write-Host "  Duration: $([math]::Round($Duration, 2)) seconds" -ForegroundColor Cyan
Write-Host ""

if ($DryRun) {
    Write-Host "  [DRY RUN] No changes made (use -DryRun to preview)" -ForegroundColor Yellow
    Write-Host ""
} else {
    Write-Host "  To auto-fix unused imports, run: npx ts-prune" -ForegroundColor Blue
    Write-Host "  To auto-fix formatting, run: npm run format" -ForegroundColor Blue
    Write-Host ""
}

# JSON output for CI/CD
if ($Json) {
    $result = @{
        UnusedImports = $unusedImports
        UnusedFunctions = $unusedFunctions
        UnusedTypes = $unusedTypes
        StaleTODOs = $staleTODOs
        TotalIssues = $TotalCount
        Duration = [math]::Round($Duration, 2)
        Items = $DeadCodeItems
        Timestamp = (Get-Date -Format "yyyy-MM-dd HH:mm:ss")
    }
    $result | ConvertTo-Json -Depth 3
}

# Exit with warning code if issues found
if ($TotalCount -gt 0) {
    exit 1
} else {
    exit 0
}
