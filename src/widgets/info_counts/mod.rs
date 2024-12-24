use ascii_ui::{
    widget_builder::{WidgetBuilder, WidgetBuilderFn, WidgetSaver},
    widgets::{FlexWidget, Text},
};
use bevy::{
    app::{Plugin, Update},
    ecs::{
        component::Component,
        entity::Entity,
        query::{QueryFilter, With},
        system::{Commands, Query, Res},
    },
    time::Time,
};
use grid_physics::{actor::Actor, solid::Solid};

use crate::player::PlayerMarker;

pub(crate) struct InfoCountsPlugin;
impl Plugin for InfoCountsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, update_info_count);
    }
}

#[derive(Debug, Component, Default)]
pub struct InfoCounts {
    pub(crate) fps_text: Option<Entity>,
    pub(crate) entity_text: Option<Entity>,
    pub(crate) actor_text: Option<Entity>,
    pub(crate) player_text: Option<Entity>,
    pub(crate) solid_text: Option<Entity>,
}

impl InfoCounts {
    pub fn build<'a>() -> WidgetBuilderFn<'a> {
        Box::new(|commands: &mut Commands| {
            let mut info = InfoCounts::default();
            FlexWidget::column(vec![
                Text::build("").save_id(&mut info.fps_text),
                Text::build("").save_id(&mut info.entity_text),
                Text::build("").save_id(&mut info.actor_text),
                Text::build("").save_id(&mut info.player_text),
                Text::build("").save_id(&mut info.solid_text),
            ])
            .apply(commands)
            .with(info)(commands)
        })
    }
}

fn update_info_count(
    time: Res<Time>,
    q_player: Query<(), With<PlayerMarker>>,
    q_solid: Query<(), With<Solid>>,
    q_actor: Query<(), With<Actor>>,
    q_entity: Query<()>,
    mut q_text: Query<&mut Text>,
    q_info_counts: Query<&InfoCounts>,
) {
    for state in q_info_counts.iter() {
        if let Some(entity) = state.fps_text {
            q_text.get_mut(entity).unwrap().text = format!("FPS: {:0.2}", 1.0 / time.delta_secs());
        }
        apply_count((&mut q_text, state.entity_text), "Entity Count", &q_entity);
        apply_count((&mut q_text, state.actor_text), "Actor  Count", &q_actor);
        apply_count((&mut q_text, state.player_text), "Player Count", &q_player);
        apply_count((&mut q_text, state.solid_text), "Solid  Count", &q_solid);
    }
}

fn apply_count<F: QueryFilter>(
    text: (&mut Query<&mut Text>, Option<Entity>),
    label: &str,
    q_count: &Query<(), F>,
) {
    if let Some(entity) = text.1 {
        text.0.get_mut(entity).unwrap().text = format!("{}: {}", label, q_count.iter().count());
    }
}
