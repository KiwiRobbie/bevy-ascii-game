use ascii_ui::{
    attachments::{self, Flex},
    col,
    mouse::InteractableMarker,
    row, sized_box, text,
    widgets::{self, Divider, FlexWidget, SingleChildWidget},
};
use bevy::prelude::*;

use bevy_ascii_game::{
    physics_grids::UiPhysicsGridMarker,
    widgets::{DebugOptions, InfoCounts},
};

use crate::layers::widget::LayersWidget;

use super::update::ToolUiContainer;

fn editor_ui_builder(commands: &mut Commands) -> Entity {
    col![
        row![
            widgets::Divider::build('=').with(Flex::new(1)),
            text!(" Tools "),
            widgets::Divider::build('=').with(Flex::new(1)),
        ],
        sized_box!(height: 1),
        text!(" Move   (G)"),
        text!(" Select (R)"),
        text!(" Text   (T)"),
        text!(" Draw   (D)"),
        text!(" Shape  (S)"),
        sized_box!(height: 1),
        LayersWidget::build(),
        sized_box!(height: 1),
        widgets::SingleChildWidget::build(None).with(ToolUiContainer),
    ]
    .build(commands)
}

pub(super) fn setup_ui(commands: &mut Commands) -> Entity {
    let settings_tab = Box::new(|commands: &mut Commands| {
        FlexWidget::column(vec![
            InfoCounts::build(),
            Divider::build('-'),
            DebugOptions::build(),
        ])
        .build(commands)
    });

    let editor_tab = Box::new(editor_ui_builder);

    let root_entity = SingleChildWidget::build(Some(widgets::TabView::build(vec![
        ("Editor", editor_tab),
        ("Settings", settings_tab),
    ])))
    .with((
        attachments::Root {
            enabled: true,
            position: IVec2 { x: 0, y: -16 },
            size: UVec2 { x: 32, y: 32 },
        },
        UiPhysicsGridMarker,
        attachments::Border::ASCII.padded(),
        attachments::RenderBundle::default(),
        DebugMenuMarker,
    ))
    .build(commands);

    root_entity
}

#[derive(Debug, Component)]
pub(crate) struct DebugMenuMarker;
