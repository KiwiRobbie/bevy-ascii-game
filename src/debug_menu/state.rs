use bevy::ecs::{entity::Entity, system::Resource};

#[derive(Debug, Resource, Default)]
pub struct DebugMenuState {
    pub enabled: bool,
    pub root_widget: Option<Entity>,
    pub position_checkbox: Option<Entity>,
    pub ui_checkbox: Option<Entity>,
    pub colliders_checkbox: Option<Entity>,
    pub player_count_text: Option<Entity>,
    pub actor_count_text: Option<Entity>,
    pub solid_count_text: Option<Entity>,
}
