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

# 函数：停止占用端口的进程
function Stop-ProcessOnPort {
    param([int]$Port)
    
    Write-Host "[INFO] Checking for processes on port $Port..." -ForegroundColor Yellow
    
    try {
        # 获取占用端口的进程
        $connections = Get-NetTCPConnection -LocalPort $Port -ErrorAction SilentlyContinue
        
        if ($connections) {
            foreach ($conn in $connections) {
                $processId = $conn.OwningProcess
                Write-Host "[INFO] Found process ID $processId on port $Port" -ForegroundColor Yellow
                
                try {
                    $process = Get-Process -Id $processId -ErrorAction SilentlyContinue
                    if ($process) {
                        Write-Host "[INFO] Stopping process: $($process.ProcessName) (PID: $processId)" -ForegroundColor Yellow
                        Stop-Process -Id $processId -Force -ErrorAction SilentlyContinue
                        
                        # 等待进程停止
                        Start-Sleep -Seconds 2
                        
                        # 验证是否已停止
                        $stillRunning = Get-Process -Id $processId -ErrorAction SilentlyContinue
                        if ($stillRunning) {
                            Write-Host "[WARN] Failed to stop process, trying again..." -ForegroundColor Yellow
                            Stop-Process -Id $processId -Force -ErrorAction Stop
                        } else {
                            Write-Host "[SUCCESS] Process stopped successfully" -ForegroundColor Green
                        }
                    }
                } catch {
                    Write-Host "[WARN] Could not stop process $processId : $_" -ForegroundColor Yellow
                }
            }
        } else {
            Write-Host "[INFO] No processes found on port $Port" -ForegroundColor Gray
        }
    } catch {
        Write-Host "[INFO] Port $Port is available or check failed: $_" -ForegroundColor Gray
    }
    
    # 额外等待确保端口释放
    Start-Sleep -Seconds 2
}

# 函数：启动开发服务器并等待就绪
function Start-DevServer {
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
                return $devProcess
            }
        } catch {
            # Server not ready yet, wait and retry
            Start-Sleep -Seconds $checkInterval
        }
    }
    
    if ((Get-Date) - $startTime -ge (New-TimeSpan -Seconds $timeout)) {
        Write-Host "[ERROR] Server failed to start within ${timeout}s" -ForegroundColor Red
        Write-Host "[INFO] Please check if Vite is configured correctly (port: $port)" -ForegroundColor Yellow
        return $null
    }
    
    return $devProcess
}

# 步骤 1: 强制关停现有服务
Write-Host "[INFO] =========================================" -ForegroundColor Cyan
Write-Host "[INFO]   Step 1: Cleanup Existing Services" -ForegroundColor Cyan
Write-Host "[INFO] =========================================" -ForegroundColor Cyan

Stop-ProcessOnPort -Port $port

# 步骤 2: 启动新的开发服务器
Write-Host "[INFO] =========================================" -ForegroundColor Cyan
Write-Host "[INFO]   Step 2: Start Development Server" -ForegroundColor Cyan
Write-Host "[INFO] =========================================" -ForegroundColor Cyan

$devProcess = Start-DevServer

if (-not $devProcess) {
    Write-Host "[ERROR] Failed to start development server" -ForegroundColor Red
    exit 1
}

# 步骤 3: 运行 E2E 测试
Write-Host "[INFO] =========================================" -ForegroundColor Cyan
Write-Host "[INFO]   Step 3: Running E2E Tests..." -ForegroundColor Cyan
Write-Host "[INFO] =========================================" -ForegroundColor Cyan

Write-Host "Running vitest run e2e (sequential)..." -ForegroundColor Gray
& npx.cmd vitest run e2e --no-file-parallelism
$testExitCode = $LASTEXITCODE

# 步骤 4: 清理 - 停止开发服务器
Write-Host "[INFO] =========================================" -ForegroundColor Cyan
Write-Host "[INFO]   Step 4: Cleanup - Stopping Server" -ForegroundColor Cyan
Write-Host "[INFO] =========================================" -ForegroundColor Cyan

if ($devProcess) {
    Write-Host "[INFO] Stopping development server..." -ForegroundColor Yellow
    try {
        Stop-Process -Id $devProcess.Id -Force -ErrorAction SilentlyContinue
        Start-Sleep -Seconds 2
        Write-Host "[SUCCESS] Development server stopped" -ForegroundColor Green
    } catch {
        Write-Host "[WARN] Could not stop dev server: $_" -ForegroundColor Yellow
    }
}

# 再次清理端口上的任何残留进程
Stop-ProcessOnPort -Port $port

# 输出测试结果
if ($testExitCode -eq 0) {
    Write-Host "[SUCCESS] All E2E tests passed!" -ForegroundColor Green
    exit 0
} else {
    Write-Host "[ERROR] E2E tests failed with exit code $testExitCode" -ForegroundColor Red
    exit $testExitCode
}