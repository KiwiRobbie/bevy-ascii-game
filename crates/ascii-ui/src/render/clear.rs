use bevy::ecs::{
    entity::Entity,
    query::With,
    system::{Commands, Query},
};
use glyph_render::glyph_render_plugin::GlyphSprite;

use super::bundle::RenderWidgetMarker;

pub fn clear_sprites(
    mut commands: Commands,
    q_ui_sprites: Query<Entity, (With<RenderWidgetMarker>, With<GlyphSprite>)>,
) {
    for entity in q_ui_sprites.iter() {
        commands.entity(entity).remove::<GlyphSprite>();
    }
}
