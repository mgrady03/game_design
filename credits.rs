use bevy::prelude::*;

use crate::{components::credits::PopupTimer, GameStates};

pub fn credit_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("JonRiklanUpdatedImage.png"),
        transform: Transform::from_xyz(0., 0., -100.8),
        ..default()
    })
    .insert(PopupTimer(Timer::from_seconds(0., TimerMode::Once)));
    commands.spawn(SpriteBundle {
        texture: asset_server.load("new_mary.png"),
        transform: Transform::from_xyz(0., 0., -100.7),
        ..default()
    })
    .insert(PopupTimer(Timer::from_seconds(3., TimerMode::Once)));
    commands.spawn(SpriteBundle {
        texture: asset_server.load("vansh-pixel-art.png"),
        transform: Transform::from_xyz(0., 0., -100.6),
        ..default()
    })
    .insert(PopupTimer(Timer::from_seconds(6., TimerMode::Once)));
    commands.spawn(SpriteBundle {
        texture: asset_server.load("credits-liberatore.png"),
        transform: Transform::from_xyz(0., 0., -100.5),
        ..default()
    })
    .insert(PopupTimer(Timer::from_seconds(9., TimerMode::Once)));
    commands.spawn(SpriteBundle {
        texture: asset_server.load("Anderis-Credits.png"),
        transform: Transform::from_xyz(0., 0., -100.4),
        ..default()
    })
    .insert(PopupTimer(Timer::from_seconds(12., TimerMode::Once)));
    commands.spawn(SpriteBundle {
        texture: asset_server.load("credits-ColeEichner.png"),
        transform: Transform::from_xyz(0., 0., -100.3),
        ..default()
    })
    .insert(PopupTimer(Timer::from_seconds(15., TimerMode::Once)));
    commands.spawn(SpriteBundle {
        texture: asset_server.load("zid8.png"),
        transform: Transform::from_xyz(0., 0., -100.2),
        ..default()
    })
    .insert(PopupTimer(Timer::from_seconds(18., TimerMode::Once)));
    commands.spawn(SpriteBundle {
        texture: asset_server.load("credits-kopco.png"),
        transform: Transform::from_xyz(0., 0., -100.1),
        ..default()
    })
    .insert(PopupTimer(Timer::from_seconds(21., TimerMode::Once)));
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.2,0.2,0.2),
            custom_size: Some(Vec2::new(1280.,720.)),
            ..default()
        },
        transform: Transform::from_xyz(0.,0.,-101.),
        ..default()
    })
    .insert(PopupTimer(Timer::from_seconds(0., TimerMode::Once)));
}

pub fn timer(time: Res<Time>, mut popup: Query<(&mut PopupTimer, &mut Transform)>,mut next_state: ResMut<NextState<GameStates>>) {
    for (mut timer, mut transform) in popup.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            transform.translation.z += 201.;
        }
    }
}