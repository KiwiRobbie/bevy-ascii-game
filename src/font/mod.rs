use anyhow::anyhow;
use bevy::{
    asset::{Asset, AssetEvent, AssetLoader, Handle},
    ecs::{
        component::Component,
        entity::Entity,
        event::EventReader,
        system::{Commands, Query, Resource},
    },
    prelude::{Deref, DerefMut},
    reflect::TypePath,
};
use swash::{CacheKey, FontRef};

#[derive(Resource, Component, PartialEq, Eq, Hash, Clone, Deref, DerefMut)]

pub struct FontSize(pub u32);

impl FontSize {
    pub fn advance(&self) -> u32 {
        ((**self as f32) * 19.0 / 32.0) as u32
    }
    pub fn line_spacing(&self) -> u32 {
        ((**self as f32) * 40.0 / 32.0) as u32
    }
}
impl Default for FontSize {
    fn default() -> Self {
        Self(32)
    }
}

#[derive(Component, DerefMut, Deref, Clone)]
pub struct CustomFont(pub Handle<CustomFontSource>);

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct CustomFontCacheKey(pub CacheKey);

#[derive(Asset, TypePath)]
pub struct CustomFontSource {
    // Full content of the font file
    data: Vec<u8>,
    // Offset to the table directory
    offset: u32,
    // Cache key
    key: CacheKey,
}

impl CustomFontSource {
    pub fn from_bytes(data: &[u8], index: usize) -> Option<Self> {
        // Create a temporary font reference for the first font in the file.
        // This will do some basic validation, compute the necessary offset
        // and generate a fresh cache key for us.
        let font = FontRef::from_index(data, index)?;
        let (offset, key) = (font.offset, font.key);
        // Return our struct with the original file data and copies of the
        // offset and key from the font reference
        Some(Self {
            data: data.to_vec(),
            offset,
            key,
        })
    }
    pub fn from_file(path: &str, index: usize) -> Option<Self> {
        // Read the full font file
        let data = std::fs::read(path).ok()?;
        Self::from_bytes(&data, index)
    }

    // Create the transient font reference for accessing this crate's
    // functionality.
    pub fn as_ref(&self) -> FontRef {
        // Note that you'll want to initialize the struct directly here as
        // using any of the FontRef constructors will generate a new key which,
        // while completely safe, will nullify the performance optimizations of
        // the caching mechanisms used in this crate.
        FontRef {
            data: &self.data,
            offset: self.offset,
            key: self.key,
        }
    }

    pub fn key(&self) -> CustomFontCacheKey {
        CustomFontCacheKey(self.key)
    }
}

#[derive(Default)]
pub struct CustomFontLoader;

impl AssetLoader for CustomFontLoader {
    type Asset = CustomFontSource;
    type Settings = ();
    type Error = anyhow::Error;

    fn extensions(&self) -> &[&str] {
        &["ttf"]
    }
    fn load<'a>(
        &'a self,
        mut reader: &'a mut bevy::asset::io::Reader,
        _settings: &'a Self::Settings,
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            bevy::asset::AsyncReadExt::read_to_end(&mut reader, &mut bytes).await?;
            match CustomFontSource::from_bytes(&bytes, 0) {
                Some(asset) => Ok(asset),
                None => Err(anyhow!(format!(
                    "Failed to create font from file {:?}",
                    load_context.path()
                ))),
            }
        })
    }
}

#[derive(Component)]
pub struct FontLoadedMarker;

pub fn font_load_system(
    mut commands: Commands,
    mut ev_asset: EventReader<AssetEvent<CustomFontSource>>,
    q_font_references: Query<(Entity, &CustomFont)>,
) {
    for ev in ev_asset.read() {
        if let AssetEvent::LoadedWithDependencies { id } = ev {
            for (entity, font) in q_font_references.iter() {
                if &font.id() == id {
                    commands.entity(entity).insert(FontLoadedMarker);
                }
            }
        }
    }
}
