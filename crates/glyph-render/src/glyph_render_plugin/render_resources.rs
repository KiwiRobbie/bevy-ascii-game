use bevy::{
    ecs::component::Component,
    prelude::{Deref, DerefMut},
    render::render_resource::{Texture, UniformBuffer},
};

use super::GlyphUniforms;

#[derive(Component, Deref, DerefMut)]
pub(crate) struct GlyphUniformBuffer(pub(crate) UniformBuffer<GlyphUniforms>);

#[derive(Component, Deref, DerefMut)]
pub(crate) struct GlyphStorageTexture(pub(crate) Texture);

#[derive(Component)]
pub(crate) struct GlyphBufferData {
    pub(crate) buffer: Texture,
    // pub(crate) vertex: Buffer,
}
