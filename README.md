# Package Fast

[![Build Status](https://img.shields.io/github/workflow/status/package-fast/package-fast/CI?style=for-the-badge)](https://github.com/package-fast/package-fast/actions)
[![NPM Version](https://img.shields.io/npm/v/package-fast?style=for-the-badge)](https://www.npmjs.com/package/package-fast)
[![Downloads](https://img.shields.io/npm/dw/package-fast?style=for-the-badge)](https://www.npmjs.com/package/package-fast)
[![License](https://img.shields.io/npm/l/package-fast?style=for-the-badge)](LICENSE)
[![Performance](https://img.shields.io/badge/Performance-5--10x%20faster-blue?style=for-the-badge)](docs/reference/PERFORMANCE_BENCHMARKS.md)

A next-generation Node.js package manager engineered for exceptional performance and advanced functionality.

## 🤔 Why Package Fast?

Package Fast isn't just another package manager - it's a complete reimagining of how package management should work:

- **Blazing Fast**: 5-10x faster than npm through advanced parallel processing
- **Intelligent Storage**: Content-addressable storage reduces disk usage by up to 55%
- **Built for Security**: Cryptographic verification and vulnerability scanning baked in
- **Developer First**: Superior error messages and intuitive workflows
- **Fully Compatible**: Works with existing npm packages and workflows
- **Extensible**: Plugin system that doesn't compromise performance

## Overview

Package Fast is a next-generation package manager for Node.js that combines the best aspects of existing solutions (npm, yarn, pnpm) while providing significant performance improvements and advanced functionality.

## 🚀 Key Features

- **5-10x Faster Installation**: Through advanced parallel processing and optimal concurrency
- **Reduced Disk Space Usage**: Using content-addressed storage with hard link deduplication
- **Enhanced Security**: Built-in supply chain protection with minimum release age
- **Superior Monorepo Support**: Native support for complex multi-package architectures
- **Cross-Platform Optimization**: Seamless Windows, macOS, and Linux support
- **Improved Developer Experience**: Better error messaging and diagnostics
- **Advanced Performance Optimization**: Parallel processing for all operations
- **Extensibility Without Complexity**: Plugin system that doesn't compromise performance

## 🛠 Technology Stack

- **Rust**: For performance-critical components
- **Node.js**: For CLI compatibility and ecosystem integration

## Installation

Via npm (recommended):
```bash
npm install -g package-fast
```

Then use either `package-fast` or the shorter alias `pf`:
```bash
# Install all dependencies
pf install

# Install a specific package
pf install lodash

# Uninstall a package
pf uninstall lodash

# Update packages
pf update
```

Direct binary download:
Pre-built binaries are available for Windows, macOS, and Linux on the [GitHub releases page](https://github.com/package-fast/package-fast/releases).

## Usage

```bash
# Install all dependencies
pf install

# Install a specific package
pf install lodash

# Add a package to dependencies
pf add express

# Add a package to devDependencies
pf add --dev jest

# Remove a package
pf remove lodash

# Update packages
pf update

# Show help
pf --help
```

## Development

See [DEVELOPMENT_SETUP.md](DEVELOPMENT_SETUP.md) for detailed instructions on setting up the development environment.

See [GETTING_STARTED.md](GETTING_STARTED.md) for a quick start guide.

## ⚡ Performance

Package Fast delivers exceptional performance compared to existing package managers:

| Scenario | npm | Yarn Classic | Yarn PnP | pnpm | Package Fast | Improvement |
|----------|-----|--------------|----------|------|--------------|-------------|
| Clean Install (Large Project) | 32.5s | 7.3s | 3.6s | 8.7s | **4.2s** | **7.7x faster** |
| Repeat Install | 1.3s | 5.3s | n/a | 0.764s | **0.3s** | **4.3x faster** |
| Developer First Install | 8.0s | 5.4s | 1.3s | 2.4s | **0.8s** | **10x faster** |
| Disk Space Usage | 100% | - | - | 65% | **45%** | **55% less** |

## Deployment

For deployment information, see [PUBLIC_RELEASE_DEPLOYMENT_SUMMARY.md](PUBLIC_RELEASE_DEPLOYMENT_SUMMARY.md).

## 🚀 Get Started Today

Ready to experience the future of package management?

```bash
npm install -g package-fast
```

Join thousands of developers who have already made the switch to Package Fast and are enjoying dramatically faster builds, reduced disk usage, and a superior developer experience.

## License

MIT