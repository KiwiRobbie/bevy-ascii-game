use bevy::prelude::*;

use crate::theme::ThemePlugin;
// use crate::widgets::{MultiChildWidget, SingleChildWidget};
use crate::{layout::plugin::LayoutPlugin, widgets::plugin::WidgetPlugin};
use crate::{mouse::InteractionPlugin, render::plugin::RenderPlugin};

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((
            LayoutPlugin,
            RenderPlugin,
            InteractionPlugin,
            WidgetPlugin,
            ThemePlugin,
        ));
    }
}

// pub struct DespawnUi {
//     pub entity: Entity,
// }
// fn inner_recurse(entity: Entity, world: &mut World) {
//     if let Some(SingleChildWidget { child: Some(child) }) = world.get::<SingleChildWidget>(entity) {
//         inner_recurse(*child, world);
//     }
//     if let Some(mut children) = world.get_mut::<MultiChildWidget>(entity) {
//         for child in core::mem::take(&mut children.0) {
//             inner_recurse(child, world);
//         }
//     }
//     world.despawn(entity);
// }
// impl Command for DespawnUi {
//     fn apply(self, world: &mut World) {
//         inner_recurse(self.entity, world);
//     }
// }

// /// Trait that holds functions for despawning recursively down the transform hierarchy
// pub trait DespawnUiExt {
//     /// Despawns the provided entity alongside all descendants.
//     fn despawn_ui_recursive(self);
// }

// impl DespawnUiExt for EntityCommands<'_> {
//     /// Despawns the provided entity and its children.
//     /// This will emit warnings for any entity that does not exist.
//     fn despawn_ui_recursive(mut self) {
//         let entity = self.id();
//         self.commands().queue(DespawnUi { entity });
//     }
// }
