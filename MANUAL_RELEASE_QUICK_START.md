# Manual Release Workflow - Quick Start

## TL;DR

Go to **Actions** → **Manual Release** → **Run workflow**

Choose version bump type:
- **patch**: Bug fixes (0.1.0 → 0.1.1)
- **minor**: New features (0.1.0 → 0.2.0)  
- **major**: Breaking changes (0.1.0 → 1.0.0)

Or enter a custom version number.

## What It Does

1. ✅ Updates `Cargo.toml` version
2. ✅ Creates git tag (e.g., `v0.2.0`)
3. ✅ Builds binaries for:
   - Linux x86_64
   - Windows x86_64
   - macOS x86_64 (Intel)
   - macOS aarch64 (Apple Silicon)
4. ✅ Creates GitHub Release with:
   - All binaries
   - Auto-generated changelog
   - Release notes

## Files Created

- `.github/workflows/manual-release.yml` - The workflow
- `MANUAL_RELEASE_WORKFLOW.md` - Detailed documentation

## Features

✨ **Automatic Version Bumping**
- Reads current version from Cargo.toml
- Calculates and applies new version
- Commits changes automatically

✨ **Custom Version Support**
- Override automatic bumping
- Jump to any version (e.g., 1.0.0)

✨ **Full Automation**
- Tag creation
- Multi-platform builds
- Release creation
- All in one workflow run

✨ **No Manual Work**
- No need to edit Cargo.toml
- No need to create tags manually
- No need to build locally

## Quick Reference

| Current | Bump | Result | Use Case |
|---------|------|--------|----------|
| 0.1.0   | patch | 0.1.1 | Bug fixes |
| 0.1.0   | minor | 0.2.0 | New features |
| 0.1.0   | major | 1.0.0 | Breaking changes |
| 0.2.2   | custom: "1.0.0" | 1.0.0 | First stable |

## Next Steps

1. Commit and push the workflow file
2. Go to GitHub Actions
3. Try running your first manual release!

For detailed information, see `MANUAL_RELEASE_WORKFLOW.md`
