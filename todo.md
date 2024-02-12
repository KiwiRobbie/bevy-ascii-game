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

# Tilemaps + Tilesets

- RON + GZIP
- Append only TileIDs: Adding/removing tiles only breaks removed tiles.
- Chunk data file, `[(tilemap: u32,tile: u32)] as [u8]` + GZIP.
- Tilemap

(
name: String,
chunk_size
chunks: {
(x,y) : (

    ),

}
)

# # Glyph Colours

- [ ] Option of solid colour or texture per character or shader
- [ ] Learn how to enable/disable sections of shaders with features

# Separating Position from Physics

- [x] Make remainder a separate component
- [x] Spatial crate: Integer positions, grids, remainders, ...
- [x] Physics crate: Depends on spatial crate, adds velocity, gravity, actors + solids, ...
- [ ] Move Solid Cache to be per World

# Rendering: Done

- Use instanced drawing instead of compute shader

# Glyph Rendering: Done

## Sprite/Animation: Done

- Text data
- Creates GlyphTexture each frame.
- Target GlyphBuffer Entity.

## GlyphBuffer: Done

- Settings: Font, FontSize, Atlas, Clear Color ...

- GlyphTextures: List of entities to be rendered into buffer
- Creates GpuGlyph buffer and renders to screen
