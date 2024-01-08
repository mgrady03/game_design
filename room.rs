use bevy::prelude::*;

use crate::WIN_H;
use crate::GameStates;
use crate::components::collision::Collidable;
use crate::components::enemy::EnemyPlugin;
use crate::components::size::Size;
use crate::systems::game::{TILE_SIZE, SCALE, ANIM_TIME};
use crate::components::game::*;
use crate::components::player::PlayerPlugin;
use crate::components::player::AnimationTimer;

// 2d array to represent empty room
pub const ROOM: [[usize; 10]; 8] = [
    [ 0,  1,  1,  2, 33, 34,  5,  1,  1,  6], 
    [ 7,  8,  8,  9, 10, 11, 12,  8,  8, 13], 
    [14, 15, 15, 15, 17, 18, 15, 15, 15, 20],
    [14, 15, 15, 15, 15, 15, 15, 15, 15, 20],
    [14, 15, 15, 15, 15, 15, 15, 15, 15, 20],
    [14, 15, 15, 15, 15, 15, 15, 15, 15, 20],
    [14, 15, 15, 15, 16, 19, 15, 15, 15, 20],
    [21, 22, 22, 22, 23, 26, 22, 22, 22, 27],
];

pub const COMBAT_ROOM: (usize,usize) = (33, 34);
pub const PUZZLE_ROOM: (usize,usize) = (24, 25);
pub const STANDARD_ROOM: (usize,usize) = (3, 4);

pub const DUNGEON_SET: (&str, Color) = ("dungeon_tiles.png", Color::rgb(0.49019607843137253,0.3843137254901961,0.36470588235294116));
pub const OUTSIDE_SET: (&str, Color) = ("outside_dungeon_tiles.png", Color::rgb(0.4118,0.4902,0.3647));
pub const PVP_SET: (&str, Color) = ("pvp_dungeon_tiles.png", Color::rgb(0.49, 0.361, 0.361));

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    spawn_room_shell(&mut commands, &asset_server, &mut texture_atlases, OUTSIDE_SET);
    spawn_empty_contents(&mut commands, &asset_server, &mut texture_atlases,  OUTSIDE_SET);
}
pub fn animate_door(
    mut commands: Commands,
    mut button: Query<&Button,With<Button>,>,
    door: Query<Entity,With<CloseDoor>,>,)
    {
        for button in button.iter_mut() {
            if button.pressed {
                for entity in door.iter(){
                    commands.entity(entity).despawn();
                }
            }
        }
}

pub fn animate_button(
    time: Res<Time>,
    commands: Commands,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut floor_button: Query<
        (
            &Button,
            &mut TextureAtlasSprite,
            &Handle<TextureAtlas>,
        ),
        (
            With<Button>,
            Without<Wall>
        )
    >,
    mut wall_button: Query<
        (
            &Button,
            &mut TextureAtlasSprite,
            &Handle<TextureAtlas>,
        ),
        (
            With<Button>,
            With<Wall>
        )
    >,
    ) {
    for button in floor_button.iter_mut() { // animate floor button(s)
        let (button1, mut sprite, texture_atlas_handle) = button;
        if button1.pressed {
            sprite.index = 30;
            } else {
            sprite.index = 29;
        }
    }
    for button in wall_button.iter_mut() {  // animate wall button
        let (button1, mut sprite, texture_atlas_handle) = button;
        if button1.pressed {
            sprite.index = 36;
            } else {
            sprite.index = 35;
        }
    }
}







/////////////////////////////////////////








use crate::components::room::{Floor, Wall, Block, Button, Door, CloseDoor, Carpet};
use crate::components::player::Player;
use crate::components::enemy::{Enemy, Fireball};

use crate::components::room::Background;


pub fn clear_floor (
    commands: &mut Commands,
    floors: &Query<Entity,(With<Floor>,Without<Carpet>,Without<Button>)>,
    buttons: &Query<Entity,With<Button>,>,
    blocks: &Query<Entity,With<Block>,>,
    enemies: &Query<Entity,With<Enemy>,>,
    fireballs: &Query<Entity,With<Fireball>,>,
)
{
    for entity in floors.iter(){
        commands.entity(entity).despawn();
    }
    for entity in buttons.iter(){
        commands.entity(entity).despawn();
    }
    for entity in blocks.iter(){
        commands.entity(entity).despawn();
    }
    for entity in enemies.iter(){
        commands.entity(entity).despawn();
    }
    for entity in fireballs.iter(){
        commands.entity(entity).despawn();
    }
}

pub fn clear_room (
    commands: &mut Commands,
    floors: &Query<Entity,(With<Floor>,Without<Carpet>,Without<Button>)>,
    buttons: &Query<Entity,With<Button>,>,
    blocks: &Query<Entity,With<Block>,>,
    enemies: &Query<Entity,With<Enemy>,>,
    fireballs: &Query<Entity,With<Fireball>,>,
    walls: &Query<Entity,(With<Wall>,Without<Block>)>,
    carpets: &Query<Entity,With<Carpet>>,
    background: &Query<Entity,With<Background>>,
)
{
    for entity in floors.iter(){
        commands.entity(entity).despawn();
    }
    for entity in buttons.iter(){
        commands.entity(entity).despawn();
    }
    for entity in blocks.iter(){
        commands.entity(entity).despawn();
    }
    for entity in enemies.iter(){
        commands.entity(entity).despawn();
    }
    for entity in fireballs.iter(){
        commands.entity(entity).despawn();
    }
    for entity in walls.iter(){
        commands.entity(entity).despawn();
    }
    for entity in carpets.iter(){
        commands.entity(entity).despawn();
    }
    for entity in background.iter(){
        commands.entity(entity).despawn();
    }

}

pub fn is_passable(
    curr_room: &[[usize; 10]; 8],
    door_type: (usize, usize),
) -> bool {
    // recursively verify path with DFS
    return path_between(curr_room, (6, 4), (1, 4), &mut [[false; 10]; 8], &mut false, door_type);
}

fn path_between(
    curr_room: &[[usize; 10]; 8],
    start: (usize, usize),
    dest: (usize, usize),
    seen: &mut [[bool; 10]; 8],
    button_found: &mut bool,
    door_type: (usize, usize),
) -> bool {
    let mut path_found: bool = false;
    if start == dest{
        // only return true if button has been found on path in puzzle rooms
        return *button_found || door_type != PUZZLE_ROOM;
    } else if seen[start.0][start.1] {
        // tile has already been seen, backtrack
        return false;
    }

    // mark tile as visited
    seen[start.0][start.1] = true;

    // check for button
    if curr_room[start.0][start.1] == 29 || curr_room[start.0][start.1] == 30 || curr_room[start.0+1][start.1] == 35 || curr_room[start.0+1][start.1] == 36 {
        *button_found = true;
    }
    
    // check up
    if !path_found && !seen[start.0-1][start.1] && is_walkable(curr_room[start.0-1][start.1]) {
        path_found = path_between(curr_room, (start.0-1, start.1), dest, seen, button_found, door_type);
    }
    // check left
    if !path_found && !seen[start.0][start.1-1] && is_walkable(curr_room[start.0][start.1-1]) {
        path_found = path_between(curr_room, (start.0, start.1-1), dest, seen, button_found, door_type);
    }
    // check right
    if !path_found && !seen[start.0][start.1+1] && is_walkable(curr_room[start.0][start.1+1]) {
        path_found = path_between(curr_room, (start.0, start.1+1), dest, seen, button_found, door_type);
    }
    // check down
    if !path_found && !seen[start.0+1][start.1] && is_walkable(curr_room[start.0+1][start.1]) {
        path_found = path_between(curr_room, (start.0+1, start.1), dest, seen, button_found, door_type);
    }
    return path_found;
}

fn is_walkable (
    tile_index: usize
) -> bool {
    // the following tile indexes are walkable, except for 35 and 36 which are wall buttons
    return (15 <= tile_index && tile_index <= 19) || tile_index == 10 || tile_index == 11 || tile_index == 29 || tile_index == 30;
}

pub fn generate_room(
    dungeon_atlas_len: usize,
    door_type: (usize, usize),
    puzzle_type: usize,
    wall_button_location: usize,
) -> [[usize; 10]; 8] {
    let mut curr_room = ROOM;
    let mut room_complete = false;

    while !room_complete {
        // Spawn each tile in a 10x8 room centered at origin
        let mut num_blocks: usize = 9+ (rand::random::<usize>() % 15);
        if door_type == COMBAT_ROOM {   // reduce block gen in combat rooms
            num_blocks = 2+ (rand::random::<usize>() % 4);
        }

        // array of length 36
        let mut other_tiles = [15; 36];
        // place num_blocks blocks and NUM_BUTTONS buttons randomly into other_tiles
        for i in 0..num_blocks {
            let mut rand = rand::random::<usize>() % 36;
            while other_tiles[rand] != 15 {
                rand = rand::random::<usize>() % 36;
            }
            other_tiles[rand] = 28;
        }

        if door_type == PUZZLE_ROOM && (puzzle_type == 0 || puzzle_type == 1){  // single floor button
            if puzzle_type == 0 || puzzle_type == 1 {   // single floor button
                let mut rand = rand::random::<usize>() % 36;
                while other_tiles[rand] != 15 {
                    rand = rand::random::<usize>() % 36;
                }
                other_tiles[rand] = 29;
            }
        }
        
        let mut floor_counter = 0;
        let mut wall_bottom_counter = 0;
        for x in 0..10 {
            for y in 0..8 {
                // 7-y to reverse order, since 2d array is spawned from bottom up
                let mut tile_index = ROOM[7-y][x] % dungeon_atlas_len;
                if tile_index == 15 {
                    tile_index = other_tiles[floor_counter];
                    floor_counter += 1;
                }
                // door top, should be different for each type of room. 33 & 34 for boss, 24 & 25 for puzzle, 3 & 4 for normal
                if tile_index == 33 {  
                    tile_index = door_type.0;
                } else if tile_index == 34{
                    tile_index = door_type.1;
                }
                // wall button
                if tile_index == 8 && door_type == PUZZLE_ROOM && puzzle_type == 2 {
                    if wall_bottom_counter == wall_button_location {    // counts from 0-3 and places button on the wall
                        tile_index = 35;
                    } 
                    wall_bottom_counter += 1;
                }
                curr_room[y][x] = tile_index;
            }
        }
        room_complete = is_passable(&curr_room, door_type);
    }
    return curr_room;
}

pub fn spawn_room_contents(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    room_type: (usize, usize),
    tile_set: (&str, Color)
) {
    let dungeon_handle = asset_server.load(tile_set.0);
    let dungeon_atlas = TextureAtlas::from_grid(dungeon_handle, Vec2::splat(TILE_SIZE), 7, 6, None, None);
    let dungeon_atlas_len = dungeon_atlas.textures.len();
    let dungeon_atlas_handle = texture_atlases.add(dungeon_atlas);

    let puzzle_type = rand::random::<usize>() % 3;  // 0 = single floor button, 1 = TODO, 2 = wall button
    let wall_button_location = rand::random::<usize>() % 4; // 0-3

    let curr_room = generate_room(dungeon_atlas_len.into(), room_type, puzzle_type, wall_button_location);

    for x in 0..10 {
        for y in 0..8 {
            let tile_index = curr_room[y][x];
            if tile_index == 15 {
                commands.spawn(SpriteSheetBundle {
                texture_atlas: dungeon_atlas_handle.clone(),
                transform: Transform::from_xyz((x as f32 - 4.5) as f32 * TILE_SIZE * SCALE, (y as f32 - 3.5) as f32 * TILE_SIZE * SCALE, -0.5)
                .with_scale(Vec3::new(SCALE,SCALE,0.0)), //Changes scale of the tile map
                sprite: TextureAtlasSprite {
                index: tile_index,
                ..default()
                },
                ..default()
                }).insert(Floor);
            } else if 16 <= tile_index && tile_index <= 19 {
                commands.spawn(SpriteSheetBundle {
                texture_atlas: dungeon_atlas_handle.clone(),
                transform: Transform::from_xyz((x as f32 - 4.5) as f32 * TILE_SIZE * SCALE, (y as f32 - 3.5) as f32 * TILE_SIZE * SCALE, -0.5)
                .with_scale(Vec3::new(SCALE,SCALE,0.0)), //Changes scale of the tile map
                sprite: TextureAtlasSprite {
                index: tile_index,
                ..default()
                },
                ..default()
                }).insert(Floor).insert(Carpet);
            } else if tile_index == 28 {
                commands.spawn(SpriteSheetBundle {
                texture_atlas: dungeon_atlas_handle.clone(),
                transform: Transform::from_xyz((x as f32 - 4.5) as f32 * TILE_SIZE * SCALE, (y as f32 - 3.5) as f32 * TILE_SIZE * SCALE, -0.5)
                .with_scale(Vec3::new(SCALE,SCALE,0.0)), //Changes scale of the tile map
                sprite: TextureAtlasSprite {
                index: tile_index,
                ..default()
                },
                ..default()
                }).insert(Wall).insert(Block)
                .insert(Collidable)
                .insert(Size::new(Vec2::new(80.,80.)));
            } else if tile_index == 29 || tile_index == 30 {
                commands.spawn(SpriteSheetBundle {
                texture_atlas: dungeon_atlas_handle.clone(),
                transform: Transform::from_xyz((x as f32 - 4.5) as f32 * TILE_SIZE * SCALE, (y as f32 - 3.5) as f32 * TILE_SIZE * SCALE, -0.5)
                .with_scale(Vec3::new(SCALE,SCALE,0.0)), //Changes scale of the tile map
                sprite: TextureAtlasSprite {
                index: tile_index,
                ..default()
                },
                ..default()
                }).insert(Floor).insert(Button::new())
                .insert(Collidable)
                .insert(Size::new(Vec2::new(50.,50.)));
            } else if tile_index == 10 || tile_index == 11 {
                commands.spawn(SpriteSheetBundle {
                texture_atlas: dungeon_atlas_handle.clone(),
                transform: Transform::from_xyz((x as f32 - 4.5) as f32 * TILE_SIZE * SCALE, (y as f32 - 3.5) as f32 * TILE_SIZE * SCALE, -0.5)
                .with_scale(Vec3::new(SCALE,SCALE,0.0)), //Changes scale of the tile map
                sprite: TextureAtlasSprite {
                index: tile_index,
                ..default()
                },
                ..default()
                }).insert(Floor).insert(Door)
                .insert(Collidable)
                .insert(Size::new(Vec2::new(80.,5.)));
                if room_type != STANDARD_ROOM && tile_set != PVP_SET {  // TODO remove second condition when pvp room is implemented
                    // spawn "closed door" wall over top to be destroyed when the door is opened
                    commands.spawn(SpriteSheetBundle {
                    texture_atlas: dungeon_atlas_handle.clone(),
                    transform: Transform::from_xyz((x as f32 - 4.5) as f32 * TILE_SIZE * SCALE, (y as f32 - 3.5) as f32 * TILE_SIZE * SCALE, -0.1)
                    .with_scale(Vec3::new(SCALE,SCALE,0.0)), //Changes scale of the tile map
                    sprite: TextureAtlasSprite {
                    index: tile_index+21,   // spawns tiles 30 & 31, the "closed door" wall
                    ..default()
                    },
                    ..default()
                    }).insert(Wall).insert(CloseDoor)
                    .insert(Collidable)
                    .insert(Size::new(Vec2::new(80.,20.)));
                }
            } else if tile_index == 33 || tile_index == 34 || tile_index == 3 || tile_index == 4 || tile_index == 24 || tile_index == 25{
                commands.spawn(SpriteSheetBundle {
                texture_atlas: dungeon_atlas_handle.clone(),
                transform: Transform::from_xyz((x as f32 - 4.5) as f32 * TILE_SIZE * SCALE, (y as f32 - 3.5) as f32 * TILE_SIZE * SCALE, -0.5)
                .with_scale(Vec3::new(SCALE,SCALE,0.0)), //Changes scale of the tile map
                sprite: TextureAtlasSprite {
                index: tile_index,
                ..default()
                },
                ..default()
                }).insert(Wall).insert(Block);  // assign top of door to Block so it is cleared on room clear
            } else if tile_index == 35 || tile_index == 36 {
                commands.spawn(SpriteSheetBundle {
                texture_atlas: dungeon_atlas_handle.clone(),
                transform: Transform::from_xyz((x as f32 - 4.5) as f32 * TILE_SIZE * SCALE, (y as f32 - 3.5) as f32 * TILE_SIZE * SCALE, -0.4)
                .with_scale(Vec3::new(SCALE,SCALE,0.0)), //Changes scale of the tile map
                sprite: TextureAtlasSprite {
                index: tile_index,
                ..default()
                },
                ..default()
                }).insert(Wall)
                .insert(Button::new())
                .insert(Collidable)
                .insert(Size::new(Vec2::new(50.,50.)));
            }
        }
    }
}

pub fn spawn_room_shell(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    tile_set: (&str, Color)
) {
    // Background
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: tile_set.1,
            custom_size: Some(Vec2::new(1280.,720.)),
            ..default()
        },
        transform: Transform::from_xyz(0., 0., -1.),
        ..default()
    });

    let dungeon_handle = asset_server.load(tile_set.0);
    let dungeon_atlas = TextureAtlas::from_grid(dungeon_handle, Vec2::splat(TILE_SIZE), 7, 6, None, None);
    let dungeon_atlas_len = dungeon_atlas.textures.len();
    let dungeon_atlas_handle = texture_atlases.add(dungeon_atlas);

    // Spawn each tile in a 10x8 room centered at origin
    for x in 0..10 {
        for y in 0..8 {
            // 7-y to reverse order, since 2d array is spawned from bottom up
            let tile_index = ROOM[7-y][x] % dungeon_atlas_len;
            let mut chosen = Vec2::new(0.,0.);
            if tile_index != 15 && tile_index != 16 && tile_index != 17 && tile_index != 18 && tile_index != 19 && tile_index != 29 && tile_index != 30 && tile_index != 10 && tile_index != 11 && tile_index != 33 && tile_index != 34{
                if tile_index == 22 || tile_index == 23 || tile_index == 26 || tile_index == 9 || tile_index == 8 || tile_index == 12{
                    chosen = Vec2::new(80.,25.);
                }
                else {
                    chosen = Vec2::new(25.,80.);
                }
                commands.spawn(SpriteSheetBundle {
                texture_atlas: dungeon_atlas_handle.clone(),
                transform: Transform::from_xyz((x as f32 - 4.5) as f32 * TILE_SIZE * SCALE, (y as f32 - 3.5) as f32 * TILE_SIZE * SCALE, -0.5)
                .with_scale(Vec3::new(SCALE,SCALE,0.0)), //Changes scale of the tile map
                sprite: TextureAtlasSprite {
                index: tile_index,
                ..default()
                },
                ..default()
                }).insert(Wall)
                .insert(Collidable)
                .insert(Size::new(chosen));
            }
        }
    }
}

pub fn spawn_empty_contents(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    tile_set: (&str, Color)
) {
    let dungeon_handle = asset_server.load(tile_set.0);
    let dungeon_atlas = TextureAtlas::from_grid(dungeon_handle, Vec2::splat(TILE_SIZE), 7, 6, None, None);
    let dungeon_atlas_len = dungeon_atlas.textures.len();
    let dungeon_atlas_handle = texture_atlases.add(dungeon_atlas);

    for x in 0..10 {
        for y in 0..8 {
            // 7-y to reverse order, since 2d array is spawned from bottom up
            let tile_index = ROOM[7-y][x] % dungeon_atlas_len;
            
            if tile_index == 15 {
                commands.spawn(SpriteSheetBundle {
                texture_atlas: dungeon_atlas_handle.clone(),
                transform: Transform::from_xyz((x as f32 - 4.5) as f32 * TILE_SIZE * SCALE, (y as f32 - 3.5) as f32 * TILE_SIZE * SCALE, -0.5)
                .with_scale(Vec3::new(SCALE,SCALE,0.0)), //Changes scale of the tile map
                sprite: TextureAtlasSprite {
                index: tile_index,
                ..default()
                },
                ..default()
                }).insert(Floor);
            } else if 16 <= tile_index && tile_index <= 19 {
                commands.spawn(SpriteSheetBundle {
                texture_atlas: dungeon_atlas_handle.clone(),
                transform: Transform::from_xyz((x as f32 - 4.5) as f32 * TILE_SIZE * SCALE, (y as f32 - 3.5) as f32 * TILE_SIZE * SCALE, -0.5)
                .with_scale(Vec3::new(SCALE,SCALE,0.0)), //Changes scale of the tile map
                sprite: TextureAtlasSprite {
                index: tile_index,
                ..default()
                },
                ..default()
                }).insert(Floor).insert(Carpet);
            } else if tile_index == 28 {
                commands.spawn(SpriteSheetBundle {
                    texture_atlas: dungeon_atlas_handle.clone(),
                    transform: Transform::from_xyz((x as f32 - 4.5) as f32 * TILE_SIZE * SCALE, (y as f32 - 3.5) as f32 * TILE_SIZE * SCALE, -0.5)
                    .with_scale(Vec3::new(SCALE,SCALE,0.0)), //Changes scale of the tile map
                    sprite: TextureAtlasSprite {
                    index: tile_index,
                    ..default()
                    },
                    ..default()
                    }).insert(Wall).insert(Block)
                    .insert(Collidable)
                    .insert(Size::new(Vec2::new(80.,80.)));
            } else if tile_index == 29 || tile_index == 30 {
                commands.spawn(SpriteSheetBundle {
                texture_atlas: dungeon_atlas_handle.clone(),
                transform: Transform::from_xyz((x as f32 - 4.5) as f32 * TILE_SIZE * SCALE, (y as f32 - 3.5) as f32 * TILE_SIZE * SCALE, -0.5)
                .with_scale(Vec3::new(SCALE,SCALE,0.0)), //Changes scale of the tile map
                sprite: TextureAtlasSprite {
                index: tile_index,
                ..default()
                },
                ..default()
                }).insert(Floor).insert(Button::new())
                .insert(Collidable)
                .insert(Size::new(Vec2::new(50.,50.)));
            } else if tile_index == 10 || tile_index == 11 {
                commands.spawn(SpriteSheetBundle {
                texture_atlas: dungeon_atlas_handle.clone(),
                transform: Transform::from_xyz((x as f32 - 4.5) as f32 * TILE_SIZE * SCALE, (y as f32 - 3.5) as f32 * TILE_SIZE * SCALE, -0.5)
                .with_scale(Vec3::new(SCALE,SCALE,0.0)), //Changes scale of the tile map
                sprite: TextureAtlasSprite {
                index: tile_index,
                ..default()
                },
                ..default()
                }).insert(Floor).insert(Door)
                .insert(Collidable)
                .insert(Size::new(Vec2::new(80.,5.)));
                // spawn "closed door" wall over top to be destroyed when the door is opened
                commands.spawn(SpriteSheetBundle {
                texture_atlas: dungeon_atlas_handle.clone(),
                transform: Transform::from_xyz((x as f32 - 4.5) as f32 * TILE_SIZE * SCALE, (y as f32 - 3.5) as f32 * TILE_SIZE * SCALE, -0.1)
                .with_scale(Vec3::new(SCALE,SCALE,0.0)), //Changes scale of the tile map
                sprite: TextureAtlasSprite {
                index: tile_index+21,   // spawns tiles 30 & 31, the "closed door" wall
                ..default()
                },
                ..default()
                }).insert(Wall).insert(CloseDoor)
                .insert(Collidable)
                .insert(Size::new(Vec2::new(80.,20.)));
            } else if tile_index == 33 || tile_index == 34 {
                commands.spawn(SpriteSheetBundle {
                texture_atlas: dungeon_atlas_handle.clone(),
                transform: Transform::from_xyz((x as f32 - 4.5) as f32 * TILE_SIZE * SCALE, (y as f32 - 3.5) as f32 * TILE_SIZE * SCALE, -0.4)
                .with_scale(Vec3::new(SCALE,SCALE,0.0)), //Changes scale of the tile map
                sprite: TextureAtlasSprite {
                index: tile_index,
                ..default()
                },
                ..default()
                }).insert(Wall).insert(Block)
                .insert(Collidable)
                .insert(Size::new(Vec2::new(80.,80.)));
            }
        }
    }
}