use bevy::prelude::*;
use network_plugin::network_channels::{DefaultChannel, BitsClient};
use serde_json::json;

use crate::components::{player::Player, game::Health, self, other_player::other_player, enemy::Enemy, room::{Button, Door, Carpet, Wall, CloseDoor}};
use crate::network::packets::{ClientPacket, ServerPacket, CurrentKey};

pub fn client_send(
    input: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    mut client: ResMut<BitsClient>,
){
    let mut mouse_pressed = false;
    let mut key: String = "".to_string();

    if get_mouse(mouse) == true {
        mouse_pressed = true;
    }

    if input.pressed(KeyCode::W) {
        key = "W".to_string();
    }
    else if input.pressed(KeyCode::A) {
        key = "A".to_string();
    }
    else if input.pressed(KeyCode::S) {
        key = "S".to_string();
    }
    else if input.pressed(KeyCode::D) {
        key = "D".to_string();
    }

    let information = json!({
        "client_id": 0,
        "key": key,
        "mouse_pressed": mouse_pressed,
    });

    send_message_system(client, information.to_string());
}

fn get_pressed_key(
    input: Res<Input<KeyCode>>,
) -> KeyCode{

    for key in input.get_pressed() {
        return key.clone();
    }

    return KeyCode::Space;
}

fn get_mouse(
    input: Res<Input<MouseButton>>,
) -> bool{
    if input.just_pressed(MouseButton::Left) {
        return true;
    }

    return false;
}

fn client_receive(
    incoming: ServerPacket,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, Option<&mut Button>, Option<&other_player>, Option<&Enemy>, Option<&mut Health>)>,
) {
    // Iterate through all entities
    for (entity, mut transform, mut button, other_player, enemy, mut health) in query.iter_mut() {
        // If the entity is a player
        if let Some(other_player) = other_player {
            // Iterate through all positions in the packet
            for position in incoming.positions.iter() {
                // If the position is for this player
                if position.0 == other_player.id {
                    // Move the player
                    transform.translation = position.2;
                }
            }
        }
        // If the entity is an enemy
        if let Some(enemy) = enemy {
            // Iterate through all positions in the packet
            for position in incoming.positions.iter() {
                // If the position is for this enemy
                if position.0 == 999 {
                    // Move the enemy
                    transform.translation = position.2;
                }
            }
        }
        // if the entity has health
        if let Some(mut health) = health {
            // check if the entity is a player or enemy
            if let Some(other_player) = other_player {
                // Iterate through all healths in the packet
                for healths in incoming.healths.iter() {
                    // If the health is for this entity
                    if healths.0 == other_player.id {
                        // Set the health
                        health.current = healths.2;
                    }
                }
            }
            // check if the entity is a player or enemy
            if let Some(enemy) = enemy {
                // Iterate through all healths in the packet
                for healths in incoming.healths.iter() {
                    // If the health is for this entity
                    if healths.0 == 999 {
                        // Set the health
                        health.current = healths.2;
                    }
                }
            }
        }
        // If the entity is a button
        if let Some(mut button) = button {
            // Set the state of the button
            button.pressed = incoming.button;
        }
    }
}

// pub fn send_message_system(mut client: ResMut<BitsClient>,input: Res<Input<KeyCode>>,mouse: Res<Input<MouseButton>>) {
//     // Send a text message to the server
//     client.send_message(DefaultChannel::ReliableOrdered, client_send(input,mouse));
// }

pub fn send_message_system(mut client: ResMut<BitsClient>, message: String) {
        // Send a text message to the server
        client.send_message(DefaultChannel::ReliableOrdered, message);
    }

pub fn receive_message_system(mut client: ResMut<BitsClient>) {
    while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
        // Handle received message
    }
}