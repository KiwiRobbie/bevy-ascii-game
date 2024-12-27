use bevy::prelude::*;

use ascii_ui::{layout::positioned::WidgetSize, mouse::ActiveMarker};
use spatial_grid::{
    global_position::GlobalPosition,
    grid::{PhysicsGridMember, SpatialGrid},
};

fn debug_active(
    mut gizmos: Gizmos,
    q_positioned: Query<(&GlobalPosition, &WidgetSize, &PhysicsGridMember), With<ActiveMarker>>,
    q_physics_grid: Query<(&GlobalPosition, &SpatialGrid, &Transform)>,
) {
    for (position, size, grid_member) in q_positioned.iter() {
        let Ok((grid_position, grid, transform)) = q_physics_grid.get(grid_member.grid) else {
            continue;
        };

        let offset = (**position - **grid_position).as_vec2() * grid.step.as_vec2()
            + transform.translation.truncate();
        let size = size.as_vec2() * grid.step.as_vec2();
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
