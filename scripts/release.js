#!/usr/bin/env node

const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

// Simple release script
console.log('Package Fast Release Script');

// Get version argument
const version = process.argv[2];
if (!version) {
  console.error('Usage: node scripts/release.js <version>');
  console.error('Example: node scripts/release.js 1.0.0');
  process.exit(1);
}

if (!version.match(/^\\d+\\.\\d+\\.\\d+/)) {
  console.error('Invalid version format. Use semantic versioning (e.g., 1.0.0)');
  process.exit(1);
}

console.log(`Preparing release for version ${version}...`);

try {
  // Update package.json
  const packageJsonPath = path.join(__dirname, '..', 'package.json');
  const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, 'utf-8'));
  const oldVersion = packageJson.version;
  packageJson.version = version;
  fs.writeFileSync(packageJsonPath, JSON.stringify(packageJson, null, 2) + '\n');
  console.log(`Updated package.json from ${oldVersion} to ${version}`);

  // Update Cargo.toml files
  const cargoFiles = [
    'Cargo.toml',
    'crates/cli/Cargo.toml',
    'crates/core/Cargo.toml',
    'crates/security/Cargo.toml'
  ];

  for (const cargoFile of cargoFiles) {
    const fullPath = path.join(__dirname, '..', cargoFile);
    if (fs.existsSync(fullPath)) {
      let content = fs.readFileSync(fullPath, 'utf-8');
      const versionRegex = /(version\\s*=\\s*)"[^"]+"/;
      if (versionRegex.test(content)) {
        content = content.replace(versionRegex, `$1"${version}"`);
        fs.writeFileSync(fullPath, content);
        console.log(`Updated ${cargoFile} to version ${version}`);
      }
    }
  }

  // Commit changes
  execSync('git add package.json Cargo.toml crates/*/Cargo.toml', { stdio: 'inherit' });
  execSync(`git commit -m "Release v${version}"`, { stdio: 'inherit' });
  
  // Create and push tag
  execSync(`git tag -a v${version} -m "Release v${version}"`, { stdio: 'inherit' });
  execSync(`git push origin v${version}`, { stdio: 'inherit' });
  
  console.log(`Release v${version} created and pushed successfully!`);
  console.log('GitHub Actions will now build and publish the release.');
} catch (error) {
  console.error('Release failed:', error.message);
  process.exit(1);
}