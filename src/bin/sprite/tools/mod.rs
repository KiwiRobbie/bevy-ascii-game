use bevy::prelude::*;

pub mod draw;
pub mod shape;
pub mod text;

#[derive(Debug, Component)]
pub struct FocusedTool;

#[derive(Debug, Component)]
pub struct ExclusiveKeyboardEventHandler;

#[derive(Component, Deref, DerefMut)]
pub struct BuildToolUi(pub Box<dyn Fn(&mut Commands) -> Entity + Send + Sync + 'static>);

pub struct EditorToolsPlugin;
impl Plugin for EditorToolsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((text::TextPlugin,));
    }
}
