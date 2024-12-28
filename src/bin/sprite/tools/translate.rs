use ascii_ui::{
    attachments::{self, Flex},
    col, row,
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
use std::sync::Arc;

use crate::{
    layers::{EditorLayer, SelectedEditorLayer},
    tools::{ExclusiveKeyboardEventHandler, FocusedTool},
};

use super::BuildToolUi;

#[derive(Debug, Component)]
#[require(Position)]
pub struct TranslateTool {
    initial: Option<IVec2>,
}

#[derive(Debug, Component)]
pub struct TranslateToolUi {}

fn build_ui(commands: &mut Commands) -> Entity {
    let ui_builder = col![row![
        widgets::Divider::build('=').with(Flex::new(1)),
        widgets::Text::build(" Move Tool "),
        widgets::Divider::build('=').with(Flex::new(1)),
    ],];
    ui_builder
        .apply(commands)
        .with((TranslateToolUi {}, FocusedTool))
        .build(commands)
}

pub(crate) fn spawn_translate_tool(commands: &mut Commands) {
    let root_entity = commands.spawn(()).id();

    commands.entity(root_entity).insert((
        FocusedTool,
        ExclusiveKeyboardEventHandler,
        TranslateTool {
            initial: Some(IVec2::ZERO),
        },
        BuildToolUi(Box::new(build_ui)),
    ));
}

fn translate_tool_update(
    mut commands: Commands,
    mut ev_keyboard: EventReader<KeyboardInput>,
    mut q_tool: Query<
        (
            Entity,
            &mut TranslateTool,
            Has<ExclusiveKeyboardEventHandler>,
        ),
        (With<FocusedTool>, Without<TranslateToolUi>),
    >,
    mut q_layer: Query<&mut Position, (With<EditorLayer>, With<SelectedEditorLayer>)>,
) {
    let Ok((tool_entity, mut tool, exclusive)) = q_tool.get_single_mut() else {
        return;
    };

    if exclusive {
        let mut layer = q_layer.get_single_mut().ok();

        for ev in ev_keyboard.read() {
            if ev.state.is_pressed() {
                enum Input {
                    None,
                    Cancel,
                    Confirm,
                    Move(IVec2),
                }

                let input: Input = match ev.key_code {
                    KeyCode::ArrowLeft => Input::Move(IVec2::NEG_X),
                    KeyCode::ArrowRight => Input::Move(IVec2::X),
                    KeyCode::ArrowDown => Input::Move(IVec2::NEG_Y),
                    KeyCode::ArrowUp => Input::Move(IVec2::Y),
                    KeyCode::Escape => Input::Cancel,
                    KeyCode::Enter => Input::Confirm,
                    _ => Input::None,
                };

                match input {
                    Input::None => {}
                    Input::Cancel => {
                        if let (Some(layer_position), Some(initial_position)) =
                            (layer.as_mut(), tool.initial)
                        {
                            tool.initial.get_or_insert(***layer_position);
                            ***layer_position = initial_position;
                        };
                        commands.entity(tool_entity).despawn_recursive();
                    }
                    Input::Confirm => {
                        commands.entity(tool_entity).despawn_recursive();
                    }
                    Input::Move(movement) => {
                        if let Some(layer_position) = layer.as_mut() {
                            tool.initial.get_or_insert(***layer_position);
                            ***layer_position += movement;
                        };
                    }
                };
            }
        }
    } else {
        ev_keyboard.clear();
    }
}

pub struct TranslatePlugin;
impl Plugin for TranslatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, translate_tool_update);
    }
}
