use bevy::prelude::*;

use crate::{layout::widget_layout::WidgetLayout, widget_builder::WidgetBuilderFn};

use super::{container::ContainerLogic, SingleChildWidget};

type StackBuilderFn = Box<dyn Fn(&mut Commands) -> Entity + Send + Sync>;

#[derive(Component)]
#[require(SingleChildWidget)]
pub struct Stack {
    builders: Vec<StackBuilderFn>,
    active: usize,
}

impl Stack {
    pub fn build<'a>(builders: Vec<StackBuilderFn>) -> WidgetBuilderFn<'a> {
        Box::new(|commands| {
            let child = builders[0](commands);
            commands
                .spawn((
                    Stack {
                        builders: builders,
                        active: 0,
                    },
                    WidgetLayout::new::<ContainerLogic>(),
                ))
                .add_child(child)
                .id()
        })
    }

    pub fn set_active(&mut self, entity: Entity, active: usize, commands: &mut Commands) {
        assert!(active < self.builders.len());
        self.active = active;
        let child_entity = self.builders[active](commands);
        commands
            .entity(entity)
            .despawn_descendants()
            .add_child(child_entity);
    }

    pub fn prev(&mut self, entity: Entity, commands: &mut Commands) {
        let stack_builders = self.builders.len();
        let new_active = (self.active + stack_builders.saturating_sub(1)) % stack_builders;

        self.set_active(entity, new_active, commands);
    }

    pub fn next(&mut self, entity: Entity, commands: &mut Commands) {
        let stack_builders = self.builders.len();
        let new_active = (self.active + 1) % stack_builders;

        self.set_active(entity, new_active, commands);
    }
    pub fn get_active(&self) -> usize {
        self.active
    }
}
