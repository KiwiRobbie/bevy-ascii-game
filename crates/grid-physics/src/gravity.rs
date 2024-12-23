use bevy_ecs::{component::Component, system::Resource};

#[derive(Component, Clone)]
pub struct Gravity {
    pub(crate) multiplier: f32,
}
impl Default for Gravity {
    fn default() -> Self {
        Self { multiplier: 1.0 }
    }
}

#[derive(Resource)]
pub(crate) struct GravityResource {
    pub(crate) acceleration: f32,
}
impl Default for GravityResource {
    fn default() -> Self {
        Self {
            acceleration: -100.0,
        }
    }
}
