# Package Fast Deployment Guide

## Overview

This guide provides instructions for deploying Package Fast for public release. Package Fast is a hybrid Node.js/Rust application that requires building both components for distribution.

## Prerequisites

Before deploying, ensure you have:

1. Node.js (v16 or later)
2. Rust toolchain (installed via rustup)
3. npm account with publishing permissions
4. GitHub account with repository access
5. Appropriate environment variables set:
   - `NPM_TOKEN` - for publishing to npm
   - `GITHUB_TOKEN` - for creating GitHub releases

## Building for Release

The build process creates binaries for all supported platforms:

```bash
# From the project root
node scripts/build-release.js
```

This script:
1. Compiles the Rust components in release mode
2. Copies binaries to the `dist/` directory
3. Compiles TypeScript files

## Manual Publishing

To manually publish a new version:

1. Update version numbers in all `package.json` and `Cargo.toml` files
2. Commit version changes
3. Create a git tag: `git tag -a v1.0.0 -m "Release v1.0.0"`
4. Push the tag: `git push origin v1.0.0`
5. Run the publish script: `node scripts/publish.js`

## Automated Publishing

Package Fast uses GitHub Actions for automated releases:

1. When a tag matching the pattern `v*` is pushed to the repository
2. GitHub Actions workflow builds binaries for all platforms
3. Creates a GitHub release with binaries
4. Publishes the package to npm

## Distribution Channels

Package Fast is distributed through:

1. **npm** - Primary distribution channel
   ```bash
   npm install -g package-fast
   ```

2. **GitHub Releases** - Direct binary downloads for all platforms
   - Windows (x64)
   - macOS (Intel and Apple Silicon)
   - Linux (x64)

3. **Package Manager Integration** - Future planned integrations:
   - Homebrew for macOS
   - Chocolatey for Windows
   - APT repository for Ubuntu/Debian

## Versioning Strategy

Package Fast follows Semantic Versioning (SemVer):

- MAJOR version for incompatible API changes
- MINOR version for backward-compatible functionality
- PATCH version for backward-compatible bug fixes

Version numbers should be synchronized across:
- `package.json` (Node.js package)
- `crates/cli/Cargo.toml` (Rust CLI)
- `crates/core/Cargo.toml` (Rust core)
- `crates/security/Cargo.toml` (Rust security)

## Testing Release Builds

Before publishing, always test release builds:

1. Install from local build:
   ```bash
   npm install -g ./package-fast-*.tgz
   ```

2. Verify installation:
   ```bash
   package-fast --version
   package-fast --help
   ```

3. Test basic functionality:
   ```bash
   package-fast install
   ```

## Troubleshooting

### Build Failures

- Ensure Rust toolchain is up to date: `rustup update`
- Clear Cargo cache: `cargo clean`
- Check for platform-specific compilation issues

### Publishing Issues

- Verify npm token permissions
- Check npm package name availability
- Ensure two-factor authentication is properly configured

### GitHub Release Issues

- Verify GitHub personal access token permissions
- Check tag format matches expected pattern
- Confirm workflow file syntax is correct