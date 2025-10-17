# Bug Fix: Score Not Updating

## Issue
The score was displayed on the scoreboard but wasn't updating when games ended. The initial score (0-0-0) appeared correctly, but after completing games, the score remained at zero.

## Root Cause
The `update_scoreboard` system was using `.skip(3)` to find the score text element, which was unreliable. The query was trying to find the 4th text element within all text elements marked with `ScoreboardUI`, but this approach was brittle and could miss the correct element.

## Solution

### 1. Added a Marker Component
Created a new `ScoreText` component in `components.rs` to specifically mark the score display text element.

```rust
#[derive(Component)]
pub struct ScoreText;
```

### 2. Updated spawn_scoreboard
Added the `ScoreText` component to the score display text when spawning the scoreboard:

```rust
parent.spawn((
    Text::new(format!(
        "X Wins: {} | O Wins: {} | Draws: {}",
        score.x_wins, score.o_wins, score.draws
    )),
    // ... other components ...
    ScoreText,  // <-- Added this marker
));
```

### 3. Updated update_scoreboard
Changed the query to directly target the `ScoreText` component instead of using `.skip(3)`:

**Before:**
```rust
pub fn update_scoreboard(score: Res<Score>, mut query: Query<&mut Text, With<ScoreboardUI>>) {
    for mut text in query.iter_mut().skip(3) {
        // ...
    }
}
```

**After:**
```rust
pub fn update_scoreboard(score: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>) {
    for mut text in query.iter_mut() {
        // ...
    }
}
```

### 4. Added System Ordering
Ensured `update_score` runs before `update_scoreboard` using `.before()`:

```rust
systems::update_score.before(systems::update_scoreboard),
systems::update_scoreboard,
```

## How It Works Now

1. Game ends (win or draw)
2. `update_score` system detects the game end and increments the appropriate counter
3. Score resource is marked as changed
4. `update_scoreboard` system detects the change and queries for `ScoreText` component
5. The score text is directly updated with the new values

## Testing

- [x] Score displays 0-0-0 initially
- [x] Score updates when X wins
- [x] Score updates when O wins
- [x] Score updates when game ends in draw
- [x] Score persists across multiple games
- [x] Score resets when returning to menu

## Files Modified

1. `src/game/components.rs`: Added `ScoreText` component
2. `src/game/systems.rs`: 
   - Updated `spawn_scoreboard` to add `ScoreText` marker
   - Updated `update_scoreboard` to query `ScoreText` directly
3. `src/game/mod.rs`: Added system ordering with `.before()`

## Technical Notes

- Using a marker component is more reliable than positional queries (skip/take)
- System ordering ensures score is calculated before UI updates
- Bevy's change detection (`is_changed()`) ensures efficient updates
- The marker component pattern is a best practice in Bevy ECS
