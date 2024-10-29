use bevy::{
    ecs::component::Component,
    prelude::{Deref, DerefMut},
    render::render_resource::{Texture, UniformBuffer},
};

use super::GlyphUniforms;

#[derive(Component, Deref, DerefMut)]
pub struct GlyphUniformBuffer(pub UniformBuffer<GlyphUniforms>);

#[derive(Component, Deref, DerefMut)]
pub struct GlyphStorageTexture(pub Texture);

#[derive(Component)]
pub struct GlyphBufferData {
    pub buffer: Texture,
    // pub vertex: Buffer,
}
