use bevy::{
    ecs::{bundle::Bundle, component::Component, system::Query},
    gizmos::gizmos::Gizmos,
    math::{IVec2, Vec2, Vec3, Vec3Swizzles},
    render::color::Color,
    transform::components::{GlobalTransform, Transform},
};

#[derive(Component, Default, Debug, Clone)]
pub struct Position {
    pub position: IVec2,
    pub remainder: Vec2,
}

pub fn position_update_transforms_system(
    mut q_position_transforms: Query<(&mut Transform, &Position)>,
    mut gizmos: Gizmos,
) {
    for (mut transform, position) in q_position_transforms.iter_mut() {
        *transform = transform.with_translation(Vec3 {
            x: (position.position.x * 19) as f32,
            y: (position.position.y * 40) as f32,
            z: transform.translation.z,
        });
        gizmos.circle_2d(transform.translation.xy(), 5.0, Color::BLUE);
    }
}

#[derive(Bundle, Default)]
pub struct PositionBundle {
    pub position: Position,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl<V: Into<IVec2>> From<V> for PositionBundle {
    fn from(value: V) -> Self {
        Self {
            position: Position {
                position: value.into(),
                remainder: Vec2::ZERO,
            },
            ..Default::default()
        }
    }
}
