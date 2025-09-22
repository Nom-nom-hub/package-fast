# Configuration Guide

Package Fast can be configured through multiple methods to customize its behavior according to your needs. This guide explains all available configuration options and how to use them.

## Configuration Methods

Package Fast supports several configuration methods, applied in the following priority order:

1. **Command-line flags** - Highest priority
2. **Environment variables** - 
3. **Project configuration files** - 
4. **User configuration files** - 
5. **Global configuration files** - Lowest priority

## Configuration File Locations

### Project Configuration

Project-level configuration files are located in your project directory:

- `.packagefastrc` - JSON or YAML format
- `.packagefastrc.json` - JSON format
- `.packagefastrc.yaml` or `.packagefastrc.yml` - YAML format
- `package-fast.json` - JSON format
- `package-fast.yaml` or `package-fast.yml` - YAML format

### User Configuration

User-level configuration files apply to all projects for a specific user:

- `~/.packagefastrc` - JSON or YAML format
- `~/.package-fast/config` - JSON format

### Global Configuration

Global configuration files apply system-wide:

- `/etc/package-fast/config` - JSON format (Unix-like systems)

## Configuration File Formats

### JSON Format

```json
{
  "registry": "https://registry.npmjs.org/",
  "cacheDir": "~/.package-fast/cache",
  "storeDir": "~/.package-fast/store",
  "network": {
    "timeout": 60000,
    "concurrency": 16
  },
  "security": {
    "verifyIntegrity": true,
    "scanVulnerabilities": true
  }
}
```

### YAML Format

```yaml
registry: https://registry.npmjs.org/
cacheDir: ~/.package-fast/cache
storeDir: ~/.package-fast/store
network:
  timeout: 60000
  concurrency: 16
security:
  verifyIntegrity: true
  scanVulnerabilities: true
```

## Core Configuration Options

### Registry Configuration

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `registry` | string | `https://registry.npmjs.org/` | Default package registry URL |
| `registries` | object | `{}` | Custom registries for scoped packages |

Example:
```json
{
  "registry": "https://registry.npmjs.org/",
  "registries": {
    "@mycompany": "https://npm.mycompany.com/"
  }
}
```

### Cache and Storage

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `cacheDir` | string | `~/.package-fast/cache` | Directory for cached packages |
| `storeDir` | string | `~/.package-fast/store` | Directory for content-addressed store |
| `cacheTimeout` | number | `86400000` | Cache timeout in milliseconds (24 hours) |

### Network Configuration

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `network.timeout` | number | `60000` | Network request timeout in milliseconds |
| `network.concurrency` | number | `16` | Maximum concurrent network requests |
| `network.retry` | number | `3` | Number of retry attempts for failed requests |
| `network.userAgent` | string | `package-fast/0.1.0` | User agent string for requests |

### Installation Configuration

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `install.hoist` | boolean | `true` | Enable dependency hoisting |
| `install.force` | boolean | `false` | Force reinstall packages |
| `install.frozenLockfile` | boolean | `false` | Don't generate lockfile |
| `install.preferOffline` | boolean | `false` | Prefer offline installation |

### Security Configuration

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `security.verifyIntegrity` | boolean | `true` | Verify package integrity |
| `security.scanVulnerabilities` | boolean | `true` | Scan for vulnerabilities |
| `security.minimumReleaseAge` | string | `72h` | Minimum age for new packages |
| `security.enableRuntimeProtection` | boolean | `true` | Enable runtime protection |

### Workspace Configuration

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `workspace.include` | array | `[]` | Additional workspace patterns |
| `workspace.ignore` | array | `[]` | Workspace patterns to ignore |
| `workspace.parallel` | boolean | `true` | Enable parallel workspace operations |

## Environment Variables

Package Fast respects several environment variables:

| Variable | Configuration Equivalent | Description |
|----------|--------------------------|-------------|
| `PF_REGISTRY` | `registry` | Custom registry URL |
| `PF_CACHE_DIR` | `cacheDir` | Custom cache directory |
| `PF_STORE_DIR` | `storeDir` | Custom store directory |
| `PF_NETWORK_TIMEOUT` | `network.timeout` | Network timeout in milliseconds |
| `PF_NETWORK_CONCURRENCY` | `network.concurrency` | Maximum concurrent requests |
| `HTTP_PROXY` | - | HTTP proxy for requests |
| `HTTPS_PROXY` | - | HTTPS proxy for requests |
| `NO_PROXY` | - | Bypass proxy for hosts |

## Command-line Flags

Most configuration options can also be set via command-line flags:

```bash
# Set registry
pf install --registry https://registry.npmjs.org/

# Set cache directory
pf install --cache-dir /tmp/package-fast-cache

# Set network timeout
pf install --network-timeout 30000
```

## Configuration Examples

### Basic Configuration

`.packagefastrc`:
```json
{
  "registry": "https://registry.npmjs.org/",
  "cacheDir": "~/.package-fast/cache",
  "storeDir": "~/.package-fast/store"
}
```

### Advanced Configuration

`.packagefastrc.yaml`:
```yaml
registry: https://registry.npmjs.org/
cacheDir: ~/.package-fast/cache
storeDir: ~/.package-fast/store
network:
  timeout: 60000
  concurrency: 16
  retry: 3
security:
  verifyIntegrity: true
  scanVulnerabilities: true
  minimumReleaseAge: "72h"
install:
  hoist: true
  force: false
plugins:
  - package-fast-plugin-typescript
  - package-fast-plugin-linter
```

### Monorepo Configuration

`package-fast.json`:
```json
{
  "workspace": {
    "include": [
      "packages/*",
      "apps/*"
    ],
    "parallel": true
  },
  "install": {
    "hoist": true
  }
}
```

## Plugin Configuration

Plugins can be configured in the configuration file:

```json
{
  "plugins": [
    "package-fast-plugin-typescript",
    {
      "name": "package-fast-plugin-linter",
      "config": {
        "eslint": true,
        "prettier": true
      }
    }
  ]
}
```

## Performance Tuning

### Network Optimization

```json
{
  "network": {
    "timeout": 30000,
    "concurrency": 32,
    "retry": 2
  }
}
```

### Cache Settings

```json
{
  "cacheDir": "/fast-ssd/package-fast/cache",
  "cacheTimeout": 43200000,
  "storeDir": "/fast-ssd/package-fast/store"
}
```

## Security Hardening

For production environments:

```json
{
  "security": {
    "verifyIntegrity": true,
    "scanVulnerabilities": true,
    "minimumReleaseAge": "168h",
    "enableRuntimeProtection": true
  }
}
```

## Troubleshooting Configuration

### Checking Current Configuration

```bash
# List all configuration values
pf config list

# Get specific configuration value
pf config get registry
```

### Resetting Configuration

```bash
# Reset to defaults
pf config reset

# Delete specific configuration
pf config delete registry
```

### Configuration Validation

Package Fast validates configuration files and will warn about invalid options:

```bash
# Validate configuration
pf config validate
```

## Best Practices

### 1. Project vs User Configuration

- Use project configuration for project-specific settings
- Use user configuration for personal preferences
- Avoid committing sensitive configuration to version control

### 2. Performance Optimization

- Adjust concurrency based on your network and system
- Use fast storage for cache and store directories
- Configure appropriate timeouts for your environment

### 3. Security Considerations

- Keep security features enabled in production
- Regularly update configuration for new security features
- Use environment variables for sensitive settings

### 4. Team Collaboration

- Document project-specific configuration
- Use consistent configuration across team members
- Version control non-sensitive configuration files

## See Also

- [CLI Commands](../api-reference/cli-commands.md) - Configuration-related commands
- [Security Documentation](../technical-reference/security.md) - Security configuration
- [Performance Optimization](../technical-reference/performance.md) - Performance-related settings