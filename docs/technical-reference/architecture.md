# Package Fast Architecture

This document provides a comprehensive overview of Package Fast's architecture, including its components, data flow, and design principles.

## System Overview

Package Fast is a next-generation Node.js package manager built with a hybrid architecture that combines the performance of Rust with the ecosystem compatibility of Node.js. The system is designed to provide 5-10x faster installations while maintaining full compatibility with the npm ecosystem.

## Core Components

### 1. CLI Interface Layer

The CLI interface layer provides the user-facing command-line interface built with Node.js. It handles:
- Command parsing and validation
- User interaction and progress reporting
- Help system and documentation
- Plugin system integration

**Key Features:**
- Intuitive command structure
- Comprehensive help system with examples
- Clear progress reporting during operations
- Cross-platform compatibility

### 2. Package Resolution Engine

The package resolution engine, implemented in Rust for performance, handles:
- Dependency graph construction
- Version constraint solving
- Lockfile generation and validation
- Workspace-aware resolution

**Key Features:**
- Fast dependency resolution using SAT solver
- Deterministic lockfile generation
- Advanced conflict resolution strategies
- Workspace-aware dependency management

### 3. Fetch and Cache Manager

The fetch and cache manager handles:
- Registry communication
- Package metadata caching
- Binary package downloading
- Content-addressed storage

**Key Features:**
- Multi-level caching strategy
- Efficient network utilization
- Content-addressed storage with deduplication
- Registry compatibility

### 4. File System Operations Layer

The file system operations layer provides:
- Atomic file operations
- Hard link and copy-on-write optimizations
- Cross-platform path handling
- Performance-optimized I/O operations

**Key Features:**
- Atomic operations with rollback support
- Hard link utilization for duplicate files
- Cross-platform compatibility
- Efficient I/O scheduling

### 5. Installation Executor

The installation executor manages:
- Parallel package installation
- Post-install script execution
- Node modules directory structure management
- Progress tracking and reporting

**Key Features:**
- Parallel processing for all operations
- Resource-aware concurrency management
- Secure script execution
- Detailed progress reporting

### 6. Workspace Manager

The workspace manager handles:
- Monorepo coordination
- Cross-package dependency linking
- Workspace-aware command execution
- Dependency sharing optimization

**Key Features:**
- Automatic workspace discovery
- Intelligent dependency hoisting
- Parallel workspace operations
- Circular dependency detection

### 7. Plugin System

The plugin system enables:
- Extensibility hooks
- Custom command registration
- Middleware pipeline
- Secure plugin execution

**Key Features:**
- Well-defined extension points
- Secure sandboxing mechanism
- Performance-optimized loading
- Backward compatibility

### 8. Security Module

The security module provides:
- Package integrity verification
- Vulnerability scanning
- Audit trail generation
- Runtime protection

**Key Features:**
- Cryptographic package verification
- Real-time vulnerability detection
- Comprehensive audit logging
- Secure script sandboxing

## Data Flow

### Package Installation Flow

1. **User Command** → CLI Parser
2. **CLI Parser** → Workspace Detection
3. **Workspace Detection** → Dependency Resolution
4. **Dependency Resolution** → Registry Query
5. **Registry Query** → Cache Check
6. **Cache Check** → Local Package Extraction (Hit) or Package Download (Miss)
7. **Package Download** → Package Verification
8. **Package Verification** → Local Package Extraction
9. **Local Package Extraction** → Parallel Installation
10. **Parallel Installation** → Post-install Scripts
11. **Post-install Scripts** → Lockfile Update
12. **Lockfile Update** → Completion

### Monorepo Workflow

1. **Workspace Command** → Workspace Discovery
2. **Workspace Discovery** → Cross-package Dependency Analysis
3. **Cross-package Dependency Analysis** → Hoisting Strategy
4. **Hoisting Strategy** → Parallel Package Resolution
5. **Parallel Package Resolution** → Shared Cache Utilization
6. **Shared Cache Utilization** → Distributed Installation
7. **Distributed Installation** → Internal Linking
8. **Internal Linking** → Workspace-specific Scripts
9. **Workspace-specific Scripts** → Completion

## Storage and Caching

### Multi-level Caching Strategy

1. **Memory Cache**
   - LRU cache for frequently accessed metadata
   - Size: Configurable (default 100MB)
   - Scope: Per-process

2. **Local Package Cache**
   - Location: `~/.package-fast/cache`
   - Structure: Content-addressable storage (CAS) with tarball fingerprints
   - Deduplication: Hard links for identical packages
   - Compression: Optional gzip for older packages

3. **Registry Metadata Cache**
   - Time-based invalidation (configurable TTL)
   - Selective refresh for frequently updated packages
   - Differential updates when supported by registry

4. **Workspace Cache**
   - Local to project directory
   - Stores resolved dependency trees
   - Incremental update tracking

### Cache Invalidation Strategy

- Content-based addressing for package tarballs
- Timestamp-based for registry metadata
- Git-aware for workspace packages
- Manual invalidation commands

## Dependency Resolution Approach

### Algorithm

1. **Constraint Collection**
   - Gather all dependency constraints from package.json files
   - Collect peerDependencies and optionalDependencies
   - Apply workspace protocol resolution

2. **Graph Construction**
   - Build initial dependency graph
   - Detect circular dependencies
   - Identify conflicting version requirements

3. **Resolution Strategy**
   - Prefer latest compatible versions
   - Implement hoisting for deduplication
   - Use SAT solver for complex constraint solving
   - Fallback to npm-compatible resolution when needed

4. **Lockfile Generation**
   - Deterministic output
   - Include integrity hashes
   - Store resolution metadata for audits

### Advanced Features

- **Selective Version Resolution**: Allow pinning specific dependencies to newer versions
- **Resolution Overrides**: Provide escape hatches for version conflicts
- **Peer Dependency Auto-installation**: Optionally install missing peer dependencies

## Parallel Processing Strategies

### Installation Parallelization

1. **Dependency-level Parallelism**
   - Install independent dependency subtrees concurrently
   - Limit by configurable worker pool (default: CPU cores + 2)

2. **Package-level Parallelism**
   - Extract multiple packages simultaneously
   - Utilize separate I/O threads for file operations

3. **Script Execution Parallelism**
   - Run lifecycle scripts with controlled concurrency
   - Isolate environment variables and working directories

### Resource Management

- **CPU-bound Tasks**: Resolution, integrity checking
- **I/O-bound Tasks**: Downloading, file extraction, linking
- **Network-bound Tasks**: Registry queries, package downloads

### Throttling Mechanisms

- Adaptive concurrency based on system resources
- Network bandwidth monitoring
- Disk I/O saturation detection

## Plugin/Extensibility Architecture

### Hook System

1. **Pre-execution Hooks**
   - Command validation
   - Environment setup

2. **Resolution Hooks**
   - Custom registry endpoints
   - Package aliasing
   - Version manipulation

3. **Installation Hooks**
   - Pre/post install scripts
   - File transformation
   - Custom linking strategies

4. **Post-execution Hooks**
   - Result transformation
   - Reporting generation

### Plugin Interface

Plugins implement a well-defined interface:

```typescript
interface Plugin {
  name: string;
  version: string;
  hooks: {
    // Resolution phase
    resolvePackage?(pkg: PackageInfo): Promise<PackageInfo>;
    
    // Installation phase
    beforeInstall?(pkg: InstalledPackage): Promise<void>;
    afterInstall?(pkg: InstalledPackage): Promise<void>;
    
    // Command extension
    registerCommands?(): Command[];
  };
}
```

### Plugin Loading

- Auto-discovery from node_modules
- Configuration-based loading
- Security verification of plugins

## Security Implementation

### Package Verification

1. **Integrity Checking**
   - SHA-512 hashes for all packages
   - Signature verification when available
   - Transparency logs integration

2. **Vulnerability Scanning**
   - Integration with security databases
   - Real-time vulnerability alerts
   - Automated remediation suggestions

3. **Supply Chain Security**
   - Publisher verification
   - Two-factor authentication status checking
   - Package provenance tracking

### Runtime Security

- Sandbox execution of lifecycle scripts
- Permission-based file system access
- Network access control for scripts

### Audit Trail

- Comprehensive logging of all operations
- Change tracking for dependency modifications
- Exportable security reports

## Monorepo/Workspace Handling

### Workspace Discovery

- Automatic detection of workspace configurations
- Support for multiple workspace formats (npm, yarn, pnpm)
- Custom workspace definition files

### Cross-package Linking

- Symbolic link management for internal dependencies
- Hoisting optimization across workspaces
- Circular dependency detection and handling

### Workspace-aware Commands

- Targeted execution for specific packages
- Dependency-aware command chaining
- Parallel execution with dependency ordering

### Advanced Workspace Features

- **Scoped Operations**: Run commands in specific workspace subsets
- **Cross-workspace Dependencies**: Handle dependencies between separate workspace roots
- **Incremental Builds**: Only rebuild affected workspace packages

## Network Optimization Techniques

### Connection Management

- Persistent HTTP/HTTPS connections
- HTTP/2 support for compatible registries
- Connection pooling with adaptive sizing

### Request Optimization

- Bulk metadata requests
- Delta updates when supported
- Compression for all network traffic

### CDN and Mirror Support

- Automatic mirror selection based on latency
- Fallback mechanisms for failed requests
- Custom registry endpoint configuration

### Offline Support

- Comprehensive local caching
- Offline-first resolution strategy
- Clear offline/online state transitions

## File System Interaction Patterns

### Atomic Operations

- Temporary directory staging
- Atomic moves for final placement
- Rollback mechanisms for failed operations

### Performance Optimizations

- Hard link utilization for duplicate files
- Copy-on-write when supported by filesystem
- Parallel file extraction with I/O prioritization

### Cross-platform Considerations

- Windows junction points for directory linking
- Unix symbolic links where appropriate
- Path length limitations handling

### File System Layout

```
node_modules/
├── .package-fast/          # Internal metadata
│   ├── store/             # Hard links to global store
│   └── index.json         # Local package index
├── .bin/                  # Executable links
└── package-name/          # Actual packages or links
    └── node_modules/      # Nested dependencies
```

### Global Store Structure

```
~/.package-fast/
├── cache/                 # Downloaded packages
├── store/                 # Extracted packages (content-addressed)
├── metadata/              # Registry metadata
└── config.json            # Global configuration
```

## Performance Targets

1. **Installation Speed**: 5-10x faster than npm for clean installs
2. **Resolution Speed**: Sub-second for most projects
3. **Disk Usage**: 50% reduction through deduplication
4. **Memory Usage**: Efficient garbage collection during operations
5. **Network Usage**: Minimal requests through intelligent caching

## Compatibility Guarantees

1. **Package.json Support**: Full npm compatibility
2. **Lockfile Formats**: Support for package-lock.json, yarn.lock
3. **Registry Compatibility**: Works with npm, yarn, and custom registries
4. **Script Execution**: Lifecycle script compatibility
5. **Environment Variables**: Standard npm environment variables

## See Also

- [Performance Optimization](performance.md) - Detailed performance strategies
- [Security Implementation](security.md) - Security features in depth
- [Plugin API](../api-reference/plugin-api.md) - Extending Package Fast