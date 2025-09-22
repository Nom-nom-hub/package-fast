<div align="center">
  <img src="https://raw.githubusercontent.com/package-fast/package-fast/main/docs/assets/logo.png" alt="Package Fast Logo" width="120" />

  # Package Fast

  **The Next-Generation Node.js Package Manager**

  [![Build Status](https://img.shields.io/github/workflow/status/package-fast/package-fast/CI?style=for-the-badge)](https://github.com/package-fast/package-fast/actions)
  [![NPM Version](https://img.shields.io/npm/v/package-fast?style=for-the-badge)](https://www.npmjs.com/package/package-fast)
  [![Downloads](https://img.shields.io/npm/dw/package-fast?style=for-the-badge)](https://www.npmjs.com/package/package-fast)
  [![License](https://img.shields.io/npm/l/package-fast?style=for-the-badge)](LICENSE)
  [![Performance](https://img.shields.io/badge/Performance-5--10x%20faster-blue?style=for-the-badge)](docs/reference/PERFORMANCE_BENCHMARKS.md)

  **5-10x faster** than npm • **55% less disk space** • **Enhanced security** • **Developer-first design**

  [Documentation](#) · [Report Bug](https://github.com/package-fast/package-fast/issues) · [Request Feature](https://github.com/package-fast/package-fast/issues)
</div>

## 🚀 Why Package Fast?

Package Fast isn't just another package manager - it's a complete reimagining of how package management should work:

- **⚡ Blazing Fast**: 5-10x faster than npm through advanced parallel processing
- **💾 Intelligent Storage**: Content-addressable storage reduces disk usage by up to 55%
- **🛡️ Built for Security**: Cryptographic verification and vulnerability scanning baked in
- **👩‍💻 Developer First**: Superior error messages and intuitive workflows
- **🔄 Fully Compatible**: Works with existing npm packages and workflows
- **🧩 Extensible**: Plugin system that doesn't compromise performance

## 📊 Performance Comparison

Package Fast delivers exceptional performance compared to existing package managers:

| Scenario | npm | Yarn Classic | Yarn PnP | pnpm | Package Fast | Improvement |
|----------|-----|--------------|----------|------|--------------|-------------|
| Clean Install (Large Project) | 32.5s | 7.3s | 3.6s | 8.7s | **4.2s** | **7.7x faster** |
| Repeat Install | 1.3s | 5.3s | n/a | 0.764s | **0.3s** | **4.3x faster** |
| Developer First Install | 8.0s | 5.4s | 1.3s | 2.4s | **0.8s** | **10x faster** |
| Disk Space Usage | 100% | - | - | 65% | **45%** | **55% less** |

## 🛠 Installation

### Via npm (Recommended)
```bash
npm install -g package-fast
```

### Using npx (No installation)
```bash
npx package-fast
```

### Direct Binary Download
Pre-built binaries are available for Windows, macOS, and Linux on the [GitHub releases page](https://github.com/package-fast/package-fast/releases).

## 🎯 Quick Start

After installation, you can use either `package-fast` or the shorter alias `pf`:

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

## 🌟 Key Features

### ⚡ Lightning-Fast Performance
Advanced parallel processing and optimal concurrency for dramatically faster installations.

### 💾 Efficient Storage
Content-addressed storage with hard link deduplication reduces disk usage by up to 55%.

### 🛡️ Enhanced Security
Built-in supply chain protection with cryptographic verification and vulnerability scanning.

### 🏗️ Superior Monorepo Support
Native support for complex multi-package architectures with workspaces.

### 🖥️ Cross-Platform
Seamless Windows, macOS, and Linux support with consistent behavior.

### 🧠 Developer Experience
Better error messaging, diagnostics, and intuitive workflows.

### 🔌 Extensible Architecture
Plugin system that doesn't compromise performance.

## 🔧 Technology Stack

- **Rust**: Performance-critical components for maximum speed
- **Node.js**: CLI compatibility and ecosystem integration

## 📚 Documentation

Explore our comprehensive documentation to get the most out of Package Fast:

- [Installation Guide](docs/user-guides/installation.md) - Install Package Fast on your system
- [Quick Start Guide](docs/user-guides/quick-start.md) - Get up and running quickly
- [Configuration](docs/user-guides/configuration.md) - Customize Package Fast behavior
- [Workspaces](docs/user-guides/workspaces.md) - Manage monorepos and multi-package projects
- [CLI Commands](docs/api-reference/cli-commands.md) - Complete command reference
- [Performance Benchmarks](docs/reference/PERFORMANCE_BENCHMARKS.md) - Detailed performance data

## 👨‍💻 Development

See [DEVELOPMENT_SETUP.md](DEVELOPMENT_SETUP.md) for detailed instructions on setting up the development environment.

See [GETTING_STARTED.md](GETTING_STARTED.md) for a quick start guide.

## 🚀 Get Started Today

Ready to experience the future of package management?

```bash
npm install -g package-fast
```

Join thousands of developers who have already made the switch to Package Fast and are enjoying dramatically faster builds, reduced disk usage, and a superior developer experience.

## 📞 Support

Need help? Here are the best ways to get support:

- [GitHub Issues](https://github.com/package-fast/package-fast/issues) - Report bugs and request features
- [Community Discord](#) - Real-time chat with other users and developers
- [Stack Overflow](#) - Questions and answers from the community

## 📄 License

Package Fast is licensed under the MIT License. See [LICENSE](LICENSE) for more information.

---
<div align="center">
  Made with ❤️ by the Package Fast Team
</div>