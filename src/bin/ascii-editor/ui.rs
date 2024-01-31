use std::ops::RangeInclusive;

use bevy::{
    app::{Plugin, PreUpdate, Update},
    asset::{Asset, Assets},
    ecs::{
        component::Component,
        entity::Entity,
        query::{With, Without},
        schedule::IntoSystemConfigs,
        system::{Commands, Query, ResMut},
        world::World,
    },
    gizmos::gizmos::Gizmos,
    math::{IVec2, UVec2, Vec2},
    render::color::Color,
};
use bevy_ascii_game::{
    glyph_render_plugin::{GlyphSprite, GlyphTexture},
    physics::position::Position,
};

#[derive(Component, Clone, Debug)]
pub struct Positioned {
    pub offset: IVec2,
    pub size: UVec2,
}

#[derive(Component, Clone)]
pub struct Constraint {
    pub width: Option<RangeInclusive<u32>>,
    pub height: Option<RangeInclusive<u32>>,
}

impl Constraint {
    pub fn remove_x_bounds(&self) -> Self {
        Self {
            width: None,
            height: self.height.clone(),
        }
    }

    pub fn remove_y_bounds(&self) -> Self {
        Self {
            width: self.width.clone(),
            height: None,
        }
    }
    pub fn constrain(&self, mut size: UVec2) -> UVec2 {
        if let Some(width) = &self.width {
            size.x = *(width.start().max(&size.x).min(width.end()));
        }
        if let Some(height) = &self.height {
            size.y = *(height.start().max(&size.y).min(height.end()));
        }
        return size;
    }
}

#[derive(Component)]
pub struct Border {
    pub top: Option<char>,
    pub bottom: Option<char>,
    pub left: Option<char>,
    pub right: Option<char>,
}

impl Border {
    pub fn symmetric(horizontal: Option<char>, vertical: Option<char>) -> Self {
        Self {
            top: vertical,
            bottom: vertical,
            left: horizontal,
            right: horizontal,
        }
    }
}

#[derive(Component)]
pub struct Padding {
    pub top: Option<u32>,
    pub bottom: Option<u32>,
    pub left: Option<u32>,
    pub right: Option<u32>,
}

impl Padding {
    pub fn symmetric(horizontal: Option<u32>, vertical: Option<u32>) -> Self {
        Self {
            top: vertical,
            bottom: vertical,
            left: horizontal,
            right: horizontal,
        }
    }
}

#[derive(Component)]
pub struct Fill {
    pub character: char,
}

#[derive(Component)]
pub struct Children {
    pub children: ChildrenLayout,
}

pub enum ChildrenLayout {
    Single(Entity),
    Row(Vec<Entity>),
    Column(Vec<Entity>),
}

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            PreUpdate,
            (clear_layout, build_layout, update_layout_children).chain(),
        )
        .add_systems(Update, (debug_positions, text_render));
    }
}

pub fn clear_layout(
    mut commands: Commands,
    q_positioned: Query<Entity, (With<Positioned>, Without<Root>)>,
    q_constrained: Query<Entity, (With<Constraint>, Without<Root>)>,
    q_children: Query<Entity, With<LayoutChildren>>,
) {
    for entity in q_positioned.iter() {
        commands.entity(entity).remove::<Positioned>();
    }
    for entity in q_constrained.iter() {
        commands.entity(entity).remove::<Constraint>();
    }
    for entity in q_children.iter() {
        commands.entity(entity).remove::<LayoutChildren>();
    }
}

#[derive(Debug, Component)]
pub struct Widget {
    pub logic: Box<dyn WidgetLogic>,
}

impl Widget {
    pub fn new<T: WidgetLogic + Default + 'static>() -> Self {
        Self {
            logic: Box::new(T::default()),
        }
    }
}

pub trait WidgetLogic: std::fmt::Debug + Send + Sync {
    fn layout(
        &self,
        entity: Entity,
        constraint: &Constraint,
        layout_resources: &LayoutResources,
        commands: &mut Commands,
    ) -> UVec2;

    fn children(&self, entity: Entity, world: &World) -> Vec<Entity>;
}

#[derive(Component)]
pub struct Root {
    pub child: Entity,
}

#[derive(Debug, Default)]
pub struct RootLogic;
impl WidgetLogic for RootLogic {
    fn layout(
        &self,
        entity: Entity,
        _constraint: &Constraint,
        layout_resources: &LayoutResources,
        commands: &mut Commands,
    ) -> UVec2 {
        let position = layout_resources
            .world
            .get::<Positioned>(entity)
            .expect("Root not positioned!");

        let root_constraint = Constraint {
            width: Some(0..=position.size.x),
            height: Some(0..=position.size.y),
        };

        let root = layout_resources
            .world
            .get::<Root>(entity)
            .expect("Root logic without Root!");

        let child = layout_resources
            .world
            .get::<Widget>(root.child)
            .expect("Root child missing Widget!");

        let size = (child.logic).layout(root.child, &root_constraint, layout_resources, commands);
        // dbg!(root.child);

        commands.entity(root.child).insert(Positioned {
            offset: IVec2::ZERO,
            size: root_constraint.constrain(size),
        });

        return position.size;
    }
    fn children(&self, entity: Entity, world: &World) -> Vec<Entity> {
        let root = world.get::<Root>(entity).expect("Root logic without Root!");
        vec![root.child]
    }
}

#[derive(Debug, Component)]
pub struct Row {
    pub children: Vec<Entity>,
}
#[derive(Debug, Default)]
pub struct RowLogic;
impl WidgetLogic for RowLogic {
    fn layout(
        &self,
        entity: Entity,
        constraint: &Constraint,
        layout_resources: &LayoutResources,
        commands: &mut Commands,
    ) -> UVec2 {
        let row = layout_resources
            .world
            .get::<Row>(entity)
            .expect("Row Widget Logic missing Row Component!");

        let child_constraint = constraint.remove_y_bounds();

        let mut cursor_y: u32 = 0;
        let mut width: u32 = 0;

        for child in row.children.iter() {
            let child_logic = layout_resources
                .world
                .get::<Widget>(*child)
                .expect("Failed to get widget logic for child");

            let size = constraint.constrain((child_logic.logic).layout(
                *child,
                &child_constraint,
                layout_resources,
                commands,
            ));
            // dbg!(child);
            commands.entity(*child).insert(Positioned {
                offset: IVec2 {
                    x: 0,
                    y: cursor_y as i32,
                },
                size,
            });

            width = width.max(size.x);
            cursor_y += size.y;
        }
        return UVec2 {
            x: width,
            y: cursor_y,
        };
    }

    fn children(&self, entity: Entity, world: &World) -> Vec<Entity> {
        world
            .get::<Row>(entity)
            .expect("Row logic without Row!")
            .children
            .clone()
    }
}

#[derive(Debug, Component)]
pub struct Text {
    pub text: String,
}
#[derive(Debug, Default)]

pub struct TextLogic;
impl WidgetLogic for TextLogic {
    fn layout(
        &self,
        entity: Entity,
        _constraint: &Constraint,
        layout_resources: &LayoutResources,
        _commands: &mut Commands,
    ) -> UVec2 {
        let text = layout_resources
            .world
            .get::<Text>(entity)
            .expect("Text Widget Logic missing Text Component!");

        return UVec2 {
            x: text.text.len() as u32,
            y: 1,
        };
    }

    fn children(&self, _entity: Entity, _world: &World) -> Vec<Entity> {
        vec![]
    }
}

pub struct LayoutResources<'world> {
    world: &'world World,
}

// Get constraint from parent
// Tell children constraints
// Get child size
// Place children
// Give size to parent

pub fn build_layout(
    mut commands: Commands,
    q_root: Query<(Entity, &Widget), With<Root>>,
    world: &World,
) {
    for (entity, widget) in q_root.iter() {
        // dbg!(widget);
        (widget.logic).layout(
            entity,
            &Constraint {
                width: None,
                height: None,
            },
            &LayoutResources { world },
            &mut commands,
        );
    }
}

pub fn update_layout_children(
    mut commands: Commands,
    q_root: Query<Entity, With<Root>>,
    world: &World,
) {
    // let mut stack: Vec<Entity> = q_root.iter().collect();
    for root_entity in q_root.iter() {
        recurse_apply_position(&mut commands, IVec2::ZERO, world, root_entity);
    }
}

pub fn recurse_apply_position(
    commands: &mut Commands,
    parent_offset: IVec2,
    world: &World,
    entity: Entity,
) {
    let Some(widget) = world.get::<Widget>(entity) else {
        return;
    };
    let children = (widget.logic).children(entity, world);
    let Some(position) = world.get::<Positioned>(entity) else {
        return;
    };
    let new_offset = position.offset + parent_offset;
    commands.entity(entity).insert(Positioned {
        offset: new_offset,
        size: position.size,
    });

    for child in children.iter() {
        recurse_apply_position(commands, new_offset, world, *child);
    }
}

#[derive(Debug, Component)]
pub struct LayoutChildren {
    pub children: Vec<Entity>,
}
pub fn debug_positions(mut gizmos: Gizmos, q_positioned: Query<&Positioned>) {
    for positioned in q_positioned.iter() {
        let offset = positioned.offset.as_vec2() * Vec2::new(19.0, 40.0);
        let size = positioned.size.as_vec2() * Vec2::new(19.0, 40.0);
        let center = offset + 0.5 * size;

        gizmos.rect_2d(center, 0.0, size, Color::ORANGE);
    }
}
pub fn text_render(
    mut glyph_textures: ResMut<Assets<GlyphTexture>>,
    mut commands: Commands,
    q_text: Query<(Entity, &Positioned, &Text)>,
) {
    for (entity, positioned, text) in q_text.iter() {
        commands
            .entity(entity)
            .insert((
                Position {
                    position: positioned.offset,
                    remainder: Vec2::ZERO,
                },
                GlyphSprite {
                    texture: glyph_textures.add(GlyphTexture {
                        data: vec![text.text.clone()],
                    }),
                    offset: IVec2::ZERO,
                },
            ))
            .log_components();
    }
}
