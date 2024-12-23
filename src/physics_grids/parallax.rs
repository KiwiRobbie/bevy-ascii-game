use bevy::prelude::{Component, Entity, Query, Without};
use spatial_grid::{
    position::{Position, SpatialTraits},
    remainder::Remainder,
};

#[derive(Debug, Component)]
#[require(Position, Remainder)]
pub struct ParallaxLayer {
    pub factor: f32,
    pub target: Entity,
}

pub fn parallax_system(
    mut q_parallax_layer: Query<(&mut Position, &mut Remainder, &ParallaxLayer)>,
    q_target: Query<(&Position, Option<&Remainder>), Without<ParallaxLayer>>,
) {
    for (position, remainder, layer) in &mut q_parallax_layer {
        let Ok((target_position, target_remainder)) = q_target.get(layer.target) else {
            continue;
        };

        (position, remainder).set(
            (target_position.as_vec2() + *target_remainder.cloned().unwrap_or_default())
                * layer.factor,
        );
    }
}
