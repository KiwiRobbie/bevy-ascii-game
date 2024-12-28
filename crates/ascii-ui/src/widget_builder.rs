use bevy::prelude::*;

type WidgetBuilderFn<'a> = Box<dyn (FnOnce(&mut Commands) -> Entity) + Send + Sync + 'a>;

pub struct WidgetBuilder<'a> {
    builder: WidgetBuilderFn<'a>,
}

impl<'a, 'b> WidgetBuilder<'a> {
    pub fn new<F>(builder: F) -> WidgetBuilder<'a>
    where
        F: FnOnce(&mut Commands<'_, '_>) -> Entity + Send + Sync + 'a,
    {
        WidgetBuilder {
            builder: Box::new(builder),
        }
    }

    pub fn build(self, commands: &mut Commands) -> Entity {
        (self.builder)(commands)
    }

    pub fn entity(entity: Entity) -> WidgetBuilder<'b> {
        let entity = entity.clone();
        WidgetBuilder::new(move |_| entity)
    }
    pub fn with<B: Bundle>(self, attachments: B) -> WidgetBuilder<'a> {
        Self::new(move |commands: &mut Commands| {
            let entity = self.build(commands);
            commands.entity(entity).insert(attachments).id()
        })
    }
    pub fn parent(self, parent: Entity) -> WidgetBuilder<'a> {
        WidgetBuilder::new(move |commands: &mut Commands| {
            let entity = self.build(commands);
            commands.entity(parent).add_child(entity);
            entity
        })
    }
    pub fn children(self, children: &'a [Entity]) -> WidgetBuilder<'a> {
        Self::new(move |commands: &mut Commands| {
            let entity = self.build(commands);
            commands.entity(entity).add_children(&children);
            entity
        })
    }
    pub fn apply(self, commands: &mut Commands) -> WidgetBuilder<'b> {
        let entity = self.build(commands);
        return WidgetBuilder {
            builder: Box::new(move |_| entity),
        };
    }
}
pub trait WidgetSaver<'a, T> {
    fn save_id(self, store: &'a mut T) -> Self;
}
impl<'a> WidgetSaver<'a, Entity> for WidgetBuilder<'a> {
    fn save_id(self, store: &'a mut Entity) -> Self {
        let func = move |commands: &mut Commands| {
            let entity = self.build(commands);
            *store = entity;
            entity
        };
        Self::new(func)
    }
}
impl<'a> WidgetSaver<'a, Option<Entity>> for WidgetBuilder<'a> {
    fn save_id(self, store: &'a mut Option<Entity>) -> Self {
        Self::new(move |commands: &mut Commands| {
            let entity = self.build(commands);
            *store = Some(entity);
            entity
        })
    }
}
impl<'a> From<Entity> for WidgetBuilder<'a> {
    fn from(value: Entity) -> Self {
        Self::entity(value)
    }
}
