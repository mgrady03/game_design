use bevy::prelude::*;
use std::fmt::Display;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde_json::ser::Formatter;

#[derive(Serialize, Deserialize)]
pub struct ServerPacket {
    pub client_id: u32,
    //id (999 if enemy), player(T) or not player(F) of all entities
    pub positions: Vec<(u32, bool, Vec3)>,
    pub healths: Vec<(u32, bool, f32)>,
    //if the button is pressed, only one button
    pub button: bool,
}

pub struct InitClientPlugin;

pub struct InitServerPlugin;

#[derive(Serialize, Deserialize)]
pub struct ClientPacket {
    pub client_id: u32,
    pub key: CurrentKey,
    pub mouse_pressed: bool,
}

pub struct CurrentKey {
    pub key: KeyCode,
}

impl Serialize for CurrentKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        return self.serialize(serializer);
    }
}

impl<'de> Deserialize<'de> for CurrentKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de> 
    {
        return CurrentKey::deserialize(deserializer);
    }
}