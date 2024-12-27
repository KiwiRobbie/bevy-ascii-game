use bevy::{
    prelude::*,
    render::render_resource::{Texture, UniformBuffer},
};

use super::GlyphRasterUniforms;

#[derive(Component, Deref, DerefMut)]
pub(crate) struct GlyphUniformBuffer(pub(crate) UniformBuffer<GlyphRasterUniforms>);

#[derive(Component)]
pub(crate) struct GlyphBufferData {
    pub(crate) buffer: Texture,
    // pub(crate) vertex: Buffer,
}
