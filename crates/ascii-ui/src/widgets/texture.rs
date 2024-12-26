use bevy::{
    ecs::{component::Component, entity::Entity, system::Commands, world::World},
    math::UVec2,
};

use crate::{
    layout::{
        constraint::Constraint,
        widget_layout::{WidgetLayout, WidgetLayoutLogic},
    },
    render::bundle::RenderBundle,
    widget_builder::WidgetBuilderFn,
};

#[derive(Component, Debug, Clone, Default)]
// #[reflect(Component)]
pub struct Texture {
    pub(crate) data: Box<[char]>,
    pub(crate) size: UVec2,
}
#[derive(Debug, Default)]

pub(crate) struct TextureLogic;
impl WidgetLayoutLogic for TextureLogic {
    fn layout(
        &self,
        entity: Entity,
        _constraint: &Constraint,
        world: &World,
        _commands: &mut Commands,
    ) -> UVec2 {
        let texture = world
            .get::<Texture>(entity)
            .expect("Texture Widget Logic missing Texture Component!");

        return texture.size;
    }
}

impl Texture {
    pub fn build<'a>(data: Box<[char]>, size: UVec2) -> WidgetBuilderFn<'a> {
        if data.len() == (size.x * size.y) as usize {
            Box::new(move |commands| {
                commands
                    .spawn((
                        Self { data, size },
                        RenderBundle::default(),
                        WidgetLayout::new::<TextureLogic>(),
                    ))
                    .id()
            })
        } else {
            Box::new(move |commands| {
                commands
                    .spawn((
                        Self {
                            data: Box::new(['?']),
                            size: UVec2::ONE,
                        },
                        RenderBundle::default(),
                        WidgetLayout::new::<TextureLogic>(),
                    ))
                    .id()
            })
        }
    }
}
