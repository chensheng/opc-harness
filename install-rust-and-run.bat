@echo off
chcp 65001 >nul
echo ==========================================
echo   OPC-HARNESS Rust安装和启动脚本
echo ==========================================
echo.

REM 检查是否以管理员身份运行
net session >nul 2>&1
if %errorLevel% neq 0 (
    echo [错误] 请以管理员身份运行此脚本！
    echo 右键点击脚本，选择"以管理员身份运行"
    pause
    exit /b 1
)

echo [1/5] 检查现有Rust安装...
where rustc >nul 2>&1
if %errorLevel% equ 0 (
    echo [OK] Rust已安装
    rustc --version
    goto :CHECK_DEPS
)

echo.
echo [2/5] 正在下载Rust安装程序...
powershell -Command "Invoke-WebRequest -Uri 'https://win.rustup.rs/x86_64' -OutFile 'rustup-init.exe' -UseBasicParsing"
if not exist rustup-init.exe (
    echo [错误] 下载失败，请检查网络连接
    pause
    exit /b 1
)

echo.
echo [3/5] 正在安装Rust...
rustup-init.exe -y --default-toolchain stable
if %errorLevel% neq 0 (
    echo [错误] Rust安装失败
    pause
    exit /b 1
)

echo.
echo [4/5] 配置环境变量...
set "PATH=%USERPROFILE%\.cargo\bin;%PATH%"

echo.
echo [5/5] 验证Rust安装...
where rustc >nul 2>&1
if %errorLevel% neq 0 (
    echo [错误] Rust安装后仍无法找到，请手动重启终端
    pause
    exit /b 1
)

echo [OK] Rust安装成功！
rustc --version
cargo --version

echo.
:CHECK_DEPS
echo.
echo ==========================================
echo   检查项目依赖
echo ==========================================
echo.

cd /d "%~dp0"

if not exist "node_modules" (
    echo 正在安装npm依赖...
    call npm install
    if %errorLevel% neq 0 (
        echo [错误] npm install 失败
        pause
        exit /b 1
    )
) else (
    echo [OK] npm依赖已安装
)

echo.
echo ==========================================
echo   启动Tauri开发服务器
echo ==========================================
echo.
echo 提示：首次编译需要5-15分钟，请耐心等待...
echo.

call npm run tauri:dev

pause
