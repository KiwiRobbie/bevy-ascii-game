use bevy::{
    ecs::{
        event::EventReader,
        query::With,
        system::{Query, Res, ResMut, Resource},
    },
    input::{
        mouse::{MouseButton, MouseWheel},
        touch::TouchInput,
        Input,
    },
    math::{Vec2, Vec3},
    prelude::{Deref, DerefMut},
    render::camera::Camera,
    transform::components::GlobalTransform,
    window::{PrimaryWindow, Window},
};

#[derive(Debug, Default, Resource, DerefMut, Deref)]
pub struct MouseInput(pub Option<MouseInputFrame>);

#[derive(Debug, Default)]
pub struct MouseInputFrame {
    pub world_position: Option<Vec3>,
    pub buttons: Option<Input<MouseButton>>,
    pub scroll: Option<Vec2>,
}

impl MouseInput {
    pub fn world_position(&self) -> Option<Vec3> {
        self.as_ref().and_then(|f| f.world_position)
    }
    pub fn buttons(&self) -> Option<&Input<MouseButton>> {
        self.as_ref().and_then(|f| f.buttons.as_ref())
    }
    pub fn scroll(&self) -> Option<Vec2> {
        self.as_ref().and_then(|f| f.scroll)
    }
    pub fn consume(&mut self) {
        (**self) = None;
    }

    pub fn pressed(&self, input: MouseButton) -> bool {
        self.buttons()
            .map(|buttons| buttons.pressed(input))
            .unwrap_or(false)
    }
    pub fn just_pressed(&self, input: MouseButton) -> bool {
        self.buttons()
            .map(|buttons| buttons.just_pressed(input))
            .unwrap_or(false)
    }
    pub fn just_released(&self, input: MouseButton) -> bool {
        self.buttons()
            .map(|buttons| buttons.just_released(input))
            .unwrap_or(false)
    }
}

pub fn update_mouse_position(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mouse_buttons: Res<Input<MouseButton>>,
    mut ev_mouse_scroll: EventReader<MouseWheel>,
    mut mouse_input: ResMut<MouseInput>,
    mut touch: EventReader<TouchInput>,
) {
    let mut frame = MouseInputFrame::default();
    frame.buttons = Some(mouse_buttons.clone());

    {
        let mut scroll_distance = Vec2::ZERO;
        for ev in ev_mouse_scroll.read() {
            scroll_distance += Vec2::new(ev.x, ev.y);
        }
        frame.scroll = Some(scroll_distance);
    }

    {
        if let Ok((camera, camera_transform)) = q_camera.get_single() {
            if let Some(position) = q_windows
                .single()
                .cursor_position()
                .or_else(|| touch.read().map(|ev| ev.position).last())
                .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                .map(|ray| ray.origin)
            {
                frame.world_position = Some(position);
            }
        }
    }

    **mouse_input = Some(frame);
}