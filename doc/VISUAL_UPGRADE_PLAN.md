# Visual Upgrade Plan - Tic-Tac-Toe

## Current State
- Basic colors and styling
- Simple sprite/mesh rendering
- No logo or branding
- Functional but basic UI

## Planned Improvements

### 1. Logo Design
Create an SVG-style logo using Bevy shapes:
- Stylized X and O interlocked
- Gradient colors
- Modern, clean design
- Animated entrance

### 2. Color Scheme Upgrade
**Current:**
- Background: Dark gray (0.09, 0.10, 0.12)
- Grid: Light gray
- X: Red
- O: Blue
- Win: Green

**New - Modern Dark Theme:**
- Background: Deep dark blue with subtle gradient
- Grid: Glowing cyan with subtle transparency
- X: Vibrant red-orange with glow effect
- O: Electric cyan-blue with glow effect
- Win: Bright gold/yellow with pulse effect
- UI: Glass morphism style with blur and transparency

### 3. Visual Effects
- Glow effects on marks
- Smooth fade-in animations for marks
- Pulse animation for win line
- Particle effects on mark placement
- Smooth transitions between states
- Hover effects on UI elements

### 4. Enhanced UI
- Glass morphism style for menu
- Smooth gradients
- Better typography
- Icon-based buttons
- Animated logo on menu screen

### 5. Grid Enhancement
- Subtle glow on grid lines
- Rounded corners on grid
- Better cell highlighting on hover
- Animated grid appearance

## Implementation Steps

### Phase 1: Color Scheme & Constants
- Update color constants
- Add new colors (glow, accent, etc.)
- Test new palette

### Phase 2: Logo Creation
- Design logo using Bevy shapes
- Add to menu screen
- Implement animations

### Phase 3: Visual Effects
- Add glow to marks
- Implement fade-in animations
- Add win line pulse

### Phase 4: UI Polish
- Glass morphism effects
- Better spacing and layout
- Improved typography

### Phase 5: Interactions
- Hover effects
- Click feedback
- Smooth transitions

## Technical Approach

### Logo
- Use Bevy's 2D shapes (circles, rectangles)
- Combine multiple entities for complex logo
- Use Transform for positioning
- Add Timer-based rotation/scale animations

### Glow Effects
- Layer multiple semi-transparent sprites/meshes
- Slightly larger outer layer with low opacity
- Use additive blending if possible

### Animations
- Use Bevy's Transform interpolation
- Timer-based animations
- Easing functions for smooth motion

### Glass Morphism
- Semi-transparent backgrounds
- Blur simulation with gradients
- Border highlights
- Backdrop effects

## Color Palette

### Main Colors
```rust
// Background
BG_COLOR: Deep space blue (#0A0E1A)
BG_GRADIENT_START: Dark blue (#0F1419)
BG_GRADIENT_END: Deep blue (#0A0E1A)

// Grid
GRID_COLOR: Cyan glow (#4ECDC4)
GRID_GLOW: Cyan with alpha (#4ECDC4, 0.3)

// Marks
X_COLOR: Hot red-orange (#FF6B6B)
X_GLOW: Red glow (#FF6B6B, 0.5)

O_COLOR: Electric cyan (#4ECDC4)
O_GLOW: Cyan glow (#4ECDC4, 0.5)

// Win
WIN_COLOR: Golden yellow (#FFD93D)
WIN_GLOW: Gold pulse (#FFD93D, 0.7)

// UI
UI_BG: Glass (#1A1F2E, 0.7)
UI_BORDER: Cyan accent (#4ECDC4, 0.5)
UI_TEXT: Pure white (#FFFFFF)
UI_ACCENT: Bright cyan (#6FFFE9)
```

### Logo Colors
- Primary: Electric cyan (#4ECDC4)
- Secondary: Hot red-orange (#FF6B6B)
- Accent: Golden yellow (#FFD93D)

## Next Steps
1. Implement new color scheme
2. Create logo system
3. Add glow effects
4. Enhance UI
5. Add animations
