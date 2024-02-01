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
pub struct Divider {
    pub character: char,
}

#[derive(Debug, Default)]
pub struct DividerLogic;
impl WidgetLayoutLogic for DividerLogic {
    fn layout(
        &self,
        _entity: Entity,
        constraint: &Constraint,
        _world: &World,
        _commands: &mut Commands,
    ) -> UVec2 {
        return UVec2 {
            x: *constraint.width.as_ref().unwrap().end(),
            y: 1,
        };
    }

    fn children(&self, _entity: Entity, _world: &World) -> Vec<Entity> {
        vec![]
    }
}

#[derive(Bundle)]
pub struct DividerBundle {
    pub divider: Divider,
    pub layout: WidgetLayout,
    pub render: RenderBundle,
}
impl DividerBundle {
    pub fn new(character: char, font: &Handle<CustomFontSource>) -> Self {
        Self {
            layout: WidgetLayout::new::<DividerLogic>(),
            render: RenderBundle::from_font(font),
            divider: Divider { character },
        }
    }
    pub fn spawn(
        commands: &mut Commands,
        character: char,
        font: &Handle<CustomFontSource>,
    ) -> Entity {
        commands.spawn(Self::new(character, font)).id()
    }
}
