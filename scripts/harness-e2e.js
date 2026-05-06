#!/usr/bin/env node
// OPC-HARNESS E2E 测试运行脚本
// 自动检查并启动开发服务器

import chalk from 'chalk';
import { execa } from 'execa';
import http from 'http';

const port = 1420;
const url = `http://localhost:${port}`;

console.log(chalk.cyan('[INFO] ========================================='));
console.log(chalk.cyan('[INFO]   OPC-HARNESS E2E Test Runner'));
console.log(chalk.cyan('[INFO] ========================================='));

// 检查端口是否被占用
function checkPort(port) {
  return new Promise((resolve) => {
    const server = http.createServer();
    
    server.once('error', (err) => {
      if (err.code === 'EADDRINUSE') {
        resolve(true); // Port is in use
      } else {
        resolve(false);
      }
      server.close();
    });
    
    server.once('listening', () => {
      server.close();
      resolve(false); // Port is free
    });
    
    server.listen(port);
  });
}

// 等待服务器就绪
async function waitForServer(url, timeout = 60) {
  const startTime = Date.now();
  const checkInterval = 2000; // 每 2 秒检查一次
  
  console.log(chalk.yellow(`[INFO] Waiting for server to be ready (timeout: ${timeout}s)...`));
  
  while (Date.now() - startTime < timeout * 1000) {
    try {
      await fetch(url, { signal: AbortSignal.timeout(2000) });
      console.log(chalk.green('[SUCCESS] Development server is ready!'));
      return true;
    } catch (error) {
      // Server not ready yet, wait and retry
      await new Promise(resolve => setTimeout(resolve, checkInterval));
    }
  }
  
  console.log(chalk.red(`[ERROR] Server failed to start within ${timeout}s`));
  console.log(chalk.yellow(`[INFO] Please check if Vite is configured correctly (port: ${port})`));
  return false;
}

try {
  const portInUse = await checkPort(port);
  
  if (portInUse) {
    console.log(chalk.green(`[SUCCESS] Development server already running on port ${port}`));
  } else {
    console.log(chalk.cyan('[INFO] Starting development server...'));
    
    // 启动开发服务器
    const devProcess = execa('npm', ['run', 'dev'], {
      stdio: 'pipe'
    });
    
    // 等待服务器启动
    const ready = await waitForServer(url, 60);
    
    if (!ready) {
      devProcess.kill();
      process.exit(1);
    }
    
    // 服务器启动后,保持进程运行但继续执行测试
    // 注意: 不等待 devProcess 完成,让它在后台运行
  }
} catch (error) {
  console.error(chalk.red(`[ERROR] Failed to check/start server: ${error.message}`));
  process.exit(1);
}

// 运行 E2E 测试
console.log(chalk.cyan('[INFO] ========================================='));
console.log(chalk.cyan('[INFO]   Running E2E Tests...'));
console.log(chalk.cyan('[INFO] ========================================='));

console.log(chalk.gray('Running vitest run e2e...'));

try {
  const result = await execa('npx', ['vitest', 'run', 'e2e'], {
    stdio: 'inherit'
  });
  
  console.log(chalk.green('[SUCCESS] All E2E tests passed!'));
  process.exit(0);
} catch (error) {
  console.error(chalk.red(`[ERROR] E2E tests failed with exit code ${error.exitCode}`));
  process.exit(error.exitCode || 1);
}
