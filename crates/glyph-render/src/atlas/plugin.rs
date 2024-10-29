use std::sync::Arc;

use bevy::{
    app::{Plugin, PostUpdate},
    asset::{AssetApp, Assets},
    ecs::{
        query::{Added, Changed, Or, With},
        system::{Query, Res, ResMut},
    },
};
use swash::{
    scale::{Render, ScaleContext, Source, StrikeWith},
    zeno::Format,
};

use crate::font::{
    CustomFont, CustomFontLoader, CustomFontSource, DefaultFont, FontLoadedMarker, FontSize,
};

use super::{AtlasBuilder, CharacterSet, FontAtlasCache, FontAtlasUser};

type FontUpdatedFilter = Or<(
    Changed<FontSize>,
    Changed<CustomFont>,
    Changed<CharacterSet>,
    Added<FontLoadedMarker>,
)>;

fn update_atlases_system(
    mut atlas_cache: ResMut<FontAtlasCache>,
    fonts: Res<Assets<CustomFontSource>>,
    q_users: Query<
        (&FontSize, Option<&CustomFont>, &CharacterSet),
        (With<FontAtlasUser>, FontUpdatedFilter),
    >,
    default_font: Res<DefaultFont>,
) {
    for (font_size, font, character_set) in q_users.iter() {
        let Some(font) = fonts.get(font.map(|f| f.id()).unwrap_or(default_font.id())) else {
            continue;
        };
        let key = (font_size.clone(), font.key());

        let new_chars: Vec<&char> = if let Some(atlas) = atlas_cache.cached.get(&key) {
            if atlas.charset.is_superset(character_set) {
                continue;
            }
            atlas.charset.union(character_set).collect()
        } else {
            character_set.iter().collect()
        };

        let font_ref = font.as_ref();

        let mut context = ScaleContext::new();
        let scaler = context
            .builder(font_ref)
            .hint(false)
            .size(font_size.0 as f32)
            .build();
        let mut render = Render::new(&[
            Source::ColorOutline(0),
            Source::ColorBitmap(StrikeWith::BestFit),
            Source::Outline,
        ]);
        // render.format(Format::Subpixel);
        render.format(Format::CustomSubpixel([0.0, 0.0, 0.0]));

        bevy::log::info!("Building Atlas at Font Size {}", **font_size);
        let mut builder = AtlasBuilder::new(font_ref, render, scaler, **font_size as f32);
        for character in new_chars {
            builder.insert_char(*character);
        }
        atlas_cache.cached.insert(key, Arc::new(builder.build()));
    }
}

pub struct FontAtlasPlugin;
impl Plugin for FontAtlasPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(PostUpdate, update_atlases_system)
            .init_resource::<FontAtlasCache>()
            .init_asset::<CustomFontSource>()
            .init_asset_loader::<CustomFontLoader>()
            .init_resource::<DefaultFont>();
    }
}
