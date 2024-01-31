use crate::ui::Border;

use super::ui;
use bevy::{
    asset::{AssetServer, Handle},
    ecs::{
        bundle::Bundle,
        system::{Commands, Res},
    },
    math::{IVec2, UVec2},
    transform::components::{GlobalTransform, Transform},
};
use bevy_ascii_game::{
    atlas::{CharacterSet, FontAtlasUser},
    font::{CustomFont, CustomFontSource, FontSize},
};

pub fn setup_ui(mut commands: Commands, server: Res<AssetServer>) {
    let font = &server.load("FiraCode-Regular.ttf");
    let text_a = commands
        .spawn((
            UiRenderBundle::from_font(font),
            ui::Text {
                text: "Text A".into(),
            },
            ui::Widget::new::<ui::TextLogic>(),
        ))
        .id();
    let text_b: bevy::prelude::Entity = commands
        .spawn((
            UiRenderBundle::from_font(font),
            ui::Text {
                text: "Text B".into(),
            },
            ui::Widget::new::<ui::TextLogic>(),
        ))
        .id();

    let divider: bevy::prelude::Entity = commands
        .spawn((
            UiRenderBundle::from_font(font),
            ui::Divider { character: '=' },
            ui::Widget::new::<ui::DividerLogic>(),
        ))
        .id();

    let text_1: bevy::prelude::Entity = commands
        .spawn((
            UiRenderBundle::from_font(font),
            ui::Text {
                text: "Text 1".into(),
            },
            ui::Widget::new::<ui::TextLogic>(),
        ))
        .id();

    let text_2: bevy::prelude::Entity = commands
        .spawn((
            UiRenderBundle::from_font(font),
            ui::Text {
                text: "Text 2".into(),
            },
            ui::Widget::new::<ui::TextLogic>(),
        ))
        .id();
    let column = commands
        .spawn((
            ui::Column {
                children: vec![text_a, text_b, divider, text_1, text_2],
            },
            ui::Widget::new::<ui::ColumnLogic>(),
        ))
        .id();

    commands.spawn((
        ui::Root,
        ui::Container { child: column },
        ui::Widget::new::<ui::ContainerLogic>(),
        ui::Positioned {
            offset: IVec2::ZERO,
            size: UVec2 { x: 15, y: 10 },
        },
        ui::BorderBundle::new(Border::symmetric(
            Some('|'),
            Some('-'),
            Some([',', '.', '`', '\'']),
        )),
        UiRenderBundle::from_font(font),
    ));
}

#[derive(Bundle)]
pub struct UiRenderBundle {
    pub font_atlas_user: FontAtlasUser,
    pub font: CustomFont,
    pub character_set: CharacterSet,
    pub font_size: FontSize,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl UiRenderBundle {
    pub fn from_font(font: &Handle<CustomFontSource>) -> Self {
        Self {
            font: CustomFont(font.clone()),
            font_atlas_user: FontAtlasUser,
            character_set: Default::default(),
            font_size: Default::default(),
            global_transform: Default::default(),
            transform: Default::default(),
        }
    }
}
