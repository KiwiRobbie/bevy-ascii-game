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
            let meta = ron::de::from_bytes::<GlyphAnimationMeta>(&bytes)?;

            let unique_files = meta
                .frames
                .iter()
                .map(|frame| frame.asset.clone())
                .collect::<HashSet<_>>();

            let mut frames = HashMap::new();
            for asset_path in unique_files.iter() {
                frames.insert(
                    asset_path,
                    Self::parse_bytes(load_context.read_asset_bytes(asset_path).await.unwrap()),
                );
            }

            let frames: Vec<GylphAnimationFrame> = meta
                .frames
                .iter()
                .map(|frame| {
                    GylphAnimationFrame::new(
                        frame,
                        frames.get(&frame.asset).unwrap(),
                        meta.size.into(),
                    )
                })
                .collect();

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

    pub start: (u32, u32),
    pub size: (u32, u32),

    #[serde(default)]
    pub offset: (u32, u32),
}
pub struct GylphAnimationFrame {
    pub data: Vec<String>,
}
impl GylphAnimationFrame {
    fn new(frame: &GlyphAnimationFrameMeta, data: &[String], frame_size: UVec2) -> Self {
        let mut frame_data = vec![String::new(); frame_size.y as usize];
        for (dst_y, src_y) in (frame.start.1..frame.start.1 + frame.size.1).enumerate() {
            let src_start_x = frame.start.0 as usize;
            let src_end_x = src_start_x + frame.size.0 as usize;

            let prefix = " ".repeat(frame.offset.0 as usize);
            let suffix = " ".repeat((frame_size.x - frame.offset.0 - frame.size.0) as usize);
            frame_data[dst_y] = prefix + &data[src_y as usize][src_start_x..src_end_x] + &suffix;
        }

        Self { data: frame_data }
    }
}

#[derive(Component)]
pub struct GlyphAnimation {
    pub source: Handle<GlyphAnimationSource>,
    pub frame: u32,
}
