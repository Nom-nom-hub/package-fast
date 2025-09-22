#!/usr/bin/env node

const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

console.log('Publishing Package Fast...');

try {
  // Check if we're on the main branch
  const currentBranch = execSync('git rev-parse --abbrev-ref HEAD', { encoding: 'utf-8' }).trim();
  if (currentBranch !== 'main' && currentBranch !== 'master') {
    console.warn(`Warning: You are on branch '${currentBranch}', not main/master. Consider switching before publishing.`);
  }

  // Get current version from package.json
  const packageJsonPath = path.join(__dirname, '..', 'package.json');
  const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, 'utf-8'));
  const version = packageJson.version;
  
  console.log(`Publishing version ${version}...`);

  // Run build first
  console.log('Building release...');
  execSync('node scripts/build-release.js', { stdio: 'inherit' });

  // Publish to npm
  console.log('Publishing to npm...');
  execSync('npm publish', { stdio: 'inherit' });

  console.log('Package published successfully!');
} catch (error) {
  console.error('Publish failed:', error.message);
  process.exit(1);
}