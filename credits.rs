use bevy::{prelude::*, window::PresentMode};

const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;

#[derive(Component, Deref, DerefMut)]
pub struct PopupTimer(Timer);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Credits".into(),
                resolution: (WIN_W, WIN_H).into(),
                present_mode: PresentMode::Fifo,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, timer)
        .run();
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("davidplaton.png"),
        transform: Transform::from_xyz(0., 0., -0.9).with_scale(Vec3::new(7.2,7.2,0.0)),
        ..default()
    })
    .insert(PopupTimer(Timer::from_seconds(3., TimerMode::Once)));
    commands.spawn(SpriteBundle {
        texture: asset_server.load("JonRiklanUpdatedImage.png"),
        transform: Transform::from_xyz(0., 0., -0.8),
        ..default()
    })
    .insert(PopupTimer(Timer::from_seconds(6., TimerMode::Once)));
    commands.spawn(SpriteBundle {
        texture: asset_server.load("new_mary.png"),
        transform: Transform::from_xyz(0., 0., -0.7),
        ..default()
    })
    .insert(PopupTimer(Timer::from_seconds(9., TimerMode::Once)));
    commands.spawn(SpriteBundle {
        texture: asset_server.load("vansh-pixel-art.png"),
        transform: Transform::from_xyz(0., 0., -0.6),
        ..default()
    })
    .insert(PopupTimer(Timer::from_seconds(12., TimerMode::Once)));
    commands.spawn(SpriteBundle {
        texture: asset_server.load("credits-liberatore.png"),
        transform: Transform::from_xyz(0., 0., -0.5),
        ..default()
    })
    .insert(PopupTimer(Timer::from_seconds(15., TimerMode::Once)));
    commands.spawn(SpriteBundle {
        texture: asset_server.load("Anderis-Credits.png"),
        transform: Transform::from_xyz(0., 0., -0.4),
        ..default()
    })
    .insert(PopupTimer(Timer::from_seconds(18., TimerMode::Once)));
    commands.spawn(SpriteBundle {
        texture: asset_server.load("credits-ColeEichner.png"),
        transform: Transform::from_xyz(0., 0., -0.3),
        ..default()
    })
    .insert(PopupTimer(Timer::from_seconds(21., TimerMode::Once)));
    commands.spawn(SpriteBundle {
        texture: asset_server.load("zid8.png"),
        transform: Transform::from_xyz(0., 0., -0.2),
        ..default()
    })
    .insert(PopupTimer(Timer::from_seconds(24., TimerMode::Once)));
    commands.spawn(SpriteBundle {
        texture: asset_server.load("credits-kopco.png"),
        transform: Transform::from_xyz(0., 0., -0.1),
        ..default()
    })
    .insert(PopupTimer(Timer::from_seconds(27., TimerMode::Once)));
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.2,0.2,0.2),
            custom_size: Some(Vec2::new(1280.,720.)),
            ..default()
        },
        ..default()
    });
    info!("Hello world!");
}

pub fn timer(time: Res<Time>, mut popup: Query<(&mut PopupTimer, &mut Transform)>) {
    for (mut timer, mut transform) in popup.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            transform.translation.z += 1.;
        }
    }
}