#!/usr/bin/env node
// Harness Engineering 架构健康检查脚本
// 用法: node scripts/harness-check.js [--verbose] [--json]
// 版本: 3.0 (Node.js 版本)

import chalk from 'chalk';
import { execa } from 'execa';
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const projectRoot = path.dirname(__dirname);

const args = process.argv.slice(2);
const verbose = args.includes('--verbose');
const jsonOutput = args.includes('--json');

// =============================================
// 配置区域 - 集中管理所有配置
// =============================================

const config = {
  scoreWeights: {
    TypeScript: 22,
    ESLint: 17,
    Prettier: 11,
    Rust: 28,
    RustTests: 22,
    TSTests: 22,
    Dependencies: 6,
    Directory: 6
  },
  
  requiredDirs: [
    'src',
    'src-tauri',
    'scripts'
  ],
  
  requiredFiles: [
    'package.json',
    'package-lock.json',
    'src-tauri/Cargo.toml',
    'src-tauri/Cargo.lock'
  ],
  
  timeouts: {
    TSTests: 30  // seconds
  }
};

const state = {
  score: 100,
  issues: [],
  cargoAvailable: false,
  npmAvailable: false,
  startTime: Date.now()
};

// =============================================
// 工具函数区域 - 通用辅助函数
// =============================================

function writeHeader(text) {
  console.log('');
  console.log(chalk.cyan('========================================'));
  console.log(chalk.cyan(`  ${text}`));
  console.log('');
}

function writeCheckStart(name, index) {
  console.log(chalk.yellow(`[${index}] ${name}...`));
}

function writeCheckResult(status, message) {
  const colors = {
    PASS: 'green',
    FAIL: 'red',
    WARN: 'yellow',
    FIX: 'blue'
  };
  
  console.log(chalk[colors[status]](`  [${status}] ${message}`));
}

function addIssue(type, severity, message, scorePenalty = 0) {
  state.issues.push({
    type,
    severity,
    message
  });
  
  if (scorePenalty > 0) {
    state.score -= scorePenalty;
  }
}

async function testCommandAvailable(commandName) {
  try {
    await execa(commandName, ['--version'], { stdio: 'pipe' });
    return true;
  } catch (error) {
    return false;
  }
}

// =============================================
// 检查函数区域 - 各项独立检查逻辑
// =============================================

async function invokeTypeScriptCheck() {
  writeCheckStart('TypeScript Type Checking', '1/8');
  
  try {
    await execa('npx', ['tsc', '--noEmit'], {
      cwd: projectRoot,
      stdio: 'pipe'
    });
    writeCheckResult('PASS', 'TypeScript type checking passed');
  } catch (error) {
    writeCheckResult('FAIL', 'TypeScript type checking failed');
    addIssue('TypeScript', 'Error', 'Type check failed', config.scoreWeights.TypeScript);
    
    if (verbose) {
      console.log(chalk.gray(error.stdout || error.stderr));
    }
  }
}

async function invokeESLintCheck() {
  writeCheckStart('ESLint Code Quality Check', '2/8');
  
  try {
    await execa('npm', ['run', 'lint'], {
      cwd: projectRoot,
      stdio: 'pipe'
    });
    writeCheckResult('PASS', 'ESLint check passed');
  } catch (error) {
    writeCheckResult('FAIL', 'ESLint check has warnings/errors');
    addIssue('ESLint', 'Error', 'Code style violation', config.scoreWeights.ESLint);
    
    if (verbose) {
      console.log(chalk.gray(error.stdout || error.stderr));
    }
  }
}

async function invokePrettierCheck() {
  writeCheckStart('Prettier Formatting Check', '3/8');
  
  try {
    await execa('npm', ['run', 'format:check'], {
      cwd: projectRoot,
      stdio: 'pipe'
    });
    writeCheckResult('PASS', 'Prettier formatting passed');
  } catch (error) {
    writeCheckResult('FAIL', 'Prettier formatting failed');
    addIssue('Prettier', 'Error', 'Code format not standard', config.scoreWeights.Prettier);
  }
}

async function invokeRustCompilationCheck() {
  writeCheckStart('Rust Compilation Check', '4/8');
  
  // Check if cargo is available
  state.cargoAvailable = await testCommandAvailable('cargo');
  
  if (!state.cargoAvailable) {
    writeCheckResult('WARN', 'Cannot execute Cargo check (Rust may not be installed)');
    addIssue('Rust', 'Warning', 'Rust environment not ready');
    return;
  }
  
  // Run cargo check
  try {
    await execa('cargo', ['check'], {
      cwd: path.join(projectRoot, 'src-tauri'),
      stdio: 'pipe'
    });
    writeCheckResult('PASS', 'Rust compilation check passed');
  } catch (error) {
    writeCheckResult('FAIL', 'Rust compilation check failed');
    addIssue('Rust', 'Error', 'Compilation error', config.scoreWeights.Rust);
    
    if (verbose) {
      console.log(chalk.gray("Run 'cd src-tauri && cargo check' for details"));
    }
  }
}

async function invokeRustTestsCheck() {
  writeCheckStart('Rust Unit Tests Check', '5/8');
  
  if (!state.cargoAvailable) {
    writeCheckResult('WARN', 'Cannot execute Rust tests (Cargo not available)');
    addIssue('Rust Tests', 'Warning', 'Rust environment not ready');
    return;
  }
  
  console.log(chalk.gray('  Running Rust unit tests...'));
  
  try {
    const rustTestScript = path.join(__dirname, 'harness-rust-tests.js');
    
    if (!fs.existsSync(rustTestScript)) {
      writeCheckResult('FAIL', `Rust test script not found: ${rustTestScript}`);
      addIssue('Rust Tests', 'Error', 'Test script missing', config.scoreWeights.RustTests);
      return;
    }
    
    // Execute the dedicated Rust test script
    const result = await execa('node', [rustTestScript], {
      cwd: projectRoot,
      stdio: 'pipe'
    });
    
    const testOutput = result.stdout + result.stderr;
    
    // Parse results
    const passMatch = testOutput.match(/\[PASS\] All (\d+) Rust tests passed/);
    if (passMatch) {
      const testCount = passMatch[1];
      writeCheckResult('PASS', `All ${testCount} Rust tests passed`);
      
      if (verbose) {
        console.log(chalk.gray(testOutput));
      }
    } else if (testOutput.includes('[FAIL]') || testOutput.includes('[ERROR]')) {
      writeCheckResult('FAIL', 'Rust tests encountered an error');
      addIssue('Rust Tests', 'Error', 'Test execution error', config.scoreWeights.RustTests);
      
      if (verbose) {
        console.log(chalk.gray(testOutput));
      }
    } else if (result.exitCode !== 0) {
      writeCheckResult('FAIL', `Rust test execution failed (exit code: ${result.exitCode})`);
      addIssue('Rust Tests', 'Error', 'Test execution failed', config.scoreWeights.RustTests);
      
      if (verbose) {
        console.log(chalk.gray(testOutput));
      }
    } else {
      writeCheckResult('WARN', 'Could not parse Rust test results');
      addIssue('Rust Tests', 'Warning', 'Unknown test result', 10);
      
      if (verbose) {
        console.log(chalk.gray(testOutput));
      }
    }
  } catch (error) {
    writeCheckResult('FAIL', `Rust test execution failed: ${error.message}`);
    addIssue('Rust Tests', 'Error', 'Test execution exception', config.scoreWeights.RustTests);
    
    if (verbose) {
      console.log(chalk.red(`Exception details: ${error.message}`));
    }
  }
}

async function invokeTSTestsCheck() {
  writeCheckStart('TypeScript Unit Tests Check', '6/8');
  
  state.npmAvailable = await testCommandAvailable('npm');
  const nodeModulesExists = fs.existsSync(path.join(projectRoot, 'node_modules'));
  
  if (!state.npmAvailable || !nodeModulesExists) {
    writeCheckResult('WARN', 'Cannot execute TS tests (npm/node_modules not available)');
    addIssue('TS Tests', 'Warning', 'TS environment not ready');
    return;
  }
  
  console.log(chalk.gray('  Running TypeScript unit tests...'));
  
  try {
    const tsTestScript = path.join(__dirname, 'harness-ts-tests.js');
    
    if (!fs.existsSync(tsTestScript)) {
      writeCheckResult('FAIL', `TS test script not found: ${tsTestScript}`);
      addIssue('TS Tests', 'Error', 'Test script missing', config.scoreWeights.TSTests);
      return;
    }
    
    // Execute the dedicated TS test script with --skip-e2e flag
    const result = await execa('node', [tsTestScript, '--skip-e2e'], {
      cwd: projectRoot,
      stdio: 'pipe',
      timeout: config.timeouts.TSTests * 1000
    });
    
    const testOutput = result.stdout + result.stderr;
    
    // Simplified logic: Rely on exit code first, then parse stats for display
    if (result.exitCode === 0) {
      // Success - extract and display statistics from dedicated script output
      const match = testOutput.match(/\[PASS\].*\((\d+)\s+files.*?(\d+)\s+tests\)/);
      if (match) {
        const testFiles = match[1];
        const totalTests = match[2];
        writeCheckResult('PASS', `All TS tests passed (${testFiles} files, ${totalTests} tests)`);
      } else {
        const filesMatch = testOutput.match(/\[PASS\].*\((\d+)\s+files\)/);
        if (filesMatch) {
          const testFiles = filesMatch[1];
          writeCheckResult('PASS', `All TS tests passed (${testFiles} files)`);
        } else {
          writeCheckResult('PASS', 'TS tests completed');
        }
      }
      
      if (verbose) {
        console.log(chalk.gray(testOutput));
      }
    } else {
      // Failure - rely on exit code, show simple summary
      writeCheckResult('FAIL', `TS test execution failed (exit code: ${result.exitCode})`);
      addIssue('TS Tests', 'Error', 'Test execution failed', config.scoreWeights.TSTests);
      
      if (verbose) {
        // Show only the summary part for debugging
        const lines = testOutput.split('\n').filter(line => 
          line.match(/\[FAIL\]|\[PASS\]|\[WARN\]|Duration/)
        );
        console.log(chalk.gray(lines.join('\n')));
      }
    }
  } catch (error) {
    writeCheckResult('FAIL', `TS test execution failed: ${error.message}`);
    addIssue('TS Tests', 'Error', 'Test execution exception', config.scoreWeights.TSTests);
    
    if (verbose) {
      console.log(chalk.red(`Exception details: ${error.message}`));
    }
  }
}

function invokeDependencyCheck() {
  writeCheckStart('Dependency Integrity Check', '7/8');
  
  let depIssues = 0;
  
  for (const file of config.requiredFiles) {
    const filePath = path.join(projectRoot, file);
    if (!fs.existsSync(filePath)) {
      const severity = file.endsWith('.json') ? 'Error' : 'Warning';
      console.log(chalk[severity === 'Error' ? 'red' : 'yellow'](`  [${severity}] Required file missing: ${file}`));
      depIssues++;
    }
  }
  
  if (depIssues === 0) {
    writeCheckResult('PASS', 'All dependencies present');
  } else {
    writeCheckResult('INFO', `Found ${depIssues} dependency issue(s)`);
    addIssue('Dependencies', 'Warning', `${depIssues} missing file(s)`, config.scoreWeights.Dependencies);
  }
}

function invokeDirectoryCheck() {
  writeCheckStart('Directory Structure Check', '8/8');
  
  let dirIssues = 0;
  for (const dir of config.requiredDirs) {
    const dirPath = path.join(projectRoot, dir);
    if (!fs.existsSync(dirPath)) {
      console.log(chalk.yellow(`  [WARN] Required directory missing: ${dir}`));
      dirIssues++;
    }
  }
  
  if (dirIssues === 0) {
    writeCheckResult('PASS', 'Directory structure valid');
  } else {
    writeCheckResult('INFO', `Found ${dirIssues} directory issue(s)`);
    addIssue('Directory', 'Warning', `${dirIssues} missing dir(s)`, config.scoreWeights.Directory);
  }
}

// =============================================
// 结果汇总区域 - 输出最终报告
// =============================================

function showSummary() {
  console.log('');
  console.log(chalk.cyan('========================================'));
  console.log(chalk.cyan('  Health Check Summary'));
  console.log('');
  
  const finalScore = Math.max(0, state.score);
  const scoreColor = finalScore >= 80 ? 'green' : finalScore >= 60 ? 'yellow' : 'red';
  
  console.log(chalk[scoreColor](`  Overall Score: ${finalScore} / 100`));
  console.log(chalk[state.issues.length > 0 ? 'yellow' : 'green'](`  Total Issues: ${state.issues.length}`));
  console.log('');
  
  if (state.issues.length > 0) {
    console.log(chalk.yellow('  Issues Breakdown:'));
    for (const issue of state.issues) {
      const severityColor = issue.severity === 'Error' ? 'red' : 'yellow';
      console.log(chalk[severityColor](`    - [${issue.severity}] ${issue.type}: ${issue.message}`));
    }
    
    console.log('');
  } else {
    console.log(chalk.green('  Status: All checks passed!'));
    console.log('');
  }
  
  const duration = Date.now() - state.startTime;
  const minutes = Math.floor(duration / 60000);
  const seconds = Math.floor((duration % 60000) / 1000);
  console.log(chalk.gray(`  Duration: ${minutes}m${seconds}s`));
  console.log('');
  console.log(chalk.cyan('========================================'));
}

// =============================================
// 主执行流程 - 按顺序执行所有检查
// =============================================

async function main() {
  writeHeader('OPC-HARNESS Architecture Health Check');
  
  // Execute all checks in sequence
  await invokeTypeScriptCheck();       // 1/8
  await invokeESLintCheck();           // 2/8
  await invokePrettierCheck();         // 3/8
  await invokeRustCompilationCheck();  // 4/8
  await invokeRustTestsCheck();        // 5/8
  await invokeTSTestsCheck();          // 6/8
  invokeDependencyCheck();             // 7/8
  invokeDirectoryCheck();              // 8/8
  
  showSummary();
  
  // Exit with appropriate code
  const finalScore = Math.max(0, state.score);
  process.exit(finalScore >= 80 ? 0 : 1);
}

main().catch(error => {
  console.error(chalk.red(`[ERROR] Unexpected error: ${error.message}`));
  process.exit(1);
});
