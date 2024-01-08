use std::convert;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::components::game::Velocity;
use crate::components::projectile::Projectile;
use crate::components::collision::Collidable;
use crate::components::player::{Player, Weapon, WeaponType};
use crate::components::size::Size;

pub fn fire_projectile(
    mouse_button_input: Res<Input<MouseButton>>,
    window: Query<&Window, &PrimaryWindow>,
    mut commands: Commands,
    mut query: Query<(&Transform, &Velocity, &Weapon, &mut Player), With<Player>>, 
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    for (player_pos, _, Weapon, mut player) in query.iter_mut() {
        if mouse_button_input.just_pressed(MouseButton::Left) && player.cooldown <= 0.0 && (Weapon.weapon_type == WeaponType::Bow || Weapon.weapon_type == WeaponType::Wand){
            if let Some(position) = window.single().cursor_position() {
                let player_pos = Vec2::new(player_pos.translation.x, player_pos.translation.y);
                let mouse_pos = recalculate_mouse_pos(position);
                let direction = (mouse_pos-player_pos).normalize();
                let projectile = Projectile {
                    damage: Weapon.damage,
                    velocity: direction * 500.0,
                };
                let mut projectile_handle = asset_server.load("arrow_anim.png");
                if(Weapon.weapon_type == WeaponType::Wand){
                    projectile_handle = asset_server.load("magic_anim.png");
                }
                let projectile_atlas = TextureAtlas::from_grid(projectile_handle, Vec2::splat(20.), 4, 4, None, None);
                let projectile_atlas_len = projectile_atlas.textures.len();
                let projectile_atlas_handle = texture_atlases.add(projectile_atlas);
                commands
                .spawn(SpriteSheetBundle {
                    //texture_atlas: player_atlas_handle.clone(),
                    texture_atlas: projectile_atlas_handle.clone(),
                    sprite: TextureAtlasSprite {
                        index: 0,
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(player_pos.x, player_pos.y, 0.0))
                        .with_scale(Vec3::splat(1.0)),
                    ..Default::default()
                })
                .insert(Projectile{damage: projectile.damage, velocity: projectile.velocity})
                .insert(Velocity{velocity: projectile.velocity})
                .insert(Collidable)
                .insert(Size::new(Vec2::new(20.,20.)));
                player.cooldown = Weapon.cooldown;
            }
        }
    }
}

pub fn move_projectile(
    mut query: Query<(&mut Transform, &Velocity), With<Projectile>>,
) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.velocity.x * 0.01;
        transform.translation.y += velocity.velocity.y * 0.01;
    }
}

fn recalculate_mouse_pos(
    pos: Vec2,
) -> Vec2{
    let new_x = pos.x - 640.;
    let mut new_pos = Vec2::new(new_x, pos.y);
    if(pos.y > 360.){
        let new_y = -1.*(pos.y - 360.);
        new_pos = Vec2::new(new_x, new_y);
    }else{
        let new_y = 360. - pos.y;
        new_pos = Vec2::new(new_x, new_y);
    }
    return new_pos;
}