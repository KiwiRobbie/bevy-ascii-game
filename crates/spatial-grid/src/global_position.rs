use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{
    bundle::Bundle,
    component::Component,
    entity::Entity,
    query::{With, Without},
    system::Query,
    world::Mut,
};
use bevy_hierarchy::{Children, Parent};
use bevy_math::{IVec2, Vec2};
use bevy_reflect::Reflect;

use crate::position::{Position, SpatialTraits};

#[derive(Component, Default, Debug, Clone, Copy, Reflect, Deref, DerefMut)]
pub struct GlobalPosition(pub IVec2);

impl GlobalPosition {
    pub fn offset(self, offset: IVec2) -> Self {
        Self(*self + offset)
    }
}

#[derive(Component, Default, Debug, Clone, Reflect, Deref, DerefMut)]
pub struct GlobalRemainder(pub Vec2);
impl From<Vec2> for GlobalRemainder {
    fn from(value: Vec2) -> Self {
        Self(value.fract())
    }
}

#[derive(Bundle, Default, Clone)]
pub struct GlobalBundle {
    pub position: GlobalPosition,
    pub remainder: GlobalRemainder,
}

impl SpatialTraits for (&mut GlobalPosition, &mut GlobalRemainder) {
    fn offset(&mut self, delta: Vec2) {
        **self.1 += delta;
        let delta = self.1.round();
        **self.1 -= delta;
        **self.0 += delta.as_ivec2();
    }
    fn set(&mut self, value: Vec2) {
        **self.0 = value.floor().as_ivec2();
        **self.1 = value.fract();
    }
}
impl<'a> SpatialTraits for (Mut<'a, GlobalPosition>, Mut<'a, GlobalRemainder>) {
    fn offset(&mut self, delta: Vec2) {
        **self.1 += delta;
        let delta = self.1.round();
        **self.1 -= delta;
        **self.0 += delta.as_ivec2();
    }
    fn set(&mut self, value: Vec2) {
        **self.0 = value.floor().as_ivec2();
        **self.1 = value.fract();
    }
}
impl From<IVec2> for GlobalBundle {
    fn from(value: IVec2) -> Self {
        Self {
            position: GlobalPosition(value.into()),
            ..Default::default()
        }
    }
}
impl<V: Into<IVec2>> From<V> for GlobalPosition {
    fn from(value: V) -> Self {
        Self(value.into())
    }
}

pub(crate) fn propagate_positions(
    q_roots: Query<(Entity, &Position, Option<&Children>), Without<Parent>>,
    q_children: Query<(Entity, &Position, Option<&Children>), With<Parent>>,
    mut q_global_pos: Query<&mut GlobalPosition>,
) {
    fn propagate_recursive(
        entity: Entity,
        position: &Position,
        parent_position: &GlobalPosition,
        children: Option<&Children>,
        q_children: &Query<(Entity, &Position, Option<&Children>), With<Parent>>,
        q_global_pos: &mut Query<&mut GlobalPosition>,
    ) {
        let mut global_pos = q_global_pos.get_mut(entity).unwrap();
        **global_pos = **parent_position + **position;

        let Some(children) = children else {
            return;
        };
        let parent_position = global_pos.clone();
        for entity in children {
            let Ok((entity, position, children)) = q_children.get(*entity) else {
                continue;
            };

            propagate_recursive(
                entity,
                position,
                &parent_position,
                children,
                q_children,
                q_global_pos,
            );
        }
    }

    for (entity, position, children) in &q_roots {
        propagate_recursive(
            entity,
            position,
            &GlobalPosition::default(),
            children,
            &q_children,
            &mut q_global_pos,
        );
    }
}
