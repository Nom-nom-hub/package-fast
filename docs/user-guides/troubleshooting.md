# Troubleshooting Guide

This guide helps you resolve common issues you might encounter while using Package Fast. If you don't find a solution here, check our [GitHub Issues](https://github.com/nom-nom-hub/package-fast/issues) or join our [Discord community](#) for help.

## Installation Issues

### Command Not Found

**Problem**: After installation, running `pf --version` returns "command not found".

**Solutions**:
1. Verify Package Fast is installed:
   ```bash
   npm list -g package-fast
   ```
2. Check if the installation directory is in your PATH:
   ```bash
   echo $PATH
   ```
3. Add npm's global bin directory to your PATH:
   ```bash
   # For bash/zsh
   export PATH=$(npm bin -g):$PATH
   
   # For fish
   set -gx PATH (npm bin -g) $PATH
   ```

### Permission Errors

**Problem**: Installation fails with permission errors.

**Solutions**:
1. **Linux/macOS**: Run with sudo:
   ```bash
   sudo npm install -g package-fast
   ```
2. **Alternative**: Configure npm to use a different directory:
   ```bash
   mkdir ~/.npm-global
   npm config set prefix '~/.npm-global'
   export PATH=~/.npm-global/bin:$PATH
   npm install -g package-fast
   ```

### Installation Hangs

**Problem**: Installation process hangs or takes too long.

**Solutions**:
1. Check your network connection
2. Try using a different registry:
   ```bash
   pf install --registry https://registry.npmjs.org/
   ```
3. Increase network timeout:
   ```bash
   pf install --network-timeout 300000
   ```

## Dependency Resolution Issues

### Version Conflicts

**Problem**: Package Fast reports version conflicts during installation.

**Solutions**:
1. Check your package.json for conflicting version requirements
2. Use resolution overrides in your configuration:
   ```json
   {
     "resolutions": {
       "lodash": "^4.17.21"
     }
   }
   ```
3. Clear the cache and reinstall:
   ```bash
   pf cache clean
   pf install
   ```

### Missing Peer Dependencies

**Problem**: Warnings about missing peer dependencies.

**Solutions**:
1. Install the missing peer dependencies:
   ```bash
   pf install react react-dom
   ```
2. Enable automatic peer dependency installation:
   ```bash
   pf install --install-peer-deps
   ```

### Circular Dependencies

**Problem**: Errors about circular dependencies.

**Solutions**:
1. Review your dependency structure
2. Refactor to eliminate circular dependencies
3. Use the `--ignore-circular-deps` flag (not recommended for production):
   ```bash
   pf install --ignore-circular-deps
   ```

## Network Issues

### Registry Connection Failures

**Problem**: Cannot connect to the package registry.

**Solutions**:
1. Check your internet connection
2. Verify the registry URL:
   ```bash
   pf config get registry
   ```
3. Try a different registry:
   ```bash
   pf install --registry https://registry.npmjs.org/
   ```
4. Configure proxy settings if behind a corporate firewall:
   ```bash
   export HTTP_PROXY=http://proxy.company.com:8080
   export HTTPS_PROXY=https://proxy.company.com:8080
   ```

### Slow Downloads

**Problem**: Package downloads are extremely slow.

**Solutions**:
1. Check your network bandwidth
2. Use a closer registry mirror:
   ```bash
   pf install --registry https://npm.registry.company.com/
   ```
3. Increase network concurrency:
   ```bash
   pf install --network-concurrency 32
   ```
4. Enable offline mode if packages are already cached:
   ```bash
   pf install --offline
   ```

## Cache Issues

### Corrupted Cache

**Problem**: Installation fails due to corrupted cache.

**Solutions**:
1. Clean the cache:
   ```bash
   pf cache clean
   ```
2. Verify cache integrity:
   ```bash
   pf cache verify
   ```
3. If problems persist, remove the entire cache directory:
   ```bash
   rm -rf ~/.package-fast/cache
   ```

### Outdated Cache

**Problem**: Getting outdated package versions.

**Solutions**:
1. Clear the cache:
   ```bash
   pf cache clean
   ```
2. Force fresh installation:
   ```bash
   pf install --force
   ```
3. Adjust cache timeout:
   ```bash
   pf install --cache-timeout 3600000
   ```

## Workspace Issues

### Workspace Packages Not Found

**Problem**: Workspace packages are not being recognized.

**Solutions**:
1. Verify workspace patterns in package.json:
   ```json
   {
     "workspaces": [
       "packages/*",
       "apps/*"
     ]
   }
   ```
2. Ensure workspace package.json files have correct names
3. Run installation from the root directory:
   ```bash
   pf install --workspace
   ```

### Internal Dependency Resolution

**Problem**: Internal workspace dependencies are not resolving correctly.

**Solutions**:
1. Check that package names match exactly:
   ```json
   // In dependent package
   {
     "dependencies": {
       "@my-org/shared-utils": "^1.0.0"
     }
   }
   
   // In workspace package
   {
     "name": "@my-org/shared-utils",
     "version": "1.0.0"
   }
   ```
2. Use the workspace protocol:
   ```json
   {
     "dependencies": {
       "@my-org/shared-utils": "workspace:*"
     }
   }
   ```

## Security Issues

### Integrity Check Failures

**Problem**: Package integrity verification fails.

**Solutions**:
1. Clear the cache and reinstall:
   ```bash
   pf cache clean
   pf install
   ```
2. If the issue persists, report it to the package maintainer
3. Temporarily disable integrity checking (not recommended):
   ```bash
   pf install --no-verify-integrity
   ```

### Vulnerability Alerts

**Problem**: Security vulnerabilities detected in dependencies.

**Solutions**:
1. Review the vulnerability report:
   ```bash
   pf audit
   ```
2. Update vulnerable packages:
   ```bash
   pf update
   ```
3. Apply automatic fixes:
   ```bash
   pf audit --fix
   ```

## Performance Issues

### Slow Installation

**Problem**: Installation process is slower than expected.

**Solutions**:
1. Check system resources (CPU, memory, disk I/O)
2. Increase concurrency settings:
   ```bash
   pf install --network-concurrency 32
   ```
3. Use SSD storage for cache and store directories
4. Enable parallel workspace installation:
   ```bash
   pf install --workspace-concurrency 8
   ```

### High Memory Usage

**Problem**: Package Fast consumes excessive memory.

**Solutions**:
1. Reduce concurrency:
   ```bash
   pf install --network-concurrency 8
   ```
2. Limit Node.js memory:
   ```bash
   NODE_OPTIONS="--max-old-space-size=4096" pf install
   ```
3. Use streaming installation for large projects:
   ```bash
   pf install --stream
   ```

## Plugin Issues

### Plugin Not Loading

**Problem**: Installed plugins are not being loaded.

**Solutions**:
1. Verify plugin installation:
   ```bash
   pf plugin list
   ```
2. Check plugin configuration:
   ```json
   {
     "plugins": [
       "package-fast-plugin-typescript"
     ]
   }
   ```
3. Ensure plugin compatibility with your Package Fast version

### Plugin Errors

**Problem**: Plugins are causing errors or unexpected behavior.

**Solutions**:
1. Disable the problematic plugin:
   ```bash
   pf config delete plugins.my-problematic-plugin
   ```
2. Check plugin documentation for compatibility issues
3. Update the plugin to the latest version:
   ```bash
   pf plugin update my-plugin
   ```

## Environment Issues

### Node.js Version Compatibility

**Problem**: Errors related to Node.js version incompatibility.

**Solutions**:
1. Check your Node.js version:
   ```bash
   node --version
   ```
2. Package Fast requires Node.js 12 or higher
3. Update Node.js if needed:
   ```bash
   # Using nvm
   nvm install 16
   nvm use 16
   ```

### Operating System Issues

**Problem**: Platform-specific errors.

**Solutions**:
1. Check the error message for platform-specific details
2. Ensure you're using a supported OS version
3. Report platform-specific issues to our GitHub repository

## Advanced Troubleshooting

### Debug Mode

Enable debug output for detailed information:

```bash
# Enable all debug output
DEBUG=* pf install

# Enable Package Fast specific debug output
DEBUG=package-fast:* pf install

# Enable specific module debug output
DEBUG=package-fast:resolver pf install
```

### Verbose Logging

Get more detailed output:

```bash
pf install --verbose
```

### Log Files

Package Fast writes logs to:

- **Linux/macOS**: `~/.package-fast/logs/`
- **Windows**: `%LOCALAPPDATA%\package-fast\logs\`

Check these logs for detailed error information.

## Getting Help

If you're unable to resolve your issue:

1. **Search Existing Issues**: Check our [GitHub Issues](https://github.com/nom-nom-hub/package-fast/issues)
2. **Ask the Community**: Join our [Discord community](#)
3. **Create an Issue**: If you've found a bug, create a detailed issue report
4. **Contact Support**: For enterprise support, contact our support team

When reporting issues, include:

- Package Fast version (`pf --version`)
- Node.js version (`node --version`)
- Operating system and version
- Exact error messages
- Steps to reproduce the issue
- Relevant configuration files

## See Also

- [Installation Guide](installation.md) - Proper installation methods
- [Configuration Guide](configuration.md) - Configuration options
- [CLI Commands](../api-reference/cli-commands.md) - Available commands and options