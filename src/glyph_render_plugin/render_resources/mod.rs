use bevy::{
    ecs::component::Component,
    prelude::{Deref, DerefMut},
    render::render_resource::{Buffer, Texture, UniformBuffer},
};

use super::GlyphUniforms;

#[derive(Component, Deref, DerefMut)]
pub struct GlyphUniformBuffer(pub UniformBuffer<GlyphUniforms>);

#[derive(Component, Deref, DerefMut)]
pub struct GlyphStorageTexture(pub Texture);

#[derive(Component, Deref, DerefMut)]
pub struct GlyphVertexBuffer(pub Buffer);
