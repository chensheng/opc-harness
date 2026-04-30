# 去中心化 Agent Node 功能测试脚本
# 使用方法: .\scripts\test-decentralized.ps1

Write-Host "🧪 开始测试去中心化 Agent Node 功能..." -ForegroundColor Cyan
Write-Host ""

# 检查 Tauri 开发服务器是否运行
$tauriProcess = Get-Process | Where-Object { $_.ProcessName -like "*opc-harness*" }
if (-not $tauriProcess) {
    Write-Host "⚠️  Tauri 应用未运行,请先执行: npm run tauri:dev" -ForegroundColor Yellow
    Write-Host ""
    exit 1
}

Write-Host "✅ Tauri 应用正在运行" -ForegroundColor Green
Write-Host ""

# 测试步骤
Write-Host "📋 测试步骤:" -ForegroundColor Cyan
Write-Host "1. 启动第一个 Node (node-1)" -ForegroundColor White
Write-Host "2. 启动第二个 Node (node-2)" -ForegroundColor White
Write-Host "3. 查看运行中的 Nodes 列表" -ForegroundColor White
Write-Host "4. 停止 node-1" -ForegroundColor White
Write-Host "5. 验证剩余 Nodes" -ForegroundColor White
Write-Host ""

Write-Host "💡 提示: 请在浏览器控制台执行以下命令进行测试:" -ForegroundColor Yellow
Write-Host ""

Write-Host "// 1. 导入 Hook" -ForegroundColor Gray
Write-Host "import { useDecentralizedNodes } from './hooks/useDecentralizedNodes'" -ForegroundColor White
Write-Host ""

Write-Host "// 2. 在 React 组件中使用" -ForegroundColor Gray
Write-Host "const { startNode, stopNode, nodes, refreshNodes } = useDecentralizedNodes()" -ForegroundColor White
Write-Host ""

Write-Host "// 3. 启动第一个 Node" -ForegroundColor Gray
Write-Host "await startNode({ nodeId: 'node-1', maxConcurrent: 3 })" -ForegroundColor White
Write-Host ""

Write-Host "// 4. 启动第二个 Node" -ForegroundColor Gray
Write-Host "await startNode({ nodeId: 'node-2', maxConcurrent: 5 })" -ForegroundColor White
Write-Host ""

Write-Host "// 5. 刷新并查看 Nodes 列表" -ForegroundColor Gray
Write-Host "await refreshNodes()" -ForegroundColor White
Write-Host "console.log('Running nodes:', nodes)" -ForegroundColor White
Write-Host ""

Write-Host "// 6. 停止 node-1" -ForegroundColor Gray
Write-Host "await stopNode('node-1')" -ForegroundColor White
Write-Host ""

Write-Host "// 7. 验证只剩 node-2" -ForegroundColor Gray
Write-Host "await refreshNodes()" -ForegroundColor White
Write-Host "console.log('Remaining nodes:', nodes)" -ForegroundColor White
Write-Host ""

Write-Host "🎯 预期结果:" -ForegroundColor Cyan
Write-Host "- 成功启动 2 个独立的 Node 实例" -ForegroundColor White
Write-Host "- 每个 Node 有唯一的 node_id" -ForegroundColor White
Write-Host "- Node 之间共享 EventBus 和 LockManager" -ForegroundColor White
Write-Host "- 可以独立停止任意 Node" -ForegroundColor White
Write-Host ""

Write-Host "📊 监控日志输出:" -ForegroundColor Cyan
Write-Host "在终端中应该看到类似日志:" -ForegroundColor White
Write-Host "[DecentralizedNode] Created node node-1 with config: ..." -ForegroundColor Gray
Write-Host "[DecentralizedNode:node-1] Starting decentralized agent loop" -ForegroundColor Gray
Write-Host "[DecentralizedNode:node-1] Event listener started" -ForegroundColor Gray
Write-Host ""

Write-Host "✅ 测试准备完成! 请在浏览器控制台执行上述命令。" -ForegroundColor Green
