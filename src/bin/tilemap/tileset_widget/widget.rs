// use bevy::{asset::Handle, ecs::component::Component};

// use ascii_ui::{
//     mouse::IntractableMarker,
//     widget_builder::{WidgetBuilder, WidgetBuilderFn},
//     widgets,
// };
// use bevy_ascii_game::tileset::asset::TilesetSource;

// #[derive(Debug, Component)]
// pub struct TilesetWidget {
//     pub tileset: Handle<TilesetSource>,
// }

// impl TilesetWidget {
//     pub fn build<'a>(tileset: &TilesetSource) -> WidgetBuilderFn<'a> {
//         let name = tileset.display_name.clone().into();
//         Box::new(move |commands| widgets::Text::build(name).with(IntractableMarker)(commands))
//     }
// }
