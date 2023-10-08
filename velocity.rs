use bevy::{prelude::*};

#[derive(Component)]

pub struct Velocity {
    pub velocity: Vec2,
}

impl Velocity {
    pub fn new() -> Self {
        Self {
            velocity: Vec2::splat(0.),
        }
    }
}

fn main() {
    // This is just here to stop an error :P
}

