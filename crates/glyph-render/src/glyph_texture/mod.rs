use std::sync::Arc;

use bevy::{
    app::Plugin,
    ecs::{component::Component, system::Resource},
    prelude::Deref,
    render::renderer::{RenderDevice, RenderQueue},
    utils::HashMap,
};
use bytemuck::cast_slice;
use swash::FontRef;
use wgpu::{Extent3d, TextureDescriptor, TextureUsages};

use crate::{
    atlas::FontAtlasSource,
    glyph_render_plugin::{ExtractedGlyphTextureSource, GlyphTextureSource, GpuGlyphTextureSource},
};

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct ExtractedTextureKey {
    pub data_ptr: usize,
    pub atlas_ptr: usize,
}

impl ExtractedTextureKey {
    pub fn new(data: &Arc<GlyphTextureSource>, atlas: &Arc<FontAtlasSource>) -> Self {
        Self {
            data_ptr: Arc::as_ptr(data) as usize,
            atlas_ptr: Arc::as_ptr(atlas) as usize,
        }
    }
}

#[derive(Default, Resource)]
pub struct ExtractedGlyphTextureCache {
    cached: HashMap<ExtractedTextureKey, Arc<ExtractedGlyphTextureSource>>,
}

impl ExtractedGlyphTextureCache {
    pub fn get(
        &self,
        data: &Arc<GlyphTextureSource>,
        atlas: &Arc<FontAtlasSource>,
    ) -> Option<Arc<ExtractedGlyphTextureSource>> {
        let key = ExtractedTextureKey::new(data, atlas);

        self.cached.get(&key).map(|arc| arc.clone())
    }
    pub fn create(
        &mut self,
        data: &Arc<GlyphTextureSource>,
        atlas: &Arc<FontAtlasSource>,
        font: FontRef,
    ) -> Arc<ExtractedGlyphTextureSource> {
        let key = ExtractedTextureKey::new(data, atlas);
        let value = Arc::new(ExtractedGlyphTextureSource::from_text_data(
            &data.as_ref().data,
            atlas,
            font,
        ));

        self.cached.insert(key, value.clone());
        value
    }
    pub fn get_or_create(
        &mut self,
        data: &Arc<GlyphTextureSource>,
        atlas: &Arc<FontAtlasSource>,
        font: FontRef,
    ) -> Arc<ExtractedGlyphTextureSource> {
        self.get(data, atlas)
            .unwrap_or_else(|| self.create(data, atlas, font))
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct PreparedTextureKey {
    pub data_ptr: usize,
}

impl PreparedTextureKey {
    pub fn new(data: &Arc<ExtractedGlyphTextureSource>) -> Self {
        Self {
            data_ptr: Arc::as_ptr(data) as usize,
        }
    }
}

#[derive(Default, Resource)]
pub struct PreparedGlyphTextureCache {
    cached: HashMap<PreparedTextureKey, Arc<GpuGlyphTextureSource>>,
}
impl PreparedGlyphTextureCache {
    pub fn get(
        &self,
        texture: &Arc<ExtractedGlyphTextureSource>,
    ) -> Option<Arc<GpuGlyphTextureSource>> {
        let key = PreparedTextureKey::new(texture);

        self.cached.get(&key).map(|arc| arc.clone())
    }
    pub fn create(
        &mut self,
        texture: &Arc<ExtractedGlyphTextureSource>,
        render_device: &RenderDevice,
        render_queue: &RenderQueue,
    ) -> Arc<GpuGlyphTextureSource> {
        let key = PreparedTextureKey::new(texture);
        let value = Arc::new(GpuGlyphTextureSource {
            width: texture.width,
            height: texture.height,
            buffer_texture: render_device.create_texture_with_data(
                &render_queue,
                &TextureDescriptor {
                    label: "glyph texture".into(),
                    size: Extent3d {
                        width: texture.width,
                        height: texture.height,
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format: wgpu::TextureFormat::R16Uint,
                    usage: TextureUsages::TEXTURE_BINDING,
                    view_formats: &[],
                },
                cast_slice(&texture.data),
            ),
        });

        self.cached.insert(key, value.clone());
        value
    }
    pub fn get_or_create(
        &mut self,
        texture: &Arc<ExtractedGlyphTextureSource>,
        render_device: &RenderDevice,
        render_queue: &RenderQueue,
    ) -> Arc<GpuGlyphTextureSource> {
        self.get(texture)
            .unwrap_or_else(|| self.create(texture, render_device, render_queue))
    }
}

pub struct RenderGlyphTextureCachePlugin;
impl Plugin for RenderGlyphTextureCachePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<ExtractedGlyphTextureCache>()
            .init_resource::<PreparedGlyphTextureCache>();
    }
}

#[derive(Component, Deref)]
pub struct ExtractedGlyphTexture(pub Arc<ExtractedGlyphTextureSource>);
