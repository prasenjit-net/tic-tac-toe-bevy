# Enhancement: Running Score Display

## Feature
Added a persistent running score display on the scoreboard that shows:
- **X Wins**: Number of games won by Player X
- **O Wins**: Number of games won by Player O
- **Draws**: Number of drawn games

## Implementation

### Changes Made:

1. **Enhanced `spawn_scoreboard` function**:
   - Added `score: Res<Score>` parameter to access current scores
   - Added a separator line (in gray)
   - Added a fourth text element showing the running score with format: "X Wins: N | O Wins: N | Draws: N"
   - Score is displayed immediately when entering the game (starts at 0-0-0)

2. **Updated `update_scoreboard` function**:
   - Changed from `skip(2)` to `skip(3)` to update the correct text element
   - This ensures the score line (4th text element) is updated when games end

3. **Score Reset on Menu Return**:
   - Already implemented in `cleanup_game` function
   - When returning to menu, `*score = Score::default()` resets all scores to 0

## Behavior

### During Gameplay:
- Scoreboard appears in the top-left corner
- Shows player types (e.g., "Human", "Computer Easy", "Computer Hard")
- Shows running score: "X Wins: N | O Wins: N | Draws: N"
- Score updates automatically when each game ends
- Score persists across multiple games

### On Return to Menu:
- Pressing **Esc** or clicking **Back to Menu** button
- All scores reset to 0
- When starting a new game mode, scores start fresh at 0-0-0

### Visual Layout:
```
┌─────────────────────────────────┐
│ Player X: Human                 │
│ Player O: Computer Hard         │
│ ─────────────────               │ (separator)
│ X Wins: 3 | O Wins: 1 | Draws: 0│
└─────────────────────────────────┘
```

## Testing Checklist

- [x] Scoreboard displays initial scores (0-0-0) when game starts
- [x] X wins increments when Player X wins
- [x] O wins increments when Player O wins  
- [x] Draws increments when game ends in a draw
- [x] Score persists across multiple games
- [x] Score resets to 0-0-0 when returning to menu
- [x] Score resets when selecting a new game mode from menu
- [x] Scoreboard properly displays player types
- [x] UI is clean and readable

## Files Modified
- `src/game/systems.rs`:
  - Enhanced `spawn_scoreboard()` to include initial score display
  - Updated `update_scoreboard()` to update the correct text element
  - Score reset already working via `cleanup_game()`

## Technical Details

### Score Tracking:
- Uses `Score` resource with fields: `x_wins`, `o_wins`, `draws`
- `update_score()` system increments counters when games end
- `update_scoreboard()` system updates the UI text when score changes
- Bevy's change detection ensures efficient updates

### State Management:
- Score persists in `Playing` state
- Score resets when exiting `Playing` state (OnExit system)
- Fresh scores when entering `Playing` state again
