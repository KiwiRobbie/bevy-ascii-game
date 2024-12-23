use bevy::{
    ecs::component::Component,
    math::{IVec2, UVec2},
};

#[derive(Component, Clone, Debug)]
pub struct Positioned {
    pub offset: IVec2,
    pub size: UVec2,
}
impl Positioned {
    pub(crate) fn contains(&self, position: IVec2) -> bool {
        self.offset.cmple(position).all()
            && position.cmplt(self.offset + self.size.as_ivec2()).all()
    }
}
