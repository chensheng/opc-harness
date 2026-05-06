#!/usr/bin/env pwsh
# Simple Rust test runner - no buffering issues

Write-Host "Running Rust unit tests..." -ForegroundColor Yellow

$originalLocation = Get-Location
$tauriPath = Join-Path $originalLocation.Path "src-tauri"

try {
    Set-Location $tauriPath
    
    # Run cargo test directly, let it output to console
    & cargo test --bin opc-harness
    $exitCode = $LASTEXITCODE
    
    if ($exitCode -eq 0) {
        Write-Host "[PASS] All Rust tests passed" -ForegroundColor Green
    } else {
        Write-Host "[FAIL] Some Rust tests failed (exit code: $exitCode)" -ForegroundColor Red
    }
    
    exit $exitCode
} catch {
    Write-Host "[ERROR] Exception: $_" -ForegroundColor Red
    exit 2
} finally {
    Set-Location $originalLocation
}
