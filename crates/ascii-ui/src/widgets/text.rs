use bevy::{
    asset::Handle,
    ecs::{
        bundle::Bundle, component::Component, entity::Entity, reflect::ReflectComponent,
        system::Commands, world::World,
    },
    math::UVec2,
    reflect::Reflect,
};
use glyph_render::font::CustomFontSource;

use crate::{
    layout::{
        constraint::Constraint,
        widget_layout::{WidgetLayout, WidgetLayoutLogic},
    },
    render::bundle::RenderBundle,
};

#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
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
pub struct TextBundle<T: Bundle> {
    pub text: Text,
    pub layout: WidgetLayout,
    pub render: RenderBundle,
    pub attachments: T,
}
impl<T: Bundle> TextBundle<T> {
    pub fn new(text: String, font: &Handle<CustomFontSource>, attachments: T) -> Self {
        Self {
            layout: WidgetLayout::new::<TextLogic>(),
            render: RenderBundle::from_font(font),
            text: Text { text },
            attachments,
        }
    }
    pub fn spawn(
        commands: &mut Commands,
        text: String,
        font: &Handle<CustomFontSource>,
        attachments: T,
    ) -> Entity {
        commands.spawn(Self::new(text, font, attachments)).id()
    }
}
