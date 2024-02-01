use super::ui::{attachments, widgets};
use ascii_ui::attachments::border::Border;
use bevy::{
    asset::AssetServer,
    ecs::system::{Commands, Res},
    math::{IVec2, UVec2},
    prelude::Entity,
};

pub fn setup_ui(mut commands: Commands, server: Res<AssetServer>) {
    let font = &server.load("FiraCode-Regular.ttf");

    let text_a: Entity = widgets::TextBundle::spawn(&mut commands, "Text A".into(), font, ());
    let text_b: Entity = widgets::TextBundle::spawn(&mut commands, "Text B".into(), font, ());
    let divider: Entity = widgets::DividerBundle::spawn(&mut commands, '=', font);

    let debug_a = widgets::CheckboxBuilder::spawn(&mut commands, "Debug Position".into(), font);
    let debug_b = widgets::CheckboxBuilder::spawn(&mut commands, "Debug Colliders".into(), font);
    let debug_c = widgets::CheckboxBuilder::spawn(&mut commands, "Debug ECS UI".into(), font);

    let column = widgets::ColumnBundle::spawn(
        &mut commands,
        vec![text_a, text_b, divider, debug_a, debug_b, debug_c],
        (),
    );

    widgets::ContainerBundle::spawn(
        &mut commands,
        Some(column),
        (
            attachments::Root {
                enabled: true,
                position: IVec2::ZERO,
                size: UVec2 { x: 30, y: 10 },
            },
            attachments::BorderBundle::new(Border::symmetric(
                Some('|'),
                Some('-'),
                Some([',', '.', '`', '\'']),
            )),
            attachments::RenderBundle::from_font(font),
        ),
    );
}
