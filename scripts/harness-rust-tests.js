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
    stdio: 'pipe' // Capture output for parsing
  });

  const output = result.stdout + result.stderr;
  
  // Parse and display results in expected format
  if (output.match(/test result: ok\. (\d+) passed/)) {
    const match = output.match(/test result: ok\. (\d+) passed/);
    const testCount = match[1];
    console.log(chalk.green(`[PASS] All ${testCount} Rust tests passed`));
    process.exit(0);
  } else if (output.includes('test result: FAILED')) {
    console.log(chalk.red('[FAIL] Some Rust tests failed'));
    // Print the full output for debugging
    console.log(output);
    process.exit(1);
  } else {
    console.log(chalk.yellow('[WARN] Could not parse test results'));
    console.log(output);
    process.exit(1);
  }
} catch (error) {
  if (error.exitCode !== undefined) {
    const output = error.stdout + error.stderr;
    
    if (output.includes('test result: FAILED')) {
      console.log(chalk.red('[FAIL] Some Rust tests failed'));
    } else {
      console.error(chalk.red(`[ERROR] Exception: ${error.message}`));
    }
    
    // Print the full output for debugging
    console.log(output);
    process.exit(error.exitCode);
  } else {
    console.error(chalk.red(`[ERROR] Exception: ${error.message}`));
    process.exit(2);
  }
}
