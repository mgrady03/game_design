use bevy::{prelude::*, window::PresentMode};

mod velocity;

const TITLE: &str = "bv02 Basic";
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;
const PLAYER_SIZE: f32 = 32.;
const TILE_SIZE: f32 = 16.;
const SCALE: f32 = 5.0;



struct Player {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    health: i32,
    damage: i32,
    velocity: velocity::Velocity,
}


impl Player{
    fn spawn() -> Player {
        //these are the tile coordinates we want the player to spawn at 
        //let tile_x = 5;
        let tile_x = TILE_SIZE * SCALE * -2.5;
        //let tile_y = 2;
        let tile_y = TILE_SIZE * SCALE * -0.5;

        //sprite size 
        let sprite_width = 40; 
        let sprite_height = 40;

        //offset from bottom left corner to player 
        let room_left_x = 240; //offset for room x coordinate
        let room_left_y = 40; //offset for room y coordinate

        //math for pixel coordinates
        let player_spawn_x = room_left_x + (tile_x * sprite_width);//coordinates
        let player_spawn_y = room_left_y + (tile_y * sprite_height);//coordinates

        //damage and health
        let player_health = 10;
        let player_damage = 3;

        //velocity
        let player_velocity = velocity::Velocity::new();

        //player struct
        Player {
            x: player_spawn_x, //coordinates
            y: player_spawn_y, //coordinates
            width: sprite_width, //->where player will spawn width
            height: sprite_height, //->where player will spawn height
            health: player_health,
            damage: player_damage,
            velocity: player_velocity,
        }
    }

}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0., 1., 1.)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: TITLE.into(),
                resolution: (WIN_W, WIN_H).into(),
                present_mode: PresentMode::Fifo,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        //.add_systems(Update, spawn)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands
    .spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb_u8(0, 0, 0),
            custom_size: Some(Vec2::splat(PLAYER_SIZE)),
            ..default()
        
        },
        transform: Transform {
            translation: Vec3::new(-WIN_W / 4., 0., 0.),
            ..default()
        },
        ..default()
    });
    //.insert(Player);
}

