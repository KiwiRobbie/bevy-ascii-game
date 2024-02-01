use bevy::ecs::component::Component;

#[derive(Component, Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum MainAxisAlignment {
    #[default]
    Start,
    End,
    SpaceBetween,
    SpaceAround,
}
