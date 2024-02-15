use std::{sync::Arc, sync::Weak};

use bevy::{
    app::Plugin,
    ecs::{
        component::Component,
        schedule::IntoSystemConfigs,
        system::{ResMut, Resource},
    },
    prelude::Deref,
    render::{
        renderer::{RenderDevice, RenderQueue},
        Render, RenderSet,
    },
    utils::hashbrown::HashMap,
};
use bytemuck::cast_slice;
use swash::FontRef;
use wgpu::{Extent3d, TextureDescriptor, TextureUsages};

use crate::{
    atlas::FontAtlasSource,
    glyph_render_plugin::{
        ExtractedGlyphTextureSource, GlyphTextureSource, PreparedGlyphTextureSource,
    },
};

pub struct CacheItem<T> {
    item: T,
    lifetime: u32,
}

pub struct ExtractedTextureKey {
    pub data: Weak<GlyphTextureSource>,
    pub atlas: Weak<FontAtlasSource>,
}
impl PartialEq for ExtractedTextureKey {
    fn eq(&self, other: &Self) -> bool {
        Weak::ptr_eq(&self.data, &other.data) && Weak::ptr_eq(&self.atlas, &other.atlas)
    }
}
impl Eq for ExtractedTextureKey {}
impl std::hash::Hash for ExtractedTextureKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_usize(Weak::as_ptr(&self.data) as usize);
        state.write_usize(Weak::as_ptr(&self.atlas) as usize);
    }
}
impl ExtractedTextureKey {
    pub fn new(data: &Arc<GlyphTextureSource>, atlas: &Arc<FontAtlasSource>) -> Self {
        Self {
            data: Arc::downgrade(data),
            atlas: Arc::downgrade(atlas),
        }
    }
}

trait GlyphCacheTrait<K, V> {
    fn get(&mut self, key: &K) -> Option<V>;
    fn insert(&mut self, key: K, item: V);
    fn get_or_create<F>(&mut self, key: K, builder: F) -> V
    where
        F: FnOnce() -> V;
    fn update(&mut self);
}

#[derive(Resource)]
pub struct GlyphCache<K, V>
where
    K: PartialEq + Eq + std::hash::Hash,
    V: Clone,
{
    cached: HashMap<K, CacheItem<V>>,
}
impl<K, V> GlyphCacheTrait<K, V> for GlyphCache<K, V>
where
    K: PartialEq + Eq + std::hash::Hash,
    V: Clone,
{
    fn get(&mut self, key: &K) -> Option<V> {
        if let Some(cache_item) = self.cached.get_mut(key) {
            cache_item.lifetime = 10;
            Some(cache_item.item.clone())
        } else {
            None
        }
    }

    fn insert(&mut self, key: K, item: V) {
        self.cached.insert(key, CacheItem { item, lifetime: 10 });
    }

    fn get_or_create<F: FnOnce() -> V>(&mut self, key: K, builder: F) -> V {
        if let Some(value) = self.get(&key) {
            value
        } else {
            let item = builder();
            self.insert(key, item.clone());
            item
        }
    }
    fn update(&mut self) {
        self.cached.retain(|_, v| {
            v.lifetime = v.lifetime.saturating_sub(1);
            v.lifetime > 0
        });
    }
}
impl<K, V> Default for GlyphCache<K, V>
where
    K: PartialEq + Eq + std::hash::Hash,
    V: Clone,
{
    fn default() -> Self {
        Self {
            cached: HashMap::default(),
        }
    }
}

pub type ExtractedGlyphTextureCache =
    GlyphCache<ExtractedTextureKey, Arc<ExtractedGlyphTextureSource>>;
impl ExtractedGlyphTextureCache {
    pub fn get_or_create(
        &mut self,
        data: &Arc<GlyphTextureSource>,
        atlas: &Arc<FontAtlasSource>,
        font: FontRef,
    ) -> Arc<ExtractedGlyphTextureSource> {
        let key = ExtractedTextureKey::new(data, atlas);
        GlyphCacheTrait::<ExtractedTextureKey, Arc<ExtractedGlyphTextureSource>>::get_or_create(
            self,
            key,
            || {
                Arc::new(ExtractedGlyphTextureSource::from_text_data(
                    &data.data, atlas, font,
                ))
            },
        )
    }
}

pub struct PreparedTextureKey {
    pub data: Weak<ExtractedGlyphTextureSource>,
}

impl PartialEq for PreparedTextureKey {
    fn eq(&self, other: &Self) -> bool {
        Weak::ptr_eq(&self.data, &other.data)
    }
}
impl Eq for PreparedTextureKey {}
impl std::hash::Hash for PreparedTextureKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_usize(Weak::as_ptr(&self.data) as usize);
    }
}
impl PreparedTextureKey {
    pub fn new(data: &Arc<ExtractedGlyphTextureSource>) -> Self {
        Self {
            data: Arc::downgrade(data),
        }
    }
}

pub type PreparedGlyphTextureCache =
    GlyphCache<PreparedTextureKey, Arc<PreparedGlyphTextureSource>>;
impl PreparedGlyphTextureCache {
    pub fn get_or_create(
        &mut self,
        texture: &Arc<ExtractedGlyphTextureSource>,
        render_device: &RenderDevice,
        render_queue: &RenderQueue,
    ) -> Arc<PreparedGlyphTextureSource> {
        let key = PreparedTextureKey::new(texture);
        GlyphCacheTrait::<PreparedTextureKey, Arc<PreparedGlyphTextureSource>>::get_or_create(
            self,
            key,
            &|| {
                Arc::new(PreparedGlyphTextureSource {
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
                })
            },
        )
    }
}

pub struct RenderGlyphTextureCachePlugin;
impl Plugin for RenderGlyphTextureCachePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<ExtractedGlyphTextureCache>()
            .init_resource::<PreparedGlyphTextureCache>()
            .add_systems(
                Render,
                (
                    maintain_cache::<ExtractedGlyphTextureCache, _, _>,
                    maintain_cache::<PreparedGlyphTextureCache, _, _>,
                )
                    .in_set(RenderSet::Cleanup),
            );
    }
}

fn maintain_cache<C, K, V>(mut cache: ResMut<C>)
where
    C: Resource + GlyphCacheTrait<K, V>,
{
    cache.update()
}

#[derive(Component, Deref)]
pub struct ExtractedGlyphTexture(pub Arc<ExtractedGlyphTextureSource>);
