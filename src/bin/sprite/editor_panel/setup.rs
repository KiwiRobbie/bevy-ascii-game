use ascii_ui::{
    attachments::{self, Flex},
    col,
    mouse::InteractableMarker,
    row,
    widget_builder::{WidgetBuilder, WidgetSaver},
    widgets::{self, Divider, FlexWidget, SingleChildWidget},
};
use bevy::{
    ecs::{
        component::Component,
        system::{Commands, ResMut},
    },
    math::{IVec2, UVec2},
};

use bevy_ascii_game::{
    physics_grids::UiPhysicsGridMarker,
    widgets::{DebugOptions, InfoCounts},
};

use super::state::EditorPanelState;

pub(super) fn setup_ui(mut commands: Commands, mut menu_state: ResMut<EditorPanelState>) {
    let menu_state = &mut *menu_state;

    let settings_tab = FlexWidget::column(vec![
        InfoCounts::build(),
        Divider::build('-'),
        DebugOptions::build(),
    ])(&mut commands);

    let editor_tab = col![
        row![
            widgets::Divider::build('=').with(Flex::new(1)),
            widgets::Text::build(" Tools "),
            widgets::Divider::build('=').with(Flex::new(1)),
        ],
        widgets::SingleChildWidget::build(None).with(attachments::SizedBox::vertical(1)),
        widgets::Text::build(" Type   (T)"),
        widgets::Text::build(" Insert (R)"),
        widgets::Text::build(" Draw   (L)"),
        widgets::Text::build(" Shape  (S)"),
        widgets::SingleChildWidget::build(None).with(attachments::SizedBox::vertical(1)),
        row![
            widgets::Divider::build('=').with(Flex::new(1)),
            widgets::Text::build(" Layers "),
            widgets::Divider::build('=').with(Flex::new(1)),
        ],
        widgets::SingleChildWidget::build(None).with(attachments::SizedBox::vertical(1)),
        row![
            widgets::Checkbox::build(),
            widgets::SingleChildWidget::build(None).with(attachments::SizedBox::horizontal(1)),
            widgets::Text::build("background").with(Flex::new(1)),
            widgets::Button::build_raw("^"),
            widgets::SingleChildWidget::build(None).with(attachments::SizedBox::horizontal(1)),
            widgets::Button::build_raw("v"),
        ],
        row![
            widgets::Checkbox::build(),
            widgets::SingleChildWidget::build(None).with(attachments::SizedBox::horizontal(1)),
            widgets::Text::build("layer 1").with(Flex::new(1)),
            widgets::Button::build_raw("^"),
            widgets::SingleChildWidget::build(None).with(attachments::SizedBox::horizontal(1)),
            widgets::Button::build_raw("v"),
        ]
        .with(attachments::MainAxisAlignment::SpaceBetween),
        row![
            widgets::Checkbox::build(),
            widgets::SingleChildWidget::build(None).with(attachments::SizedBox::horizontal(1)),
            widgets::Text::build("layer 2").with(Flex::new(1)),
            widgets::Button::build_raw("^"),
            widgets::SingleChildWidget::build(None).with(attachments::SizedBox::horizontal(1)),
            widgets::Button::build_raw("v"),
        ]
        .with(attachments::MainAxisAlignment::SpaceBetween),
        widgets::SingleChildWidget::build(None).with(attachments::SizedBox::vertical(1)),
        row![
            widgets::SingleChildWidget::build(Some(widgets::Text::build("New Layer")))
                .with(attachments::Flex::new(1))
                .with(attachments::Padding::symmetric(1, 0)),
            widgets::Button::build("Create"),
        ],
        widgets::SingleChildWidget::build(None).with(attachments::SizedBox::vertical(1)),
        widgets::SingleChildWidget::build(None).save_id(&mut menu_state.tool_container),
    ](&mut commands);

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
