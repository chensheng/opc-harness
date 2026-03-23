#!/usr/bin/env pwsh
# CLI Browser 检测脚本
# 检测当前可用的 CLI 浏览器工具

param(
    [string]$ForceCLI = ""  # 强制指定 CLI: kimi, claude, opencode
)

$CLIInfo = @{
    Name = $null
    Version = $null
    BrowserCommand = $null
    Type = $null
}

# 1. 检查是否强制指定
if ($ForceCLI) {
    $CLIInfo.Name = $ForceCLI.ToLower()
    switch ($ForceCLI.ToLower()) {
        "kimi" { 
            $CLIInfo.Type = "kimi"
            $CLIInfo.BrowserCommand = "@browser"
        }
        "claude" { 
            $CLIInfo.Type = "claude"
            $CLIInfo.BrowserCommand = "Browser"
        }
        "opencode" { 
            $CLIInfo.Type = "opencode"
            $CLIInfo.BrowserCommand = "browser"
        }
        default {
            Write-Error "未知的 CLI 类型: $ForceCLI"
            exit 1
        }
    }
    return $CLIInfo
}

# 2. 环境变量检测
$DetectedCLI = $env:HARNESS_CLI_BROWSER

if ($DetectedCLI) {
    $CLIInfo.Name = $DetectedCLI
    switch ($DetectedCLI.ToLower()) {
        "kimi" { $CLIInfo.Type = "kimi"; $CLIInfo.BrowserCommand = "@browser" }
        "claude" { $CLIInfo.Type = "claude"; $CLIInfo.BrowserCommand = "Browser" }
        "opencode" { $CLIInfo.Type = "opencode"; $CLIInfo.BrowserCommand = "browser" }
    }
    return $CLIInfo
}

# 3. 自动检测

# 检测 Kimi Code CLI
if ($env:KIMI_CLI_VERSION -or (Get-Command "kimi" -ErrorAction SilentlyContinue)) {
    $CLIInfo.Name = "kimi"
    $CLIInfo.Type = "kimi"
    $CLIInfo.Version = $env:KIMI_CLI_VERSION
    $CLIInfo.BrowserCommand = "@browser"
    if (-not $CLIInfo.Version) {
        try {
            $CLIInfo.Version = (kimi --version 2>$null) -replace '.*?([\d.]+).*', '$1'
        } catch {}
    }
    return $CLIInfo
}

# 检测 Claude Code
if ($env:CLAUDE_CODE_VERSION -or (Get-Command "claude" -ErrorAction SilentlyContinue)) {
    $CLIInfo.Name = "claude"
    $CLIInfo.Type = "claude"
    $CLIInfo.Version = $env:CLAUDE_CODE_VERSION
    $CLIInfo.BrowserCommand = "Browser"
    return $CLIInfo
}

# 检测 OpenCode
if (Get-Command "opencode" -ErrorAction SilentlyContinue) {
    $CLIInfo.Name = "opencode"
    $CLIInfo.Type = "opencode"
    $CLIInfo.BrowserCommand = "browser"
    try {
        $versionInfo = opencode --version 2>&1
        $CLIInfo.Version = $versionInfo -replace '.*?([\d.]+).*', '$1'
    } catch {}
    return $CLIInfo
}

# 4. 无法检测
Write-Warning "未能检测到支持的 CLI 浏览器工具"
Write-Host "支持的 CLI: Kimi Code CLI, Claude Code, OpenCode" -ForegroundColor Yellow
Write-Host "可以通过环境变量强制指定: `$env:HARNESS_CLI_BROWSER='kimi'" -ForegroundColor Gray

return $CLIInfo
