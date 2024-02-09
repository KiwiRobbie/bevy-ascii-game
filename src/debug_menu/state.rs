use bevy::ecs::{entity::Entity, system::Resource};

#[derive(Debug, Resource)]
pub struct DebugMenuState {
    pub enabled: bool,
    pub root_widget: Option<Entity>,

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

            colliders_checkbox: None,
            position_checkbox: None,
            ui_checkbox: None,
            pause_checkbox: None,
        }
    }
}
