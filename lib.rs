pub use network_channels;

use bevy::prelude::*;

use network_channels::{BitsClient, BitsServer, ServerEvent};

pub mod transport;
pub struct BitsServerPlugin;

pub struct BitsClientPlugin;

impl Plugin for BitsServerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Events<ServerEvent>>();
        app.add_systems(PreUpdate, Self::update_system.run_if(resource_exists::<BitsServer>()));
    }
}

impl BitsServerPlugin {
    pub fn update_system(mut server: ResMut<BitsServer>, time: Res<Time>, mut server_events: EventWriter<ServerEvent>) {
        server.update(time.delta());

        while let Some(event) = server.get_event() {
            server_events.send(event);
        }
    }
}

impl Plugin for BitsClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, Self::update_system.run_if(resource_exists::<BitsClient>()));
    }
}

impl BitsClientPlugin {
    pub fn update_system(mut client: ResMut<BitsClient>, time: Res<Time>) {
        client.update(time.delta());
    }
}
