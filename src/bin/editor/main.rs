use bevy::{prelude::*, window::WindowResolution};
use bevy_ascii_game::{
    debug::DebugPlugin, debug_menu::plugin::DebugMenuPlugin, physics_grids::PhysicsGridPlugin,
    player::PlayerPlugin, tilemap::plugin::TilemapPlugin, tileset::plugin::TilesetPlugin,
    widgets::UiSectionsPlugin,
};
use glyph_render::{
    atlas::FontAtlasPlugin, glyph_animation::GlyphAnimationPlugin,
    glyph_animation_graph::plugin::GlyphAnimationGraphPlugin,
    glyph_render_plugin::GlyphRenderPlugin, glyph_sprite::GlyphTexturePlugin,
};
use grid_physics::plugin::PhysicsPlugin;

fn main() {
    let mut app = bevy::app::App::new();
    app.add_plugins((
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::default().with_scale_factor_override(1.0),
                    present_mode: bevy::window::PresentMode::AutoNoVsync,
                    ..Default::default()
                }),
                ..Default::default()
            }),
        PlayerPlugin,
        GlyphAnimationPlugin,
        GlyphAnimationGraphPlugin,
        FontAtlasPlugin,
        TilesetPlugin,
        TilemapPlugin,
        PhysicsPlugin,
        GlyphTexturePlugin,
        GlyphRenderPlugin,
        DebugPlugin,
        DebugMenuPlugin,
        PhysicsGridPlugin,
        UiSectionsPlugin,
    ));
    app.run();
}
