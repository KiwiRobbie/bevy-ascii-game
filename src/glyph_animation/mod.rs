use bevy::{
    asset::{io::Reader, Asset, AssetLoader, AsyncReadExt, Handle},
    ecs::component::Component,
    math::UVec2,
    reflect::TypePath,
    utils::{HashMap, HashSet},
};

#[derive(Default)]
pub struct GlyphAnimationAssetLoader {}
impl GlyphAnimationAssetLoader {
    fn parse_bytes(bytes: Vec<u8>) -> Vec<String> {
        bytes
            .split(|&b| b == b'\n')
            .map(|line| {
                String::from_utf8(line.strip_suffix(b"\r").unwrap_or(line).to_vec()).unwrap()
            })
            .collect::<Vec<_>>()
    }
}

impl AssetLoader for GlyphAnimationAssetLoader {
    type Asset = GlyphAnimationSource;
    type Error = anyhow::Error;
    type Settings = ();

    fn extensions(&self) -> &[&str] {
        &["anim.ron"]
    }

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let meta = {
                let mut meta = ron::de::from_bytes::<GlyphAnimationMeta>(&bytes)?;
                for frame in meta.frames.iter_mut() {
                    frame.size = match frame.size {
                        (0, 0) => meta.size,
                        _ => frame.size,
                    }
                }
                meta
            };

            let unique_assets = meta
                .frames
                .iter()
                .map(|frame| frame.asset.clone())
                .collect::<HashSet<_>>();

            let mut frames_data = HashMap::new();
            for asset_path in unique_assets.iter() {
                frames_data.insert(
                    asset_path,
                    Self::parse_bytes(load_context.read_asset_bytes(asset_path).await.unwrap()),
                );
            }

            let mut frames: Vec<GylphAnimationFrame> = Vec::with_capacity(meta.frames.len());
            let mut cursor = UVec2::ZERO;
            for frame in meta.frames {
                let mut frame = frame.clone();
                let (frame_count, frame_step) = match frame.frame_count {
                    CountDirection::Single => (1, UVec2::ZERO),
                    CountDirection::X(x) => (x, UVec2::X * frame.size.0),
                    CountDirection::Y(y) => (y, UVec2::Y * frame.size.1),
                };

                cursor = match frame.start {
                    FrameIndex::Frame(x, y) => UVec2 { x, y } * Into::<UVec2>::into(frame.size),
                    FrameIndex::Pixel(x, y) => UVec2 { x, y },
                    FrameIndex::NextX => cursor + UVec2::X * frame.size.0,
                    FrameIndex::NextY => cursor + UVec2::Y * frame.size.1,
                };

                cursor -= frame_step; // Cancel step for first frame when reading sequence of several frames
                for _ in 0..frame_count {
                    cursor += frame_step;
                    frames.push(GylphAnimationFrame::new(
                        &frame,
                        frames_data.get(&frame.asset).unwrap(),
                        cursor,
                        meta.size.into(),
                    ));
                    frame.start = match frame.frame_count {
                        CountDirection::X(_) => FrameIndex::NextX,
                        CountDirection::Y(_) => FrameIndex::NextY,
                        CountDirection::Single => unreachable!(),
                    };
                }
            }

            Ok(GlyphAnimationSource {
                name: meta.name.clone(),
                size: meta.size.into(),
                frames,
            })
        })
    }
}

#[derive(Asset, TypePath)]
pub struct GlyphAnimationSource {
    pub name: String,
    pub size: UVec2,
    pub frames: Vec<GylphAnimationFrame>,
}

#[derive(serde::Deserialize, Asset, TypePath, Clone)]
pub struct GlyphAnimationMeta {
    pub name: String,
    pub size: (u32, u32),

    #[serde(default)]
    pub default_name: Option<String>,

    pub frames: Vec<GlyphAnimationFrameMeta>,
}

#[derive(serde::Deserialize, Asset, TypePath, Clone)]
pub struct GlyphAnimationFrameMeta {
    pub asset: String,

    #[serde(default)]
    pub start: FrameIndex,

    #[serde(default)]
    pub size: (u32, u32),

    #[serde(default)]
    pub offset: (u32, u32),

    #[serde(default)]
    pub frame_count: CountDirection,
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

#[derive(Clone, Debug)]
pub struct GylphAnimationFrame {
    pub data: Vec<String>,
}
impl GylphAnimationFrame {
    fn new(
        frame: &GlyphAnimationFrameMeta,
        data: &[String],
        cursor: UVec2,
        frame_size: UVec2,
    ) -> Self {
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
            let suffix =
                " ".repeat(frame_size.x as usize - frame.offset.0 as usize - src_data_width);
            frame_data[dst_y] = prefix + &line[src_start_x..src_end_x] + &suffix;
        }

        Self { data: frame_data }
    }
}

#[derive(Component, Clone)]
pub struct GlyphAnimation {
    pub source: Handle<GlyphAnimationSource>,
    pub frame: u32,
}
