use bevy::{
    asset::Handle,
    ecs::{bundle::Bundle, component::Component, entity::Entity, system::Commands, world::World},
    math::UVec2,
};
use glyph_render::font::CustomFontSource;

use crate::{
    layout::{
        constraint::Constraint,
        widget_layout::{WidgetLayout, WidgetLayoutLogic},
    },
    render::bundle::RenderBundle,
};

#[derive(Debug, Component)]
pub struct Text {
    pub text: String,
}
#[derive(Debug, Default)]

pub struct TextLogic;
impl WidgetLayoutLogic for TextLogic {
    fn layout(
        &self,
        entity: Entity,
        _constraint: &Constraint,
        world: &World,
        _commands: &mut Commands,
    ) -> UVec2 {
        let text = world
            .get::<Text>(entity)
            .expect("Text Widget Logic missing Text Component!");

        return UVec2 {
            x: text.text.len() as u32,
            y: 1,
        };
    }

    fn children(&self, _entity: Entity, _world: &World) -> Vec<Entity> {
        vec![]
    }
}

#[derive(Bundle)]
pub struct TextBundle {
    pub text: Text,
    pub layout: WidgetLayout,
    pub render: RenderBundle,
}
impl TextBundle {
    pub fn new(text: String, font: &Handle<CustomFontSource>) -> Self {
        Self {
            layout: WidgetLayout::new::<TextLogic>(),
            render: RenderBundle::from_font(font),
            text: Text { text },
        }
    }
    pub fn spawn(commands: &mut Commands, text: String, font: &Handle<CustomFontSource>) -> Entity {
        commands.spawn(Self::new(text, font)).id()
    }
}
