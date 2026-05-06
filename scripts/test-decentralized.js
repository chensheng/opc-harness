#!/usr/bin/env node
// 去中心化 Agent Node 功能测试脚本
// 使用方法: node scripts/test-decentralized.js

import chalk from 'chalk';
import { execa } from 'execa';

console.log(chalk.cyan('🧪 开始测试去中心化 Agent Node 功能...'));
console.log('');

// 检查 Tauri 开发服务器是否运行
try {
  const result = await execa('tasklist', ['/FI', 'IMAGENAME eq opc-harness.exe']);
  
  if (!result.stdout.includes('opc-harness.exe')) {
    console.log(chalk.yellow('⚠️  Tauri 应用未运行,请先执行: npm run tauri:dev'));
    console.log('');
    process.exit(1);
  }
  
  console.log(chalk.green('✅ Tauri 应用正在运行'));
  console.log('');
} catch (error) {
  // Windows 上 tasklist 可能失败,继续执行
  console.log(chalk.yellow('⚠️  无法检测 Tauri 进程,请确保已启动'));
  console.log('');
}

// 测试步骤
console.log(chalk.cyan('📋 测试步骤:'));
console.log(chalk.white('1. 启动第一个 Node (node-1)'));
console.log(chalk.white('2. 启动第二个 Node (node-2)'));
console.log(chalk.white('3. 查看运行中的 Nodes 列表'));
console.log(chalk.white('4. 停止 node-1'));
console.log(chalk.white('5. 验证剩余 Nodes'));
console.log('');

console.log(chalk.yellow('💡 提示: 请在浏览器控制台执行以下命令进行测试:'));
console.log('');

console.log(chalk.gray('// 1. 导入 Hook'));
console.log(chalk.white("import { useDecentralizedNodes } from './hooks/useDecentralizedNodes'"));
console.log('');

console.log(chalk.gray('// 2. 在 React 组件中使用'));
console.log(chalk.white('const { startNode, stopNode, nodes, refreshNodes } = useDecentralizedNodes()'));
console.log('');

console.log(chalk.gray('// 3. 启动第一个 Node'));
console.log(chalk.white("await startNode({ nodeId: 'node-1', maxConcurrent: 3 })"));
console.log('');

console.log(chalk.gray('// 4. 启动第二个 Node'));
console.log(chalk.white("await startNode({ nodeId: 'node-2', maxConcurrent: 5 })"));
console.log('');

console.log(chalk.gray('// 5. 刷新并查看 Nodes 列表'));
console.log(chalk.white('await refreshNodes()'));
console.log(chalk.white("console.log('Running nodes:', nodes)"));
console.log('');

console.log(chalk.gray('// 6. 停止 node-1'));
console.log(chalk.white("await stopNode('node-1')"));
console.log('');

console.log(chalk.gray('// 7. 验证只剩 node-2'));
console.log(chalk.white('await refreshNodes()'));
console.log(chalk.white("console.log('Remaining nodes:', nodes)"));
console.log('');

console.log(chalk.cyan('🎯 预期结果:'));
console.log(chalk.white('- 成功启动 2 个独立的 Node 实例'));
console.log(chalk.white('- 每个 Node 有唯一的 node_id'));
console.log(chalk.white('- Node 之间共享 EventBus 和 LockManager'));
console.log(chalk.white('- 可以独立停止任意 Node'));
console.log('');

console.log(chalk.cyan('📊 监控日志输出:'));
console.log(chalk.white('在终端中应该看到类似日志:'));
console.log(chalk.gray('[DecentralizedNode] Created node node-1 with config: ...'));
console.log(chalk.gray('[DecentralizedNode:node-1] Starting decentralized agent loop'));
console.log(chalk.gray('[DecentralizedNode:node-1] Event listener started'));
console.log('');

console.log(chalk.green('✅ 测试准备完成! 请在浏览器控制台执行上述命令。'));
process.exit(0);
