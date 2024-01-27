use bevy::{
    ecs::{bundle::Bundle, component::Component, system::Query},
    math::{IVec2, Vec2, Vec3},
    transform::components::{GlobalTransform, Transform},
};

#[derive(Component, Default, Debug)]
pub struct Position {
    pub position: IVec2,
    pub remainder: Vec2,
}

pub fn position_update_transforms_system(
    mut q_position_transforms: Query<(&mut Transform, &Position)>,
) {
    for (mut transform, position) in q_position_transforms.iter_mut() {
        *transform = transform.with_translation(Vec3 {
            x: (position.position.x * 19) as f32,
            y: (position.position.y * 40) as f32,
            z: transform.translation.z,
        });
    }
}

#[derive(Bundle, Default)]
pub struct PositionBundle {
    pub position: Position,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}
