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

use crate::player::PlayerMarker;
use ascii_ui::{
    widget_builder::{WidgetBuilder, WidgetSaver},
    widgets::{FlexWidget, Text},
};
use grid_physics::{actor::Actor, solid::Solid};

pub(crate) struct InfoCountsPlugin;
impl Plugin for InfoCountsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, update_info_count);
    }
}

#[derive(Debug, Component)]
pub struct InfoCounts {
    pub(crate) fps_text: Entity,
    pub(crate) entity_text: Entity,
    pub(crate) actor_text: Entity,
    pub(crate) player_text: Entity,
    pub(crate) solid_text: Entity,
}

impl InfoCounts {
    pub fn build<'a>() -> WidgetBuilder<'a> {
        WidgetBuilder::new(|commands: &mut Commands| {
            let info = InfoCounts {
                fps_text: Text::build("").build(commands),
                entity_text: Text::build("").build(commands),
                actor_text: Text::build("").build(commands),
                player_text: Text::build("").build(commands),
                solid_text: Text::build("").build(commands),
            };
            FlexWidget::column(vec![
                info.fps_text.into(),
                info.entity_text.into(),
                info.actor_text.into(),
                info.player_text.into(),
                info.solid_text.into(),
            ])
            .apply(commands)
            .with(info)
            .build(commands)
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
        q_text.get_mut(state.fps_text).unwrap().text =
            format!("FPS: {:0.2}", 1.0 / time.delta_secs());

        apply_count((&mut q_text, state.entity_text), "Entity Count", &q_entity);
        apply_count((&mut q_text, state.actor_text), "Actor  Count", &q_actor);
        apply_count((&mut q_text, state.player_text), "Player Count", &q_player);
        apply_count((&mut q_text, state.solid_text), "Solid  Count", &q_solid);
    }
}

fn apply_count<F: QueryFilter>(
    text: (&mut Query<&mut Text>, Entity),
    label: &str,
    q_count: &Query<(), F>,
) {
    text.0.get_mut(text.1).unwrap().text = format!("{}: {}", label, q_count.iter().count());
}
