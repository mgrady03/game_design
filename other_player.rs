use bevy::prelude::*;

use super::game::Health;

#[derive(Component)]
pub struct other_player {
    pub moving: bool,
    pub id: u32,
    pub cooldown: f32,
}