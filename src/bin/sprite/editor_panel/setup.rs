use ascii_ui::{
    attachments::{self, Flex},
    col,
    mouse::InteractableMarker,
    row, sized_box, text,
    widget_builder::{WidgetBuilder, WidgetSaver},
    widgets::{self, Divider, FlexWidget, SingleChildWidget},
};
use bevy::prelude::*;

use bevy_ascii_game::{
    physics_grids::UiPhysicsGridMarker,
    widgets::{DebugOptions, InfoCounts},
};

use super::state::EditorPanelState;

fn editor_ui_builder(commands: &mut Commands) -> Entity {
    col![
        row![
            widgets::Divider::build('=').with(Flex::new(1)),
            text!(" Tools "),
            widgets::Divider::build('=').with(Flex::new(1)),
        ],
        sized_box!(vertical: 1),
        text!(" Type   (T)"),
        text!(" Draw   (D)"),
        text!(" Shape  (S)"),
        sized_box!(vertical: 1),
        row![
            widgets::Divider::build('=').with(Flex::new(1)),
            text!(" Layers "),
            widgets::Divider::build('=').with(Flex::new(1)),
        ],
        sized_box!(vertical: 1),
        row![
            widgets::Checkbox::build(),
            sized_box!(vertical: 1),
            text!("background").with(Flex::new(1)),
            widgets::Button::build_raw("^"),
            sized_box!(vertical: 1),
            widgets::Button::build_raw("v"),
        ],
        row![
            widgets::Checkbox::build(),
            sized_box!(vertical: 1),
            text!("layer 1").with(Flex::new(1)),
            widgets::Button::build_raw("^"),
            sized_box!(vertical: 1),
            widgets::Button::build_raw("v"),
        ]
        .with(attachments::MainAxisAlignment::SpaceBetween),
        row![
            widgets::Checkbox::build(),
            sized_box!(vertical: 1),
            text!("layer 2").with(Flex::new(1)),
            widgets::Button::build_raw("^"),
            sized_box!(vertical: 1),
            widgets::Button::build_raw("v"),
        ]
        .with(attachments::MainAxisAlignment::SpaceBetween),
        widgets::SingleChildWidget::build(None).with(attachments::SizedBox::vertical(1)),
        row![
            widgets::SingleChildWidget::build(Some(text!("New Layer")))
                .with(attachments::Flex::new(1))
                .with(attachments::Padding::symmetric(1, 0)),
            widgets::Button::build("Create"),
        ],
        sized_box!(vertical: 2),
        text!("Selected Layer"),
        sized_box!(vertical: 1),
        text!("Size: 64 x 32"),
        text!("Name: background"),
        sized_box!(vertical: 1),
        // widgets::SingleChildWidget::build(None).save_id(&mut menu_state.tool_container),
    ](commands)
}

pub(super) fn setup_ui(mut commands: Commands, mut menu_state: ResMut<EditorPanelState>) {
    // let menu_state = &mut *menu_state;

    let settings_tab = Box::new(|commands: &mut Commands| {
        FlexWidget::column(vec![
            InfoCounts::build(),
            Divider::build('-'),
            DebugOptions::build(),
        ])(commands)
    });

    let editor_tab = Box::new(editor_ui_builder);

    SingleChildWidget::build(Some(widgets::TabView::build(vec![
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
        attachments::Border::symmetric(
            Some('|'),
            Some('-'),
            [Some(','), Some('.'), Some('`'), Some('\'')],
        )
        .padded(),
        attachments::RenderBundle::default(),
        DebugMenuMarker,
        InteractableMarker,
    ))
    .save_id(&mut menu_state.root_widget)(&mut commands);
}

#[derive(Debug, Component)]
pub(crate) struct DebugMenuMarker;
