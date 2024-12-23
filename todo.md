# Combat 
Intent based 
- Defend `,` / `<` key
- Attack  `.` / `>` key 

Timer/cooldown 
Randomness affected by sword stats?


## Feint 
Double tap attack 

## Wrench 
Tap then hold 

## Close Defend and move closer 

## Riposte 
Back + attack 





# Combat 
- Use movement keys combined with attack/parry to determine action.
- VSCode like "chords" 
- Close: Parry, Forward Attack
- Remise, Back Attack 
- Modifier key for upper/lower strikes


A: Attack 
P: Parry 
I: In 
O: Out 


PIA: Close 
OA:  Remise 

Honour System? 


# Horses 
Horse fun, make ride horse 

Player control switches between different entities (IE avatar vs horse), 
component to track player control.

# Simple tasks

- Add z-sorting / ordering
- Add GZIP to art/tilemaps

# Improve text rendering

- Less CPU side calculations
- Use atlases and avoid re-uploading data.
- Pack atlas UV data into a texture. (WebGL2 support)

## Operation

- Create atlas texture
- Insert visible glyph character textures into atlas.
- Build mesh from visible sprites, reference atlas, apply instancing
- Render mesh into glyph buffer texture.
- Rendering glyph buffers onto screen as single quad.

### Minimal Example

- Add glyph buffer UV data as packed texture
- Render sprites to glyph buffer texture
- Draw glyph buffer texture using quad

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
