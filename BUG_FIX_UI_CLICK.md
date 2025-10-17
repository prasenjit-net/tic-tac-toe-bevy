# Bug Fix: UI Button Click Passthrough

## Issue
When clicking the "New Game" button in the game over screen, the click event was propagating through to the game board, causing a move to be registered on the middle cell immediately after the game reset.

## Root Cause
Both the `handle_game_over_buttons` system and the `handle_clicks` system were processing the same mouse click event in the same frame. After the button reset the game state, the click was still being processed by the board click handler.

## Solution
Added a check in `handle_clicks` to detect if any UI button is currently being interacted with (pressed or hovered). If so, the board click is ignored.

### Code Change
In the `handle_clicks` function, added a new parameter:
```rust
interaction_query: Query<&Interaction, With<Button>>
```

And added this check before processing board clicks:
```rust
// Don't process board clicks if interacting with any UI button
for interaction in &interaction_query {
    if *interaction == Interaction::Pressed || *interaction == Interaction::Hovered {
        return;
    }
}
```

This ensures that:
1. When a button is clicked (Pressed), board clicks are blocked
2. When a button is hovered, board clicks are also blocked (prevents accidental clicks)
3. Board clicks only work when not interacting with any UI elements

## Testing
- [x] Click "New Game" button - should not place a mark on the board
- [x] Click "Back to Menu" button - should not cause any board interaction
- [x] Menu buttons should work normally
- [x] Board clicks should work normally when no UI is being interacted with

## Files Modified
- `src/game/systems.rs`: Updated `handle_clicks` function
