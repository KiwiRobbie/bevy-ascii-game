use bevy::{asset::Asset, math::UVec2, reflect::TypePath};

#[derive(serde::Deserialize, Asset, TypePath, Clone)]
pub struct GlyphAnimationMeta {
    pub name: String,
    pub size: (u32, u32),

    #[serde(default)]
    pub default_name: Option<String>,

    pub frames: Vec<(FrameMeta, MirroredFrame)>,
}

#[derive(serde::Deserialize, Default, Clone)]
pub enum MirroredFrame {
    #[default]
    None,
    Auto(#[serde(default)] i32, #[serde(default)] i32),
    Override(FrameMeta),
}

#[derive(serde::Deserialize, Asset, TypePath, Clone)]
pub struct FrameMeta {
    pub asset: String,

    #[serde(default)]
    pub start: FrameIndex,

    #[serde(default)]
    pub size: (u32, u32),

    #[serde(default)]
    pub offset: (i32, i32),

    #[serde(default)]
    pub frame_count: CountDirection,
}

pub fn create_data(
    frame: &FrameMeta,
    data: &[String],
    cursor: UVec2,
    frame_size: UVec2,
) -> Vec<String> {
    let (start_x, start_y) = match frame.start {
        FrameIndex::Pixel(x, y) => (x, y),
        FrameIndex::Frame(x, y) => (x * frame_size.x, y * frame_size.y),
        FrameIndex::NextX => cursor.into(),
        FrameIndex::NextY => cursor.into(),
    };

    let mut frame_data = vec![String::new(); frame_size.y as usize];
    for (dst_y, src_y) in (start_y..start_y + frame.size.1).enumerate() {
        let line = &data[src_y as usize];

        let src_start_x = start_x as usize;
        let src_data_width = (frame.size.0 as usize).min(line.len() - src_start_x);
        let src_end_x = src_start_x + src_data_width;

        let prefix = " ".repeat(frame.offset.0 as usize);
        let suffix = " ".repeat(frame_size.x as usize - frame.offset.0 as usize - src_data_width);
        frame_data[dst_y] = prefix + &line[src_start_x..src_end_x] + &suffix;
    }

    frame_data
}
#[derive(serde::Deserialize, Clone)]
pub enum FrameIndex {
    Pixel(u32, u32),
    Frame(u32, u32),
    NextY,
    NextX,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub enum CountDirection {
    Single,
    X(u32),
    Y(u32),
}
impl CountDirection {
    pub fn count(&self) -> u32 {
        match self {
            Self::Single => 1,
            Self::X(x) => *x,
            Self::Y(y) => *y,
        }
    }
}

impl Default for CountDirection {
    fn default() -> Self {
        Self::Single
    }
}

impl Default for FrameIndex {
    fn default() -> Self {
        Self::NextY
    }
}
