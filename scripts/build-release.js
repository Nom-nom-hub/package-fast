#!/usr/bin/env node

const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

console.log('Building Package Fast for release...');

try {
  // Create dist directory if it doesn't exist
  const distDir = path.join(__dirname, '..', 'dist');
  if (!fs.existsSync(distDir)) {
    fs.mkdirSync(distDir, { recursive: true });
  }

  // Build the Rust CLI
  console.log('Building Rust CLI...');
  execSync('cargo build --release', { stdio: 'inherit', cwd: path.join(__dirname, '..') });

  // Copy the built binary to dist
  const targetDir = process.platform === 'win32' ? 'target/release/package-fast.exe' : 'target/release/package-fast';
  const sourceBinary = path.join(__dirname, '..', targetDir);
  const destBinary = path.join(distDir, process.platform === 'win32' ? 'package-fast.exe' : 'package-fast');
  
  if (fs.existsSync(sourceBinary)) {
    fs.copyFileSync(sourceBinary, destBinary);
    console.log(`Copied binary to ${destBinary}`);
  } else {
    console.error(`Built binary not found at ${sourceBinary}`);
    process.exit(1);
  }

  // Build TypeScript files
  console.log('Building TypeScript files...');
  execSync('npx tsc', { stdio: 'inherit' });

  console.log('Build completed successfully!');
} catch (error) {
  console.error('Build failed:', error.message);
  process.exit(1);
}