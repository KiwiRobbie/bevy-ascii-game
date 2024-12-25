use bevy::prelude::*;

pub mod draw;
pub mod shape;
pub mod text;

#[derive(Debug, Component)]
pub struct FocusedTool;

#[derive(Debug, Component)]
pub struct ExclusiveKeyboardEventHandler;

#[derive(Debug, Component, Deref, DerefMut)]
pub struct ToolUiEntity(pub Entity);

pub struct EditorToolsPlugin;
impl Plugin for EditorToolsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((text::TextPlugin,));
    }
}
