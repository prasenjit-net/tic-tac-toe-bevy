# Documentation Index

This directory contains all documentation for the Tic-Tac-Toe Bevy project.

## 📖 Getting Started

- **[README.md](../README.md)** - Main project overview (in root)
- **[IMPLEMENTATION_GUIDE.md](IMPLEMENTATION_GUIDE.md)** - Step-by-step implementation guide
- **[FEATURES_COMPLETED.md](FEATURES_COMPLETED.md)** - Complete feature list

## 🐛 Bug Fixes

- **[BUG_FIX_UI_CLICK.md](BUG_FIX_UI_CLICK.md)** - Fixed UI button click passthrough
- **[BUG_FIX_SCORE_UPDATE.md](BUG_FIX_SCORE_UPDATE.md)** - Fixed score not updating after games
- **[BUG_FIX_PLAYER_O_CIRCLE.md](BUG_FIX_PLAYER_O_CIRCLE.md)** - Fixed Player O displaying as square
- **[BUG_FIX_CIRCLES_NOT_DESPAWNING.md](BUG_FIX_CIRCLES_NOT_DESPAWNING.md)** - Fixed circles not despawning on reset

## ✨ Enhancements

- **[ENHANCEMENT_UNIFIED_MESH_RENDERING.md](ENHANCEMENT_UNIFIED_MESH_RENDERING.md)** - Unified mesh-based rendering for X and O
- **[SCORE_DISPLAY_ENHANCEMENT.md](SCORE_DISPLAY_ENHANCEMENT.md)** - Running score display implementation

## 🔧 Code Quality

- **[CLIPPY_LINT_FIXES.md](CLIPPY_LINT_FIXES.md)** - Clippy lint warnings resolution
- **[REFACTORING_PLAN.md](REFACTORING_PLAN.md)** - Analysis and plan for systems.rs refactoring
- **[REFACTORING_COMPLETE.md](REFACTORING_COMPLETE.md)** - Detailed refactoring results
- **[REFACTORING_SUCCESS.md](REFACTORING_SUCCESS.md)** - Quick refactoring summary

## 🚀 CI/CD & Releases

- **[WORKFLOW_ANALYSIS.md](WORKFLOW_ANALYSIS.md)** - GitHub Actions workflow analysis (cd.yml vs manual-release.yml)
- **[MANUAL_RELEASE_WORKFLOW.md](MANUAL_RELEASE_WORKFLOW.md)** - Complete manual release workflow documentation
- **[MANUAL_RELEASE_QUICK_START.md](MANUAL_RELEASE_QUICK_START.md)** - Quick reference for manual releases

## 📁 Document Organization

All documentation files are organized in this `doc/` directory. The main `README.md` remains in the project root for visibility on GitHub.

### Document Categories

```
doc/
├── BUG_FIX_*.md              # Bug fix documentation
├── ENHANCEMENT_*.md           # Feature enhancements
├── REFACTORING_*.md          # Code refactoring docs
├── MANUAL_RELEASE_*.md       # Release workflow docs
├── CLIPPY_LINT_FIXES.md     # Code quality improvements
├── FEATURES_COMPLETED.md     # Feature tracking
├── IMPLEMENTATION_GUIDE.md   # Implementation details
├── SCORE_DISPLAY_*.md        # Score feature docs
├── WORKFLOW_ANALYSIS.md      # CI/CD analysis
└── INDEX.md                  # This file
```

## 🎯 Quick Links

### For Developers
- [Implementation Guide](IMPLEMENTATION_GUIDE.md) - How features are implemented
- [Refactoring Complete](REFACTORING_COMPLETE.md) - Code structure overview

### For Contributors
- [Bug Fixes](BUG_FIX_UI_CLICK.md) - Examples of bug fixes
- [Code Quality](CLIPPY_LINT_FIXES.md) - Maintaining code quality

### For Maintainers
- [Manual Release](MANUAL_RELEASE_QUICK_START.md) - How to release
- [Workflow Analysis](WORKFLOW_ANALYSIS.md) - CI/CD setup

## 📝 Creating New Documentation

When creating new documentation:

1. **Save to `doc/` directory** - All documentation goes here
2. **Use descriptive names** - `CATEGORY_DESCRIPTION.md` format
3. **Update this index** - Add links to new documents
4. **Follow categories**:
   - `BUG_FIX_*` - Bug fix documentation
   - `ENHANCEMENT_*` - Feature enhancements
   - `REFACTORING_*` - Code refactoring
   - `MANUAL_*` - Manual processes/workflows
   - Standalone names for other docs

## 🔍 Search Tips

To find documentation:
```bash
# List all docs
ls doc/

# Search for specific topic
grep -r "topic" doc/

# View a specific doc
cat doc/DOCUMENT_NAME.md
```

---

**Last Updated:** October 17, 2025
