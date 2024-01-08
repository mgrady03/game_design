use bevy::prelude::*;
use serde_json::json;

use crate::GameStates;
use crate::WIN_H;
use crate::components::collision::Collidable;
use crate::components::game::Health;
use crate::components::player::FakePlayer;
use crate::components::player::Weapon;
use crate::components::player::WeaponType;
use crate::components::size::Size;
use crate::components::player::Melee;

pub const PLAYER_SPEED: f32 = 1.;

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    let player_handle = asset_server.load("players.png");
    let player_atlas = TextureAtlas::from_grid(player_handle, Vec2::splat(PLAYER_SIZE), 4, 4, None, None);
    let player_atlas_len = player_atlas.textures.len();
    let player_atlas_handle = texture_atlases.add(player_atlas);
    let health_value = 100.0;

    let player_handle = asset_server.load("players.png");
    let player_atlas =
        TextureAtlas::from_grid(player_handle, Vec2::splat(PLAYER_SIZE), 4, 4 , None, None);
    let player_atlas_handle = texture_atlases.add(player_atlas);
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: player_atlas_handle,
            sprite: TextureAtlasSprite {
                index: 0,
                ..default()
            },
            transform: Transform::from_xyz(0., -(WIN_H / 2.) + (TILE_SIZE * SCALE * 2.0), 50.)
                .with_scale(Vec3::splat(SCALE)),
            ..default()
        })
        .insert(AnimationTimer(Timer::from_seconds(
            ANIM_TIME,
            TimerMode::Repeating,
        )))
        .insert(Velocity::new())
        .insert(Player {moving:true, id: 0, cooldown: 0.0, akey:0.0, wkey:0.0, skey:0.0, dkey:0.0})
        .insert(Health::new(health_value))
        .insert(Size::new(Vec2::new(40.,40.)))
        .insert(Collidable)
        .insert(Weapon{id: 0, weapon_type: WeaponType::Sword, damage: 3.0, cooldown: 0.5});
}

pub fn move_player(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut pl: Query<&mut Player,With<Player>>,
    mut player: Query<(&mut Transform, &mut Velocity), (With<Player>, Without<Wall>)>,
) {
    let mut a =0.;
    let mut s=0.;
    let mut d=0.;
    let mut w = 0.;

    for play in player.iter_mut() {
        let (mut pt, mut pv) = play;

        let mut deltav = Vec2::splat(0.);

        if input.pressed(KeyCode::A) {
            deltav.x -= PLAYER_SPEED;
            //player.akey+=1.0;
            a+=1.;
        }

        if input.pressed(KeyCode::D) {
            deltav.x += PLAYER_SPEED;
            //player.dkey+=1.0;
            d+=1.;
        }

        if input.pressed(KeyCode::W) {
            deltav.y += PLAYER_SPEED;
            //player.wkey+=1.0;
            w+=1.;
        }

        if input.pressed(KeyCode::S) {
            deltav.y -= PLAYER_SPEED;
            //player.skey+=1.0;
            s+=1.;
        }

        let deltat = time.delta_seconds();
        let acc = ACCEL_RATE * deltat;

        pv.velocity = if deltav.length() > 0. {
            (pv.velocity + (deltav.normalize_or_zero() * acc)).clamp_length_max(MAX_SPEED)
        } else if pv.velocity.length() > acc {
            pv.velocity + (pv.velocity.normalize_or_zero() * -acc)
        } else {
            Vec2::splat(0.)
        };
        let change = pv.velocity * deltat;

        let new_pos = pt.translation+Vec3::new(change.x, change.y, 0.0);
            for mut p in pl.iter_mut() {
                p.akey +=a;
                p.skey +=s;
                p.wkey +=w;
                p.dkey +=d;
                if p.moving {
                    pt.translation = new_pos;
                }
                else {
                    p.moving = true;
                }
            }
    }
}

pub fn swing_melee(
    input: Res<Input<MouseButton>>,
    mut query: Query<(&Transform, &Velocity, &Weapon, &mut Player), With<Player>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    for (player_pos, _, Weapon, mut player) in query.iter_mut() {
        if input.just_pressed(MouseButton::Left) && player.cooldown <= 0.0 && (Weapon.weapon_type == WeaponType::Sword || Weapon.weapon_type == WeaponType::Axe){
            let melee_ent = Melee {
                damage: Weapon.damage
            };
            let melee_handle = asset_server.load("sword_anim.png");
            let melee_atlas =
                TextureAtlas::from_grid(melee_handle, Vec2::splat(SWORD_SIZE), 9, 1 , None, None);
            let melee_atlas_handle = texture_atlases.add(melee_atlas);
            commands.spawn(SpriteSheetBundle {
                texture_atlas: melee_atlas_handle,
                sprite: TextureAtlasSprite {
                    index: 0,
                    ..default()
                },
                transform: Transform::from_xyz(player_pos.translation.x, player_pos.translation.y, 100.0)
                    .with_scale(Vec3::splat(SCALE)),
                ..default()
            })
            .insert(AnimationTimer(Timer::from_seconds(
                ANIM_TIME,
                TimerMode::Repeating,
            )))
            // .insert(Damage::new(3.));
            .insert(Melee{damage: melee_ent.damage})
            .insert(Collidable)
            .insert(Size::new(Vec2::new(80.,80.)));
            player.cooldown = Weapon.cooldown;
        }
    }
}
//TODO: Need to switch to swing_melee or have a wrapper that calls this when weapon_type == WeaponType::Sword
// pub fn swing_sword(
//     input: Res<Input<MouseButton>>,
//     mut player: Query<(&mut Transform, &mut Velocity, &mut Player), (With<Player>, Without<Wall>)>,
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     mut texture_atlases: ResMut<Assets<TextureAtlas>>,
// ) {
//     for play in player.iter_mut() {
//         let pt = play.0;
//         //Add cooldown check
//         if input.just_pressed(MouseButton::Left) {
//             let sword_handle = asset_server.load("sword_anim.png");
//             let sword_atlas =
//                 TextureAtlas::from_grid(sword_handle, Vec2::splat(SWORD_SIZE), 9, 1 , None, None);
//             let sword_atlas_handle = texture_atlases.add(sword_atlas);
//             commands.spawn(SpriteSheetBundle {
//                 texture_atlas: sword_atlas_handle,
//                 sprite: TextureAtlasSprite {
//                     index: 0,
//                     ..default()
//                 },
//                 transform: Transform::from_xyz(pt.translation.x, pt.translation.y, 100.0)
//                     .with_scale(Vec3::splat(SCALE)),
//                 ..default()
//             })
//             .insert(AnimationTimer(Timer::from_seconds(
//                 ANIM_TIME,
//                 TimerMode::Repeating,
//             )))
//             .insert(Sword)
//             // .insert(Damage::new(3.));
//             .insert(Collidable)
//             .insert(Size::new(Vec2::new(80.,80.)));
//             //insert damage component, set player cooldown to weapon cooldown
//         }
//     }
// }

pub fn animate_melee(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut melee: Query<(&mut TextureAtlasSprite, &Handle<TextureAtlas>, &mut AnimationTimer, &mut Transform, Entity), With<Melee>>,
    mut commands: Commands,
    mut player: Query<&mut Transform, (With<Player>, Without<Wall>, Without<Melee>)>,
) {
    for melee in melee.iter_mut() {
        let (mut sprite, texture_atlas_handle, mut timer, mut transform, melee_entity) = melee;
        for play in player.iter_mut() {
            let pt = play;
            transform.translation = pt.translation;
            timer.tick(time.delta()*8);
            if timer.just_finished() && texture_atlases.get(texture_atlas_handle).is_some(){
                let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
                sprite.index += 1;
                if sprite.index >= texture_atlas.textures.len() {
                    // sword swing is over
                    commands.entity(melee_entity).despawn();
                }
                sprite.index = sprite.index % texture_atlas.textures.len();
            }
        }
    }
}


// pub fn animate_sword(
//     time: Res<Time>,
//     texture_atlases: Res<Assets<TextureAtlas>>,
//     mut swords: Query<
//         (
//             &mut TextureAtlasSprite,
//             &Handle<TextureAtlas>,
//             &mut AnimationTimer,
//             &mut Transform,
//             Entity,
//         ),
//         With<Sword>,
//     >,
//     mut commands: Commands,
//     mut player: Query<&mut Transform, (With<Player>, Without<Wall>, Without<Sword>)>,
// ) {
//     for sword in swords.iter_mut() {
//         let (mut sprite, texture_atlas_handle, mut timer, mut transform, sword_entity) = sword;
//         for play in player.iter_mut() {
//             let pt = play;
//             transform.translation = pt.translation;
//             timer.tick(time.delta()*8);
//             if timer.just_finished() && texture_atlases.get(texture_atlas_handle).is_some(){
//                 let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
//                 sprite.index += 1;
//                 if sprite.index >= texture_atlas.textures.len() {
//                     // sword swing is over
//                     commands.entity(sword_entity).despawn();
//                 }
//                 sprite.index = sprite.index % texture_atlas.textures.len();
//             }
//         }
//     }
// }

pub fn animate_player(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut player: Query<
        (
            &Velocity,
            &mut TextureAtlasSprite,
            &Handle<TextureAtlas>,
            &mut AnimationTimer,
        ),
        With<Player>,
    >,
) {
    for play in player.iter_mut() {
        let (v, mut sprite, texture_atlas_handle, mut timer) = play;
        if v.velocity.cmpne(Vec2::ZERO).any() {
            timer.tick(time.delta());

            if timer.just_finished() {
                let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
                let mut color = 0;
                if sprite.index < 4{
                    color = 0;
                }
                else if sprite.index < 8{
                    color = 4;
                }
                else if sprite.index < 12{
                    color = 8;
                }
                else{
                    color = 12;
                }
                if v.velocity.x.abs() > v.velocity.y.abs() {
                    if v.velocity.x > 0. {
                        sprite.index = color + 1;
                    } else {
                        sprite.index = color + 3;
                    }
                } else {
                    if v.velocity.y > 0. {
                        sprite.index = color + 2;
                    } else {
                        sprite.index =  color + 0;
                    }
                }
                // sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
            }
        }
    }
}

pub fn spawn_fake_players(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    for i in 1..4 {
        let player_handle = asset_server.load("players.png");
    let player_atlas = TextureAtlas::from_grid(player_handle, Vec2::splat(PLAYER_SIZE), 4, 4, None, None);
    let player_atlas_len = player_atlas.textures.len();
    let player_atlas_handle = texture_atlases.add(player_atlas);
    let health_value = 100.0;

    let player_handle = asset_server.load("players.png");
    let player_atlas =
        TextureAtlas::from_grid(player_handle, Vec2::splat(PLAYER_SIZE), 4, 4 , None, Some(Vec2::new(0.,8. * i as f32)));
    let player_atlas_handle = texture_atlases.add(player_atlas);
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: player_atlas_handle,
            sprite: TextureAtlasSprite {
                index: 0,
                ..default()
            },
            transform: Transform::from_xyz(0. + 8. * i as f32, -(WIN_H / 2.) + (TILE_SIZE * SCALE * 2.0), 49.)
                .with_scale(Vec3::splat(SCALE)),
            ..default()
        })
        .insert(AnimationTimer(Timer::from_seconds(
            ANIM_TIME,
            TimerMode::Repeating,
        )))
        .insert(Velocity::new())
        .insert(FakePlayer { id: 0 })
        .insert(Health::new(health_value))
        .insert(Size::new(Vec2::new(40.,40.)))
        .insert(Collidable);
    }
}




///////////////////////////////////////










use bevy::sprite::collide_aabb::collide;
use crate::components::player::{Player, AnimationTimer, Sword, Damage};
use crate::components::room::{Button, Door, Wall};
use crate::components::enemy::Enemy;
use crate::components::game::{GameData,Velocity};

use crate::systems::game::{TILE_SIZE, ACCEL_RATE, ANIM_TIME, SCALE};

pub const MAX_SPEED: f32 = 300.;
pub const PLAYER_SIZE: f32 = 8.;
pub const SWORD_SIZE: f32 = 16.;


// pub fn check_player_collisions (
//     target_pos: Vec3,
//     wall_query: &Query<&Transform, (With<Wall>, Without<Player>, Without<Sword>)>,
//     button_query: &mut Query<(&Transform, &mut Pressed), (With<Button>, Without<Player>, Without<Enemy>)>,
//     door_query: &Query<&Transform, (With<Door>, Without<Player>, Without<Wall>, Without<Enemy>, Without<Sword>)>,
// ) -> bool{
//     return collide_wall(target_pos, wall_query) && player_collide_button(target_pos, button_query) && collide_door(target_pos, door_query, game_state_query);
// }

pub fn player_collide_button(
    target_pos: Vec3,
    button_query: &mut Query<(&Transform), (With<Button>, Without<Player>, Without<Enemy>)>,
) -> bool {
    let moving = true;
    for btn in button_query.iter_mut(){
        let button_transform = btn;
        let collision = collide(
            target_pos, 
            Vec2::splat(TILE_SIZE * 0.9 * SCALE + PLAYER_SIZE),
            button_transform.translation,
            Vec2::splat(1.0)
        );
        if collision.is_some(){
            let mut p = btn;
            // p.pressed = true;
        }
        
    }
    return moving;
}

pub fn switch_player_color(
    input: Res<Input<KeyCode>>,
    mut sprite_bundle: Query< &mut TextureAtlasSprite, With<Player>>,
){
    for mut sprite_b in sprite_bundle.iter_mut() {
        let sprite = sprite_b.as_mut();
        if input.pressed(KeyCode::Right){
            sprite.index = 0;
        }

        if input.pressed(KeyCode::Left){
            sprite.index = 4;
        }

        if input.pressed(KeyCode::Up){
            sprite.index = 8;
        }

        if input.pressed(KeyCode::Down){
            sprite.index = 12;
        }
    }
}

pub fn cooldown_countdown(
    time: Res<Time>,
    mut player: Query<(&mut Player), With<Player>>,
) {
    for (mut p) in player.iter_mut() {
        if p.cooldown > 0.0 {
            p.cooldown -= time.delta_seconds();
        }
        else {
            p.cooldown = 0.0;
        }
    }
}
