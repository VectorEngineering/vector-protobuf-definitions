#!/usr/bin/env node

import { Command } from 'commander';
import chalk from 'chalk';
import { generate } from './commands/generate.js';

const program = new Command();

program
  .name('vector')
  .description('Vector API CLI tool')
  .version('0.1.0');

program
  .command('generate')
  .description('Generate API client code from OpenAPI specification')
  .requiredOption('-i, --input <path>', 'Path to OpenAPI specification file')
  .requiredOption('-o, --output <path>', 'Output directory for generated code')
  .option('-t, --template <path>', 'Path to custom templates directory')
  .action(generate);

if (!process.argv.slice(2).length) {
  program.outputHelp();
} else {
  program.parse();
} 