use ascii_ui::widgets::{self};
use bevy::{
    ecs::component::{ComponentId, ComponentInfo},
    prelude::*,
    reflect::{ReflectFromPtr, ReflectRef, TypeRegistry},
};
use grid_physics::velocity::Velocity;
use spatial_grid::position::Position;
use std::any::{Any, TypeId};

use crate::player::{
    input::keyboard::PlayerInputKeyboardMarker, movement::jump::PlayerJumpVelocity,
};

#[derive(Debug, Component, Default)]
pub(crate) struct InspectorTab {
    target: Option<Entity>,
}
pub(crate) fn inspector_init_system(
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
        if archetype.entities().iter().any(|a| a.id() == entity) {
            return Some(archetype.components());
        }
    }
    None
}

type BuildEcsUi = fn(&dyn Any, &mut Commands) -> Entity;

fn uvec2_ui(data: &dyn Any, commands: &mut Commands) -> Entity {
    let data = data.downcast_ref::<UVec2>().unwrap();
    widgets::Text::build(format!("({}, {})", data.x, data.y)).build(commands)
}
fn ivec2_ui(data: &dyn Any, commands: &mut Commands) -> Entity {
    let data = data.downcast_ref::<IVec2>().unwrap();
    widgets::Text::build(format!("({}, {})", data.x, data.y)).build(commands)
}
fn vec2_ui(data: &dyn Any, commands: &mut Commands) -> Entity {
    let data = data.downcast_ref::<Vec2>().unwrap();
    widgets::Text::build(format!("({:0.3}, {:0.3})", data.x, data.y)).build(commands)
}

fn float_ui<T: std::fmt::Display + 'static>(data: &dyn Any, commands: &mut Commands) -> Entity {
    let data = data.downcast_ref::<T>().unwrap();
    widgets::Text::build(format!("{:0.3}", data)).build(commands)
}

#[derive(Debug, Clone)]
pub(crate) struct EcsUiFor {
    pub(crate) fn_readonly: BuildEcsUi,
}

fn add<T: 'static>(type_registry: &mut TypeRegistry, fn_readonly: BuildEcsUi) {
    type_registry
        .get_mut(TypeId::of::<T>())
        .unwrap_or_else(|| panic!("{} not registered", std::any::type_name::<T>()))
        .insert(EcsUiFor { fn_readonly });
}

#[derive(Resource)]
pub(crate) struct TypeRegistryResource(pub(crate) TypeRegistry);
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

pub(crate) fn inspector_fetch_system(
    mut commands: Commands,
    mut q_inspector: Query<(Entity, &InspectorTab), With<widgets::MultiChildWidget>>,
    world: &World,
    type_registry: Res<TypeRegistryResource>,
) {
    let type_registry = &type_registry.0;

    for (inspector_entity, inspector) in q_inspector.iter_mut() {
        commands.entity(inspector_entity).despawn_descendants();
        if let Some(target) = inspector.target {
            let mut inspector_widgets = vec![];
            inspector_widgets
                .push(widgets::Text::build(format!("Entity: {:?}", target)).build(&mut commands));

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
                                    field_widgets.push(
                                        (ui_for.fn_readonly)(
                                            field.try_as_reflect().unwrap().as_any(),
                                            &mut commands,
                                        )
                                        .into(),
                                    );
                                }

                                inspector_widgets.push(
                                    widgets::FlexWidget::row(field_widgets).build(&mut commands),
                                );
                            }
                        }
                        ReflectRef::TupleStruct(reflected) => {
                            for field in reflected.iter_fields() {
                                if let Some(ui_for) =
                                    type_registry.get_type_data::<EcsUiFor>(field.type_id())
                                {
                                    inspector_widgets.push((ui_for.fn_readonly)(
                                        field.try_as_reflect().unwrap().as_any(),
                                        &mut commands,
                                    ));
                                }
                            }
                        }
                        _ => (),
                    }
                }
            }

            commands
                .entity(inspector_entity)
                .add_children(&inspector_widgets);
        }
    }
}

pub(crate) struct InspectorPlugin;
impl Plugin for InspectorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (inspector_init_system, inspector_fetch_system).chain(),
        )
        .init_resource::<TypeRegistryResource>();
    }
}
