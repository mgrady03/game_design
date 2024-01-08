use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::components::game::{Health, Velocity, GameData};
use crate::components::enemy::{Enemy, Fireball};
use crate::components::player::Player;
use crate::components::other_player::other_player;
use crate::components::room::{Button, Door, Carpet, Wall, CloseDoor};

use crate::network::packets::{ClientPacket, ServerPacket, CurrentKey};

use network_plugin::{transport::NetcodeServerPlugin, network_channels::{BitsServer, ConnectionConfig, transport::{NetcodeServerTransport, ServerAuthentication}}, BitsServerPlugin};
use network_channels::DefaultChannel;


pub fn send_game_data(
    query: Query<(Entity, Option<&Velocity>, &Transform, Option<&Button>, Option<&Door>, Option<&Player>, Option<&other_player>, Option<&Enemy>, Option<&Health>)>,
    client_id: u32,
) {
    //create a new packet
    let mut packet = ServerPacket {
        client_id,
        positions: Vec::new(),
        healths: Vec::new(),
        button: false,
    };
    //add all positions to the packet
    for (entity, velocity, transform, button, door, player, other_player, enemy, health) in query.iter() {
        //add position of player to packet
        if let Some(player) = player {
            packet.positions.push((player.id, true, transform.translation));
            //add health of player to packet
            if let Some(health) = health {
                packet.healths.push((player.id, true, health.current));
            }
        }
        //add position of other player to packet
        if let Some(other_player) = other_player {
            packet.positions.push((other_player.id, true, transform.translation));
            //add health of other player to packet
            if let Some(health) = health {
                packet.healths.push((other_player.id, true, health.current));
            }
        }
        //add position of enemy to packet
        if let Some(enemy) = enemy {
            packet.positions.push((999, false, transform.translation));
            //add health of enemy to packet
            if let Some(health) = health {
                packet.healths.push((999, false, health.current));
            }
        }
        //add the state of the button to the packet
        if let Some(button) = button {
            packet.button = button.pressed;
        }
    }
    //send the packet
    // send_packet(packet);
}

pub(crate) fn send_message_system(mut server: ResMut<BitsServer>) {
    let channel_id = 0;
    // Send a text message for all clients
    // The enum DefaultChannel describe the channels used by the default configuration
    server.broadcast_message(DefaultChannel::ReliableOrdered, "server message");
}