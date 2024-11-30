use std::sync::Arc;

use anyhow::Ok;
use bevy::asset::{AssetLoader, AsyncReadExt};

use crate::glyph_render_plugin::{GlyphTexture, GlyphTextureSource};

pub(super) struct GlyphTextureLoader;

impl AssetLoader for GlyphTextureLoader {
    type Asset = GlyphTexture;
    type Settings = ();
    type Error = anyhow::Error;

    fn extensions(&self) -> &[&str] {
        &["art"]
    }

    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        _settings: &'a Self::Settings,
        _load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;

            let data = bytes
                .split(|&b| b == b'\n')
                .map(|line| {
                    String::from_utf8(line.strip_suffix(b"\r").unwrap_or(line).to_vec()).unwrap()
                })
                .collect::<Vec<_>>();

            Ok(data.into())
        })
    }
}
