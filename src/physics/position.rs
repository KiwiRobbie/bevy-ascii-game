use bevy::{
    ecs::{component::Component, system::Query},
    math::{IVec2, Vec2, Vec3},
    transform::components::Transform,
};

#[derive(Component, Default, Debug)]
pub struct Position {
    pub position: IVec2,
    pub remainder: Vec2,
}

pub fn update_transforms(mut q_position_transforms: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in q_position_transforms.iter_mut() {
        *transform = transform.with_translation(Vec3 {
            x: (position.position.x * 19) as f32,
            y: (position.position.y * 32) as f32,
            z: transform.translation.z,
        });
    }
}
