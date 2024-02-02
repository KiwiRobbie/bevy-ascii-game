use bevy::{
    ecs::{bundle::Bundle, component::Component, entity::Entity, query::Without, system::Query},
    math::UVec2,
    prelude::{Deref, DerefMut},
    utils::HashSet,
};

use crate::{
    atlas::FontAtlasUser,
    font::{CustomFont, FontSize},
};
pub mod extract;
pub mod prepare;

#[derive(Component, Clone)]
pub struct GlyphBuffer {
    pub textures: HashSet<Entity>,
    pub size: UVec2,
}

#[derive(Bundle)]
pub struct GlyphBufferBundle {
    pub buffer: GlyphBuffer,
    pub font: CustomFont,
    pub font_size: FontSize,
    pub atlas_user: FontAtlasUser,
}

#[derive(Component, Clone, Deref, DerefMut)]
pub struct TargetGlyphBuffer(pub Entity);

pub fn update_glyph_buffer_entities(
    q_sources: Query<(Entity, &TargetGlyphBuffer), Without<GlyphBuffer>>,
    mut q_buffers: Query<&mut GlyphBuffer, Without<TargetGlyphBuffer>>,
) {
    for mut buffer in q_buffers.iter_mut() {
        buffer.textures.clear();
    }

    for (source_entity, source_target) in q_sources.iter() {
        let target = **source_target;

        q_buffers
            .get_mut(target)
            .unwrap()
            .textures
            .insert(source_entity);
    }
}
