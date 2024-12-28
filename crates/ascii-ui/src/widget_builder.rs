use bevy::prelude::*;

type WidgetBuilderFn<'a> = Box<dyn (FnOnce(&mut Commands) -> Entity) + Send + Sync + 'a>;

pub struct WidgetBuilder<'a> {
    builder: WidgetBuilderFn<'a>,
}

impl<'a, 'b> WidgetBuilder<'a>
where
    'a: 'b,
{
    pub fn new<F>(builder: F) -> WidgetBuilder<'b>
    where
        F: FnOnce(&mut Commands<'_, '_>) -> Entity + Send + Sync + 'b,
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
        Self::new(move |_| entity)
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
    fn save_id(self, store: &'static mut T) -> Self;
}
impl<'a> WidgetSaver<'a, Entity> for WidgetBuilder<'a> {
    fn save_id(self, store: &'static mut Entity) -> Self {
        Self::new(move |commands: &mut Commands| {
            let entity = self.build(commands);
            *store = entity;
            entity
        })
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
