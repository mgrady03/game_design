use bevy::prelude::*;

pub const TILE_SIZE: f32 = 16.;
pub const SCALE: f32 = 5.0;
pub const ANIM_TIME: f32 = 0.2;
pub const ACCEL_RATE: f32 = 3600.;

pub fn no_health(
    mut commands: Commands,
    mut entity_query: Query<(Entity,&Health),With<Health>>,
    enemy_query: Query<(Entity,&mut Transform, &mut Velocity), (With<Enemy>, Without<Player>)>,
    mut closed_door_query: Query<Entity, (With<CloseDoor>, Without<Player>)>,
) {
    for entity in entity_query.iter_mut() {
        if entity.1.current <= 0. {
            commands.entity(entity.0).despawn_recursive();
            // despawn closed door if no enemies remain
            if enemy_query.iter().count() <= 1 { // if that was the last enemy
                for entity in closed_door_query.iter_mut() {
                    commands.entity(entity).despawn_recursive();
                }
            }
        }
    }
}

pub fn enemy_dead(
    mut commands: Commands,
    mut enemies: Query<(&Enemy, &Health), With<Enemy>>,
    mut doors: Query<Entity, With<CloseDoor>>,
) {
    for enemy in enemies.iter_mut() {
        if enemy.1.current <= 0. {
            for door in doors.iter_mut() {
                commands.entity(door).despawn_recursive();
            }
        }
    }
}



///////////////////////////////////////






use bevy::sprite::collide_aabb::collide;

use crate::GameStates;
use crate::components::enemy::{Enemy, Fireball, AnimationTimer, self};
use crate::systems::enemy::{boss_spawn, spawn_enemy};

use crate::components::game::{GameData,Health,Velocity};

use crate::components::room::{Wall, Button, Door, Floor, Block, Carpet, Background, CloseDoor};

use crate::systems::room::{spawn_room_contents, clear_room, spawn_empty_contents, clear_floor, STANDARD_ROOM, COMBAT_ROOM, PUZZLE_ROOM, OUTSIDE_SET, DUNGEON_SET, PVP_SET, spawn_room_shell};


use crate::systems::player::PLAYER_SIZE;
use crate::components::player::{Player, Sword, FakePlayer};

pub const WIN_W: f32 = 1280.;
pub const WIN_H: f32 = 720.;

pub fn check_for_room_clear(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    //mut enemy_query: Query<(&mut Transform, &mut Velocity), (With<Enemy>, Without<Player>)>,
    mut game_data: ResMut<GameData>,
    
    floors: Query<Entity,(With<Floor>,Without<Carpet>,Without<Button>)>,
    buttons: Query<Entity,With<Button>,>,
    blocks: Query<Entity,With<Block>,>,
    enemies: Query<Entity,With<Enemy>,>,
    fireballs: Query<Entity,With<Fireball>,>,
    walls: Query<Entity,(With<Wall>,Without<Block>)>,
    carpets: Query<Entity,With<Carpet>,>,
    background: Query<Entity,With<Background>,>,
    mut player: Query<(&mut Transform, &mut Velocity), (With<Player>, Without<Wall>, Without<FakePlayer>)>,
    mut fake_player: Query<(&mut Transform, &mut Velocity), (With<FakePlayer>, Without<Wall>, Without<Player>)>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    if game_data.room_cleared {
        game_data.room_cleared = false;
        // move player to spawn point
        for (mut pt, mut pv) in player.iter_mut() {
            pt.translation = Vec3::new(0., -(WIN_H / 2.) + (TILE_SIZE * SCALE * 2.0), 50.);
            pv.velocity = Vec2::splat(0.);
        }
        for (mut fpt, mut fpv) in fake_player.iter_mut() {
            fpt.translation = Vec3::new(0., -(WIN_H / 2.) + (TILE_SIZE * SCALE * 2.0), 50.);
            fpv.velocity = Vec2::splat(0.);
        }
        // generate new room
        let gamelevel = game_data.game_level;
        
        if gamelevel == 12 {
            clear_floor(&mut commands, &floors, &buttons, &blocks, &enemies, &fireballs);
            spawn_empty_contents(&mut commands, &asset_server, &mut texture_atlases, DUNGEON_SET);
            boss_spawn(commands, asset_server, texture_atlases);
        } else if gamelevel == 0{
            clear_floor(&mut commands, &floors, &buttons, &blocks, &enemies, &fireballs);
            spawn_room_contents(&mut commands, &asset_server, &mut texture_atlases, STANDARD_ROOM, OUTSIDE_SET);
        } else if gamelevel == 4 || gamelevel == 8 {
            clear_floor(&mut commands, &floors, &buttons, &blocks, &enemies, &fireballs);
            spawn_room_contents(&mut commands, &asset_server, &mut texture_atlases, COMBAT_ROOM, DUNGEON_SET);
            spawn_enemy(commands, asset_server, texture_atlases);
        } else if gamelevel == 1 {
            clear_floor(&mut commands, &floors, &buttons, &blocks, &enemies, &fireballs);
            spawn_room_contents(&mut commands, &asset_server, &mut texture_atlases, PUZZLE_ROOM, OUTSIDE_SET);
        } else if gamelevel == 2 {
            clear_room(&mut commands, &floors, &buttons, &blocks, &enemies, &fireballs, &walls, &carpets, &background);
            spawn_room_shell(&mut commands, &asset_server, &mut texture_atlases, DUNGEON_SET);
            spawn_room_contents(&mut commands, &asset_server, &mut texture_atlases, PUZZLE_ROOM, DUNGEON_SET);
        } else if gamelevel == 13 {
            clear_room(&mut commands, &floors, &buttons, &blocks, &enemies, &fireballs, &walls, &carpets, &background);
            spawn_room_shell(&mut commands, &asset_server, &mut texture_atlases, PVP_SET);
            spawn_room_contents(&mut commands, &asset_server, &mut texture_atlases, COMBAT_ROOM, PVP_SET);
        } else if rand::random::<bool>() {  // randomly choose between standard and puzzle rooms for non-fighting levels
            clear_floor(&mut commands, &floors, &buttons, &blocks, &enemies, &fireballs);
            spawn_room_contents(&mut commands, &asset_server, &mut texture_atlases, STANDARD_ROOM, DUNGEON_SET);
        } else {
            clear_floor(&mut commands, &floors, &buttons, &blocks, &enemies, &fireballs);
            spawn_room_contents(&mut commands, &asset_server, &mut texture_atlases, PUZZLE_ROOM, DUNGEON_SET);
        }
        game_data.game_level += 1;
    }
}
