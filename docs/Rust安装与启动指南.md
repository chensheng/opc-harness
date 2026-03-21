# Rust安装与Tauri启动指南

> **文档版本**: v1.0  
> **适用系统**: Windows 10/11

---

## 安装步骤

### 第一步: 安装Rust

#### 方法1: 使用Winget (推荐)

```powershell
# 以管理员身份打开PowerShell
winget install Rustlang.Rustup

# 安装完成后重启终端
# 验证安装
rustc --version  # 应显示 >= 1.70.0
cargo --version
```

#### 方法2: 官方安装脚本

```powershell
# 下载并运行安装程序
Invoke-WebRequest -Uri https://win.rustup.rs/x86_64 -OutFile rustup-init.exe
.\rustup-init.exe -y --default-toolchain stable

# 重启终端后验证
rustc --version
```

#### 方法3: 手动下载

1. 访问 https://rustup.rs/
2. 下载 Windows 64-bit 版本
3. 运行安装程序，选择默认选项
4. 安装完成后重启终端

---

### 第二步: 安装Tauri依赖

#### 1. 安装 WebView2 Runtime

Tauri需要Microsoft Edge WebView2:

```powershell
# 方法1: 通过Winget安装
winget install Microsoft.EdgeWebView2Runtime

# 方法2: 手动下载
# 访问 https://developer.microsoft.com/en-us/microsoft-edge/webview2/
# 下载 "Evergreen Standalone Installer" 并安装
```

#### 2. 安装 Visual Studio Build Tools (Windows必需)

```powershell
# 方法1: 通过Winget安装
winget install Microsoft.VisualStudio.2022.BuildTools

# 方法2: 手动下载
# 访问 https://visualstudio.microsoft.com/visual-cpp-build-tools/
# 下载并安装 "C++ build tools"
# 在安装器中选择:
#   - MSVC v143 - VS 2022 C++ x64/x86 生成工具
#   - Windows 11 SDK
```

---

### 第三步: 配置Rust环境

```powershell
# 添加Cargo到PATH (如果还没添加)
$env:PATH += ";$env:USERPROFILE\.cargo\bin"

# 验证安装
rustc --version    # 应 >= 1.70.0
cargo --version    # 应 >= 1.70.0

# 安装Tauri CLI (可选但推荐)
cargo install tauri-cli
```

---

## 启动Tauri应用

### 开发模式 (热更新)

```powershell
# 进入项目目录
cd D:\workspace\opc-harness

# 确保前端依赖已安装
npm install

# 启动Tauri开发服务器
npm run tauri:dev
```

**预期输出**:
```
> tauri dev

     Running BeforeDevCommand (`npm run dev`)

  VITE v5.4.21  ready in xxx ms

  ➜  Local:   http://localhost:1420/
  ➜  Network: use --host to expose

     Running DevCommand (`cargo run`)
    Compiling opc-harness v0.1.0
     Finished dev [unoptimized + debuginfo] target(s) in xx s
     Running `target\debug\opc-harness.exe`
```

应用启动后会自动打开窗口，显示OPC-HARNESS界面。

---

### 构建发布版本

```powershell
# 构建生产版本
npm run tauri:build
```

**输出位置**:
- 安装包: `src-tauri/target/release/bundle/`
- 可执行文件: `src-tauri/target/release/opc-harness.exe`

---

## 常见问题

### 1. 编译错误: "linker not found"

**原因**: 缺少Visual Studio Build Tools

**解决**:
```powershell
winget install Microsoft.VisualStudio.2022.BuildTools
# 然后在安装器中选择C++桌面开发工作负载
```

### 2. 错误: "WebView2 not found"

**原因**: 未安装WebView2 Runtime

**解决**:
```powershell
winget install Microsoft.EdgeWebView2Runtime
```

### 3. Cargo命令找不到

**原因**: PATH环境变量未配置

**解决**:
```powershell
# 临时添加 (当前会话)
$env:PATH += ";$env:USERPROFILE\.cargo\bin"

# 永久添加
[Environment]::SetEnvironmentVariable(
    "Path",
    [Environment]::GetEnvironmentVariable("Path", "User") + ";$env:USERPROFILE\.cargo\bin",
    "User"
)
```

### 4. 编译速度慢

**建议**: 
- 首次编译需要下载依赖，时间较长（5-15分钟）
- 后续编译会使用缓存，速度会快很多
- 使用SSD硬盘可显著提升编译速度

---

## 验证清单

安装完成后，运行以下命令验证:

```powershell
# 1. Rust版本
rustc --version
# 预期: rustc 1.70.0 或更高

# 2. Cargo版本  
cargo --version
# 预期: cargo 1.70.0 或更高

# 3. WebView2
Get-ItemProperty -Path "HKLM:\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}" -ErrorAction SilentlyContinue
# 应显示WebView2版本信息

# 4. 项目编译
cd D:\workspace\opc-harness
npm run tauri:build
# 应成功构建，无错误
```

---

## 下一步

启动应用后，您可以:

1. **测试Vibe Design**: 输入产品想法，查看界面是否正常
2. **配置AI厂商**: 在设置中添加OpenAI/Claude API密钥
3. **继续开发**: 参考 `MVP开发任务拆解.md` 进行后续开发

---

## 相关链接

- [Rust官网](https://www.rust-lang.org/)
- [Tauri文档](https://tauri.app/)
- [Tauri Windows先决条件](https://tauri.app/start/prerequisites/#windows)
- [WebView2下载](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)

---

> **提示**: 如果在安装过程中遇到问题，请检查:
> 1. 是否以管理员身份运行PowerShell
> 2. 网络连接是否正常
> 3. 磁盘空间是否充足 (需要至少2GB可用空间)
