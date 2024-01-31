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

    let text_a: Entity = widgets::TextBundle::spawn(&mut commands, "Text A".into(), font);
    let text_b: Entity = widgets::TextBundle::spawn(&mut commands, "Text B".into(), font);
    let divider: Entity = widgets::DividerBundle::spawn(&mut commands, '=', font);
    let text_1: Entity = widgets::TextBundle::spawn(&mut commands, "Text 1".into(), font);
    let text_2: Entity = widgets::TextBundle::spawn(&mut commands, "Text 2".into(), font);

    let column =
        widgets::ColumnBundle::spawn(&mut commands, vec![text_a, text_b, divider, text_1, text_2]);

    widgets::ContainerBundle::spawn(
        &mut commands,
        column,
        (
            attachments::Root {
                position: IVec2::ZERO,
                size: UVec2 { x: 15, y: 10 },
            },
            attachments::BorderBundle::new(Border::symmetric(
                Some('|'),
                Some('-'),
                Some([',', '.', '`', '\'']),
            )),
            attachments::RenderBundle::from_font(font),
        ),
    );

    // commands.spawn((
    //     ui::Root {
    //         position: IVec2::ZERO,
    //         size: UVec2 { x: 15, y: 10 },
    //     },
    //     ui::Container { child: column },
    //     ui::WidgetLayout::new::<ui::ContainerLogic>(),
    //     ui::BorderBundle::new(Border::symmetric(
    //         Some('|'),
    //         Some('-'),
    //         Some([',', '.', '`', '\'']),
    //     )),
    //     ui::RenderBundle::from_font(font),
    // ));
}
