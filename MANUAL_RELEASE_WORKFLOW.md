# Manual Release Workflow

## Overview
This workflow allows you to manually trigger a release build from the GitHub Actions UI. It will automatically:
1. Bump the version in `Cargo.toml`
2. Create a git tag
3. Build binaries for all platforms
4. Create a GitHub release with all binaries and changelog

## How to Use

### From GitHub UI

1. Go to the **Actions** tab in your GitHub repository
2. Click on **Manual Release** workflow in the left sidebar
3. Click the **Run workflow** button (top right)
4. Choose your options:
   - **Version bump type**: Select `major`, `minor`, or `patch`
   - **Custom version** (optional): Enter a specific version like `1.0.0` to override the bump type
5. Click **Run workflow**

### Version Bumping

The workflow supports three types of version bumps:

- **Patch** (0.1.0 → 0.1.1): Bug fixes and minor changes
- **Minor** (0.1.0 → 0.2.0): New features, backward compatible
- **Major** (0.1.0 → 1.0.0): Breaking changes

#### Examples

Current version: `0.1.0`

| Bump Type | New Version | Use Case |
|-----------|-------------|----------|
| patch     | 0.1.1       | Bug fixes, small improvements |
| minor     | 0.2.0       | New features (menu system, AI players) |
| major     | 1.0.0       | Complete rewrite or breaking API changes |

### Custom Version

If you need a specific version number (e.g., jumping to 1.0.0 for initial release):
1. Leave **Version bump type** at default
2. Enter the desired version in **Custom version** field (e.g., `1.0.0`)
3. The custom version will override the bump type

## What Happens

### Step 1: Version Update
- Reads current version from `Cargo.toml`
- Calculates new version based on your input
- Updates `Cargo.toml` with new version
- Updates `Cargo.lock` to match
- Commits changes to `main` branch

### Step 2: Tag Creation
- Creates an annotated git tag (e.g., `v0.2.0`)
- Pushes tag to GitHub

### Step 3: Build Release
- Builds binaries for 4 platforms:
  - Linux (x86_64)
  - Windows (x86_64)
  - macOS (Intel x86_64)
  - macOS (Apple Silicon aarch64)
- Each build is optimized with release profile

### Step 4: Create GitHub Release
- Downloads all built binaries
- Generates changelog from git commits
- Creates GitHub Release with:
  - All platform binaries
  - Auto-generated changelog
  - Commit history since last release

## Workflow Jobs

The workflow consists of 3 jobs that run in sequence:

1. **create-tag-and-release**: Bumps version, creates tag
2. **build-release**: Builds binaries for all platforms (runs in parallel)
3. **create-release**: Creates the GitHub release with all binaries

## Requirements

- **Permissions**: Requires `contents: write` permission (already configured)
- **Branch**: Must be run from `main` branch
- **Access**: Repository maintainers with workflow run permissions

## Automation

After the tag is created by this workflow, the existing `Release` workflow (`.github/workflows/cd.yml`) will also trigger automatically when it detects the new tag. To avoid duplication, you may want to:

- Keep both workflows (this one includes build in same workflow)
- OR rely on the existing `cd.yml` and modify this workflow to only create the tag

**Current Implementation**: This workflow includes the full build process to ensure everything completes in one run.

## Monitoring

You can monitor the workflow progress:
1. Go to **Actions** tab
2. Click on the running workflow
3. See real-time logs for each job
4. Download binaries from the Artifacts section (if release creation fails)

## Troubleshooting

### Build Fails
- Check the build logs for specific errors
- Ensure `Cargo.toml` changes don't break compilation
- Verify all dependencies are compatible

### Tag Already Exists
- If the tag already exists, the workflow will fail
- Delete the tag first: `git tag -d v0.x.x && git push origin :refs/tags/v0.x.x`
- Run the workflow again

### Release Not Created
- Check if the `create-release` job has proper permissions
- Verify artifacts were uploaded successfully
- Check the job logs for detailed error messages

## Example Usage Scenarios

### Scenario 1: Bug Fix Release
```
Current version: 0.2.2
Action: Select "patch"
Result: Creates v0.2.3 with bug fixes
```

### Scenario 2: New Feature Release
```
Current version: 0.2.3
Action: Select "minor"
Result: Creates v0.3.0 with new features
```

### Scenario 3: Jumping to 1.0
```
Current version: 0.3.0
Action: Custom version = "1.0.0"
Result: Creates v1.0.0 for initial stable release
```

## Integration with Existing Workflows

This workflow complements the existing workflows:

- **CI** (`.github/workflows/ci.yml`): Still runs on every push/PR
- **Release** (`.github/workflows/cd.yml`): Can coexist or be disabled
- **Manual Release**: Provides controlled release process with version management

## Version History

All version bumps are automatically committed to the repository with messages like:
```
chore: bump version to 0.2.0
```

This creates a clear version history in your git log.
