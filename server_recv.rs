use std::str::from_utf8;

use bevy::prelude::*;
use bevy::input::keyboard::KeyCode;
use bevy::transform::commands;
use serde::{Deserialize, Serialize};

use crate::components::game::Velocity;
use crate::components::player::Player;
use crate::components::other_player::other_player;

use crate::network::packets::{ClientPacket, ServerPacket, CurrentKey};

use super::client;

use network_plugin::{transport::NetcodeServerPlugin, network_channels::{BitsServer, ConnectionConfig, transport::{NetcodeServerTransport, ServerAuthentication}}, BitsServerPlugin};
use network_channels::{DefaultChannel, ServerEvent};


pub struct ClientPacketList { //what the server combines all client packets into
    pub packets: Vec<ClientPacket>,
}

pub fn recv_input_data(
    ClientPacketList: ClientPacketList,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut Velocity, Option<&mut other_player>)>,
) {
    //iterate via query
    for(entity, mut transform, mut velocity, other_player) in query.iter_mut() {
        //if the entity is an other player
        if let Some(other_player) = other_player {
            //iterate via packets
            for packet in ClientPacketList.packets.iter() {
                //if the packet is for this other player
                if packet.client_id == other_player.id {
                    //move the other player
                    move_entity(packet.key.key, &mut transform, &mut velocity);
                }
            }
        }

    }
}

fn move_entity(
    key_pressed: KeyCode,
    transform: &mut Transform,
    velocity: &mut Velocity,
) {
    //move the entity based on the key pressed
    match key_pressed {
        KeyCode::W => {
            velocity.velocity.y = 1.;
            transform.translation.y += velocity.velocity.y;
        }
        KeyCode::A => {
            velocity.velocity.x = -1.;
            transform.translation.x -= velocity.velocity.x;
        }
        KeyCode::S => {
            velocity.velocity.y = -1.;
            transform.translation.y -= velocity.velocity.y;
        }
        KeyCode::D => {
            velocity.velocity.x = 1.;
            transform.translation.x += velocity.velocity.x;
        }
        _ => {}
    }
}

pub fn receive_message_system(mut server: ResMut<BitsServer>) {
    for client_id in server.clients_id() {
        while let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableOrdered) {
            // Handle received message
            println!("{:?}",from_utf8(&message));
        }
    }
}

pub fn handle_events_system(mut server_events: EventReader<ServerEvent>) {
    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                println!("Client {} connected", client_id);
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                println!("Client {} disconnected: {}", client_id, reason);
            }
        }
    }
}