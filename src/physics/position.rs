use bevy::{
    ecs::{
        bundle::Bundle,
        component::Component,
        system::{Query, Res},
    },
    math::{IVec2, Vec2, Vec3},
    transform::components::{GlobalTransform, Transform},
};

use crate::font::FontSize;

#[derive(Component, Default, Debug, Clone)]
pub struct Position {
    pub position: IVec2,
    pub remainder: Vec2,
}

pub fn position_update_transforms_system(
    mut q_position_transforms: Query<(&mut Transform, &Position)>,
    font_size: Res<FontSize>,
) {
    for (mut transform, position) in q_position_transforms.iter_mut() {
        *transform = transform.with_translation(Vec3 {
            x: (position.position.x * font_size.advance() as i32) as f32,
            y: (position.position.y * font_size.line_spacing() as i32) as f32,
            z: transform.translation.z,
        });
    }
}

#[derive(Bundle, Default, Clone)]
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
