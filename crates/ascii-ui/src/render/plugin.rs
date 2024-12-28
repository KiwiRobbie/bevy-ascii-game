use bevy::prelude::*;

use super::{
    attachments::border::border_render,
    clear::clear_sprites,
    widgets::{divider::divider_render, text::text_render, texture::texture_render},
    UiRenderSet,
};
use crate::layout::{render_clip::ClipRegion, UiLayoutSet};
use glyph_render::{glyph_render_plugin::GlyphTexture, glyph_sprite::GlyphSprite};
pub(crate) struct RenderPlugin;
impl Plugin for RenderPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            PostUpdate,
            (
                clear_sprites,
                (text_render, divider_render, border_render, texture_render),
                apply_clipping,
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
    q_clipped: Query<(Entity, &ClipRegion, &GlyphSprite)>,
    textures: ResMut<Assets<GlyphTexture>>,
) {
    for (entity, clip, sprite) in q_clipped.iter() {
        let clip = clip.to_world_coord();
        let texture = textures.get(sprite.texture.id()).unwrap();

        let texture_start = sprite.offset;
        let texture_end = texture_start + texture.size().as_ivec2();

        let clip_rect_start = clip.start + IVec2::Y;
        let clip_rect_end = clip.start + clip.size.as_ivec2();

        if clip_rect_end.cmplt(texture_start).any() || texture_end.cmple(clip_rect_start).any() {
            commands.entity(entity).remove::<GlyphSprite>();
        }
    }
}
