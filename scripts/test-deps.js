import chalk from 'chalk';
import { execa } from 'execa';

console.log(chalk.cyan('Testing chalk and execa...'));
console.log(chalk.green('✓ Chalk is working!'));

try {
  const result = await execa('node', ['--version']);
  console.log(chalk.green(`✓ Execa is working! Node version: ${result.stdout}`));
  process.exit(0);
} catch (error) {
  console.error(chalk.red('✗ Execa failed:'), error.message);
  process.exit(1);
}
