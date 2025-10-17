# Tic-Tac-Toe Game Enhancement Implementation Guide

This guide shows how to add a menu system, AI players (Easy/Hard), scoreboard, and game over screen to the Bevy tic-tac-toe game.

## Summary of Changes

1. **Add `rand` dependency** for AI randomness
2. **Update components.rs** - Add UI components and PlayerType enum
3. **Update state.rs** - Add game states, player config, score tracking
4. **Update systems.rs** - Add menu, AI, scoreboard, and game over systems
5. **Update mod.rs** - Wire everything with state management

## Detailed Changes

### 1. Cargo.toml
Add `rand` dependency:
```toml
[dependencies]
bevy = { version = "0.17", default-features = true }
rand = "0.8"
```

### 2. components.rs
Add these new components and types:
```rust
#[derive(Component)]
pub struct MenuUI;

#[derive(Component)]
pub struct ScoreboardUI;

#[derive(Component)]
pub struct GameOverUI;

#[derive(Component)]
pub struct MenuButton {
    pub x_type: PlayerType,
    pub o_type: PlayerType,
}

#[derive(Component)]
pub struct BackToMenuButton;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PlayerType {
    Human,
    ComputerEasy,
    ComputerHard,
}

impl PlayerType {
    pub fn label(&self) -> &str {
        match self {
            PlayerType::Human => "Human",
            PlayerType::ComputerEasy => "Computer Easy",
            PlayerType::ComputerHard => "Computer Hard",
        }
    }
}
```

### 3. state.rs
Key additions:
- Import `PlayerType` from components
- Add `other()` method to `Player` enum
- Add `AppState` enum with `Menu` and `Playing` states
- Add `reset()` method to `GameState`
- Add `PlayerConfig`, `Score`, and `ComputerMoveTimer` resources

### 4. systems.rs
New systems to add:
- `spawn_menu()` - Creates menu UI
- `handle_menu_buttons()` - Handles menu button clicks
- `cleanup_menu()` - Cleans up menu when transitioning
- `spawn_scoreboard()` - Creates scoreboard overlay
- `update_scoreboard()` - Updates scoreboard with scores
- `computer_player()` - AI logic with random delay
- `show_game_over_ui()` - Shows game over screen
- `handle_game_over_buttons()` - Handles game over buttons
- `keyboard_controls()` - R to restart, Esc to return to menu
- `cleanup_game()` - Cleans up game state when returning to menu

Helper functions for AI:
- `find_computer_move()` - Routes to easy/hard AI
- `find_easy_move()` - Random move selection
- `find_hard_move()` - Strategic moves (win, block, center, corner)
- `find_winning_move()` - Finds winning moves
- `make_move()` - Applies a move to the game state

### 5. mod.rs
Update the plugin to use states and register all systems:
- Initialize `AppState` and all resources
- Add `OnEnter(AppState::Menu)` systems
- Add `Update` systems that run only in specific states
- Add `OnExit` systems for cleanup

## Features

### Menu Screen
- Human vs Human
- Human vs Computer (Easy)
- Human vs Computer (Hard)
- Computer vs Computer

### AI Difficulty
**Easy**: Random move selection
**Hard**: 
1. Try to win
2. Block opponent from winning
3. Take center
4. Take a corner
5. Take any available spot

### Random Delay
Computer players wait 200-800ms before making a move (configurable)

### Scoreboard
Shows:
- Player X type (e.g., "Human")
- Player O type (e.g., "Computer Hard")
- Win/Draw statistics

### Game Over Screen
- Shows winner or draw message
- "New Game (R)" button - starts new game with same players
- "Back to Menu (Esc)" button - returns to menu, resets scores

### Keyboard Controls
- **R** - New game (during or after game)
- **Esc** - Return to menu (from playing state)

## Testing

Run `cargo build` and then `cargo run` to test:
1. Menu appears on startup
2. Select different game modes
3. Play against AI (Easy should be beatable, Hard should be challenging)
4. Check scoreboard updates
5. Test game over screen
6. Test keyboard shortcuts

## Architecture

The game uses Bevy's state system to manage transitions between Menu and Playing states. Each state has its own set of systems that run when the state is active, and cleanup systems that run when exiting the state.

Resources track:
- Current game board state
- Player configuration (Human/AI types)
- Cumulative scores
- AI move timer

This keeps the code organized and prevents systems from running when they shouldn't.
