# Plugin Development Guide

Package Fast provides a powerful plugin system that allows you to extend and customize its functionality. This guide will walk you through creating your first Package Fast plugin.

## Prerequisites

Before developing a plugin, ensure you have:

1. Node.js (version 16 or higher)
2. Package Fast installed
3. Basic knowledge of JavaScript/TypeScript
4. Understanding of Package Fast's core concepts

## Plugin Architecture

Package Fast plugins are Node.js modules that export a specific interface. Plugins can hook into various stages of the package management lifecycle to modify behavior or add new functionality.

### Plugin Interface

All plugins must export a default object that implements the Plugin interface:

```typescript
interface Plugin {
  name: string;
  version: string;
  description?: string;
  hooks?: {
    // Resolution phase hooks
    resolvePackage?(pkg: PackageInfo): Promise<PackageInfo>;
    
    // Installation phase hooks
    beforeInstall?(pkg: InstalledPackage): Promise<void>;
    afterInstall?(pkg: InstalledPackage): Promise<void>;
    
    // Command extension hooks
    registerCommands?(): Command[];
  };
}
```

## Creating Your First Plugin

Let's create a simple plugin that logs package installation events.

### 1. Initialize the Plugin Project

```bash
mkdir package-fast-plugin-logger
cd package-fast-plugin-logger
npm init -y
```

### 2. Install Development Dependencies

```bash
npm install --save-dev typescript @types/node package-fast
```

### 3. Create the Plugin File

Create `index.ts`:

```typescript
import { Plugin, InstalledPackage } from 'package-fast';

const loggerPlugin: Plugin = {
  name: 'package-fast-plugin-logger',
  version: '1.0.0',
  description: 'Logs package installation events',
  
  hooks: {
    async beforeInstall(pkg: InstalledPackage): Promise<void> {
      console.log(`[Logger] About to install ${pkg.name}@${pkg.version}`);
    },
    
    async afterInstall(pkg: InstalledPackage): Promise<void> {
      console.log(`[Logger] Successfully installed ${pkg.name}@${pkg.version}`);
    }
  }
};

export default loggerPlugin;
```

### 4. Configure TypeScript

Create `tsconfig.json`:

```json
{
  "compilerOptions": {
    "target": "ES2020",
    "module": "commonjs",
    "lib": ["ES2020"],
    "outDir": "./dist",
    "rootDir": "./src",
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true,
    "declaration": true
  },
  "include": ["src/**/*"],
  "exclude": ["node_modules", "dist"]
}
```

### 5. Update package.json

```json
{
  "name": "package-fast-plugin-logger",
  "version": "1.0.0",
  "description": "Logs package installation events",
  "main": "dist/index.js",
  "types": "dist/index.d.ts",
  "scripts": {
    "build": "tsc",
    "prepublishOnly": "npm run build"
  },
  "keywords": ["package-fast", "plugin", "logger"],
  "peerDependencies": {
    "package-fast": "^0.1.0"
  },
  "devDependencies": {
    "typescript": "^4.5.0",
    "@types/node": "^16.0.0",
    "package-fast": "^0.1.0"
  }
}
```

### 6. Build the Plugin

```bash
npm run build
```

## Testing Your Plugin

### 1. Install the Plugin Locally

In your test project:

```bash
npm install ../path/to/package-fast-plugin-logger
```

### 2. Configure Package Fast to Use the Plugin

Create or update `.packagefastrc` in your project:

```json
{
  "plugins": ["package-fast-plugin-logger"]
}
```

### 3. Test the Plugin

Run a package installation:

```bash
pf install lodash
```

You should see the log messages from your plugin.

## Advanced Plugin Features

### Adding Custom Commands

Plugins can register custom commands:

```typescript
import { Plugin, Command } from 'package-fast';

const customCommandPlugin: Plugin = {
  name: 'package-fast-plugin-custom-command',
  version: '1.0.0',
  
  hooks: {
    registerCommands(): Command[] {
      return [
        {
          name: 'hello',
          description: 'Say hello from the plugin',
          handler: async (args: string[]) => {
            console.log('Hello from the custom plugin command!');
            return 0; // Exit code
          }
        }
      ];
    }
  }
};

export default customCommandPlugin;
```

### Modifying Package Resolution

Plugins can modify how packages are resolved:

```typescript
import { Plugin, PackageInfo } from 'package-fast';

const resolutionPlugin: Plugin = {
  name: 'package-fast-plugin-resolution',
  version: '1.0.0',
  
  hooks: {
    async resolvePackage(pkg: PackageInfo): Promise<PackageInfo> {
      // Modify package information
      if (pkg.name === 'lodash') {
        // Always use a specific version of lodash
        return {
          ...pkg,
          version: '4.17.21'
        };
      }
      return pkg;
    }
  }
};

export default resolutionPlugin;
```

## Plugin Distribution

### Publishing to npm

To publish your plugin to npm:

1. Create an npm account if you don't have one
2. Log in to npm:
   ```bash
   npm login
   ```
3. Publish your plugin:
   ```bash
   npm publish
   ```

### Naming Conventions

Follow these naming conventions for Package Fast plugins:
- Start with `package-fast-plugin-`
- Use descriptive names that indicate functionality
- Use kebab-case for multi-word names

## Plugin Configuration

Plugins can accept configuration options:

### 1. Define Configuration Interface

```typescript
interface LoggerPluginConfig {
  logLevel: 'info' | 'warn' | 'error';
  outputPath?: string;
}
```

### 2. Access Configuration in Plugin

```typescript
import { Plugin, InstalledPackage } from 'package-fast';

const loggerPlugin: Plugin = {
  name: 'package-fast-plugin-logger',
  version: '1.0.0',
  
  hooks: {
    async beforeInstall(pkg: InstalledPackage): Promise<void> {
      // Access plugin configuration
      const config = pkg.context.config.plugins?.['package-fast-plugin-logger'] as LoggerPluginConfig || {};
      const logLevel = config.logLevel || 'info';
      
      if (logLevel === 'info') {
        console.log(`[Logger] About to install ${pkg.name}@${pkg.version}`);
      }
    }
  }
};

export default loggerPlugin;
```

### 3. User Configuration

Users configure plugins in their `.packagefastrc`:

```json
{
  "plugins": {
    "package-fast-plugin-logger": {
      "logLevel": "info",
      "outputPath": "./logs/package-install.log"
    }
  }
}
```

## Best Practices

### Security Considerations

1. **Validate Input**: Always validate input from hooks
2. **Limit Permissions**: Only request necessary permissions
3. **Secure Storage**: Encrypt sensitive configuration data
4. **Audit Logging**: Log plugin actions for security review

### Performance Considerations

1. **Async Operations**: Use async/await for non-blocking operations
2. **Caching**: Cache expensive operations when possible
3. **Resource Cleanup**: Clean up resources in shutdown hooks
4. **Efficient Algorithms**: Use efficient algorithms for data processing

### Error Handling

1. **Graceful Degradation**: Handle errors without crashing Package Fast
2. **Meaningful Error Messages**: Provide clear error information
3. **Error Logging**: Log errors for debugging
4. **Recovery Mechanisms**: Implement recovery where possible

## Plugin API Reference

### Hook Types

#### resolvePackage
- **When**: Called during package resolution
- **Purpose**: Modify package information before installation
- **Parameters**: `PackageInfo`
- **Returns**: `Promise<PackageInfo>`

#### beforeInstall
- **When**: Called before package installation
- **Purpose**: Perform actions before installation begins
- **Parameters**: `InstalledPackage`
- **Returns**: `Promise<void>`

#### afterInstall
- **When**: Called after package installation
- **Purpose**: Perform actions after installation completes
- **Parameters**: `InstalledPackage`
- **Returns**: `Promise<void>`

#### registerCommands
- **When**: Called during CLI initialization
- **Purpose**: Register custom commands
- **Parameters**: None
- **Returns**: `Command[]`

## Debugging Plugins

### Debug Mode

Run Package Fast with debug output:

```bash
pf --debug install lodash
```

### Plugin Debugging

Enable plugin-specific debugging:

```bash
DEBUG=package-fast:plugin:* pf install lodash
```

### Development Tips

1. **Use TypeScript**: Provides better development experience
2. **Write Tests**: Test your plugin functionality
3. **Document**: Provide clear documentation for users
4. **Version Carefully**: Follow semantic versioning

## Example Plugins

### 1. TypeScript Plugin

A plugin that adds TypeScript support:

```typescript
import { Plugin } from 'package-fast';

const typescriptPlugin: Plugin = {
  name: 'package-fast-plugin-typescript',
  version: '1.0.0',
  description: 'Adds TypeScript support to Package Fast',
  
  hooks: {
    registerCommands() {
      return [
        {
          name: 'tsc',
          description: 'Run TypeScript compiler',
          handler: async (args: string[]) => {
            // Implementation for running tsc
            return 0;
          }
        }
      ];
    }
  }
};

export default typescriptPlugin;
```

### 2. Monorepo Plugin

A plugin that enhances monorepo support:

```typescript
import { Plugin } from 'package-fast';

const monorepoPlugin: Plugin = {
  name: 'package-fast-plugin-monorepo',
  version: '1.0.0',
  description: 'Enhanced monorepo support',
  
  hooks: {
    async resolvePackage(pkg) {
      // Custom monorepo resolution logic
      return pkg;
    }
  }
};

export default monorepoPlugin;
```

## Publishing and Distribution

### Package.json Requirements

Ensure your plugin's `package.json` includes:

```json
{
  "name": "package-fast-plugin-your-plugin",
  "version": "1.0.0",
  "description": "Brief description of your plugin",
  "main": "dist/index.js",
  "types": "dist/index.d.ts",
  "keywords": ["package-fast", "plugin"],
  "peerDependencies": {
    "package-fast": "^0.1.0"
  },
  "publishConfig": {
    "access": "public"
  }
}
```

### Documentation

Include comprehensive documentation:
1. README.md with usage instructions
2. API documentation
3. Examples and use cases
4. Configuration options

## Getting Help

### Community Resources

- [GitHub Issues](https://github.com/nom-nom-hub/package-fast/issues)
- [Discord Community](#)
- [Plugin Registry](#)

### Contributing

If you'd like to contribute to the plugin system itself:
1. Read the [Contributing Guide](../developer-guides/contributing.md)
2. Check existing plugin-related issues
3. Submit pull requests for improvements

## See Also

- [Plugin API Reference](../api-reference/plugin-api.md) - Detailed API documentation
- [CLI Commands](../api-reference/cli-commands.md) - Plugin-related commands
- [Configuration Options](../api-reference/configuration-options.md) - Plugin configuration