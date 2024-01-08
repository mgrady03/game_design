use bevy::{prelude::*, DefaultPlugins, window::{WindowPlugin, Window, PresentMode}};

mod components;
mod plugins;
mod network;
mod resources;
mod systems;

use crate::plugins::menus::MenuPlugin;
use crate::network::packets::{InitClientPlugin, InitServerPlugin};

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameStates {
    #[default]
    MainMenu,
    HostMenu,
    JoinMenu,
    SelectMenu,
    Game,
    GameOver,
    Credits,
}

const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;

fn main() {
    App::new()
        .add_state::<GameStates>()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                    title: "Far From Fatality".into(),
                    resolution: (WIN_W, WIN_H).into(),
                    present_mode: PresentMode::Fifo,
                    ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, camera)
        .add_plugins((
            MenuPlugin,
            InitServerPlugin,
            InitClientPlugin,
        ))
        .run()
}

fn camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}