use ascii_ui::{
    attachments::{self, border::Border},
    widget_builder::{WidgetBuilder, WidgetSaver},
    widgets::{self, Checkbox, Column, Container, Divider, TabView, Text},
};
use bevy::{
    ecs::{
        component::Component,
        system::{Commands, ResMut},
    },
    math::{IVec2, UVec2},
};

use super::{inspector::InspectorTab, state::DebugMenuState};

pub fn setup_ui(mut commands: Commands, mut menu_state: ResMut<DebugMenuState>) {
    let menu_state = &mut *menu_state;

    let settings_tab = Column::build(vec![
        Text::build("".into()).save_id(&mut menu_state.fps_text),
        Text::build("".into()).save_id(&mut menu_state.player_count_text),
        Text::build("".into()).save_id(&mut menu_state.actor_count_text),
        Text::build("".into()).save_id(&mut menu_state.solid_count_text),
        Divider::build('-'),
        Checkbox::build("Debug Position".into()).save_id(&mut menu_state.position_checkbox),
        Checkbox::build("Debug Colliders".into()).save_id(&mut menu_state.colliders_checkbox),
        Checkbox::build("Debug ECS UI".into()).save_id(&mut menu_state.ui_checkbox),
        Checkbox::build("Pause Physics".into()).save_id(&mut menu_state.pause_checkbox),
    ])(&mut commands);

    let inspector_tab = Column::build(vec![]).with(InspectorTab::default())(&mut commands);

    Container::build(Some(Column::build(vec![
        Text::build("[F3 Debug Menu]".into()),
        Divider::build('='),
        TabView::build(vec![
            ("Settings".into(), settings_tab),
            ("Inspector".into(), inspector_tab),
        ]),
    ])))
    .with((
        attachments::Root {
            enabled: true,
            position: IVec2 { x: 0, y: -1 },
            size: UVec2 { x: 32, y: 11 },
        },
        attachments::BorderBundle::new(Border::symmetric(
            Some('|'),
            Some('-'),
            Some([',', '.', '`', '\'']),
        )),
        attachments::RenderBundle::default(),
        DebugMenuMarker,
    ))
    .save_id(&mut menu_state.root_widget)(&mut commands);
}

#[derive(Debug, Component)]
pub struct DebugMenuMarker;
