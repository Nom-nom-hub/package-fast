# Contributing to Package Fast

Thank you for your interest in contributing to Package Fast! This document provides guidelines and information to help you contribute effectively to the project.

## Code of Conduct

Please read and follow our [Code of Conduct](CODE_OF_CONDUCT.md) to ensure a welcoming and inclusive environment for all contributors.

## Getting Started

### Prerequisites

Before you start contributing, ensure you have:

1. Rust toolchain (latest stable version)
2. Node.js (version 16 or higher)
3. Git
4. Basic knowledge of Rust and Node.js

### Development Setup

1. Fork the repository on GitHub
2. Clone your fork:
   ```bash
   git clone https://github.com/your-username/package-fast.git
   cd package-fast
   ```
3. Install dependencies:
   ```bash
   npm install
   ```
4. Build the project:
   ```bash
   cargo build
   npm run build
   ```
5. Run tests to verify setup:
   ```bash
   cargo test
   npm test
   ```

For detailed setup instructions, see the [Development Setup Guide](development-setup.md).

## How to Contribute

### Reporting Bugs

Before reporting a bug, please check if it has already been reported in our [issue tracker](https://github.com/nom-nom-hub/package-fast/issues).

When reporting a bug, include:
1. A clear and descriptive title
2. Steps to reproduce the issue
3. Expected behavior
4. Actual behavior
5. Environment information (OS, Package Fast version, Node.js version)
6. Any relevant logs or error messages

### Suggesting Features

Feature requests are welcome! Before creating a feature request:
1. Check if it's already been requested
2. Consider if it aligns with Package Fast's goals

When suggesting a feature, include:
1. A clear and descriptive title
2. Detailed description of the proposed feature
3. Use cases and benefits
4. Potential implementation approaches (if known)

### Code Contributions

#### Finding Issues to Work On

Look for issues labeled:
- `good first issue` - Suitable for newcomers
- `help wanted` - Ready for contribution
- `bug` - Fixes for existing issues

#### Development Workflow

1. Create a new branch for your work:
   ```bash
   git checkout -b feature/your-feature-name
   ```
2. Make your changes, following our [Code Style Guide](code-style.md)
3. Add tests for your changes
4. Ensure all tests pass:
   ```bash
   cargo test
   npm test
   ```
5. Commit your changes using [Conventional Commits](https://www.conventionalcommits.org/):
   ```bash
   git commit -m "feat: add new feature"
   ```
6. Push your branch:
   ```bash
   git push origin feature/your-feature-name
   ```
7. Create a pull request

#### Pull Request Guidelines

When creating a pull request:
1. Provide a clear title and description
2. Reference any related issues
3. Include tests for new functionality
4. Update documentation as needed
5. Ensure CI checks pass

### Code Review Process

All pull requests go through a review process:
1. Automated checks (tests, linting, etc.)
2. Review by core team members
3. Address feedback
4. Approval and merge

## Development Guidelines

### Project Structure

```
/package-fast
├── crates/
│   ├── cli/          # CLI interface (Node.js)
│   ├── core/         # Core functionality (Rust)
│   └── security/     # Security features (Rust)
├── docs/             # Documentation
├── tests/            # Integration tests
├── benchmarks/       # Performance benchmarks
└── packages/         # Additional packages
```

### Branching Strategy

- `main` - Production-ready code
- `develop` - Integration branch for completed features
- `feature/*` - Individual feature development
- `release/*` - Preparation for releases
- `hotfix/*` - Urgent production fixes

### Commit Messages

We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:
- `feat:` - New features
- `fix:` - Bug fixes
- `docs:` - Documentation changes
- `style:` - Code style changes
- `refactor:` - Code refactoring
- `test:` - Test changes
- `chore:` - Maintenance tasks

### Testing

All contributions must include appropriate tests:
1. Unit tests for new functionality
2. Integration tests for complex features
3. Performance tests for performance-critical code

Run tests with:
```bash
# Rust tests
cargo test

# Node.js tests
npm test

# Integration tests
npm run test:integration

# Performance tests
npm run test:performance
```

## Code Style

### Rust

Follow the [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/README.html) and use:
- `rustfmt` for formatting
- `clippy` for linting

```bash
# Format code
cargo fmt

# Run linter
cargo clippy
```

### Node.js/TypeScript

Follow common TypeScript best practices and use:
- ESLint for linting
- Prettier for formatting

```bash
# Lint code
npm run lint

# Format code
npm run format
```

## Documentation

When making changes, update relevant documentation:
1. User guides for feature changes
2. API documentation for interface changes
3. Developer guides for development workflow changes

## Performance Considerations

Package Fast prioritizes performance:
1. Profile changes that may affect performance
2. Include benchmarks for performance-critical code
3. Optimize for common use cases

## Security

Security is paramount:
1. Follow secure coding practices
2. Report security vulnerabilities responsibly
3. Update dependencies regularly

## Community

### Communication

Join our community channels:
- [Discord](#) - Real-time chat
- [GitHub Discussions](#) - Longer-form discussions
- [Twitter](#) - Announcements and updates

### Recognition

Contributors are recognized in:
- Release notes
- Contributor list
- Community spotlights

## License

By contributing to Package Fast, you agree that your contributions will be licensed under the MIT License. See [LICENSE](../LICENSE) for details.

## Questions?

If you have questions about contributing:
1. Check existing documentation
2. Join our Discord community
3. Create an issue with your question
4. Contact core team members directly

Thank you for contributing to Package Fast!