use bevy::prelude::*;

use ascii_ui::{
    attachments,
    widget_builder::{WidgetBuilder, WidgetSaver},
    widgets::{self, Divider, FlexWidget, SingleChildWidget, TabView, Text},
};

use crate::{
    physics_grids::UiPhysicsGridMarker,
    widgets::{DebugOptions, InfoCounts},
};

use super::{inspector::InspectorTab, state::DebugMenuState};

pub(crate) fn setup_ui(commands: &mut Commands, menu_state: &mut DebugMenuState) {
    let debug_menu_state = &mut *menu_state;

    let settings_tab = Box::new(|commands: &mut Commands| {
        FlexWidget::column(vec![
            InfoCounts::build(),
            Divider::build('-'),
            DebugOptions::build(),
        ])(commands)
    });
    let inspector_tab = Box::new(|commands: &mut Commands| {
        FlexWidget::column(vec![widgets::Text::build("text")]).with((InspectorTab::default(),))(
            commands,
        )
    });

    SingleChildWidget::build(Some(FlexWidget::column(vec![
        Text::build("[F3 Debug Menu]"),
        Divider::build('='),
        TabView::build(vec![
            ("Settings", settings_tab),
            ("Inspector", inspector_tab),
        ]),
    ])))
    .with((
        attachments::Root {
            enabled: true,
            position: IVec2 { x: 0, y: -16 },
            size: UVec2 { x: 32, y: 24 },
        },
        UiPhysicsGridMarker,
        attachments::Border::symmetric(
            Some('|'),
            Some('-'),
            [Some(','), Some('.'), Some('`'), Some('\'')],
        )
        .padded(),
        attachments::RenderBundle::default(),
        DebugMenuMarker,
    ))
    .save_id(&mut debug_menu_state.root_widget)(commands);
}

#[derive(Debug, Component)]
pub(crate) struct DebugMenuMarker;
