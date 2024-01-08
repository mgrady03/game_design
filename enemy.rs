use bevy::prelude::*;

pub fn spawn_enemy(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    let enemy_handle = asset_server.load("kevin1.png");
    let enemy_atlas = TextureAtlas::from_grid(enemy_handle, Vec2::splat(80.), 4, 4, None, None);
    let enemy_atlas_len = enemy_atlas.textures.len();
    let enemy_atlas_handle = texture_atlases.add(enemy_atlas);
    let health_value = 100.0;

    commands
    .spawn(SpriteSheetBundle {
        texture_atlas: enemy_atlas_handle.clone(),
        sprite: TextureAtlasSprite {
            index: 0,
            ..default()
        },
        transform: Transform::from_xyz(0., -(WIN_H / 2.) + (TILE_SIZE * SCALE * 6.1), 50.) // Adjust the starting position
            .with_scale(Vec3::splat(0.75)),
        ..default()
    })
    .insert(AnimationTimer(Timer::from_seconds(
        ANIM_TIME,
        TimerMode::Repeating,
    )))
    .insert(Velocity::new())
    // .insert(Enemy { health: Heal ::new(health_value) });
    .insert(Enemy::new(false))
    .insert(Health::new(health_value))
    .insert(Size::new(Vec2::new(60.,60.)))
    .insert(Collidable);
}

pub fn follow_player(
    mut commands: Commands,
    time: Res<Time>,
    mut pl: Query<&mut Player,With<Player>>,
    mut config: ResMut<FireballTimer>,
    player_query: Query<(&Transform, &Velocity), (With<Player>, Without<Sword>)>,
    mut enemy_query: Query<(&mut Transform, &mut Velocity, Entity, &mut Health, &mut Enemy, &mut TextureAtlasSprite), (With<Enemy>, Without<Player>, Without<Wall>, Without<Door>)>,
    wall_query: Query<&Transform, (With<Wall>, Without<Player>, Without<Sword>)>,
    door_query: Query<&Transform, (With<Door>, Without<Player>, Without<Wall>, Without<Enemy>, Without<Sword>)>,
    asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    //mut boss_shield_query: Query<(Entity, &mut TextureAtlasSprite, &Enemy), With<Enemy>,>,
    sword_query: Query<(&Transform, &Damage), (With<Sword>, Without<Player>, Without<Wall>, Without<Door>, Without<Enemy>)>,
) {


        let projectile_handle = asset_server.load("projectile.png");
        let projectile_atlas = TextureAtlas::from_grid(projectile_handle, Vec2::splat(20.), 4, 4, None, None);
        let projectile_atlas_len = projectile_atlas.textures.len();
        let projectile_atlas_handle = texture_atlases.add(projectile_atlas);
        for (player_transform, _player_velocity) in player_query.iter() {
        for (mut enemy_transform,mut enemy_velocity,  entity, mut health, mut enemy, mut sprite) in enemy_query.iter_mut() {
            //calculate the direction from enemy to player
            
            let direction = player_transform.translation.truncate()- enemy_transform.translation.truncate();
            //let direction =  enemy_transform.translation.truncate()-player_transform.translation.truncate();
            let distance = direction.length();
            let speed = 2.0; //adjust the enemy's speed meh
            let variability_number = calculate_variability_scale(&mut pl);
            print!("{}",variability_number);
            print!("{}"," ");
            if distance<= 25.0{
                enemy.isblocking = true;
                // for (entity_boss, mut sprite, boss_enemy) in boss_shield_query.iter_mut() {
                if(variability_number>0.5)
                {
                    enemy.isdodging=true;
                }else{
                    enemy.isblocking = true;
                }
                // }

                if(enemy.isboss == true)
                {
                    sprite.index = 1;
                }

            }else{
                enemy.isblocking=false;
                // for (entity_boss, mut sprite, boss_enemy) in boss_shield_query.iter_mut() {
                //     if(boss_enemy.isboss == true)
                //     {
                //         sprite.index = 0;
                //     }
                // }

                if(enemy.isboss == true)
                {
                    sprite.index = 0;
                }
            }
            if distance > 150.0{
                enemy.isdodging = false;
            }
            
            if distance > 1.0 {
                // normalize the direction vector and convert it to vec2
                
                let normalized_direction = direction.normalize_or_zero(); // convert to Vec2 cuz not vec3

                // calculate the velocity to move towards the player
                if (enemy.isdodging == true && enemy.isboss == true){
                   
                    enemy_velocity.velocity = normalized_direction * speed*-2.0;
                }else{
                    enemy_velocity.velocity = normalized_direction * speed;
                }

                // check for collision with walls maybe?
                let new_position = enemy_transform.translation.truncate()
                    + Vec2::new(
                        enemy_velocity.velocity.x,
                        enemy_velocity.velocity.y,
                    );

                    config.timer.tick(time.delta());

                    if config.timer.finished() {
                    commands
                    .spawn(SpriteSheetBundle {
                        //texture_atlas: player_atlas_handle.clone(),
                        texture_atlas: projectile_atlas_handle.clone(),
                        sprite: TextureAtlasSprite {
                            index: 0,
                            ..default()
                        },
                        transform: Transform::from_xyz(new_position.x+19., new_position.y+24., 20.) // Adjust the starting position
                            .with_scale(Vec3::splat(1.0)),
                        ..default()
                    })
                    .insert(Velocity::new())
                    .insert(Fireball::new())
                    .insert(Size::new(Vec2::new(20.,20.)))
                    .insert(Collidable);
                    }
                // if check_enemy_collisions(new_position.extend(0.0), &wall_query, &door_query, &mut game_state_query, &sword_query, (&mut enemy_transform, &mut enemy_velocity, &mut health, entity), &mut commands) {
                //     enemy_transform.translation = new_position.extend(0.0);
                // }
                enemy_transform.translation = new_position.extend(0.0);
            } else {
                // if the enemy is already at the player's position, stop moving i think?
                enemy_velocity.velocity = Vec2::ZERO;
            }
        }
    }
}

pub fn setup_fireball_spawning(
    mut commands: Commands,
) {
    commands.insert_resource(FireballTimer {
        // create the repeating timer
        timer: Timer::new(Duration::from_millis(650), TimerMode::Repeating),
    });
}

pub fn shoot_fireball(
    mut commands: Commands,
    player_query: Query<(&Transform, &Velocity), (With<Player>, Without<Sword>)>,
    mut fireball_query: Query<(&mut Transform, &mut Velocity, &mut Fireball, Entity), (With<Fireball>, Without<Player>, Without<Wall>)>,
    wall_query: Query<&Transform, (With<Wall>, Without<Player>, Without<Sword>)>,
    ) {
    
    for play in player_query.iter() {
        // Get the player's initial position
        let (player_transform, _player_velocity) = play;
        
        // Iterate over all fireballs
        for (mut fireball_transform, mut fireball_velocity,mut fireball, fireball_entity) in fireball_query.iter_mut() {
            // collision detection
            // if !check_fireball_collisions(fireball_transform.translation, &wall_query, &player_query) {
            //     // despawn fireball
            //     commands.entity(fireball_entity).despawn();
            // }
            
            // Calculate the direction only once (from fireball to initial player position)

            if fireball.flag == false{
                let direction = player_transform.translation.truncate() - fireball_transform.translation.truncate();
                let speed = 10.0;
                let normalized_direction = direction.normalize_or_zero();
                fireball_velocity.velocity = normalized_direction * speed;
                fireball.flag = true;
            }

            // Update the fireball's pfosition using the new velocity
            let new_position = fireball_transform.translation.truncate() + Vec2::new(
                fireball_velocity.velocity.x,
                fireball_velocity.velocity.y,
            );

        fireball_transform.translation = new_position.extend(0.0);
        }
    }
}









//////////////////////////////////////








use bevy::sprite::collide_aabb::collide;
use std::time::Duration;

use crate::components::collision::Collidable;
use crate::components::enemy::{Enemy, FireballTimer, AnimationTimer, Fireball, self};
use crate::components::game::Health;

use crate::components::size::Size;
use crate::systems::game::{WIN_H, WIN_W, TILE_SIZE, SCALE, ANIM_TIME};
use crate::components::game::{GameData, Velocity};

use crate::systems::player::PLAYER_SIZE;
use crate::components::player::{Player, Sword, Damage};

use crate::components::room::{Wall, Button, Door, Floor};

pub const BOSS_SIZE: Vec2 = Vec2::new(40., 70.);
static mut LOOP: i32 = 0;

pub fn boss_spawn(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    mut texture_atlases: ResMut<Assets<TextureAtlas>>) {

    let health_value = 100.;

    let enemy_handle = asset_server.load("boss_movement_shield_2.png");
    let enemy_atlas =
        TextureAtlas::from_grid(enemy_handle, BOSS_SIZE, 3, 3 , None, None);
    let enemy_atlas_handle = texture_atlases.add(enemy_atlas);
    commands
        .spawn(
            SpriteSheetBundle {
            texture_atlas: enemy_atlas_handle,
            sprite: TextureAtlasSprite {
                index: 0,
                ..default()
            },
            transform: Transform::from_xyz(0., -(WIN_H / 2.) + (TILE_SIZE * 10.0), 50.)
                .with_scale(Vec3::splat(3.)),
            ..default()
        })
        .insert(AnimationTimer(Timer::from_seconds(
            ANIM_TIME,
            TimerMode::Repeating,
        )))
        .insert(Velocity::new())
        .insert(Enemy::new(true))
        .insert(Collidable)
        .insert(Health::new(health_value))
        .insert(Size::new(Vec2::new(80.,80.)));

        // for (player_transform, _player_velocity) in player_query.iter() {
        
        //     for (mut enemy_transform,mut enemy_velocity, mut sprite, texture_atlas_handle, mut timer,) in &mut enemy_query.iter_mut() {
        //         //calculate the direction from enemy to player
                
        //         let direction = player_transform.translation.truncate()- enemy_transform.translation.truncate();
        //         //let direction =  enemy_transform.translation.truncate()-player_transform.translation.truncate();
        //         let distance = direction.length();
        //         let speed = 3.0; //adjust the enemy's speed meh
    
        //         if distance > 10.0 {
        //             // normalize the direction vector and convert it to vec2
                    
        //             let normalized_direction = direction.normalize_or_zero(); // convert to Vec2 cuz not vec3
    
        //             // calculate the velocity to move towards the player
        //             enemy_velocity.velocity = normalized_direction * speed;
    
        //             // check for collision with walls maybe?
        //             let new_position = enemy_transform.translation.truncate()
        //                 + Vec2::new(
        //                     enemy_velocity.velocity.x,
        //                     enemy_velocity.velocity.y,
        //                 );
    
        //             enemy_transform.translation = new_position.extend(0.0);
    
        //             timer.tick(time.delta());
        //             if timer.finished() {
        //                 //let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
        //                 if enemy_velocity.velocity.x.abs() > enemy_velocity.velocity.y.abs() {
        //                     if enemy_velocity.velocity.x > 0. {
        //                         sprite.index = (sprite.index + 1) % 3 + 6;
        //                     } else {
        //                         sprite.index = (sprite.index + 1) % 3 + 3;
        //                     }
        //                 } else {
        //                     if enemy_velocity.velocity.y > 0. {
        //                         sprite.index = (sprite.index + 1) % 3;
        //                     } else {
        //                         sprite.index = (sprite.index + 1) % 3;
        //                     }
        //                 }
        //             }
    
    
        //         } else{
                    
        //             enemy_velocity.velocity = Vec2::ZERO;
        //             sprite.index = 0;
        //         }
        //     }
        // }

}

// pub fn follow_player_boss(
//     time: Res<Time>,
//     player_query: Query<(&Transform, & Velocity), With<Player>>,
//     mut enemy_query: Query<(&mut Transform, &mut Velocity, &mut TextureAtlasSprite, &Handle<TextureAtlas>, &mut AnimationTimer), (With<Enemy>, Without<Player>, Without<Wall>)>,
//     //mut enemy_query: Query<(&mut Transform, &mut Velocity), (With<Enemy>, Without<Player>)>,
// ) {
    
//         for (player_transform, _player_velocity) in player_query.iter() {
        
//         for (mut enemy_transform,mut enemy_velocity, mut sprite, texture_atlas_handle, mut timer,) in &mut enemy_query.iter_mut() {
//             //calculate the direction from enemy to player
            
//             let direction = player_transform.translation.truncate()- enemy_transform.translation.truncate();
//             //let direction =  enemy_transform.translation.truncate()-player_transform.translation.truncate();
//             let distance = direction.length();
//             let speed = 3.0; //adjust the enemy's speed meh

//             if distance > 10.0 {
//                 // normalize the direction vector and convert it to vec2
                
//                 let normalized_direction = direction.normalize_or_zero(); // convert to Vec2 cuz not vec3

//                 // calculate the velocity to move towards the player
//                 enemy_velocity.velocity = normalized_direction * speed;

//                 // check for collision with walls maybe?
//                 let new_position = enemy_transform.translation.truncate()
//                     + Vec2::new(
//                         enemy_velocity.velocity.x,
//                         enemy_velocity.velocity.y,
//                     );

//                 enemy_transform.translation = new_position.extend(0.0);

//                 timer.tick(time.delta());
//                 if timer.finished() {
//                     //let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
//                     if enemy_velocity.velocity.x.abs() > enemy_velocity.velocity.y.abs() {
//                         if enemy_velocity.velocity.x > 0. {
//                             sprite.index = (sprite.index + 1) % 3 + 6;
//                         } else {
//                             sprite.index = (sprite.index + 1) % 3 + 3;
//                         }
//                     } else {
//                         if enemy_velocity.velocity.y > 0. {
//                             sprite.index = (sprite.index + 1) % 3;
//                         } else {
//                             sprite.index = (sprite.index + 1) % 3;
//                         }
//                     }
//                 }


//             } else{
                
//                 enemy_velocity.velocity = Vec2::ZERO;
//                 sprite.index = 0;
//             }
//         }
//     }
// }

// pub fn check_fireball_collisions (
//     target_pos: Vec3,
//     wall_query: &Query<&Transform, (With<Wall>, Without<Player>, Without<Sword>)>,
//     player_query: &Query<(&Transform, &Velocity), (With<Player>, Without<Sword>)>,
// ) -> bool{
//     return collide_wall(target_pos, wall_query) && collide_player(target_pos, player_query);
// }

// pub fn check_enemy_collisions (
//     target_pos: Vec3,
//     wall_query: &Query<&Transform, (With<Wall>, Without<Player>, Without<Sword>)>,
//     door_query: &Query<&Transform, (With<Door>, Without<Player>, Without<Wall>, Without<Enemy>, Without<Sword>)>,
//     sword_query: &Query<(&Transform, &Damage), (With<Sword>, Without<Player>, Without<Wall>, Without<Door>, Without<Enemy>)>,
//     enemy_tuple: (&mut Transform, &mut Velocity, &mut Health, Entity),
//     commands: &mut Commands,
// ) -> bool{
//     return collide_wall(target_pos, wall_query) && collide_door(target_pos, door_query, game_state_query) && collide_sword(target_pos, sword_query, enemy_tuple, commands);
// }

pub fn collide_player(
    target_pos: Vec3,
    player_query: &Query<(&Transform, &Velocity), (With<Player>, Without<Sword>)>,
) -> bool {
    let mut moving = true;
    for (player_transform, _player_velocity) in player_query.iter() {
        let collision = collide(
            target_pos, 
            Vec2::splat(PLAYER_SIZE),
            player_transform.translation,
            Vec2::splat(PLAYER_SIZE)
        );
        if collision.is_some(){
            moving = false;
        }
    }
    return moving;
}

pub fn collide_sword(
    target_pos: Vec3,
    sword_query: &Query<(&Transform, &Damage), (With<Sword>, Without<Player>, Without<Wall>, Without<Door>, Without<Enemy>)>,
    enemy_tuple: (&mut Transform, &mut Velocity, &mut Health, Entity),
    commands: &mut Commands,
) -> bool {
    let mut moving = true;
    let (mut enemy_transform, mut velocity, mut health, entity) = enemy_tuple;
    for sword in sword_query.iter() {
        let collision = collide(
            target_pos, 
            BOSS_SIZE,
            sword.0.translation,
            Vec2::splat(TILE_SIZE)
        );
        if collision.is_some(){
            health.current -= sword.1.damage;
            if health.current <= 0.0 {
                commands.entity(entity).despawn();
            }
        }
    }
    return moving;
}

// pub fn player_hit_by_fireball(
//     mut player_query: Query<(&mut Player, &Transform)>,
//     fireball_query: Query<(&Transform, &Fireball), (With<Fireball>, With<Player>)>,
// ) {
//     for (mut player, player_transform) in player_query.iter_mut() {
//         for (_, fireball) in fireball_query.iter() {
//             //damage
//             let damage_amount = 5.0;

//             //update health component
//             player.health.take_damage(damage_amount);

//             print!("This is a test");

//             //zero or less
//             if player.health.current <= 0.0 {
//                 //death
//                 print!("Player has died");
//             }
//         }
//     }
// }

pub fn calculate_variability_scale(
    //player_query: &Query<(&Transform, &Velocity,&mut Player), (With<Player>, Without<Sword>)>
    pl: &mut Query<&mut Player,With<Player>>,
) -> f32 {
    let mut variability_scale =0.0;
    for mut p in pl.iter_mut(){
        let total_key_presses = p.wkey + p.akey + p.skey + p.dkey;

        // Calculate probabilities
        let p_w = p.wkey / total_key_presses;
        let p_a = p.akey / total_key_presses;
        let p_s = p.skey / total_key_presses;
        let p_d = p.dkey / total_key_presses;
    
        // Calculate entropy-based variability
        let variability = 1.0
            - (p_w * p_w.log2() + p_a * p_a.log2() + p_s * p_s.log2() + p_d * p_d.log2());
    
        // Linear transformation to scale from 1 to 10
        let min_variability = 0.0;  // Adjust based on your observations
        let max_variability = 1.0;  // Adjust based on your observations
        variability_scale =
            1.0 + 9.0 * (variability - min_variability)/(max_variability - min_variability);
        return variability_scale;
    }

    
    

    return variability_scale;
}

