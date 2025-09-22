# Package Fast Scripts

This directory contains various scripts for building, testing, and deploying Package Fast.

## Build Scripts

- `build-release.js` - Builds the complete Package Fast release for distribution
- `publish.js` - Publishes Package Fast to npm and creates GitHub releases

## Release Scripts

- `release.js` - Automates version bumping, git tagging, and initiating the release process

## Utility Scripts

- `install.js` - Provides installation guidance for end users
- `progress-reporter.js` - Reports development progress (existing)
- `status-report.js` - Reports project status (existing)
- `benchmark-runner.js` - Runs performance benchmarks (existing)

## Usage

To create a new release:
```bash
node scripts/release.js 1.0.0
```

To build for release:
```bash
node scripts/build-release.js
```

To publish manually:
```bash
node scripts/publish.js
```