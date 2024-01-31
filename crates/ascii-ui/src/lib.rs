use std::ops::RangeInclusive;

use bevy::{
    app::{Plugin, PreUpdate, Update},
    asset::Assets,
    ecs::{
        bundle::Bundle,
        component::Component,
        entity::Entity,
        query::{With, Without},
        schedule::{apply_deferred, IntoSystemConfigs},
        system::{Commands, Query, Res, ResMut},
        world::World,
    },
    gizmos::gizmos::Gizmos,
    math::{IVec2, UVec2, Vec2},
    render::color::Color,
};

use glyph_render::glyph_render_plugin::{GlyphSprite, GlyphTexture};
use grid_physics::position::{GridSize, Position};

#[derive(Component, Clone, Debug)]
pub struct Positioned {
    pub offset: IVec2,
    pub size: UVec2,
}

#[derive(Clone)]
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

    pub fn max(&self) -> UVec2 {
        let x = if let Some(x) = &self.width {
            *x.end()
        } else {
            0
        };

        let y = if let Some(y) = &self.height {
            *y.end()
        } else {
            0
        };

        UVec2 { x, y }
    }
}

#[derive(Debug, Bundle)]
pub struct BorderBundle {
    pub border: Border,
    pub padding: Padding,
}
impl BorderBundle {
    pub fn new(border: Border) -> Self {
        Self {
            border: border.clone(),
            padding: Padding(EdgeInsets {
                top: border.top.is_some() as u32,
                bottom: border.bottom.is_some() as u32,
                left: border.left.is_some() as u32,
                right: border.right.is_some() as u32,
            }),
        }
    }
}

#[derive(Debug, Component, Clone)]
pub struct Border {
    pub top: Option<char>,
    pub bottom: Option<char>,
    pub left: Option<char>,
    pub right: Option<char>,

    pub corners: Option<[char; 4]>,
}

impl Border {
    pub fn symmetric(
        horizontal: Option<char>,
        vertical: Option<char>,
        corners: Option<[char; 4]>,
    ) -> Self {
        Self {
            top: vertical,
            bottom: vertical,
            left: horizontal,
            right: horizontal,
            corners: if vertical.is_some() && horizontal.is_some() {
                corners
            } else {
                None
            },
        }
    }
    fn sides(pos: UVec2, size: UVec2) -> (bool, bool, bool, bool) {
        let l = pos.x == 0;
        let r = pos.x == size.x - 1;
        let t = pos.y == 0;
        let b = pos.y == size.y - 1;
        return (l, r, t, b);
    }

    pub fn create_data(&self, size: UVec2) -> Vec<String> {
        (0..size.y)
            .map(|y| {
                (0..size.x)
                    .map(|x| {
                        (match Self::sides(UVec2 { x, y }, size) {
                            (true, false, true, false) => self.corners.map(|c| c[0]).or(self.left),
                            (false, true, true, false) => self.corners.map(|c| c[1]).or(self.right),
                            (false, true, false, true) => self.corners.map(|c| c[2]).or(self.left),
                            (true, false, false, true) => self.corners.map(|c| c[3]).or(self.right),
                            (true, false, false, false) => self.left,
                            (false, true, false, false) => self.right,
                            (false, false, true, false) => self.top,
                            (false, false, false, true) => self.bottom,
                            _ => None,
                        })
                        .unwrap_or(' ')
                    })
                    .collect()
            })
            .collect()
    }
}

#[derive(Debug, Component, Default, Clone)]
pub struct Padding(pub EdgeInsets);

#[derive(Debug, Default, Clone)]
pub struct EdgeInsets {
    pub top: u32,
    pub bottom: u32,
    pub left: u32,
    pub right: u32,
}

impl EdgeInsets {
    pub fn shrink_constraint(&self, constraint: &Constraint) -> Constraint {
        // TODO: Handle to small

        Constraint {
            width: if let Some(width) = &constraint.width {
                let end = width.end() - self.top - self.bottom;
                Some(*width.start()..=end)
            } else {
                None
            },
            height: if let Some(height) = &constraint.height {
                let end = height.end() - self.left - self.right;
                Some(*height.start()..=end)
            } else {
                None
            },
        }
    }
}

impl EdgeInsets {
    pub fn symmetric(horizontal: u32, vertical: u32) -> Self {
        Self {
            top: vertical,
            bottom: vertical,
            left: horizontal,
            right: horizontal,
        }
    }
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
            (
                clear_layout,
                apply_deferred,
                build_layout,
                apply_deferred,
                update_layout_children,
            )
                .chain(),
        )
        .add_systems(Update, (text_render, border_render, divider_render));
    }
}

pub fn clear_layout(
    mut commands: Commands,
    q_positioned: Query<Entity, (With<Positioned>, Without<Root>)>,
) {
    for entity in q_positioned.iter() {
        commands.entity(entity).remove::<Positioned>();
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
        world: &World,
        commands: &mut Commands,
    ) -> UVec2;

    fn children(&self, entity: Entity, world: &World) -> Vec<Entity>;
}

#[derive(Component)]
pub struct Root;

#[derive(Debug, Component)]
pub struct Container {
    pub child: Entity,
}

#[derive(Debug, Default)]
pub struct ContainerLogic;
impl WidgetLogic for ContainerLogic {
    fn layout(
        &self,
        entity: Entity,
        constraint: &Constraint,
        world: &World,
        commands: &mut Commands,
    ) -> UVec2 {
        let container = world
            .get::<Container>(entity)
            .expect("Root logic without Root!");

        let padding = world
            .get::<Padding>(entity)
            .map(|p| p.clone())
            .unwrap_or_default();

        let constraint = padding.0.shrink_constraint(constraint);

        let child_widget = world
            .get::<Widget>(container.child)
            .expect("Root child missing Widget!");

        let size = (child_widget.logic).layout(container.child, &constraint, world, commands);

        let offset = IVec2 {
            x: padding.0.left as i32,
            y: padding.0.top as i32,
        };

        commands.entity(container.child).insert(Positioned {
            offset,
            size: constraint.constrain(size),
        });

        return constraint.max();
    }
    fn children(&self, entity: Entity, world: &World) -> Vec<Entity> {
        let container = world
            .get::<Container>(entity)
            .expect("Root logic without Root!");
        vec![container.child]
    }
}

#[derive(Debug, Component)]
pub struct Column {
    pub children: Vec<Entity>,
}
#[derive(Debug, Default)]
pub struct ColumnLogic;
impl WidgetLogic for ColumnLogic {
    fn layout(
        &self,
        entity: Entity,
        constraint: &Constraint,
        world: &World,
        commands: &mut Commands,
    ) -> UVec2 {
        let row = world
            .get::<Column>(entity)
            .expect("Row Widget Logic missing Row Component!");

        let child_constraint = constraint.remove_y_bounds();

        let mut cursor_y: u32 = 0;
        let mut width: u32 = 0;

        for child in row.children.iter() {
            let child_logic = world
                .get::<Widget>(*child)
                .expect("Failed to get widget logic for child");

            let size = constraint.constrain((child_logic.logic).layout(
                *child,
                &child_constraint,
                world,
                commands,
            ));

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
            .get::<Column>(entity)
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
        world: &World,
        _commands: &mut Commands,
    ) -> UVec2 {
        let text = world
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

#[derive(Debug, Component)]
pub struct Divider {
    pub character: char,
}

#[derive(Debug, Default)]
pub struct DividerLogic;
impl WidgetLogic for DividerLogic {
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

// Get constraint from parent
// Tell children constraints
// Get child size
// Place children
// Give size to parent

pub fn build_layout(
    mut commands: Commands,
    q_root: Query<(Entity, &Widget, &Positioned), (With<Root>, With<Container>)>,
    world: &World,
) {
    for (entity, widget, positioned) in q_root.iter() {
        (widget.logic).layout(
            entity,
            &Constraint {
                width: Some(0..=positioned.size.x),
                height: Some(0..=positioned.size.y),
            },
            world,
            &mut commands,
        );
    }
}

pub fn update_layout_children(
    mut commands: Commands,
    q_root: Query<Entity, With<Root>>,
    world: &World,
) {
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
    commands.entity(entity).log_components();
    let Some(widget) = world.get::<Widget>(entity) else {
        println!("no widget");
        return;
    };
    let Some(position) = world.get::<Positioned>(entity) else {
        println!("no position");
        return;
    };
    let children = (widget.logic).children(entity, world);

    let new_offset = position.offset + parent_offset;
    commands.entity(entity).insert(Positioned {
        offset: new_offset,
        size: position.size,
    });

    for child in children.iter() {
        recurse_apply_position(commands, new_offset, world, *child);
    }
}

pub fn debug_positions(
    mut gizmos: Gizmos,
    q_positioned: Query<&Positioned>,
    grid_size: Res<GridSize>,
) {
    for positioned in q_positioned.iter() {
        let offset = positioned.offset.as_vec2() * grid_size.as_vec2() * Vec2::new(1.0, -1.0);
        let size = positioned.size.as_vec2() * grid_size.as_vec2() * Vec2::new(1.0, -1.0);
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
        dbg!(entity, positioned);
        commands.entity(entity).insert((
            Position {
                position: positioned.offset * IVec2::new(1, -1)
                    - IVec2::Y * positioned.size.y as i32,
                remainder: Vec2::ZERO,
            },
            GlyphSprite {
                texture: glyph_textures.add(GlyphTexture {
                    data: vec![text.text.clone()],
                }),
                offset: IVec2::ZERO,
            },
        ));
    }
}
pub fn divider_render(
    mut glyph_textures: ResMut<Assets<GlyphTexture>>,
    mut commands: Commands,
    q_text: Query<(Entity, &Positioned, &Divider)>,
) {
    for (entity, positioned, divider) in q_text.iter() {
        dbg!(entity, positioned);
        commands.entity(entity).insert((
            Position {
                position: positioned.offset * IVec2::new(1, -1)
                    - IVec2::Y * positioned.size.y as i32,
                remainder: Vec2::ZERO,
            },
            GlyphSprite {
                texture: glyph_textures.add(GlyphTexture {
                    data: vec![divider
                        .character
                        .to_string()
                        .repeat(positioned.size.x as usize)],
                }),
                offset: IVec2::ZERO,
            },
        ));
    }
}
pub fn border_render(
    mut glyph_textures: ResMut<Assets<GlyphTexture>>,
    mut commands: Commands,
    q_text: Query<(Entity, &Positioned, &Border)>,
) {
    for (entity, positioned, border) in q_text.iter() {
        let data = border.create_data(positioned.size);
        let position = positioned.offset * IVec2::new(1, -1) - IVec2::Y * positioned.size.y as i32;
        commands.entity(entity).insert((
            Position {
                position,
                remainder: Vec2::ZERO,
            },
            GlyphSprite {
                texture: glyph_textures.add(GlyphTexture { data }),
                offset: IVec2::ZERO,
            },
        ));
    }
}

// Widget -> Widget Logic
// - Row/Column
// - Container
// - Grid?
