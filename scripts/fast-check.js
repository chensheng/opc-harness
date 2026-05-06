#!/usr/bin/env node
// 快速编译脚本 - 仅检查语法,不链接
// 使用方法: node scripts/fast-check.js

import chalk from 'chalk';
import { execa } from 'execa';

console.log(chalk.cyan('🚀 快速语法检查 (跳过链接阶段)...'));

const startTime = Date.now();

try {
  // 检查 Rust 代码语法
  await execa('cargo', ['check', '--quiet'], {
    cwd: 'src-tauri',
    stdio: 'inherit'
  });

  const duration = ((Date.now() - startTime) / 1000).toFixed(2);
  console.log(chalk.green(`✅ 语法检查通过! 耗时: ${duration} 秒`));
  process.exit(0);
} catch (error) {
  console.error(chalk.red('❌ 发现语法错误,请查看上方详细信息'));
  process.exit(1);
}
