use bevy::ecs::{component::Component, system::Resource};

#[derive(Component)]
pub struct Gravity {
    pub multiplier: f32,
}
impl Default for Gravity {
    fn default() -> Self {
        Self { multiplier: 1.0 }
    }
}

#[derive(Resource)]
pub struct GravityResource {
    pub acceleration: f32,
}
impl Default for GravityResource {
    fn default() -> Self {
        Self {
            acceleration: -50.0,
        }
    }
}
