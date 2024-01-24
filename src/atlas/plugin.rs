use std::sync::Arc;

use bevy::{
    app::{Plugin, PostUpdate},
    asset::Assets,
    ecs::{
        query::{Added, Changed, Or},
        system::{Query, Res, ResMut},
    },
};
use swash::{
    scale::{Render, ScaleContext, Source, StrikeWith},
    zeno::Format,
};

use crate::font::{CustomFont, CustomFontSource, FontLoadedMarker, FontSize};

use super::{AtlasBuilder, CharacterSet, FontAtlasCache, FontAtlasUser};

fn update_atlases_system(
    mut atlas_cache: ResMut<FontAtlasCache>,
    fonts: Res<Assets<CustomFontSource>>,
    q_users: Query<
        (&FontSize, &CustomFont, &CharacterSet),
        (
            &FontAtlasUser,
            Or<(
                Changed<FontSize>,
                Changed<CustomFont>,
                Changed<CharacterSet>,
                Added<FontLoadedMarker>,
            )>,
        ),
    >,
) {
    for (font_size, font, character_set) in q_users.iter() {
        let Some(font) = fonts.get(font.id()) else {
            continue;
        };
        let key = (font_size.clone(), font.key());

        let new_chars: Vec<&char> = if let Some(atlas) = atlas_cache.cached.get(&key) {
            if atlas.charset.is_superset(character_set) {
                continue;
            }
            atlas.charset.union(character_set).into_iter().collect()
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

        let mut builder = AtlasBuilder::new(font_ref, render, scaler);
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
            .init_resource::<FontAtlasCache>();
    }
}
