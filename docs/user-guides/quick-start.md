# Quick Start Guide

Get up and running with Package Fast in minutes. This guide will show you how to install dependencies, add packages, and understand the basic workflow.

## Prerequisites

Before starting, ensure you have:
1. Package Fast installed (see [Installation Guide](installation.md))
2. A Node.js project with a `package.json` file

## Initializing a Project

If you don't already have a Node.js project, create one:

```bash
mkdir my-project
cd my-project
npm init -y
```

This creates a basic `package.json` file for your project.

## Installing Dependencies

### Install All Dependencies

To install all dependencies listed in your `package.json`:

```bash
pf install
```

This command will:
1. Read your `package.json` file
2. Resolve all dependencies and their versions
3. Download packages from the npm registry
4. Create a `node_modules` directory with all installed packages
5. Generate or update a lockfile for deterministic installs

### Install Specific Packages

To install specific packages:

```bash
pf install lodash express
```

This will:
1. Install the latest versions of `lodash` and `express`
2. Add them to your `dependencies` in `package.json`
3. Update your lockfile

To install a specific version:

```bash
pf install lodash@4.17.20
```

### Install Development Dependencies

To install packages as development dependencies:

```bash
pf install --dev jest eslint
```

This will install `jest` and `eslint` as development dependencies.

## Adding Packages

To add packages to your project:

```bash
pf add express
```

This is equivalent to `pf install` but explicitly adds packages to your `package.json`.

To add development dependencies:

```bash
pf add --dev jest
```

## Removing Packages

To remove packages from your project:

```bash
pf remove express
```

This will:
1. Remove `express` from your `dependencies`
2. Delete the package from `node_modules`
3. Update your lockfile

## Updating Packages

To update all packages to their latest compatible versions:

```bash
pf update
```

To update specific packages:

```bash
pf update lodash express
```

## Working with Workspaces

If you're working with a monorepo using workspaces:

```bash
pf install --workspace
```

This will install dependencies for all workspaces in your project.

## Checking Package Information

To get information about a package:

```bash
pf info lodash
```

This displays package metadata including version, description, and dependencies.

## Performance Benefits

Package Fast is designed to be significantly faster than traditional package managers:

- **Parallel Installation**: Packages are installed concurrently
- **Content-Addressed Storage**: Reduces disk space usage through deduplication
- **Intelligent Caching**: Minimizes network requests

You'll notice these benefits most when:
- Installing large dependency trees
- Working with monorepos
- Reinstalling dependencies in CI/CD environments

## Next Steps

Now that you've learned the basics:
1. Explore [Configuration Options](configuration.md) to customize Package Fast
2. Learn about [Workspaces](workspaces.md) for monorepo management
3. Discover [Plugins](plugins.md) to extend functionality
4. Check out [Advanced Workflows](../tutorials/advanced-workflows.md) for complex use cases

## Getting Help

For more information about any command:

```bash
pf --help
pf install --help
```

You can also check our [Troubleshooting Guide](troubleshooting.md) if you encounter any issues.