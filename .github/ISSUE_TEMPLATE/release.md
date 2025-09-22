---
name: Release
about: Track a new Package Fast release
title: 'Release vX.X.X'
labels: 'release'
assignees: ''
---

## Release Checklist

- [ ] Update version numbers in all package.json and Cargo.toml files
- [ ] Update CHANGELOG.md with release notes
- [ ] Run full test suite on all platforms
- [ ] Build release binaries for all platforms
- [ ] Test release binaries
- [ ] Create git tag and push
- [ ] Verify GitHub Actions release workflow
- [ ] Verify npm package publication
- [ ] Verify GitHub release creation
- [ ] Test installation from npm
- [ ] Update documentation if needed
- [ ] Announce release