use ascii_ui::{layout::positioned::Positioned, mouse::ActiveMarker};
use bevy::{
    app::{Plugin, Update},
    color::palettes::css::ORANGE,
    ecs::{
        query::With,
        schedule::IntoSystemConfigs,
        system::{Query, Res, Resource},
    },
    gizmos::gizmos::Gizmos,
    math::Vec2,
    prelude::{Deref, DerefMut},
    transform::components::Transform,
};
use spatial_grid::grid::{PhysicsGridMember, SpatialGrid};

fn debug_active(
    mut gizmos: Gizmos,
    q_positioned: Query<(&Positioned, &PhysicsGridMember), With<ActiveMarker>>,
    q_physics_grid: Query<(&SpatialGrid, &Transform)>,
) {
    for (positioned, grid_member) in q_positioned.iter() {
        let Ok((grid, transform)) = q_physics_grid.get(grid_member.grid) else {
            continue;
        };

        let offset = positioned.offset.as_vec2() * grid.size.as_vec2() * Vec2::new(1.0, -1.0)
            + transform.translation.truncate();
        let size = positioned.size.as_vec2() * grid.size.as_vec2() * Vec2::new(1.0, -1.0);
        let center = offset + 0.5 * size;

        gizmos.rect_2d(center, size, ORANGE);
    }
}

pub(crate) struct UiDebugPlugin;
impl Plugin for UiDebugPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            debug_active.run_if(|enabled: Res<DebugUi>| **enabled),
        )
        .init_resource::<DebugUi>();
    }
}

#[derive(Debug, Resource, DerefMut, Deref, Default)]
pub(crate) struct DebugUi(pub(crate) bool);
