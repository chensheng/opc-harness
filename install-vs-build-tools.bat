@echo off
chcp 65001 >nul
echo ==========================================
echo   安装 Visual Studio Build Tools
echo ==========================================
echo.
echo 正在下载 Visual Studio Installer...
echo.

powershell -Command "Invoke-WebRequest -Uri 'https://aka.ms/vs/17/release/vs_buildtools.exe' -OutFile 'vs_buildtools.exe' -UseBasicParsing"

if not exist "vs_buildtools.exe" (
    echo [错误] 下载失败，请检查网络连接
    pause
    exit /b 1
)

echo.
echo 正在安装 Visual Studio Build Tools (包含 C++ 组件)...
echo 这可能需要 10-30 分钟，取决于网络速度
echo.

vs_buildtools.exe --quiet --wait --norestart --nocache ^
    --add Microsoft.VisualStudio.Workload.VCTools ^
    --add Microsoft.VisualStudio.Component.VC.Tools.x86.x64 ^
    --add Microsoft.VisualStudio.Component.VC.ATL ^
    --add Microsoft.VisualStudio.Component.Windows11SDK.22621

if %errorLevel% equ 0 (
    echo.
    echo [成功] Visual Studio Build Tools 安装完成！
    echo.
    echo 请重启终端后再次运行：npm run tauri:dev
) else (
    echo.
    echo [错误] 安装失败，错误代码：%errorLevel%
    echo 请手动下载并安装：https://visualstudio.microsoft.com/zh-hans/downloads/
)

del vs_buildtools.exe
pause
