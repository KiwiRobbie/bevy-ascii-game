use std::any::{Any, TypeId};

use ascii_ui::widgets;
use bevy::{
    app::{Plugin, Update},
    ecs::{
        component::{Component, ComponentId, ComponentInfo},
        entity::Entity,
        query::{Changed, With},
        system::{Commands, Query, Res, Resource},
        world::World,
    },
    math::{IVec2, UVec2, Vec2},
    reflect::{ReflectFromPtr, ReflectRef, TypeRegistry},
};
use grid_physics::{position::Position, velocity::Velocity};

use crate::player::{
    input::keyboard::PlayerInputKeyboardMarker, movement::jump::PlayerJumpVelocity,
};

#[derive(Debug, Component, Default)]
pub struct InspectorTab {
    target: Option<Entity>,
}
pub fn inspector_init_system(
    q_player: Query<Entity, With<PlayerInputKeyboardMarker>>,
    mut q_inspector: Query<&mut InspectorTab>,
) {
    for mut inspector in q_inspector.iter_mut() {
        inspector.target = q_player.get_single().ok();
    }
}

fn get_components_ids(
    world: &World,
    entity: Entity,
) -> Option<impl Iterator<Item = ComponentId> + '_> {
    // components and entities are linked through archetypes
    for archetype in world.archetypes().iter() {
        if archetype.entities().iter().any(|a| a.entity() == entity) {
            return Some(archetype.components());
        }
    }
    None
}

type BuildEcsUi = fn(&dyn Any, &mut Commands) -> Entity;

fn uvec2_ui(data: &dyn Any, commands: &mut Commands) -> Entity {
    let data = data.downcast_ref::<UVec2>().unwrap();
    widgets::TextBundle::spawn(commands, format!("({}, {})", data.x, data.y), ())
}
fn ivec2_ui(data: &dyn Any, commands: &mut Commands) -> Entity {
    let data = data.downcast_ref::<IVec2>().unwrap();
    widgets::TextBundle::spawn(commands, format!("({}, {})", data.x, data.y), ())
}
fn vec2_ui(data: &dyn Any, commands: &mut Commands) -> Entity {
    let data = data.downcast_ref::<Vec2>().unwrap();
    widgets::TextBundle::spawn(commands, format!("({:0.3}, {:0.3})", data.x, data.y), ())
}

fn float_ui<T: std::fmt::Display + 'static>(data: &dyn Any, commands: &mut Commands) -> Entity {
    let data = data.downcast_ref::<T>().unwrap();
    widgets::TextBundle::spawn(commands, format!("{:0.3}", data), ())
}

#[derive(Debug, Clone)]
pub struct EcsUiFor {
    pub fn_readonly: BuildEcsUi,
}

fn add<T: 'static>(type_registry: &mut TypeRegistry, fn_readonly: BuildEcsUi) {
    type_registry
        .get_mut(TypeId::of::<T>())
        .unwrap_or_else(|| panic!("{} not registered", std::any::type_name::<T>()))
        .insert(EcsUiFor { fn_readonly });
}

#[derive(Resource)]
pub struct TypeRegistryResource(pub TypeRegistry);
impl Default for TypeRegistryResource {
    fn default() -> Self {
        let mut type_registry = TypeRegistry::default();
        type_registry.register::<Position>();
        type_registry.register::<Velocity>();
        type_registry.register::<PlayerJumpVelocity>();
        type_registry.register::<UVec2>();
        type_registry.register::<IVec2>();
        type_registry.register::<Vec2>();

        type_registry.register::<f32>();
        type_registry.register::<f64>();

        add::<UVec2>(&mut type_registry, uvec2_ui);
        add::<IVec2>(&mut type_registry, ivec2_ui);
        add::<Vec2>(&mut type_registry, vec2_ui);
        add::<f32>(&mut type_registry, float_ui::<f32>);
        add::<f64>(&mut type_registry, float_ui::<f64>);

        Self(type_registry)
    }
}

fn get_component_info(world: &World, component_id: ComponentId) -> Option<&ComponentInfo> {
    let components = world.components();
    components.get_info(component_id)
}

pub fn inspector_fetch_system(
    mut commands: Commands,
    mut q_inspector: Query<(Entity, &InspectorTab), Changed<InspectorTab>>,
    world: &World,
    type_registry: Res<TypeRegistryResource>,
) {
    let type_registry = &type_registry.0;

    for (entity, inspector) in q_inspector.iter_mut() {
        if let Some(target) = inspector.target {
            let mut children = vec![widgets::TextBundle::spawn(
                &mut commands,
                format!("Entity: {:?}", target),
                (),
            )];

            if let Some(component_ids) = get_components_ids(world, target)
            // .map(|ids| ids.map(|id| get_component_info(&world, id)))
            {
                let component_ids: Vec<ComponentId> = component_ids.collect();
                for component_id in component_ids.into_iter() {
                    let Some(ptr) = world.get_by_id(target, component_id) else {
                        continue;
                    };
                    let Some(info) = get_component_info(world, component_id) else {
                        continue;
                    };

                    let Some(type_id) = info.type_id() else {
                        continue;
                    };

                    let Some(reflect_data) = type_registry.get(type_id) else {
                        continue;
                    };

                    let Some(reflect_from_ptr) = reflect_data.data::<ReflectFromPtr>() else {
                        continue;
                    };
                    // SAFE: `value` is of type `Reflected`, which the `ReflectFromPtr` was created for
                    let value = unsafe { reflect_from_ptr.as_reflect(ptr) };
                    children.push(widgets::text::TextBundle::spawn(
                        &mut commands,
                        value.reflect_short_type_path().into(),
                        (),
                    ));

                    let ReflectRef::Struct(reflected) = value.reflect_ref() else {
                        unreachable!()
                    };

                    for index in 0..reflected.field_len() {
                        let name = reflected.name_at(index).unwrap();
                        let field = reflected.field_at(index).unwrap();

                        let mut field_widgets = Vec::new();

                        field_widgets.push(widgets::text::TextBundle::spawn(
                            &mut commands,
                            format!("+-{} = ", name),
                            (),
                        ));

                        if let Some(ui_for) =
                            type_registry.get_type_data::<EcsUiFor>(field.type_id())
                        {
                            field_widgets.push((ui_for.fn_readonly)(field.as_any(), &mut commands));
                        }

                        children.push(widgets::RowBundle::spawn(&mut commands, field_widgets, ()));
                    }
                }
            }

            commands
                .entity(entity)
                .insert(widgets::column::Column { children });
        } else {
            commands
                .entity(entity)
                .insert(widgets::column::Column { children: vec![] });
        }
    }
}

pub struct InspectorPlugin;
impl Plugin for InspectorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, (inspector_fetch_system, inspector_init_system))
            .init_resource::<TypeRegistryResource>();
    }
}
