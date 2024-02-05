use bevy::{
    app::{Plugin, Update},
    ecs::{
        component::Component,
        entity::Entity,
        query::With,
        system::{Commands, Query, Res},
    },
    input::{mouse::MouseButton, Input},
    math::{IVec2, Vec4Swizzles},
    render::{camera::Camera, color::Color},
    transform::components::GlobalTransform,
    window::{PrimaryWindow, Window},
};
use glyph_render::glyph_render_plugin::GlyphSolidColor;
use spatial_grid::grid::{PhysicsGridMember, SpatialGrid};

use crate::layout::positioned::Positioned;

#[derive(Debug, Component)]
pub struct IntractableMarker;

#[derive(Debug, Component)]
pub struct ActiveMarker;

#[derive(Debug, Component)]
pub struct TriggeredMarker;

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, mouse_interaction);
    }
}

pub fn mouse_interaction(
    mut commands: Commands,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_intractable: Query<(Entity, &Positioned, &PhysicsGridMember), With<IntractableMarker>>,
    q_active: Query<Entity, With<ActiveMarker>>,
    q_triggered: Query<Entity, With<TriggeredMarker>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    q_mouse_buttons: Res<Input<MouseButton>>,
    q_physics_grid: Query<(&SpatialGrid, &GlobalTransform)>,
) {
    let (camera, camera_transform) = q_camera.single();

    for entity in q_active.iter() {
        commands
            .entity(entity)
            .remove::<ActiveMarker>()
            .remove::<GlyphSolidColor>();
    }
    for entity in q_triggered.iter() {
        commands.entity(entity).remove::<TriggeredMarker>();
    }
    // Games typically only have one window (the primary window)
    if let Some(position) = q_windows
        .single()
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin)
    {
        for (entity, positioned, grid_member) in q_intractable.iter() {
            let Ok((grid, transform)) = q_physics_grid.get(grid_member.grid) else {
                continue;
            };

            let position = (transform.compute_matrix().inverse() * position.extend(1.0)).xy()
                / grid.size.as_vec2();
            let position = position.as_ivec2() + IVec2::Y;

            dbg!(position);

            let cursor_position = IVec2::new(1, -1) * position;
            if positioned.contains(cursor_position) {
                commands
                    .entity(entity)
                    .insert(ActiveMarker)
                    .insert(GlyphSolidColor {
                        color: Color::ORANGE,
                    });
                if q_mouse_buttons.just_pressed(MouseButton::Left) {
                    commands.entity(entity).insert(TriggeredMarker);
                }
            }
        }
    } else {
    }
}
