#!/usr/bin/env node
// Harness Engineering 代码质量自动修复脚本
// 用法: node scripts/fix-code-quality.js [--skip-type-check] [--dry-run]

import chalk from 'chalk';
import { execa } from 'execa';

const args = process.argv.slice(2);
const skipTypeCheck = args.includes('--skip-type-check');
const dryRun = args.includes('--dry-run');

const startTime = Date.now();

console.log(chalk.cyan('========================================'));
console.log(chalk.cyan('  OPC-HARNESS Code Quality Auto-Fix'));
console.log('');

if (dryRun) {
  console.log(chalk.yellow('[DRY RUN] The following actions will be performed:'));
  console.log('');
}

try {
  // Step 1: TypeScript type check
  console.log(chalk.yellow('[Step 1/3] Running TypeScript Type Check...'));
  
  if (!skipTypeCheck) {
    if (dryRun) {
      console.log(chalk.gray('  Would run: npx tsc --noEmit'));
    } else {
      try {
        await execa('npx', ['tsc', '--noEmit'], {
          stdio: 'pipe'
        });
        console.log(chalk.green('  [PASS] TypeScript type check passed'));
      } catch (error) {
        console.error(chalk.red('  [ERROR] TypeScript type check failed!'));
        console.error(chalk.gray(error.stdout || error.stderr));
        console.log('');
        console.log(chalk.yellow('Please fix TypeScript errors before continuing.'));
        process.exit(1);
      }
    }
  } else {
    console.log(chalk.yellow('  [SKIP] TypeScript check skipped'));
  }

  // Step 2: Prettier formatting
  console.log('');
  console.log(chalk.yellow('[Step 2/3] Formatting Code with Prettier...'));
  
  if (dryRun) {
    console.log(chalk.gray('  Would run: npm run format'));
  } else {
    try {
      await execa('npm', ['run', 'format'], {
        stdio: 'pipe'
      });
      console.log(chalk.green('  [OK] Code formatted successfully'));
    } catch (error) {
      console.error(chalk.red('  [ERROR] Prettier formatting failed'));
      process.exit(1);
    }
  }

  // Step 3: ESLint auto-fix (if available)
  console.log('');
  console.log(chalk.yellow('[Step 3/3] Attempting ESLint Auto-Fix...'));
  
  if (dryRun) {
    console.log(chalk.gray('  Would run: npm run lint:fix'));
  } else {
    try {
      const result = await execa('npm', ['run', 'lint:fix'], {
        stdio: 'pipe'
      });
      
      if (result.exitCode === 0) {
        console.log(chalk.green('  [OK] ESLint auto-fix completed'));
      } else {
        console.log(chalk.yellow('  [WARN] ESLint has some issues that couldn\'t be auto-fixed'));
      }
    } catch (error) {
      console.log(chalk.yellow('  [WARN] ESLint not available, skipping auto-fix'));
    }
  }

  // Summary
  const endTime = Date.now();
  const duration = ((endTime - startTime) / 1000).toFixed(2);

  console.log('');
  console.log(chalk.cyan('========================================'));
  console.log(chalk.cyan('  Fix Summary'));
  console.log(chalk.cyan('========================================'));
  console.log('');
  console.log(chalk.cyan(`  Duration: ${duration} seconds`));
  console.log(chalk.green('  Status: Completed'));
  console.log('');

  if (!dryRun) {
    console.log(chalk.blue('Next steps:'));
    console.log(chalk.gray('  1. Run \'node scripts/harness-check.js\' to verify overall health'));
    console.log(chalk.gray('  2. Review git diff to see what changed'));
    console.log(chalk.gray('  3. Commit your changes'));
    console.log('');
  }

  process.exit(0);
} catch (error) {
  console.error(chalk.red(`[ERROR] Unexpected error: ${error.message}`));
  process.exit(1);
}
