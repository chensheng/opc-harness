#!/usr/bin/env node
// harness-ts-tests.js - TypeScript Unit Test Executor
// Dedicated script for executing TypeScript unit tests with reliable output parsing

import chalk from 'chalk';
import { execa } from 'execa';

const args = process.argv.slice(2);
const verbose = args.includes('--verbose') || args.includes('-v');
const debug = args.includes('--debug');
const skipE2E = args.includes('--skip-e2e');

console.log(chalk.gray('Running TypeScript unit tests...'));

if (skipE2E) {
  console.log(chalk.yellow('Skipping E2E tests (dev server not running)...'));
}

try {
  // Execute tests and capture both stdout and stderr
  const testArgs = ['run', 'test:unit'];
  if (skipE2E) {
    testArgs.push('--', '--exclude', '**/e2e/*.spec.ts');
  }
  
  const result = await execa('npm', testArgs, {
    stdio: 'pipe'
  });

  const output = result.stdout + result.stderr;
  
  if (verbose) {
    console.log('\n' + chalk.cyan('=== Full Test Output ==='));
    console.log(chalk.gray(output));
    console.log(chalk.cyan('========================\n'));
  }

  // Function to strip ANSI escape codes
  const removeAnsiCodes = (text) => {
    return text.replace(/\x1b\[[0-9;]*m/g, '').replace(/\x1b\[[0-9;]*K/g, '');
  };

  // Search for summary lines
  const lines = output.split('\n');
  let testFilesLine = null;
  let testsLine = null;
  let testFiles = 0;
  let totalTests = 0;

  for (const line of lines) {
    if (typeof line !== 'string') continue;
    
    // Strip ANSI color codes before processing
    const cleanLine = removeAnsiCodes(line);
    const trimmedLine = cleanLine.trim();
    
    // Match patterns on clean text
    const filesMatch = trimmedLine.match(/Test Files\s+(\d+)\s+passed/);
    if (filesMatch) {
      testFilesLine = line;
      testFiles = parseInt(filesMatch[1]);
    }
    
    const testsMatch = trimmedLine.match(/Tests\s+(\d+)\s+passed/);
    if (testsMatch) {
      testsLine = line;
      totalTests = parseInt(testsMatch[1]);
    }
  }

  if (testFilesLine) {
    if (testsLine) {
      console.log(chalk.green(`[PASS] All TS tests passed (${testFiles} files, ${totalTests} tests)`));
    } else {
      console.log(chalk.green(`[PASS] All TS tests passed (${testFiles} files)`));
    }
    
    // Try to extract duration from cleaned output
    for (const line of lines) {
      if (typeof line !== 'string') continue;
      
      const cleanLine = removeAnsiCodes(line);
      const durationMatch = cleanLine.trim().match(/Duration\s+([\d.]+)s/);
      if (durationMatch) {
        console.log(chalk.gray(`  Duration: ${durationMatch[1]}s`));
        break;
      }
    }
    
    process.exit(0);
  } else {
    // No summary line found
    console.log(chalk.yellow('[WARN] Could not find test summary in output'));
    
    if (debug) {
      console.log('\nRaw output (last 15 lines):');
      const lastLines = lines.slice(-15);
      for (const line of lastLines) {
        console.log(chalk.gray(`[${line}]`));
      }
    }
    
    // If no summary found but exit code is 0, assume success
    if (result.exitCode === 0) {
      console.log(chalk.gray('Assuming success based on exit code...'));
      process.exit(0);
    } else {
      console.log(chalk.red(`[FAIL] Test execution failed (exit code: ${result.exitCode})`));
      process.exit(1);
    }
  }
} catch (error) {
  if (error.exitCode !== undefined) {
    const output = error.stdout + error.stderr;
    
    console.log(chalk.red(`[FAIL] Test execution failed (exit code: ${error.exitCode})`));
    
    if (debug) {
      console.log('\nError output:');
      console.log(chalk.gray(output));
    }
    
    process.exit(error.exitCode);
  } else {
    console.error(chalk.red(`[ERROR] Exception: ${error.message}`));
    process.exit(2);
  }
}
