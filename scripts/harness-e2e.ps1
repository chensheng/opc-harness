param(
    [switch]$SkipServerCheck,
    [switch]$Help
)

# OPC-HARNESS E2E 测试运行脚本
# 自动检查并启动开发服务器

$port = 1420
$url = "http://localhost:$port"

Write-Host "[INFO] =========================================" -ForegroundColor Cyan
Write-Host "[INFO]   OPC-HARNESS E2E Test Runner" -ForegroundColor Cyan
Write-Host "[INFO] =========================================" -ForegroundColor Cyan

# 检查端口是否被占用
try {
    $tcpClient = New-Object System.Net.Sockets.TcpClient("127.0.0.1", $port)
    $tcpClient.Close()
    Write-Host "[SUCCESS] Development server already running on port $port" -ForegroundColor Green
} catch {
    Write-Host "[INFO] Starting development server..." -ForegroundColor Cyan
    
    # 启动开发服务器（使用 npm.cmd）
    $devProcess = Start-Process -FilePath "npm.cmd" -ArgumentList "run", "dev" -PassThru -NoNewWindow
    
    # 等待服务器启动（最多 60 秒）
    $timeout = 60
    $startTime = Get-Date
    $checkInterval = 2  # 每 2 秒检查一次
    
    Write-Host "[INFO] Waiting for server to be ready (timeout: ${timeout}s)..." -ForegroundColor Yellow
    
    while ((Get-Date) - $startTime -lt (New-TimeSpan -Seconds $timeout)) {
        try {
            $response = Invoke-WebRequest -Uri $url -TimeoutSec 2 -UseBasicParsing -ErrorAction Stop
            if ($response.StatusCode -eq 200) {
                Write-Host "[SUCCESS] Development server is ready!" -ForegroundColor Green
                break
            }
        } catch {
            # Server not ready yet, wait and retry
            Start-Sleep -Seconds $checkInterval
        }
    }
    
    if ((Get-Date) - $startTime -ge (New-TimeSpan -Seconds $timeout)) {
        Write-Host "[ERROR] Server failed to start within ${timeout}s" -ForegroundColor Red
        Write-Host "[INFO] Please check if Vite is configured correctly (port: $port)" -ForegroundColor Yellow
        exit 1
    }
}

# 运行 E2E 测试
Write-Host "[INFO] =========================================" -ForegroundColor Cyan
Write-Host "[INFO]   Running E2E Tests..." -ForegroundColor Cyan
Write-Host "[INFO] =========================================" -ForegroundColor Cyan

# 直接运行 Vitest，避免循环调用
Write-Host "Running vitest run e2e..." -ForegroundColor Gray
& npx.cmd vitest run e2e
$testExitCode = $LASTEXITCODE

if ($testExitCode -eq 0) {
    Write-Host "[SUCCESS] All E2E tests passed!" -ForegroundColor Green
    exit 0
} else {
    Write-Host "[ERROR] E2E tests failed with exit code $testExitCode" -ForegroundColor Red
    exit $testExitCode
}
