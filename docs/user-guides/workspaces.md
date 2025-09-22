# Workspaces Guide

Package Fast provides first-class support for monorepos and multi-package projects through its workspaces feature. This guide explains how to set up, configure, and manage workspaces effectively.

## What are Workspaces?

Workspaces are a feature that allows you to manage multiple packages from a single top-level root package. This is particularly useful for:

- Monorepos with multiple related packages
- Projects that need to share code between packages
- Organizations that maintain multiple packages
- Complex applications with shared components

## Setting Up Workspaces

### 1. Create the Root Package

Start by creating a root package.json file:

```json
{
  "name": "my-monorepo",
  "private": true,
  "workspaces": [
    "packages/*"
  ]
}
```

### 2. Configure Workspace Patterns

Define workspace patterns in your package.json:

```json
{
  "name": "my-monorepo",
  "private": true,
  "workspaces": [
    "packages/*",
    "apps/*",
    "tools/*"
  ]
}
```

You can also use an object format for more control:

```json
{
  "name": "my-monorepo",
  "private": true,
  "workspaces": {
    "packages": [
      "packages/*",
      "apps/*"
    ],
    "nohoist": [
      "**/react-native",
      "**/react-native/**"
    ]
  }
}
```

### 3. Create Workspace Packages

Create your workspace packages in the specified directories:

```
my-monorepo/
├── package.json
├── packages/
│   ├── package-a/
│   │   └── package.json
│   └── package-b/
│       └── package.json
└── apps/
    └── my-app/
        └── package.json
```

Each workspace package should have its own package.json:

`packages/package-a/package.json`:
```json
{
  "name": "@my-monorepo/package-a",
  "version": "1.0.0",
  "dependencies": {
    "lodash": "^4.17.21"
  }
}
```

## Workspace Commands

### Installing Dependencies

Install dependencies for all workspaces:

```bash
# Install all dependencies for all workspaces
pf install

# Install dependencies for a specific workspace
pf workspace package-a install
```

### Running Scripts

Run scripts in workspaces:

```bash
# Run build script in all workspaces
pf run build

# Run build script in a specific workspace
pf workspace package-a run build

# Run test script in all workspaces
pf run test

# Run test script in workspaces matching a pattern
pf workspaces --pattern "package-*" run test
```

### Adding Dependencies

Add dependencies to workspaces:

```bash
# Add dependency to a specific workspace
pf workspace package-a add lodash

# Add dev dependency to a specific workspace
pf workspace package-a add --dev jest

# Add dependency to root package
pf add --ignore-workspace-root-dependencies typescript
```

### Removing Dependencies

Remove dependencies from workspaces:

```bash
# Remove dependency from a specific workspace
pf workspace package-a remove lodash

# Remove dependency from root package
pf remove --ignore-workspace-root-dependencies typescript
```

## Workspace Resolution

### Internal Dependencies

Workspace packages can depend on each other:

`packages/package-b/package.json`:
```json
{
  "name": "@my-monorepo/package-b",
  "version": "1.0.0",
  "dependencies": {
    "@my-monorepo/package-a": "^1.0.0"
  }
}
```

When you run `pf install`, Package Fast will:

1. Link internal dependencies directly
2. Install external dependencies from the registry
3. Create a unified dependency tree

### Protocol Support

Package Fast supports the `workspace:` protocol for explicit workspace linking:

```json
{
  "dependencies": {
    "@my-monorepo/package-a": "workspace:*"
  }
}
```

This ensures the dependency is always resolved to the workspace version.

## Advanced Workspace Features

### Workspace Filtering

Run commands on specific workspaces:

```bash
# Run command on workspaces matching pattern
pf workspaces --pattern "package-*" run build

# Run command on specific workspaces
pf workspaces --include "package-a,package-b" run test

# Exclude specific workspaces
pf workspaces --exclude "my-app" run build
```

### Parallel Execution

Workspaces can be executed in parallel for better performance:

```bash
# Run tests in parallel across workspaces
pf workspaces run --parallel test

# Limit parallelism
pf workspaces run --parallel --concurrency 4 build
```

### Workspace Information

Get information about workspaces:

```bash
# List all workspaces
pf workspaces

# Get detailed information about a workspace
pf workspace package-a info

# Show dependency graph
pf workspaces --graph
```

## Configuration Options

### Workspace-specific Configuration

Configure workspace behavior in your root package.json:

```json
{
  "name": "my-monorepo",
  "private": true,
  "workspaces": [
    "packages/*",
    "apps/*"
  ],
  "package-fast": {
    "workspace": {
      "parallel": true,
      "hoist": true,
      "include": [
        "packages/*",
        "apps/*"
      ],
      "ignore": [
        "packages/legacy/*"
      ]
    }
  }
}
```

### Environment Variables

Use environment variables for workspace configuration:

```bash
# Set workspace concurrency
PF_WORKSPACE_CONCURRENCY=8 pf install

# Enable parallel workspace execution
PF_WORKSPACE_PARALLEL=true pf workspaces run build
```

## Best Practices

### 1. Project Structure

Organize your monorepo with a clear structure:

```
my-monorepo/
├── package.json          # Root package
├── packages/             # Reusable libraries
│   ├── components/
│   ├── utils/
│   └── shared/
├── apps/                 # Applications
│   ├── web/
│   ├── mobile/
│   └── api/
├── tools/                # Development tools
│   ├── cli/
│   └── scripts/
└── docs/                 # Documentation
```

### 2. Dependency Management

- Use consistent versioning across workspaces
- Prefer internal dependencies over external ones when possible
- Regularly audit and update dependencies
- Use lockfiles to ensure consistent installations

### 3. Versioning Strategy

Consider using a consistent versioning strategy:

```json
{
  "name": "@my-monorepo/package-a",
  "version": "1.2.3",
  "dependencies": {
    "@my-monorepo/package-b": "^1.2.3"
  }
}
```

### 4. Publishing

When publishing workspace packages:

```bash
# Publish a specific workspace
pf workspace package-a publish

# Publish all workspaces
pf workspaces publish

# Publish with version bump
pf workspaces publish --bump minor
```

## Troubleshooting

### Common Issues

1. **Dependency Resolution Issues**
   - Ensure workspace package names match dependencies exactly
   - Check for circular dependencies
   - Verify workspace patterns in package.json

2. **Installation Problems**
   - Run `pf install --force` to force reinstall
   - Check network connectivity for external dependencies
   - Verify registry configuration

3. **Script Execution Issues**
   - Ensure scripts are defined in each workspace package.json
   - Check for missing dependencies
   - Verify Node.js version compatibility

### Debugging

Enable debug output for workspaces:

```bash
# Enable workspace debugging
DEBUG=package-fast:workspace pf install

# Enable detailed logging
pf install --verbose
```

## Performance Optimization

### Caching

Workspaces benefit from shared caching:

```json
{
  "package-fast": {
    "cacheDir": "/fast-ssd/.package-fast/cache",
    "storeDir": "/fast-ssd/.package-fast/store"
  }
}
```

### Parallel Processing

Enable parallel workspace operations:

```bash
# Install with maximum parallelism
pf install --workspace-concurrency 16

# Run scripts in parallel
pf workspaces run --parallel build
```

### Hoisting

Enable dependency hoisting to reduce duplication:

```json
{
  "package-fast": {
    "install": {
      "hoist": true
    }
  }
}
```

## Integration with Tools

### CI/CD Integration

Example GitHub Actions workflow:

```yaml
name: CI
on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions/setup-node@v2
        with:
          node-version: '16'
      - run: npm install -g package-fast
      - run: pf install
      - run: pf workspaces run test
```

### IDE Integration

Most IDEs support workspaces automatically, but you can enhance the experience:

- Configure your IDE to recognize workspace packages
- Set up path mappings for TypeScript
- Enable workspace-aware linting and formatting

## See Also

- [Configuration Guide](configuration.md) - Workspace configuration options
- [CLI Commands](../api-reference/cli-commands.md) - Workspace-related commands
- [Plugin Development](../tutorials/plugin-development.md) - Creating workspace-aware plugins