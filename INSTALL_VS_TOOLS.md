# 安装 Visual Studio Build Tools

由于 Tauri 需要 Rust 编译成 Windows 原生应用，必须安装 Visual Studio C++ Build Tools。

## 快速安装（推荐）

运行以下命令自动安装：

```batch
install-vs-build-tools.bat
```

## 手动安装

1. 访问 https://visualstudio.microsoft.com/zh-hans/downloads/
2. 下载 "Visual Studio Build Tools 2022"
3. 运行安装程序，选择 **"C++ 生成工具"** 工作负载
4. 确保包含：
   - MSVC v143 - VS 2022 C++ x64/x86 生成工具
   - Windows 10/11 SDK
5. 安装完成后**重启终端**

## 验证安装

安装完成后，打开新的命令提示符，运行：

```batch
cl
```

如果显示 Microsoft (R) C/C++ Optimizing Compiler 版本信息，说明安装成功。

## 安装后运行项目

```batch
npm run tauri:dev
```
