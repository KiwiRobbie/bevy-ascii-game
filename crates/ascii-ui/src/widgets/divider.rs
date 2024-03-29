use bevy::{
    ecs::{
        bundle::Bundle, component::Component, entity::Entity, reflect::ReflectComponent,
        system::Commands, world::World,
    },
    math::UVec2,
    reflect::Reflect,
};

use crate::{
    layout::{
        constraint::Constraint,
        widget_layout::{WidgetLayout, WidgetLayoutLogic},
    },
    render::bundle::RenderBundle,
    widget_builder::WidgetBuilderFn,
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
    pub fn new(character: char) -> Self {
        Self {
            layout: WidgetLayout::new::<DividerLogic>(),
            render: RenderBundle::default(),
            divider: Divider { character },
        }
    }
    pub fn spawn(commands: &mut Commands, character: char) -> Entity {
        commands.spawn(Self::new(character)).id()
    }
}

impl Divider {
    pub fn build<'a>(character: char) -> WidgetBuilderFn<'a> {
        Box::new(move |commands| {
            commands
                .spawn((
                    Self { character },
                    RenderBundle::default(),
                    WidgetLayout::new::<DividerLogic>(),
                ))
                .id()
        })
    }
}
