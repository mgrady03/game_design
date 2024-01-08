use bevy::prelude::*;

#[derive(Component)]
//a new component with a single field for size, all entities are squares
pub struct Size {
    pub value: Vec2,
}

impl Size {
    pub fn new(size: Vec2) -> Self {
        Self {
            value: size,
        }
    }
}   