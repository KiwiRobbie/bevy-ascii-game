use bevy::ecs::{entity::Entity, system::Resource};

#[derive(Debug, Resource)]
pub struct DebugMenuState {
    pub enabled: bool,
    pub root_widget: Option<Entity>,

    pub fps_text: Option<Entity>,
    pub player_count_text: Option<Entity>,
    pub actor_count_text: Option<Entity>,
    pub solid_count_text: Option<Entity>,
    pub entity_count: Option<Entity>,
    pub position_checkbox: Option<Entity>,
    pub colliders_checkbox: Option<Entity>,
    pub ui_checkbox: Option<Entity>,
    pub pause_checkbox: Option<Entity>,
}

impl Default for DebugMenuState {
    fn default() -> Self {
        DebugMenuState {
            enabled: true,
            root_widget: None,

            fps_text: None,
            player_count_text: None,
            actor_count_text: None,
            solid_count_text: None,
            entity_count: None,

            colliders_checkbox: None,
            position_checkbox: None,
            ui_checkbox: None,
            pause_checkbox: None,
        }
    }
}
