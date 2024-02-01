use bevy::{
    ecs::{
        bundle::Bundle, component::Component, entity::Entity, reflect::ReflectComponent,
        system::Commands, world::World,
    },
    math::{IVec2, UVec2},
    reflect::{std_traits::ReflectDefault, Reflect},
};

use crate::{
    attachments::padding::Padding,
    layout::{
        constraint::Constraint,
        positioned::Positioned,
        widget_layout::{WidgetLayout, WidgetLayoutLogic},
    },
};

#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
pub struct Container {
    pub child: Option<Entity>,
}

#[derive(Debug, Default)]
pub struct ContainerLogic;
impl WidgetLayoutLogic for ContainerLogic {
    fn layout(
        &self,
        entity: Entity,
        constraint: &Constraint,
        world: &World,
        commands: &mut Commands,
    ) -> UVec2 {
        let container = world
            .get::<Container>(entity)
            .expect("Container Logic without Container!");

        let padding = world
            .get::<Padding>(entity)
            .map(|p| p.clone())
            .unwrap_or_default();

        let constraint = padding.0.shrink_constraint(constraint);

        if let Some(child) = container.child {
            let child_widget = world
                .get::<WidgetLayout>(child)
                .expect("Container child invalid!");

            let size = (child_widget.logic).layout(child, &constraint, world, commands);

            let offset = IVec2 {
                x: padding.0.left as i32,
                y: padding.0.top as i32,
            };

            commands.entity(child).insert(Positioned {
                offset,
                size: constraint.constrain(size),
            });
        }

        return constraint.max();
    }
    fn children(&self, entity: Entity, world: &World) -> Vec<Entity> {
        let container = world
            .get::<Container>(entity)
            .expect("Root logic without Root!");
        container.child.map_or(vec![], |child| vec![child])
    }
}

#[derive(Debug, Bundle)]
pub struct ContainerBundle<T: Bundle> {
    attachments: T,
    container: Container,
    layout: WidgetLayout,
}

impl<T: Bundle> ContainerBundle<T> {
    pub fn new(child: Option<Entity>, attachments: T) -> Self {
        Self {
            attachments,
            container: Container { child },
            layout: WidgetLayout::new::<ContainerLogic>(),
        }
    }

    pub fn spawn(commands: &mut Commands, child: Option<Entity>, attachments: T) -> Entity {
        commands.spawn(Self::new(child, attachments)).id()
    }
}
