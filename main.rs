use bevy::{prelude::*, window::PresentMode};
use std::convert::From;
use bevy::sprite::collide_aabb::collide;

mod credits;
mod velocity;

const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;
const TILE_SIZE: f32 = 16.;
const PLAYER_SIZE: f32 = 8.;
const ENEMY_SIZE: f32 = 8.;
const SCALE: f32 = 5.0;

const ANIM_TIME: f32 = 0.2;
const PLAYER_SPEED: f32 = 300.;
const ACCEL_RATE: f32 = 3600.;

// 2d array to represent empty room
const ROOM: [[usize; 10]; 8] = [
    [ 0,  1,  1,  2, 33, 34,  5,  1,  1,  6], 
    [ 7,  8,  8,  9, 10, 11, 12,  8,  8, 13], 
    [14, 15, 15, 15, 17, 18, 15, 15, 15, 20],
    [14, 15, 15, 15, 15, 15, 15, 15, 15, 20],
    [14, 15, 15, 15, 15, 15, 15, 15, 15, 20],
    [14, 15, 15, 15, 15, 15, 15, 15, 15, 20],
    [14, 15, 15, 15, 16, 19, 15, 15, 15, 20],
    [21, 22, 22, 22, 23, 26, 22, 22, 22, 27],
];

#[derive(Component)]
struct Floor;

#[derive(Component)]
struct Wall;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Block;

#[derive(Component)]
struct Enemy;

struct Sides {
    top: f32,
    bottom: f32,
    left: f32,
    right: f32,
}

impl From<Vec3> for Sides {
    fn from(pos: Vec3) -> Self {
        Self {
            top: pos.y + 300. / 2.,
            bottom: pos.y - 300. / 2.,
            left: pos.x - PLAYER_SIZE / 2.,
            right: pos.x + PLAYER_SIZE/ 2.,
        }
    }
}

#[derive(Component)]
struct Button;

#[derive(Component)]
struct Pressed {
    pressed: bool,
}

impl Pressed {
    fn new() -> Self {
        Self {
            pressed: false,
        }
    }
}

#[derive(Component)]
struct Door;

#[derive(Component, Deref, DerefMut)]
struct PopupTimer(Timer);

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn main() {
    App::new()
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
        .add_systems(Startup, setup)
        .add_systems(Update, move_player)
        .add_systems(Update, animate_player.after(move_player))
        .add_systems(Update,follow_player)
        // .add_systems(Startup,credits::setup)
        // .add_systems(Update,credits::timer)
        .add_systems(Update, animate_button.after(move_player))
        .run();
}

fn check_for_collision(
    target_pos: Vec3,
    wall_query: &Query<&Transform, (With<Wall>, Without<Player>)>,
    button_query: &Query<(&Transform, &Pressed), (With<Button>, Without<Player>, Without<Enemy>)>,
    door_query: &Query<&Transform, (With<Door>, Without<Player>, Without<Wall>, Without<Enemy>)>,
) ->bool{
    let mut moving = true;

    for wall_transform in wall_query.iter(){
        let collision = collide(
            target_pos, 
            Vec2::splat(TILE_SIZE * 0.9 * SCALE + PLAYER_SIZE),
            wall_transform.translation,
            Vec2::splat(1.0)
        );
        if collision.is_some(){
            moving = false;
        }
        
    }

    for btn in button_query.iter(){
        let mut button_transform = btn.0;
        let collision = collide(
            target_pos, 
            Vec2::splat(TILE_SIZE * 0.9 * SCALE + PLAYER_SIZE),
            button_transform.translation,
            Vec2::splat(1.0)
        );
        if collision.is_some(){
            // change button Pressed component to true
            info!("button pressed");
            // let (mut t, mut p) = button_query.single_mut();
            // p = Pressed { pressed: true };
        }
        
    }

    for door in door_query.iter(){
        let mut door_transform = door;
        let collision = collide(
            target_pos, 
            Vec2::splat(TILE_SIZE * 0.9 * SCALE + PLAYER_SIZE),
            door_transform.translation,
            Vec2::splat(1.0)
        );
        if collision.is_some() && moving {  // only enter door if player is moving (prevents entering closed door)
            // clear screen

            // roll credits
            info!("door entered");
        }
        
    }
    
    return moving;
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    commands.spawn(Camera2dBundle::default());

    // Tiling setup
    let dungeon_handle = asset_server.load("dungeon_tiles.png");
    let dungeon_atlas = TextureAtlas::from_grid(dungeon_handle, Vec2::splat(TILE_SIZE), 7, 5, None, None);
    let dungeon_atlas_len = dungeon_atlas.textures.len();
    let dungeon_atlas_handle = texture_atlases.add(dungeon_atlas);

    let player_handle = asset_server.load("players.png");
    let player_atlas = TextureAtlas::from_grid(player_handle, Vec2::splat(PLAYER_SIZE), 4, 4, None, None);
    let player_atlas_len = player_atlas.textures.len();
    let player_atlas_handle = texture_atlases.add(player_atlas);

    let enemy_handle = asset_server.load("kevin.png");
    let enemy_atlas = TextureAtlas::from_grid(enemy_handle, Vec2::splat(ENEMY_SIZE), 4, 4, None, None);
    let enemy_atlas_len = enemy_atlas.textures.len();
    let enemy_atlas_handle = texture_atlases.add(enemy_atlas);

    commands
        .spawn(SpriteSheetBundle {
            //texture_atlas: player_atlas_handle.clone(),
            texture_atlas: enemy_atlas_handle,
            sprite: TextureAtlasSprite {
                index: 0,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0) // Adjust the starting position
                .with_scale(Vec3::splat(5.0)),
            ..default()
        })
        .insert(AnimationTimer(Timer::from_seconds(
            ANIM_TIME,
            TimerMode::Repeating,
        )))
        .insert(velocity::Velocity::new())
        .insert(Enemy);

        // Background
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.49019607843137253,0.3843137254901961,0.36470588235294116),
                custom_size: Some(Vec2::new(1280.,720.)),
                ..default()
            },
            transform: Transform::from_xyz(0., 0., -1.),
            ..default()
        });

    let x_bound = WIN_W / 2. - TILE_SIZE / 2.;
    let y_bound = WIN_H / 2. - TILE_SIZE / 2.;

    // Spawn each tile in a 10x8 room centered at origin
    const NUM_BLOCKS: i32 = 3;
    const NUM_BUTTONS: i32 = 1;
    // array of length 36
    let mut other_tiles = [15; 36];
    // place NUM_BLOCKS blocks and NUM_BUTTONS buttons randomly into other_tiles
    for i in 0..NUM_BLOCKS {
        let mut rand = rand::random::<usize>() % 36;
        while other_tiles[rand] != 15 {
            rand = rand::random::<usize>() % 36;
        }
        other_tiles[rand] = 28;
    }
    for i in 0..NUM_BUTTONS {
        let mut rand = rand::random::<usize>() % 36;
        while other_tiles[rand] != 15 {
            rand = rand::random::<usize>() % 36;
        }
        other_tiles[rand] = 29;
    }

    let mut floor_counter = 0;
    for x in 0..10 {
        for y in 0..8 {
            // 7-y to reverse order, since 2d array is spawned from bottom up
            let mut tile_index = ROOM[7-y][x] % dungeon_atlas_len;
            if tile_index == 15 {
                tile_index = other_tiles[floor_counter];
                floor_counter += 1;
            }
        if 15 <= tile_index && tile_index <= 19 {
            commands.spawn(SpriteSheetBundle {
            texture_atlas: dungeon_atlas_handle.clone(),
            transform: Transform::from_xyz((x as f32 - 4.5) as f32 * TILE_SIZE * SCALE, (y as f32 - 3.5) as f32 * TILE_SIZE * SCALE, 0.)
            .with_scale(Vec3::new(SCALE,SCALE,0.0)), //Changes scale of the tile map
            sprite: TextureAtlasSprite {
            index: tile_index,
            ..default()
            },
            ..default()
            }).insert(Floor);
        } else if tile_index == 29 || tile_index == 30 {
            commands.spawn(SpriteSheetBundle {
            texture_atlas: dungeon_atlas_handle.clone(),
            transform: Transform::from_xyz((x as f32 - 4.5) as f32 * TILE_SIZE * SCALE, (y as f32 - 3.5) as f32 * TILE_SIZE * SCALE, 0.)
            .with_scale(Vec3::new(SCALE,SCALE,0.0)), //Changes scale of the tile map
            sprite: TextureAtlasSprite {
            index: tile_index,
            ..default()
            },
            ..default()
            }).insert(Floor).insert(Button).insert(Pressed::new());
        } else if tile_index == 10 || tile_index == 11 {
            commands.spawn(SpriteSheetBundle {
            texture_atlas: dungeon_atlas_handle.clone(),
            transform: Transform::from_xyz((x as f32 - 4.5) as f32 * TILE_SIZE * SCALE, (y as f32 - 3.5) as f32 * TILE_SIZE * SCALE, 0.)
            .with_scale(Vec3::new(SCALE,SCALE,0.0)), //Changes scale of the tile map
            sprite: TextureAtlasSprite {
            index: tile_index,
            ..default()
            },
            ..default()
            }).insert(Floor).insert(Door);
            // spawn "closed door" wall over top to be destroyed when the door is opened
            // commands.spawn(SpriteSheetBundle {
            //     texture_atlas: dungeon_atlas_handle.clone(),
            //     transform: Transform::from_xyz((x as f32 - 4.5) as f32 * TILE_SIZE * SCALE, (y as f32 - 3.5) as f32 * TILE_SIZE * SCALE, 0.1)
            //     .with_scale(Vec3::new(SCALE,SCALE,0.0)), //Changes scale of the tile map
            //     sprite: TextureAtlasSprite {
            //     index: tile_index+21,   // spawns tiles 30 & 31, the "closed door" wall
            //     ..default()
            //     },
            //     ..default()
            //     }).insert(Wall).insert(Door);
        } else {
            commands.spawn(SpriteSheetBundle {
            texture_atlas: dungeon_atlas_handle.clone(),
            transform: Transform::from_xyz((x as f32 - 4.5) as f32 * TILE_SIZE * SCALE, (y as f32 - 3.5) as f32 * TILE_SIZE * SCALE, 0.)
            .with_scale(Vec3::new(SCALE,SCALE,0.0)), //Changes scale of the tile map
            sprite: TextureAtlasSprite {
            index: tile_index,
            ..default()
            },
            ..default()
            }).insert(Wall);
            }
        }
    }
            

    let player_handle = asset_server.load("players.png");
    let player_atlas =
        TextureAtlas::from_grid(player_handle, Vec2::splat(PLAYER_SIZE), 4, 1 , None, None);
    let player_atlas_handle = texture_atlases.add(player_atlas);
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: player_atlas_handle,
            sprite: TextureAtlasSprite {
                index: 0,
                ..default()
            },
            transform: Transform::from_xyz(0., -(WIN_H / 2.) + (TILE_SIZE * SCALE * 1.5), 900.)
                .with_scale(Vec3::splat(SCALE)),
            ..default()
        })
        .insert(AnimationTimer(Timer::from_seconds(
            ANIM_TIME,
            TimerMode::Repeating,
        )))
        .insert(velocity::Velocity::new())
        .insert(Player);

    // credits(commands, asset_server);
}

fn move_player(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut player: Query<(&mut Transform, &mut velocity::Velocity), (With<Player>, Without<Wall>)>,
    wall_query: Query<&Transform, (With<Wall>, Without<Player>)>,
    button_query: Query<(&Transform, &Pressed), (With<Button>, Without<Player>, Without<Enemy>)>,
    door_query: Query<&Transform, (With<Door>, Without<Player>, Without<Wall>, Without<Enemy>)>,
) {
    let (mut pt, mut pv) = player.single_mut();

    let mut deltav = Vec2::splat(0.);

    if input.pressed(KeyCode::A) {
        deltav.x -= 1.;
    }

    if input.pressed(KeyCode::D) {
        deltav.x += 1.;
    }

    if input.pressed(KeyCode::W) {
        deltav.y += 1.;
    }

    if input.pressed(KeyCode::S) {
        deltav.y -= 1.;
    }

    let deltat = time.delta_seconds();
    let acc = ACCEL_RATE * deltat;

    pv.velocity = if deltav.length() > 0. {
        (pv.velocity + (deltav.normalize_or_zero() * acc)).clamp_length_max(PLAYER_SPEED)
    } else if pv.velocity.length() > acc {
        pv.velocity + (pv.velocity.normalize_or_zero() * -acc)
    } else {
        Vec2::splat(0.)
    };
    let change = pv.velocity * deltat;

    let new_pos = pt.translation+Vec3::new(change.x, 0.0, 0.0);
    if check_for_collision(new_pos, &wall_query, &button_query, &door_query){
        pt.translation = new_pos;
    }
    let new_pos = pt.translation+Vec3::new(0.0, change.y, 0.0);
    if check_for_collision(new_pos, &wall_query, &button_query, &door_query){
        pt.translation = new_pos;
    }
}

fn animate_button(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut button: Query<
        (
            &Pressed,
            &mut TextureAtlasSprite,
            &Handle<TextureAtlas>,
        ),
        With<Button>,
    >,
) {
    let (pressed, mut sprite, texture_atlas_handle) = button.single_mut();
    if pressed.pressed {
        let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
        sprite.index = 30;
    } else {
        let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
        sprite.index = 29;
    }
}

fn animate_player(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut player: Query<
        (
            &velocity::Velocity,
            &mut TextureAtlasSprite,
            &Handle<TextureAtlas>,
            &mut AnimationTimer,
        ),
        With<Player>,
    >,
) {
    let (v, mut sprite, texture_atlas_handle, mut timer) = player.single_mut();
    if v.velocity.cmpne(Vec2::ZERO).any() {
        timer.tick(time.delta());

        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            if v.velocity.x.abs() > v.velocity.y.abs() {
                if v.velocity.x > 0. {
                    sprite.index = 1;
                } else {
                    sprite.index = 3;
                }
            } else {
                if v.velocity.y > 0. {
                    sprite.index = 2;
                } else {
                    sprite.index = 0;
                }
            }
            // sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}


fn follow_player(
    time: Res<Time>,
    mut player_query: Query<(&Transform, & velocity::Velocity), With<Player>>,
    mut enemy_query: Query<(&mut Transform, &mut velocity::Velocity), (With<Enemy>, Without<Player>, Without<Wall>)>,
   
    //mut enemy_query: Query<(&mut Transform, &mut Velocity), (With<Enemy>, Without<Player>)>,
    wall_query: Query<&Transform, (With<Wall>, Without<Player>)>,
    button_query: Query<(&Transform, &Pressed), (With<Button>, Without<Player>, Without<Enemy>)>,
    door_query: Query<&Transform, (With<Door>, Without<Player>, Without<Wall>, Without<Enemy>)>,
) {
    
        for (player_transform, _player_velocity) in player_query.iter() {
        
        for (mut enemy_transform,mut enemy_velocity) in &mut enemy_query.iter_mut() {
            //calculate the direction from enemy to player
            
            let direction = player_transform.translation.truncate()- enemy_transform.translation.truncate();
            //let direction =  enemy_transform.translation.truncate()-player_transform.translation.truncate();
            let distance = direction.length();
            let speed = 2.0; //adjust the enemy's speed meh

            if distance > 0.0 {
                // normalize the direction vector and convert it to vec2
                
                let normalized_direction = direction.normalize_or_zero(); // convert to Vec2 cuz not vec3

                // calculate the velocity to move towards the player
                enemy_velocity.velocity = normalized_direction * speed;

                // check for collision with walls maybe?
                let new_position = enemy_transform.translation.truncate()
                    + Vec2::new(
                        enemy_velocity.velocity.x,
                        enemy_velocity.velocity.y,
                    );
                if check_for_collision(new_position.extend(0.0), &wall_query, &button_query, &door_query) {
                    enemy_transform.translation = new_position.extend(0.0);
                }
                //enemy_transform.translation = new_position.extend(0.0);
            } else {
                // if the enemy is already at the player's position, stop moving i think?
                enemy_velocity.velocity = Vec2::ZERO;
            }
        }
    }
}
