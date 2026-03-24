# harness-ts-tests.ps1 - TypeScript Unit Test Executor
# Dedicated script for executing TypeScript unit tests with reliable output parsing

param(
    [switch]$Verbose,
    [switch]$Debug
)

[Console]::OutputEncoding = [System.Text.Encoding]::UTF8

Write-Host "Running TypeScript unit tests..." -ForegroundColor Gray

# Execute tests and capture both stdout and stderr
$output = & npm run test:unit 2>&1
$exitCode = $LASTEXITCODE

if ($Verbose) {
    Write-Host ""
    Write-Host "=== Full Test Output ===" -ForegroundColor Cyan
    foreach ($line in $output) { Write-Host $line -ForegroundColor Gray }
    Write-Host "========================"
    Write-Host ""
}

# Function to strip ANSI escape codes
function Remove-AnsiCodes {
    param([string]$text)
    return $text -replace '\e\[[0-9;]*m', '' -replace '\e\[[0-9;]*K', ''
}

# Search for summary lines by iterating through the array
$testFilesLine = $null
$testsLine = $null
$testFiles = 0
$totalTests = 0

foreach ($line in $output) {
    # Only process string types, skip ErrorRecord and other objects
    if ($line -isnot [string]) {
        continue
    }
    
    # Strip ANSI color codes before processing
    $cleanLine = Remove-AnsiCodes $line
    $trimmedLine = $cleanLine.Trim()
    
    # Match patterns on clean text
    if ($trimmedLine -match 'Test Files\s+(\d+)\s+passed') {
        $testFilesLine = $line
        $testFiles = $matches[1]
    }
    if ($trimmedLine -match 'Tests\s+(\d+)\s+passed') {
        $testsLine = $line
        $totalTests = $matches[1]
    }
}

if ($testFilesLine) {
    if ($testsLine) {
        Write-Host "[PASS] All TS tests passed ($testFiles files, $totalTests tests)" -ForegroundColor Green
    } else {
        Write-Host "[PASS] All TS tests passed ($testFiles files)" -ForegroundColor Green
    }
    
    # Try to extract duration from cleaned output
    foreach ($line in $output) {
        if ($line -isnot [string]) { continue }
        
        $cleanLine = Remove-AnsiCodes $line
        if ($cleanLine.Trim() -match 'Duration\s+([\d.]+)s') {
            Write-Host "  Duration: $($matches[1])s" -ForegroundColor Gray
            break
        }
    }
    
    exit 0
} else {
    # No summary line found
    Write-Host "[WARN] Could not find test summary in output" -ForegroundColor Yellow
    
    if ($Debug) {
        Write-Host ""
        Write-Host "Raw output (last 15 lines):" -ForegroundColor Gray
        ($output | Select-Object -Last 15) | ForEach-Object { 
            if ($_ -is [string]) {
                Write-Host "[$_]" -ForegroundColor Gray 
            } else {
                Write-Host "[$($_.GetType().Name)]" -ForegroundColor Gray
            }
        }
    }
    
    # If no summary found but exit code is 0, assume success
    if ($exitCode -eq 0) {
        Write-Host "Assuming success based on exit code..." -ForegroundColor Gray
        exit 0
    } else {
        Write-Host "[FAIL] Test execution failed (exit code: $exitCode)" -ForegroundColor Red
        exit 1
    }
}
