# Installation Guide

Package Fast is a next-generation Node.js package manager that can be installed on Windows, macOS, and Linux. This guide will walk you through the installation process for each platform.

## Prerequisites

Before installing Package Fast, ensure you have:

- A supported operating system (Windows 10+, macOS 10.14+, or Linux)
- Node.js installed (version 12 or higher)
- Internet connectivity for downloading packages

## Installation Methods

There are several ways to install Package Fast depending on your preferences and system configuration.

### Method 1: Using npm (Recommended)

The easiest way to install Package Fast is through npm:

```bash
npm install -g package-fast
```

This will install Package Fast globally on your system, making it available from any directory.

### Method 2: Using Binary Distribution

For systems where npm installation isn't possible, you can download pre-built binaries:

1. Visit the [releases page](https://github.com/nom-nom-hub/package-fast/releases)
2. Download the appropriate binary for your platform
3. Extract the archive
4. Add the binary to your system PATH

### Method 3: Building from Source

If you want to build Package Fast from source:

1. Install Rust toolchain from [rustup.rs](https://rustup.rs/)
2. Install Node.js from [nodejs.org](https://nodejs.org/)
3. Clone the repository:
   ```bash
   git clone https://github.com/nom-nom-hub/package-fast.git
   cd package-fast
   ```
4. Build the project:
   ```bash
   cargo build --release
   npm install
   npm run build
   ```

## Platform-Specific Instructions

### Windows

On Windows, you can install Package Fast using npm as shown above. Alternatively, you can use the Windows Package Manager (winget):

```bash
winget install PackageFast
```

If you're installing from binaries:
1. Download the Windows zip file from the releases page
2. Extract it to a directory of your choice
3. Add the directory to your system PATH

### macOS

On macOS, you can install Package Fast using npm, Homebrew, or MacPorts:

**Using npm:**
```bash
npm install -g package-fast
```

**Using Homebrew:**
```bash
brew install package-fast
```

**Using MacPorts:**
```bash
sudo port install package-fast
```

If you're installing from binaries:
1. Download the macOS tar.gz file from the releases page
2. Extract it:
   ```bash
   tar -xzf package-fast-macos.tar.gz
   ```
3. Add the binary to your PATH

### Linux

On Linux, you can install Package Fast using npm or by downloading binaries:

**Using npm:**
```bash
npm install -g package-fast
```

**Using curl:**
```bash
curl -fsSL https://get.package-fast.io | sh
```

If you're installing from binaries:
1. Download the Linux tar.gz file from the releases page
2. Extract it:
   ```bash
   tar -xzf package-fast-linux.tar.gz
   ```
3. Move the binary to a directory in your PATH:
   ```bash
   sudo mv pf /usr/local/bin/
   ```

## Verification

After installation, verify that Package Fast is properly installed by checking its version:

```bash
pf --version
```

You should see output similar to:
```
package-fast 0.1.0
```

## Configuration

Package Fast can be configured through:
1. Command-line flags
2. Configuration files (`.packagefastrc`, `package-fast.json`)
3. Environment variables

See the [Configuration Guide](configuration.md) for detailed information.

## Troubleshooting

### Common Issues

1. **Command not found**: Ensure the installation directory is in your PATH
2. **Permission errors**: Try running with sudo (Linux/macOS) or as administrator (Windows)
3. **Node.js version conflicts**: Ensure you're using Node.js 12 or higher

### Getting Help

If you encounter issues during installation:
1. Check the [Troubleshooting Guide](troubleshooting.md)
2. Search existing issues on [GitHub](https://github.com/nom-nom-hub/package-fast/issues)
3. Join our [Discord community](#) for real-time support

## Next Steps

After installing Package Fast, proceed to the [Quick Start Guide](quick-start.md) to learn how to use it effectively.