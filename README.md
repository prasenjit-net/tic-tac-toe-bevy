# Tic-Tac-Toe (Bevy 0.17)

A feature-rich Tic-Tac-Toe game built with the Bevy game engine 0.17.

## Features

- 🎮 **Multiple Game Modes**: Human vs Human, Human vs AI (Easy/Hard), AI vs AI
- 🤖 **Smart AI**: Easy (random) and Hard (strategic) computer players
- 📊 **Score Tracking**: Running score display with X/O wins and draws
- 🎨 **Clean UI**: Menu system, scoreboard overlay, and game-over screen
- ⌨️ **Keyboard Controls**: Quick access with R (new game) and Esc (menu)
- 🎯 **Win Detection**: Highlights winning line in green

## Quick Start

### Prerequisites
- Rust toolchain (stable)

### Run the Game
```bash
cargo run
```

## Controls

### In Game
- **Left Click**: Place your mark (X or O)
- **R Key**: Start a new game
- **Esc Key**: Return to main menu

### In Menu
- Click a button to choose game mode

## Game Modes

1. **Human vs Human** - Two players on the same computer
2. **Human vs Computer (Easy)** - Play against random AI
3. **Human vs Computer (Hard)** - Play against strategic AI
4. **Computer vs Computer** - Watch AI battle itself

## Documentation

📚 **[Full Documentation Index](doc/INDEX.md)**

Key documents:
- [Implementation Guide](doc/IMPLEMENTATION_GUIDE.md) - How it's built
- [Features Completed](doc/FEATURES_COMPLETED.md) - Complete feature list
- [Bug Fixes](doc/BUG_FIX_UI_CLICK.md) - Bug fix history
- [Refactoring](doc/REFACTORING_COMPLETE.md) - Code structure

## Project Structure

```
src/
├── main.rs
└── game/
    ├── mod.rs          # Plugin definition
    ├── components.rs   # ECS components
    ├── state.rs        # Game state & resources
    ├── utils.rs        # Helper functions
    └── systems/        # Game systems (organized by feature)
        ├── camera.rs
        ├── menu.rs
        ├── scoreboard.rs
        ├── game.rs
        ├── ai.rs
        ├── game_over.rs
        └── rendering.rs
```

## Development

### Build
```bash
cargo build
```

### Run Tests
```bash
cargo test
```

### Code Quality
```bash
cargo clippy        # Linting
cargo fmt           # Formatting
```

## CI/CD

GitHub Actions workflows:
- **CI**: Builds on Linux/Windows/Mac for main branch, Linux only for PRs
- **Manual Release**: One-click releases with automatic versioning

See [Manual Release Quick Start](doc/MANUAL_RELEASE_QUICK_START.md) for details.

## Technical Details

- **Engine**: Bevy 0.17
- **Rendering**: Mesh-based (2D shapes, no external assets)
- **Architecture**: ECS (Entity Component System)
- **AI**: Strategic heuristics with win detection

## Notes

- No external assets required - all rendering uses Bevy primitives
- Win line is highlighted in green
- Draw occurs when the board is full with no winner
- Score resets when returning to menu
