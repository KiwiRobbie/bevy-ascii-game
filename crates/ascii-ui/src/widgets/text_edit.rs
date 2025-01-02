use std::borrow::BorrowMut;

use bevy::{
    input::keyboard::{Key, KeyboardInput},
    prelude::*,
};

use crate::{
    attachments,
    mouse::{ActiveMarker, InteractableMarker},
    row, text,
    widget_builder::WidgetBuilder,
    widgets::{self, text},
};

use super::Text;

#[derive(Debug, Component)]
pub struct TextEdit {
    cursor: usize,
    content: Entity,
    text: String,
}

#[derive(Debug, PartialEq, Eq)]
enum Input<'a> {
    None,
    Left,
    Right,
    Backspace,
    Delete,
    Character(&'a str),
}

impl TextEdit {
    pub fn build<'a>(title: impl Into<String>) -> WidgetBuilder<'a> {
        use crate as ascii_ui;
        WidgetBuilder::new(|commands: &mut Commands| {
            let content = text!("testing").build(commands);
            row![
                widgets::SingleChildWidget::build_existing(Some(content)).with((
                    attachments::Border::UNICODE.padded(),
                    InteractableMarker,
                    TextEdit {
                        cursor: 0,
                        content,
                        text: "testing".into(),
                    },
                    attachments::Flex::new(1)
                )),
            ]
            .build(commands)
        })
    }

    pub(crate) fn update(
        mut q_self: Query<(&mut TextEdit, Has<ActiveMarker>), With<TextEdit>>,
        mut q_text: Query<&mut Text>,
        mut keyboard: EventReader<KeyboardInput>,
        time: Res<Time>,
        mut showing_cursor: Local<bool>,
    ) {
        let now_showing_cursor = time.elapsed_secs_wrapped() % 1.0 > 0.5;
        let cursor_updated = now_showing_cursor ^ *showing_cursor;
        *showing_cursor = now_showing_cursor;

        let mut updated = false;
        for ev in keyboard.read() {
            if !ev.state.is_pressed() {
                continue;
            }

            let input = match &ev.logical_key {
                Key::Character(character) => Input::Character(character),
                Key::Space => Input::Character(" "),
                Key::ArrowLeft => Input::Left,
                Key::ArrowRight => Input::Right,
                Key::Backspace => Input::Backspace,
                Key::Delete => Input::Delete,
                _ => Input::None,
            };

            apply_input(
                &mut q_self,
                &mut q_text,
                now_showing_cursor,
                cursor_updated,
                input,
            );
            updated = true;

            // Update cusror
        }
        if !updated {
            apply_input(
                &mut q_self,
                &mut q_text,
                now_showing_cursor,
                cursor_updated,
                Input::None,
            );
        }
    }
}

fn apply_input(
    q_self: &mut Query<'_, '_, (&mut TextEdit, Has<ActiveMarker>), With<TextEdit>>,
    q_text: &mut Query<'_, '_, &mut Text>,
    now_showing_cursor: bool,
    cursor_updated: bool,
    input: Input,
) {
    if input != Input::None || cursor_updated {
        for (mut text_edit, active) in q_self {
            if active {
                let text_edit = &mut *text_edit;
                let text = &mut text_edit.text;
                let cursor = &mut text_edit.cursor;
                match input {
                    Input::None => {}
                    Input::Left => {
                        *cursor = cursor.saturating_sub(1);
                    }
                    Input::Right => {
                        *cursor = cursor.saturating_add(1).min(text.chars().count());
                    }
                    Input::Backspace => {
                        if *cursor > 0 {
                            if let Some((position, _)) = text.char_indices().nth(*cursor - 1) {
                                *cursor -= 1;
                                text.remove(position);
                            }
                        }
                    }
                    Input::Delete => {
                        if let Some((position, _)) = text.char_indices().nth(*cursor) {
                            text.remove(position);
                        }
                    }
                    Input::Character(character) => {
                        text.insert_str(*cursor, character);
                        *cursor += character.chars().count();
                    }
                }

                let target_text = &mut q_text.get_mut(text_edit.content).unwrap().text;

                if now_showing_cursor {
                    *target_text = text
                        .chars()
                        .take(*cursor)
                        .chain(['_'].into_iter())
                        .chain(text.chars().skip(*cursor + 1))
                        .collect();
                } else {
                    *target_text = text.clone();
                }
            }
        }
    }
}
