use bevy::prelude::*;

use super::{
    attachments::border::border_render,
    clear::clear_sprites,
    widgets::{divider::divider_render, text::text_render, texture::texture_render},
    UiRenderSet,
};
use crate::layout::{render_clip::ClipRegion, UiLayoutSet};
use glyph_render::{glyph_render_plugin::GlyphTexture, glyph_sprite::GlyphSprite};
use spatial_grid::position::Position;

pub(crate) struct RenderPlugin;
impl Plugin for RenderPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            PostUpdate,
            (
                clear_sprites,
                (text_render, divider_render, border_render, texture_render),
                // apply_clipping,
            )
                .chain()
                .in_set(UiRenderSet)
                .before(TransformSystem::TransformPropagate)
                .after(UiLayoutSet),
        );
    }
}

fn apply_clipping(
    mut commands: Commands,
    q_clipped: Query<(Entity, &ClipRegion, &Position, &GlyphSprite)>,
    textures: ResMut<Assets<GlyphTexture>>,
) {
    for (entity, clip, pos, sprite) in q_clipped.iter() {
        let clip = clip.to_world_coord();
        let texture = textures.get(sprite.texture.id()).unwrap();

        let texture_start = **pos + sprite.offset;

        let clip_end = clip.start + clip.size.as_ivec2();

        let clipping_start = (clip.start - texture_start).max(IVec2::ZERO);
        let clipping_end = (clip_end - texture_start).min(texture.size().as_ivec2());

        if (clipping_end - clipping_start).cmple(IVec2::ZERO).any() {
            commands.entity(entity).remove::<GlyphSprite>();
        } else {
            if clipping_start.cmpgt(IVec2::ZERO).any()
                || clipping_end.cmplt(texture.size().as_ivec2()).any()
            {}
        }
    }
}
