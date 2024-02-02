use bevy::{
    ecs::{bundle::Bundle, entity::Entity, system::Commands},
    prelude::{Deref, DerefMut},
};

pub type WidgetBuilderFn<'a> = Box<dyn (FnOnce(&mut Commands) -> Entity) + 'a>;

#[derive(Deref, DerefMut)]
pub struct WidgetBuilderStruct<'a>(pub WidgetBuilderFn<'a>);

pub trait WidgetBuilder<'a, 'b> {
    fn entity(entity: Entity) -> WidgetBuilderFn<'a>;
    fn with<B: Bundle>(self, attachments: B) -> WidgetBuilderFn<'a>;
    fn apply(self, commands: &mut Commands) -> WidgetBuilderFn<'b>;
}

pub trait WidgetSaver<'a, T> {
    fn save_id(self, store: &'a mut T) -> WidgetBuilderFn<'a>;
}

impl<'a, 'b> WidgetBuilder<'a, 'b> for WidgetBuilderFn<'a> {
    fn entity(entity: Entity) -> WidgetBuilderFn<'a> {
        Box::new(move |_| entity)
    }
    fn with<B: Bundle>(self, attachments: B) -> WidgetBuilderFn<'a> {
        Box::new(move |commands: &mut Commands| {
            let entity = self(commands);
            commands.entity(entity).insert(attachments).id()
        })
    }

    fn apply(self, commands: &mut Commands) -> WidgetBuilderFn<'b> {
        let entity = self(commands);
        Box::new(move |_: &mut Commands| entity)
    }
}

impl<'a> WidgetSaver<'a, Entity> for WidgetBuilderFn<'a> {
    fn save_id(self, store: &'a mut Entity) -> Self {
        Box::new(move |commands: &mut Commands| {
            let entity = self(commands);
            *store = entity;
            entity
        })
    }
}

impl<'a> WidgetSaver<'a, Option<Entity>> for WidgetBuilderFn<'a> {
    fn save_id(self, store: &'a mut Option<Entity>) -> Self {
        Box::new(move |commands: &mut Commands| {
            let entity = self(commands);
            *store = Some(entity);
            entity
        })
    }
}
