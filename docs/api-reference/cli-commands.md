# CLI Commands Reference

Package Fast provides a comprehensive set of command-line interface commands for managing Node.js dependencies. This document details all available commands, their options, and usage examples.

## Command Structure

```bash
pf [command] [options] [arguments]
```

## Global Options

These options can be used with any command:

| Option | Alias | Description |
|--------|-------|-------------|
| `--help` | `-h` | Display help for the command |
| `--version` | `-v` | Display the version of Package Fast |
| `--debug` | `-d` | Enable debug output |
| `--quiet` | `-q` | Suppress output except for errors |
| `--silent` | | Suppress all output |
| `--cwd <path>` | | Set working directory |

## Core Commands

### install

Install dependencies from package.json or specific packages.

**Syntax:**
```bash
pf install [options] [packages...]
```

**Options:**

| Option | Alias | Description |
|--------|-------|-------------|
| `--dev` | `-D` | Install dev dependencies only |
| `--prod` | `-P` | Install production dependencies only |
| `--force` | `-f` | Force reinstall packages |
| `--workspace` | `-w` | Install dependencies for all workspaces |
| `--frozen-lockfile` | | Don't generate a lockfile and fail if an update is needed |

**Examples:**
```bash
# Install all dependencies
pf install

# Install specific packages
pf install lodash express

# Install specific version
pf install lodash@4.17.20

# Install dev dependencies only
pf install --dev

# Force reinstall all packages
pf install --force
```

### add

Add packages to dependencies and install them.

**Syntax:**
```bash
pf add [options] [packages...]
```

**Options:**

| Option | Alias | Description |
|--------|-------|-------------|
| `--dev` | `-D` | Add to devDependencies |
| `--exact` | `-E` | Install exact version |
| `--tilde` | `-T` | Install minor version with tilde (~) |

**Examples:**
```bash
# Add packages to dependencies
pf add express lodash

# Add packages to devDependencies
pf add --dev jest eslint

# Add exact version
pf add --exact react@17.0.2
```

### remove

Remove packages from dependencies and delete them.

**Syntax:**
```bash
pf remove [packages...]
```

**Examples:**
```bash
# Remove packages
pf remove express lodash
```

### update

Update packages to their latest versions.

**Syntax:**
```bash
pf update [options] [packages...]
```

**Options:**

| Option | Alias | Description |
|--------|-------|-------------|
| `--latest` | `-L` | Update to latest version ignoring ranges |
| `--workspace` | `-w` | Update dependencies for all workspaces |

**Examples:**
```bash
# Update all packages
pf update

# Update specific packages
pf update lodash express

# Update to latest versions
pf update --latest
```

### info

Display information about a package.

**Syntax:**
```bash
pf info <package>
```

**Examples:**
```bash
# Get package information
pf info lodash
```

## Workspace Commands

### workspace

Run commands in specific workspaces.

**Syntax:**
```bash
pf workspace <workspace-name> <command>
```

**Examples:**
```bash
# Run install in a specific workspace
pf workspace my-app install

# Run a script in a specific workspace
pf workspace my-app run build
```

### workspaces

List all workspaces in the project.

**Syntax:**
```bash
pf workspaces
```

## Plugin Commands

### plugin

Manage plugins for Package Fast.

**Syntax:**
```bash
pf plugin <subcommand>
```

**Subcommands:**
- `list` - List installed plugins
- `install <plugin>` - Install a plugin
- `remove <plugin>` - Remove a plugin
- `info <plugin>` - Get information about a plugin

**Examples:**
```bash
# List installed plugins
pf plugin list

# Install a plugin
pf plugin install package-fast-plugin-typescript
```

## Utility Commands

### init

Initialize a new Node.js project.

**Syntax:**
```bash
pf init [name]
```

**Examples:**
```bash
# Initialize in current directory
pf init

# Initialize with project name
pf init my-project
```

### run

Run scripts defined in package.json.

**Syntax:**
```bash
pf run <script-name>
```

**Examples:**
```bash
# Run build script
pf run build

# Run test script
pf run test
```

### list

List installed packages.

**Syntax:**
```bash
pf list [options]
```

**Options:**

| Option | Alias | Description |
|--------|-------|-------------|
| `--depth <number>` | | Limit the depth of the dependency tree |
| `--pattern <pattern>` | | Filter by pattern |
| `--json` | | Output in JSON format |

### outdated

Check for outdated packages.

**Syntax:**
```bash
pf outdated [options]
```

**Options:**

| Option | Alias | Description |
|--------|-------|-------------|
| `--workspace` | `-w` | Check for all workspaces |
| `--compatible` | | Show only compatible updates |
| `--json` | | Output in JSON format |

### audit

Check for security vulnerabilities.

**Syntax:**
```bash
pf audit [options]
```

**Options:**

| Option | Alias | Description |
|--------|-------|-------------|
| `--fix` | | Automatically fix vulnerabilities |
| `--json` | | Output in JSON format |
| `--level <level>` | | Minimum level of vulnerability (low, moderate, high, critical) |

## Configuration Commands

### config

Manage Package Fast configuration.

**Syntax:**
```bash
pf config <subcommand>
```

**Subcommands:**
- `set <key> <value>` - Set a configuration value
- `get <key>` - Get a configuration value
- `delete <key>` - Delete a configuration value
- `list` - List all configuration values

**Examples:**
```bash
# Set registry URL
pf config set registry https://registry.npmjs.org/

# Get registry URL
pf config get registry

# List all configuration
pf config list
```

## Cache Commands

### cache

Manage Package Fast cache.

**Syntax:**
```bash
pf cache <subcommand>
```

**Subcommands:**
- `clean` - Clean the cache
- `dir` - Display cache directory
- `verify` - Verify cache integrity

**Examples:**
```bash
# Clean cache
pf cache clean

# Show cache directory
pf cache dir
```

## Advanced Commands

### exec

Execute a shell command in the context of the project.

**Syntax:**
```bash
pf exec <command>
```

**Examples:**
```bash
# Run a command with project dependencies in PATH
pf exec tsc --version
```

### dlx

Run a package in a temporary environment.

**Syntax:**
```bash
pf dlx <package> [args...]
```

**Examples:**
```bash
# Run create-react-app without installing it
pf dlx create-react-app my-app
```

## Environment Variables

Package Fast respects several environment variables:

| Variable | Description |
|----------|-------------|
| `PF_REGISTRY` | Custom registry URL |
| `PF_CACHE_DIR` | Custom cache directory |
| `PF_STORE_DIR` | Custom store directory |
| `HTTP_PROXY` | HTTP proxy for network requests |
| `HTTPS_PROXY` | HTTPS proxy for network requests |
| `NO_PROXY` | Comma-separated list of hosts to bypass proxy |

## Exit Codes

Package Fast uses standard exit codes:

| Code | Description |
|------|-------------|
| 0 | Success |
| 1 | Generic error |
| 2 | Invalid command or options |
| 3 | Network error |
| 4 | Package resolution error |
| 5 | Security vulnerability detected |

## See Also

- [Configuration Options](configuration-options.md) - Detailed configuration parameters
- [Plugin API](plugin-api.md) - Extending Package Fast with plugins
- [JavaScript API](javascript-api.md) - Programmatic usage