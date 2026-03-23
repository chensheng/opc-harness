#!/usr/bin/env pwsh
# 示例：使用 Harness 任务执行器包装开发任务
# 这个文件展示了如何在开发任务后自动运行验证

# 示例 1: 添加新组件后自动验证
function Add-Component {
    param([string]$ComponentName)
    
    .\scripts\harness-task.ps1 `
        -TaskName "添加组件：$ComponentName" `
        -ScriptBlock {
            # 在这里编写你的开发任务
            # 例如：创建组件文件、更新路由等
            Write-Host "创建组件文件..."
            # New-Item -ItemType File -Path "src/components/$ComponentName.tsx"
            
            Write-Host "更新类型定义..."
            # 添加类型定义代码
            
            Write-Host "组件创建完成"
        }
}

# 示例 2: 重构代码后自动验证
function Refactor-Code {
    param([string]$ModulePath)
    
    .\scripts\harness-task.ps1 `
        -TaskName "重构模块：$ModulePath" `
        -ScriptBlock {
            # 执行重构操作
            Write-Host "执行重构..."
            # 重构代码
            
            Write-Host "运行测试..."
            # npm test
        } `
        -KeepDevServer  # 验证后保持服务器运行，方便手动检查
}

# 示例 3: 快速修改后快速验证
function Quick-Fix {
    .\scripts\harness-task.ps1 `
        -TaskName "快速修复" `
        -ScriptBlock {
            npm run lint:fix
            npm run format
        } `
        -SkipAutoVerify  # 跳过完整验证，只做健康检查
}

# 显示帮助
function Show-Help {
    Write-Host @"
Harness Task Runner 使用示例

1. 添加新组件（完整验证）:
   .\example-task.ps1; Add-Component -ComponentName "MyButton"

2. 重构代码（保持服务器运行）:
   .\example-task.ps1; Refactor-Code -ModulePath "src/stores"

3. 快速修复（跳过验证）:
   .\example-task.ps1; Quick-Fix

4. 自定义任务:
   .\scripts\harness-task.ps1 `
       -TaskName "我的任务" `
       -ScriptBlock { 
           # 你的代码
       }
"@
}

# 如果没有参数，显示帮助
if ($args.Count -eq 0) {
    Show-Help
}
