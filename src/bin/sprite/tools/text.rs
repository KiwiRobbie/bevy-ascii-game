use std::sync::Arc;

use ascii_ui::{
    attachments::{self, Flex},
    col,
    mouse::InteractableMarker,
    row,
    widget_builder::{WidgetBuilder, WidgetSaver},
    widgets::{self, button::ButtonJustPressedMarker},
};
use bevy::{input::keyboard::KeyboardInput, prelude::*};
use bevy_ascii_game::physics_grids::{
    GamePhysicsGrid, GamePhysicsGridMarker, PrimaryGlyphBufferMarker,
};
use glyph_render::{
    glyph_buffer::TargetGlyphBuffer,
    glyph_render_plugin::{GlyphTexture, GlyphTextureSource},
    glyph_sprite::GlyphSprite,
};
use spatial_grid::{depth::Depth, position::Position};

use crate::tools::{ExclusiveKeyboardEventHandler, FocusedTool, FocusedToolUi};

#[derive(Debug, Default)]
pub enum TypeMode {
    #[default]
    Regular,
    Insert,
    Inplace,
}
impl TypeMode {
    pub fn id(&self) -> usize {
        match self {
            TypeMode::Regular => 0,
            TypeMode::Insert => 1,
            TypeMode::Inplace => 1,
        }
    }
    pub fn next(&self) -> Self {
        match self {
            TypeMode::Regular => TypeMode::Insert,
            TypeMode::Insert => TypeMode::Inplace,
            TypeMode::Inplace => TypeMode::Regular,
        }
    }
    pub fn cycle(&mut self) {
        *self = self.next();
    }
}

#[derive(Debug, Component)]
#[require(Position)]
pub struct TypeToolCursor;

#[derive(Debug, Component)]
#[require(Position)]
pub struct TypeTool {
    mode: TypeMode,
    active: bool,
    ui_entity: Entity,
    cursors: [Handle<GlyphTexture>; 3],
}

#[derive(Debug, Component)]
pub struct TypeToolUi {
    mode_entity: Entity,
}

fn type_tool_setup(mut commands: Commands, mut glyph_textures: ResMut<Assets<GlyphTexture>>) {
    use ascii_ui::widgets;

    let mut mode_entity = Entity::PLACEHOLDER;
    let ui_builder = col![
        row![
            widgets::Divider::build('=').with(Flex::new(1)),
            widgets::Text::build(" Type Tool "),
            widgets::Divider::build('=').with(Flex::new(1)),
        ],
        widgets::SingleChildWidget::build(None).with(attachments::SizedBox::vertical(1)),
        row![
            widgets::Text::build("Mode: "),
            widgets::Text::build("")
                .with(InteractableMarker)
                .save_id(&mut mode_entity)
        ]
    ];

    let ui_entity = ui_builder
        .apply(&mut commands)
        .with(TypeToolUi { mode_entity })
        .with(FocusedToolUi)(&mut commands);

    let cursors = [
        glyph_textures.add(GlyphTexture::new(Arc::new(GlyphTextureSource::new(
            1,
            1,
            Box::new(['_']),
        )))),
        glyph_textures.add(GlyphTexture::new(Arc::new(GlyphTextureSource::new(
            1,
            1,
            Box::new(['#']),
        )))),
        glyph_textures.add(GlyphTexture::new(Arc::new(GlyphTextureSource::new(
            1,
            1,
            Box::new(['#']),
        )))),
    ];
    commands.spawn((
        GlyphSprite {
            texture: cursors[0].clone(),
            offset: IVec2 { x: 0, y: 0 },
        },
        TypeTool {
            ui_entity,
            mode: TypeMode::Regular,
            active: true,
            cursors,
        },
        Position(IVec2::new(-10, 0)),
        Depth(10.0),
        FocusedTool,
        GamePhysicsGridMarker,
        ExclusiveKeyboardEventHandler,
    ));
}

fn type_tool_cursor_update(
    mut commands: Commands,
    game_grid: Res<GamePhysicsGrid>,
    time: Res<Time>,
    mut q_tool: Query<
        (Entity, &TypeTool, &mut GlyphSprite, Has<TargetGlyphBuffer>),
        (With<FocusedTool>, Without<TypeToolUi>),
    >,
) {
    let Ok((tool_entity, tool, mut sprite, has_target)) = q_tool.get_single_mut() else {
        return;
    };
    if time.elapsed_secs().fract() < 0.5 {
        if !has_target {
            commands
                .entity(tool_entity)
                .insert(TargetGlyphBuffer(game_grid.unwrap()));
        }
        let active_cursor = &tool.cursors[tool.mode.id()];
        if &sprite.texture != active_cursor {
            sprite.texture = active_cursor.clone();
        }
    } else {
        if has_target {
            commands.entity(tool_entity).remove::<TargetGlyphBuffer>();
        }
    }
}

fn type_tool_update(
    mut commands: Commands,
    mut ev_keyboard: EventReader<KeyboardInput>,
    mut q_tool: Query<
        (
            Entity,
            &mut TypeTool,
            &mut Position,
            Has<ExclusiveKeyboardEventHandler>,
        ),
        (With<FocusedTool>, Without<TypeToolUi>),
    >,
) {
    let Ok((tool_entity, mut tool, mut cursor_position, exclusive)) = q_tool.get_single_mut()
    else {
        return;
    };
    if tool.active {
        commands
            .entity(tool_entity)
            .insert(ExclusiveKeyboardEventHandler);
    } else {
        commands
            .entity(tool_entity)
            .remove::<ExclusiveKeyboardEventHandler>();
    };

    if exclusive {
        for ev in ev_keyboard.read() {
            if ev.state.is_pressed() {
                match ev.key_code {
                    KeyCode::Escape => {
                        tool.active = false;
                        commands
                            .entity(tool_entity)
                            .remove::<ExclusiveKeyboardEventHandler>();
                    }

                    KeyCode::ArrowLeft => cursor_position.x -= 1,
                    KeyCode::ArrowRight => cursor_position.x += 1,
                    KeyCode::ArrowDown => cursor_position.y -= 1,
                    KeyCode::ArrowUp => cursor_position.y += 1,
                    _ => {}
                }
            }
        }
    } else {
        ev_keyboard.clear();
    }
}

fn type_tool_ui_update(
    mut q_tool: Query<(&mut TypeTool, &Position), (With<FocusedTool>, Without<TypeToolUi>)>,
    q_ui_root: Query<&TypeToolUi, Without<TypeTool>>,
    mut q_mode_text: Query<(&mut widgets::Text, Has<ButtonJustPressedMarker>)>,
) {
    let Ok((mut tool, cursor_position)) = q_tool.get_single_mut() else {
        return;
    };

    let Ok(tool_ui) = q_ui_root.get(tool.ui_entity) else {
        return;
    };

    if let Ok((mut mode_text, pressed)) = q_mode_text.get_mut(tool_ui.mode_entity) {
        if pressed {
            tool.mode.cycle();
        }
        mode_text.text = match tool.mode {
            TypeMode::Regular => "Regular".into(),
            TypeMode::Insert => "Insert".into(),
            TypeMode::Inplace => "Inplace".into(),
        };
    }
}

pub struct TextPlugin;
impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, type_tool_setup).add_systems(
            Update,
            (
                type_tool_update,
                type_tool_cursor_update,
                type_tool_ui_update,
            )
                .chain(),
        );
    }
}
