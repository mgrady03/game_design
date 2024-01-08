use std::{net::{SocketAddr, Ipv4Addr, UdpSocket}, time::SystemTime};

use bevy::prelude::*;
use network_channels::transport::ServerConfig;
use network_plugin::{transport::NetcodeServerPlugin, network_channels::{BitsServer, ConnectionConfig, transport::{NetcodeServerTransport, ServerAuthentication}}, BitsServerPlugin};
use serde::ser;

use crate::{network::packets::InitServerPlugin, systems::server_send::send_message_system, systems::server_recv::{receive_message_system, handle_events_system}, GameStates};
use local_ip_address::local_ip;

impl Plugin for InitServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BitsServerPlugin);
        let config = ConnectionConfig::default();
        let server = BitsServer::new(config);
        app.insert_resource(server);
        app.add_plugins(NetcodeServerPlugin);

        let ip = local_ip().unwrap();

        // a list of port numbers to try
        let ports = [":63357", ":63358", ":63359", ":63360"];

        // try to bind to each port in turn, stopping when we succeed
        let socket = ports.iter()
            .map(|port| UdpSocket::bind(format!("{}{}", ip.to_string(), port)))
            .filter_map(Result::ok)
            .next()
            .expect("Failed to bind to any port");
        let server_addr = socket.local_addr().unwrap();
        print!("Server listening on {}", server_addr);
        let server_config = ServerConfig {
            max_clients: 10,
            protocol_id: 0,
            public_addr: server_addr,
            authentication: ServerAuthentication::Unsecure
        };
        let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        let transport = NetcodeServerTransport::new(current_time, server_config, socket).unwrap();
        app.insert_resource(transport);

        app.add_systems(PostUpdate, send_message_system);
        app.add_systems(PreUpdate, receive_message_system);
        app.add_systems(PreUpdate, handle_events_system);
    }
}