use ascii_ui::plugin::UiPlugin;
use bevy::app::{Plugin, Startup, Update};

use super::{
    inspector::InspectorPlugin,
    setup::setup_ui,
    state::DebugMenuState,
    update::{toggle_menu, update_position, update_values},
};

pub struct DebugMenuPlugin;
impl Plugin for DebugMenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((UiPlugin, InspectorPlugin))
            .add_systems(Startup, setup_ui)
            .add_systems(Update, (update_values, update_position, toggle_menu))
            .init_resource::<DebugMenuState>();
    }
}

// fn catch_leak(archetypes: &Archetypes, world: &World, q_text: Query<&widgets::Text>) {
//     println!("\n\n\n");
//     for archetype in archetypes.iter() {
//         if &archetype.entities().len() > &1000 {
//             println!("\n");
//             for component_id in archetype.components() {
//                 // let entity = archetype.entities().first().unwrap();
//                 let components = world.components();
//                 println!("{:?}", components.get_info(component_id).map(|c| c.name()));
//             }
//         }
//     }

//     let mut counts: HashMap<&str, usize> = HashMap::new();
//     for text in q_text.iter().map(|t| t.text.as_str()) {
//         *counts.entry(text).or_insert(0usize) += 1;
//     }
//     let mut counts = counts.iter().collect::<Vec<_>>();
//     counts.sort_by(|a, b| a.1.cmp(b.1));
//     dbg!(counts);
// }
