#!/usr/bin/env pwsh
# Simple Rust test runner - no buffering issues

Write-Host "Running Rust unit tests..." -ForegroundColor Yellow

$originalLocation = Get-Location
$tauriPath = Join-Path $originalLocation.Path "src-tauri"

try {
    Set-Location $tauriPath
    
    # Run cargo test and capture output as string
    $testOutput = (& cargo test --bin opc-harness 2>&1) | Out-String
    $exitCode = $LASTEXITCODE
    
    # Parse and display results in expected format
    if ($testOutput -match "test result: ok\. (\d+) passed") {
        $testCount = $matches[1]
        Write-Host "[PASS] All $testCount Rust tests passed" -ForegroundColor Green
    } elseif ($testOutput -match "test result: FAILED") {
        Write-Host "[FAIL] Some Rust tests failed" -ForegroundColor Red
    } else {
        Write-Host "[WARN] Could not parse test results" -ForegroundColor Yellow
    }
    
    exit $exitCode
} catch {
    Write-Host "[ERROR] Exception: $_" -ForegroundColor Red
    exit 2
} finally {
    Set-Location $originalLocation
}
