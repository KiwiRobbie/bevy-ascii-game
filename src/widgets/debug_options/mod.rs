use bevy::{
    app::{Plugin, Update},
    ecs::{
        component::Component,
        entity::Entity,
        query::With,
        system::{Commands, Query, ResMut, Resource},
    },
    prelude::Has,
};

use crate::debug::{DebugCollisions, DebugPositions, DebugUi};
use ascii_ui::{
    widget_builder::{WidgetBuilder, WidgetSaver},
    widgets::{checkbox::CheckboxEnabledMarker, Checkbox, FlexWidget},
};
use grid_physics::sets::EnablePhysicsSystems;

pub(crate) struct DebugOptionsPlugin;
impl Plugin for DebugOptionsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, update_values);
    }
}

#[derive(Component, Default, Resource)]
pub struct DebugOptions {
    pub(crate) position_checkbox: Option<Entity>,
    pub(crate) colliders_checkbox: Option<Entity>,
    pub(crate) ui_checkbox: Option<Entity>,
    pub(crate) pause_checkbox: Option<Entity>,
}

impl DebugOptions {
    pub fn build<'a>() -> WidgetBuilder<'a> {
        WidgetBuilder::new(|commands: &mut Commands| {
            let mut options = DebugOptions::default();
            FlexWidget::column(vec![
                Checkbox::build_labeled("Debug Position").save_id(&mut options.position_checkbox),
                Checkbox::build_labeled("Debug Colliders").save_id(&mut options.colliders_checkbox),
                Checkbox::build_labeled("Debug ECS UI").save_id(&mut options.ui_checkbox),
                Checkbox::build_labeled("Pause Physics").save_id(&mut options.pause_checkbox),
            ])
            .apply(commands)
            .with(options)
            .build(commands)
        })
    }
}

fn update_values(
    mut collisions: ResMut<DebugCollisions>,
    mut positions: ResMut<DebugPositions>,
    mut pause_physics: ResMut<EnablePhysicsSystems>,
    mut ui: ResMut<DebugUi>,
    q_debug_options: Query<&DebugOptions>,
    q_checkbox: Query<Has<CheckboxEnabledMarker>, With<Checkbox>>,
) {
    for state in q_debug_options.iter() {
        if let Some(entity) = state.colliders_checkbox {
            let state = q_checkbox.get(entity).unwrap();
            **collisions = state;
        }
        if let Some(entity) = state.position_checkbox {
            let state = q_checkbox.get(entity).unwrap();
            **positions = state;
        }
        if let Some(entity) = state.pause_checkbox {
            let state = q_checkbox.get(entity).unwrap();
            **pause_physics = !state;
        }
        if let Some(entity) = state.ui_checkbox {
            let state = q_checkbox.get(entity).unwrap();
            **ui = state;
        }
    }
}
