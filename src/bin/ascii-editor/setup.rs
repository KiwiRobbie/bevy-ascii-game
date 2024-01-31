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
    let font = server.load("FiraCode-Regular.ttf");

    let text_a = commands
        .spawn((
            UiRenderBundle::from_font(font.clone()),
            ui::Text {
                text: "Text A".into(),
            },
            ui::Widget::new::<ui::TextLogic>(),
        ))
        .id();
    let text_b = commands
        .spawn((
            UiRenderBundle::from_font(font),
            ui::Text {
                text: "Text B".into(),
            },
            ui::Widget::new::<ui::TextLogic>(),
        ))
        .id();

    let row = commands
        .spawn((
            ui::Row {
                children: vec![text_a, text_b],
            },
            ui::Widget::new::<ui::RowLogic>(),
        ))
        .id();

    commands.spawn((
        ui::Widget::new::<ui::RootLogic>(),
        ui::Root { child: row },
        ui::Positioned {
            offset: IVec2::ZERO,
            size: UVec2 { x: 30, y: 20 },
        },
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
    pub fn from_font(font: Handle<CustomFontSource>) -> Self {
        Self {
            font: CustomFont(font),
            font_atlas_user: FontAtlasUser,
            character_set: Default::default(),
            font_size: Default::default(),
            global_transform: Default::default(),
            transform: Default::default(),
        }
    }
}
