use bevy::{color::palettes::css, ecs::entity::EntityHashMap, prelude::*};

use ascii_ui::{
    attachments::{self, Flex, Padding, SizedBox},
    col,
    mouse::{ExternalStateMarker, InteractableMarker, TriggeredMarker},
    row, sized_box, text,
    widget_builder::{WidgetBuilder, WidgetBuilderFn, WidgetSaver},
    widgets::{self, checkbox::CheckboxEnabledMarker, Checkbox},
};
use glyph_render::glyph_render_plugin::GlyphSolidColor;

#[derive(Component)]
pub(super) struct IndirectListBuilder {
    target: Entity,
    src_to_dst: EntityHashMap<Entity>,
}
impl IndirectListBuilder {
    fn new(target: Entity) -> Self {
        Self {
            target,
            src_to_dst: Default::default(),
        }
    }
}
#[derive(Component, Deref)]
pub(super) struct BuilderFn(pub Box<dyn Fn(Entity, &World) -> WidgetBuilderFn + Send + Sync>);

pub(super) fn update_indirect_list_builder(
    mut commands: Commands,
    q_list: Query<(Entity, &IndirectListBuilder, &BuilderFn)>,
    q_target: Query<&Children>,
    world: &World,
) {
    for (widget_entity, list_widget, builder) in &q_list {
        let src_entities = q_target
            .get(list_widget.target)
            .map(|children| &**children)
            .unwrap_or(&[]);
        let dst_entities = q_target
            .get(widget_entity)
            .map(|children| &**children)
            .unwrap_or(&[]);

        let mut dst_desired_entities = vec![];
        let mut updated_src_to_dst = EntityHashMap::default();
        for &src in src_entities.iter() {
            match list_widget.src_to_dst.get(&src) {
                Some(&dst) => {
                    dst_desired_entities.push(dst);
                    updated_src_to_dst.insert(src, dst);
                }
                None => {
                    let dst = (builder)(src, world)(&mut commands);
                    dst_desired_entities.push(dst);
                    updated_src_to_dst.insert(src, dst);
                }
            }
        }

        for dst in dst_entities {
            if !dst_desired_entities.contains(dst) {
                commands.entity(*dst).despawn_recursive();
            }
        }
        commands
            .entity(widget_entity)
            .clear_children()
            .add_children(&dst_desired_entities)
            .insert(IndirectListBuilder {
                target: list_widget.target,
                src_to_dst: updated_src_to_dst,
            });
    }
}

use super::{EditorLayer, EditorLayers, SelectedEditorLayer};

#[derive(Component)]
pub struct LayersWidget {
    layer_list: Entity,
}

#[derive(Debug, Component)]
pub struct LayerEntryWidget {
    layer_entity: Entity,
    name_widget: Entity,
    visible_checkbox_widget: Entity,
}

impl LayerEntryWidget {
    fn build(entity: Entity, world: &World) -> WidgetBuilderFn {
        Box::new(move |commands: &mut Commands| {
            let mut name_widget = Entity::PLACEHOLDER;
            let mut visible_checkbox_widget = Entity::PLACEHOLDER;
            let layer = world.get::<EditorLayer>(entity).unwrap();
            row![
                sized_box![horizontal: 1],
                widgets::Checkbox::build()
                    .with((CheckboxEnabledMarker, ExternalStateMarker))
                    .save_id(&mut visible_checkbox_widget)
                    .apply(commands),
                sized_box![horizontal: 1],
                text!(&layer.name)
                    .with(InteractableMarker)
                    .save_id(&mut name_widget)
                    .apply(commands),
            ]
            .with(LayerEntryWidget {
                layer_entity: entity,
                visible_checkbox_widget,
                name_widget,
            })(commands)
        })
    }
}
pub fn update_layer_entry_widget(
    q_layers_widget: Query<&LayerEntryWidget>,
    q_name_widget: Query<(Entity, Has<TriggeredMarker>)>,
    mut q_layers: Query<(&mut EditorLayer, Has<SelectedEditorLayer>)>,
    q_checkbox: Query<(Has<TriggeredMarker>, Has<CheckboxEnabledMarker>), With<Checkbox>>,
    mut commands: Commands,
    q_selected: Query<Entity, (With<EditorLayer>, With<SelectedEditorLayer>)>,
) {
    for widget in &q_layers_widget {
        let (mut layer, layer_selected) = q_layers.get_mut(widget.layer_entity).unwrap();
        if let Ok((triggered, enabled)) = q_checkbox.get(widget.visible_checkbox_widget) {
            if triggered {
                layer.visible = !layer.visible;
                Checkbox::toggle(&mut commands, enabled, widget.visible_checkbox_widget)
            }
        }

        let (name_widget, select_triggered) = q_name_widget.get(widget.name_widget).unwrap();
        if select_triggered {
            for entity in &q_selected {
                commands.entity(entity).remove::<SelectedEditorLayer>();
            }

            commands
                .entity(widget.layer_entity)
                .insert(SelectedEditorLayer);
        }
        commands.entity(name_widget).insert(GlyphSolidColor {
            color: [css::GRAY, css::RED][layer_selected as usize].into(),
        });
    }
}
pub(super) fn init_layer_list_ui(
    mut commands: Commands,
    q_layers_widget: Query<&LayersWidget, Added<LayersWidget>>,
    q_layers: Query<Entity, With<EditorLayers>>,
) {
    let target = q_layers.get_single().unwrap();
    for widget in &q_layers_widget {
        commands.entity(widget.layer_list).insert((
            IndirectListBuilder::new(target),
            BuilderFn(Box::new(LayerEntryWidget::build)),
        ));
    }
}

impl LayersWidget {
    pub fn build<'a>() -> WidgetBuilderFn<'a> {
        Box::new(|commands: &mut Commands| {
            let mut layer_list = Entity::PLACEHOLDER;
            col![
                row![
                    widgets::Divider::build('=').with(Flex::new(1)),
                    text!(" Layers "),
                    widgets::Divider::build('=').with(Flex::new(1)),
                ],
                sized_box!(vertical: 1),
                col![].save_id(&mut layer_list).apply(commands),
                widgets::SingleChildWidget::build(None).with(SizedBox::vertical(1)),
                row![
                    widgets::SingleChildWidget::build(Some(text!("New Layer")))
                        .with(Flex::new(1))
                        .with(Padding::symmetric(1, 0)),
                    widgets::Button::build("Create"),
                ],
                sized_box!(vertical: 2),
                text!("Selected Layer"),
                sized_box!(vertical: 1),
                text!("Size: 64 x 32"),
                text!("Name: background"),
            ]
            .with(LayersWidget { layer_list })(commands)
        })
    }
}
