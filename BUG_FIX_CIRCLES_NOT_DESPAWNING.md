# Bug Fix: Circles Not Despawning on Reset

## Issue
When the reset button is pressed or a new game starts, Player O circles remain on the screen and are not properly despawned.

## Root Cause
After changing Player O rendering from sprites to mesh-based circles (to fix the square appearance bug), the `draw_marks` query was still looking for entities with `Sprite` components:

```rust
marks: Query<(Entity, &mut Sprite, &mut Transform), With<Mark>>
```

This query could only find Player X marks (which use sprites), but not Player O marks (which use `Mesh2d` and `MeshMaterial2d` components). As a result, when clearing marks for a new game, the circles weren't included in the query and remained on screen.

## Solution
Changed the `draw_marks` query to only filter by the `Mark` component, without requiring `Sprite`:

### Changes Made

**Updated `draw_marks` system** (`src/game/systems.rs`, line 683):
- Changed from: `Query<(Entity, &mut Sprite, &mut Transform), With<Mark>>`
- Changed to: `Query<Entity, With<Mark>>`
- Simplified the iteration from `marks.iter_mut()` to `marks.iter()`
- Removed unnecessary tuple destructuring `(e, _, _)` to just `e`

### Technical Details

**Before:**
```rust
pub fn draw_marks(
    mut commands: Commands,
    mut marks: Query<(Entity, &mut Sprite, &mut Transform), With<Mark>>,
    state: Res<GameState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if !state.is_changed() {
        return;
    }
    for (e, _, _) in marks.iter_mut() {
        commands.entity(e).despawn();
    }
    // ... rest of function
}
```

**After:**
```rust
pub fn draw_marks(
    mut commands: Commands,
    marks: Query<Entity, With<Mark>>,
    state: Res<GameState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if !state.is_changed() {
        return;
    }
    for e in marks.iter() {
        commands.entity(e).despawn();
    }
    // ... rest of function
}
```

## Why This Works
The `Mark` component is added to both X marks (sprites) and O marks (meshes). By querying only for the `Mark` component without requiring specific rendering components, the query now correctly finds all marks regardless of their rendering implementation.

This approach is also more flexible and maintainable:
- Works with different rendering methods (sprites, meshes, etc.)
- Follows the ECS pattern of querying by logical components rather than implementation details
- Matches the pattern already used in `cleanup_game` function

## Testing
- ✅ Code compiles successfully
- 🧪 Manual testing required: 
  1. Start a game with Player O moves
  2. Press reset or start a new game
  3. Verify all circles are properly cleared from the board

## Impact
- **Functionality**: Reset button now properly clears all marks from the board
- **User Experience**: No visual artifacts remain when starting a new game
- **Code Quality**: More maintainable query that focuses on logical components

## Files Modified
- `src/game/systems.rs` (`draw_marks` function)

## Related Issues
- Related to previous fix: Player O circle rendering (BUG_FIX_PLAYER_O_CIRCLE.md)
- This bug was introduced when we changed O marks from sprites to meshes
- User reported: "a new bug when reset button pressed all circled stays in screen and not despawned"
