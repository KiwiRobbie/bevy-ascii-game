use bevy::prelude::*;

pub struct DrawToolPlugin;

impl Plugin for DrawToolPlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Debug, Component)]
pub struct DrawTool {}

#[derive(Debug, Component)]
pub struct DrawToolUi {}

fn setup_draw_tool() {}

fn update_draw_tool() {}

fn update_draw_tool_ui() {}
