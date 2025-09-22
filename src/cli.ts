#!/usr/bin/env node

import { program } from 'commander';
import chalk = require('chalk');

program
  .version('0.1.0')
  .description('Package Fast - A very fast Node.js package manager')
  .option('-v, --verbose', 'Enable verbose output')
  .command('install [packages...]', 'Install packages')
  .command('uninstall [packages...]', 'Uninstall packages')
  .command('update [packages...]', 'Update packages')
  .parse(process.argv);

if (!process.argv.slice(2).length) {
  console.log(chalk.green('Package Fast - A very fast Node.js package manager'));
  console.log('Use --help to see available commands.');
}