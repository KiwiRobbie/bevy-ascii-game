use std::sync::Arc;

use ascii_ui::{
    attachments::{self, Flex},
    col,
    mouse::InteractableMarker,
    row,
    widget_builder::WidgetSaver,
    widgets::{self, button::ButtonJustPressedMarker},
};
use bevy::{
    input::keyboard::{Key, KeyboardInput},
    prelude::*,
};
use bevy_ascii_game::physics_grids::{GamePhysicsGrid, GamePhysicsGridMarker};
use glyph_render::{
    glyph_buffer::TargetGlyphBuffer,
    glyph_render_plugin::{GlyphTexture, GlyphTextureSource},
    glyph_sprite::GlyphSprite,
};
use spatial_grid::{depth::Depth, global_position::GlobalPosition, position::Position};

use crate::{
    layers::{EditorLayer, SelectedEditorLayer},
    tools::{ExclusiveKeyboardEventHandler, FocusedTool},
};

use super::BuildToolUi;

#[derive(Debug, Default, PartialEq, Eq)]
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
    cursor_entity: Entity,
    cursors: [Handle<GlyphTexture>; 3],
}

#[derive(Debug, Component)]
pub struct TypeToolUi {
    mode_entity: Entity,
}

fn build_ui(commands: &mut Commands) -> Entity {
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
        ],
    ];
    ui_builder
        .apply(commands)
        .with((TypeToolUi { mode_entity }, FocusedTool))
        .build(commands)
}

pub(crate) fn spawn_type_tool(commands: &mut Commands, glyph_textures: &mut Assets<GlyphTexture>) {
    let root_entity = commands.spawn(()).id();

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

    let cursor_entity = commands
        .spawn((
            GlyphSprite {
                texture: cursors[0].clone(),
                offset: IVec2 { x: 0, y: 0 },
            },
            Position(IVec2::new(-10, 0)),
            Depth(10.0),
            GamePhysicsGridMarker,
        ))
        .id();

    commands
        .entity(root_entity)
        .insert((
            FocusedTool,
            ExclusiveKeyboardEventHandler,
            TypeTool {
                mode: TypeMode::Regular,
                cursor_entity,
                cursors,
            },
            BuildToolUi(Box::new(build_ui)),
        ))
        .add_child(cursor_entity);
}

fn type_tool_cursor_update(
    mut commands: Commands,
    game_grid: Res<GamePhysicsGrid>,
    time: Res<Time>,
    q_tool: Query<(Entity, &TypeTool)>,
    mut q_cursor: Query<(&mut GlyphSprite, Has<TargetGlyphBuffer>)>,
) {
    let Ok((tool_entity, tool)) = q_tool.get_single() else {
        return;
    };
    let Ok((mut cursor_sprite, has_target)) = q_cursor.get_mut(tool.cursor_entity) else {
        return;
    };
    if time.elapsed_secs().fract() < 0.5 {
        if !has_target {
            commands
                .entity(tool_entity)
                .insert(TargetGlyphBuffer(game_grid.unwrap()));
        }
        let active_cursor = &tool.cursors[tool.mode.id()];
        if &cursor_sprite.texture != active_cursor {
            cursor_sprite.texture = active_cursor.clone();
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
        (Entity, &mut TypeTool, Has<ExclusiveKeyboardEventHandler>),
        (With<FocusedTool>, Without<TypeToolUi>),
    >,
    mut q_cursor: Query<(&mut Position,)>,
    mut q_layer: Query<(&mut EditorLayer, &GlobalPosition), With<SelectedEditorLayer>>,
) {
    let Ok((tool_entity, tool, exclusive)) = q_tool.get_single_mut() else {
        return;
    };
    let Ok((mut cursor_position,)) = q_cursor.get_mut(tool.cursor_entity) else {
        return;
    };

    if exclusive {
        let mut layer = q_layer.get_single_mut().ok();

        for ev in ev_keyboard.read() {
            if ev.state.is_pressed() {
                enum Input {
                    None,
                    Key(char),
                    Backspace,
                    Delete,
                }

                let mut input_character: Input = Input::None;
                match ev.key_code {
                    KeyCode::Escape => {
                        commands.entity(tool_entity).despawn_recursive();
                    }

                    KeyCode::ArrowLeft => cursor_position.x -= 1,
                    KeyCode::ArrowRight => cursor_position.x += 1,
                    KeyCode::ArrowDown => cursor_position.y -= 1,
                    KeyCode::ArrowUp => cursor_position.y += 1,
                    KeyCode::Space => {
                        input_character = Input::Key(' ');
                    }
                    KeyCode::Backspace => {
                        input_character = Input::Backspace;
                    }
                    KeyCode::Delete => {
                        input_character = Input::Delete;
                    }
                    _ => match &ev.logical_key {
                        Key::Character(key) => {
                            if key.chars().count() == 1 {
                                input_character = key
                                    .chars()
                                    .next()
                                    .map(|character| Input::Key(character))
                                    .unwrap_or(Input::None);
                            }
                        }
                        _ => {}
                    },
                }

                if let Some((ref mut layer, layer_position)) = layer.as_mut() {
                    match input_character {
                        Input::Key(character) => {
                            if layer
                                .write_character(**cursor_position - ***layer_position, character)
                                .is_ok()
                            {
                                if tool.mode != TypeMode::Inplace {
                                    cursor_position.x += 1;
                                }
                            };
                        }
                        Input::Backspace => {
                            cursor_position.x -= 1;
                            let _ =
                                layer.write_character(**cursor_position - ***layer_position, ' ');
                        }
                        Input::Delete => {
                            let _ =
                                layer.write_character(**cursor_position - ***layer_position, ' ');
                            cursor_position.x += 1;
                        }
                        Input::None => {}
                    }
                };
            }
        }
    } else {
        ev_keyboard.clear();
    }
}

fn type_tool_ui_update(
    mut q_tool: Query<&mut TypeTool, (With<FocusedTool>, Without<TypeToolUi>)>,
    q_ui_root: Query<&TypeToolUi, Without<TypeTool>>,
    mut q_mode_text: Query<(&mut widgets::Text, Has<ButtonJustPressedMarker>)>,
) {
    let Ok(mut tool) = q_tool.get_single_mut() else {
        return;
    };

    let Ok(tool_ui) = q_ui_root.get_single() else {
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
        app.add_systems(
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
