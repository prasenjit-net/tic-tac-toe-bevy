# Enhancement: Unified Mesh-Based Rendering for X and O

## Overview
Converted Player X marks from sprite-based rendering to mesh-based rendering to match the implementation used for Player O marks. This creates a consistent rendering approach for all game marks.

## Motivation
Previously, the game used two different rendering systems:
- **Player X**: Sprite-based rendering using `Sprite::from_color()`
- **Player O**: Mesh-based rendering using `Mesh2d` with `Circle`

This inconsistency:
- Made the code harder to maintain
- Required special handling in queries (caused the circles not despawning bug)
- Mixed two different rendering paradigms unnecessarily

## Changes Made

### Updated `spawn_x` function (`src/game/systems.rs`, line 736)

**Before (Sprite-based):**
```rust
fn spawn_x(commands: &mut Commands, center: Vec2) {
    let len = CELL_SIZE * 0.6;
    let thickness = LINE_THICKNESS * 1.5;
    let z = 0.5;
    for angle in [45f32.to_radians(), -45f32.to_radians()] {
        commands.spawn((
            Mark,
            Sprite::from_color(X_COLOR, Vec2::new(len, thickness)),
            Transform::from_translation(Vec3::new(center.x, center.y, z))
                .with_rotation(Quat::from_rotation_z(angle)),
        ));
    }
}
```

**After (Mesh-based):**
```rust
fn spawn_x(commands: &mut Commands, center: Vec2, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<ColorMaterial>>) {
    let len = CELL_SIZE * 0.6;
    let thickness = LINE_THICKNESS * 1.5;
    let z = 0.5;
    for angle in [45f32.to_radians(), -45f32.to_radians()] {
        commands.spawn((
            Mark,
            Mesh2d(meshes.add(Rectangle::new(len, thickness))),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(X_COLOR))),
            Transform::from_translation(Vec3::new(center.x, center.y, z))
                .with_rotation(Quat::from_rotation_z(angle)),
        ));
    }
}
```

### Updated `draw_marks` system (`src/game/systems.rs`, line 683)

Updated the call to `spawn_x` to pass the meshes and materials resources:
```rust
Player::X => spawn_x(&mut commands, center, &mut meshes, &mut materials),
```

## Technical Details

### Rendering Components
Both X and O marks now use:
- `Mesh2d`: 2D mesh component for shape rendering
- `MeshMaterial2d`: Material for coloring the mesh
- `ColorMaterial`: Color material with the player's color

### Shapes Used
- **Player X**: Two rotated `Rectangle` meshes (45° and -45°) forming a cross
- **Player O**: Two `Circle` meshes (outer and inner) forming a ring

### Benefits of Mesh-Based Rendering
1. **Consistency**: Single rendering system for all marks
2. **Maintainability**: Uniform code structure and patterns
3. **Flexibility**: Easier to add effects or modify appearance
4. **Performance**: Meshes are efficiently rendered by Bevy's 2D pipeline
5. **Query Simplicity**: All marks have the same component structure

## Impact
- **Visual**: No visible change to the player (X marks look identical)
- **Code Quality**: Cleaner, more consistent codebase
- **Bug Prevention**: Eliminates the category of bugs related to mixed rendering systems
- **Future-Proof**: Easier to enhance or modify mark rendering in the future

## Testing
- ✅ Code compiles successfully
- 🧪 Manual testing required:
  1. Play a game with both X and O moves
  2. Verify X marks render correctly as crossed lines
  3. Verify O marks render correctly as circles
  4. Reset/start new game and verify all marks despawn properly

## Files Modified
- `src/game/systems.rs`:
  - `spawn_x` function: Changed from sprite to mesh rendering
  - `draw_marks` function: Updated spawn_x call to pass meshes/materials

## Related Changes
- Follows the pattern established in BUG_FIX_PLAYER_O_CIRCLE.md
- Works with the fix in BUG_FIX_CIRCLES_NOT_DESPAWNING.md
- User requested: "now it works but can you use same system for creating the x and o like use meshes for both"
