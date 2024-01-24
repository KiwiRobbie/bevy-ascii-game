use anyhow::anyhow;
use bevy::{
    asset::{Asset, AssetLoader},
    reflect::TypePath,
};
use swash::{CacheKey, FontRef};

#[derive(Asset, TypePath)]
pub struct CustomFont {
    // Full content of the font file
    data: Vec<u8>,
    // Offset to the table directory
    offset: u32,
    // Cache key
    key: CacheKey,
}

impl CustomFont {
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
}

#[derive(Default)]
pub struct CustomFontLoader;

impl AssetLoader for CustomFontLoader {
    type Asset = CustomFont;
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
            match CustomFont::from_bytes(&bytes, 0) {
                Some(asset) => Ok(asset),
                None => Err(anyhow!(format!(
                    "Failed to create font from file {:?}",
                    load_context.path()
                ))),
            }
        })
    }
}
