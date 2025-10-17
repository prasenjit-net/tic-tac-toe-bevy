# Bug Fix: Player O Circle Rendering

## Issue
Player O was rendering as a square instead of a circle on the game board.

## Root Cause
The `spawn_o` function was using `Sprite::from_color()` to create square sprites with dimensions `Vec2::new(outer, outer)`. While the function attempted to create a ring effect by overlaying two squares of different sizes, this approach resulted in a square appearance rather than a circular one.

## Solution
Replaced the square sprite approach with proper circle meshes using Bevy's 2D rendering system:

### Changes Made

1. **Updated `spawn_o` function** (`src/game/systems.rs`, line 748):
   - Changed from using `Sprite::from_color()` with square dimensions
   - Now uses `Mesh2d` with `Circle::new(radius)` for proper circular shapes
   - Added `meshes` and `materials` parameters to create mesh assets
   - Uses `ColorMaterial` for rendering the circles

2. **Updated `draw_marks` system** (`src/game/systems.rs`, line 683):
   - Added `meshes: ResMut<Assets<Mesh>>` parameter
   - Added `materials: ResMut<Assets<ColorMaterial>>` parameter
   - Updated `spawn_o` call to pass these resources

### Technical Details

**Before:**
```rust
fn spawn_o(commands: &mut Commands, center: Vec2) {
    let outer = CELL_SIZE * 0.6;
    let thickness = LINE_THICKNESS * 1.5;
    let z = 0.5;
    commands.spawn((
        Mark,
        Sprite::from_color(O_COLOR, Vec2::new(outer, outer)),
        Transform::from_translation(Vec3::new(center.x, center.y, z)),
    ));
    // Inner square to create "ring" effect
}
```

**After:**
```rust
fn spawn_o(commands: &mut Commands, center: Vec2, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<ColorMaterial>>) {
    let radius = CELL_SIZE * 0.3;
    let thickness = LINE_THICKNESS * 1.5;
    let z = 0.5;
    
    // Outer circle
    commands.spawn((
        Mark,
        Mesh2d(meshes.add(Circle::new(radius))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(O_COLOR))),
        Transform::from_translation(Vec3::new(center.x, center.y, z)),
    ));
    
    // Inner circle (background color to create ring effect)
    commands.spawn((
        Mark,
        Mesh2d(meshes.add(Circle::new(radius - thickness))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(BG_COLOR))),
        Transform::from_translation(Vec3::new(center.x, center.y, z + 0.01)),
    ));
}
```

## Testing
- ✅ Code compiles successfully
- ✅ Build completes without errors
- 🧪 Manual testing required: Launch game and verify Player O displays as a proper circle

## Impact
- **Visual Quality**: Player O now displays as a proper circle, improving visual clarity
- **User Experience**: Better distinction between Player X (diagonal cross) and Player O (circle)
- **Performance**: Minimal impact; circle meshes are efficiently rendered by Bevy

## Files Modified
- `src/game/systems.rs` (2 functions updated)

## Related Issues
This fix addresses the user-reported issue: "i see instead of a sircle a square is displayed to represent player o"
