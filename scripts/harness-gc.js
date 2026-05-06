#!/usr/bin/env node
// Harness Engineering Garbage Collection Script
// Purpose: Clean up outdated documents, dead code, and redundant configurations
// Usage: node scripts/harness-gc.js [--dry-run] [--verbose] [--force]

import chalk from 'chalk';
import { execa } from 'execa';
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';
import readline from 'readline';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const projectRoot = path.dirname(__dirname);

const args = process.argv.slice(2);
const dryRun = args.includes('--dry-run');
const verbose = args.includes('--verbose');
const force = args.includes('--force');

const startTime = Date.now();
let deletedCount = 0;
let skippedCount = 0;
let totalSize = 0;

// Create readline interface for user input
const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout
});

function askQuestion(query) {
  return new Promise((resolve) => {
    rl.question(query, (answer) => {
      resolve(answer);
    });
  });
}

console.log(chalk.cyan('========================================'));
console.log(chalk.cyan('  OPC-HARNESS Garbage Collection'));
console.log('');

if (dryRun) {
  console.log(chalk.yellow('[INFO] DRY RUN MODE - No files will be deleted'));
  console.log('');
}

// Helper function: Safe file removal
async function removeSafeFile(filePath) {
  if (!fs.existsSync(filePath)) {
    return;
  }
  
  const stats = fs.statSync(filePath);
  const fileSizeKB = Math.round(stats.size / 1024 * 100) / 100;
  
  if (dryRun) {
    console.log(chalk.gray(`  [DRY RUN] Would delete: ${filePath} (${fileSizeKB} KB)`));
    deletedCount++;
  } else {
    if (force) {
      fs.unlinkSync(filePath);
      console.log(chalk.green(`  [DELETED] ${filePath} (${fileSizeKB} KB)`));
      totalSize += stats.size;
      deletedCount++;
    } else {
      const confirm = await askQuestion(`  Confirm delete: ${filePath}? (y/n) `);
      if (confirm.toLowerCase() === 'y') {
        fs.unlinkSync(filePath);
        console.log(chalk.green(`  [DELETED] ${filePath} (${fileSizeKB} KB)`));
        totalSize += stats.size;
        deletedCount++;
      } else {
        console.log(chalk.yellow(`  [SKIPPED] ${filePath}`));
        skippedCount++;
      }
    }
  }
}

// Helper function: Remove directory recursively
async function removeDirectory(dirPath, description) {
  if (!fs.existsSync(dirPath)) {
    return;
  }
  
  // Calculate directory size
  let dirSize = 0;
  function calculateSize(dir) {
    const files = fs.readdirSync(dir);
    for (const file of files) {
      const filePath = path.join(dir, file);
      const stats = fs.statSync(filePath);
      if (stats.isDirectory()) {
        calculateSize(filePath);
      } else {
        dirSize += stats.size;
      }
    }
  }
  
  try {
    calculateSize(dirPath);
  } catch (error) {
    // Ignore errors in size calculation
  }
  
  const sizeMB = Math.round(dirSize / (1024 * 1024) * 100) / 100;
  
  if (dryRun) {
    console.log(chalk.gray(`  [DRY RUN] Would clean directory: ${description} (${sizeMB} MB)`));
    deletedCount++;
  } else {
    if (force) {
      fs.rmSync(dirPath, { recursive: true, force: true });
      console.log(chalk.green(`  [CLEANED] ${description}`));
      deletedCount++;
    } else {
      const confirm = await askQuestion(`  Confirm cleaning build directory ${description}? (y/n) `);
      if (confirm.toLowerCase() === 'y') {
        fs.rmSync(dirPath, { recursive: true, force: true });
        console.log(chalk.green(`  [CLEANED] ${description}`));
        deletedCount++;
      } else {
        console.log(chalk.yellow(`  [SKIPPED] ${description}`));
        skippedCount++;
      }
    }
  }
}

try {
  // 1. Clean up temporary files
  console.log(chalk.yellow('[1/7] Cleaning temporary files...'));
  const tempPatterns = ['*.tmp', '*.bak', '*.old', '*.log', '*~', '.DS_Store', 'Thumbs.db'];
  
  for (const pattern of tempPatterns) {
    // Use glob-like search with execa
    try {
      const result = await execa('find', ['.'], {
        cwd: projectRoot,
        stdio: 'pipe'
      });
      
      const files = result.stdout.split('\n').filter(f => {
        const ext = path.extname(f);
        const base = path.basename(f);
        
        if (pattern === '*.tmp') return ext === '.tmp';
        if (pattern === '*.bak') return ext === '.bak';
        if (pattern === '*.old') return ext === '.old';
        if (pattern === '*.log') return ext === '.log';
        if (pattern === '*~') return base.endsWith('~');
        if (pattern === '.DS_Store') return base === '.DS_Store';
        if (pattern === 'Thumbs.db') return base === 'Thumbs.db';
        return false;
      });
      
      for (const file of files) {
        if (file) {
          await removeSafeFile(path.join(projectRoot, file));
        }
      }
    } catch (error) {
      // find command may not work on Windows, skip silently
    }
  }

  // 2. Clean up Node.js build artifacts
  console.log(chalk.yellow('[2/7] Cleaning Node.js build artifacts...'));
  const buildArtifacts = ['dist', 'build', '.vite', 'tsconfig.tsbuildinfo'];
  
  for (const artifact of buildArtifacts) {
    const artifactPath = path.join(projectRoot, artifact);
    if (fs.existsSync(artifactPath)) {
      await removeDirectory(artifactPath, artifact);
    }
  }

  // 3. Clean up Rust build artifacts
  console.log(chalk.yellow('[3/7] Cleaning Rust build artifacts...'));
  const rustArtifactPath = path.join(projectRoot, 'src-tauri', 'target');
  if (fs.existsSync(rustArtifactPath)) {
    await removeDirectory(rustArtifactPath, 'src-tauri/target');
  }

  // 4. Scan for unused dependencies (simple check)
  console.log(chalk.yellow('[4/7] Scanning for unused dependencies...'));
  try {
    const packageJsonPath = path.join(projectRoot, 'package.json');
    const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, 'utf-8'));
    
    const allDeps = [];
    if (packageJson.dependencies) {
      allDeps.push(...Object.keys(packageJson.dependencies));
    }
    if (packageJson.devDependencies) {
      allDeps.push(...Object.keys(packageJson.devDependencies));
    }
    
    const nodeModulesPath = path.join(projectRoot, 'node_modules');
    if (fs.existsSync(nodeModulesPath)) {
      console.log(chalk.cyan(`  [INFO] Found ${allDeps.length} dependency packages`));
      console.log(chalk.gray('  [TIP] Use depcheck tool for deeper analysis'));
    }
  } catch (error) {
    console.log(chalk.yellow('  [WARN] Cannot analyze dependencies'));
  }

  // 5. Check for outdated documents (>30 days old)
  console.log(chalk.yellow('[5/7] Checking outdated documents...'));
  const cutoffDate = new Date();
  cutoffDate.setDate(cutoffDate.getDate() - 30);
  
  const docDirs = [
    'scripts/context-engineering/execution-logs',
    'scripts/context-engineering/decision-records'
  ];
  
  for (const dir of docDirs) {
    const dirPath = path.join(projectRoot, dir);
    if (fs.existsSync(dirPath)) {
      const files = fs.readdirSync(dirPath);
      for (const file of files) {
        const filePath = path.join(dirPath, file);
        const stats = fs.statSync(filePath);
        
        if (stats.isFile() && !file.endsWith('.md') && stats.mtime < cutoffDate) {
          const daysOld = Math.floor((Date.now() - stats.mtime.getTime()) / (1000 * 60 * 60 * 24));
          console.log(chalk.yellow(`  [OUTDATED] ${file} (${daysOld} days old)`));
          await removeSafeFile(filePath);
        }
      }
    }
  }

  // 6. Scan code comments (TODO/FIXME/HACK)
  console.log(chalk.yellow('[6/7] Scanning code comment markers...'));
  let todoCount = 0;
  let fixmeCount = 0;
  let hackCount = 0;
  
  const sourceDirs = ['src', 'src-tauri/src'];
  const extensions = ['.ts', '.tsx', '.rs'];
  
  for (const srcDir of sourceDirs) {
    const dirPath = path.join(projectRoot, srcDir);
    if (!fs.existsSync(dirPath)) continue;
    
    function scanDirectory(dir) {
      const files = fs.readdirSync(dir);
      for (const file of files) {
        const filePath = path.join(dir, file);
        const stats = fs.statSync(filePath);
        
        if (stats.isDirectory()) {
          scanDirectory(filePath);
        } else if (extensions.some(ext => file.endsWith(ext))) {
          try {
            const content = fs.readFileSync(filePath, 'utf-8');
            todoCount += (content.match(/TODO/g) || []).length;
            fixmeCount += (content.match(/FIXME/g) || []).length;
            hackCount += (content.match(/HACK/g) || []).length;
          } catch (error) {
            // Skip files that can't be read
          }
        }
      }
    }
    
    scanDirectory(dirPath);
  }
  
  console.log(chalk.cyan(`  TODO: ${todoCount}`));
  console.log(chalk.cyan(`  FIXME: ${fixmeCount}`));
  console.log(chalk.yellow(`  HACK: ${hackCount}`));
  
  if (fixmeCount > 0 || hackCount > 0) {
    console.log(chalk.gray('  [TIP] Prioritize fixing FIXME and HACK markers'));
  }

  // 7. Verify critical file integrity
  console.log(chalk.yellow('[7/7] Verifying critical files...'));
  const criticalFiles = [
    'package.json',
    'src-tauri/Cargo.toml',
    'src/App.tsx',
    'src-tauri/src/main.rs',
    'AGENTS.md'
  ];
  
  const missingFiles = [];
  for (const file of criticalFiles) {
    const filePath = path.join(projectRoot, file);
    if (!fs.existsSync(filePath)) {
      missingFiles.push(file);
      console.log(chalk.red(`  [MISSING] Critical file: ${file}`));
    }
  }
  
  if (missingFiles.length === 0) {
    console.log(chalk.green('  [PASS] All critical files present'));
  }

  // Summary Report
  const endTime = Date.now();
  const duration = ((endTime - startTime) / 1000).toFixed(2);
  const totalSizeMB = Math.round(totalSize / (1024 * 1024) * 100) / 100;

  console.log('');
  console.log(chalk.cyan('========================================'));
  console.log(chalk.cyan('  Garbage Collection Report'));
  console.log(chalk.cyan('========================================'));
  console.log('');
  
  console.log(chalk.cyan(`  Duration: ${duration} seconds`));
  console.log(chalk[deletedCount === 0 ? 'green' : 'yellow'](`  Files Deleted: ${deletedCount}`));
  console.log(chalk.yellow(`  Files Skipped: ${skippedCount}`));
  console.log(chalk.green(`  Space Freed: ${totalSizeMB} MB`));
  console.log('');
  
  if (dryRun) {
    console.log(chalk.yellow('[INFO] Remove --dry-run parameter to actually delete files'));
  } else {
    console.log(chalk.green('[SUCCESS] Garbage collection complete!'));
  }
  
  console.log('');
  console.log(chalk.blue('Recommendations:'));
  console.log(chalk.gray('  1. Run this script regularly to keep project clean'));
  console.log(chalk.gray('  2. Use \'node scripts/harness-check.js\' to verify architecture health'));
  console.log(chalk.gray('  3. Fix FIXME and HACK markers promptly'));
  console.log('');
  
  rl.close();
  process.exit(0);
} catch (error) {
  console.error(chalk.red(`[ERROR] Unexpected error: ${error.message}`));
  rl.close();
  process.exit(1);
}

