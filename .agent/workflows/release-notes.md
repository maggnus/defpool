---
description: Generate release notes and update changelog
---

# Release Notes & Changelog Workflow

Always maintain CHANGELOG.md and generate release notes when completing major features or milestones.

## When to Update

Update the changelog:
- ✅ After completing a major feature
- ✅ After fixing critical bugs
- ✅ Before creating a git tag/release
- ✅ After significant refactoring
- ✅ When API changes occur

## Changelog Format

Follow [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) format:

```markdown
## [Version] - YYYY-MM-DD

### Added
- New features

### Changed
- Changes to existing functionality

### Deprecated
- Soon-to-be removed features

### Removed
- Removed features

### Fixed
- Bug fixes

### Security
- Security fixes
```

## Steps

### 1. Update CHANGELOG.md

Add entry under `[Unreleased]` section:

```bash
# Edit CHANGELOG.md
vim CHANGELOG.md
```

Example:
```markdown
## [Unreleased]

### Added
- Real-time profitability calculation with CoinGecko API
- Multi-pool support with automatic switching

### Changed
- Server configuration now uses TOML arrays for pools
```

### 2. Generate Release Notes

When ready to release, move `[Unreleased]` to a versioned section:

```markdown
## [0.2.0] - 2025-12-07

### Added
- Real-time profitability calculation with CoinGecko API
- Multi-pool support with automatic switching
```

### 3. Create Git Tag

```bash
git tag -a v0.2.0 -m "Release v0.2.0: Multi-pool profitability switching"
git push origin v0.2.0
```

### 4. Generate GitHub Release Notes

Use the changelog entry as the release description:

```markdown
# DefPool v0.2.0

## 🎉 What's New

### Added
- **Real API Integration**: CoinGecko for prices, MoneroBlocks for difficulty
- **Multi-Pool Support**: Configure multiple pools, automatic switching
- **Profitability System**: Real-time calculation every 60s with 5% hysteresis

### Changed
- Server configuration now supports multiple pools via TOML
- State management refactored for pool tracking

## 📦 Installation

\`\`\`bash
docker-compose up -d
\`\`\`

## 🔗 Links
- [Full Changelog](CHANGELOG.md)
- [Documentation](doc/design.md)
```

## Semantic Versioning

Follow [SemVer](https://semver.org/):

- **MAJOR** (1.0.0): Breaking changes
- **MINOR** (0.1.0): New features, backward compatible
- **PATCH** (0.0.1): Bug fixes, backward compatible

Examples:
- `0.1.0 → 0.2.0`: Added multi-pool support (new feature)
- `0.2.0 → 0.2.1`: Fixed profitability calculation bug (bug fix)
- `0.2.1 → 1.0.0`: Changed API endpoints (breaking change)

## Automation

### Pre-commit Hook

Create `.git/hooks/pre-commit`:
```bash
#!/bin/bash
# Check if CHANGELOG.md was updated
if git diff --cached --name-only | grep -q "CHANGELOG.md"; then
    echo "✅ CHANGELOG.md updated"
else
    echo "⚠️  Consider updating CHANGELOG.md"
fi
```

### CI/CD Integration

Add to `.github/workflows/release.yml`:
```yaml
name: Release
on:
  push:
    tags:
      - 'v*'
jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Extract changelog
        run: |
          sed -n '/^## \['"${GITHUB_REF#refs/tags/v}"'\]/,/^## \[/p' CHANGELOG.md > release-notes.md
      - name: Create Release
        uses: actions/create-release@v1
        with:
          tag_name: ${{ github.ref }}
          release_name: Release ${{ github.ref }}
          body_path: release-notes.md
```

## AI Agent Rule

**Always:**
1. Update `CHANGELOG.md` after completing features
2. Use proper categories (Added, Changed, Fixed, etc.)
3. Include both user-facing and technical details
4. Follow semantic versioning
5. Generate release notes before tagging
6. Keep entries concise but informative
