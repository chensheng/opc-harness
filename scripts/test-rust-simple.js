#!/usr/bin/env node
// Simple Rust test runner - no buffering issues

import chalk from 'chalk';
import { execa } from 'execa';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const projectRoot = dirname(__dirname);

console.log(chalk.yellow('Running Rust unit tests...'));

try {
  const result = await execa('cargo', ['test', '--bin', 'opc-harness'], {
    cwd: join(projectRoot, 'src-tauri'),
    stdio: 'inherit'
  });

  console.log(chalk.green('[PASS] All Rust tests passed'));
  process.exit(0);
} catch (error) {
  if (error.exitCode !== undefined) {
    console.log(chalk.red(`[FAIL] Some Rust tests failed (exit code: ${error.exitCode})`));
    process.exit(error.exitCode);
  } else {
    console.error(chalk.red(`[ERROR] Exception: ${error.message}`));
    process.exit(2);
  }
}
