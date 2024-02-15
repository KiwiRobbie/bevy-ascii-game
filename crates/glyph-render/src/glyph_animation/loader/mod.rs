use std::sync::Arc;

use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt},
    math::{IVec2, UVec2},
    utils::{HashMap, HashSet},
};

use text_util::text_mirror::mirror_lines;

use crate::glyph_render_plugin::GlyphTextureSource;

use super::{GlyphAnimationFrame, GlyphAnimationSource};
pub mod meta;

use self::meta::{
    create_data, CountDirection, FrameIndex, FrameMeta, GlyphAnimationMeta, MirroredFrame,
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
                    frame.0.size = match frame.0.size {
                        (0, 0) => meta.size,
                        _ => frame.0.size,
                    };

                    if let MirroredFrame::Override(frame) = &mut frame.1 {
                        frame.size = match frame.size {
                            (0, 0) => meta.size,
                            _ => frame.size,
                        };
                    }
                }
                meta
            };

            let unique_assets = meta
                .frames
                .iter()
                .flat_map(|frame| {
                    if let MirroredFrame::Override(mirrored) = &frame.1 {
                        vec![frame.0.asset.clone(), mirrored.asset.clone()]
                    } else {
                        vec![frame.0.asset.clone()]
                    }
                })
                .collect::<HashSet<_>>();

            let mut source_file_data = HashMap::new();
            for asset_path in unique_assets.iter() {
                source_file_data.insert(
                    asset_path,
                    Self::parse_bytes(load_context.read_asset_bytes(asset_path).await.unwrap()),
                );
            }

            let mut frame_data: Vec<Vec<String>> = Vec::with_capacity(meta.frames.len());
            let mut mirrored_frame_data: Vec<Vec<String>> = Vec::new();
            build_frames(
                meta.size.into(),
                &mut meta.frames.iter().map(|frame| &frame.0),
                &mut frame_data,
                &source_file_data,
            );
            build_frames(
                meta.size.into(),
                &mut meta.frames.iter().flat_map(|frame| match &frame.1 {
                    MirroredFrame::Override(meta) => Some(meta),
                    _ => None,
                }),
                &mut mirrored_frame_data,
                &source_file_data,
            );
            let mut frames: Vec<(GlyphAnimationFrame, Option<GlyphAnimationFrame>)> =
                Vec::with_capacity(frame_data.len());
            let mut mirrored_iter = mirrored_frame_data.into_iter();
            for (data, meta) in frame_data.into_iter().zip(
                meta.frames
                    .iter()
                    .flat_map(|meta| vec![meta.clone(); meta.0.frame_count.count() as usize]),
            ) {
                let mirrored = match &meta.1 {
                    MirroredFrame::Auto(mirror_offset_x, mirror_offset_y) => {
                        Some(GlyphAnimationFrame::new(
                            mirror_lines(&data),
                            Into::<IVec2>::into(meta.0.offset)
                                + Into::<IVec2>::into((*mirror_offset_x, *mirror_offset_y)),
                        ))
                    }
                    MirroredFrame::Override(meta) => Some(GlyphAnimationFrame::new(
                        mirrored_iter.next().expect("Missing mirrored frame!"),
                        meta.offset.into(),
                    )),
                    MirroredFrame::None => None,
                };
                frames.push((
                    GlyphAnimationFrame::new(data, meta.0.offset.into()),
                    mirrored,
                ))
            }

            Ok(GlyphAnimationSource {
                name: meta.name.clone(),
                size: meta.size.into(),
                frames,
            })
        })
    }
}

fn build_frames(
    size: UVec2,
    meta: &mut dyn Iterator<Item = &FrameMeta>,
    frames: &mut Vec<Vec<String>>,
    frames_data: &bevy::utils::hashbrown::HashMap<&String, Vec<String>>,
) {
    let mut cursor = UVec2::ZERO;

    for frame in meta {
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

        // Cancel step for first frame when reading sequence of several frames
        for _ in 0..frame_count {
            frames.push(create_data(
                &frame,
                frames_data.get(&frame.asset).unwrap(),
                cursor,
                size,
            ));
            frame.start = match frame.frame_count {
                CountDirection::X(_) => FrameIndex::NextX,
                CountDirection::Y(_) => FrameIndex::NextY,
                CountDirection::Single => break,
            };
            cursor += frame_step;
        }
    }
}
