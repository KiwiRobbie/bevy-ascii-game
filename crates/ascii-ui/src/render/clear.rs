use bevy::prelude::*;

use glyph_render::glyph_sprite::GlyphSprite;

use super::bundle::RenderWidgetMarker;

pub(crate) fn clear_sprites(
    mut commands: Commands,
    q_ui_sprites: Query<Entity, (With<RenderWidgetMarker>, With<GlyphSprite>)>,
) {
    for entity in q_ui_sprites.iter() {
        commands.entity(entity).remove::<GlyphSprite>();
    }
}
