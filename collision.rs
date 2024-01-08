use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};
use crate::GameStates;
use crate::components::collision::{Collidable};
use crate::components::player::{Player, Melee, FakePlayer};
use crate::systems::player::PLAYER_SPEED;
use crate::components::game::{Health, Velocity, GameData};
use crate::components::enemy::{Enemy, Fireball};
use crate::components::room::{Button, Door, Carpet, Wall, CloseDoor};
use crate::components::projectile::{Projectile};
use crate::components::size::{Size};

use super::enemy;


//constant for player speed
pub const CHANGE_PLAYER_SPEED: f32 = PLAYER_SPEED;

pub const CHANGE_ENEMY_SPEED: f32 = 2.;


pub fn collision(
    mut commands: Commands,
    mut game_data: ResMut<GameData>,
    mut next_state: ResMut<NextState<GameStates>>,
    mut query: Query<(Entity,Option<&mut Velocity>, &mut Transform, &Size, Option<&mut Player>, Option<&mut FakePlayer>, Option<&mut Button>, Option<&Door>, Option<&mut Fireball>, Option<&Enemy>, Option<&mut Health>, Option<&Melee>, Option<&Projectile>), (With<Collidable>, Without<Carpet>)>,
) {
    let mut iter = query.iter_combinations_mut();
    while let Some([(entity1,mut velocity1,mut transform1, size1, player1, fake1, button1, door1, fireball1, enemy1, health1, melee1, projectile1), (entity2,mut velocity2 ,mut transform2, size2, player2, fake2, button2, door2, fireball2, enemy2, health2, melee2, projectile2)]) = iter.fetch_next() {
        let collision = collide(
            transform1.translation,
            size1.value,
            transform2.translation,
            size2.value,
        );
        if let Some(collision) = collision {
            if player1.is_some() && melee2.is_none() && projectile2.is_none(){
                if button2.is_some(){
                    if let Some(mut button) = button2{
                        button.pressed = true;
                    }
                    continue;
                }
                if fireball2.is_some(){
                    if let Some(mut health) = health1{
                        health.current -= 1.;
                        commands.entity(entity2).despawn();
                    }
                    continue;
                }
                if door2.is_some(){
                    game_data.room_cleared = true;
                    if game_data.game_level == 14 {
                        next_state.set(GameStates::Credits);
                    }
                    continue;
                }
                if enemy2.is_some(){
                    continue;
                }
                if fake2.is_some(){
                    continue;
                }
                if let Some(mut player) = player1 {
                    if let Some(mut velocity) = velocity1 {
                        match collision {
                            Collision::Left => {
                                player.moving = false;
                                transform1.translation.x = transform1.translation.x - CHANGE_PLAYER_SPEED;
                                velocity.velocity.x = 0.;
                                // velocity.velocity.y = 0.;
                            }
                            Collision::Right => {
                                player.moving = false;
                                transform1.translation.x = transform1.translation.x + CHANGE_PLAYER_SPEED;
                                velocity.velocity.x = 0.;
                                // velocity.velocity.y = 0.;
                            }
                            Collision::Top => {
                                player.moving = false;
                                transform1.translation.y = transform1.translation.y + CHANGE_PLAYER_SPEED;
                                // velocity.velocity.x = 0.;
                                velocity.velocity.y = 0.;
                            }
                            Collision::Bottom => {
                                player.moving = false;
                                transform1.translation.y = transform1.translation.y - CHANGE_PLAYER_SPEED;
                                // velocity.velocity.x = 0.;
                                velocity.velocity.y = 0.;
                            }
                            Collision::Inside => {
                                player.moving = false;
                                // if transform1.translation.x < 1280. / 2. {
                                //     transform1.translation.x = transform1.translation.x + CHANGE_PLAYER_SPEED * 3.;
                                //     velocity.velocity.x = 0.;
                                //     velocity.velocity.y = 0.;
                                // } 
                                // if transform1.translation.x > 1280. / 2. {
                                //     transform1.translation.x = transform1.translation.x - CHANGE_PLAYER_SPEED * 3.;
                                //     velocity.velocity.x = 0.;
                                //     velocity.velocity.y = 0.;
                                // } 
                                // if transform1.translation.y < 720. / 2. {
                                //     transform1.translation.y = transform1.translation.y + CHANGE_PLAYER_SPEED * 3.;
                                //     velocity.velocity.x = 0.;
                                //     velocity.velocity.y = 0.;
                                // } 
                                // if transform1.translation.y > 720. / 2. {
                                //     transform1.translation.y = transform1.translation.y - CHANGE_PLAYER_SPEED * 3.;
                                //     velocity.velocity.x = 0.;
                                //     velocity.velocity.y = 0.;
                                // } 
                                // transform1.translation.x = transform1.translation.x - CHANGE_PLAYER_SPEED * 4.;
                                // transform1.translation.y = transform1.translation.y - CHANGE_PLAYER_SPEED * 4.;
                                // velocity.velocity.x = -velocity.velocity.x / 10.;
                                // velocity.velocity.y = -velocity.velocity.y / 10.;
                            }
                        }
                    }
                    continue;
                }
            }

            if player2.is_some() && melee1.is_none() && projectile1.is_none(){
                if button1.is_some(){
                    if let Some(mut button) = button1{
                        button.pressed = true;
                    }
                    continue;
                }
                if fireball1.is_some(){
                    if let Some(mut health) = health2{
                        health.current -= 1.;
                        commands.entity(entity1).despawn();
                    }
                    continue;
                }
                if door1.is_some(){
                    game_data.room_cleared = true;
                    if game_data.game_level == 14 {
                        next_state.set(GameStates::Credits);
                    }
                    continue;
                }
                if enemy1.is_some(){
                    continue;
                }
                if fake1.is_some(){
                    continue;
                }
                if let Some(mut velocity) = velocity2 {
                    match collision {
                        Collision::Left => {
                            transform2.translation.x = transform2.translation.x - CHANGE_PLAYER_SPEED;
                            velocity.velocity.x = 0.;
                            // velocity.velocity.y = 0.;
                        }
                        Collision::Right => {
                            transform2.translation.x = transform2.translation.x + CHANGE_PLAYER_SPEED;
                            velocity.velocity.x = 0.;
                            // velocity.velocity.y = 0.;
                        }
                        Collision::Top => {
                            transform2.translation.y = transform2.translation.y + CHANGE_PLAYER_SPEED;
                            // velocity.velocity.x = 0.;
                            velocity.velocity.y = 0.;
                        }
                        Collision::Bottom => {
                            transform2.translation.y = transform2.translation.y - CHANGE_PLAYER_SPEED;
                            // velocity.velocity.x = 0.;
                            velocity.velocity.y = 0.;
                        }
                        Collision::Inside => {
                            // if transform2.translation.x < 1280. / 2. {
                            //     transform2.translation.x = transform2.translation.x + CHANGE_PLAYER_SPEED * 3.;
                            //     velocity.velocity.x = 0.;
                            //     velocity.velocity.y = 0.;
                            // } 
                            // if transform2.translation.x > 1280. / 2. {
                            //     transform2.translation.x = transform2.translation.x - CHANGE_PLAYER_SPEED * 3.;
                            //     velocity.velocity.x = 0.;
                            //     velocity.velocity.y = 0.;
                            // } 
                            // if transform2.translation.y < 720. / 2. {
                            //     transform2.translation.y = transform2.translation.y + CHANGE_PLAYER_SPEED * 3.;
                            //     velocity.velocity.x = 0.;
                            //     velocity.velocity.y = 0.;
                            // } 
                            // if transform2.translation.y > 720. / 2. {
                            //     transform2.translation.y = transform2.translation.y - CHANGE_PLAYER_SPEED * 3.;
                            //     velocity.velocity.x = 0.;
                            //     velocity.velocity.y = 0.;
                            // } 
                        }
                    }
                }
                continue;
            }

            if enemy1.is_some(){
                if projectile2.is_some() {
                    if let Some(mut health) = health1{
                        health.current -= projectile2.unwrap().damage;
                        commands.entity(entity2).despawn();
                    }
                    continue;
                }
                if melee2.is_some(){
                    if let Some(mut health) = health1{
                        // Currently hits every frame, will need to add invincibility frames
                        if let Some(mut enemy) = enemy1{
                            if enemy.isblocking == false {
                                health.current -= melee2.unwrap().damage;
                            } else {
                                health.current -= melee2.unwrap().damage / 2.;
                            }
                        }
                        // commands.entity(entity2).despawn();
                    }
                    continue;
                }
                if player2.is_none() && fireball2.is_none(){
                    if let Some(mut velocity) = velocity1 {
                        match collision {
                            Collision::Left => {
                                transform1.translation.x = transform1.translation.x - CHANGE_ENEMY_SPEED;
                                velocity.velocity.x = 0.;
                                // velocity.velocity.y = 0.;
                            }
                            Collision::Right => {
                                transform1.translation.x = transform1.translation.x + CHANGE_ENEMY_SPEED;
                                velocity.velocity.x = 0.;
                                // velocity.velocity.y = 0.;
                            }
                            Collision::Top => {
                                transform1.translation.y = transform1.translation.y + CHANGE_ENEMY_SPEED;
                                // velocity.velocity.x = 0.;
                                velocity.velocity.y = 0.;
                            }
                            Collision::Bottom => {
                                transform1.translation.y = transform1.translation.y - CHANGE_ENEMY_SPEED;
                                // velocity.velocity.x = 0.;
                                velocity.velocity.y = 0.;
                            }
                            Collision::Inside => {
    
                            }
                        }
                    }
                }
                continue;
            }

            if enemy2.is_some(){
                if projectile1.is_some() {
                    if let Some(mut health) = health2{
                        health.current -= projectile1.unwrap().damage;
                        commands.entity(entity1).despawn();
                    }
                    continue;
                }
                if melee1.is_some(){
                    if let Some(mut health) = health2{
                        // Currently hits every frame, will need to add invincibility frames
                        if let Some(mut enemy) = enemy2{
                            if enemy.isblocking == false {
                                health.current -= melee1.unwrap().damage;
                            } else {
                                health.current -= melee1.unwrap().damage / 2.;
                            }
                        }
                    }
                    
                    continue;
                }
                if player1.is_none() && fireball1.is_none(){
                    if let Some(mut velocity) = velocity2 {
                        match collision {
                            Collision::Left => {
                                transform2.translation.x = transform2.translation.x - CHANGE_ENEMY_SPEED;
                                velocity.velocity.x = 0.;
                                // velocity.velocity.y = 0.;
                            }
                            Collision::Right => {
                                transform2.translation.x = transform2.translation.x + CHANGE_ENEMY_SPEED;
                                velocity.velocity.x = 0.;
                                // velocity.velocity.y = 0.;
                            }
                            Collision::Top => {
                                transform2.translation.y = transform2.translation.y + CHANGE_ENEMY_SPEED;
                                // velocity.velocity.x = 0.;
                                velocity.velocity.y = 0.;
                            }
                            Collision::Bottom => {
                                transform2.translation.y = transform2.translation.y - CHANGE_ENEMY_SPEED;
                                // velocity.velocity.x = 0.;
                                velocity.velocity.y = 0.;
                            }
                            Collision::Inside => {
    
                            }
                        }
                    }
                }
                continue;
            }
            if fireball1.is_some() {
                if enemy2.is_none() && melee2.is_none() && player2.is_none() {
                    commands.entity(entity1).despawn();
                }
            }
            if fireball2.is_some() {
                if enemy1.is_none() && melee1.is_none() && player1.is_none() {
                    commands.entity(entity2).despawn();
                }
            }
            if projectile1.is_some() {
                if enemy2.is_none() && melee2.is_none() && player2.is_none() {
                    commands.entity(entity1).despawn();
                }
            }
            if projectile2.is_some() {
                if enemy1.is_none() && melee1.is_none() && player1.is_none() {
                    commands.entity(entity2).despawn();
                }
            }
        }
    }
}