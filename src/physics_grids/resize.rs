use bevy::{
    ecs::{
        event::EventReader,
        system::{Query, Res},
    },
    log,
    math::{UVec2, Vec2},
    transform::components::Transform,
    window::WindowResized,
};
use glyph_render::{font::FontSize, glyph_buffer::GlyphBuffer};
use spatial_grid::grid::SpatialGrid;

use super::{GamePhysicsGrid, UiPhysicsGrid};

pub(super) fn grid_resize_update(
    mut resize_reader: EventReader<WindowResized>,
    mut q_glyph_buffer: Query<(
        &mut Transform,
        &mut SpatialGrid,
        &mut FontSize,
        &mut GlyphBuffer,
    )>,
    game_grid: Res<GamePhysicsGrid>,
    ui_grid: Res<UiPhysicsGrid>,
) {
    let (Some(game_grid), Some(ui_grid)) = (**game_grid, **ui_grid) else {
        return;
    };

    if let Some(e) = resize_reader.read().last() {
        log::info!("{:?}", e);

        if let Ok((mut transform, mut grid, mut font_size, mut buffer)) =
            q_glyph_buffer.get_mut(game_grid)
        {
            // let game_font = (e.width * 32.0 / 19.0 / 100.0) as u32;
            // **font_size = game_font;
            // grid.size = UVec2::new(font_size.advance(), font_size.line_spacing());

            *transform =
                Transform::from_translation(-0.5 * Vec2::new(e.width, e.height).extend(0.0));

            // buffer.size.y = (e.height / grid.size.y as f32) as u32;
        }

        if let Ok((mut transform, mut grid, mut font_size, mut buffer)) =
            q_glyph_buffer.get_mut(ui_grid)
        {
            let ui_font = (e.width * 32.0 / 19.0 / 200.0) as u32;
            **font_size = ui_font;
            grid.size = UVec2::new(font_size.advance(), font_size.line_spacing());
            *transform =
                Transform::from_translation(-0.5 * Vec2::new(e.width, e.height).extend(0.0));
            buffer.size.x = (e.width / grid.size.x as f32) as u32;
            buffer.size.y = (e.height / grid.size.y as f32) as u32;
        }
    }
}
