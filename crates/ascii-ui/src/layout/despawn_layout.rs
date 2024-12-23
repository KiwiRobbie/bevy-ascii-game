use bevy::prelude::*;

use super::widget_layout::WidgetLayout;

pub fn delete_layout_recursive(entity: Entity, commands: &mut Commands, world: &World) {
    if let Some(layout) = world.entity(entity).get::<WidgetLayout>() {
        for child in layout.logic.children(entity, world) {
            delete_layout_recursive(child, commands, world);
        }
    }

    commands.entity(entity).despawn();
}
