# Layout system

- Separate layout and UI
- Use layout system to place PhysicsGrid and UI
- Separate passes?
- Split Positioned into spatial_grid::Position and spatial_grid::Size
  Break hierarchy using different parent child link.
- SpatialChild: Apply position through hierarchy
- Standard components make queries easier, remove get_children() from WidgetLogic.

## Usage

- System to update root to window dimensions
- Layout system places containers with game/ui/...
- Set physics grid sizes / ui root sizes in system
- Game/UI layout + render independently

# Rendering

- Use instanced drawing instead of compute shader

# # Glyph Colours

- Option of solid colour or texture per character or shader
- Learn how to enable/disable sections of shaders with features

# Separating Position from Physics

- Make remainder a separate component
- Spatial crate: Integer positions, grids, remainders, ...
- Physics crate: Depends on spatial crate, adds velocity, gravity, actors + solids, ...
- Move Solid Cache to be per World

# Glyph Rendering: Done

## Sprite/Animation: Done

- Text data
- Creates GlyphTexture each frame.
- Target GlyphBuffer Entity.

## GlyphBuffer: Done

- Settings: Font, FontSize, Atlas, Clear Color ...

- GlyphTextures: List of entities to be rendered into buffer
- Creates GpuGlyph buffer and renders to screen