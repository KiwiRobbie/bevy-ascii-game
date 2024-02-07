use bevy::{
    app::{Plugin, Update},
    asset::Assets,
    ecs::{
        entity::Entity,
        schedule::{apply_deferred, IntoSystemConfigs},
        system::{Commands, Query, ResMut},
    },
    math::IVec2,
};
use glyph_render::glyph_render_plugin::{GlyphSprite, GlyphTextureSource};
use spatial_grid::position::Position;

use crate::layout::render_clip::ClipRegion;

use super::{
    attachments::border::border_render,
    clear::clear_sprites,
    widgets::{divider::divider_render, text::text_render, texture::texture_render},
};

pub struct RenderPlugin;
impl Plugin for RenderPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (
                clear_sprites,
                apply_deferred,
                (text_render, divider_render, border_render, texture_render),
                apply_deferred,
                apply_clipping,
            )
                .chain(),
        );
    }
}

fn apply_clipping(
    mut commands: Commands,
    q_clipped: Query<(Entity, &ClipRegion, &Position, &GlyphSprite)>,
    mut textures: ResMut<Assets<GlyphTextureSource>>,
) {
    for (entity, clip, pos, sprite) in q_clipped.iter() {
        let clip = clip.to_world_coord();
        let texture = textures.get(sprite.texture.id()).unwrap();

        let texture_start = pos.position + sprite.offset;

        let clip_end = clip.start + clip.size.as_ivec2();

        let clipping_start = (clip.start - texture_start).max(IVec2::ZERO);
        let clipping_end = (clip_end - texture_start).min(texture.size().as_ivec2());

        if (clipping_end - clipping_start).cmple(IVec2::ZERO).any() {
            commands.entity(entity).remove::<GlyphSprite>();
        } else {
            if clipping_start.cmpgt(IVec2::ZERO).any()
                || clipping_end.cmplt(texture.size().as_ivec2()).any()
            {
                let mut data = Vec::new();

                let t = texture.data.len() as usize;
                for src_y in (t - clipping_end.y as usize)..(t - clipping_start.y as usize) {
                    let src_start_x = clipping_start.x as usize;
                    let src_end_x = clipping_end.x as usize;
                    data.push(texture.data[src_y][src_start_x..src_end_x].to_string());
                }
                commands.entity(entity).insert(GlyphSprite {
                    offset: sprite.offset + clipping_start,
                    texture: textures.add(GlyphTextureSource { data }),
                });
            }
        }
    }
}
