use std::any::{Any, TypeId};

use ascii_ui::{
    widget_builder::{WidgetBuilder, WidgetBuilderFn},
    widgets,
};
use bevy::{
    app::{Plugin, Update},
    ecs::{
        component::{Component, ComponentId, ComponentInfo},
        entity::Entity,
        query::With,
        schedule::{apply_deferred, IntoSystemConfigs},
        system::{Commands, Query, Res, Resource},
        world::World,
    },
    hierarchy::DespawnRecursiveExt,
    math::{IVec2, UVec2, Vec2},
    reflect::{ReflectFromPtr, ReflectRef, TypeRegistry},
};
use grid_physics::velocity::Velocity;
use spatial_grid::position::Position;

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
    widgets::Text::build(format!("({}, {})", data.x, data.y))(commands)
}
fn ivec2_ui(data: &dyn Any, commands: &mut Commands) -> Entity {
    let data = data.downcast_ref::<IVec2>().unwrap();
    widgets::Text::build(format!("({}, {})", data.x, data.y))(commands)
}
fn vec2_ui(data: &dyn Any, commands: &mut Commands) -> Entity {
    let data = data.downcast_ref::<Vec2>().unwrap();
    widgets::Text::build(format!("({:0.3}, {:0.3})", data.x, data.y))(commands)
}

fn float_ui<T: std::fmt::Display + 'static>(data: &dyn Any, commands: &mut Commands) -> Entity {
    let data = data.downcast_ref::<T>().unwrap();
    widgets::Text::build(format!("{:0.3}", data))(commands)
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
    mut q_inspector: Query<(Entity, &InspectorTab, &widgets::Column)>,
    world: &World,
    type_registry: Res<TypeRegistryResource>,
) {
    let type_registry = &type_registry.0;

    for (inspector_entity, inspector, column) in q_inspector.iter_mut() {
        if let Some(target) = inspector.target {
            let mut inspector_widgets = vec![];
            inspector_widgets.push(widgets::Text::build(format!("Entity: {:?}", target))(
                &mut commands,
            ));

            if let Some(component_ids) = get_components_ids(world, target) {
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
                    inspector_widgets.push(widgets::text::TextBundle::spawn(
                        &mut commands,
                        value.reflect_short_type_path().into(),
                        (),
                    ));

                    match value.reflect_ref() {
                        ReflectRef::Struct(reflected) => {
                            for index in 0..reflected.field_len() {
                                let name = reflected.name_at(index).unwrap();
                                let field = reflected.field_at(index).unwrap();

                                let mut field_widgets = Vec::new();

                                field_widgets
                                    .push(widgets::text::Text::build(format!("+-{} = ", name)));

                                if let Some(ui_for) =
                                    type_registry.get_type_data::<EcsUiFor>(field.type_id())
                                {
                                    field_widgets.push(WidgetBuilderFn::entity((ui_for
                                        .fn_readonly)(
                                        field.as_any(),
                                        &mut commands,
                                    )));
                                }

                                inspector_widgets
                                    .push(widgets::Row::build(field_widgets)(&mut commands));
                            }
                        }
                        ReflectRef::TupleStruct(reflected) => {
                            for field in reflected.iter_fields() {
                                if let Some(ui_for) =
                                    type_registry.get_type_data::<EcsUiFor>(field.type_id())
                                {
                                    inspector_widgets
                                        .push((ui_for.fn_readonly)(field.as_any(), &mut commands));
                                }
                            }
                        }
                        _ => (),
                    }
                }
            }

            for entity in column.children.iter() {
                commands.entity(*entity).despawn_recursive();
            }

            commands
                .entity(inspector_entity)
                .insert(widgets::column::Column {
                    children: inspector_widgets,
                });
        } else {
            commands
                .entity(inspector_entity)
                .insert(widgets::column::Column { children: vec![] });
        }
    }
}

pub struct InspectorPlugin;
impl Plugin for InspectorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (
                inspector_init_system,
                inspector_fetch_system,
                apply_deferred,
            )
                .chain(),
        )
        .init_resource::<TypeRegistryResource>();
    }
}
